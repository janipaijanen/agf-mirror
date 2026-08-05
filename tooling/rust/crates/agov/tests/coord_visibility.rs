use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn reports_unbridged_obligations_and_sequence_collisions() {
    let repo = GitFixture::new();
    repo.write(
        "docs/coordination/00100-governor-reviewer.md",
        &decision(100, "base decision", &[]),
    );
    repo.write(
        "docs/coordination/00188-planner-architect.md",
        &decision(188, "base collision", &[]),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "base"]);
    repo.git(&["branch", "-M", "main"]);
    repo.git(&["checkout", "-b", "feat/hidden"]);
    repo.write(
        "docs/coordination/00111-coder-tester.md",
        &handoff(111, "governor-reviewer", "unbridged gate2"),
    );
    repo.write(
        "docs/coordination/00188-coder-tester.md",
        &handoff(188, "governor-reviewer", "collision"),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "hidden coordination"]);

    let output = repo.agov(&[
        "coord",
        "visibility",
        "--base-branch",
        "main",
        "--branch",
        "feat/hidden",
    ]);

    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ERR sequence collision"));
    assert!(stdout.contains("seq: 188"));
    assert!(stdout.contains("WARN branch-local coordination obligations not visible"));
    assert!(stdout.contains("branch: feat/hidden"));
    assert!(stdout.contains("topic: unbridged gate2"));
}

#[test]
fn ignores_handoffs_bridged_by_reference_or_body() {
    let repo = GitFixture::new();
    repo.write(
        "docs/coordination/00100-governor-reviewer.md",
        &decision(100, "base decision", &[]),
    );
    repo.write(
        "docs/coordination/00120-coder-tester.md",
        &decision(
            120,
            "reference bridge",
            &["docs/coordination/00110-coder-tester.md"],
        ),
    );
    repo.write(
        "docs/coordination/00130-coder-tester.md",
        &(decision(130, "body bridge", &[])
            + "\nfeat/bridged docs/coordination/00121-coder-tester.md\n"),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "base"]);
    repo.git(&["branch", "-M", "main"]);
    repo.git(&["checkout", "-b", "feat/bridged"]);
    repo.write(
        "docs/coordination/00110-coder-tester.md",
        &handoff(110, "governor-reviewer", "referenced bridge"),
    );
    repo.write(
        "docs/coordination/00121-coder-tester.md",
        &handoff(121, "planner-architect", "body bridge"),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "bridged coordination"]);

    let output = repo.agov(&[
        "coord",
        "visibility",
        "--base-branch",
        "main",
        "--branch",
        "feat/bridged",
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("OK coordination visibility clean relative to main"));
}

