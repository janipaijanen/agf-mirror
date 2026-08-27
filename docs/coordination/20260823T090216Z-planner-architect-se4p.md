---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  must_read:
  - docs/plans/20260823T085346Z-agov-version-drift-warning.md
  - GitLab issue #114
  next_agent: contribution-auditor
  status: ready
  task:
  - Independently review the Gate 1 plan and its bundled implementation
    (MR !1785, exact head dce9cab192983068a71f958aa66573d876ba2a73) and
    publish CLEAR or concrete findings.
id: 20260823T090216Z-planner-architect-se4p
immutable: true
kind: handoff
model: claude-sonnet-5
references-to: []
requires-ack: false
timestamp: 2026-08-23T09:02:16Z
to: contribution-auditor
topic: issue-114-version-drift-plan-and-impl-ready
---

# Issue #114: version-drift warning plan and implementation ready

Human governor directed immediate implementation, targeted at the next
release, after I identified (during a direct exchange, not a prior
incident) that `agov` never compares its own build version against the
commissioned `agf/VERSION` of the tree it runs against. Filed as GitLab
issue #114.

Published `docs/plans/20260823T085346Z-agov-version-drift-warning.md`
(MR !1785, exact head `dce9cab192983068a71f958aa66573d876ba2a73`) with
the implementation bundled in the same commit, per the governor's
explicit direction to move fast — this deviates from the usual plan-
then-implement sequencing (`development-workflow-rule.md`), and both
still need your independent review and the governor's Gate 1/Gate 2
decisions before merge.

## Design summary

- `agov-core::commissioned_version()`: reads `agf/VERSION`, `None` if
  absent.
- `agov::version_drift_warning()`: compares against
  `env!("CARGO_PKG_VERSION")`; `Some(message)` only on real mismatch.
- Wired into `run_all_validators` (`agov validate` umbrella, also what
  CI's `agov:governance:validate` runs) and `SessionGitStatus::warnings()`
  (shared by `session_resume`/`session_bootstrap` — the literal first
  cold-start command per `docs/reference/agent-cold-start-habits.md`).
- Advisory only, never blocking — no existing rule grants this check
  blocking authority (`tool-authority-boundary-rule.md`).

## Evidence

- TDD: RED confirmed before implementation (mismatch test failed for the
  right reason — no such warning existed yet).
- `cargo test --workspace`: full pass, 22 test groups.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- `agov validate` against this actual repo: `version_drift` reports OK
  (matches, as expected on a fresh checkout).
- `CHANGELOG.md`/`agf/CHANGELOG.md`: new `[Unreleased]` section, one
  `### Added` entry, matching the established incremental-changelog
  convention (precedent commit `0ad8d69e`).

Requesting your independent review — I authored both plan and
implementation, so cannot self-satisfy `6srq` for this one.
