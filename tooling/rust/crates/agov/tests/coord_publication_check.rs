use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn committed_record_fixture(content: &str) -> GitFixture {
    let repo = GitFixture::new();
    repo.write("README.md", "base");
    repo.git(&["add", "."]);
    repo.git(&["commit", "-q", "-m", "chore: base"]);
    repo.git(&["branch", "-M", "main"]);
    repo.git(&["checkout", "-q", "-b", "coord/candidate"]);
    repo.write("docs/coordination/record.md", content);
    repo.git(&["add", "."]);
    repo.git(&["commit", "-q", "-m", "coordination: add record"]);
    repo
}

fn add_merge_history(repo: &GitFixture) {
    repo.git(&["checkout", "-q", "-b", "side", "main"]);
    repo.write("docs/coordination/side.md", &coord("side"));
    repo.git(&["add", "."]);
    repo.git(&["commit", "-q", "-m", "coordination: side"]);
    repo.git(&["checkout", "-q", "coord/candidate"]);
    repo.git(&[
        "merge",
        "--no-ff",
        "-q",
        "side",
        "-m",
        "coordination: imported merge",
    ]);
}

#[cfg(unix)]
#[test]
fn head_drift_after_range_enumeration_reports_only_pinned_candidate() {
    use std::os::unix::fs::PermissionsExt;
    let repo = committed_record_fixture(&coord("valid"));
    let sha = |name| {
        String::from_utf8(
            Command::new("git")
                .current_dir(&repo.root)
                .args(["rev-parse", name])
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap()
        .trim()
        .to_string()
    };
    let clean = sha("HEAD");
    let target = sha("main");
    add_merge_history(&repo);
    let forbidden = sha("HEAD");
    repo.git(&["reset", "--hard", &clean]);
    let real_git = Command::new("sh")
        .args(["-c", "command -v git"])
        .output()
        .unwrap();
    let real_git = String::from_utf8(real_git.stdout).unwrap();
    repo.write(
        "fakebin/git",
        r#"#!/bin/sh
printf '%s\n' "$*" >> "$AGOV_TEST_ARG_LOG"
if [ "$1" = rev-list ] && [ "$2" = --reverse ]; then
  "$AGOV_TEST_REAL_GIT" "$@" || exit $?
  "$AGOV_TEST_REAL_GIT" update-ref HEAD "$AGOV_TEST_DRIFT_TO" || exit $?
  exit 0
fi
exec "$AGOV_TEST_REAL_GIT" "$@"
"#,
    );
    fs::set_permissions(
        repo.root.join("fakebin/git"),
        fs::Permissions::from_mode(0o755),
    )
    .unwrap();
    let mut paths = vec![repo.root.join("fakebin")];
    paths.extend(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    ));
    let schema_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../agf/schemas")
        .canonicalize()
        .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_agov"))
        .current_dir(&repo.root)
        .env("PATH", std::env::join_paths(paths).unwrap())
        .env("AGOV_TEST_REAL_GIT", real_git.trim())
        .env("AGOV_TEST_DRIFT_TO", &forbidden)
        .env("AGOV_TEST_ARG_LOG", repo.root.join("argv.log"))
        .env("AGOV_SCHEMA_DIR", schema_dir)
        .env("AGOV_TOKEN_ROI", "0")
        .args([
            "coord",
            "publication-check",
            "--base-ref",
            "main",
            "--format",
            "json",
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["candidate_sha"], clean);
    assert_eq!(json["target_sha"], target);
    assert_eq!(sha("HEAD"), forbidden);
    let log = fs::read_to_string(repo.root.join("argv.log")).unwrap();
    let calls: Vec<_> = log.lines().collect();
    let pin = calls
        .iter()
        .position(|line| line.ends_with("HEAD^{commit}"))
        .unwrap();
    assert!(
        calls
            .iter()
            .skip(pin + 1)
            .all(|line| !line.contains("HEAD")),
        "{log}"
    );
    assert!(
        !json["commits_checked"]
            .as_array()
            .unwrap()
            .iter()
            .any(|s| s == &forbidden)
    );
}

#[cfg(unix)]
#[test]
fn decisive_git_read_failures_report_nonzero_json_and_operation() {
    use std::os::unix::fs::PermissionsExt;
    for history in ["clean", "merge", "revert"] {
        let repo = committed_record_fixture(&coord("valid"));
        if history == "merge" {
            add_merge_history(&repo);
        }
        if history == "revert" {
            repo.write("src/forbidden.rs", "implementation");
            repo.git(&["add", "."]);
            repo.git(&["commit", "-q", "-m", "coordination: implementation"]);
            fs::remove_file(repo.root.join("src/forbidden.rs")).unwrap();
            repo.git(&["add", "-A"]);
            repo.git(&["commit", "-q", "-m", "coordination: revert implementation"]);
        }
        let real_git = Command::new("sh")
            .args(["-c", "command -v git"])
            .output()
            .unwrap();
        assert!(real_git.status.success());
        let real_git = String::from_utf8(real_git.stdout).unwrap();
        repo.write(
            "fakebin/git",
            r#"#!/bin/sh
operation=""
if [ "$1" = rev-parse ] && [ "$4" = 'HEAD^{commit}' ]; then
  operation=candidate-resolution
else
case "$1 $2 $3" in
  'rev-parse --verify '*) operation=target-resolution ;;
  'merge-base '*) operation=merge-base ;;
  'rev-list --merges '*) operation=merges ;;
  'rev-list --reverse '*) operation=commits ;;
  'log -1 --format=%s') operation=subject ;;
  'log -1 --format=%B') operation=message ;;
  'diff-tree '*) operation=commit-diff ;;
  'diff --name-status '*) operation=aggregate-diff ;;
  'ls-tree '*) operation=tree ;;
  'cat-file '*) operation=blob ;;
