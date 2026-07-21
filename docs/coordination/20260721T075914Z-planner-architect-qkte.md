---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260721T075914Z-planner-architect-qkte
timestamp: 2026-07-21T07:59:14Z
model: codex-gpt-5.6-sol
immutable: true
references-to:
  - 20260721T002215Z-governor-reviewer-7mg5
  - 20260721T074951Z-contribution-auditor-rgbr
topic: philosophical-lenses-learning-additions-post-v2-3-implementation
requires-ack: true
artifacts_owned:
  - agf/docs/site/docs/concepts/philosophical-lenses.md
  - docs/sources.md
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - schedule and implement the approved six learning-focused philosophical lenses after v2.3.0
    - obtain independent content and citation review before Gate 2
  must_read:
    - docs/plans/philosophical-lenses-learning-additions.md
    - docs/coordination/20260721T002215Z-governor-reviewer-7mg5.md
  do_not_touch:
    - agf/VERSION
    - tooling/rust/Cargo.toml
    - tooling/rust/Cargo.lock
---

# Future-version implementation: six learning-focused lenses

The approved `philosophical-lenses-learning-additions` plan remains real work,
not completed work. Implement it in a future version **after v2.3.0**. It is not
a v2.3.0 dependency and must not be rushed into that release.

The future implementation must add the six approved sections:

1. Observe-Orient-Decide-Act (OODA);
2. Plan-Do-Check-Act / Plan-Do-Study-Act (PDCA/PDSA);
3. single-loop versus double-loop learning;
4. espoused theory versus theory-in-use;
5. Cynefin; and
6. SECI and the concept of *ba*.

Follow the plan's citation corrections, evidence-status boundaries, and page
structure. Before Gate 2, verify every historical/theoretical attribution,
check overlap with the Learning loop and evidence page, run strict MkDocs, and
obtain independent review of both conceptual accuracy and source quality.

Completion requires an implementation MR, green validation, a durable Gate 2
handoff, and governor merge approval. The approved plan alone is not evidence
that these sections ship.
