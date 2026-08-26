use serde_json::Value;
use sha2::{Digest, Sha256};
use std::cell::Cell;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const SERIES: &str = "20260825T010000Z-coder-tester-s001";

#[test]
fn two_blocks_require_reset_and_invariants_remain_independent() {
    let fixture = Fixture::new();
    let c1 = fixture.candidate(
        "c001",
        1,
        &fixture.sha1,
        &fixture.digest1,
        &["INV-A", "INV-B"],
    );
    fixture.review(
        "r001",
        1,
        &c1,
        &fixture.sha1,
        &fixture.digest1,
        &["INV-A", "INV-B"],
        "block",
    );
    let c2 = fixture.candidate("c002", 2, &fixture.sha2, &fixture.digest2, &["INV-A"]);
    fixture.review(
        "r002",
        2,
        &c2,
        &fixture.sha2,
        &fixture.digest2,
        &["INV-A"],
        "block",
    );

    let report = fixture.json_report();
    assert_eq!(report["status"], "ok");
    assert_invariant(&report, "INV-A", "reset-required", 2, "abstraction-reset");
    assert_invariant(&report, "INV-B", "open", 1, "candidate");
}

#[test]
fn unchanged_digest_transport_and_exact_duplicate_review_count_once() {
    let fixture = Fixture::new();
    let _original = fixture.candidate("c101", 1, &fixture.sha1, &fixture.digest1, &["INV-A"]);
    fixture.ordinary("n101");
    let rebased = fixture.candidate("c102", 1, &fixture.sha1b, &fixture.digest1, &["INV-A"]);
    fixture.review(
        "r101",
        1,
        &rebased,
        &fixture.sha1b,
        &fixture.digest1,
        &["INV-A"],
        "clear",
    );
    fixture.review(
        "r102",
        1,
        &rebased,
        &fixture.sha1b,
        &fixture.digest1,
        &["INV-A"],
        "clear",
    );

    let report = fixture.json_report();
    assert_eq!(report["status"], "ok");
    assert_invariant(&report, "INV-A", "closed", 0, "none");
    assert_eq!(
        invariant(&report, "INV-A")["canonical_candidate"]["event_id"],
        rebased
    );
}

#[test]
fn post_reset_clear_closes_series() {
    let fixture = Fixture::new();
    fixture.two_failures();
    fixture.reset("x201", "redesign");
    let c3 = fixture.candidate("c203", 3, &fixture.sha3, &fixture.digest3, &["INV-A"]);
    fixture.review(
        "r203",
        3,
        &c3,
        &fixture.sha3,
        &fixture.digest3,
        &["INV-A"],
        "clear",
    );

    let report = fixture.json_report();
    assert_eq!(report["status"], "ok");
    assert_invariant(&report, "INV-A", "closed", 0, "none");
}

#[test]
fn post_reset_block_requires_one_governor_return() {
    let fixture = Fixture::new();
    fixture.two_failures();
    fixture.reset("x301", "redesign");
    let c3 = fixture.candidate("c303", 3, &fixture.sha3, &fixture.digest3, &["INV-A"]);
    fixture.review(
        "r303",
        3,
        &c3,
        &fixture.sha3,
        &fixture.digest3,
        &["INV-A"],
        "block",
    );

    let before = fixture.json_report();
    assert_invariant(
        &before,
        "INV-A",
        "governor-return-required",
        1,
        "governor-disposition",
    );

    fixture.governor("g301", "defer");
    let after = fixture.json_report();
    assert_invariant(&after, "INV-A", "closed", 1, "none");
}

