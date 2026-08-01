---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260801T080912Z-coder-tester-2rpj
timestamp: 2026-08-01T08:09:12Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260801T074929Z-coder-tester-hj5s
topic: ai-figure-accessibility-and-transparency
requires-ack: false
artifacts_owned:
  - README.md
  - agf/docs/build/hooks/figure_accessibility.py
  - agf/docs/build/hooks/release_info.py
  - agf/docs/figures/
  - agf/docs/site/
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the expanded AI-assistance acknowledgement and public-figure accessibility/disclosure implementation for Gate 2.
  must_read:
    - README.md
    - agf/docs/build/hooks/figure_accessibility.py
    - agf/docs/site/reference/figures.md
  do_not_touch:
    - .claude/settings.local.json
---

# AI-figure accessibility and transparency amendment ready for Gate 2

The governor expanded the acknowledgement task before Gate 2: every public
figure created with AI assistance must have descriptive alternative text and a
visible AI-generation disclosure, and the policy must appear in the root README
and generated release-information page.

This message amends the earlier handoff, whose three-file scope is now stale.
The work remains a documentation fast track: the repeated page edits, SVG
metadata, deterministic figure regeneration, and docs-build regression check
implement one publication invariant. The change does not alter an adopter
schema, runtime/API surface, security boundary, migration, version, release,
issue state, or gate authority. Gate 2 remains required.

The implementation:

- gives all 34 public Markdown figure embeds descriptive alt text and an
  adjacent visible disclosure that the figure is AI-generated and publication
  remains the human maintainer's responsibility;
- states the same publication rule in the README, generated release page, and
  figure authoring guidance;
- adds a strict-build hook and focused tests that reject generic/missing alt
  text or a missing nearby disclosure;
- adds title/description metadata to all 13 public SVG figures, regenerates
  byte-identical authoring/site copies, and fixes the renderer's documented
  relative-path invocation; and
- carefully describes the measure as supporting Article 50 transparency and
  accessibility objectives, not as legal certification. Article 50's duties
  are content- and role-specific; the project voluntarily applies the visible
  disclosure rule more broadly to technical diagrams.

Validation completed:

- docs hook unit tests: 27 passed;
- relative-path SVG renderer regression test: passed;
- strict MkDocs build including the new hook: passed;
- all 13 SVG pairs contain title/description metadata and are byte-identical;
- palette checker self-test and complete WCAG contrast scan: passed;
- `agov validate`: passed with pre-existing warnings only; and
- `git diff --check`: passed.

The academic-paper PDF figures are outside the public Pages/README embed
inventory enforced here. True PDF alternative text requires a separately
validated tagged-PDF toolchain; this handoff does not claim that existing
academic PDFs are accessible or legally certified.