esac
fi
if [ "$operation" = "$AGOV_TEST_FAIL_OPERATION" ]; then
  echo "injected git $operation failure" >&2
  exit 1
fi
exec "$AGOV_TEST_REAL_GIT" "$@"
"#,
        );
        fs::set_permissions(
            repo.root.join("fakebin/git"),
            fs::Permissions::from_mode(0o755),
        )
        .unwrap();
        let mut paths = vec![repo.root.join("fakebin")];
        paths.extend(std::env::split_paths(
            &std::env::var_os("PATH").unwrap_or_default(),
        ));
        let schema_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../../../agf/schemas")
            .canonicalize()
            .unwrap();
        for operation in [
            "target-resolution",
            "candidate-resolution",
            "merge-base",
            "merges",
            "commits",
            "subject",
            "message",
            "commit-diff",
            "aggregate-diff",
            "tree",
            "blob",
        ] {
            if history == "merge" && operation != "merges" {
                continue;
            }
            if history == "revert" && !["commit-diff", "aggregate-diff"].contains(&operation) {
                continue;
            }
            let output = Command::new(env!("CARGO_BIN_EXE_agov"))
                .current_dir(&repo.root)
                .env("PATH", std::env::join_paths(&paths).unwrap())
                .env("AGOV_TEST_REAL_GIT", real_git.trim())
                .env("AGOV_TEST_FAIL_OPERATION", operation)
                .env("AGOV_TOKEN_ROI", "0")
                .env("AGOV_SCHEMA_DIR", &schema_dir)
                .args([
                    "coord",
                    "publication-check",
                    "--base-ref",
                    "main",
                    "--format",
                    "json",
                ])
                .output()
                .unwrap();
            assert_eq!(
                output.status.code(),
                Some(1),
                "{operation}: {}",
                stderr(&output)
            );
            let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
            assert_eq!(json["ok"], false, "{operation}: {json}");
            assert!(
                json["reasons"].as_array().unwrap().iter().any(|r| r
                    .as_str()
                    .unwrap()
                    .contains(&format!("injected git {operation} failure"))),
                "{operation}: {json}"
            );
        }
    }
}

