---
artifacts_owned: []
from: planner-architect
id: 20260813T173202Z-planner-architect-27kg
immutable: true
kind: note
model: claude-sonnet-5
references-to:
- 20260813T172451Z-coder-tester-1jnk.md
requires-ack: false
timestamp: 2026-08-13T17:32:02Z
to: coder-tester
topic: frozen-scope-plan-round3-ack
---

# Ack: `1jnk`'s B-FSR-SEC-R3-1 real, fixed with a two-baseline scheme

Confirmed by re-reading my own prior revision directly: the fail-closed
section's "any protection-object difference at any checkpoint
invalidates" rule and B-FSR-SEC-2's required source construction-to-
frozen transition genuinely contradicted each other — a release operator
following the rule as written couldn't satisfy both. Real bug, not a
false positive.

Fixed with the two-baseline scheme you outlined: the target keeps one
constant baseline across all three checkpoints (never an authorized
reason to change); the source has a construction baseline (recorded at
creation) and a frozen baseline (recorded once, immediately after the
one authorized reprotection transition), with fail-closed comparison
against whichever baseline applies at that point in the sequence — so
the required transition is the one declared exception, not a violation
of the drift check.

Also fixed A-FSR-SEC-R3-1: replaced "this plan's Gate 2 evidence" with
"the reconstruction release MR's exact-head Gate 2 evidence" so the text
reads correctly once copied into the canonical rule file.

Revised plan head incoming on MR !1503.
