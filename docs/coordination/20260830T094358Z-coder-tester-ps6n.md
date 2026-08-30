---
kind: review
from: coder-tester
to: governor-reviewer
id: 20260830T094358Z-coder-tester-ps6n
timestamp: 2026-08-30T09:43:58Z
model: gpt-5.6-sol
immutable: true
references-to:
  - 20260830T090033Z-coder-tester-2gqk.md
  - 20260830T084415Z-coder-tester-br41.md
topic: slice-c-third-bounded-exact-head-rereview
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Slice C: third bounded exact-head re-review

Scope: only GME-R1 and GME-R2 closure on MR !2222 at exact source head
`f0eb048287e857ac5cb18357a4646938019a1cb1`, against target `main`
`2c35ef4dddc58dac070988173a3a586495f61cf5`.

Coverage: COMPLETE for the bounded request. Recommendation: **BLOCK Gate 1**.
Blocking findings: 1. Advisory findings: 0. Open findings with accepted risk:
0. Authority: recommendation only; the governor-reviewer decides Gate 1.

MR !2222 has zero unresolved discussions. Canonical MR pipeline `2803107084`
passed at exact head `f0eb0482`; governance job `16189649556` and
commercial-corpus job `16189649557` succeeded. In an isolated exact-head
worktree, `agov plan validate`, `agov plan projection-check --allow-planned
--base-ref origin/main`, and `git diff --check
343e19ba708d92b0d73c436fd4c9b3bd26f19bcf..f0eb048287e857ac5cb18357a4646938019a1cb1`
passed. These checks establish artifact shape but do not detect the remaining
semantic contradiction.

## GME-R1: CLOSED

The projection's prospective adopter instruction now describes only the two
local assertions. It no longer directs implementation to document a GitLab
API dependency or a cross-forge gap. Removed assertion-3 language is confined
to explicitly labelled superseded review history. This satisfies the `br41`
resolution condition.

## GME-R2: PARTLY CLOSED; one current disposition still contradicts 2a/2b

Check status: reviewed. Outcome: fail. Requirement strength: mandatory.
Impact: material. Enforcement: blocking. Finding: GME-R2-PROPAGATION.
Finding state: open. Risk disposition: none.

The plan's assertion definitions, lifecycle table, acceptance criteria 4 and
6, and adopter projection now consistently split the contract:

- assertion 2a requires active-entry paths to resolve and is waived after
  retirement; and
- assertion 2b makes registry-entry identity permanent and is never waived.

One current review-disposition row did not receive that split. In **Review
disposition — `8mar`**, the GME-A1 row still says:

> Retirement now waives assertions 1 and 2 for the retired entry, so obsolete
> files can be deleted. [...] the entry may never be deleted

Unlike the nearby discussion that explicitly describes an earlier
contradiction, this row is the plan's current disposition. Read literally, it
still waives assertion 2 wholesale while relying on part of assertion 2 to
prevent deletion. An implementer using that row alone would receive the old,
contradictory contract.

The current lifecycle prose also says entry deletion "fails assertion 2".
That is ambiguous after assertion 2 became an umbrella for 2a and 2b; deletion
must be attributed specifically to assertion 2b.

Resolution condition: update the current GME-A1 disposition to state that
retirement waives assertions 1 and 2a while 2b remains enforced, and change
the current deletion sentence to say deletion fails assertion 2b. Repeat the
exact-string and semantic-class scan across the whole plan and projection so
every current instruction uses the same split. Clearly labelled superseded
history may retain the old wording.

This finding does not reopen GME-R1 or the two-assertion design. It identifies
the incomplete propagation that `2gqk` explicitly asked this review to treat
as the most likely failure mode.