#[test]
fn emits_stable_json_fields() {
    let repo = GitFixture::new();
    repo.write(
        "docs/coordination/00100-governor-reviewer.md",
        &decision(100, "base decision", &[]),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "base"]);
    repo.git(&["branch", "-M", "main"]);
    repo.git(&["checkout", "-b", "feat/json"]);
    repo.write(
        "docs/coordination/00111-coder-tester.md",
        &handoff(111, "governor-reviewer", "json handoff"),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "json coordination"]);

    let output = repo.agov(&[
        "coord",
        "visibility",
        "--base-branch",
        "main",
        "--branch",
        "feat/json",
        "--format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    assert_eq!(json["base_branch"], "main");
    assert_eq!(json["exit_code"], 1);
    assert_eq!(json["unbridged_obligations"][0]["branch"], "feat/json");
    assert_eq!(json["unbridged_obligations"][0]["seq"], 111);
}

/// Issue #85, outcome (a) of the required exit-code matrix: sequence
/// collisions confined purely to base-branch history (both colliding
/// files exist only on the base, unrelated to the branch under
/// inspection) must not block -- exit `0`, reported as a single summary
/// line, not a per-collision `ERR` block.
#[test]
fn legacy_base_only_collision_does_not_block_and_reports_summary() {
    let repo = GitFixture::new();
    repo.write(
        "docs/coordination/00050-governor-reviewer.md",
        &decision(50, "legacy base collision one", &[]),
    );
    repo.write(
        "docs/coordination/00050-planner-architect.md",
        &decision(50, "legacy base collision two", &[]),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "base with legacy collision"]);
    repo.git(&["branch", "-M", "main"]);
    repo.git(&["checkout", "-b", "feat/clean"]);
    repo.write("docs/coordination/.gitkeep", "");
    repo.git(&["add", "."]);
    repo.git(&[
        "commit",
        "-m",
        "unrelated branch commit, no coordination messages",
    ]);

    let output = repo.agov(&[
        "coord",
        "visibility",
        "--base-branch",
        "main",
        "--branch",
        "feat/clean",
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("ERR sequence collision"));
    assert!(stdout.contains(
        "WARN 1 legacy sequence collision group(s) confined to base branch history; run agov coord validate for detail"
    ));
}

/// Issue #85, outcome (b) -- the mixed case `p438`/`61gt` required:
/// base-history-only legacy noise plus a genuine unbridged obligation on
/// the branch under test must still exit `1` (the legacy noise never
/// escalates to `2`, and never suppresses the `1`), with the obligation
/// block rendered before the legacy-collision summary line.
#[test]
fn legacy_base_only_collision_plus_unbridged_obligation_exits_one_with_obligation_first() {
    let repo = GitFixture::new();
    repo.write(
        "docs/coordination/00050-governor-reviewer.md",
        &decision(50, "legacy base collision one", &[]),
    );
    repo.write(
        "docs/coordination/00050-planner-architect.md",
        &decision(50, "legacy base collision two", &[]),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "base with legacy collision"]);
    repo.git(&["branch", "-M", "main"]);
    repo.git(&["checkout", "-b", "feat/obligation"]);
    repo.write(
        "docs/coordination/00300-coder-tester.md",
        &handoff(300, "governor-reviewer", "unbridged gate2"),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "unbridged obligation"]);

    let output = repo.agov(&[
        "coord",
        "visibility",
        "--base-branch",
        "main",
        "--branch",
        "feat/obligation",
    ]);

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("ERR sequence collision"));
    let obligation_pos = stdout
        .find("WARN branch-local coordination obligations not visible")
        .expect("obligation block must be present");
    let summary_pos = stdout
        .find("legacy sequence collision group(s) confined to base branch history")
        .expect("legacy-collision summary line must be present");
    assert!(
        obligation_pos < summary_pos,
        "obligation block must render before the legacy-collision summary:\n{stdout}"
    );
}

/// Issue #85 (B-V26-1 frozen contract), positive and negative JSON shape
/// assertions per `boundary-shape-testing-rule.md`: `legacy_base_collisions`
/// is always an object with integer `count` and array `seqs`, and
/// `sequence_collisions` no longer contains `"type": "base_duplicate_seq"`
/// entries -- the negative case that would silently pass if an
/// implementation kept the old field contents and only changed exit-code
/// logic around them.
#[test]
fn legacy_base_collisions_json_shape_and_narrowed_sequence_collisions() {
    let repo = GitFixture::new();
    repo.write(
        "docs/coordination/00050-governor-reviewer.md",
        &decision(50, "legacy base collision one", &[]),
    );
    repo.write(
        "docs/coordination/00050-planner-architect.md",
        &decision(50, "legacy base collision two", &[]),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "base with legacy collision"]);
    repo.git(&["branch", "-M", "main"]);
    repo.git(&["checkout", "-b", "feat/json-legacy"]);
    repo.write("docs/coordination/.gitkeep", "");
    repo.git(&["add", "."]);
    repo.git(&[
        "commit",
        "-m",
        "unrelated branch commit, no coordination messages",
    ]);

    let output = repo.agov(&[
        "coord",
        "visibility",
        "--base-branch",
        "main",
        "--branch",
        "feat/json-legacy",
        "--format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    assert_eq!(json["legacy_base_collisions"]["count"], 1);
    assert_eq!(
        json["legacy_base_collisions"]["seqs"],
        serde_json::json!([50])
    );
    assert_eq!(json["sequence_collisions"], serde_json::json!([]));
    for entry in json["sequence_collisions"].as_array().unwrap() {
        assert_ne!(
            entry["type"], "base_duplicate_seq",
            "sequence_collisions must never contain base_duplicate_seq entries after this change"
        );
    }
}

/// Companion negative-shape check using the clean-repository case: even
/// with zero legacy collisions, `legacy_base_collisions` must be present
/// as `{"count": 0, "seqs": []}`, never `null` or omitted.
#[test]
fn legacy_base_collisions_present_with_empty_values_when_clean() {
    let repo = GitFixture::new();
    repo.write(
        "docs/coordination/00100-governor-reviewer.md",
        &decision(100, "base decision", &[]),
    );
    repo.write(
        "docs/coordination/00120-coder-tester.md",
        &decision(
            120,
            "reference bridge",
            &["docs/coordination/00110-coder-tester.md"],
        ),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "base"]);
    repo.git(&["branch", "-M", "main"]);
    repo.git(&["checkout", "-b", "feat/clean-json"]);
    repo.write(
        "docs/coordination/00110-coder-tester.md",
        &handoff(110, "governor-reviewer", "referenced bridge"),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "bridged coordination"]);

    let output = repo.agov(&[
        "coord",
        "visibility",
        "--base-branch",
        "main",
        "--branch",
        "feat/clean-json",
        "--format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    assert_eq!(
        json["legacy_base_collisions"],
        serde_json::json!({"count": 0, "seqs": []})
    );
}

