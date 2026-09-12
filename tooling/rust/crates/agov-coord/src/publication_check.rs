//! Mechanical validation for the coordination-publication fast path.
//!
//! Implements the mechanically-decidable subset of
//! `agf/rules/workflow/coordination-publication-rule.md`: a coordination-only
//! candidate must be rooted on the target branch, every commit in the range
//! (not merely the tip diff) must touch only newly-added files under the
//! coordination directory, and no merge/imported commit may be present.
//!
//! This module inspects the full commit range, not just the aggregate diff,
//! because MR !692 demonstrated that an aggregate/tip diff can look
//! coordination-only while individual commits in the range carry
//! implementation changes (e.g. an implementation commit followed by a
//! revert followed by a coordination commit nets to a coordination-only
//! diff, but the range is not coordination-only).

use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::Path;
use std::process::Command;

/// Outcome of a publication-check run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationCheckReport {
    pub ok: bool,
    pub base_ref: String,
    pub merge_base: Option<String>,
    pub commits_checked: Vec<String>,
    pub added_files: Vec<String>,
    pub reasons: Vec<String>,
}

/// Run the publication check for `HEAD` against `base_ref`, treating
/// `coord_dir` (relative to `cwd`, e.g. `docs/coordination`) as the only
/// path new commits in the range may touch. Every commit's subject line in
/// the range must start with `subject_prefix` (e.g. `"coordination:"`) --
/// this is condition 6 of `coordination-publication-rule.md`: the range
/// must use the protected fast-path commit convention throughout, not only
/// at the tip.
pub fn publication_check(
    base_ref: &str,
    coord_dir: &str,
    subject_prefix: &str,
    cwd: &Path,
) -> PublicationCheckReport {
    publication_check_validated(base_ref, coord_dir, subject_prefix, cwd, &|_| Vec::new())
}

/// Also validate each added record from its introducing commit's raw object bytes.
/// The callback shares the exact object read used for copy detection, never a
/// working-tree file or the final head's possibly modified version.
pub fn publication_check_validated(
    base_ref: &str,
    coord_dir: &str,
    subject_prefix: &str,
    cwd: &Path,
    validate_added: &impl Fn(&[u8]) -> Vec<String>,
) -> PublicationCheckReport {
    publication_check_with_git(
        base_ref,
        coord_dir,
        subject_prefix,
        cwd,
        &git_bytes,
        validate_added,
    )
}

