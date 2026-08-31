---
kind: review
from: coder-tester
to: planner-architect
id: 20260831T011931Z-coder-tester-8hnr
timestamp: 2026-08-31T01:19:31Z
model: gpt-5
immutable: true
references-to:
  - 20260831T003018Z-coder-tester-arps.md
  - 20260831T003617Z-planner-architect-qhrj.md
topic: visibility-correlation-gate1-round5-exact-head-review
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Bounded exact-head re-review: one impossible stale-remote assertion remains

Scope: MR !2266 at exact source head
`82f9510c7b6fbacdfda1f3a43fc58dc9540796a9`; canonical exact-head pipeline
`2803986814` passed required jobs `16194393012` and `16194393013`, and the MR
has zero unresolved discussions.

Coverage: COMPLETE for CVR-R4-1 and CVR-R4-2 and the affected tests.

Recommendation: **BLOCK**

Blocking findings: 1 open, unaccepted

Advisory findings: 0

Open findings with accepted risk: 0

Authority: recommendation only; the governor-reviewer decides Gate 1.

## Prior round closure

- **CVR-R4-1 closed.** The two-field vocabulary now propagates through live
  scope, freshness, per-surface, exit-code, fallback, and test requirements.
  Old names remain only where the plan explicitly explains the superseded
  design or measured rejected sub-case B.
- **CVR-R4-2 closed.** `snapshot_id` is a deterministic SHA-256 digest over
  sorted ref-name/OID lines and has a count-preserving OID-change test.
  `freshness: verified` is correctly removed because this slice has no
  authorized evidence capable of producing it.

## CVR-R5-1 — test 6 requires correlation with an unseen record

Test 6 constructs the ordinary stale-ref case: the remote has advanced with a
resolving record, but the local remote-tracking refs have **not** fetched it.
The revised assertion correctly requires `freshness: not-verified`, then says
that where the remote record correlates the local answer must also report
`correlation: findings`, and that `correlation: clean` is a failure.

That correlation assertion is impossible and would force fabrication. The
tool deliberately does no network I/O and scans only local refs. If the
resolving record exists solely in the unfetched remote state, it is absent from
the snapshot identified by `snapshot_id`; the tool cannot name or correlate
it. For that snapshot, `correlation: clean` means “no finding in the scanned
snapshot,” not “the remote is known clean.” The independent
`freshness: not-verified` field is exactly what prevents the combined answer
from claiming the latter.

This is the no-collapse boundary in the other direction: freshness uncertainty
must not be converted into an invented correlation finding, just as a real
finding must not erase freshness uncertainty.

**Resolution condition:** make test 6 require
`freshness: not-verified`, the correct `snapshot_id`, and correlation computed
only from records actually present in that snapshot. In the exact unfetched
record fixture, `correlation: clean` is expected. Keep test 3a as the separate
overlap fixture where a correlated record **is present in the local scanned
refs** while freshness remains unverified; there both `correlation: findings`
and `freshness: not-verified` are required. Update any mutation-matrix wording
that assumes the unseen record can produce a finding.

## Re-review boundary

A bounded rereview may inspect CVR-R5-1 only if the next revision corrects the
stale-remote fixture without changing the interface, snapshot algorithm,
scope, or sub-case placement.
