// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jani Päijänen
// OCI runner split (2026-10-22): .gitlab-ci.yml changes a declared shared
// surface, but not collision behavior. Rerun these controls alongside the
// positive and wrong-pool routing checks in ci_needs_implication.rs.

// Coord-schema-recompile companion evidence (2026-10-21), matching
// `tests/gate_change_check.rs`'s own companion comment for the same commit:
// `channel_summary`, `coord_publication_check`, `read_coord_messages_with_
// invalid`, `validate_coord_files`, and `duplicate_id_errors_in_dirs` in
// main.rs (a declared `plan-collision-block` surface) were changed to
// compile the coordination-message schema once per invocation instead of
// once per scanned file. Performance-only: no plan-collision-block logic
// touched, and the gate itself is unchanged. Real behavioral coverage for
// these five call sites lives in their existing command-level test files
// (`coord_visibility.rs`, `coord_convergence.rs`, `coord_write.rs`,
// `handoff_branch.rs`, `handoff_next_ack_pending_display.rs`,
// `coord_publication_check.rs`, `urgency_ordering.rs`, `supersession.rs`,
// `session_resume.rs`), all of which pass unchanged; this comment is this
// file's companion touch for that same shared-surface (`main.rs`) commit.

// Windows tooling retirement (2026-10-22): .gitlab-ci.yml drops optional
// Windows preview jobs, a declared shared surface of plan-collision-block.
// Rerun these collision controls alongside the CI job-selection assertions in
// ci_needs_implication.rs; the collision gate itself is unchanged.

use std::{fs, path::PathBuf, process::Command};