#[test]
fn conflicting_clear_and_block_pause_without_tool_selected_winner() {
    let fixture = Fixture::new();
    let candidate = fixture.candidate("c401", 1, &fixture.sha1, &fixture.digest1, &["INV-A"]);
    fixture.review(
        "r401",
        1,
        &candidate,
        &fixture.sha1,
        &fixture.digest1,
        &["INV-A"],
        "clear",
    );
    fixture.review(
        "r402",
        1,
        &candidate,
        &fixture.sha1,
        &fixture.digest1,
        &["INV-A"],
        "block",
    );

    let output = fixture.run(&[
        "coord",
        "convergence",
        "--series",
        SERIES,
        "--format",
        "json",
    ]);
    assert_eq!(output.status.code(), Some(1));
    let report: Value = serde_json::from_slice(&output.stdout).expect("json report");
    assert_eq!(report["status"], "conflict");
    assert_invariant(&report, "INV-A", "conflict", 0, "governor-resolution");
}

#[test]
fn invalid_digest_and_invalid_transition_are_blocking() {
    let fixture = Fixture::new();
    fixture.candidate("c501", 1, &fixture.sha1, &"0".repeat(64), &["INV-A"]);
    fixture.reset("x501", "redesign");

    let output = fixture.run(&[
        "coord",
        "convergence",
        "--series",
        SERIES,
        "--format",
        "json",
    ]);
    assert_eq!(output.status.code(), Some(1));
    let report: Value = serde_json::from_slice(&output.stdout).expect("json report");
    assert_eq!(report["status"], "invalid");
    let errors = report["errors"].as_array().expect("errors");
    assert!(errors
        .iter()
        .any(|error| error.as_str().unwrap().contains("scope_digest mismatch")));
    assert!(errors
        .iter()
        .any(|error| error.as_str().unwrap().contains("unknown invariant")));
}

#[test]
fn schema_requires_block_materiality_and_accepts_historical_message() {
    let fixture = Fixture::new();
    let historical = fixture.ordinary("n601");
    let historical_output = fixture.run(&["coord", "validate", historical.to_str().unwrap()]);
    assert_eq!(historical_output.status.code(), Some(0));

    let candidate = fixture.candidate("c601", 1, &fixture.sha1, &fixture.digest1, &["INV-A"]);
    let invalid = fixture.write_raw(
        "r601",
        &format!(
            "convergence:\n  series_id: {SERIES}\n  event: targeted-review\n  attempt: 1\n  invariant_ids: [INV-A]\n  candidate_event: {candidate}\n  candidate_sha: {}\n  scope_paths: [scope.txt]\n  scope_digest: {}\n  outcome: block\n",
            fixture.sha1, fixture.digest1
        ),
    );
    let output = fixture.run(&["coord", "validate", invalid.to_str().unwrap()]);
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("materiality_basis"));
    assert!(stderr.contains("authority_ref"));
    assert!(stderr.contains("evidence_refs"));
}

#[test]
fn text_and_json_report_the_same_state_and_counts() {
    let fixture = Fixture::new();
    fixture.two_failures();
    let json = fixture.json_report();
    let text = fixture.run(&[
        "coord",
        "convergence",
        "--series",
        SERIES,
        "--format",
        "text",
    ]);
    assert_eq!(text.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&text.stdout);
    let inv = invariant(&json, "INV-A");
    assert!(stdout.contains(&format!("state: {}", inv["state"].as_str().unwrap())));
    assert!(stdout.contains(&format!(
        "consecutive-semantic-failures: {}",
        inv["consecutive_semantic_failures"].as_u64().unwrap()
    )));
    assert!(stdout.contains(&format!(
        "required-next-event: {}",
        inv["required_next_event"].as_str().unwrap()
    )));
}

struct Fixture {
    root: tempfile::TempDir,
    coord: PathBuf,
    next: Cell<u32>,
    sha1: String,
    sha1b: String,
    sha2: String,
    sha3: String,
    digest1: String,
    digest2: String,
    digest3: String,
}

