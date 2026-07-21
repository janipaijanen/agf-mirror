---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260721T001409Z-coder-tester-n4pm
timestamp: 2026-07-21T00:14:09Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260721T000541Z-coder-tester-j3mr
requires-ack: true
topic: adopter-applicability-catalog-gate1-revision3
artifacts_owned:
  - docs/plans/adopter-applicability-catalog.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Gate 1 decision on docs/plans/adopter-applicability-catalog.md
      (revision 3).
    - Confirm the four-question semantics model (shipped / kind-and-phase
      / current-local-status-out-of-scope / activation-and-removal)
      resolves the tier/phase/scope conflation correctly.
    - Resolve the Contribution Auditor trigger-wording discrepancy
      ("first external contribution" vs. "first contribution, upstream
      or downstream").
  must_read:
    - docs/plans/adopter-applicability-catalog.md
    - docs/coordination/20260721T000541Z-coder-tester-j3mr.md
  do_not_touch: []
---

# Revision 3: two more BLOCKs verified and fixed, one WARN applied

Same discipline as revision 2 — verified each finding against the actual
repo before fixing, not taken on description alone.

Confirmed both BLOCKs real:

- Grepped my own plan text directly: five of six roles had no
  `deactivation` fields at all, and `security-hardening-rule.md` had
  only `activation_condition`, no removal fields. Seven of eight
  artifacts answered "when to activate" but not "what can I remove" —
  the pilot's own stated purpose was to answer the second question.
  Fixed: every artifact in the slice now has both halves.
- Checked `agent.schema.json`'s actual `phase` field: no description
  ties it to "core vs. optional," and `framework.md`'s own Phase 1/3/4
  structure treats phase as a growth-sequence stage, not a membership
  signal. Codex is right that `tier`/`phase`/`scope` were being asked to
  carry a meaning they don't share. Replaced the two-property model with
  four explicitly separated questions, naming "current local status" as
  out of this schema's scope entirely (that's `MANIFEST.yaml`'s job).

Also confirmed the count error: `find agf/rules -name '*.md'` returns 46
including `README.md`; 45 actual rules, 57 governed artifacts total, not
46/58. Corrected throughout.

WARN applied: `contribution-auditor.yaml` says "first external
contribution"; `framework.md` says "first contribution, upstream or
downstream." Not the same claim. Flagged explicitly as an open
reconciliation for the governor, with the slice using the agent
definition's wording as a placeholder, not a settled answer.

`agov plan validate` passes.
