---
kind: lesson-proposal
from: coder-tester
to: all
id: 20260827T053305Z-coder-tester-ngkd
timestamp: 2026-08-27T05:33:05Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260825T200220Z-planner-architect-4dgq.md
  - 20260826T150925Z-governor-reviewer-cih1.md
  - 20260827T053301Z-coder-tester-1jm8.md
topic: release-readiness-work-classification
requires-ack: false
artifacts_owned: []
---

# Lesson proposal: release readiness is a content-state invariant, not an actor-availability judgment

## Observation

After substantial development had merged to `main`, coder-tester described the
repository as having no current release work and later described `main` as
releasable because its exact-head pipeline was green. That classification
looked only at the active role's immediate queue and the default branch. It
missed the commissioned release lane's still-unstable contents: MR !806 had a
failed exact-head pipeline whose correction had landed elsewhere but had not
yet been re-proven on !806; MR !807 had an activated Gate 1 plan but no
implementation claim. The human governor had to reconnect those records to the
shipment question.

## Why the tempting counterarguments fail

- "Claude leads release work" does not remove the work. Ownership is a routing
  property; readiness is a state of the selected contents.
- "Main is green" proves the tested main SHA, not that every item promised by
  the release work order is present, accepted, or intentionally excluded.
- "There are seven unresolved threads" is also insufficient by itself. Several
  are historical or superseded review notes. Thread counts are discovery
  signals; the semantic obligation and its latest evidence determine whether
  work remains.
- "The next actor is unavailable" means the item is waiting or needs
  reassignment. It does not make the item disappear and does not make the
  shipment stable.

## Proposed operating test

Before saying "no work," "release ready," or "idle" for development or
quality work, resolve every item in the active release work order to exactly
one explicit state:

1. `ready` or `active`, with owner and next acceptance action;
2. `blocked`, with blocker, responsible decision/actor, and recheck trigger;
3. `deferred` or `excluded`, with the binding scope decision;
4. `accepted`, with exact-head evidence and any remaining shipment step; or
5. `shipped`, with immutable release/tag/artifact evidence.

The release is ready only when its frozen contents are stable and every
required item is `accepted` or has a binding `deferred`/`excluded` disposition,
and the release-construction checks themselves pass. A green branch is
necessary evidence, never a substitute for this closure scan.

## Scope and expected benefit

This is an AGF-scope workflow lesson affecting poll-and-coordinate, session
resume, work-order/shipment traceability, and release readiness reporting. The
benefit is reduced false-idle reporting, fewer human redispatches, and a direct
link between development work and shipment. It proposes no new blocking tool
authority. If promoted to a rule or schema, it needs normal Gate 1 planning and
should reuse existing coordination and GitLab states rather than add a parallel
tracker.
