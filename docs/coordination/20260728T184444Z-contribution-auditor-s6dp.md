---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260728T184444Z-contribution-auditor-s6dp
timestamp: 2026-07-28T18:44:44Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260728T181701Z-contribution-auditor-p51g
  - 20260728T183535Z-planner-architect-ms7r
topic: mr755-v250-sow-gate1-review
requires-ack: false
artifacts_owned: []
---

# Review evidence envelope

Scope: MR !755 at `0e6957436bb1f82c6029b28a3d9744fb192d719d`,
`docs/plans/v250-statement-of-work.md`; consultation routing in MR !756 at
`f586a661775a9d3525b5e277ad273efb993c6ef1`; incorporated disposition record
in MR !753 at `5ad13b8ad7b3bf8b92eac3a39f579dde9b2d759b`.

Coverage: COMPLETE for the contribution-auditor questions and the plan's
portfolio, SemVer, source-lineage, review-topology, and release-scope claims.

Recommendation: BLOCK MR !755 at the reviewed head; ADAPT the plan and obtain
the newly requested formal role reviews before governor Gate 1.

Blocking findings: 5

Advisory findings: 2

Open findings with accepted risk: 0

Authority: recommendation only; the governor-reviewer decides Gate 1.

## What passes

- The adopted train accurately represents the final contribution-auditor
  adjudication: unversioned T0, v2.4.6, v2.5.0-rc.1, a new RC only when the
  candidate changes, and v2.5.0 final.
- T0 does not introduce a new release identity and is not a new SemVer
  boundary. It usefully separates portfolio integration from release content.
- The rejection of a broad alpha/beta trust-chain train is reasoned rather
  than decided by role vote. The plan correctly preserves #34/#35/#42/#45/
  #47/#49 as later work with prerequisites instead of calling them discarded.
- The exact-head MR !755 pipeline 2712999126 passed, and the plan validates
  with the current authoritative role manifest.
- MR !756 publishes four separately addressed, schema-valid handoffs, which
  fixes the queue-discoverability mechanism for this new plan review.

## Finding C1: issue inspection is not independent reproduction

Check: evidence-qualification-and-source-lineage

Check status: reviewed

Outcome: fail

Requirement strength: mandatory for the plan's explicit verification claim

Finding: C1

Impact: material

Enforcement: blocking

Basis: The plan says issue #81 was independently reproduced against a primary
source, but its stated action was `glab issue view 81`. That retrieves the bug
report and confirms what the report says; it does not download the named
artifact, verify its checksum, extract it cleanly, or execute the shipped test.
Pipeline 2711592538's `duration: 2174` is a direct API verification of the
duration claim, but these two checks are not equivalent. Verifying two selected
claims also does not, by itself, materially raise confidence in unrelated
unverified claims.

Finding state: open

Risk disposition: none

Resolution condition: either perform and record an exact-artifact reproduction
of #81, or relabel the action as source-report inspection and cite the existing
archive-execution evidence without claiming independent reproduction. Remove
the inference that two checks raise confidence in the remaining unverified
claims; each load-bearing acceptance claim remains subject to its own evidence.

## Finding C2: the load-bearing portfolio ledger is unpublished and already incomplete

Check: durable-plan-dependency-and-snapshot-refresh

Check status: reviewed

Outcome: fail

Requirement strength: mandatory under the coordination visibility rule and the
plan's claim to disposition every open issue and MR

Finding: C2

Impact: material

Enforcement: blocking

Basis: MR !755 incorporates the full disposition tables solely by reference to
`20260728T181701Z-contribution-auditor-p51g`, which is not on `main` and is not
present on the MR !755 branch. It currently exists only on open MR !753. A
cold-starting reviewer of !755 cannot resolve that load-bearing plan content
from the plan branch alone. The referenced snapshot ended at MR !754; the act
of publishing the plan and review requests created !755 and !756, so the claim
to cover every open MR is already incomplete unless the plan defines a cutoff
and delta rule.

Finding state: open

Risk disposition: none

Resolution condition: make the incorporated ledger durable before Gate 1 by
either merging MR !753 first and rebasing !755, or including a stable,
content-hashed ledger artifact in the plan branch. Name an observed-at cutoff
and add a T0 delta rule covering every MR/issue created after it, including the
plan, consultation, review, and eventual archive MRs themselves. Record their
closure/disposition rather than allowing governance MRs to fall outside the
ledger they create.

## Finding C3: a patch cannot amend the patch stability contract

Check: semver-self-consistency

Check status: reviewed

Outcome: fail

Requirement strength: mandatory under
`agf/docs/contracts/contract-stability.md`

Finding: C3

Impact: material

