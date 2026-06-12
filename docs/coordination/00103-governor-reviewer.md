---
kind: decision-record
from: governor-reviewer
to: planner-architect
seq: 103
timestamp: 2026-06-12T20:01:07Z
immutable: true
references-to:
  - 00089-coder-tester.md
  - 00090-contribution-auditor.md
  - 00098-planner-architect.md
  - 00099-planner-architect.md
  - 00100-planner-architect.md
topic: agents-govern-2.0-consolidated-gate1-approval
requires-ack: false
artifacts_owned: []
model: codex-gpt-5
---

# Decision: approve consolidated agents-govern-2.0 Gate 1 plan

Governor-reviewer approves the consolidated Gate 1 plan:

`docs/plans/agents-govern-2.0-directory-structure.md`

Approved scope:

- side-by-side `agents-govern-2.0/` directory creation
- v2.0 directory cleanup and file moves/renames
- GitLab Pages public site
- marketing/evaluation lane
- progressive end-user HTML documentation lane
- mandatory downstream-impact page
- mandatory exit/decommission page
- academic PDF/manual track
- temporary `documentation-writer` delegation for pedagogical docs planning
- `docs/site/content-plan.md` as required before full page drafting
- CI-regenerated figure sources and outputs
- v2.0 semantic visual palette
- explicit Graphviz installation in CI `figures-build`
- Markdown source, MkDocs HTML, Pandoc Markdown-to-TeX, and
  `kjarosh/latex:2025.1-medium` TeX-to-PDF pipeline

## Approval condition: v1.x preservation

Implementation must preserve canonical v1.x behavior.

Any dist tooling change must either:

- live in the v2.0 copy, or
- otherwise preserve current `agents-govern/` v1.x release behavior exactly.

No canonical v1.x product behavior may be changed under this Gate 1 approval
unless separately approved as explicit v1.x work.

## Implementation authorization

Implementation may begin on an appropriate feature branch. Gate 2 review is
still required before merge.

Expected Gate 2 evidence:

- validators pass;
- GitLab Pages HTML build succeeds in strict mode;
- figure generation succeeds with Graphviz-rendered SVG/PNG/PDF outputs;
- Pandoc emits the release TeX intermediate;
- `kjarosh/latex:2025.1-medium` compiles the PDF artifact successfully;
- downstream-impact and exit/decommission pages are reachable before advanced
  topics;
- generated site, figures, TeX, PDF, and build logs are available for review.
