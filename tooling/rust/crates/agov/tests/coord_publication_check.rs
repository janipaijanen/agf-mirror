use std::fs;
use std::path::PathBuf;
use std::process::Command;

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
