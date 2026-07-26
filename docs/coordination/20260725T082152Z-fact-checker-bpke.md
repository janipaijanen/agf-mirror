---
kind: review
from: fact-checker
to: governor-reviewer
id: 20260725T082152Z-fact-checker-bpke
timestamp: 2026-07-25T08:21:52Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260725T080107Z-governor-reviewer-k4m5
  - 20260725T073230Z-planner-architect-g2k3
topic: openable-svg-figures-factcheck
requires-ack: false
artifacts_owned:
  - agf/docs/site/index.md
  - agf/docs/site/docs/concepts/position.md
  - agf/docs/site/docs/concepts/coordination-protocol.md
  - agf/docs/site/docs/start/framework-map.md
  - agf/docs/figures/src
---

# MR !511 figure/prose fact-check — ADOPT

Scope: MR !511 at exact SHA
`004f897c98b81bc2d3ddffea32756fa55389451b`.

Coverage: **COMPLETE**.

Disposition: **ADOPT**.

Blocking findings: **0**. Advisory findings: **0**. Authority: factual
recommendation only; governor-reviewer decides Gate 2.

## Claim-preservation matrix

| Replacement | Preserved claims and relationships |
|---|---|
| Governed change flow | AI plan → human Gate 1 → AI implementation → CI validation → human Gate 2 → merge; actor classes remain distinct by label and shape. |
| Runtime/governance boundary | Runtime still owns invocation, tool call, and memory/state execution; AGF still defines roles/gates/artifacts, jurisdictions, forbidden actions, topology, and learnings; constrain/feed relationships are preserved. |
| Governance-scope position | All six categories remain; every named relationship is preserved: governs runtimes, evidence consumed by CI/CD, narrower within SDLC, different axis from ML governance, implemented within policy/ethics. |
| Gate handoff sequence | The original participants, message order, implementation, review, decision record, and completion are preserved top-to-bottom; no parallelism is introduced. |
| Framework map | Every original category and leaf concept remains. The AGF-to-category relationship labels, Gate timing labels, boundary statements, tooling commands, and closed evidence-loop edge are preserved. The vertical-row layout is a presentation change only. |

Surrounding prose retains the essential relationship descriptions and the new
alt text does not strengthen them. Each image and its distinct text link resolve
to the same standalone SVG. SVG `<title>` and `<desc>` text match the source
claims without asserting interception, compliance certification, universal
accessibility, or independent-adopter validation.

Evidence checked: zero remaining public Mermaid fences or CDN/runtime
references; deterministic byte-stable SVG regeneration; strict MkDocs;
evaluation-dossier linked-image regression test; AGF adopter-doc, learning, and
umbrella validation; direct SVG responses; desktop and 390px rendered checks.
The implementation claims only direct foundry feedback on two pages plus the
closed five-diagram inventory.

The GitLab exact-head pipeline was still running when this advisory record was
authored; Gate 2 must separately require it to finish green.