/// Issue #89: a coordination file that exists only on a fetched-but-not-
/// checked-out remote branch (`refs/remotes/origin/<branch>`) must be
/// found by the default, no-`--branch` scan.
#[test]
fn default_scan_discovers_remote_only_branch_after_fetch() {
    let repo = GitFixture::new();
    repo.write(
        "docs/coordination/00100-governor-reviewer.md",
        &decision(100, "base decision", &[]),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "base"]);
    repo.git(&["branch", "-M", "main"]);

    repo.git(&["checkout", "-b", "feat/remote-only"]);
    repo.write(
        "docs/coordination/00400-coder-tester.md",
        &handoff(400, "governor-reviewer", "remote-only obligation"),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "remote-only obligation"]);
    let oid = repo.rev_parse("feat/remote-only");
    repo.set_remote_ref("origin", "feat/remote-only", &oid);
    repo.git(&["checkout", "main"]);
    repo.git(&["branch", "-D", "feat/remote-only"]);

    let output = repo.agov(&[
        "coord",
        "visibility",
        "--base-branch",
        "main",
        "--format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    assert_eq!(
        json["scanned_branches"],
        serde_json::json!(["origin/feat/remote-only"]),
        "dedup case (b): a remote-only ref reports once under its remote-tracking name"
    );
    assert_eq!(
        json["unbridged_obligations"][0]["branch"],
        "origin/feat/remote-only"
    );
    assert_eq!(json["unbridged_obligations"][0]["seq"], 400);
}

/// Dedup case (a): a local branch plus a remote-tracking ref at the same
/// OID must report once, under the local (Class 1) name.
#[test]
fn dedup_local_and_remote_alias_reports_once_under_local_name() {
    let repo = GitFixture::new();
    repo.write(
        "docs/coordination/00100-governor-reviewer.md",
        &decision(100, "base decision", &[]),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "base"]);
    repo.git(&["branch", "-M", "main"]);

    repo.git(&["checkout", "-b", "feat/alias"]);
    repo.write(
        "docs/coordination/00401-coder-tester.md",
        &handoff(401, "governor-reviewer", "alias obligation"),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "alias obligation"]);
    let oid = repo.rev_parse("feat/alias");
    repo.set_remote_ref("origin", "feat/alias", &oid);
    repo.git(&["checkout", "main"]);

    let output = repo.agov(&[
        "coord",
        "visibility",
        "--base-branch",
        "main",
        "--format",
        "json",
    ]);

    assert_eq!(output.status.code(), Some(1));
    let json: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&output.stdout).trim()).unwrap();
    assert_eq!(json["scanned_branches"], serde_json::json!(["feat/alias"]));
}

