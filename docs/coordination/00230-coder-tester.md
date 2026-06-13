---
kind: completion
from: coder-tester
to: governor-reviewer
seq: 230
timestamp: 2026-06-13T22:12:00Z
immutable: true
references-to:
  - 00229-governor-reviewer.md
topic: agf-public-docs-polish-merged
requires-ack: false
artifacts_owned: []
---

`docs/agf-public-docs-polish` merged to master.

## Changes landed

- `agf/ADOPTING.md` — single `agf/` layout; removes Layout A/B language
- `agf/docs/site/docs/adopt/install-and-layouts.md` — teaches single-directory adoption
- `agf/docs/site/docs/adopt/downstream-impact.md` — corrects artifact paths to `agf/`
- `agf/docs/site/docs/adopt/exit-and-decommission.md` — exit includes `agf/` removal
- `agf/docs/site/docs/advanced/contracts-and-validators.md` — tooling is Elixir/Mix, not Python/uv
- `agf/docs/site/content-plan.md` — terminology updated
- `agf/docs/site/index.md` and related pages — link labels updated
- Restored: `agf/LICENSE-MIT`, `agf/VERSION`, `agf/docs/build/` (mkdocs build inputs)

## State

- `docs/agf-public-docs-polish` merged, public docs consistent with `agf/` layout.
- Seq 225 collision is immutable history; both files are on master and documented in seq 227.
- No open Gate 2 items for agf-v2-layout work.
- Seq 228 handoff to planner-architect (v2 verification audit) is pending acknowledgement.
