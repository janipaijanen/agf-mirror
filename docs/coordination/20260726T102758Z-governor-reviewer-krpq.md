---
kind: fast-track-waiver
from: governor-reviewer
to: planner-architect
id: 20260726T102758Z-governor-reviewer-krpq
timestamp: 2026-07-26T10:27:58Z
model: human-governor
immutable: true
references-to:
  - 20260726T102147Z-planner-architect-mhh1
topic: mr567-retroactive-fast-track-waiver
requires-ack: true
artifacts_owned: []
---

# Fast-Track Waiver — MR !567 (retroactive correction)

Task: relocate the existing README “Start development in this checkout” block
without changing its commands or anchor.

Why Gate 1 is waived: one-file documentation relocation with no schema,
authority, security, runtime, data-model, or adopter-contract change.

Who waived: Jani, human governor-reviewer, by explicitly approving both
dispositions in chat on 2026-07-26.

Risk assessment: low. Exact implementation SHA
`65e21fc105b04ef4c7d5850b4723a5dc6bfd2e45` preserves the content and passed
both push pipeline `2706413733` and MR pipeline `2706413840`.

Required verification at Gate 2: exact diff remains a relocation, links/anchor
remain valid, documentation checks pass, and no adjacent content changes.

Revert to full plan if: any content semantics, command, anchor, governed rule,
or adopter-facing contract must change.

This waiver is retroactive because MR !567 was already merged without the
required prior governor authorization. It corrects the authority trail; it
does not claim the original process was compliant.