/// Dedup case (c): two different, non-`origin` remotes' tracking refs at
/// the same OID with no local branch must report once, under the
/// lexicographically-smallest fully-qualified ref name.
#[test]
fn dedup_two_non_origin_remotes_same_oid_reports_once_lexicographically_smallest() {
    let repo = GitFixture::new();
    repo.write(
        "docs/coordination/00100-governor-reviewer.md",
        &decision(100, "base decision", &[]),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "base"]);
    repo.git(&["branch", "-M", "main"]);

    repo.git(&["checkout", "-b", "feat/multi-remote"]);
    repo.write(
        "docs/coordination/01000-coder-tester.md",
        &handoff(1000, "governor-reviewer", "multi-remote obligation"),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "multi-remote obligation"]);
    let oid = repo.rev_parse("feat/multi-remote");
    repo.set_remote_ref("upstream", "feat/multi-remote", &oid);
    repo.set_remote_ref("backup", "feat/multi-remote", &oid);
    repo.git(&["checkout", "main"]);
    repo.git(&["branch", "-D", "feat/multi-remote"]);

    let output = repo.agov(&[
        "coord",
        "visibility",
        "--base-branch",
        "main",
        "--format",
        "json",
    ]);

    let json: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&output.stdout).trim()).unwrap();
    assert_eq!(
        json["scanned_branches"],
        serde_json::json!(["backup/feat/multi-remote"]),
        "refs/remotes/backup/... sorts before refs/remotes/upstream/... lexicographically"
    );
}

/// Dedup case (d): `refs/remotes/<remote>/HEAD` is a symbolic ref and
/// must never be enumerated as a candidate branch.
#[test]
fn dedup_never_enumerates_remote_head_symbolic_ref() {
    let repo = GitFixture::new();
    repo.write(
        "docs/coordination/00100-governor-reviewer.md",
        &decision(100, "base decision", &[]),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "base"]);
    repo.git(&["branch", "-M", "main"]);

    repo.git(&["checkout", "-b", "feat/normal"]);
    repo.write(
        "docs/coordination/00700-coder-tester.md",
        &handoff(700, "governor-reviewer", "normal obligation"),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "normal obligation"]);
    let oid = repo.rev_parse("feat/normal");
    repo.set_remote_ref("origin", "feat/normal", &oid);
    repo.set_remote_head("origin", "feat/normal");
    repo.git(&["checkout", "main"]);
    repo.git(&["branch", "-D", "feat/normal"]);

    let output = repo.agov(&[
        "coord",
        "visibility",
        "--base-branch",
        "main",
        "--format",
        "json",
    ]);

    let json: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&output.stdout).trim()).unwrap();
    assert_eq!(
        json["scanned_branches"],
        serde_json::json!(["origin/feat/normal"])
    );
}

/// Dedup case (e): two local branches at the same OID must report once,
/// under the lexicographically-smallest `refs/heads/*` name.
#[test]
fn dedup_two_local_branches_same_oid_reports_once_lexicographically_smallest() {
    let repo = GitFixture::new();
    repo.write(
        "docs/coordination/00100-governor-reviewer.md",
        &decision(100, "base decision", &[]),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "base"]);
    repo.git(&["branch", "-M", "main"]);

    repo.git(&["checkout", "-b", "zzz-branch"]);
    repo.write(
        "docs/coordination/00800-coder-tester.md",
        &handoff(800, "governor-reviewer", "dup-local obligation"),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "dup-local obligation"]);
    repo.git(&["branch", "aaa-branch", "zzz-branch"]);
    repo.git(&["checkout", "main"]);

    let output = repo.agov(&[
        "coord",
        "visibility",
        "--base-branch",
        "main",
        "--format",
        "json",
    ]);

    let json: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&output.stdout).trim()).unwrap();
    assert_eq!(json["scanned_branches"], serde_json::json!(["aaa-branch"]));
}

/// Dedup case (f): two `origin/*` refs at the same OID with no local
/// branch must report once, under the lexicographically-smallest
/// `refs/remotes/origin/*` name.
#[test]
fn dedup_two_origin_refs_same_oid_reports_once_lexicographically_smallest() {
    let repo = GitFixture::new();
    repo.write(
        "docs/coordination/00100-governor-reviewer.md",
        &decision(100, "base decision", &[]),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "base"]);
    repo.git(&["branch", "-M", "main"]);

    repo.git(&["checkout", "-b", "feat/origin-dup"]);
    repo.write(
        "docs/coordination/00900-coder-tester.md",
        &handoff(900, "governor-reviewer", "origin-dup obligation"),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "origin-dup obligation"]);
    let oid = repo.rev_parse("feat/origin-dup");
    repo.set_remote_ref("origin", "zzz-name", &oid);
    repo.set_remote_ref("origin", "aaa-name", &oid);
    repo.git(&["checkout", "main"]);
    repo.git(&["branch", "-D", "feat/origin-dup"]);

    let output = repo.agov(&[
        "coord",
        "visibility",
        "--base-branch",
        "main",
        "--format",
        "json",
    ]);

    let json: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&output.stdout).trim()).unwrap();
    assert_eq!(
        json["scanned_branches"],
        serde_json::json!(["origin/aaa-name"])
    );
}

