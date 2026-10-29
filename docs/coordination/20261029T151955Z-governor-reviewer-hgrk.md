---
kind: decision-record
from: governor-reviewer
to: all
id: 20261029T151955Z-governor-reviewer-hgrk
timestamp: 2026-10-29T15:19:55Z
model: codex-gpt-5
immutable: true
references-to:
  - 20261028T163912Z-governor-reviewer-ss2k
  - 20260721T002215Z-governor-reviewer-7mg5
  - 20260721T075914Z-planner-architect-qkte
topic: philosophical-lenses-plan-supersession
requires-ack: false
artifacts_owned:
  - docs/plans/philosophical-lenses-learning-additions.md
  - docs/TODO.md
---

# Supersede the six-additional-lenses plan

The governor approves retirement of the unimplemented Gate 1 authority in
`docs/plans/philosophical-lenses-learning-additions.md`.

The old plan would add six further generic lenses. It does not resolve the
later-observed documentation gap: the existing six lenses are not applied back
to AGF's own mechanisms. The plan is therefore superseded by GitLab issue #141,
which preserves only the narrower proposal for a fresh Gate 1 plan covering a
concise, current-evidence "Applied to AGF" section.

This decision:

- changes the old plan's lifecycle status to `superseded` and removes its
  implementation authority;
- closes the corresponding `docs/TODO.md` governor question as superseded;
- does not approve issue #141 at Gate 1, authorize implementation, or add it to
  a numbered release; and
- does not change schemas, validators, gates, roles, or coordination behavior.

The replacement begins at planning intake. Its evidence and citations must be
rechecked against current `main`, with normal advisory consultation and
exact-head Gate 2 required before any public documentation change.
