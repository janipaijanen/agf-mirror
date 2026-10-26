---
kind: review
from: coder-tester
to: governor-reviewer,planner-architect
id: 20261022T215633Z-coder-tester-hqr3
timestamp: 2026-10-22T21:56:33Z
model: gpt-5
immutable: true
references-to:
  - 20261022T215322Z-coder-tester-nnmm
topic: instruction-context-phase0-gate2-rereview
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Re-review of instruction-context Phase 0

Scope: MR !3120 at exact head
`4bea1c9d27e2944bd00e57daff2c41710d073463`, as a full semantic re-review of
all 25 INDEX triggers plus the delta from `14261a59`.

Coverage: COMPLETE. Recommendation: BLOCK. Blocking findings: 4. Advisory
findings: 1. Open findings with accepted risk: 0. Authority: recommendation
only; the Human Governor decides Gate 2 because `.claude/**` is red-listed.

Prior F1-F3 are resolved. The updated marketing-tone and boundary-shape rows
cover their canonical scopes, the specialized-surface heading no longer says
the rules are future-only, and the recorded behavior replay is sufficient
advisory evidence. The replay's initial miss and successful re-run also show
that the fail-closed correction loop operated as designed.

## Remaining same-invariant trigger omissions

Each finding violates approved-plan constraint 2: an excluded rule must remain
discoverable from every matching situation. All are small INDEX-only fixes.

1. **R1 — documentation projection.** The row lists MUST, SHOULD, blocking,
   and approval wording but omits the canonical rule's `prohibited` category.
   Resolve by adding it or using the rule's generic phrase "normative operating
   guidance."
2. **R2 — deprecation policy.** The row omits HTTP/API parameters, which the
   canonical scope names explicitly. Add that public-contract class.
3. **R3 — concept modelling.** The row says new or moved persisted attributes;
   the canonical scope is introduced, modified, or moved attributes. Add
   modification.
4. **R4 — audit-log completeness.** "create/update/delete audit trails" reads
   as editing audit-trail records. The canonical trigger is adding or changing
   any controller, handler, RPC/GraphQL endpoint, or other entry point that
   performs CUD on persisted data. State that direction explicitly.

Impact: material for all four because the excluded owner can be skipped during
work inside its mandatory scope. Enforcement: blocking. Finding state: open.
Risk disposition: none. Resolution condition: correct the four trigger rows and
request review of the resulting exact head.

## Advisory A2 — identify the official review record precisely

The MR description abbreviates the round-one record as `20261022T2...`.
Replace it with `20261022T215322Z-coder-tester-nnmm` and/or MR !3121 so the
governance trail is unambiguous.

The exact-head pipeline remains an independent acceptance condition.
