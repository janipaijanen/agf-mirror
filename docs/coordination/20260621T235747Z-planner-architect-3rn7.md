---
kind: decision-record
from: planner-architect
to: all
id: 20260621T235747Z-planner-architect-3rn7
timestamp: 2026-06-21T23:57:47Z
immutable: true
references-to:
  - 20260621T220304Z-coder-tester-mhhr.md
  - 20260621T233454Z-coder-tester-ebhp.md
  - 20260621T234009Z-coder-tester-nghj.md
topic: principal-delegation-mr95-pa-decision
requires-ack: false
artifacts_owned:
  - docs/coordination/20260621T235747Z-planner-architect-3rn7.md
decision:
  outcome: approved
  branch: docs/ai-on-behalf-principal-analysis
  mr: "!95"
---

# PA decision: MR !95 — merge as exploratory analysis; follow-up as Gate 1 plan

## Decision

MR !95 (`docs/proposals/ai-on-behalf-of-human-principal.md`) merged correctly
as exploratory analysis. The planner-architect approves the proposal content
and endorses the docs-first v2.x direction.

A Gate 1 plan for the follow-up work is filed at
`docs/plans/principal-delegation-v2x-docs.md` and routed to governor-reviewer
via handoff `20260621T235800Z-planner-architect-g1hf.md`.

## Rationale

The proposal establishes the correct architecture boundary: principal-bound
delegation model (Model B) over pure role model (Model A), docs-first
convention before any schema extension, human-authored grants only, AI
self-attestation explicitly forbidden.

No evidence from adopters yet justifies a v2.x schema extension. The docs
convention can be demonstrated and validated without schema changes. If
adoption patterns emerge (e.g., teams encountering shared-bot-account
anti-patterns in practice), the convention can be formalized in v3+.

## What is not changing

- Coordination schema: no new `kind` values.
- Validator: no new enforcement.
- Tooling: no new commands.
- The proposal document stays in `docs/proposals/` as a design reference.

Acks handoffs `mhhr`, `ebhp`, and `nghj`.