struct Repo(tempfile::TempDir);
impl Repo {
    fn new() -> Self {
        let r = Self(tempfile::tempdir().unwrap());
        r.git(&["init", "-b", "main"]);
        r.git(&["config", "user.email", "test@example.invalid"]);
        r.git(&["config", "user.name", "Test"]);
        r.write("docs/plans/.keep", "");
        r.write("old.txt", "rename me\n");
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
    fn run(&self, args: &[&str]) -> (i32, String) {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(4)
            .unwrap()
            .to_path_buf();
        let out = Command::new(env!("CARGO_BIN_EXE_agov"))
            .args(args)
            .env("AGOV_SCHEMA_DIR", root.join("agf/schemas"))
            .env("AGOV_TOKEN_ROI", "0")
            .current_dir(self.0.path())
            .output()
            .unwrap();
        (
            out.status.code().unwrap(),
            format!(
                "{}{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            ),
        )
    }
    fn check(&self) -> (i32, String) {
        self.run(&["plan", "collision-check"])
    }
    fn plan(&self, name: &str, field: &str) {
        self.write(&format!("docs/plans/{name}.md"), &format!("---\nid: {name}\nstatus: approved\n{field}\nadvisory_consultation:\n  - role: infosec-sentinel\n    status: consulted\n  - role: contribution-auditor\n    status: consulted\n---\nPlan.\n"));
    }
    fn plan_with_depends_on(&self, name: &str, affected_paths: &str, depends_on: &str) {
        self.write(&format!("docs/plans/{name}.md"), &format!("---\nid: {name}\nstatus: approved\naffected_paths: {affected_paths}\ndepends_on: {depends_on}\nadvisory_consultation:\n  - role: infosec-sentinel\n    status: consulted\n  - role: contribution-auditor\n    status: consulted\n---\nPlan.\n"));
    }
    /// Issue #130: a `pending-gate1` plan carrying a `pending`
    /// `advisory_consultation` row (the new enum value), otherwise a
    /// normal candidate for collision inventory.
    fn plan_pending_gate1_with_pending_advisory(&self, name: &str, field: &str) {
        self.write(&format!("docs/plans/{name}.md"), &format!("---\nid: {name}\nstatus: pending-gate1\n{field}\nadvisory_consultation:\n  - role: infosec-sentinel\n    status: consulted\n  - role: contribution-auditor\n    status: pending\n---\nPlan.\n"));
    }
}

#[test]
fn optional_field_preserves_validation_and_names_unassessed_coverage() {
    let r = Repo::new();
    r.plan("old", "");
    assert_eq!(r.run(&["plan", "validate", "docs/plans/old.md"]).0, 0);
    let (code, text) = r.check();
    assert_eq!(code, 2, "{text}");
    assert!(text.contains("INCOMPLETE") && text.contains("no affected_paths declared"));
}

#[test]
fn literal_and_future_overlaps_need_no_existing_files() {
    let r = Repo::new();
    for (left, right) in [
        ("future/file.rs", "future/file.rs"),
        ("future/**", "future/not-yet.rs"),
    ] {
        r.plan("a", &format!("affected_paths: ['{left}']"));
        r.plan("b", &format!("affected_paths: ['{right}']"));
        let (code, text) = r.check();
        assert_eq!(code, 1, "{text}");
        assert!(text.contains("plan docs/plans/a.md <-> plan docs/plans/b.md"));
        assert!(text.contains(left) && text.contains(right));
    }
}

#[test]
fn clean_and_incomplete_are_distinct_even_with_overlaps() {
    let r = Repo::new();
    r.plan("a", "affected_paths: ['a.rs']");
    r.plan("b", "affected_paths: ['b.rs']");
    let (code, text) = r.check();
    assert_eq!(code, 0, "{text}");
    assert!(text.contains("Coverage: COMPLETE"));
    r.plan("old", "");
    assert_eq!(r.check().0, 2);
    r.plan("b", "affected_paths: ['a.rs']");
    let (code, text) = r.check();
    assert_eq!(code, 1, "{text}");
    assert!(text.contains("Coverage: INCOMPLETE"));
}

#[test]
fn local_and_remote_refs_include_both_rename_paths_and_base_scope() {
    let r = Repo::new();
    r.git(&["checkout", "-b", "rename"]);
    r.git(&["mv", "old.txt", "new.txt"]);
    let head = r.commit();
    r.git(&["update-ref", "refs/remotes/other/rename", &head]);
    r.git(&["checkout", "main"]);
    r.plan("old", "affected_paths: ['old.txt']");
    r.plan("new", "affected_paths: ['new.txt']");
    let (code, text) = r.check();
    assert_eq!(code, 1, "{text}");
    assert!(
        text.contains("ref refs/heads/rename") && text.contains("ref refs/remotes/other/rename")
    );
    assert!(text.contains("old.txt <-> old.txt") && text.contains("new.txt <-> new.txt"));
    // Changing the base changes diffs, never the working-tree plan inventory.
    let (code, text) = r.run(&["plan", "collision-check", "--base-ref", "rename"]);
    assert_eq!(code, 0, "{text}");
    assert!(text.contains("plans inspected: 2"));
}

#[test]
fn malformed_shape_and_embedded_double_star_are_excluded_with_errors() {
    let r = Repo::new();
    for field in [
        "affected_paths: [42]",
        "affected_paths: ['']",
        "affected_paths: ['/absolute']",
        "affected_paths: ['a/../b']",
        "affected_paths: ['src/a**/x.rs']",
        "affected_paths: ['pre**post/file.rs']",
    ] {
        r.plan("bad", field);
        let (code, text) = r.run(&["plan", "validate", "docs/plans/bad.md"]);
        assert_ne!(code, 0, "{field}: {text}");
        if field.contains("**") {
            assert!(
                text.contains("** is allowed only as a whole path segment"),
                "{text}"
            );
        }
        let (code, text) = r.check();
        assert_eq!(code, 2, "{text}");
        assert!(text.contains("Invalid (excluded): plan docs/plans/bad.md"));
    }
    for pattern in ["**", "a/**", "**/a", "a/**/b", "a/*.rs", "a/?.rs"] {
        r.plan("bad", &format!("affected_paths: ['{pattern}']"));
        assert_eq!(
            r.run(&["plan", "validate", "docs/plans/bad.md"]).0,
            0,
            "{pattern}"
        );
    }
}

#[test]
fn unreadable_ref_is_named_without_losing_known_overlap() {
    let r = Repo::new();
    r.plan("a", "affected_paths: ['a']");
    r.write(
        ".git/refs/heads/broken",
        "1111111111111111111111111111111111111111\n",
    );
    let (code, text) = r.check();
    assert_eq!(code, 2, "{text}");
    assert!(text.contains("refs/heads/broken"));
    r.plan("b", "affected_paths: ['a']");
    assert_eq!(r.check().0, 1);
}

// --- Row 6 S4 (#123): blocking-mode tests. All prior tests above run
// unmodified with none of the three new flags, proving default (no-flag)
// behavior is byte-for-byte unchanged. ---

const BLOCKING: &[&str] = &[
    "plan",
    "collision-check",
    "--plans-only",
    "--changed-only",
    "--fail-on-unacknowledged-collision",
];

#[test]
fn unacknowledged_collision_blocks() {
    let r = Repo::new();
    r.plan("existing", "affected_paths: ['a.rs']");
    let base = r.commit();
    r.plan("new", "affected_paths: ['a.rs']");
    r.commit();
    let mut args = BLOCKING.to_vec();
    args.extend(["--base-ref", &base]);
    let (code, text) = r.run(&args);
    assert_eq!(code, 1, "{text}");
    assert!(text.contains("severity: blocking"), "{text}");
    assert!(text.contains("1 unacknowledged collision"), "{text}");
}

#[test]
fn depends_on_acknowledges_and_clears_the_block() {
    let r = Repo::new();
    r.plan("existing", "affected_paths: ['a.rs']");
    let base = r.commit();
    r.plan_with_depends_on("new", "['a.rs']", "['existing']");
    r.commit();
    let mut args = BLOCKING.to_vec();
    args.extend(["--base-ref", &base]);
    let (code, text) = r.run(&args);
    assert_eq!(code, 0, "{text}");
    assert!(text.contains("severity: informational"), "{text}");
    assert!(text.contains("0 unacknowledged collision"), "{text}");
}

#[test]
fn changed_only_excludes_a_pre_existing_collision_but_default_still_reports_it() {
    let r = Repo::new();
    r.plan("a", "affected_paths: ['x.rs']");
    r.plan("b", "affected_paths: ['x.rs']");
    let base = r.commit();
    // Nothing changes after `base` -- both plans already existed and
    // already collided before this diff.
    let mut args = BLOCKING.to_vec();
    args.extend(["--base-ref", &base]);
    let (code, text) = r.run(&args);
    assert_eq!(code, 0, "{text}");
    assert!(text.contains("0 unacknowledged collision"), "{text}");

    // The exact same collision, without --changed-only, is still reported
    // in the (unchanged) advisory report -- proving the flag genuinely
    // restricts scope rather than the collision having vanished.
    let (code, text) = r.check();
    assert_eq!(code, 1, "{text}");
    assert!(text.contains("plan docs/plans/a.md <-> plan docs/plans/b.md"));
}

#[test]
fn unassessed_plan_is_not_conflated_with_an_unacknowledged_collision() {
    let r = Repo::new();
    r.plan("existing", "affected_paths: ['a.rs']");
    let base = r.commit();
    r.plan("new", "affected_paths: ['b.rs']"); // disjoint, no collision
    r.plan("no-paths", ""); // unassessed: no affected_paths declared
    r.commit();
    let mut args = BLOCKING.to_vec();
    args.extend(["--base-ref", &base]);
    let (code, text) = r.run(&args);
    assert_eq!(code, 0, "{text}");
    assert!(text.contains("Unassessed:"), "{text}");
}

#[test]
fn plans_only_ignores_a_colliding_local_branch() {
    let r = Repo::new();
    r.git(&["checkout", "-b", "other"]);
    r.write("a.rs", "x\n");
    r.commit();
    r.git(&["checkout", "main"]);
    r.plan("existing", "affected_paths: ['a.rs']");
    let base = r.commit();
    r.plan("new", "affected_paths: ['z.rs']"); // no plan-vs-plan collision
    r.commit();
    let mut args = BLOCKING.to_vec();
    args.extend(["--base-ref", &base]);
    let (code, text) = r.run(&args);
    // A plain (non-plans-only) check would report the branch collision;
    // --plans-only must not consult ref state at all.
    assert_eq!(code, 0, "{text}");
    assert!(!text.contains("ref refs/heads/other"), "{text}");
}

#[test]
fn unresolvable_base_fails_closed_under_blocking_mode() {
    let r = Repo::new();
    r.plan("new", "affected_paths: ['a.rs']");
    r.commit();
    let mut args = BLOCKING.to_vec();
    args.extend(["--base-ref", "does-not-exist"]);
    let (code, _text) = r.run(&args);
    assert_eq!(code, 1, "an unresolvable base must block, never silently pass");
}

/// S4-I1 regression (`20260914T173527Z-coder-tester-hkj4`): a live plan
/// added to the target branch AFTER the candidate forked must not be
/// invisible to the blocking check. Exact topology from the review's own
/// controlled reproduction: fork, add a colliding plan on the candidate,
/// then add a colliding plan on the target -- the candidate's own working
/// tree never sees the target-side addition via the filesystem alone.
#[test]
fn target_side_plan_added_after_fork_is_not_invisible() {
    let r = Repo::new();
    let fork = r.git(&["rev-parse", "HEAD"]);
    r.git(&["checkout", "-b", "candidate"]);
    r.plan("candidate", "affected_paths: ['shared.rs']");
    r.commit();
    r.git(&["checkout", "main"]);
    r.plan("existing", "affected_paths: ['shared.rs']");
    r.commit();
    r.git(&["checkout", "candidate"]);

    let (code, text) = r.run(&[
        "plan",
        "collision-check",
        "--plans-only",
        "--changed-only",
        "--fail-on-unacknowledged-collision",
        "--base-ref",
        "main",
    ]);
    assert_eq!(
        code, 1,
        "a target-only live plan added after fork must still block: {text}"
    );
    assert!(text.contains("candidate.md"), "{text}");
    assert!(text.contains("existing.md"), "{text}");
    assert!(text.contains("severity: blocking"), "{text}");
    let _ = fork; // establishes the divergence point; not asserted directly
}

/// S4-I1 regression, positive counterpart: the same fork/target-add
/// topology, but the candidate's plan declares `depends_on` naming the
/// target-only plan's id -- must clear the block despite never having
/// seen that plan locally before the check ran.
#[test]
fn target_side_plan_can_be_acknowledged_via_depends_on() {
    let r = Repo::new();
    r.git(&["checkout", "-b", "candidate"]);
    r.plan_with_depends_on("candidate", "['shared.rs']", "['existing']");
    r.commit();
    r.git(&["checkout", "main"]);
    r.plan("existing", "affected_paths: ['shared.rs']");
    r.commit();
    r.git(&["checkout", "candidate"]);

    let (code, text) = r.run(&[
        "plan",
        "collision-check",
        "--plans-only",
        "--changed-only",
        "--fail-on-unacknowledged-collision",
        "--base-ref",
        "main",
    ]);
    assert_eq!(code, 0, "{text}");
    assert!(text.contains("severity: informational"), "{text}");
}

#[test]
fn plans_only_suppresses_forge_querying_even_with_include_forge() {
    let r = Repo::new();
    r.plan("a", "affected_paths: ['x.rs']");
    let base = r.commit();
    let (code, text) = r.run(&[
        "plan",
        "collision-check",
        "--plans-only",
        "--include-forge",
        "--base-ref",
        &base,
    ]);
    assert_eq!(code, 0, "{text}");
    assert!(text.contains("mode: plans-only"), "{text}");
    assert!(!text.contains("MRs inspected: 1"), "{text}");
}

#[test]
fn equivalent_relative_spellings_cannot_hide_a_real_branch_overlap() {
    let r = Repo::new();
    r.git(&["checkout", "-b", "edit"]);
    r.write("old.txt", "modified\n");
    r.write("a/b", "new\n");
    r.commit();
    r.git(&["checkout", "main"]);
    for pattern in ["./old.txt", "old.txt/", "a/./b", "a//b", "./", "."] {
        r.plan("alias", &format!("affected_paths: ['{pattern}']"));
        assert_eq!(r.run(&["plan", "validate", "docs/plans/alias.md"]).0, 0);
        let (code, text) = r.check();
        assert_eq!(code, 1, "{pattern}: {text}");
        assert!(text.contains("ref refs/heads/edit"));
    }
    r.git(&["branch", "-D", "edit"]);
    r.plan("alias", "affected_paths: ['./future//file.rs']");
    r.plan("canonical", "affected_paths: ['future/file.rs']");
    let (code, text) = r.check();
    assert_eq!(code, 1, "{text}");
    assert!(text.contains("plan docs/plans/alias.md <-> plan docs/plans/canonical.md"));
}

/// Issue #130 regression guard (round-3 independent audit `damr`'s
/// required companion test for `.agf/gates.yaml`'s `plan-collision-block`
/// gate, which registers this schema as a surface): a plan validly using
/// the new `pending` advisory_consultation value at `status: pending-gate1`
/// is still scanned and included in collision-check's inventory like any
/// other candidate, and still participates correctly in overlap detection.
#[test]
fn pending_advisory_status_plan_is_still_scanned_and_detects_overlap() {
    let r = Repo::new();
    r.plan_pending_gate1_with_pending_advisory("mid-draft", "affected_paths: ['shared.rs']");
    r.plan("existing", "affected_paths: ['shared.rs']");
    let (code, text) = r.check();
    assert_eq!(code, 1, "{text}");
    assert!(text.contains("plan docs/plans/existing.md <-> plan docs/plans/mid-draft.md"));
    assert!(text.contains("plans inspected: 2"), "{text}");
}

#[test]
fn live_ci_keeps_the_plan_collision_gate_after_an_unrelated_job_removal() {
    // Issue 140 removed an unrelated CI job from .gitlab-ci.yml. The shared CI
    // surface is registered to plan-collision-block, so show that the gate job
    // and its self-test wiring are still present.
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("repository root")
        .to_path_buf();
    let configuration =
        fs::read_to_string(root.join(".gitlab-ci.yml")).expect("read live CI configuration");
    assert!(
        configuration.contains("agov:governance:plan-collision-gate:"),
        "plan-collision gate job missing"
    );
    assert!(
        configuration.contains("plan-collision-self-test.sh"),
        "plan-collision self-test wiring missing"
    );
}