fn publication_check_with_git(
    base_ref: &str,
    coord_dir: &str,
    subject_prefix: &str,
    cwd: &Path,
    read_git: &impl Fn(&Path, &[&str]) -> Result<Vec<u8>, String>,
    validate_added: &impl Fn(&[u8]) -> Vec<String>,
) -> PublicationCheckReport {
    let git = |cwd: &Path, args: &[&str]| {
        read_git(cwd, args).map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
    };
    let mut reasons = Vec::new();
    let coord_prefix = format!("{}/", coord_dir.trim_end_matches('/'));

    let failure = |reason: String| PublicationCheckReport {
        ok: false,
        base_ref: base_ref.to_string(),
        merge_base: None,
        commits_checked: Vec::new(),
        added_files: Vec::new(),
        reasons: vec![reason],
    };
    // Resolve the selected target exactly once. Subsequent reads must use this
    // immutable commit, including ancestry and both inventory snapshots.
    let target = match git(
        cwd,
        &[
            "rev-parse",
            "--verify",
            "--end-of-options",
            &format!("{base_ref}^{{commit}}"),
        ],
    ) {
        Ok(sha) if !sha.trim().is_empty() => sha.trim().to_string(),
        result => {
            return failure(format!(
                "git rev-parse target failed for '{base_ref}': {}",
                result
                    .err()
                    .unwrap_or_else(|| "empty commit resolution".into())
            ));
        }
    };
    let merge_base = match git(cwd, &["merge-base", &target, "HEAD"]) {
        Ok(sha) if !sha.trim().is_empty() => sha.trim().to_string(),
        result => {
            return failure(format!(
                "git merge-base failed between pinned target {target} and HEAD: {}",
                result
                    .err()
                    .unwrap_or_else(|| "empty merge-base resolution".into())
            ));
        }
    };
    macro_rules! checked_git {
        ($args:expr) => {
            match git(cwd, $args) {
                Ok(value) => value,
                Err(error) => {
                    reasons.push(format!("git {} failed: {error}", $args.join(" ")));
                    String::new()
                }
            }
        };
    }

    let range = format!("{merge_base}..HEAD");

    let merge_commits = checked_git!(&["rev-list", "--merges", &range]);
    for line in merge_commits.lines().filter(|line| !line.trim().is_empty()) {
        reasons.push(format!(
            "merge commit {line} present in the candidate range; merge/imported history is rejected"
        ));
    }

    let commits: Vec<String> = checked_git!(&["rev-list", "--reverse", &range])
        .lines()
        .map(str::to_string)
        .filter(|line| !line.is_empty())
        .collect();

    if commits.is_empty() {
        reasons.push("no commits in range; nothing to publish".to_string());
    }

    for commit in &commits {
        let subject = checked_git!(&["log", "-1", "--format=%s", commit])
            .trim()
            .to_string();
        if !subject.starts_with(subject_prefix) {
            reasons.push(format!(
                "commit {commit} subject does not use the protected fast-path convention \
                 (expected to start with '{subject_prefix}'): {subject:?}"
            ));
        }

        // GitLab (and GitHub) skip pipeline *creation* entirely when a
        // commit message contains one of these markers -- a platform-level
        // behavior no .gitlab-ci.yml rule can override. A coordination-only
        // commit that carries one can silently never be mechanically
        // checked at all. Reject it here so the check is at least a
        // reliable local pre-flight (`agov coord publication-check` run
        // before push) even when CI itself would never run.
        let full_message = checked_git!(&["log", "-1", "--format=%B", commit]);
        let lower = full_message.to_lowercase();
        if lower.contains("[ci skip]") || lower.contains("[skip ci]") {
            reasons.push(format!(
                "commit {commit} message contains a ci-skip marker ([ci skip]/[skip ci]); \
                 this bypasses pipeline creation at the platform level and is forbidden on \
                 the coordination-publication fast path -- see coordination-publication-rule.md"
            ));
        }
    }

    let mut added_files = Vec::new();
    // Hashes only narrow the search; actual raw bytes decide identity.
    let mut seen = SeenContent::default();
    for snapshot in [&merge_base, &target] {
        // Inventory overlap is expected when target and merge base share records;
        // inserting it is not a candidate publication and never emits a copy.
        let entries = match read_git(cwd, &["ls-tree", "-r", "-z", snapshot, "--", coord_dir]) {
            Ok(entries) => entries,
            Err(error) => {
                reasons.push(format!("git ls-tree inventory {snapshot} failed: {error}"));
                continue;
            }
        };
        for entry in entries
            .split(|byte| *byte == 0)
            .filter(|entry| !entry.is_empty())
        {
            let Some(tab) = entry.iter().position(|byte| *byte == b'\t') else {
                reasons.push(format!(
                    "git ls-tree inventory {snapshot} returned malformed entry"
                ));
                continue;
            };
            let path = match std::str::from_utf8(&entry[tab + 1..]) {
                Ok(path) => path,
                Err(error) => {
                    reasons.push(format!(
                        "git ls-tree inventory {snapshot} returned undecodable path: {error}"
                    ));
                    continue;
                }
            };
            if !is_channel_record(path, &coord_prefix)
                || !(entry.starts_with(b"100644 blob ") || entry.starts_with(b"100755 blob "))
            {
                continue;
            }
            match read_git(cwd, &["cat-file", "blob", &format!("{snapshot}:{path}")]) {
                Ok(bytes) => {
                    seen.insert(path, bytes);
                }
                Err(error) => reasons.push(format!(
                    "git cat-file inventory {snapshot}:{path} failed: {error}"
                )),
            }
        }
    }

    for commit in &commits {
        let name_status =
            checked_git!(&["diff-tree", "--no-commit-id", "--name-status", "-r", commit]);
        for (status, path) in parse_name_status(&name_status) {
            if !is_channel_record(&path, &coord_prefix) {
                reasons.push(format!(
                    "commit {commit} touches a non-coordination path: {path}"
                ));
                continue;
            }
            if status != "A" {
                reasons.push(format!(
                    "commit {commit} performs a non-addition ({status}) on coordination path: {path}"
                ));
                continue;
            }
            // A symlink is a blob too, but it is not a Markdown record file.
            // Verify the introducing tree entry before accepting its bytes.
            match read_git(cwd, &["ls-tree", "-z", commit, "--", &path]) {
                Ok(entry)
                    if entry.starts_with(b"100644 blob ") || entry.starts_with(b"100755 blob ") => {
                }
                Ok(_) => {
                    reasons.push(format!(
                        "commit {commit} adds a non-regular coordination record: {path}"
                    ));
                    continue;
                }
                Err(error) => {
                    reasons.push(format!("cannot inspect added coordination record at commit {commit}: {path}: {error}"));
                    continue;
                }
            }
            match read_git(cwd, &["cat-file", "blob", &format!("{commit}:{path}")]) {
                Ok(bytes) => {
                    for error in validate_added(&bytes) {
                        reasons.push(format!("commit {commit} added record {path}: {error}"));
                    }
                    if let Some(existing) = seen.insert(&path, bytes) {
                        reasons.push(format!("commit {commit} adds a byte-identical copy of an existing coordination record: {path} == {existing}"));
                    }
                }
                Err(error) => reasons.push(format!(
                    "cannot read added coordination record at commit {commit}: {path}: {error}"
                )),
            }
            added_files.push(path);
        }
    }

    // Aggregate cross-check: the net range diff must also be
    // addition-only under the coordination prefix. This catches shapes
    // (e.g. implementation-then-revert-then-coordination) where every
    // individual commit might otherwise look plausible in isolation but
    // the aggregate still touches non-coordination history.
    let aggregate = checked_git!(&["diff", "--name-status", &range]);
    for (status, path) in parse_name_status(&aggregate) {
        if !is_channel_record(&path, &coord_prefix) {
            reasons.push(format!(
                "aggregate range diff touches a non-coordination path: {path}"
            ));
        } else if status != "A" {
            reasons.push(format!(
                "aggregate range diff performs a non-addition ({status}) on coordination path: {path}"
            ));
        }
    }

    added_files.sort();
    added_files.dedup();
    reasons.sort();
    reasons.dedup();

    PublicationCheckReport {
        ok: reasons.is_empty(),
        base_ref: base_ref.to_string(),
        merge_base: Some(merge_base),
        commits_checked: commits,
        added_files,
        reasons,
    }
}