impl Fixture {
    fn new() -> Self {
        let root = tempfile::tempdir().expect("tempdir");
        git(root.path(), &["init"]);
        git(
            root.path(),
            &["config", "user.email", "test@example.invalid"],
        );
        git(root.path(), &["config", "user.name", "Test User"]);
        let coord = root.path().join("coord");
        fs::create_dir_all(&coord).expect("coord dir");

        write(root.path(), "scope.txt", "version one\n");
        git(root.path(), &["add", "."]);
        git(root.path(), &["commit", "-m", "scope version one"]);
        let sha1 = git_stdout(root.path(), &["rev-parse", "HEAD"]);
        let digest1 = digest("scope.txt", b"version one\n");

        write(root.path(), "transport.txt", "ancestry only\n");
        git(root.path(), &["add", "."]);
        git(root.path(), &["commit", "-m", "transport only"]);
        let sha1b = git_stdout(root.path(), &["rev-parse", "HEAD"]);

        write(root.path(), "scope.txt", "version two\n");
        git(root.path(), &["add", "."]);
        git(root.path(), &["commit", "-m", "scope version two"]);
        let sha2 = git_stdout(root.path(), &["rev-parse", "HEAD"]);
        let digest2 = digest("scope.txt", b"version two\n");

        write(root.path(), "scope.txt", "version three\n");
        git(root.path(), &["add", "."]);
        git(root.path(), &["commit", "-m", "scope version three"]);
        let sha3 = git_stdout(root.path(), &["rev-parse", "HEAD"]);
        let digest3 = digest("scope.txt", b"version three\n");

        Self {
            root,
            coord,
            next: Cell::new(0),
            sha1,
            sha1b,
            sha2,
            sha3,
            digest1,
            digest2,
            digest3,
        }
    }

    fn candidate(
        &self,
        suffix: &str,
        attempt: u64,
        sha: &str,
        scope_digest: &str,
        invariants: &[&str],
    ) -> String {
        let id = self.event_id(suffix);
        self.write_message(
            &id,
            &format!(
                "convergence:\n  series_id: {SERIES}\n  event: candidate\n  attempt: {attempt}\n  invariant_ids: [{}]\n  candidate_sha: {sha}\n  scope_paths: [scope.txt]\n  scope_digest: {scope_digest}\n",
                invariants.join(", ")
            ),
        );
        id
    }

    #[allow(clippy::too_many_arguments)]
    fn review(
        &self,
        suffix: &str,
        attempt: u64,
        candidate_event: &str,
        sha: &str,
        scope_digest: &str,
        invariants: &[&str],
        outcome: &str,
    ) {
        let id = self.event_id(suffix);
        let block_fields = if outcome == "block" {
            "  materiality_basis: mandatory-acceptance-criterion\n  authority_ref: docs/plans/test.md\n  evidence_refs: [test::evidence]\n"
        } else {
            ""
        };
        self.write_message(
            &id,
            &format!(
                "convergence:\n  series_id: {SERIES}\n  event: targeted-review\n  attempt: {attempt}\n  invariant_ids: [{}]\n  candidate_event: {candidate_event}\n  candidate_sha: {sha}\n  scope_paths: [scope.txt]\n  scope_digest: {scope_digest}\n  outcome: {outcome}\n{block_fields}",
                invariants.join(", ")
            ),
        );
    }

    fn reset(&self, suffix: &str, disposition: &str) {
        let id = self.event_id(suffix);
        let authority = if disposition == "redesign" {
            ""
        } else {
            "  authority_ref: governor-decision-test\n"
        };
        self.write_message(
            &id,
            &format!(
                "convergence:\n  series_id: {SERIES}\n  event: abstraction-reset\n  invariant_ids: [INV-A]\n  disposition: {disposition}\n{authority}"
            ),
        );
    }

    fn governor(&self, suffix: &str, disposition: &str) {
        let id = self.event_id(suffix);
        self.write_message(
            &id,
            &format!(
                "convergence:\n  series_id: {SERIES}\n  event: governor-disposition\n  invariant_ids: [INV-A]\n  disposition: {disposition}\n  authority_ref: governor-decision-test\n"
            ),
        );
    }