#[test]
fn diverged_target_inventory_rejects_copy_and_accepts_distinct_record() {
    for duplicate in [true, false] {
        let repo = GitFixture::new();
        repo.write("README.md", "base");
        repo.git(&["add", "."]);
        repo.git(&["commit", "-q", "-m", "chore: base"]);
        repo.git(&["branch", "-M", "main"]);
        repo.git(&["branch", "candidate"]);
        repo.write("docs/coordination/target.md", &coord("target"));
        repo.git(&["add", "."]);
        repo.git(&["commit", "-q", "-m", "coordination: target"]);
        repo.git(&["checkout", "-q", "candidate"]);
        repo.write(
            "docs/coordination/candidate.md",
            &coord(if duplicate { "target" } else { "distinct" }),
        );
        repo.git(&["add", "."]);
        repo.git(&["commit", "-q", "-m", "coordination: candidate"]);
        let output = repo.agov(&[
            "coord",
            "publication-check",
            "--base-ref",
            "main",
            "--format",
            "json",
        ]);
        assert_eq!(
            output.status.code(),
            Some(if duplicate { 1 } else { 0 }),
            "{}",
            stderr(&output)
        );
        let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(json["ok"], !duplicate);
    }
}

#[test]
fn nested_record_is_outside_publication_channel() {
    let repo = committed_record_fixture(&coord("direct"));
    repo.write("docs/coordination/nested/record.md", &coord("nested"));
    repo.git(&["add", "."]);
    repo.git(&["commit", "-q", "-m", "coordination: nested"]);
    let output = repo.agov(&["coord", "publication-check", "--base-ref", "main"]);
    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(stderr(&output).contains("non-coordination path: docs/coordination/nested/record.md"));
}

#[test]
fn invalid_committed_record_cannot_be_repaired_by_dirty_valid_file() {
    let repo = committed_record_fixture("---\nkind: note\n---\ninvalid");
    repo.write("docs/coordination/record.md", &coord("valid-dirty"));
    let output = repo.agov(&["coord", "publication-check", "--base-ref", "main"]);
    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(stderr(&output).contains("schema validation failed"));
}

#[test]
fn valid_committed_record_ignores_dirty_missing_and_untracked_files() {
    let repo = committed_record_fixture(&coord("valid"));
    repo.write("docs/coordination/record.md", "invalid dirty content");
    repo.write(
        "docs/coordination/untracked.md",
        "invalid untracked content",
    );
    let args = &["coord", "publication-check", "--base-ref", "main"];
    let output = repo.agov(args);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    fs::remove_file(repo.root.join("docs/coordination/record.md")).unwrap();
    let output = repo.agov(args);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
}

#[cfg(unix)]
#[test]
fn symlink_to_valid_record_is_not_a_regular_publication_record() {
    let repo = GitFixture::new();
    repo.write("docs/coordination/base.md", &coord("base"));
    repo.git(&["add", "."]);
    repo.git(&["commit", "-q", "-m", "chore: base"]);
    repo.git(&["branch", "-M", "main"]);
    repo.git(&["checkout", "-q", "-b", "coord/candidate"]);
    std::os::unix::fs::symlink("base.md", repo.root.join("docs/coordination/link.md")).unwrap();
    repo.git(&["add", "."]);
    repo.git(&["commit", "-q", "-m", "coordination: symlink"]);
    let output = repo.agov(&["coord", "publication-check", "--base-ref", "main"]);
    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(stderr(&output).contains("non-regular coordination record"));
}

#[test]
fn byte_identical_copy_is_rejected_by_cli_with_both_paths() {
    let repo = GitFixture::new();
    repo.write("docs/coordination/base.md", &coord("same"));
    repo.git(&["add", "."]);
    repo.git(&["commit", "-q", "-m", "chore: base"]);
    repo.git(&["branch", "-M", "main"]);
    repo.git(&["checkout", "-q", "-b", "coord/candidate"]);
    repo.write("docs/coordination/copy.md", &coord("same"));
    repo.git(&["add", "."]);
    repo.git(&["commit", "-q", "-m", "coordination: copy"]);
    let output = repo.agov(&["coord", "publication-check", "--base-ref", "main"]);
    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    let diagnostic = stderr(&output);
    assert!(diagnostic.contains("byte-identical copy"), "{diagnostic}");
    assert!(
        diagnostic.contains("docs/coordination/copy.md == docs/coordination/base.md"),
        "{diagnostic}"
    );
}

