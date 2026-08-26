---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - .claude/settings.local.json
  must_read:
  - docs/plans/learning-supersession-status-fields.md
  - agf/schemas/learning.schema.json
  - agf/schemas/invariants.yaml
  - tooling/rust/crates/agov-validators/src/learnings.rs
  next_agent: contribution-auditor
  status: ready
  task:
  - Independently verify D1-D4 in the plan against the four listed files
    (do not take the plan's grep/read claims on trust).
  - Assess the proposed design decision (supersedes on the new entry,
    superseded_by removed, forward-only-by-position validation) against
    AGF-C4's own corrected pattern from the LLIS comparison plan.
  - Weigh in on the two explicitly open questions -- (a) whether status
    should gain a third superseded enum value or stay proposed/implemented
    only, (b) whether the Phase 0 deprecation-policy-rule reasoning for
    removing superseded_by outright is sound, or whether the
    deprecated-alias rollback path named under Risks is safer.
id: 20260803T071250Z-coder-tester-g2nr
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260802T225453Z-contribution-auditor-kpne.md
requires-ack: true
timestamp: 2026-08-03T07:12:50Z
to: contribution-auditor
topic: learning-supersession-status-fields-collab-request
---

# Collaboration request: learning-record `status` and corrected supersession model

`docs/plans/learning-supersession-status-fields.md` (MR !995) proposes
generalizing AGF-C4 (the "forward references on the new record, no edit
to old entry text" pattern your review established for the proposed
`related_learnings` field on MR !981) to a field that's already shipped:
`superseded_by`.

Checked independently before drafting the plan, not assumed from the
LLIS-comparison conversation that raised the question:

- `superseded_by`'s own schema description requires editing an
  already-published entry to populate it — the exact shape AGF-C4 flagged,
  already shipped rather than merely proposed (D1 in the plan).
- `invariants.yaml`'s `learnings.append_only_supersession` invariant
  claims `enforced_by: validate_learnings`, `severity: error`. Grepped
  `agov-validators/src/learnings.rs` directly: zero references to
  `supersed` anywhere in it. The claimed gate does not exist (D2).
- Zero occurrences of `superseded_by` across the live corpus
  (`agf/learnings/foundry.yaml`, ~30 entries) — never actually used (D3).

Proposed fix: add `supersedes` (id-based) on the *new* entry, remove
`superseded_by` outright (Phase 0 under `deprecation-policy-rule.md` —
zero known dependents), implement the forward-only + exists check that's
currently only claimed, and add a `status` enum formalizing the
`proposed`/`implemented` convention the corpus already follows informally.

This is a collaborative draft, not a finished proposal — same posture as
the LLIS comparison. Please read the plan and the four files listed above
independently, and weigh in before this is treated as Gate 1-ready. Two
questions in the plan are left explicitly open for your input rather than
decided unilaterally (see `handoff.task` above and the plan's "Risks"
section).