/// Issue #89, base-ref freshness (B1): when the base branch's configured
/// upstream OID differs from the local base branch's OID, the upstream
/// OID is used as the effective comparison base, and both text and JSON
/// output state which OID was actually used.
#[test]
fn base_freshness_prefers_upstream_oid_when_local_is_stale() {
    let repo = GitFixture::new();
    repo.write(
        "docs/coordination/00100-governor-reviewer.md",
        &decision(100, "base decision", &[]),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "base v1"]);
    repo.git(&["branch", "-M", "main"]);
    let stale_local_oid = repo.rev_parse("main");

    repo.git(&["checkout", "-b", "origin-main-simulation"]);
    repo.write(
        "docs/coordination/00105-governor-reviewer.md",
        &decision(105, "base decision v2", &[]),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "base v2, only on upstream"]);
    let fresh_upstream_oid = repo.rev_parse("origin-main-simulation");
    repo.set_remote_ref("origin", "main", &fresh_upstream_oid);
    repo.git(&["checkout", "main"]);
    repo.git(&["branch", "-D", "origin-main-simulation"]);
    repo.set_upstream("main", "origin/main");
    assert_ne!(stale_local_oid, fresh_upstream_oid);

    repo.git(&["checkout", "-b", "feat/needs-fresh-base"]);
    repo.write(
        "docs/coordination/00500-coder-tester.md",
        &handoff(500, "governor-reviewer", "genuine obligation"),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "branch work"]);

    let json_output = repo.agov(&[
        "coord",
        "visibility",
        "--base-branch",
        "main",
        "--branch",
        "feat/needs-fresh-base",
        "--format",
        "json",
    ]);
    let json: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&json_output.stdout).trim()).unwrap();
    assert_eq!(json["effective_base_oid"], fresh_upstream_oid);
    assert_eq!(json["effective_base_source"], "upstream");
    assert_eq!(json["unbridged_obligations"][0]["seq"], 500);

    let text_output = repo.agov(&[
        "coord",
        "visibility",
        "--base-branch",
        "main",
        "--branch",
        "feat/needs-fresh-base",
    ]);
    let stdout = String::from_utf8_lossy(&text_output.stdout);
    assert!(
        stdout.contains(&format!("effective-base: {fresh_upstream_oid} (upstream)")),
        "text output must state which OID was used:\n{stdout}"
    );
}

/// Companion to the freshness test: with no upstream configured for the
/// base branch, behavior is unchanged (local OID only).
#[test]
fn base_freshness_falls_back_to_local_when_no_upstream_configured() {
    let repo = GitFixture::new();
    repo.write(
        "docs/coordination/00100-governor-reviewer.md",
        &decision(100, "base decision", &[]),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "base"]);
    repo.git(&["branch", "-M", "main"]);
    let local_oid = repo.rev_parse("main");

    let output = repo.agov(&[
        "coord",
        "visibility",
        "--base-branch",
        "main",
        "--format",
        "json",
    ]);
    let json: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&output.stdout).trim()).unwrap();
    assert_eq!(json["effective_base_oid"], local_oid);
    assert_eq!(json["effective_base_source"], "local");
}