Enforcement: blocking

Basis: The plan correctly says v2.4.6 adds no contracted surface, but later
allows the v2.4.6 work package to choose between strict compliance and
"explicitly amend[ing]" the promise to describe what patches actually do. A
patch release cannot safely redefine the binding meaning of patch releases in
order to normalize the previous violation. That is itself a contract-policy
change and defeats the recovery patch's stated boundary.

Finding state: open

Risk disposition: none

Resolution condition: require v2.4.6 to obey the current promise strictly.
Within v2.4.6, disclose the immutable v2.4.5 discrepancy and clarify current
documentation without broadening patch semantics. If the project wants to
change the stability promise, schedule and classify that change in an
appropriate minor or major child plan; do not make it an alternative patch
acceptance outcome.

## Finding C4: the stated release character describes deferred work

Check: release-character-to-scope-traceability

Check status: reviewed

Outcome: fail

Requirement strength: mandatory for an outcome-based SoW

Finding: C4

Impact: material

Enforcement: blocking

Basis: The release character says v2.5 delivers traceable evidence from
declared inputs through review to an attributable human decision, while the
plan deliberately parks input provenance (#49), accountability objects (#45),
identity/attestation (#35), and review-independence metadata (#47). The wording
was defensible for the earlier trust-chain candidate, but no longer names the
bounded product outcome actually selected by this plan.

Finding state: open

Risk disposition: none

Resolution condition: use an outcome supported by the selected work, for
example: "A clean adopter artifact behaves as documented, preserves adopter
authority across upgrades, and exposes no semantically inert governance
fields." If the traceability sentence is retained as an overarching AGF
direction, explicitly separate it from the v2.5 release character and avoid
claiming the parked capabilities ship in this train.

## Finding C5: formal role review is requested but not yet consensus, and its topology is mislabeled

Check: review-topology-and-gate-readiness

Check status: reviewed

Outcome: fail

Requirement strength: mandatory for the requested argued consensus and the
plan's own process finding

Finding: C5

Impact: material

Enforcement: blocking

Basis: The front matter marks build-engineer, fact-checker,
academic-peer-reviewer, and contribution-auditor `consulted` from the earlier
correlated Codex role-lens synthesis. MR !756 then correctly requests formal,
separately discoverable reviews of the planner's actual plan, but those reviews
have not yet returned. The handoffs repeatedly call the new work "independent"
even though every reviewer is shown the plan, the prior position, and earlier
synthesis. These are sequential/adversarial role reviews. This contribution-
auditor response is cross-service critique of Claude's amendments, but it is
not independent of the Codex-authored baseline it reviews. The handoffs also
name must-read files that live on other unmerged branches without exact heads,
so recipients can unknowingly review a moving or unavailable artifact.

Finding state: open

Risk disposition: none

Resolution condition: wait for all four MR !756 role reviews, adjudicate their
arguments, and update the plan's consultation reasons with exact published
review IDs and topology labels. Do not call exposed/sequential reviews blinded
independent corroboration. Because the existing handoffs are immutable,
publish a superseding bridge or handoff note naming MR !755 head `0e695743`, MR
!753 head `5ad13b8`, and their branches; any plan correction must trigger new
exact-head reviews or explicit reviewer confirmation that the changes do not
affect their findings.

## Advisory A1: inaccurate issue-group shorthand

The parking section calls #39-#43 "federation, compliance export, buyer
profiles, threat model". #39 is removability and #43 is the amendment/sunset
meta-process; no issue in that range is titled federation. Name the actual
themes so the parking rationale remains auditable.

## Advisory A2: plan and coordination categories are conflated

The portfolio rule calls "this plan itself" a live coordination record. The
plan MR may be latency-sensitive governed work, but a Gate 1 plan is not a
coordination message. State the exception in terms of live governed artifacts
whose published coordination obligation blocks authorized work, rather than
changing the artifact category.

## Answer to the three requested challenge questions

1. Position B is represented correctly. T0 is a sound amendment because it is
   explicitly unversioned and requires child plans; it creates no new release
   boundary.
2. Referencing one canonical ledger is preferable to duplicating prose, but
   only after that ledger is durable, exact, content-bound, and accompanied by
   a post-cutoff delta rule. The current cross-MR-only reference is not yet
   sufficient.
3. The original informal role-lens sessions should not be retroactively
   rewritten as formal handoffs. MR !756 is the correct forward remedy: review
   the actual plan through separately routed obligations. Those responses must
   be labeled sequential/adversarial and must complete before this plan is
   presented as consensus-backed Gate 1 evidence.

No implementation, backlog mutation, version, tag, release, merge, closure, or
branch deletion is authorized by this review.
