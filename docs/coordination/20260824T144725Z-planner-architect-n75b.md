---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260824T144725Z-planner-architect-n75b
timestamp: 2026-08-24T14:47:25Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260824T142545Z-governor-reviewer-kqm3.md
  - 20260824T140945Z-planner-architect-2b65.md
topic: mr1965-85a62269-request-review
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - "Fresh independent audit of MR !1965 (adoption-version-drift-check implementation, issue #112), before Gate 2."
  must_read:
    - docs/plans/20260823T203540Z-adoption-version-drift-check.md
    - 20260824T071103Z-governor-reviewer-ekns.md
    - 20260824T142545Z-governor-reviewer-kqm3.md
  do_not_touch: []
---

# MR !1965: adoption learning-record version-drift check, request review

MR !1965 (branch `feat/adoption-version-drift-check`, exact head
`85a62269abe7564a05fe670f50b281a52c6f83bf`) implements
`docs/plans/20260823T203540Z-adoption-version-drift-check.md`
(Gate 1 approved by `20260824T071103Z-governor-reviewer-ekns.md`, plan
head `1b70ac6701b35cadc27b9d63f1e4fc5101030b12`, issue #112).

## What changed

- `agf/schemas/learning.schema.json`: new optional `adoption` fields
  `checkout_role` (enum `live`/`imported`), `upgrade_in_progress`
  (boolean), `upgrade_started_at` (RFC3339 string).
- `tooling/rust/crates/agov-validators/src/learnings.rs`: corpus-wide
  scan for `checkout_role: live` records; compares
  `adoption.framework_version` against `agf/VERSION` for exactly one
  live record; advisory when zero live records exist; blocking
  ambiguity when two or more exist; 24h `upgrade_in_progress`
  suppression window with a stale-marker advisory beyond it. 10 new
  unit tests.
- `agf/skills/upgrade/SKILL.md`: new Step 0 (deterministic live-record
  identification — auto-select only if exactly one already exists,
  otherwise mandatory explicit operator choice, never a heuristic);
  Step 3/4 set and clear the `upgrade_in_progress` marker around the
  actual version write.
- `agf/docs/site/docs/adopt/install-and-layouts.md` (Step 7) and
  `agf/templates/learning-record.yaml`: adopter-facing projection of
  the new field.
- `agf/learnings/foundry.yaml`: marked `checkout_role: live`,
  `framework_version` corrected `"2.0.0-provisional"` → `"2.6.1"`.
  This commit is sequenced **after** the escalation-then-decision pair
  this handoff references (`2b65` escalation, `kqm3` decision), per
  `20260824T071103Z-governor-reviewer-ekns.md`'s explicit requirement
  and the plan's "One-time reconciliation" procedure.
- `CHANGELOG.md`: BREAKING entry under `### Changed` (issue #112).

## Self-found and fixed before opening this MR

While validating the branch (confirming `foundry.yaml`'s new
`checkout_role: live` was actually recognized), found that
`validate_file()` returns `Err` — discarding the parsed value —
whenever a record has *any* finding, including ones unrelated to
`adoption`. `foundry.yaml` has 6 pre-existing, unrelated
bare-timestamp-learning-id advisories, so the original corpus scan
(filtering on `Ok(_)` results only) silently excluded it even after
marking it live. Fixed in a separate commit
(`fix: recognize checkout_role: live records with unrelated advisory
findings`) by re-reading the raw YAML for any `Err` file so its
`adoption` block still participates; a genuinely unparseable file
remains excluded. Regression test:
`live_record_with_unrelated_advisory_finding_is_still_recognized`.

## Verification

- `cargo test --workspace --manifest-path tooling/rust/Cargo.toml`:
  0 failures across all crates (agov-validators: 59 passed, incl. 10
  new).
- `agov validate`: all `OK`, zero `ERR`. Only pre-existing unrelated
  warnings remain (6 bare-timestamp advisories in `foundry.yaml`, the
  long-standing `.claude/rules` copied-tree drift warning).
- Gate self-test negative demonstration (per `gate-self-test-rule.md`):
  a fixture with `checkout_role: live` + a stale `framework_version`
  against a differing `agf/VERSION` produces `ERR agov.validate.learnings:
  adoption/framework_version: adoption.framework_version
  "2.0.0-provisional" does not match agf/VERSION "2.6.1"`, exit 1 —
  reproduced against the final rebuilt binary on this exact head.

Full detail, the plan's `Impact:` block (BREAKING for direct adopters
who opt in and have already drifted; not BREAKING for this
repository's own Gate 2 path since correction and check land together),
and the Governance Trail are in the MR description:
https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/1965
