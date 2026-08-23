---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260823T191334Z-governor-reviewer-am2p
timestamp: 2026-08-23T19:13:34Z
model: human-governor
immutable: true
references-to:
  - 20260823T184103Z-planner-architect-trhq.md
  - 20260823T185658Z-contribution-auditor-34ma.md
topic: mr1817-net-new-projection-path-waiver
requires-ack: true
artifacts_owned: []
---

# Binding one-time projection-path waiver for MR !1817

The human governor approves a one-time, non-precedential waiver for MR !1817
from the projection validator's existing-path requirement solely for the
proposed net-new normative-owner file.

The plan must remain plan-only before Gate 1. Its projection dispositions must
still be complete, and the implementation must create the declared rule and
provide a fully resolving canonical projection manifest before Gate 2.

Planner-architect must separately initiate governed corrective work so future
net-new-rule plans can represent planned paths without requiring pre-Gate-1
implementation.

## Exact scope and unresolved prerequisites

This decision resolves only the projection conflict escalated in `trhq`. It
does not:

- resolve contribution-auditor findings R4-1 or R4-2 in `34ma`;
- supply or substitute for a fresh COMPLETE independent contribution audit;
- approve Gate 1;
- authorize pre-Gate-1 implementation, merge, release, tag, or deployment; or
- establish precedent for another plan, rule, validator, or planned path.

At decision time, MR !1817 remains open and plan-only at exact head
`1587a9a877307cd918011f4e27ad27170966e7a1`, with successful exact-head
pipeline `2783436182`. Planner-architect must correct R4-1/R4-2 on a validated
plan-only head, acknowledge this decision, and then request the fresh COMPLETE
audit required before any Gate 1 request.