#[test]
fn clean_coordination_range_passes_and_reports_added_files() {
    let repo = GitFixture::new();
    repo.write("README.md", "base\n");
    repo.git(&["add", "."]);
    repo.git(&["commit", "-q", "-m", "chore: base"]);
    repo.git(&["branch", "-M", "main"]);
    repo.git(&["checkout", "-q", "-b", "coord/candidate"]);

    repo.write(
        "docs/coordination/20260727T000000Z-a.md",
        &coord("20260727T000000Z-a"),
    );
    repo.write(
        "docs/coordination/20260727T000001Z-b.md",
        &coord("20260727T000001Z-b"),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-q", "-m", "coordination: add same-cycle records"]);

    let output = repo.agov(&[
        "coord",
        "publication-check",
        "--base-ref",
        "main",
        "--format",
        "json",
    ]);

    assert_eq!(
        output.status.code(),
        Some(0),
        "stderr: {}\nstdout: {}",
        stderr(&output),
        String::from_utf8_lossy(&output.stdout)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"ok\":true"));
    assert!(stdout.contains("20260727T000000Z-a.md"));
    assert!(stdout.contains("20260727T000001Z-b.md"));
}

#[test]
fn mr692_shape_is_rejected_by_the_cli() {
    let repo = GitFixture::new();
    repo.write("README.md", "base\n");
    repo.git(&["add", "."]);
    repo.git(&["commit", "-q", "-m", "chore: base"]);
    repo.git(&["branch", "-M", "main"]);
    repo.git(&["checkout", "-q", "-b", "coord/candidate"]);

    repo.write("src/roi.rs", "fn roi() {}\n");
    repo.git(&["add", "."]);
    repo.git(&["commit", "-q", "-m", "fix: isolate token roi logs"]);

    repo.write(
        "docs/coordination/20260727T000100Z-b.md",
        &coord("20260727T000100Z-b"),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-q", "-m", "coordination: hand off for gate 2"]);

    let output = repo.agov(&["coord", "publication-check", "--base-ref", "main"]);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    let stderr_text = String::from_utf8_lossy(&output.stderr);
    assert!(stderr_text.contains("src/roi.rs"));
}

#[test]
fn invalid_added_record_fails_schema_validation() {
    let repo = GitFixture::new();
    repo.write("README.md", "base\n");
    repo.git(&["add", "."]);
    repo.git(&["commit", "-q", "-m", "chore: base"]);
    repo.git(&["branch", "-M", "main"]);
    repo.git(&["checkout", "-q", "-b", "coord/candidate"]);

    repo.write(
        "docs/coordination/20260727T000300Z-d.md",
        "---\nkind: note\n---\n\nmissing required fields\n",
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-q", "-m", "coordination: add invalid record"]);

    let output = repo.agov(&["coord", "publication-check", "--base-ref", "main"]);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
}

fn stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).to_string()
}

fn coord(id: &str) -> String {
    format!(
        "---\nkind: note\nfrom: coder-tester\nto: all\nid: {id}\ntimestamp: 2026-07-27T00:00:00Z\nimmutable: true\nreferences-to: []\ntopic: t\nrequires-ack: false\n---\n\n# body\n"
    )
}

struct GitFixture {
    root: PathBuf,
}

impl GitFixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "agov_coord_publication_check_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        fs::create_dir_all(root.join("docs/coordination")).unwrap();
        let fixture = Self { root };
        fixture.git(&["init", "-q"]);
        fixture.git(&["config", "user.email", "test@example.invalid"]);
        fixture.git(&["config", "user.name", "AGOV Test"]);
        fixture
    }

    fn write(&self, path: &str, content: &str) {
        let path = self.root.join(path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, content).unwrap();
    }

    fn git(&self, args: &[&str]) {
        let output = Command::new("git")
            .args(args)
            .current_dir(&self.root)
            .output()
            .expect("run git");
        assert!(
            output.status.success(),
            "git {:?} failed\nstdout:\n{}\nstderr:\n{}",
            args,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    fn agov(&self, args: &[&str]) -> std::process::Output {
        let schema_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../../../agf/schemas")
            .canonicalize()
            .expect("resolve real repo schema dir");
        Command::new(env!("CARGO_BIN_EXE_agov"))
            .env("AGOV_TOKEN_ROI", "0")
            .env("AGOV_SCHEMA_DIR", schema_dir)
            .args(args)
            .current_dir(&self.root)
            .output()
            .expect("run agov")
    }
}

impl Drop for GitFixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn unique_suffix() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}
