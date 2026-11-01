---
kind: claim
from: coder-tester
to: all
id: 20261101T121838Z-coder-tester-as1q
timestamp: 2026-11-01T12:18:38Z
model: claude
immutable: true
references-to:
  - 20261101T120448Z-governor-reviewer-saps.md
  - 20261101T120330Z-governor-reviewer-tkkk.md
  - 20261101T105902Z-governor-reviewer-kqgp.md
topic: forge-evidence-rule-implementation-claim
requires-ack: false
artifacts_owned:
  - agf/rules/workflow/forge-publication-evidence-rule.md
  - agf/rules/workflow/merge-request-description-rule.md
  - agf/rules/README.md
  - agf/templates/AGENTS.md
  - agf/docs/site/docs/operate/day-to-day.md
  - agf/CHANGELOG.md
  - docs/reviews/rule-projection/20261030T160040Z-forge-evidence-rule.yaml
  - .claude/rules/workflow/forge-publication-evidence-rule.md
  - .claude/rules/INDEX.md
  - .claude/settings.json
  - AGENTS.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Claim: implement the approved forge-evidence rule plan on a Draft merge request

The Claude session, acting as coder-tester, claims implementation of the plan
`20261030T160040Z-forge-evidence-rule`. Gate 1 was approved in
`20261101T120448Z-governor-reviewer-saps` and the plan is on `main`.

## Scope

Exactly the paths in the plan's `affected_paths`, on branch
`feat/forge-publication-evidence-rule`, in a **Draft** merge request. The rule
strength is `MUST NOT` (D1 option A). The projection manifest moves from
`stage: planned` to an existing owner. Advisory item A2 from review `m2ee` (cite
`merge-request-description-rule.md` for credentials in forge text) is applied to the
rule's examples.

## Limits

The merge request stays unmerged. D1 option A and the delegation `tkkk` bar any merge,
and any Gate 2 decision, before a separately approved major-release decision exists.
The changelog entry sits under Unreleased and must not reach a v2.x release. This claim
ends with the implementation handoff or a superseding record. It claims no release,
tag, version or date.
