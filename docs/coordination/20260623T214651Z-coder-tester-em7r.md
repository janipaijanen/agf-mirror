---
kind: note
from: coder-tester
to: all
id: 20260623T214651Z-coder-tester-em7r
timestamp: 2026-06-23T21:46:51Z
immutable: true
model: codex-gpt-5
references-to:
  - 20260623T212551Z-coder-tester-ghgk.md
topic: maturity-vocabulary-sticky
requires-ack: false
artifacts_owned:
  - AGENTS.md
  - agf/docs/site/reference/maturity-vocabulary.md
  - agf/docs/site/reference/tooling-conformance.md
  - docs/coordination/20260623T214651Z-coder-tester-em7r.md
---

# Sticky memory: use maturity labels for capability claims

AGF-NG now carries forward the useful 1.x maturity vocabulary as a sticky
AI/human reference. When describing tooling or framework capability, use:

- Level 1 Planned
- Level 2 Surface-defined
- Level 3 Instantiated
- Level 4 Validated
- Level 5 Deployable

The operator lesson is concrete: a command implemented and tested in one branch
is Level 4 there. It is not Level 5 for another checkout, wrapper, release, or
operator path until that selected binary actually includes the command and the
docs match it.

This language should be used in reviews and handoffs instead of binary
"supported/not supported" claims when maturity matters. Canonical reference:
`agf/docs/site/reference/maturity-vocabulary.md`.

Also use evidence-status labels for framework claims, principles, and
recommendations:

- Empirically validated
- Empirically informed
- Theoretical

Capability maturity answers "what exists?" Evidence status answers "how strong
is the proof behind the claim?" Use both when a claim could otherwise sound
stronger than the evidence supports.
