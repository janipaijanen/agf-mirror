---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261031T112235Z-planner-architect-spkg
timestamp: 2026-10-31T11:22:36Z
model: claude
immutable: true
references-to:
  - 20261031T104910Z-governor-reviewer-5gkr.md
  - 20261031T104250Z-planner-architect-k5p4.md
  - 20261031T095048Z-planner-architect-6d8r.md
topic: pages-slice-2-batch-2-ready-for-recheck
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Route the existing source and factual reviewers to recheck the batch 2 head against the five groups of review 5gkr. Rendered review waits until source is accepted.
  must_read:
    - docs/reviews/pages-structure-and-voice/20261031-slice-2-preservation-and-anchor-map.md
  do_not_touch:
    - agf/rules/
    - agf/agents/
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Pages slice 2: batch 2 ready for recheck

Under claim 6d8r. This record supersedes the readiness claim in k5p4 for head `8bbd049a`. Any change to the new head invalidates
this claim. I merge nothing to main until Gate 2 and the Pages-bearing main pipeline ends.

| Item | MR | Exact head (40 characters) | Pipeline |
|---|---|---|---|
| Slice 2 batch 2 (draft) | !3462 | `77c4ca84df2b704d89c769e69055e48aed61ed9a` | 3011682070, `success`, all seven jobs |

Jobs: commercial-corpus validate 17630039252, rust agov validate 17630039251, governance validate 17630039250, release paths
validate 17630039249, distribution validate 17630039248, distribution preview 17630039247, strict `site-html-build` 17630039246.
Base is `6bd6cfb02b594ecc5a2ce94eb96f56374cb330cd`. One push, one pipeline.

## The five groups of 5gkr

Wording for groups 1 and 2 follows the Codex draft preflight in ACS seq 1122. Groups 3 to 5 follow drafts 1120 and 1121, which both
reviewers accepted.

1. Decide Basics: "AGF defines two approval checkpoints. By default, a human approves at both."
2. Decide Assistant workflows, and `use-cases.md` (inside claim 6d8r): loading rules assigns no role and grants no approval
   authority; eligible roles only through an authorized assignment; the human governor stays accountable; parallel work needs separate
   work boundaries, claims, any required recorded and acknowledged allocation, and recorded handoffs. The sentence that role boundaries
   and claims do not themselves prevent conflicting writes is kept.
3. Catalog: described as an incomplete pilot, not a validated pruning guide. The stale "8 of 58" is removed. Coverage numbers stay on
   the catalog page. The unvalidated-pruning distinction is kept.
4. Small-team example: written conventions, intended benefit, adherence still needs review.
5. FAQ split gates: Gate 1 decision, conditions, accepted risks, diff and evidence comparison, scope changes back to Gate 1 and one
   primary authority are restored. The report no longer overstates what was kept.

## Evidence

- Original A1 script (hash `aaa2cb0444c362cc`) on the rendered artifact of job 17630039246: Evaluate answer-key sections 68, 135, 131,
  71 and 265 words = 670; with Home 312 = **982**, equal to the ceiling. Whole Decide page 2,075 words. This is disclosed growth and
  not a reading reduction.
- Extractor and anchor report regenerated. All cross-file anchors resolve. Every "not found" sentence has a disposition. Added ids only.
- Strict `agov plan projection-check` OK. The owner manifest now lists `agf/docs/site/use-cases.md`.

## Not done

No browser run, no Home edit. Gate 2 is not requested. R1, R2, Home q3 and the Operate +67 cap stay with their owners.
