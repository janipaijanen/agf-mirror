---
kind: decision-record
from: governor-reviewer
to: all
id: 20260720T224012Z-governor-reviewer-g3st
timestamp: 2026-07-20T22:40:12Z
model: human-governor-via-codex-session
immutable: true
references-to:
  - 20260720T224012Z-planner-architect-rbg1
topic: adoption-surface-clarity-gate1-approved
requires-ack: false
artifacts_owned: []
---

# Gate 1 approved: adoption-surface clarity

Codex-scribed record of the human governor's direct instruction in the active
session. After reviewing the proposed documentation response — a canonical
"installed versus activated" statement on the adoption page, explicit review
lenses on the AI evaluation landing and dossier, consistent disclosure on the
public landing and install path, and no implication that the starter product
already ships — the governor responded: "nice. please implement."

Decision: Gate 1 is approved for
`docs/plans/adoption-surface-clarity.md` as written.

Conditions:

- Preserve one canonical detailed statement and use an exact short disclosure
  elsewhere, rather than creating slightly different claims on each page.
- Do not hard-code inventory counts.
- State that selective activation is shipped guidance while the reduced
  starter product remains Level 1 Planned and lacks independent pilot evidence.
- Do not use the missing legacy `intensity-guide.md` as evidence.
- Run strict documentation build, adopter-docs validation, distribution
  validation, and rendered dossier checks before Gate 2 handoff.

Gate 2 remains a separate governor-reviewer decision after the implementation
and verification evidence are published.
