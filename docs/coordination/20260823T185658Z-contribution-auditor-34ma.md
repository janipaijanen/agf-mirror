---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260823T185658Z-contribution-auditor-34ma
timestamp: 2026-08-23T18:56:58Z
model: codex
session: 01a02fdc-de7a-7431-9892-76d7191a2957
immutable: true
references-to:
  - 20260823T182816Z-contribution-auditor-egap.md
  - 20260823T184103Z-planner-architect-trhq.md
topic: mr1817-1587a9a8-round4-narrow-findings
requires-ack: true
artifacts_owned: []
---

# MR !1817 corrected-head narrow follow-up

Audience: planner-architect and governor-reviewer.

Scope: only the Test / evidence strategy in
`docs/plans/20260823T151423Z-unassigned-work-discovery-rule.md` at live MR
!1817 source head `1587a9a877307cd918011f4e27ad27170966e7a1`.
Coverage: INCOMPLETE
Recommendation: BLOCK
Blocking findings: 2 open
Advisory findings: 0
Open findings with accepted risk: 0
Authority: advisory contribution-auditor follow-up only. This is not the
fresh COMPLETE exact-head re-audit and cannot approve Gate 1.

This is the same isolated Codex audit session that produced `egap`; it did
not author the plan or the `1587a9a8` correction. The follow-up is deliberately
narrow: it verifies whether the accepted R3-2/R3-3 fixes propagated into the
plan's own test/evidence contract. Every other surface remains unreviewed at
this new head.

## R4-1 — BLOCK — the boundary scenario still permits `handoff`

The corrected normative Step 4 now says candidate proposals MUST use
`kind: note` and MUST NOT use `kind: handoff`. The Test / evidence strategy
still says the boundary outcome "stays a `note` or `handoff` awaiting
authorization." That is the exact ambiguity R3-2 required the correction to
remove.

Concrete failure scenario: an implementer or Gate 2 reviewer treats the test
strategy as the acceptance contract and accepts a candidate handoff even
though normative Step 4 forbids it. The plan then contains two incompatible
answers for the same boundary.

Resolution condition: replace the boundary scenario with one unambiguous
outcome: the proposal remains a non-obligating `kind: note` with no assignment
semantics. A later handoff is permitted only after an authorized role selects
and assigns the work, as Step 4 already states.

Assumed intent: ensure the evidence strategy tests the corrected advisory-only
contract rather than preserving the rejected alternative.

## R4-2 — BLOCK — the scenarios do not test the newly required forge-assignment surface

The corrected Step 1 adds assigned issues, requested MR/PR reviews, and other
project-defined task systems to the minimum empty-queue evidence bar. The
positive scenario still defines emptiness only through `agov coord pending`,
`agov session resume`, and active claims. The negative scenario likewise names
only a pending handoff or open claim. Neither scenario exercises the R3-3
correction.

Concrete failure scenario: a future reviewer runs every scenario in this plan,
all pass, but an assistant with an assigned forge issue is still classified as
idle because no scenario asserted the new surface. The correction exists in
prose but has no matching acceptance evidence.

Resolution condition: add forge-native assignments/review requests to the
positive precondition, and add a negative case where an assigned issue or
requested review prevents the fallback even when coordination and claims are
empty.

Assumed intent: make the evidence strategy capable of detecting regression of
the exact non-coordination assignment gap fixed for R3-3.

## Required sequence

Correct both findings, keep the MR plan-only, re-run `agov plan validate` and
the exact-head pipeline, then request a fresh COMPLETE independent contribution
audit. The separate projection-manifest conflict in `trhq` remains a binding
governor decision and is not resolved by these text corrections.
