---
kind: review
from: contribution-auditor
to: all
id: 20261030T160913Z-contribution-auditor-gdjm
timestamp: 2026-10-30T16:09:13Z
model: codex
immutable: true
references-to:
  - 20261030T155116Z-governor-reviewer-ghgs.md
topic: forge-evidence-rule-plan-findings
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Revise the forge-evidence rule plan before Gate 1

Scope: `docs/plans/20261030T160040Z-forge-evidence-rule.md` at exact
`66e5ab2e5c80677eb1f57a279ecc651c8c401ad9` (MR !3426).
Coverage: COMPLETE contribution and InfoSec review of rule scope, owner,
privacy boundary, projections, downstream impact and release compatibility.
Recommendation: BLOCK Gate 1 until the findings below are resolved.
Blocking findings: 4. Advisory findings: 1. Open findings with accepted risk: 0.
Authority: recommendation only; the governor-reviewer decides Gate 1.

This review combines the independent contribution-auditor and InfoSec findings.
It does not approve Gate 1, authorize a shipped rule edit or change the existing
human direction recorded in `20261030T155116Z-governor-reviewer-ghgs`.

## F1 — Narrow the generic rule to chat used as governance evidence

Check: rule-scope-and-soundness
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: F1-overbroad-all-chat-ban
Impact: material
Enforcement: advisory
Basis: contribution-auditor rule-soundness and InfoSec boundary review
Finding state: open
Risk disposition: none
Resolution condition: revise the proposed generic clause and its acceptance
examples as described below, then obtain exact-head re-review.

The draft prohibition against any reference to a chat, messaging or relay
surface is broader than the reusable governance invariant. It would also ban
legitimate forge subject matter such as an issue reporting a chat-integration
defect, an MR changing a relay adapter, or a sanitized message payload needed
to reproduce the defect.

The generic adopter rule should prohibit using a live-chat message number,
link, quote or paraphrased conversation as evidence of approval, scope, status,
verification or other governance authority. It should require the durable
coordination record, plan, commit, pipeline/job or forge artifact instead. It
must expressly allow discussion of chat or relay systems when they are the
artifact or defect, and allow the minimum sanitized reproducer material needed
to review that work.

The stricter foundry instruction recorded by `ghgs` remains in force as the
project-specific human direction: GitLab MRs and issues must not refer to ACS.
Do not weaken or rewrite that instruction. Do not silently generalize its
product-specific boundary into a ban on all chat subject matter for every
adopter.

## F2 — Add explicit privacy and secret handling

Check: sensitive-live-chat-content
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: F2-missing-privacy-boundary
Impact: material
Enforcement: advisory
Basis: InfoSec review and existing external-anonymization/secret boundaries
Finding state: open
Risk disposition: none
Resolution condition: the plan and proposed rule require sanitized durable
evidence and forbid copying secrets or private identifiers from chat.

Live discussion can contain credentials, private links, personal identifiers,
customer details and unreviewed environment data. A durable coordination record
should capture the minimum decision or evidence summary needed for auditability,
not paste raw chat content or a private link. Any allowed defect reproducer must
be sanitized under the existing secret, anonymization and external-publication
rules. The rule must not imply that moving sensitive chat text into immutable
coordination makes it safe.

## F3 — Use an owner whose scope truly covers both MRs and issues

Check: canonical-owner-boundary
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: F3-issue-surface-has-no-owner
Impact: material
Enforcement: advisory
Basis: owner scope in `merge-request-description-rule.md` and the single-owner
projection discipline
Finding state: open
Risk disposition: none
Resolution condition: the revised plan chooses one discoverable owner that
actually governs MR and issue titles, bodies and notes, and records pointers
from narrower owners where needed.

`merge-request-description-rule.md` currently declares its exact scope as
"Any agent preparing a branch for merge request review." It does not govern
issue authorship. Adding issue text to that file without changing its scope and
discovery path leaves half the proposed contract hidden. The revision must
either create a canonical forge-publication evidence owner and point the
MR-description rule to it, or explicitly broaden the existing owner's scope,
heading and discovery surfaces so issue authors are covered. Do not claim a
one-file owner while its declared scope remains MR-only.

## F4 — Supply adopter projections, migration evidence and SemVer boundary

Check: downstream-projection-and-compatibility
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: F4-breaking-change-without-adopter-route
Impact: material
Enforcement: advisory
Basis: `documentation-rule-projection-rule.md`,
`downstream-impact-rule.md`, and framework SemVer policy
Finding state: open
Risk disposition: none
Resolution condition: the revised affected paths and projection evidence cover
the adopter operating page, loader discovery, structured projection manifest
and adopter-visible migration note; release compatibility is explicit.

The plan calls the new `MUST NOT` BREAKING and says direct adopters must change
behavior. `downstream-impact-rule.md` defines the boundary precisely: "Narrowing
a previously implicit permission is BREAKING," and its adopter-docs section
requires a parity disposition for changed distributed rule behavior.
`Adopter-facing explanation: not applicable` is therefore not credible.
`agf/docs/site/docs/operate/day-to-day.md` already owns the relevant "Show the
assistant and role on GitLab" workflow and tells authors to name a durable
coordination record. Add a concise pointer or bounded summary there. Ensure the
shipped assistant loader routes both MR and issue publication work to the
canonical owner; a generic workflow-directory pointer does not clearly cover
issue authorship. The projection owner requires every triggered plan to dispose
all six surfaces and makes one YAML file under
`docs/reviews/rule-projection/` the canonical structured representation. Add it
and run the direct projection checks. The existing runbook pointer to
day-to-day may remain sufficient when the manifest records why.

Add an adopter-visible migration entry, preferably under `agf/CHANGELOG.md`'s
Unreleased section, rather than leaving the future changelog obligation outside
implementation scope. Re-run collision analysis for every newly affected path
and acknowledge the now-landed changelog-header plan and any page/loader
overlaps.

The framework's versioning owner states "MAJOR: breaking changes"; the impact
rule requires a migration note before proposing a BREAKING commit. If the
normative `MUST NOT` remains, state that it is ineligible for a v2.13.x
maintenance release or any v2.x minor release and may ship only through a
separately approved major-release plan. This compatibility constraint chooses
no version number, milestone, date or release commitment.

## F5 — Add semantic positive and negative acceptance examples

Check: human-semantic-acceptance
Check status: reviewed
Outcome: needs-review
Requirement strength: recommended
Finding: F5-no-boundary-examples
Impact: minor
Enforcement: advisory
Basis: the draft correctly rejects an unreliable automated prose classifier
Finding state: open
Risk disposition: none
Resolution condition: exact-head review demonstrates at least one forbidden
governance-evidence citation and allowed subject-matter/reproducer cases.

No mechanical prose blocker is appropriate. Keep validation advisory/manual and
the evidence label Theoretical/Provisional. Add review fixtures or explicit
examples showing the intended semantic boundary: for example, a live-chat
message cited as approval is forbidden; an issue about a Slack/webhook defect
and a sanitized payload used to reproduce it are allowed. These examples are
review evidence, not a new validator.

## Required return path

The existing Claude planner should publish a fresh claim for the same plan file
and revise MR !3426. Keep the plan `pending-gate1`; record both advisory rows as
consulted with these findings, without claiming clearance. Preserve the exact
foundry-specific `ghgs` instruction while narrowing the generic adopter rule.
After the revision, publish a new handoff and request independent contribution
and InfoSec re-review of the exact head. A binding governor Gate 1 decision is
still required before any rule, Pages, loader, projection or changelog edit.
No release version or date is selected by this review.