fn is_channel_record(path: &str, prefix: &str) -> bool {
    path.strip_prefix(prefix)
        .is_some_and(|name| !name.contains('/') && name.ends_with(".md"))
}

fn parse_name_status(output: &str) -> Vec<(String, String)> {
    output
        .lines()
        .filter_map(|line| {
            let mut parts = line.splitn(2, '\t');
            let status = parts.next()?.trim();
            let path = parts.next()?.trim();
            if status.is_empty() || path.is_empty() {
                return None;
            }
            // Rename/copy statuses carry a similarity suffix (e.g. "R100");
            // normalise to the leading letter so callers match on it.
            let status = status.chars().next().unwrap_or('?').to_string();
            Some((status, path.to_string()))
        })
        .collect()
}

#[derive(Default)]
struct SeenContent(HashMap<u64, Vec<(String, Vec<u8>)>>);

impl SeenContent {
    fn insert(&mut self, path: &str, bytes: Vec<u8>) -> Option<String> {
        let mut hash = DefaultHasher::new();
        bytes.hash(&mut hash);
        self.insert_bucket(hash.finish(), path, bytes)
    }

    fn insert_bucket(&mut self, hash: u64, path: &str, bytes: Vec<u8>) -> Option<String> {
        let bucket = self.0.entry(hash).or_default();
        if let Some((existing, _)) = bucket.iter().find(|(_, content)| *content == bytes) {
            return Some(existing.clone());
        }
        bucket.push((path.to_string(), bytes));
        None
    }
}

#[cfg(test)]
fn git(cwd: &Path, args: &[&str]) -> Result<String, String> {
    git_bytes(cwd, args).map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
}

