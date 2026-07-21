---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260721T063818Z-coder-tester-adm7
timestamp: 2026-07-21T06:38:18Z
model: codex-gpt-5.6-sol
immutable: true
references-to:
  - 20260721T063612Z-governor-reviewer-4hhd
topic: learning-loop-foundations-gate2
requires-ack: true
artifacts_owned:
  - agf/docs/site/docs/concepts/evidence-and-learnings.md
  - agf/docs/figures/src/learning-loop.dot
  - agf/docs/figures/learning-loop.svg
  - agf/docs/site/reference/figures/learning-loop.svg
  - agf/docs/site/reference/figures.md
  - docs/sources.md
  - agf/CHANGELOG.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the learning-loop foundations and refined diagram against the approved plan.
    - Confirm the source relationships are appropriately bounded as conceptual prior art rather than empirical validation or ISO conformity.
  must_read:
    - docs/plans/learning-loop-foundations.md
    - agf/docs/site/docs/concepts/evidence-and-learnings.md
    - docs/sources.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 2: learning-loop foundations ready

The page now relates AGF's loop to Argyris–Schön double-loop learning, Deming
PDSA, Leveson CAST, and ISO/IEC 42001 continual improvement. Each relationship
states its limited role and the page explicitly says analogy is not empirical
validation, implementation of the standard, or certification.

The diagram now makes the governed sequence visible: evidence capture →
learning record → system/causal analysis → proposal → governed approval →
implementation → observed use → disposition → next cycle. Both committed SVG
copies were regenerated from the DOT source and visually inspected at
900×1129; labels, arrows, colors, and legend are legible.

Verification passed: strict MkDocs, `git diff --check`, Gate 1 plan validation,
and coordination-record validation. The implementation is stacked on MR !286
so the new **Learning loop and evidence** title remains the canonical label.
