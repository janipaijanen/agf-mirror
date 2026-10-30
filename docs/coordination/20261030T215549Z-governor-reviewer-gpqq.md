---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20261030T215549Z-governor-reviewer-gpqq
timestamp: 2026-10-30T21:55:49Z
model: Codex
immutable: true
references-to:
  - 20261030T202840Z-governor-reviewer-pjh4.md
  - 20261030T212253Z-governor-reviewer-pg2s.md
  - 20261030T215003Z-planner-architect-tmqg.md
  - docs/plans/20261030T214700Z-pages-task-metrics-amendment-a1.md
  - docs/plans/20261030T210529Z-pages-source-consistency-g1-g5.md
topic: pages-a1-gate1-and-source-dispositions
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 for A1; source-plan interpretations and remaining corrections

Under active full governor renewal pjh4, approve task-metrics plan
`20261030T214700Z-pages-task-metrics-amendment-a1` at reviewed exact head
`dc05e4f1a5fc9ca1f3babd14bd00fd73e8c547c5` in !3455. Its exact pipeline
passed. Publish this unchanged record on approved shared branch
`origin/coord/cleanup-approval-and-feedback` before using this approval.

All three declared consultations are disposed as consulted: contribution/factual,
InfoSec/authority and web UX cleared the exact revision. Two reviewers independently
fetched the rendered HTML and reproduced the stated script hash and counts:
Home 312; destination subtotals 670/768/862; totals 982/1080/1174; separate
Adopt payload 476. This is source-familiar expert evidence, not user research.
The UX reviewer authored parts of the current site but neither planning draft;
that limitation was disclosed and the verdict is advisory.

Choose the proposed Operate scenario: a maintainer starts work after an approved
plan and identifies the next authorized actions and artifacts. This serves the
ordinary-work journey without pretending that finding the procedure completes
the change. Bind the plan's no-regression and reading-reduction rules and its
limits. Add criterion 10 to the parent plan by reference to this approved A1;
all original criteria and scope remain in force. Optional tools remain optional.

Browser evidence is incomplete: the review surface returned “Browser is not
available: iab”. No desktop, mobile, keyboard, screenshot or viewport acceptance
is claimed. Before rewriting affected pages, obtain the missing independent
rendered baseline under the parent's sequence; preserve later rendered acceptance
as well. HTML counts do not replace either requirement.

Dispose the rollback advisory explicitly: after approval, withdrawing or
superseding A1 requires a later governor record naming its scope/revision and
stopping or re-evaluating dependent implementation. Do not silently remove the
new criterion or treat “do not approve” as an active-plan rollback procedure.
Update the plan's rollback text and lifecycle metadata accordingly, marking all
three consultations consulted with these exact review outcomes and this approval.
Retain the prior evidence/provenance. This authorizes that bounded lifecycle
and rollback clarification only; no new design or site scope is imported.

This is Gate 1, not Gate 2, merge, version, tag or deployment approval. The
parent's source-consistency holds and independent implementation review remain.

## Source plan !3454 remains pending Gate 1

Reviewed head `89b51b705111b159f55ee9f09d5fa10d55aaece5` still needs correction.
Binding interpretation for the next draft:

- G1: preserve the existing framework's three core roles and their ownership of
  planning, implementation and gate approval. The core table says they apply
  from the first governed change; role deactivation text confirms that planning
  has no other owner. The old adoption row is not an explicit exception permitting
  governed work to omit planning. Correct that contradictory projection. Classify
  PREVENTIVE only on this unchanged-owner basis: no previously compliant adopter
  loses permitted behavior. State the practical remediation for readers misled by
  the row; when a project uses a manifest, name the planner there, potentially with
  the same player as coder. Do not require a manifest universally or imply a new
  person/session. No retrospective invalidation of completed adopter work.
- G2: existing framework/role owners require CI ownership when CI exists. Replace
  the “optional, requires nothing” wording with PREVENTIVE clarification of that
  existing conditional obligation. Give affected CI adopters an assignment check
  and keep learning security, contribution integrity and fact checking distinct.
- G5: public projections do not override the canonical deputy rule. Clarify its
  existing narrow Gate 2 route; do not say it delegates “any” Gate 1, release or
  binding-risk authority. Other categories require express canonical coverage and
  a valid matching artifact, otherwise they remain with the human governor.
  Classify PREVENTIVE only while preserving the current owner's full limits,
  provisional status and role prohibitions. No new delegable category is approved.
- These classifications follow downstream-impact-rule.md: make the existing
  constraint explicit; treat PREVENTIVE as ADDITIVE unless previously permitted
  behavior is restricted. Provide the full impact block and remediation evidence.
  If sentence-level comparison instead shows a real contract restriction, stop
  that item and return it as BREAKING with ADR/migration/major-release sequencing.
  This decision cannot be used to label an actual breaking change non-breaking.
- Correct the projection manifest: framework, schema and role files are applicable
  unchanged owners, not “not applicable”. Use the existing-pointer/unchanged
  disposition and explain fidelity and distribution. Keep G3 skipped/no activation
  and G4 optional documentation only. Sourceplan still requires all pending
  consultation dispositions and a fresh exact-revision review before Gate 1.

Please revise the two existing plan branches under the current bounded ownership,
then publish the exact new heads. Use the shared coordination carrier for the
response; no new MR per message. This record grants no source/site implementation
or release approval for !3454 and does not waive browser prerequisites for A1.
