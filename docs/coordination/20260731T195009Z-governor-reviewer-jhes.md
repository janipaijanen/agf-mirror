---
artifacts_owned:
  - docs/plans/issue97-vscode-adopter-doc-accuracy.md
from: governor-reviewer
id: 20260731T195009Z-governor-reviewer-jhes
immutable: true
kind: decision-record
model: human
references-to:
- 20260731T180442Z-contribution-auditor-g1sr.md
- 20260731T182633Z-contribution-auditor-1jha.md
requires-ack: false
timestamp: 2026-07-31T19:50:09Z
to: coder-tester
topic: issue97-vscode-adopter-docs-gate1-approved
---

# Gate 1 approved: issue #97 adopter prompt and Visual Studio Code evidence

The human governor-reviewer approved Gate 1 for MR !882 at exact head
`9ba35ba78c645f44189ff2b966057dc310b25805`. Exact-head pipeline
`2722572277` passed. Independent contribution-auditor review (`g1sr`)
returned CLEAR with two advisories on the prior head; the explicit-author-
attribution advisory was incorporated ("The project author has developed
AGF and AGOV through sustained use of both the Claude and Codex
integrations in Visual Studio Code") and independently re-confirmed at the
new head (`1jha`). The disclosed telemetry-count verification limit
requires no plan change.

The approved boundary is exactly
`docs/plans/issue97-vscode-adopter-doc-accuracy.md`: a bounded public-
documentation correction — the commissioning-prompt wording, the approved
Visual Studio Code compatibility prose, and the independent-adopter-gap
framing — scoped to `agf/docs/site/docs/adopt/install-and-layouts.md`,
`agf/docs/site/docs/concepts/position.md`, `agf/docs/site/contribute.md`,
and only exact-search conflicts elsewhere, each such expansion to be
enumerated in the implementation MR.

No mdBook/Ayu migration, version bump, tag, release publication, tool
behavior, schema, validator, role, or adopter-contract change is authorized
by this decision. Issue #97 closes only after the correction is published
and verified on the public Pages site.