fn git_bytes(cwd: &Path, args: &[&str]) -> Result<Vec<u8>, String> {
    let output = Command::new("git")
        .current_dir(cwd)
        .args(args)
        .output()
        .map_err(|err| err.to_string())?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command as StdCommand;

    struct TempRepo {
        dir: tempfile::TempDir,
    }

    impl TempRepo {
        fn new() -> Self {
            let dir = tempfile::tempdir().unwrap();
            run(dir.path(), &["init", "-q", "-b", "main"]);
            run(dir.path(), &["config", "user.email", "test@example.com"]);
            run(dir.path(), &["config", "user.name", "Test"]);
            Self { dir }
        }

        fn path(&self) -> &Path {
            self.dir.path()
        }

        fn write(&self, relative: &str, content: &str) {
            let full = self.path().join(relative);
            std::fs::create_dir_all(full.parent().unwrap()).unwrap();
            std::fs::write(full, content).unwrap();
        }

        fn commit(&self, message: &str) {
            run(self.path(), &["add", "-A"]);
            run(self.path(), &["commit", "-q", "-m", message]);
        }

        fn head(&self) -> String {
            git(self.path(), &["rev-parse", "HEAD"])
                .unwrap()
                .trim()
                .to_string()
        }
    }

    fn run(cwd: &Path, args: &[&str]) {
        let status = StdCommand::new("git")
            .current_dir(cwd)
            .args(args)
            .status()
            .unwrap();
        assert!(status.success(), "git {args:?} failed");
    }

    fn coord_message(id: &str) -> String {
        format!(
            "---\nkind: note\nfrom: coder-tester\nto: all\nid: {id}\ntimestamp: 2026-07-27T00:00:00Z\nimmutable: true\nreferences-to: []\ntopic: t\nrequires-ack: false\n---\n\n# body\n"
        )
    }

    #[test]
    fn schema_callback_receives_introducing_bytes_not_later_head() {
        let repo = TempRepo::new();
        repo.write("README.md", "base");
        repo.commit("chore: base");
        run(repo.path(), &["branch", "target"]);
        repo.write("docs/coordination/a.md", "invalid introduction");
        repo.commit("coordination: introduce");
        repo.write("docs/coordination/a.md", "valid later version");
        repo.commit("coordination: replace");
        let report = publication_check_validated(
            "target",
            "docs/coordination",
            "coordination:",
            repo.path(),
            &|bytes| {
                assert_eq!(bytes, b"invalid introduction");
                vec!["original object rejected".into()]
            },
        );
        assert!(
            report
                .reasons
                .iter()
                .any(|r| r.contains("original object rejected"))
        );
    }

    #[test]
    fn nested_base_records_do_not_supply_channel_copy_candidates() {
        let repo = TempRepo::new();
        repo.write("docs/coordination/nested/a.md", &coord_message("same"));
        repo.commit("chore: base");
        run(repo.path(), &["branch", "target"]);
        repo.write("docs/coordination/b.md", &coord_message("same"));
        repo.commit("coordination: direct record");
        assert!(publication_check("target", "docs/coordination", "coordination:", repo.path()).ok);
    }

    #[test]
    fn copied_merge_base_record_fails_with_both_paths() {
        let repo = TempRepo::new();
        repo.write("docs/coordination/a.md", &coord_message("same-id"));
        repo.commit("chore: base");
        run(repo.path(), &["branch", "target"]);
        repo.write("docs/coordination/b.md", &coord_message("same-id"));
        repo.commit("coordination: copy");
        // Working-tree bytes cannot affect the decision about committed objects.
        repo.write("docs/coordination/a.md", "different dirty source");
        repo.write("docs/coordination/b.md", "different dirty addition");
        let report = publication_check("target", "docs/coordination", "coordination:", repo.path());
        assert!(!report.ok);
        assert!(
            report
                .reasons
                .iter()
                .any(|r| r.contains("docs/coordination/b.md == docs/coordination/a.md"))
        );
    }

    #[test]
    fn duplicate_new_records_fail_within_commit_and_across_range() {
        for separate_commits in [false, true] {
            let repo = TempRepo::new();
            repo.write("README.md", "base");
            repo.commit("chore: base");
            run(repo.path(), &["branch", "target"]);
            repo.write("docs/coordination/b.md", &coord_message("same"));
            if separate_commits {
                repo.commit("coordination: first");
            }
            repo.write("docs/coordination/c.md", &coord_message("same"));
            repo.commit("coordination: second");
            let report =
                publication_check("target", "docs/coordination", "coordination:", repo.path());
            assert!(!report.ok);
            assert!(
                report
                    .reasons
                    .iter()
                    .any(|r| r.contains("docs/coordination/c.md == docs/coordination/b.md"))
            );
        }
    }

    #[test]
    fn template_similar_distinct_record_passes() {
        let repo = TempRepo::new();
        let base = format!(
            "{}\n<!-- SPDX-License-Identifier: MIT -->\n## Evidence\noriginal",
            coord_message("base")
        );
        repo.write("docs/coordination/a.md", &base);
        repo.commit("chore: base");
        run(repo.path(), &["branch", "target"]);
        let distinct = base
            .replace("id: base", "id: distinct")
            .replace("00:00:00", "00:01:00")
            .replace("topic: t", "topic: distinct")
            .replace("original", "new evidence");
        repo.write("docs/coordination/b.md", &distinct);
        repo.commit("coordination: distinct");
        assert!(publication_check("target", "docs/coordination", "coordination:", repo.path()).ok);
    }

    #[test]
    fn divergent_target_copy_rejects_but_distinct_addition_passes() {
        for duplicate in [true, false] {
            let repo = divergent_repo(duplicate);
            let report =
                publication_check("target", "docs/coordination", "coordination:", repo.path());
            assert_eq!(report.ok, !duplicate, "{:?}", report.reasons);
            if duplicate {
                assert!(
                    report
                        .reasons
                        .iter()
                        .any(|r| r.contains("b.md == docs/coordination/a.md"))
                );
            }
        }
    }

    fn divergent_repo(duplicate: bool) -> TempRepo {
        let repo = TempRepo::new();
        repo.write("docs/coordination/base.md", &coord_message("base"));
        repo.commit("chore: base");
        run(repo.path(), &["branch", "candidate"]);
        repo.write("docs/coordination/a.md", &coord_message("target"));
        repo.commit("coordination: target moves");
        run(repo.path(), &["branch", "target"]);
        run(repo.path(), &["checkout", "-q", "candidate"]);
        repo.write(
            "docs/coordination/b.md",
            &coord_message(if duplicate { "target" } else { "candidate" }),
        );
        repo.commit("coordination: candidate");
        repo
    }

    #[test]
    fn target_ref_is_resolved_once_then_only_pinned_objects_are_read() {
        let repo = divergent_repo(true);
        let calls = std::cell::RefCell::new(Vec::<Vec<String>>::new());
        let report = publication_check_with_git(
            "target",
            "docs/coordination",
            "coordination:",
            repo.path(),
            &|cwd, args| {
                calls
                    .borrow_mut()
                    .push(args.iter().map(|s| s.to_string()).collect());
                let result = git_bytes(cwd, args);
                if args[0] == "rev-parse" {
                    // Delete the mutable ref immediately after resolution: any later
                    // ancestry/inventory lookup by its name would now fail.
                    run(cwd, &["branch", "-D", "target"]);
                }
                result
            },
            &|_| Vec::new(),
        );
        assert!(!report.ok);
        assert!(
            report
                .reasons
                .iter()
                .any(|r| r.contains("byte-identical copy"))
        );
        let calls = calls.borrow();
        assert_eq!(calls.iter().filter(|a| a[0] == "rev-parse").count(), 1);
        assert!(
            calls
                .iter()
                .skip(1)
                .all(|a| a.iter().all(|v| v != "target" && v != "target^{commit}"))
        );
        assert!(!report.reasons.iter().any(|r| r.contains("failed")));
    }

    #[test]
    fn every_security_decisive_git_read_fails_closed() {
        let repo = divergent_repo(false);
        let calls = std::cell::RefCell::new(Vec::<Vec<String>>::new());
        let baseline = publication_check_with_git(
            "target",
            "docs/coordination",
            "coordination:",
            repo.path(),
            &|cwd, args| {
                calls
                    .borrow_mut()
                    .push(args.iter().map(|s| s.to_string()).collect());
                git_bytes(cwd, args)
            },
            &|_| Vec::new(),
        );
        assert!(baseline.ok, "{:?}", baseline.reasons);
        for failure in calls.into_inner() {
            let operation = failure.join(" ");
            let report = publication_check_with_git(
                "target",
                "docs/coordination",
                "coordination:",
                repo.path(),
                &|cwd, args| {
                    if args == failure {
                        Err(format!("injected git {operation} failure"))
                    } else {
                        git_bytes(cwd, args)
                    }
                },
                &|_| Vec::new(),
            );
            assert!(!report.ok, "must fail closed: {operation}");
            assert!(
                report
                    .reasons
                    .iter()
                    .any(|r| r.contains(&format!("injected git {operation} failure"))),
                "{:?}",
                report.reasons
            );
        }
    }

    #[test]
    fn enumeration_and_base_and_candidate_blob_errors_fail_closed() {
        let repo = TempRepo::new();
        repo.write("docs/coordination/a.md", &coord_message("a"));
        repo.commit("chore: base");
        run(repo.path(), &["branch", "target"]);
        repo.write("docs/coordination/b.md", &coord_message("b"));
        repo.commit("coordination: add");
        for failure in ["ls-tree", "a.md", "b.md"] {
            let report = publication_check_with_git(
                "target",
                "docs/coordination",
                "coordination:",
                repo.path(),
                &|cwd, args| {
                    if args[0] == failure || (args[0] == "cat-file" && args[2].ends_with(failure)) {
                        Err(format!("injected {failure} failure"))
                    } else {
                        git_bytes(cwd, args)
                    }
                },
                &|_| Vec::new(),
            );
            assert!(!report.ok, "{failure} must fail closed");
            assert!(
                report
                    .reasons
                    .iter()
                    .any(|r| r.contains(&format!("injected {failure} failure")))
            );
        }
    }

    #[test]
    fn hashes_are_only_an_index_and_raw_bytes_decide() {
        let mut seen = SeenContent::default();
        assert_eq!(seen.insert_bucket(1, "a", vec![0xff]), None);
        assert_eq!(seen.insert_bucket(1, "b", vec![0xfe]), None);
        assert_eq!(seen.insert_bucket(1, "c", vec![0xff]), Some("a".into()));
    }

    #[test]
    fn clean_coordination_only_range_passes() {
        let repo = TempRepo::new();
        repo.write("docs/coordination/base.md", "# base\n");
        repo.commit("chore: base");
        let base = repo.head();
        run(repo.path(), &["branch", "target", &base]);

        repo.write(
            "docs/coordination/20260727T000000Z-a.md",
            &coord_message("20260727T000000Z-a"),
        );
        repo.commit("coordination: add a");

        let report = publication_check("target", "docs/coordination", "coordination:", repo.path());
        assert!(
            report.ok,
            "expected pass, got reasons: {:?}",
            report.reasons
        );
        assert_eq!(
            report.added_files,
            vec!["docs/coordination/20260727T000000Z-a.md"]
        );
    }

    #[test]
    fn multi_record_coordination_only_range_passes() {
        let repo = TempRepo::new();
        repo.write("docs/coordination/base.md", "# base\n");
        repo.commit("chore: base");
        let base = repo.head();
        run(repo.path(), &["branch", "target", &base]);

        repo.write(
            "docs/coordination/20260727T000010Z-a.md",
            &coord_message("20260727T000010Z-a"),
        );
        repo.write(
            "docs/coordination/20260727T000011Z-b.md",
            &coord_message("20260727T000011Z-b"),
        );
        repo.commit("coordination: add same-cycle records");

        let report = publication_check("target", "docs/coordination", "coordination:", repo.path());
        assert!(
            report.ok,
            "expected multi-record range to pass, got: {:?}",
            report.reasons
        );
        assert_eq!(
            report.added_files,
            vec![
                "docs/coordination/20260727T000010Z-a.md",
                "docs/coordination/20260727T000011Z-b.md",
            ]
        );
    }

    #[test]
    fn multi_record_batch_with_non_coordination_file_fails() {
        let repo = TempRepo::new();
        repo.write("README.md", "base\n");
        repo.commit("chore: base");
        let base = repo.head();
        run(repo.path(), &["branch", "target", &base]);

        repo.write(
            "docs/coordination/20260727T000020Z-a.md",
            &coord_message("20260727T000020Z-a"),
        );
        repo.write(
            "docs/coordination/20260727T000021Z-b.md",
            &coord_message("20260727T000021Z-b"),
        );
        repo.write("README.md", "changed\n");
        repo.commit("coordination: smuggle a non-coordination change");

        let report = publication_check("target", "docs/coordination", "coordination:", repo.path());
        assert!(!report.ok);
        assert!(
            report
                .reasons
                .iter()
                .any(|reason| reason.contains("README.md")),
            "expected a reason naming README.md, got: {:?}",
            report.reasons
        );
    }

    #[test]
    fn mr692_shape_fails_even_though_tip_diff_looks_coordination_only() {
        let repo = TempRepo::new();
        repo.write("README.md", "base\n");
        repo.commit("chore: base");
        let base = repo.head();
        run(repo.path(), &["branch", "target", &base]);

        // Implementation commit -- the branch is rooted on this, exactly
        // like MR !692 was rooted on issue #68's implementation tip.
        repo.write("src/roi.rs", "fn roi() {}\n");
        repo.commit("fix: isolate token roi logs");

        // Tip commit touches only coordination -- this is what made !692's
        // *tip* diff look coordination-only.
        repo.write(
            "docs/coordination/20260727T000100Z-b.md",
            &coord_message("20260727T000100Z-b"),
        );
        repo.commit("coordination: hand off for gate 2");

        let report = publication_check("target", "docs/coordination", "coordination:", repo.path());
        assert!(!report.ok);
        assert!(
            report.reasons.iter().any(|r| r.contains("src/roi.rs")),
            "expected a reason naming the implementation path, got: {:?}",
            report.reasons
        );
    }

    #[test]
    fn implementation_then_revert_then_coordination_fails() {
        let repo = TempRepo::new();
        repo.write("README.md", "base\n");
        repo.commit("chore: base");
        let base = repo.head();
        run(repo.path(), &["branch", "target", &base]);

        repo.write("src/roi.rs", "fn roi() {}\n");
        repo.commit("fix: isolate token roi logs");

        // Revert: the aggregate diff for roi.rs nets to zero, but the
        // commit is still in the range.
        std::fs::remove_file(repo.path().join("src/roi.rs")).unwrap();
        repo.commit("revert: isolate token roi logs");

        repo.write(
            "docs/coordination/20260727T000200Z-c.md",
            &coord_message("20260727T000200Z-c"),
        );
        repo.commit("coordination: hand off for gate 2");

        let report = publication_check("target", "docs/coordination", "coordination:", repo.path());
        assert!(!report.ok);
        assert!(
            report.reasons.iter().any(|r| r.contains("src/roi.rs")),
            "expected a per-commit reason naming src/roi.rs even though the aggregate diff nets to zero for it, got: {:?}",
            report.reasons
        );
    }

    #[test]
    fn merge_commit_in_coordination_range_fails() {
        let repo = TempRepo::new();
        repo.write("README.md", "base\n");
        repo.commit("chore: base");
        let base = repo.head();
        run(repo.path(), &["branch", "target", &base]);

        run(repo.path(), &["checkout", "-q", "-b", "side"]);
        repo.write(
            "docs/coordination/20260727T000210Z-a.md",
            &coord_message("20260727T000210Z-a"),
        );
        repo.commit("coordination: add side record");

        run(
            repo.path(),
            &["checkout", "-q", "-b", "candidate", "target"],
        );
        repo.write(
            "docs/coordination/20260727T000211Z-b.md",
            &coord_message("20260727T000211Z-b"),
        );
        repo.commit("coordination: add candidate record");
        run(
            repo.path(),
            &[
                "merge",
                "--no-ff",
                "-q",
                "side",
                "-m",
                "coordination: merge imported history",
            ],
        );

        let report = publication_check("target", "docs/coordination", "coordination:", repo.path());
        assert!(!report.ok);
        assert!(
            report
                .reasons
                .iter()
                .any(|reason| reason.contains("merge commit")),
            "expected a merge-commit reason, got: {:?}",
            report.reasons
        );
    }

    #[test]
    fn modification_of_existing_coordination_file_fails() {
        let repo = TempRepo::new();
        repo.write("docs/coordination/base.md", &coord_message("base"));
        repo.commit("chore: base");
        let base = repo.head();
        run(repo.path(), &["branch", "target", &base]);

        repo.write("docs/coordination/base.md", &coord_message("base-edited"));
        repo.commit("coordination: edit base");

        let report = publication_check("target", "docs/coordination", "coordination:", repo.path());
        assert!(!report.ok);
        assert!(report.reasons.iter().any(|r| r.contains("non-addition")));
    }

    #[test]
    fn deletion_of_existing_coordination_file_fails() {
        let repo = TempRepo::new();
        repo.write("docs/coordination/base.md", &coord_message("base"));
        repo.commit("chore: base");
        let base = repo.head();
        run(repo.path(), &["branch", "target", &base]);

        std::fs::remove_file(repo.path().join("docs/coordination/base.md")).unwrap();
        repo.commit("coordination: delete base");

        let report = publication_check("target", "docs/coordination", "coordination:", repo.path());
        assert!(!report.ok);
        assert!(report.reasons.iter().any(|r| r.contains("non-addition")));
    }

    #[test]
    fn rename_of_existing_coordination_file_fails() {
        let repo = TempRepo::new();
        repo.write("docs/coordination/base.md", &coord_message("base"));
        repo.commit("chore: base");
        let base = repo.head();
        run(repo.path(), &["branch", "target", &base]);

        run(
            repo.path(),
            &[
                "mv",
                "docs/coordination/base.md",
                "docs/coordination/renamed.md",
            ],
        );
        repo.commit("coordination: rename base");

        let report = publication_check("target", "docs/coordination", "coordination:", repo.path());
        assert!(!report.ok);
        assert!(report.reasons.iter().any(|r| r.contains("non-addition")));
    }

    #[test]
    fn ci_skip_marker_in_commit_message_fails() {
        for marker in ["[ci skip]", "[skip ci]"] {
            let repo = TempRepo::new();
            repo.write("docs/coordination/base.md", "# base\n");
            repo.commit("chore: base");
            let base = repo.head();
            run(repo.path(), &["branch", "target", &base]);

            repo.write(
                "docs/coordination/20260727T000500Z-f.md",
                &coord_message("20260727T000500Z-f"),
            );
            repo.commit(&format!("coordination: add f {marker}"));

            let report =
                publication_check("target", "docs/coordination", "coordination:", repo.path());
            assert!(!report.ok);
            assert!(
                report.reasons.iter().any(|r| r.contains("ci-skip marker")),
                "expected a ci-skip-marker reason, got: {:?}",
                report.reasons
            );
        }
    }

    #[test]
    fn wrong_subject_prefix_fails_even_with_coordination_only_paths() {
        let repo = TempRepo::new();
        repo.write("docs/coordination/base.md", "# base\n");
        repo.commit("chore: base");
        let base = repo.head();
        run(repo.path(), &["branch", "target", &base]);

        // Path/status containment is satisfied (pure addition under
        // docs/coordination/), but the commit subject does not use the
        // protected fast-path convention.
        repo.write(
            "docs/coordination/20260727T000400Z-e.md",
            &coord_message("20260727T000400Z-e"),
        );
        repo.commit("chore: add e");

        let report = publication_check("target", "docs/coordination", "coordination:", repo.path());
        assert!(!report.ok);
        assert!(
            report
                .reasons
                .iter()
                .any(|r| r.contains("protected fast-path convention")),
            "expected a subject-prefix reason, got: {:?}",
            report.reasons
        );
    }

    #[test]
    fn unresolvable_base_fails_closed() {
        let repo = TempRepo::new();
        repo.write("docs/coordination/a.md", &coord_message("a"));
        repo.commit("coordination: add a");

        let report = publication_check(
            "does-not-exist",
            "docs/coordination",
            "coordination:",
            repo.path(),
        );
        assert!(!report.ok);
        assert!(report.merge_base.is_none());
    }
}
