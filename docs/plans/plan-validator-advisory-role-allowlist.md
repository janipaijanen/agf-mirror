---
id: plan-validator-advisory-role-allowlist
status: superseded
topic: plan-validator-advisory-roles
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: tooling logic change to a validator's role-name allowlist; no secrets, auth, or security-boundary code touched
  - role: contribution-auditor
    status: not-applicable
    reason: internal bug fix in existing Rust tooling, not an external contribution; fast-track eligible per simplicity-rule (single-file bug fix with confirmed root cause)
---

# Plan: `agov plan validate` rejects `build-engineer` despite it being a real AGF role (issue #30)

**Date:** 2026-07-20T07:55:03Z
**Status:** SUPERSEDED — never Gate 1 approved. Issue #30 was instead resolved
under `docs/plans/issue30-plan-validator-role-status-diagnostics.md`
(Gate 1 approved by governor-reviewer 2026-07-24T10:10:00Z), which keeps
`suspended` roles rejected but reports their authoritative status instead of
this plan's proposal to widen the allowlist to accept them. That plan is
implemented on `fix/issue30-role-status-diagnostics` (MR !463), pending
Gate 2. This document is retained for its root-cause analysis and reproduction
steps only; do not implement the allowlist-widening fix described below.
**Label note:** this issue should get the `~bug` label; currently unlabeled.

**Meta-note:** this plan itself fixes the exact validator this front matter is
checked against. Its `advisory_consultation` array deliberately uses only
`contribution-auditor` and `infosec-sentinel` (the two roles the schema/validator
already accept unconditionally) so this plan file validates cleanly under the
*current, unfixed* validator. `build-engineer` is not listed here even though it
would be thematically the right advisory role to consult on a tooling/validator
change, precisely because listing it would currently fail with the bug this plan
fixes. Once the fix in this plan lands, future plans MAY list `build-engineer`.

---

## Intent

Fix `agov plan validate` so it accepts `build-engineer` (and other real,
manifest-defined AGF roles that are temporarily `suspended`) as valid entries
in a plan's `advisory_consultation` array, instead of rejecting them with
`unknown advisory role: build-engineer`.

## Root cause (confirmed by reproduction, per debugging-rule)

**Reproduction.** A minimal plan file with a `build-engineer` entry in
`advisory_consultation` was validated against the real repo state:

```
$ cargo run -q -p agov --manifest-path tooling/rust/Cargo.toml -- plan validate /tmp/.../repro-issue30.md
ERR severity: blocking /tmp/.../repro-issue30.md:
    #/advisory_consultation: unknown advisory role: build-engineer
```

This confirms the bug exactly as reported in the issue.

**Fault location.** `agf/schemas/gate1-plan.schema.json` was read directly and
confirmed to impose no enumeration on `advisory_consultation[].role` — it only
requires a non-empty string. The unknown-role rejection is NOT a schema
restriction; it comes from application-level semantics in
`tooling/rust/crates/agov-coord/src/plan_validator.rs`, function
`allowed_roles()` (lines 120-144):

```rust
fn allowed_roles() -> HashSet<String> {
    let mut roles: HashSet<String> = required_roles()
        .iter()
        .map(|role| role.to_string())
        .collect();

    let Ok(manifest) = yaml::read_yaml_file_json(&agov_core::manifest_path()) else {
        return roles;
    };

    let Some(agents) = manifest.get("agents").and_then(serde_json::Value::as_object) else {
        return roles;
    };

    for (role, attrs) in agents {
        if attrs.get("status").and_then(serde_json::Value::as_str) == Some("active") {
            roles.insert(role.clone());
        }
    }

    roles
}
```

`allowed_roles()` already reads every role from `agf/agents/MANIFEST.yaml`
(not a hardcoded list) — but it only admits roles whose `status` field is
exactly `"active"`. `agf/agents/MANIFEST.yaml` records:

```yaml
build-engineer:
  status: suspended
  reason: "no CI pipeline yet; agov validators run manually"
  resume_when: "CI pipeline established for automated validation"
```

