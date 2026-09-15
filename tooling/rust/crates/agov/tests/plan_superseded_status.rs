// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jani Päijänen
//! qbka Slice A: provenance-bound `status: superseded` plan lifecycle.
//! Gate-self-test-rule.md negative fixtures per
//! docs/plans/20260913T130049Z-qbka-slice-a-superseded-plan-lifecycle.md's
//! own required test strategy.
use std::{fs, path::PathBuf, process::Command};

struct Repo(tempfile::TempDir);
impl Repo {
    fn new() -> Self {
        Self(tempfile::tempdir().unwrap())
    }
    fn write(&self, path: &str, content: &str) {
        let p = self.0.path().join(path);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, content).unwrap();
    }
    fn validate(&self, name: &str) -> (i32, String) {
        self.validate_with_manifest(name, None)
    }
    /// `manifest_path`, when set, is passed as `AGOV_MANIFEST_PATH` so a
    /// declared optional role (e.g. `fact-checker`) resolves as `active`
    /// instead of falling back to "unknown advisory role" -- required to
    /// isolate the schema `pending` conditional's own diagnostic from the
    /// unrelated required-role-only fallback (round-2 Gate 2 review j78p).
    fn validate_with_manifest(
        &self,
        name: &str,
        manifest_path: Option<&std::path::Path>,
    ) -> (i32, String) {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(4)
            .unwrap()
            .to_path_buf();
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_agov"));
        cmd.args(["plan", "validate", name])
            .env("AGOV_SCHEMA_DIR", root.join("agf/schemas"))
            .env("AGOV_TOKEN_ROI", "0")
            .current_dir(self.0.path());
        if let Some(manifest_path) = manifest_path {
            cmd.env("AGOV_MANIFEST_PATH", manifest_path);
        } else {
            cmd.env_remove("AGOV_MANIFEST_PATH");
        }
        let out = cmd.output().unwrap();
        (
            out.status.code().unwrap(),
            format!(
                "{}{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            ),
        )
    }
    fn plan(&self, name: &str, front_matter_extra: &str) {
        self.write(
            &format!("docs/plans/{name}.md"),
            &format!(
                "---\nid: {name}\nstatus: superseded\n{front_matter_extra}advisory_consultation:\n  - role: infosec-sentinel\n    status: consulted\n  - role: contribution-auditor\n    status: consulted\n---\nPlan.\n"
            ),
        );
    }
    /// For issue #130's `pending` conditional: arbitrary `status` and an
    /// explicit `advisory_consultation` block (caller controls per-row status).
    fn plan_with_status(&self, name: &str, status: &str, advisory_consultation_yaml: &str) {
        self.write(
            &format!("docs/plans/{name}.md"),
            &format!(
                "---\nid: {name}\nstatus: {status}\nadvisory_consultation:\n{advisory_consultation_yaml}\n---\nPlan.\n"
            ),
        );
    }
}

#[test]
fn superseded_status_with_all_provenance_fields_is_valid() {
    let repo = Repo::new();
    repo.plan(
        "ok",
        "superseded_by: replacement-plan-id\nsuperseded_decision: 20260913T130413Z-governor-reviewer-td7j.md\nsuperseded_at: 2026-09-13T13:04:13Z\n",
    );
    let (code, out) = repo.validate("docs/plans/ok.md");
    assert_eq!(code, 0, "{out}");
}

/// Gate-self-test negative: missing `superseded_by` is rejected.
#[test]
fn superseded_status_missing_superseded_by_is_rejected() {
    let repo = Repo::new();
    repo.plan(
        "missing-by",
        "superseded_decision: 20260913T130413Z-governor-reviewer-td7j.md\nsuperseded_at: 2026-09-13T13:04:13Z\n",
    );
    let (code, out) = repo.validate("docs/plans/missing-by.md");
    assert_ne!(code, 0, "{out}");
    assert!(out.contains("superseded_by"), "{out}");
}

/// Gate-self-test negative: missing `superseded_decision` is rejected.
#[test]
fn superseded_status_missing_superseded_decision_is_rejected() {
    let repo = Repo::new();
    repo.plan(
        "missing-decision",
        "superseded_by: replacement-plan-id\nsuperseded_at: 2026-09-13T13:04:13Z\n",
    );
    let (code, out) = repo.validate("docs/plans/missing-decision.md");
    assert_ne!(code, 0, "{out}");
    assert!(out.contains("superseded_decision"), "{out}");
}

/// Gate-self-test negative: missing `superseded_at` is rejected.
#[test]
fn superseded_status_missing_superseded_at_is_rejected() {
    let repo = Repo::new();
    repo.plan(
        "missing-at",
        "superseded_by: replacement-plan-id\nsuperseded_decision: 20260913T130413Z-governor-reviewer-td7j.md\n",
    );
    let (code, out) = repo.validate("docs/plans/missing-at.md");
    assert_ne!(code, 0, "{out}");
    assert!(out.contains("superseded_at"), "{out}");
}

