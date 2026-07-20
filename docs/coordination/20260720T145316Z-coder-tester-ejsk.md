---
kind: handoff
from: coder-tester
model: claude-sonnet-5
to: governor-reviewer
id: 20260720T145316Z-coder-tester-ejsk
timestamp: 2026-07-20T14:53:16Z
immutable: true
references-to:
  - 20260720T144305Z-coder-tester-hpsg.md
  - docs/plans/docs-audience-boundary-proposal.md
topic: docs-audience-implementation-mr1-gate2
requires-ack: false
artifacts_owned:
  - agf/docs/site/docs/index.md
  - agf/docs/figures/src/docs-audience-map.dot
  - agf/docs/figures/docs-audience-map.svg
  - agf/docs/site/reference/figures/docs-audience-map.svg
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review MR-1 (branch docs/audience-taxonomy-callout-mr1) for Gate 2.
    - Added "How to read this site" section to agf/docs/site/docs/index.md, before "## The path", stating the four-bucket taxonomy in plain language.
    - Added a new docs-audience-map DOT source and rendered SVG (both agf/docs/figures/ and agf/docs/site/reference/figures/ per existing dual-copy convention), embedded next to the callout.
    - Did not touch install-and-layouts.md (Codex's MR-4 claim) or relabel other pages (MR-2's scope).
  must_read:
    - docs/plans/docs-audience-boundary-proposal.md
    - docs/coordination/20260720T144305Z-coder-tester-hpsg.md
    - agf/docs/site/docs/index.md
  do_not_touch:
    - agf/docs/site/docs/adopt/install-and-layouts.md
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 2 handoff: MR-1 audience taxonomy callout + documentation map

Implemented MR-1 from the approved documentation audience-boundary plan on
branch `docs/audience-taxonomy-callout-mr1`.

Implementation summary:

- New "How to read this site" section on `docs/index.md`, ahead of the
  existing stage diagram, defining the four buckets (Human decision /
  AI-operator instructions / Shared concept / Reference) in one short
  paragraph each.
- New `docs-audience-map.dot` source (styled consistent with the existing
  `operating-responsibility-map.dot`: same color legend convention — orange
  for human-only, light blue for AI-executed, white/blue-outline for
  shared, gray for reference), rendered to SVG in both the existing
  dual-copy locations (`agf/docs/figures/` and
  `agf/docs/site/reference/figures/`), embedded directly under the callout.

Validation:

- `mkdocs build -f agf/docs/build/mkdocs.yml --strict` passed — only the
  pre-existing, unrelated warnings (Material for MkDocs 2.0 notice,
  `reference/figures.md`/`reference/palette.md` not in nav).
- Rendered a PNG preview directly from the DOT source and visually
  inspected it per `visible-artifact-verification-rule` — caught and fixed
  one cosmetic issue (a stray `--` placeholder in the legend's first row,
  inconsistent with the other three blank-swatch rows) before finalizing.
- `git diff --check` passed.
- Both SVG copies confirmed byte-identical (`diff` clean), matching the
  existing convention for every other figure in the repo.