`build-engineer` is a real, fully-defined AGF role
(`agf/agents/build-engineer.yaml` exists and documents it owning CI/build
automation) — it is simply `suspended` in this project's own manifest because
there is no CI pipeline yet. The validator conflates "is this role fully
active in the day-to-day agent rotation" with "is this a legitimate role a
plan may list an advisory consultation for." Those are different questions:
a plan can meaningfully record "I asked build-engineer's perspective on this
CI/HPC design" even while the role is not currently active as a full
participant in the coordination channel.

**Root cause, one sentence:** `allowed_roles()` filters manifest agents to
`status == "active"` only, so any role marked `suspended` (a real, defined
role that is temporarily inactive) is misclassified as `unknown` — the
allowlist conflates "currently active in the rotation" with "a legitimate
role name," when the schema itself already treats `role` as an open string
and only the Rust semantics layer imposes closure.

## Fix

Widen the status filter in `allowed_roles()` to admit `active` and
`suspended` roles, while continuing to exclude `skipped` roles (roles the
project has explicitly decided do not apply here, e.g.
`documentation-writer`, `deployment-engineer`, `academic-visual-reviewer` —
excluding these remains correct: they are not roles anyone should be
consulting for this project).

```rust
for (role, attrs) in agents {
    let status = attrs.get("status").and_then(serde_json::Value::as_str);
    if matches!(status, Some("active") | Some("suspended")) {
        roles.insert(role.clone());
    }
}
```

This is additive and minimal: it does not change `required_roles()`
(`infosec-sentinel`, `contribution-auditor` remain mandatory), does not
change schema shape, and does not affect any currently-passing plan file
(widening an allowlist cannot turn a previously-valid plan invalid).

## Affected files

| File | Action |
|---|---|
| `tooling/rust/crates/agov-coord/src/plan_validator.rs` | Modify `allowed_roles()` status filter; add `#[cfg(test)] mod tests` |
| `agf/docs/site/docs/advanced/contracts-and-validators.md` | Update, if it documents the advisory-role allowlist rule (grep and confirm during implementation; add one sentence noting `suspended` roles are consultable, `skipped` are not) |
| `CHANGELOG.md` | Add an `[Unreleased]` entry — user-visible validator behavior change |

## Migration needed

No. This is a pure widening of an allowlist; no schema change, no data
migration, no existing plan file becomes invalid.

## Test strategy

Add `#[cfg(test)] mod tests` inside `plan_validator.rs` (no existing test
coverage exists for this file today — confirmed by searching
`tooling/rust/crates/agov-coord/` for a `tests/` directory or inline
`#[test]`, none found). Tests set `AGOV_MANIFEST_PATH` to a temporary fixture
manifest (the function already honors this env var via
`agov_core::manifest_path()`), so tests do not depend on this repo's own
`agf/agents/MANIFEST.yaml` staying in its current shape.

Unit tests (all against `validate_semantics()` / `allowed_roles()`):

1. `suspended_role_is_allowed` — fixture manifest with a role `status:
   suspended` listed in `advisory_consultation` → no "unknown advisory role"
   finding.
2. `skipped_role_is_rejected` — fixture manifest with a role `status: skipped`
   listed in `advisory_consultation` → "unknown advisory role" finding
   (regression guard: skipped roles must stay excluded).
3. `active_role_is_allowed` — existing behavior preserved (no regression).
4. `truly_unknown_role_is_rejected` — a role name absent from the manifest
   entirely → "unknown advisory role" finding (regression guard: this is the
   exact case the allowlist exists to catch — a typo'd role name).
5. Integration-style: run `cargo run -p agov -- plan validate` against the
   real repo's `agf/agents/MANIFEST.yaml` with a `build-engineer` entry (the
   reproduction case above) → passes after the fix, confirming the reported
   bug is resolved end-to-end, not just at the unit level.

## Risks

Low. The only behavior change is that `suspended`-status roles become valid
advisory-consultation entries; this cannot break any plan that currently
validates (widening a set is backward compatible), and `skipped` roles
remain correctly rejected.

## Rollback

Revert the one-line status-filter change in `allowed_roles()`. No schema or
data changes to unwind.
