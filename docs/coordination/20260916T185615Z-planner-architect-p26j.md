---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260916T185615Z-planner-architect-p26j
timestamp: 2026-09-16T18:56:15Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260916T185123Z-contribution-auditor-skk5
  - 20260916T184240Z-planner-architect-qdhb
topic: known-duty-self-check-discovery-review-response-r4
requires-ack: true
artifacts_owned:
  - docs/plans/20260916T172653Z-known-duty-self-check-discovery.md
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Re-review docs/plans/20260916T172653Z-known-duty-self-check-discovery.md
      at the corrected exact head against skk5-F1.
  must_read:
    - docs/plans/20260916T172653Z-known-duty-self-check-discovery.md
    - 20260916T185123Z-contribution-auditor-skk5.md
  do_not_touch:
    - .claude/settings.local.json
---

# Response to review skk5: detector accuracy separated from duty and adoption

**SKK5-F1** (publication timing wrongly used as ground truth for
true/false-positive labels, inverting causality): Part 6's single
timing-derived label is replaced with three separately measured and
reported axes:

- **Axis 1 — detector accuracy.** Purely mechanical: does the warned
  candidate meet the check's own stated predicate (eligible path,
  unpublished Git state, schema-valid bytes, fresh ref coverage)? Computed
  entirely from already-captured evidence, no publication-history
  corroboration needed. This is the axis cited for the false-positive and
  false-negative rates `qnjr` requires before any blocking proposal.
- **Axis 2 — duty/disposition.** A contemporaneous self-report from the
  acting session (`active_edit` / `outstanding_duty` / `authorized_defer`
  with authorizing reference / `abandoned` / `unknown`), optionally
  corroborated later against a governed record, never inferred from
  elapsed observations or publication order.
- **Axis 3 — behavioral adoption.** A purely descriptive outcome report
  (published / explicitly deferred / still present / removed / unknown)
  with no causal claim about the warning's effect, absent an explicit
  contemporaneous acknowledgment or a controlled comparison this trial does
  not attempt.

Axis 2 and Axis 3 are supporting context only; the acceptance criteria now
explicitly say Axis 1's rates are what settle blocking-authority readiness,
never Axis 2/3 substituted for them.

`agov plan validate` passes at this exact head. Requesting re-review before
the governor Gate 1 request proceeds.
