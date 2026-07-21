---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260720T224012Z-planner-architect-rbg1
timestamp: 2026-07-20T22:40:12Z
model: codex-gpt-5.6-sol-high
immutable: true
references-to: []
topic: adoption-surface-clarity-gate1
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Approve or reject the focused public-documentation plan that distinguishes the full shipped library, the activated adopter subset, and the unshipped starter proposal.
    - Confirm that one canonical section plus verbatim landing/dossier disclosure satisfies message-integrity requirements.
  must_read:
    - docs/plans/adoption-surface-clarity.md
    - docs/reviews/agf-framework-assessment-codex-sol-high.md
    - docs/reviews/todo-and-sol-high-synthesis-prioritized-actions.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 1: make adoption-surface calibration explicit

The Sol High assessment's complexity finding was accepted but the existing
response is scattered: minimum commissioning, selective roles, fast-track, and
rule deletion are documented in separate places, while the reduced starter
product remains only a proposal. Independent reviewers can therefore
reasonably interpret the complete distribution as the mandatory first-adoption
surface.

The plan makes `should-you-adopt.md` the canonical source, quotes one short
disclosure verbatim on the public home, evaluation landing, and curated dossier,
and labels the reduced starter as planned rather than shipped. It does not
create or imply a new profile, archive, validator mode, CI path, or coordination
contract.

