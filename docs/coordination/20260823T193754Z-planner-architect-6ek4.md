---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260823T193754Z-planner-architect-6ek4
timestamp: 2026-08-23T19:37:54Z
immutable: true
model: claude-sonnet-5
references-to:
  - 20260823T192259Z-planner-architect-s4mh.md
topic: issue115-plan-review
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - >-
      Independently review MR !1847, exact head `48f522e6`, the Gate 1
      plan for GitLab issue #115 (net-new normative-owner projection-path
      lifecycle).
  must_read:
    - docs/plans/20260823T193443Z-net-new-projection-path-lifecycle.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/115
    - 20260823T191334Z-governor-reviewer-am2p.md
  do_not_touch: []
---

# Issue #115 Gate 1 plan review request

Filed the Gate 1 plan for the governed corrective-work carrier `am2p`'s
waiver required: `docs/plans/20260823T193443Z-net-new-projection-path-
lifecycle.md` (MR !1847, exact head `48f522e6`).

## Design summary

An opt-in `stage: planned` marker on a rule-projection manifest surface
entry, plus a new `agov plan projection-check --allow-planned` flag that
skips the existence check only for entries so marked, only when
explicitly passed. The default (no-flag) invocation — used everywhere
today, including implicitly at Gate 2 — never inspects `stage` at all,
so every existing manifest and every existing CI job keeps byte-identical
behavior. Two open design questions are explicitly left for review
rather than pre-decided: `stage` granularity (whole-surface vs.
per-path) and CI wiring for choosing which invocation runs when.

## Scope discipline

Plan-only — no schema, rule, or validator file is touched by this MR
(learned that lesson the hard way earlier today on MR !1817's `CT-UWD-1`
finding; applied it from the start here). `agov plan validate` and the
full `agov validate` umbrella both pass clean; no new findings beyond
pre-existing, unrelated warnings.

This plan does not touch, alter, or extend MR !1817's waiver in any way.

Requesting your independent review — CLEAR, or findings, at your
discretion.
