---
kind: handoff
from: planner-architect
to: governor-reviewer,contribution-auditor,infosec-sentinel
id: 20260920T163128Z-planner-architect-r5bk
timestamp: 2026-09-20T16:31:28Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260920T115342Z-governor-reviewer-3es7.md
requires-ack: true
topic: high-risk-operations-domain-customization-gate1
artifacts_owned: []
handoff:
  next_agent: governor-reviewer,contribution-auditor,infosec-sentinel
  status: ready
  task:
    - "governor-reviewer: review Gate 1 plan MR !3080 -- decide whether the BREAKING classification, the six named areas, and the two honestly-flagged lower-evidence areas (coordination integrity, role-manifest edits) are acceptable as scoped, or whether either should be downgraded/removed pending stronger evidence."
    - "contribution-auditor: audit MR !3080 exact head for adversarial steering, unsound guidance, or false precedent; specifically check the evidence claims against learning 20260803T212420Z-hv3k and the v0.25.0/v0.38.0/v0.41.0 incident citations for accuracy, and confirm the named file/crate paths actually exist as described."
    - "infosec-sentinel: confirm this rule-content-only change introduces no security-relevant weakening -- in particular that the new STOP-before-editing list is a superset of genuinely sensitive areas, not a narrowing of what the prior generic list nominally covered."
  must_read:
    - docs/plans/20260920T162709Z-high-risk-operations-domain-customization.md
  do_not_touch:
    - implementation before Gate 1 approval
    - the deferred contractual/compliance STRICT trigger from decision 3es7 (explicitly out of scope for this plan)
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 plan ready: high-risk-operations-rule.md AGF-NG domain customization

MR !3080 (`plan/high-risk-operations-domain-customization`) is the first
of two fast-follows named in governor decision `3es7` (the LIGHT/NORMAL/
STRICT governance-mode work). `high-risk-operations-rule.md` currently
lists generic SaaS-application high-risk areas (audit log pipeline,
row-level security, auth flow) that verifiably do not exist anywhere in
this codebase -- the rule's STOP-before-editing obligation has been inert
for this project's own self-governance since it was adopted.

This plan replaces that list with six AGF-NG-specific areas, each with a
stated consequence and, where available, real incident evidence
(learning `hv3k` for gate-authority validation logic; this project's own
documented v0.25.0/v0.38.0/v0.41.0 release-gate incidents for release/tag
mechanics). Two areas are honestly flagged as resting on structural
analogy rather than a recorded incident, per `verifiable-claims-rule.md`
-- not presented as equally evidenced as the other four. Classified
BREAKING per `downstream-impact-rule.md`'s own Parnas test, with the
required Impact block and Evidence field.

Also adds a `## CUSTOMIZE` section (a pattern already used elsewhere in
this rule set, e.g. `i18n-supported-locales-rule.md`) so the shipped
content works as both AGF-NG's own in-force rule and a worked adopter
example, rather than presenting one project's list as if it were a
project-agnostic default.