    fn ordinary(&self, suffix: &str) -> PathBuf {
        self.write_raw(suffix, "")
    }

    fn two_failures(&self) {
        let c1 = self.candidate("c201", 1, &self.sha1, &self.digest1, &["INV-A"]);
        self.review(
            "r201",
            1,
            &c1,
            &self.sha1,
            &self.digest1,
            &["INV-A"],
            "block",
        );
        let c2 = self.candidate("c202", 2, &self.sha2, &self.digest2, &["INV-A"]);
        self.review(
            "r202",
            2,
            &c2,
            &self.sha2,
            &self.digest2,
            &["INV-A"],
            "block",
        );
    }

    fn write_raw(&self, suffix: &str, convergence: &str) -> PathBuf {
        let id = self.event_id(suffix);
        self.write_message(&id, convergence)
    }

    fn event_id(&self, suffix: &str) -> String {
        let ordinal = self.next.get() + 1;
        self.next.set(ordinal);
        format!("20260825T01{:02}00Z-coder-tester-{suffix}", ordinal)
    }

    fn write_message(&self, id: &str, convergence: &str) -> PathBuf {
        let timestamp = readable_timestamp(id);
        let content = format!(
            "---\nkind: note\nfrom: coder-tester\nto: contribution-auditor\nid: {id}\ntimestamp: {timestamp}\nimmutable: true\nreferences-to: []\ntopic: convergence test\nrequires-ack: false\nartifacts_owned: []\n{convergence}---\n\n# Test event\n"
        );
        let path = self.coord.join(format!("{id}.md"));
        fs::write(&path, content).expect("write coordination event");
        path
    }

    fn run(&self, args: &[&str]) -> Output {
        let mut full = args.to_vec();
        if args.get(0..2) == Some(&["coord", "convergence"]) {
            full.push("--dir");
            full.push(self.coord.to_str().unwrap());
        }
        Command::new(env!("CARGO_BIN_EXE_agov"))
            .env("AGOV_TOKEN_ROI", "0")
            .env("AGOV_SCHEMA_DIR", repo_root().join("agf/schemas"))
            .args(full)
            .current_dir(self.root.path())
            .output()
            .expect("run agov")
    }

    fn json_report(&self) -> Value {
        let output = self.run(&[
            "coord",
            "convergence",
            "--series",
            SERIES,
            "--format",
            "json",
        ]);
        assert_eq!(
            output.status.code(),
            Some(0),
            "stderr: {}\nstdout: {}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        );
        serde_json::from_slice(&output.stdout).expect("json report")
    }
}

fn assert_invariant(report: &Value, id: &str, state: &str, failures: u64, next: &str) {
    let invariant = invariant(report, id);
    assert_eq!(invariant["state"], state);
    assert_eq!(invariant["consecutive_semantic_failures"], failures);
    assert_eq!(invariant["required_next_event"], next);
}

fn invariant<'a>(report: &'a Value, id: &str) -> &'a Value {
    report["invariants"]
        .as_array()
        .expect("invariants")
        .iter()
        .find(|entry| entry["invariant_id"] == id)
        .expect("invariant present")
}

fn digest(path: &str, bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(path.as_bytes());
    hasher.update([0]);
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

fn readable_timestamp(id: &str) -> String {
    let raw = id.split('-').next().expect("timestamp");
    format!(
        "{}-{}-{}T{}:{}:{}Z",
        &raw[0..4],
        &raw[4..6],
        &raw[6..8],
        &raw[9..11],
        &raw[11..13],
        &raw[13..15]
    )
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("repo root")
        .to_path_buf()
}

fn write(root: &Path, rel: &str, content: &str) {
    let path = root.join(rel);
    fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    fs::write(path, content).expect("write file");
}

fn git(root: &Path, args: &[&str]) {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .expect("git");
    assert!(
        output.status.success(),
        "git {:?}: {}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );
}

fn git_stdout(root: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .expect("git");
    assert!(output.status.success());
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}
