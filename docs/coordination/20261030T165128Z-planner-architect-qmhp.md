---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261030T165128Z-planner-architect-qmhp
timestamp: 2026-10-30T16:51:28Z
model: Codex
immutable: true
references-to:
  - 20261030T163253Z-planner-architect-3sgb.md
  - docs/plans/20261030T163253Z-pages-structure-and-voice.md
topic: pages-structure-and-voice-gate1
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the concrete Pages IA and voice proposal; obtain the outstanding final estate-steward consultation before any Gate 1 approval.
    - Disposition the older pending discovery plan explicitly at Gate 1 rather than running overlapping plans.
  must_read:
    - docs/plans/20261030T163253Z-pages-structure-and-voice.md
    - docs/reviews/rule-projection/20261030T163253Z-pages-structure-and-voice.yaml
  do_not_touch:
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Handoff: concrete public Pages structure and voice proposal

MR !3433 at exact `a4137ff6574cf79159468efb158e41ee38045f0a`, shared branch
`origin/plan/pages-structure-and-voice`, contains the pending-Gate-1 plan and
initial canonical projection manifest. Claim `3sgb` (shared through !3432)
is released by this handoff. Codex resumes coordination/review duties.
No site implementation, merge, role activation, version, tag or release is
approved by this record. Ready means ready for governor review, not that
all Gate 1 preconditions are complete.

## Result and source of direction

The human requested a substantial replan for first-time visitors, CTOs,
business owners and AI leads, with consistent voice and understandable role
terms. Follow-up asked that agent roles, participants, defaults, activation,
self-approval boundaries and requesting new specialists be explained.
Claude supplied read-only documentation-estate consultation and challenged the
first concrete draft. This plan incorporates the challenge, not just menu labels.

Home explains the framework through one illustrative change. Five purposes
follow it: Decide, Get started, Work with AGF, Reference and About. Every page
has a declared content/navigation owner. Setup has one sequence. Human readers
and assistant execution are explicitly distinguished. The role page separates
shipped definitions, skipped example entries, documented solo/standard models
and the adopter's actual assigned roster. No automatically running role is
invented. Existing URLs/fragments, unique safeguards and exact build provenance
remain protected; llms.txt follows the same reading paths.

Six bounded implementation MRs are sequenced as roles, Decide, setup, work,
Home and navigation/reference. Content precedes navigation. Existing main CI
can publish each slice: its Gate 2 covers that automatic consequence, then
live affected pages/links/source are verified before dependent slices proceed.
Final integrated verification and a later release are separate events.
Patch eligibility is conditional on unchanged normative meaning and contracts.

## Consultation evidence

- Claude's initial navigation/voice advice was followed by ten concrete
  challenges to draft b968a1fb: size, absent baseline, ambiguous ownership,
  static link compatibility, section-index mechanics, llms.txt, authority
  preservation, local roster presentation, illustrative examples and expert
  reviewer routing. Revision c917f4ab addresses them. Final estate-steward
  review of c917f4ab plus the bounded a4137ff6 deployment correction was
  requested and remains outstanding at publication of this handoff.
- Independent web-ux-reviewer, existing session `cipins_image_triage`, reviewed
  the baseline and plan. It required role-comprehension tests, named desktop/
  mobile first-screen CTA checks, exact shared-summary contract preservation
  and per-slice automatic-deployment acceptance. Final advisory CLEAR at
  a4137ff6; no rendered implementation acceptance is claimed.
- Independent contribution-auditor, existing session `mr3340_contribution_audit`,
  required canonical six-surface projection evidence and conditional patch
  classification. Final contribution advisory CLEAR at a4137ff6.
- The same independent reviewer supplied a separately identified factual
  assessment, CLEAR at a4137ff6: skipped template and documented solo/standard
  models are accurate; foundry's live roster is not an adopter default.
- Independent InfoSec reviewer, existing session `mr3340_infosec_review`,
  returned CLEAR at a4137ff6. Privacy, trust, provenance, release and authority
  boundaries remain intact; compatibility pages must not become duplicate
  normative owners.

These are plan reviews, never gate approvals. Keep the plan pending until
consultation metadata is reconciled and the final estate review arrives.
The genuine tension between public self-approval wording, ADOPTING Phase 1
human conflict guidance and bounded deputy delegation requires owner-based
factual reconciliation. If that requires policy, stop the affected wording
for its own binding decision; editorial simplification cannot decide it.

## Validation and remaining work

Plan schema, diff whitespace and the strict projection check pass. The
changed-plan collision gate passes with the 16 declared-path predecessor
intersections acknowledged; 204 pathless plans remain outside automated
coverage. The overlapping older pending discovery plan
`20260809T063148Z-pages-rework-discovery-and-ia` was read manually and is
explicitly proposed for a binding supersession disposition, not silently
closed. The active forge-evidence plan's day-to-day page section remains
reserved pending explicit implementation sequencing.

Final usability evidence will compare sixteen fixed audience questions on
baseline and candidate, plus role-activation/solo/active-specialist scenarios,
old-link coverage and independent light/dark desktop/mobile rendered review.
No implementation walkthrough, visitor study or resulting usability gain is
claimed yet. All later implementation claims, exact-head checks, decisions
and publication evidence remain mandatory.
