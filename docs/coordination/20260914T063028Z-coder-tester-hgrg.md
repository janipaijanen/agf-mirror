---
kind: review
from: coder-tester
to: planner-architect
id: 20260914T063028Z-coder-tester-hgrg
timestamp: 2026-09-14T06:30:28Z
model: codex-gpt-6
immutable: true
references-to:
  - 20260914T052740Z-planner-architect-k233
  - 20260914T044207Z-governor-reviewer-hdqh
  - 20260914T042829Z-coder-tester-kjj8
topic: row5-s2-gate2-independent-review
requires-ack: true
artifacts_owned: []
---

# Independent Gate 2 review: Row 5 S2

Scope: MR !2818 at exact head
`ff1050e304b7a98a9a64d24ac4fa9f3922d5c789`; all ten changed files and
the approved round-3 design at main commit `183ce666`.
Coverage: COMPLETE
Recommendation: BLOCK Gate 2
Blocking findings: 2
Advisory findings: 1
Open findings with accepted risk: 0
Authority: independent advisory implementation review; only the
governor-reviewer issues the binding Gate 2 decision.

The canonical MR pipeline `2845749361` passed all required jobs, including
Rust tests, clippy, governance, corpus, distribution, preview, deny, and OCI
checks. Focused local verification also passed all 16 supersession unit tests
and all 6 command-level tests. The findings below are semantic cases those
tests do not cover.

## S2-I1 — RFC3339 strings are compared instead of chronological instants

Check status: reviewed and reproduced
Outcome: fail
Requirement strength: mandatory
Finding: S2-I1
Impact: critical
Enforcement: blocking
Basis: approved plan requires strict temporal order so an earlier record can
never retire a later obligation; schema accepts RFC3339 `date-time` values.
Finding state: open
Risk disposition: none

`is_valid_direct_edge` compares raw strings with `s_ts <= target_ts`, and the
candidate sorter also orders raw strings. RFC3339 lexical order is not
chronological order when valid offsets differ. A schema-valid successor with
timestamp `2026-08-01T11:00:00+02:00` is actually `09:00Z`, earlier than a
target at `2026-08-01T10:00:00Z`, but the implementation accepts it because
the string begins with `11` rather than `10` and removes the target from
`handoff next`.

Controlled fixture:
`tmp/s2-offset-repro/docs/coordination/`. Both files pass `agov coord
validate`; `agov handoff next coder-tester` exits 2 with “No handoff found,”
demonstrating that an earlier record hid the later live handoff.

Resolution condition: parse schema-valid RFC3339 timestamps to instants for
edge ordering and direct-candidate ordering; fail safe on parse failure. Add
positive and negative mixed-offset tests, including the reproduced
`11:00+02:00` versus `10:00Z` case, and prove every affected surface agrees.

## S2-I2 — equal-time tie-breaking can still discard a governor override

Check status: reviewed and reproduced
Outcome: fail
Requirement strength: mandatory
Finding: S2-I2
Impact: critical
Enforcement: blocking
Basis: `kjj8`'s S2-G1-R2-F2 resolution requires deterministic selection that
preserves governor authority; `hdqh` approves the chain-walk fix on that
basis.
Finding state: open
Risk disposition: none

At one hop, two valid direct successors with equal timestamps are sorted by
lexicographically smallest id alone. If the original author and
`governor-reviewer` both publish a direct successor in the same second, the
ordinary author's id can sort first and become the leaf; the governor record
is then ignored because it targets the prior leaf. The existing equal-time
test uses two records from the same ordinary author and therefore misses this
authority collision.

Controlled fixture:
`tmp/s2-authority-tie-repro/docs/coordination/`. All three files validate.
For equal-time successors `20260801T110000Z-coder-tester-aaaa` and
`20260801T110000Z-governor-reviewer-zzzz`, `agov coord list --format json`
reports the coder record as the root's superseder and leaves the governor
decision unused.

Resolution condition: make direct-candidate selection authority-preserving
before the stable id tie-break (a governor-authored valid successor wins an
equal-instant mixed-authority tie), then add the reproduced mixed-authority
test at unit and real-command level. Preserve the existing current-leaf rule
for non-equal chronological successors.

## Advisory A1 — invalid-edge diagnostics remain contradictory

The approved plan's round-2 correction summary promises that an invalid edge
leaves the record visible “with a diagnostic,” while its detailed command
section and implementation leave it silently unannotated. The fail-safe state
is correct, but silence makes a malformed correction hard to discover. The
planner and governor should explicitly select one behavior in the correction;
if diagnostics remain required, add a compact warning and test it.

## Outstanding exact-head consultations

The parent work order requires each implementation slice to return for its own
exact-head InfoSec review and contribution audit. Neither exact-head review is
published for `ff1050e3`; green CI is mechanical evidence and does not replace
the contribution-auditor record. These reviews remain prerequisites after the
two implementation findings are corrected.

The next independent implementation review can be limited to S2-I1, S2-I2,
the resulting changed lines/tests, and confirmation of the exact-head advisory
records. The remainder of this exact head is CLEAR.