/// Gate-self-test negative (QBKA-A1): `superseded_at` must be a real
/// RFC3339 timestamp, not merely a non-empty string -- a controlled
/// candidate with an obviously non-date-time value must be rejected.
#[test]
fn superseded_status_malformed_superseded_at_is_rejected() {
    let repo = Repo::new();
    repo.plan(
        "malformed-at",
        "superseded_by: replacement-plan-id\nsuperseded_decision: 20260913T130413Z-governor-reviewer-td7j.md\nsuperseded_at: definitely-not-rfc3339\n",
    );
    let (code, out) = repo.validate("docs/plans/malformed-at.md");
    assert_ne!(code, 0, "{out}");
}

/// Gate-self-test negative (QBKA-A1, round 2): a syntactically valid
/// RFC3339 timestamp with a non-UTC offset must still be rejected --
/// `format: date-time` alone accepts any offset, but the field is
/// documented as UTC-only, matching this project's own Z-suffixed
/// timestamp convention.
#[test]
fn superseded_status_non_utc_offset_superseded_at_is_rejected() {
    let repo = Repo::new();
    repo.plan(
        "offset-at",
        "superseded_by: replacement-plan-id\nsuperseded_decision: 20260913T130413Z-governor-reviewer-td7j.md\nsuperseded_at: \"2026-09-13T15:00:00+02:00\"\n",
    );
    let (code, out) = repo.validate("docs/plans/offset-at.md");
    assert_ne!(code, 0, "{out}");
}

/// Gate-self-test negative: all three provenance fields missing.
#[test]
fn superseded_status_missing_all_provenance_is_rejected() {
    let repo = Repo::new();
    repo.plan("missing-all", "");
    let (code, out) = repo.validate("docs/plans/missing-all.md");
    assert_ne!(code, 0, "{out}");
    assert!(out.contains("superseded_by"), "{out}");
    assert!(out.contains("superseded_decision"), "{out}");
    assert!(out.contains("superseded_at"), "{out}");
}

/// Gate-self-test negative: an empty-string provenance field is rejected
/// (minLength: 1), not merely a missing key.
#[test]
fn superseded_status_empty_provenance_field_is_rejected() {
    let repo = Repo::new();
    repo.plan(
        "empty-by",
        "superseded_by: \"\"\nsuperseded_decision: 20260913T130413Z-governor-reviewer-td7j.md\nsuperseded_at: 2026-09-13T13:04:13Z\n",
    );
    let (code, out) = repo.validate("docs/plans/empty-by.md");
    assert_ne!(code, 0, "{out}");
}

/// Regression: the four pre-existing statuses are unaffected by this change.
#[test]
fn pre_existing_statuses_still_validate_without_provenance_fields() {
    let repo = Repo::new();
    for status in ["pending-gate1", "approved", "active", "complete"] {
        let name = format!("regress-{status}");
        repo.write(
            &format!("docs/plans/{name}.md"),
            &format!(
                "---\nid: {name}\nstatus: {status}\nadvisory_consultation:\n  - role: infosec-sentinel\n    status: consulted\n  - role: contribution-auditor\n    status: consulted\n---\nPlan.\n"
            ),
        );
        let (code, out) = repo.validate(&format!("docs/plans/{name}.md"));
        assert_eq!(code, 0, "status {status}: {out}");
    }
}

// Issue #130: schema-owned `allOf`/`if`/`then` conditional blocking
// `status: approved|active|complete` while any `advisory_consultation`
// row is `pending`. Gate-self-test-rule negative fixtures per
// docs/plans/20260915T075705Z-issue130-pending-advisory-consultation-status.md's
// Test strategy.

/// Gate-self-test negative: a required-role row `pending` at `status:
/// approved` is rejected.
#[test]
fn pending_required_role_at_approved_is_rejected() {
    let repo = Repo::new();
    repo.plan_with_status(
        "pending-required",
        "approved",
        "  - role: infosec-sentinel\n    status: consulted\n  - role: contribution-auditor\n    status: pending\n",
    );
    let (code, out) = repo.validate("docs/plans/pending-required.md");
    assert_ne!(code, 0, "{out}");
}

/// Gate-self-test negative: an *optional*-role row `pending` is rejected
/// too -- the schema's per-item constraint is uniform across required and
/// optional rows, not required-role-only (round-2 independent audit rhs6's
/// suggested safer default).
#[test]
fn pending_optional_role_at_approved_is_also_rejected() {
    let repo = Repo::new();
    // Declare fact-checker active so the failure is isolated to the new
    // schema conditional, not the unrelated "unknown advisory role"
    // fallback that fires when AGOV_MANIFEST_PATH is absent.
    repo.write(
        "MANIFEST.yaml",
        "agents:\n  fact-checker:\n    status: active\n",
    );
    let manifest_path = repo.0.path().join("MANIFEST.yaml");
    repo.plan_with_status(
        "pending-optional",
        "approved",
        "  - role: infosec-sentinel\n    status: consulted\n  - role: contribution-auditor\n    status: consulted\n  - role: fact-checker\n    status: pending\n",
    );
    let (code, out) =
        repo.validate_with_manifest("docs/plans/pending-optional.md", Some(&manifest_path));
    assert_ne!(code, 0, "{out}");
    // Pin the diagnostic to the schema's pending conditional specifically
    // (pointer at the fact-checker row, index 2), not merely any nonzero
    // exit -- a masking "unknown advisory role" error would also exit
    // nonzero and must not make this test pass for the wrong reason.
    assert!(
        out.contains("#/advisory_consultation/2/status") && out.contains("pending"),
        "{out}"
    );
    assert!(!out.contains("unknown advisory role"), "{out}");
}

