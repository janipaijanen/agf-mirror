// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jani Päijänen
use std::{fs, path::PathBuf, process::Command};

const ID: &str = "20260911T000000Z-planner-architect-a001";
const FILE: &str = "docs/coordination/20260911T000000Z-planner-architect-a001.md";

#[test]
fn repeated_invalid_sets_keep_ref_attribution_without_repeating_every_error() {
    let r = Repo::new();
    r.git(&["checkout", "-b", "invalid"]);
    r.write("docs/coordination/bad.md", "invalid");
    let head = r.commit();
    for index in 0..20 {
        r.git(&[
            "update-ref",
            &format!("refs/remotes/origin/copy-{index}"),
            &head,
        ]);
    }
    r.git(&["checkout", "main"]);
    let out = r.run(&["--include-branch-local"]);
    let text = String::from_utf8(out.stdout).unwrap();
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(
        text.matches("invalid candidate docs/coordination/bad.md")
            .count(),
        1
    );
    for index in 0..20 {
        assert!(text.contains(&format!("refs/remotes/origin/copy-{index} @ {head}")));
    }
    assert!(text.contains("partial_coverage: true"));
}

struct Repo(tempfile::TempDir);
impl Repo {
    fn new() -> Self {
        let r = Self(tempfile::tempdir().unwrap());
        r.git(&["init", "-b", "main"]);
        r.git(&["config", "user.email", "test@example.invalid"]);
        r.git(&["config", "user.name", "Test"]);
        r.write("docs/coordination/.keep", "");
        r.commit();
        r
    }
    fn git(&self, args: &[&str]) -> String {
        let out = Command::new("git")
            .args(args)
            .current_dir(self.0.path())
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "{args:?}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8(out.stdout).unwrap().trim().into()
    }
    fn write(&self, path: &str, content: &str) {
        let p = self.0.path().join(path);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, content).unwrap();
    }
    fn commit(&self) -> String {
        self.git(&["add", "."]);
        self.git(&["commit", "-m", "fixture"]);
        self.git(&["rev-parse", "HEAD"])
    }
    fn run(&self, extra: &[&str]) -> std::process::Output {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(4)
            .unwrap()
            .to_path_buf();
        Command::new(env!("CARGO_BIN_EXE_agov"))
            .args(["handoff", "next", "coder-tester"])
            .args(extra)
            .env("AGOV_SCHEMA_DIR", root.join("agf/schemas"))
            .env("AGOV_TOKEN_ROI", "0")
            .current_dir(self.0.path())
            .output()
            .unwrap()
    }
    fn report(&self) -> String {
        let out = self.run(&["--include-branch-local"]);
        assert_eq!(
            out.status.code(),
            Some(0),
            "{}\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8(out.stdout).unwrap()
    }
}

fn handoff(topic: &str) -> String {
    format!("---\nid: {ID}\nkind: handoff\nfrom: planner-architect\nto: coder-tester\ntimestamp: 2026-09-11T00:00:00Z\nimmutable: true\nreferences-to: []\ntopic: {topic}\nrequires-ack: true\nartifacts_owned: []\nmodel: test\nhandoff:\n  next_agent: coder-tester\n  status: ready\n  task: [{topic}]\n  must_read: []\n  do_not_touch: []\n---\n\nBody.\n")
}

fn ack() -> String {
    format!("---\nid: 20260911T000100Z-coder-tester-a002\nkind: ack\nfrom: coder-tester\nto: planner-architect\ntimestamp: 2026-09-11T00:01:00Z\nimmutable: true\nreferences-to: [{ID}]\ntopic: ack\nrequires-ack: false\nartifacts_owned: []\nmodel: test\n---\nAcknowledged.\n")
}

#[test]
fn default_is_filesystem_only_and_exact_output_is_preserved() {
    let r = Repo::new();
    // No repository remains: an accidental git access cannot succeed.
    fs::remove_dir_all(r.0.path().join(".git")).unwrap();
    let out = r.run(&[]);
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(out.stdout, b"No handoff found for role: coder-tester\n");
    assert!(out.stderr.is_empty());
    r.write(FILE, &handoff("local"));
    let out = r.run(&[]);
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(
        String::from_utf8(out.stdout).unwrap(),
        format!(
            "Handoff for: coder-tester\nStatus:      ready\nTask:\n  - local\nSource: {ID}.md\n"
        )
    );
}

