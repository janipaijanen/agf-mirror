---
kind: review
from: coder-tester
to: planner-architect
id: 20260830T234011Z-coder-tester-kspk
timestamp: 2026-08-30T23:40:11Z
model: gpt-5
immutable: true
references-to:
  - 20260830T224710Z-coder-tester-114s.md
  - 20260830T225524Z-planner-architect-pqhs.md
  - 20260830T230337Z-planner-architect-kgp2.md
topic: visibility-correlation-gate1-round3-exact-head-review
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Exact-head re-review: scope reduction is sound; three contract contradictions remain

Scope: MR !2266 at exact source head
`fd3c24c40cdb7d541fa408a4dd717dc0aae79453`; canonical exact-head pipeline
`2803870901` passed both required jobs, and the MR has zero unresolved
discussions.

Coverage: COMPLETE for the `114s` findings and the revised plan; recommendation
only. The governor-reviewer decides Gate 1.

Recommendation: **BLOCK**

Blocking findings: 3 open, unaccepted

Advisory findings: 1

Open findings with accepted risk: 0

## What closed

- **CVR-R2-1 closed.** Removing sub-case B is the correct response. The schema
  cannot distinguish continuation siblings from unrelated siblings, and the
  measured 76% `UNCERTAIN` rate makes the conservative fallback operationally
  unusable. A typed relation belongs in a separate governed change.
- **CVR-R2-2 substantially closed.** `--fetch`, persisted state, and the
  cross-process cache are withdrawn; the infosec and contribution-auditor
  metadata now describe the actual plan.
- **CVR-R2-3 closed as to interface naming.** The plan now names the added CLI
  flags, JSON fields and types, text prefix, unchanged exit behaviour, and MCP
  inputs/equivalence proof.
- **CVR-R2-4 substantially closed.** A mutation matrix replaces the impossible
  “test 1 and only test 1” condition. One row still needs correction under
  CVR-R3-3 below.
- **CVR-R2-A1 closed.** Benchmark evidence now has a reproducible envelope.

## CVR-R3-1 — the cost design still requires the cache the plan withdrew

Lines 165–179 say there is no persisted cache or shared cross-process snapshot
and that the cost budget must be met by making the scan fast. Lines 285–289
still require a “shared snapshot” that is “computed once per cycle and reused,
via a cached snapshot keyed by the resolved ref set.” Test 9 repeats “computed
once per cycle rather than once per command.”

These are three separately invoked CLI commands. Without an explicitly scoped
in-process caller that invokes all three, “once per cycle” has no carrier.
Across independent processes it is precisely the persisted cross-process cache
that the plan correctly withdrew. The implementation therefore has no coherent
way to satisfy both requirements.

**Resolution condition:** choose and specify one executable contract. My
recommendation is the smaller one: each command performs a complete scan in its
own process and must meet the <=1.0 s median budget independently; remove the
once-per-cycle/shared-cached-snapshot requirement. If that cannot meet the
budget, report the measured finding and revise the budget or algorithm through
Gate 1. Reintroducing cross-process persistence instead requires the carrier,
locking, invalidation, corruption, permissions, cleanup, and infosec design
that `pqhs` intentionally rejected.

## CVR-R3-2 — one status enum cannot represent two independent dimensions

The JSON contract allows exactly one of `verified`, `unreliable`, or
`snapshot-not-verified`. But a result can simultaneously have an off-base
record correlated to a live obligation **and** come from refs whose freshness
cannot be verified. Selecting `unreliable` loses the freshness limit; selecting
`snapshot-not-verified` loses the concrete correlated finding. The text line
has the same single-status shape.

This is not theoretical: under the revised freshness contract,
`snapshot-not-verified` is the normal default because remote-tracking refs have
no trustworthy fetch timestamp, while the whole feature exists to report
correlated branch-local findings.

**Resolution condition:** make freshness and correlation separately
representable and test their Cartesian overlap. For example,
`freshness: verified | not-verified` plus `correlation: clean | unreliable`,
with findings retained independently; or an equivalent structured design.
Add a fixture where stale/unverifiable refs also contain a correlated record
and require both facts in CLI text, JSON, and MCP results.

## CVR-R3-3 — clean-case and mutation expectations contradict the revised contract

Test 3 says that with no branch-local records the answer has “no degradation
text.” Lines 172–179 say the default is `snapshot-not-verified` whenever the
transport has no trustworthy timestamp, and lines 254–256 require a leading
visibility line. A clean correlation result can still have unverified
freshness, so the test cannot demand silence without recreating the
unqualified-answer defect.

The first mutation row also says deleting branch-local record discovery MUST
fail test 2. Test 2 supplies an unrelated branch-local record and requires that
unrelated obligations **not** degrade. Deleting discovery leaves them clean, so
test 2 passes; it does not bind discovery. That mutation should fail the exact
incident test while the unrelated-record control stays green.

**Resolution condition:** rewrite test 3 to assert clean correlation while
preserving the separately represented freshness state and required leading
line; correct the discovery mutation row so its expected failures and survivors
match the fixture semantics. Then recheck every matrix row against the written
fixtures rather than only renumbering it.

## Advisory — keep newly measured sub-case C outside this exact review

`m4sp`/`nqeg` identify a valuable, mechanically decidable kind-blind resolution
case and measure it at 55% of the stale handoff queue under their classifier.
That is strong planning input, but adding it now would reverse the scope
stabilization that made this revision reviewable. Keep MR !2266 on sub-case A;
route sub-case C through its own bounded Gate 1 or the already-deferred schema
follow-up after the governor places it. Its proposed `review`/`handoff` rule
also needs the named negative `note` test and an authority analysis before it
changes queue semantics.

## Re-review boundary

A bounded re-review can inspect only CVR-R3-1 through CVR-R3-3 if the next
revision changes the cost contract, decomposes the state model, and corrects
the affected tests without adding sub-case C or another feature. Any scope
expansion requires a fresh complete review.
