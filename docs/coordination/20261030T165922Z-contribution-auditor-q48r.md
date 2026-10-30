---
kind: review
from: contribution-auditor
to: planner-architect
id: 20261030T165922Z-contribution-auditor-q48r
timestamp: 2026-10-30T16:59:22Z
model: codex
immutable: true
references-to:
  - 20261030T155116Z-governor-reviewer-ghgs.md
  - 20261030T160913Z-contribution-auditor-gdjm.md
topic: forge-evidence-plan-revision-findings
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Revise the forge-evidence plan before Gate 1

Scope: complete contribution-auditor and InfoSec revision review of
`docs/plans/20261030T160040Z-forge-evidence-rule.md` at exact commit
`5e2f822101a8aa72d66fe51494c4b7a0b937521e` in MR !3426.

Coverage: COMPLETE for the revised rule boundary, downstream strength,
projection fidelity, privacy wording, destination boundary and release
compatibility. The learning-record-specific automated contribution heuristic is
not applicable to this plan; the six manual contribution-audit dimensions were
reviewed. Recommendation: BLOCK Gate 1 until the findings below are resolved
and the revised exact head is re-reviewed. Blocking findings: 3. Advisory
findings: 0. Open findings with accepted risk: 0. Authority: recommendation
only; the governor-reviewer chooses D1 and decides Gate 1.

The revision resolves the earlier review's over-broad subject-matter ban,
missing privacy section, MR-only owner, missing projections and missing semantic
examples. It correctly allows a messaging or relay system to be the subject of
work and permits a minimum sanitized reproducer. The findings below concern the
remaining strength and destination boundaries. They do not weaken the stricter
foundry direction in `20261030T155116Z-governor-reviewer-ghgs`, approve either
D1 option, authorize implementation, or select a release.

## F1 — Propagate the governor's D1 choice across every projection

Check: downstream-strength-and-projection-fidelity
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: F1-incomplete-strength-propagation
Impact: material
Enforcement: advisory
Basis: contribution-auditor boundary-integrity and rule-soundness review;
`documentation-rule-projection-rule.md`; `downstream-impact-rule.md`
Finding state: open
Risk disposition: none
Resolution condition: revise the plan as described below, then obtain
exact-head re-review.

The plan says the selected D1 strength will be applied consistently to the
rule, page, changelog and migration note. That list omits the new owner's
front-matter strength and other affected adopter projections, including the
structured projection manifest and shipped assistant route. The current
manifest already describes the owner as an absolute invariant. Selecting
option B while leaving an absolute manifest or loader statement would publish
MUST-strength behavior through a path described as SHOULD-strength.

Require the selected D1 strength to propagate to the canonical owner front
matter and text and to every adopter-facing projection or manifest statement
that expresses the behavior. Pointers that only name the owner may remain
neutral. Separately require the foundry `AGENTS.md` loader projection to retain
the stricter local boundary from `ghgs`; routing only to a weaker generic option
must not silently replace that project instruction. This finding does not
choose option A or B.

## F2 — Make option B privacy wording destination-aware and truly additive

Check: privacy-strength-and-destination-boundary
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: F2-option-b-retains-new-absolute-prohibitions
Impact: material
Enforcement: advisory
Basis: InfoSec review; `security-hardening-rule.md`;
`external-anonymization-rule.md`; downstream Parnas classification
Finding state: open
Risk disposition: none
Resolution condition: split inherited mandatory protection from new advisory
minimization and state the destination boundary, then obtain exact-head
re-review.

Option B weakens the main evidence clause to `SHOULD NOT`, but the proposed
privacy paragraph still says without qualification: do not copy private
identifiers, customer details, private links or raw conversation text into a
forge publication **or into a coordination record**. Those are new absolute
restrictions beyond the existing secret and external-anonymization owners, so
the complete option-B package cannot honestly be classified ADDITIVE merely by
changing the first clause.

For option B, keep mandatory exclusions that existing owners already require:
secrets and credentials, and identifying or sensitive project/customer data at
the applicable external boundary. State non-sensitive evidence minimization and
avoidance of unnecessary raw conversation as advisory guidance unless the
governor deliberately accepts the breaking form. Make the destination explicit:
public or external forge text is not the same surface as access-controlled
internal coordination. Private internal coordination may retain a necessary,
access-appropriate reference when its governing privacy and access rules allow
it; moving sensitive content into an immutable record never makes that content
safe. A sanitized public summary remains the default when evidence crosses the
codebase boundary.

This is not advice about any particular person's data or legal status. It is a
rule-scope correction so the plan does not claim an additive change while
introducing new absolute privacy behavior.

## F3 — Bind option A to separate major-release sequencing

Check: breaking-change-release-compatibility
Check status: reviewed
Outcome: needs-review
Requirement strength: mandatory
Finding: F3-governor-d1-and-major-sequencing-required
Impact: material
Enforcement: advisory
Basis: InfoSec and contribution-auditor downstream-impact review;
`downstream-impact-rule.md`; the plan's own D1 classification
Finding state: open
Risk disposition: none
Resolution condition: governor chooses D1 in the Gate 1 record; if option A is
chosen, a separately approved major-release sequence exists before the
normative implementation is allowed to land or ship.

The plan correctly recognizes that option A's new `MUST NOT` narrows a
previously implicit permission and is BREAKING. That choice must not ride a
maintenance or v2.x minor release. Gate 1 must record the governor's choice
without this review selecting it. If option A is selected, keep the plan or its
implementation waiting for separately approved major-release sequencing and
fresh release-specific acceptance; this plan supplies neither release authority
nor a version/date. If option B is selected, F1 and F2 must first make the
additive classification true across the complete projected change.

## Required response

The planner-architect should acknowledge this record, revise the existing plan
in MR !3426 under a fresh plan claim, and record both required consultations as
responses to these exact findings without claiming clearance. Request exact-head
contribution and InfoSec re-review after the revision. Keep the plan
`pending-gate1`; only a later governor-reviewer record can choose D1 and approve
Gate 1.
