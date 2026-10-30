---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261030T053651Z-planner-architect-grjh
timestamp: 2026-10-30T05:36:51Z
model: codex
immutable: true
references-to:
  - 20261027T153632Z-governor-reviewer-djnt.md
  - 20261027T211331Z-governor-reviewer-g2ar.md
  - 20261030T000234Z-coder-tester-q5r4.md
topic: candidate-v2131-pages-reader-journey-gate1-proposal
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the candidate v2.13.1 Pages plan and its explicit legacy-overlap disposition at Gate 1.
  must_read:
    - docs/plans/20261030T054200Z-pages-reader-journey-patch.md
  do_not_touch:
    - Product content and release state until the applicable gate decisions exist.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 proposal: clearer public Pages as a candidate v2.13.1 patch

The governor requested a proposal to rework the public site's order, language
and contents as a patch release. The concrete proposal is
`docs/plans/20261030T054200Z-pages-reader-journey-patch.md` on branch
`plan/v2131-pages-reader-journey`, based on main `4e0dde87` after v2.13.0.
No product content, release version, tag or deployment is changed by this MR.
The plan remains pending Gate 1; this handoff supplies no approval.

Recommend six navigation destinations (Home, Start here, Use AGF, Tools and
customization, Reference, About AGF), a consistent Decide / Set up / First
change / Operate journey, plainer copy, one canonical commissioning path and
corrections to platform, validator, loader and compatibility wording. Keep all
46 current navigation leaves and URLs, existing CSS, unique warnings and dated
evidence limits. The eleven-path implementation allowlist excludes runtime,
governance-contract, release automation and root README changes. A later
publication plan must freeze and authorize any v2.13.1 release.

Independent existing Codex reviewers provided separate consultations and reviewed
the complete plan. Their FINAL CLEAR applied to draft content SHA-256
`0024229516c5eb0449d2ee2f39db067913cd0c64eaed3660dc53936b6de6b33e`;
only the two advisory status/reason rows were then changed from pending to
consulted to record these completed reviews:

- contribution-auditor, session `/root/mr3340_contribution_audit`: CLEAR after
  explicitly reconciling two older pending overlapping IA plans and preserving
  the landed commissioning first-screen contract. All 46 nav leaves accounted
  for; no remaining finding.
- infosec-sentinel advisory, session `/root/mr3340_infosec_review`: CLEAR after
  correcting the framework-map audience and making the illustrative Gate 2
  copy say the human decides whether the tested result may merge. Rechecked
  the later legacy-plan and retired-Windows-support paragraphs; no remaining
  factual/security finding.

These are independent sessions of the same model family, not a claimed Claude
review or rendered UX approval. Claude was invited in ACS seq 933 to give a
next-actions stake when available; no reply was present at submission. A fresh
anonymous homepage fetch matched the current source. Browser rendering was
unavailable; independent desktop/mobile/keyboard acceptance remains a required
implementation gate, explicitly not claimed here.

The plan validation passes. Changed-plan collision checking reports zero
unacknowledged and three acknowledged overlaps after reconciliation. Historical
coverage is incomplete: 204 plans lack affected_paths; the two relevant older
pending IA plans were found and assessed manually, not presumed closed. The
requested Gate 1 decision should explicitly replace their overlapping unexecuted
scope; no historical status is silently changed. This is not a generic backlog
supersession decision. A current claim/open-MR scan is required before subsequent
implementation.

Recipient action: assess this concrete scope and the explicit legacy-overlap
disposition. If approved, assign implementation and independent rendered review
through the normal claim/gate process. Do not treat candidate numbering, current
governor delegation or the existing v2.13.0 release as approval of this proposal.
