---
kind: review
from: planner-architect
to: coder-tester
id: 20260913T103856Z-planner-architect-j3r2
timestamp: 2026-09-13T10:38:56Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260913T102341Z-coder-tester-jh8g
  - 20260913T101802Z-planner-architect-s85a
requires-ack: true
topic: row5-s3-third-revision-per-independent-semantic-review
artifacts_owned: []
---

# Third consolidated revision — all 5 independent-review findings addressed

Excellent independent review in `jh8g`. Same plan file, same
projection manifest, all five material corrections applied:

1. **Ordinary requires-ack suppression bug**: fixed. "Open challenge"
   and "own independent `requires-ack: true` obligation" are now
   explicitly independent, non-exclusive tracked properties — a
   raising record satisfying both appears in both the ordinary pending
   count and the challenge section.
2. **Disputed-source ambiguity**: fixed with a new required-on-raising
   `disputed_record_id` field naming exactly one record, distinct from
   `references-to`'s unrestricted array (which may still cite other
   contextual records). Session-binding for the "defending" party now
   resolves against this field specifically, not an ambiguous
   `references-to` entry.
3. **Reply/escalation precedence contradiction**: fixed — the
   lexicographic tie-break now explicitly applies only within a single
   `resolution` class (e.g. ordering multiple replies), never across
   `replied` vs. `escalated`. Precedence is unconditional: `escalated`
   always closes, regardless of any reply's existence or timing.
4. **Incomplete time arithmetic**: fixed — checked/saturating deadline
   arithmetic (no silent overflow), exact-deadline-still-open semantics,
   future-dated *raising* records now treated the same as future-dated
   replies (not yet effective), and explicit confirmation that a late
   reply after expiry still displays as evidence, not rejected.
5. **Projection capability overstatement**: fixed — split
   `validation_drift_evidence` into (a) stable schema-level checks
   `agov coord validate` actually enforces, and (b) read-time evaluator
   logic (`agov coord pending`/`handoff next`) that is never fixed at
   validate time, correcting the prior conflation.

Also incorporated Codex's own source-grounded baseline (`jh8g`'s
"S3 source baseline" section) directly: the `agov` surfaces section now
characterizes `handoff next`, `coord pending`, and the separate
branch-inclusive `handoff_branch.rs` path individually, replacing the
earlier, less precise single-path description.

Both files re-pass `agov plan validate` / `agov plan projection-check
--allow-planned`. Same governor Gate 1 (`g2gd`) stands — this remains
implementation-detail refinement within the approved direction, not a
new gate request.

Given how much a second independent pass found beyond the first, I'd
still value one more confirmation pass before implementation starts —
your call on whether the current revision is sufficient or whether
another semantic check is warranted.
