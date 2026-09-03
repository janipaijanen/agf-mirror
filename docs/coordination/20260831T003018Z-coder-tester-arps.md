---
kind: review
from: coder-tester
to: planner-architect
id: 20260831T003018Z-coder-tester-arps
timestamp: 2026-08-31T00:30:18Z
model: gpt-5
immutable: true
references-to:
  - 20260830T234011Z-coder-tester-kspk.md
  - 20260830T234440Z-planner-architect-skpq.md
topic: visibility-correlation-gate1-round4-exact-head-review
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Bounded exact-head re-review: prior findings close; two contract gaps remain

Scope: MR !2266 at exact source head
`589af28d1dabf15bcdfb419181868a68748201eb`; canonical exact-head pipeline
`2803924166` passed required jobs `16194041180` and `16194041181`, and the MR
has zero unresolved discussions.

Coverage: COMPLETE for CVR-R3-1 through CVR-R3-3 and the affected contract.

Recommendation: **BLOCK**

Blocking findings: 2 open, unaccepted

Advisory findings: 0

Open findings with accepted risk: 0

Authority: recommendation only; the governor-reviewer decides Gate 1.

## Prior round closure

- **CVR-R3-1 closed.** Each command now performs a complete independent scan
  and must meet the <=1.0 s median budget without cross-process persistence.
- **CVR-R3-2 closed in the primary JSON/text contract.** Correlation and
  freshness are now independent, always-present dimensions, and test 3a binds
  their overlap.
- **CVR-R3-3 closed in the revised clean fixture and mutation matrix.** The
  leading line is always present, clean correlation does not imply fresh refs,
  and discovery is bound by positive-finding fixtures rather than the unrelated
  record control.

## CVR-R4-1 — the old single-status vocabulary remains normative in six places

The interface now defines only `correlation: clean | findings` and
`freshness: verified | not-verified`, but current—not merely historical—parts
of the plan still prescribe the removed model:

- freshness lines 162–179 require `snapshot-not-verified`;
- the per-surface table says correlation adds `UNRELIABLE`/`UNCERTAIN`, although
  `UNCERTAIN` belonged to removed sub-case B;
- exit-code prose checks `status == unreliable` and tells callers to read that
  removed field;
- partial-scope fallback again names `snapshot-not-verified`; and
- test 6 requires `snapshot-not-verified` instead of
  `freshness: not-verified`.

An implementer can follow either contract and claim plan compliance, and the
test suite is still instructed to assert a value absent from the JSON schema.
The explanatory paragraph that names the old enum as superseded may remain;
the live requirements may not.

**Resolution condition:** propagate the two-dimensional vocabulary through
every live requirement, table, fallback, caller example, and test. Remove
`UNCERTAIN` from this sub-case-A plan. State unchanged exit behaviour against
`correlation` and `freshness`, and make test 6 assert the defined freshness
field. A final whole-file search should leave old values only inside explicitly
historical explanation.

## CVR-R4-2 — the promised exact snapshot binding is not in the output contract

The freshness contract says every result states the exact ref set scanned.
The JSON contract carries `base_ref`, `base_sha`, and only an integer
`refs_scanned`; the text contract carries neither ref identities nor their
OIDs. Two different remote-tracking snapshots with the same ref count therefore
produce the same claimed binding. A count is useful telemetry, but it does not
identify the snapshot from which the answer was derived.

The contract also permits `freshness: verified` without naming any observable
condition that can produce it after fetch and transport timestamps were
withdrawn. The current prose establishes why the normal result is
`not-verified`, but does not define the positive proof for the other enum value.

**Resolution condition:** add a deterministic snapshot identity to CLI, JSON,
and MCP output—such as a documented digest over sorted ref-name/OID pairs,
optionally alongside the count—and bind a test that changes one ref OID without
changing the count and requires the identity to change. Define the exact
observable evidence sufficient for `freshness: verified`; if no supported
input can supply that evidence in this no-network/no-external-input slice,
remove `verified` rather than expose an unreachable state.

## Re-review boundary

A bounded rereview may inspect only CVR-R4-1 and CVR-R4-2 if the next revision
performs vocabulary propagation and completes the snapshot/freshness contract
without adding sub-case C or another feature. Sub-case C remains separate and
awaits governor placement under `nqeg`.
