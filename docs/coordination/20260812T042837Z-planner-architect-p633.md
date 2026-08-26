---
artifacts_owned: []
from: planner-architect
id: 20260812T042837Z-planner-architect-p633
immutable: true
kind: completion
model: claude-sonnet-5
references-to:
- 20260811T212756Z-planner-architect-rhrj.md
- 20260812T041137Z-planner-architect-rjhs.md
requires-ack: false
timestamp: 2026-08-12T04:28:37Z
to: all
topic: low-iid-cleanup-batch2
---

# Low-IID cleanup, second batch (10 MRs skimmed: !654–!829) — zero closures

Continuing the `rhrj` low-IID-upward pass (batch 1: `rjhs`, MR !1471).
Same method: title + description, then an "is this already on `main`?"
check for anything that looked plausibly superseded.

## Closed: none

All 10 are substantive, unmerged plans/learnings/proposals with no
supersession signal found — none demonstrably obsolete, so none
closed, per the standing instruction to close only demonstrably
obsolete MRs and retain ambiguous/substantive ones:

- **!654** — `.claude/rules/` vs `agf/rules/` reconciliation plan;
  `.claude/rules/` on `main` is still a real, separate tree (not a
  symlink/mirror), consistent with the plan's premise still holding.
- **!658** — implements three capability-review proposals the human
  Governor already approved directly ("I agree with your assessment
  and proposal"). **Flagging, not deciding**: `agf/agents/
  MANIFEST.yaml` on current `main` still shows `capability-developer:
  status: suspended` — the approved reactivation has not landed. This
  looks like ready-to-merge work sitting idle, not dead work; surfacing
  for Governor attention rather than acting on it myself (role
  authority: MANIFEST changes are not mine to merge unilaterally).
- **!739, !776, !829** — learning records; none of their stated
  learning IDs (`20260728T082825Z-m969`, `20260730T134710Z-q429`, and
  !776's release-agent-identity guardrail) appear in `main`'s
  `agf/learnings/foundry.yaml`. Genuinely unmerged.
- **!745** — issue #75 still open.
- **!800** — draft proposal, explicitly parked by its own description;
  no supersession signal.
- **!806, !807** — issues #86/#88 and #87/#90 (respectively) all still
  open; both plans cite independent contribution-auditor reproduction.
- **!810** — read-only CI/latency assessment under issue #53, which is
  still open (consistent with batch 1's !459 finding on the same
  issue).

## Scope note

Same discipline as batch 1: bounded, not exhaustive. Next slice picks
up from IID !850 upward.
