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
    publication_check_with_git(base_ref, coord_dir, subject_prefix, cwd, &git_bytes)
}

fn publication_check_with_git(
    base_ref: &str,
    coord_dir: &str,
    subject_prefix: &str,
    cwd: &Path,
    read_git: &impl Fn(&Path, &[&str]) -> Result<Vec<u8>, String>,
) -> PublicationCheckReport {
    let git = |cwd: &Path, args: &[&str]| {
        read_git(cwd, args).map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
    };
    let mut reasons = Vec::new();
    let coord_prefix = format!("{}/", coord_dir.trim_end_matches('/'));

    let merge_base = match git(cwd, &["merge-base", base_ref, "HEAD"]) {
        Ok(sha) if !sha.trim().is_empty() => sha.trim().to_string(),
        _ => {
            return PublicationCheckReport {
                ok: false,
                base_ref: base_ref.to_string(),
                merge_base: None,
                commits_checked: Vec::new(),
                added_files: Vec::new(),
                reasons: vec![format!(
                    "no valid merge-base between '{base_ref}' and HEAD; \
                     target may be unfetched or ancestry is unresolvable"
                )],
            };
        }
    };

    let range = format!("{merge_base}..HEAD");

    let merge_commits = git(cwd, &["rev-list", "--merges", &range]).unwrap_or_default();
    for line in merge_commits.lines().filter(|line| !line.trim().is_empty()) {
        reasons.push(format!(
            "merge commit {line} present in the candidate range; merge/imported history is rejected"
        ));
    }

    let commits: Vec<String> = git(cwd, &["rev-list", "--reverse", &range])
        .unwrap_or_default()
        .lines()
        .map(str::to_string)
        .filter(|line| !line.is_empty())
        .collect();

    if commits.is_empty() {
        reasons.push("no commits in range; nothing to publish".to_string());
    }

    for commit in &commits {
        let subject = git(cwd, &["log", "-1", "--format=%s", commit])
            .unwrap_or_default()
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
        let full_message = git(cwd, &["log", "-1", "--format=%B", commit]).unwrap_or_default();
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
    let base_records = read_git(
        cwd,
        &[
            "ls-tree",
            "-r",
            "-z",
            "--name-only",
            &merge_base,
            "--",
            coord_dir,
        ],
    );
    match base_records {
        Err(error) => reasons.push(format!(
            "cannot enumerate merge-base coordination records: {error}"
        )),
        Ok(paths) => {
            for raw_path in paths
                .split(|byte| *byte == 0)
                .filter(|path| !path.is_empty())
            {
                let path = match std::str::from_utf8(raw_path) {
                    Ok(path) => path,
                    Err(error) => {
                        reasons.push(format!(
                            "cannot decode merge-base coordination path: {error}"
                        ));
                        continue;
                    }
                };
                if !path.starts_with(&coord_prefix) || !path.ends_with(".md") {
                    continue;
                }
                match read_git(cwd, &["cat-file", "blob", &format!("{merge_base}:{path}")]) {
                    Ok(bytes) => {
                        seen.insert(path, bytes);
                    }
                    Err(error) => reasons.push(format!(
                        "cannot read merge-base coordination record {path}: {error}"
                    )),
                }
            }
        }
    }

    for commit in &commits {
        let name_status = git(
            cwd,
            &["diff-tree", "--no-commit-id", "--name-status", "-r", commit],
        )
        .unwrap_or_default();
        for (status, path) in parse_name_status(&name_status) {
            if !path.starts_with(&coord_prefix) || !path.ends_with(".md") {
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
            match read_git(cwd, &["cat-file", "blob", &format!("{commit}:{path}")]) {
                Ok(bytes) => {
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
    let aggregate = git(cwd, &["diff", "--name-status", &range]).unwrap_or_default();
    for (status, path) in parse_name_status(&aggregate) {
        if !path.starts_with(&coord_prefix) || !path.ends_with(".md") {
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
        assert!(report
            .reasons
            .iter()
            .any(|r| r.contains("docs/coordination/b.md == docs/coordination/a.md")));
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
            assert!(report
                .reasons
                .iter()
                .any(|r| r.contains("docs/coordination/c.md == docs/coordination/b.md")));
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
    fn target_tip_record_is_not_a_merge_base_record() {
        let repo = TempRepo::new();
        repo.write("README.md", "base");
        repo.commit("chore: base");
        run(repo.path(), &["branch", "candidate"]);
        repo.write("docs/coordination/a.md", &coord_message("same"));
        repo.commit("coordination: target moves");
        run(repo.path(), &["branch", "target"]);
        run(repo.path(), &["checkout", "-q", "candidate"]);
        repo.write("docs/coordination/b.md", &coord_message("same"));
        repo.commit("coordination: candidate");
        assert!(publication_check("target", "docs/coordination", "coordination:", repo.path()).ok);
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
            );
            assert!(!report.ok, "{failure} must fail closed");
            assert!(report
                .reasons
                .iter()
                .any(|r| r.contains(&format!("injected {failure} failure"))));
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
        let repo = TempRepo::new();
        repo.write("docs/coordination/base.md", "# base\n");
        repo.commit("chore: base");
        let base = repo.head();
        run(repo.path(), &["branch", "target", &base]);

        repo.write(
            "docs/coordination/20260727T000500Z-f.md",
            &coord_message("20260727T000500Z-f"),
        );
        repo.commit("coordination: add f [ci skip]");

        let report = publication_check("target", "docs/coordination", "coordination:", repo.path());
        assert!(!report.ok);
        assert!(
            report.reasons.iter().any(|r| r.contains("ci-skip marker")),
            "expected a ci-skip-marker reason, got: {:?}",
            report.reasons
        );
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
