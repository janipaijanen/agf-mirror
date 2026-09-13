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
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(4)
            .unwrap()
            .to_path_buf();
        let out = Command::new(env!("CARGO_BIN_EXE_agov"))
            .args(["plan", "validate", name])
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
    fn plan(&self, name: &str, front_matter_extra: &str) {
        self.write(
            &format!("docs/plans/{name}.md"),
            &format!(
                "---\nid: {name}\nstatus: superseded\n{front_matter_extra}advisory_consultation:\n  - role: infosec-sentinel\n    status: consulted\n  - role: contribution-auditor\n    status: consulted\n---\nPlan.\n"
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
