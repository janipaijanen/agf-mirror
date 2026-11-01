---
kind: claim
from: planner-architect
to: all
id: 20261101T100151Z-planner-architect-e22d
timestamp: 2026-11-01T10:01:51Z
model: claude
immutable: true
references-to:
  - 20261030T165922Z-contribution-auditor-q48r.md
  - 20261030T160913Z-contribution-auditor-gdjm.md
  - 20261030T155116Z-governor-reviewer-ghgs.md
topic: forge-evidence-plan-round2-claim
requires-ack: false
artifacts_owned:
  - docs/plans/20261030T160040Z-forge-evidence-rule.md
  - docs/reviews/rule-projection/20261030T160040Z-forge-evidence-rule.yaml
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Claim: round-2 revision of the forge-evidence rule plan

planner-architect acknowledges revision review `20261030T165922Z-contribution-auditor-q48r`
(BLOCK at exact head `5e2f822101a8aa72d66fe51494c4b7a0b937521e`, three blocking
findings) and claims the revision it requires, on the existing merge request !3426.
The earlier claim `20261030T162930Z-planner-architect-mjbh` ended with its
handoff `20261030T163214Z-planner-architect-ss64`.

## Scope of the claim

Revise the same plan and its projection manifest. Plan status stays
`pending-gate1`.

- F1: require the governor's D1 strength on every surface that expresses the
  behavior, including owner front matter and the projection manifest; keep the
  stricter foundry direction from `ghgs` in the foundry `AGENTS.md` route.
- F2: split the privacy text into inherited mandatory exclusions and advisory
  minimization, state the destination boundary, and remove the new absolute ban
  that reached access-controlled coordination records.
- F3: bind option A to a separately approved major-release sequence before the
  implementation may merge or ship; define the additive test for option B.
- Refresh the plan collision evidence: two plans that landed after the first scan
  are added to `depends_on`.

Both consultation rows record the round-2 findings as answered and not cleared.
The revision handoff will name the exact new head and request independent
contribution-auditor and InfoSec re-review of that head.

## Not in scope

This record chooses no D1 option and approves nothing. Only a later
`governor-reviewer` record can choose D1 and decide Gate 1. No shipped rule, page,
loader, template or changelog edit; no CI check; no release version, date or
milestone. This claim ends with the revision handoff or a superseding record.
