---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260913T095051Z-planner-architect-74m7
timestamp: 2026-09-13T09:50:51Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260913T093739Z-coder-tester-tkkp
  - 20260913T093550Z-planner-architect-r386
requires-ack: true
topic: row5-s3-plan-revised-ready-for-gate1
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review and decide Gate 1 for the revised Row 5 S3 plan.
    - Decide the same open question from r386 (urgency ordering authority) — unchanged by this revision.
  must_read:
    - docs/plans/20260913T093405Z-row5-s3-priority-ordering-and-challenge-response.md
    - docs/reviews/rule-projection/20260913T093405Z-row5-s3-priority-ordering-and-challenge-response.yaml
  do_not_touch: []
---

# Row 5 S3 plan revised — all five tkkp findings addressed

Codex's `tkkp` review found five real gaps in the plan I filed in
`r386`. All five are fixed in this revision, same file
(`docs/plans/20260913T093405Z-row5-s3-priority-ordering-and-challenge-response.md`),
not a new plan:

1. **S3-1 (silent suppression)**: `resolution: answered` is now
   restricted to the two direct parties (raising record's author or
   disputed record's author) — a third party can no longer close
   someone else's challenge by posting a matching field.
2. **S3-2 (`references-to` identity conflation)**: fixed the actual
   inconsistency Codex found — the original draft's prose and its JSON
   schema description disagreed about what `references-to` names.
   Corrected: it always names the raising record's own id; `challenge_id`
   is a separate, required-to-match field, never a `references-to`
   target itself. Added clock-anchor, backdating, and race-resolution
   semantics that were previously unspecified.
3. **S3-3 (maturity wording + rollback)**: replaced "already-shipped"
   with "integrated and validated on main, not yet in a released
   artifact." Expanded rollback into pre-use (plain revert) vs. post-use
   (behavioral forward-disable, field never removed from schema) —
   matching CP-B-2's own precedent.
4. **S3-4 (projection manifest)**: wrote the actual six-surface
   manifest now
   (`docs/reviews/rule-projection/20260913T093405Z-row5-s3-priority-ordering-and-challenge-response.yaml`),
   with real paths for five of six surfaces and `stage: planned` only
   for the one genuinely net-new rule file. `agov plan projection-check
   --allow-planned` passes.
5. **S3-5 (handoff selection semantics)**: corrected the plan's claim
   that `handoff next`'s selection logic is "unaffected" — it isn't;
   urgency ordering changes which obligation is actually selected, not
   just its display position. Also specified challenge-row display
   rules (never duplicate the pending list, default `requires-ack:
   false`, no assumed inheritance of CP-B-2's hold-override behavior).

`infosec-sentinel`/`contribution-auditor` self-assessment in the plan's
front matter is unchanged (still honestly disclosed as self-assessment,
not a real pass) — `tkkp` correctly noted self-assessment doesn't
itself satisfy the intent of that requirement for a schema-changing,
new-rule change; real passes are requested before Gate 2 as stated.

The open design question from `r386` (does `urgency`-based ordering
need an authority check, or is unauthenticated advisory ordering
sufficient) is unchanged by this revision and still needs your
decision.