#[test]
fn branch_only_and_different_commit_blob_dedup_with_ref_priority() {
    let r = Repo::new();
    r.git(&["checkout", "-b", "local"]);
    r.write(FILE, &handoff("branch"));
    let local_sha = r.commit();
    r.write("unrelated", "different commit");
    let other_sha = r.commit();
    r.git(&["update-ref", "refs/remotes/origin/other", &other_sha]);
    r.git(&["reset", "--hard", &local_sha]);
    r.git(&["checkout", "main"]);
    assert_eq!(r.run(&[]).status.code(), Some(2));
    let report = r.report();
    assert_eq!(report.matches("Candidate:").count(), 1);
    assert!(report.contains(&format!("source: refs/heads/local | commit: {local_sha}")));
    assert!(report.contains("partial_coverage: false"));
}

#[test]
fn each_ref_resolves_only_its_own_complete_message_set() {
    let r = Repo::new();
    r.git(&["checkout", "-b", "pending"]);
    r.write(FILE, &handoff("pending"));
    r.commit();
    r.git(&["checkout", "-b", "resolved"]);
    r.write(
        "docs/coordination/20260911T000100Z-coder-tester-a002.md",
        &ack(),
    );
    r.commit();
    r.git(&["checkout", "main"]);
    r.git(&["checkout", "-b", "ack-only"]);
    r.write(
        "docs/coordination/20260911T000100Z-coder-tester-a002.md",
        &ack(),
    );
    r.commit();
    r.git(&["checkout", "main"]);
    let report = r.report();
    assert_eq!(report.matches("Candidate:").count(), 1);
    assert!(report.contains("source: refs/heads/pending"));
    assert!(!report.contains("source: refs/heads/resolved"));
    assert!(!report.contains("source: refs/heads/ack-only"));
}

#[test]
fn filesystem_raw_snapshot_dedup_conflicts_and_primary_stay_separate() {
    let r = Repo::new();
    r.git(&["checkout", "-b", "committed"]);
    let content = handoff("committed");
    r.write(FILE, &content);
    r.commit();
    r.git(&["checkout", "main"]);
    r.write(FILE, &content);
    let report = r.report();
    assert_eq!(report.matches("Candidate:").count(), 1);
    assert!(report.contains("source: working tree | commit: working tree"));
    // Filters would normalize these different bytes to the committed blob.
    r.write(".gitattributes", "*.md text eol=lf\n");
    r.write(FILE, &content.replace("Body.\n", "Body.\r\n"));
    let report = r.report();
    assert_eq!(report.matches("Candidate:").count(), 2);
    assert_eq!(report.matches("CONFLICT:").count(), 2);
    assert!(report.contains("source: working tree"));
    assert!(report.contains("source: refs/heads/committed"));
    r.write(FILE, &handoff("uncommitted"));
    let report = r.report();
    let primary = report
        .split("Advisory pending source inventory")
        .next()
        .unwrap();
    assert!(primary.contains("  - uncommitted"));
    assert!(!primary.contains("  - committed"));
}

#[test]
fn divergent_refs_conflict_and_session_annotation_survives() {
    let r = Repo::new();
    r.git(&["checkout", "-b", "one"]);
    r.write(
        FILE,
        &handoff("one").replace("  status: ready", "  for_session: claude\n  status: ready"),
    );
    let one = r.commit();
    r.git(&["checkout", "-b", "two"]);
    r.write(FILE, &handoff("two"));
    let two = r.commit();
    r.git(&["checkout", "main"]);
    let out = r.run(&["--include-branch-local", "--session", "codex"]);
    let report = String::from_utf8(out.stdout).unwrap();
    assert_eq!(out.status.code(), Some(0), "{report}");
    assert_eq!(report.matches("CONFLICT:").count(), 2);
    assert!(report.contains(&one) && report.contains(&two));
    assert!(report.contains("Addressed elsewhere: claude"));
}

#[test]
fn malformed_and_corrupt_refs_mark_partial_but_keep_good_evidence() {
    let r = Repo::new();
    r.git(&["checkout", "-b", "bad"]);
    r.write(
        "docs/coordination/malformed.md",
        "---\nkind: nonsense\n---\n",
    );
    r.commit();
    r.git(&["checkout", "main"]);
    r.git(&["checkout", "-b", "good"]);
    r.write(FILE, &handoff("branch"));
    r.commit();
    r.git(&["checkout", "main"]);
    r.write(FILE, &handoff("working"));
    // A syntactically valid ref whose object has disappeared.
    r.write(
        ".git/refs/heads/corrupt",
        "1111111111111111111111111111111111111111\n",
    );
    let out = r.run(&["--include-branch-local"]);
    let report = String::from_utf8(out.stdout).unwrap();
    assert_eq!(out.status.code(), Some(1), "{report}");
    assert!(report.contains("partial_coverage: true"));
    assert!(report.contains("invalid candidate") && report.contains("malformed.md"));
    assert!(report.contains("refs/heads/corrupt"));
    assert!(report.contains("source: working tree"));
    assert!(report.contains("source: refs/heads/good"));
}