/// Positive, mid-draft: `pending` is legitimate at `status: pending-gate1`.
#[test]
fn pending_row_at_pending_gate1_status_is_valid() {
    let repo = Repo::new();
    repo.plan_with_status(
        "mid-draft",
        "pending-gate1",
        "  - role: infosec-sentinel\n    status: consulted\n  - role: contribution-auditor\n    status: pending\n",
    );
    let (code, out) = repo.validate("docs/plans/mid-draft.md");
    assert_eq!(code, 0, "{out}");
}

/// Positive, flipped: the same plan with the row resolved to `consulted`
/// validates cleanly at `status: approved` (matches Row 6 S4's own real
/// session experience).
#[test]
fn resolved_row_at_approved_is_valid() {
    let repo = Repo::new();
    repo.plan_with_status(
        "resolved",
        "approved",
        "  - role: infosec-sentinel\n    status: consulted\n  - role: contribution-auditor\n    status: consulted\n",
    );
    let (code, out) = repo.validate("docs/plans/resolved.md");
    assert_eq!(code, 0, "{out}");
}

/// `status: active` and `status: complete` block on `pending` too, not
/// only `approved` -- the same defect at any post-Gate-1 status.
#[test]
fn pending_row_blocks_at_active_and_complete_too() {
    let repo = Repo::new();
    for status in ["active", "complete"] {
        let name = format!("blocked-{status}");
        repo.plan_with_status(
            &name,
            status,
            "  - role: infosec-sentinel\n    status: consulted\n  - role: contribution-auditor\n    status: pending\n",
        );
        let (code, out) = repo.validate(&format!("docs/plans/{name}.md"));
        assert_ne!(code, 0, "status {status}: {out}");
    }
}

/// Backward compatibility: every pre-existing enum value, at every plan
/// status, continues to validate exactly as before the `pending` addition.
#[test]
fn pre_existing_advisory_status_values_still_validate_at_every_plan_status() {
    let repo = Repo::new();
    for plan_status in ["pending-gate1", "approved", "active", "complete"] {
        for row_status in ["consulted", "not-applicable"] {
            let name = format!("compat-{plan_status}-{row_status}");
            repo.plan_with_status(
                &name,
                plan_status,
                &format!(
                    "  - role: infosec-sentinel\n    status: {row_status}\n  - role: contribution-auditor\n    status: consulted\n"
                ),
            );
            let (code, out) = repo.validate(&format!("docs/plans/{name}.md"));
            assert_eq!(code, 0, "{plan_status}/{row_status}: {out}");
        }
        let name = format!("compat-{plan_status}-waived");
        repo.plan_with_status(
            &name,
            plan_status,
            "  - role: infosec-sentinel\n    status: waived\n    reason: test\n  - role: contribution-auditor\n    status: consulted\n",
        );
        let (code, out) = repo.validate(&format!("docs/plans/{name}.md"));
        assert_eq!(code, 0, "{plan_status}/waived: {out}");
    }
    // The fifth plan status, `superseded`, was omitted above (round-2 Gate 2
    // review j78p) -- it needs its own provenance fields, so it can't share
    // `plan_with_status`'s bare front matter. The new `pending` conditional
    // only triggers on approved/active/complete, so `superseded` is
    // unaffected either way; this closes the claimed "every plan status"
    // coverage gap directly rather than narrowing the claim.
    for (row_status, extra) in [
        ("consulted", ""),
        ("not-applicable", ""),
        ("waived", "    reason: test\n"),
    ] {
        let name = format!("compat-superseded-{row_status}");
        repo.write(
            &format!("docs/plans/{name}.md"),
            &format!(
                "---\nid: {name}\nstatus: superseded\nsuperseded_by: replacement-plan-id\nsuperseded_decision: 20260913T130413Z-governor-reviewer-td7j.md\nsuperseded_at: 2026-09-13T13:04:13Z\nadvisory_consultation:\n  - role: infosec-sentinel\n    status: {row_status}\n{extra}  - role: contribution-auditor\n    status: consulted\n---\nPlan.\n"
            ),
        );
        let (code, out) = repo.validate(&format!("docs/plans/{name}.md"));
        assert_eq!(code, 0, "superseded/{row_status}: {out}");
    }
}