/// Issue #89, base exclusion (R1): every ref that peels to the effective
/// base OID -- the local base branch itself and any remote-tracking ref
/// at that same OID -- is excluded from `scanned_branches` entirely.
#[test]
fn base_exclusion_never_reports_base_aliases_as_candidates() {
    let repo = GitFixture::new();
    repo.write(
        "docs/coordination/00100-governor-reviewer.md",
        &decision(100, "base decision", &[]),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "base"]);
    repo.git(&["branch", "-M", "main"]);
    let main_oid = repo.rev_parse("main");
    repo.set_remote_ref("origin", "main", &main_oid);
    repo.set_upstream("main", "origin/main");

    repo.git(&["checkout", "-b", "feat/real-work"]);
    repo.write(
        "docs/coordination/00600-coder-tester.md",
        &handoff(600, "governor-reviewer", "real obligation"),
    );
    repo.git(&["add", "."]);
    repo.git(&["commit", "-m", "real work"]);
    repo.git(&["checkout", "main"]);

    let output = repo.agov(&[
        "coord",
        "visibility",
        "--base-branch",
        "main",
        "--format",
        "json",
    ]);
    let json: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&output.stdout).trim()).unwrap();
    assert_eq!(
        json["scanned_branches"],
        serde_json::json!(["feat/real-work"]),
        "neither 'main' nor 'origin/main' (both at the effective base OID) may appear"
    );
}

struct GitFixture {
    root: PathBuf,
}

impl GitFixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "agov_coord_visibility_{}_{}",
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
        Command::new(env!("CARGO_BIN_EXE_agov"))
            .env("AGOV_TOKEN_ROI", "0")
            .args(args)
            .current_dir(&self.root)
            .output()
            .expect("run agov")
    }

    fn rev_parse(&self, refname: &str) -> String {
        let output = Command::new("git")
            .args(["rev-parse", refname])
            .current_dir(&self.root)
            .output()
            .expect("run git rev-parse");
        assert!(
            output.status.success(),
            "git rev-parse {refname} failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    /// Fabricates a remote-tracking ref (`refs/remotes/<remote>/<branch>`)
    /// pointing at `oid`, without a real network remote -- sufficient for
    /// exercising enumeration/dedup/freshness logic, which only reads ref
    /// state, not remote connectivity.
    fn set_remote_ref(&self, remote: &str, branch: &str, oid: &str) {
        self.git(&[
            "update-ref",
            &format!("refs/remotes/{remote}/{branch}"),
            oid,
        ]);
    }

    /// Points `refs/remotes/<remote>/HEAD` as a symbolic ref, matching
    /// what a real `git remote add` + fetch produces -- must never be
    /// enumerated as a candidate branch.
    fn set_remote_head(&self, remote: &str, target_branch: &str) {
        self.git(&[
            "symbolic-ref",
            &format!("refs/remotes/{remote}/HEAD"),
            &format!("refs/remotes/{remote}/{target_branch}"),
        ]);
    }

    fn set_upstream(&self, local_branch: &str, remote_ref: &str) {
        let (remote, _) = remote_ref
            .split_once('/')
            .expect("remote_ref must be <remote>/<branch>");
        // `--set-upstream-to` requires a registered remote to resolve
        // against (it maps branch.<name>.merge through remote.<remote>.fetch)
        // -- the URL is never contacted, only its config entry matters.
        let _ = Command::new("git")
            .args([
                "remote",
                "add",
                remote,
                &format!("file:///nonexistent-{remote}"),
            ])
            .current_dir(&self.root)
            .output();
        self.git(&[
            "branch",
            &format!("--set-upstream-to={remote_ref}"),
            local_branch,
        ]);
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

fn handoff(seq: i64, to: &str, topic: &str) -> String {
    frontmatter(&format!(
        r#"kind: handoff
from: coder-tester
to: {to}
seq: {seq}
timestamp: 2026-06-13T00:00:00Z
immutable: true
references-to: []
topic: {topic}
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: {to}
  status: ready
  task:
    - {topic}
  must_read: []
  do_not_touch: []
"#
    ))
}

fn decision(seq: i64, topic: &str, refs: &[&str]) -> String {
    let refs = if refs.is_empty() {
        "[]".to_string()
    } else {
        format!(
            "\n{}",
            refs.iter()
                .map(|item| format!("  - {item}"))
                .collect::<Vec<_>>()
                .join("\n")
        )
    };
    frontmatter(&format!(
        r#"kind: decision-record
from: governor-reviewer
to: all
seq: {seq}
timestamp: 2026-06-13T00:01:00Z
immutable: true
references-to: {refs}
topic: {topic}
requires-ack: false
artifacts_owned: []
"#
    ))
}

fn frontmatter(yaml: &str) -> String {
    format!("---\n{yaml}---\n\nFixture body.\n")
}
