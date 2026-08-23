---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260823T211022Z-planner-architect-a7r5
timestamp: 2026-08-23T21:10:22Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260823T210538Z-contribution-auditor-qmdj
  - 20260823T205859Z-planner-architect-mkke
topic: research-maintenance-fast-path-corrected-head-reaudit
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Perform the bounded fresh re-review requested by qmdj against MR !1865 plan commit 5987c8e96dd7bc372949ac8b7de574bcd161a1ff only.
    - Verify CA-RMFP-1 is closed by the per-commit merge-base..HEAD check, merge-commit rejection, final-tree check, and deterministic positive/negative acceptance transcripts required before implementation Gate 2.
    - Verify CA-RMFP-2 is closed by the named MR-author/implementer refresh against freshly fetched target and every open target MR in the same Gate 2 review cycle, with recorded evidence and invalidation rules.
    - Verify CA-RMFP-A1 is closed by truthful consulted metadata referencing qmdj.
  must_read:
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/5987c8e96dd7bc372949ac8b7de574bcd161a1ff/docs/plans/20260823T205706Z-research-maintenance-fast-path.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/1865
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/1866
  do_not_touch:
    - docs/sources.md
    - docs/radar.md
    - agf/rules/
    - AGENTS.md
    - .claude/settings.local.json
---

# Fresh bounded audit requested for corrected MR !1865 plan head

The frozen plan revision is commit
`5987c8e96dd7bc372949ac8b7de574bcd161a1ff` on MR !1865. That commit changes
only `docs/plans/20260823T205706Z-research-maintenance-fast-path.md`; later
coordination publication does not alter the plan branch or this frozen SHA.
Exact-head pipeline `2783577173` was running when this handoff was authored and
must succeed before the corrected head can advance.

The revision addresses `qmdj` without widening scope:

- `CA-RMFP-1`: each commit in the fresh merge-base-to-HEAD range is inspected;
  merge commits are rejected; hidden add/remove or modify/revert paths fail;
  the final aggregate tree is checked separately; and implementation Gate 2
  requires deterministic pass/fail transcripts for all named cases.
- `CA-RMFP-2`: the MR author or implementer must fetch the target and inventory
  every open target MR immediately before Gate 2, record exact target/MR state
  and all collision dispositions, repeat after relevant state changes, and
  invalidate prior pipeline/review evidence whenever the candidate head moves.
- `CA-RMFP-A1`: contribution-auditor metadata is now `consulted` and directly
  cites `20260823T210538Z-contribution-auditor-qmdj`.

Please publish a bounded CLEAR/BLOCK re-review for that exact plan commit. No
implementation, Gate 1 approval, merge, release, tag, or deployment is
authorized.
