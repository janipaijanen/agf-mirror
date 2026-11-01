---
kind: decision-record
from: governor-reviewer
to: all
id: 20261101T105902Z-governor-reviewer-kqgp
timestamp: 2026-11-01T10:59:02Z
model: claude
immutable: true
references-to:
  - 20261101T104342Z-planner-architect-2drg.md
  - 20261101T101947Z-planner-architect-rj1s.md
  - 20261101T100352Z-planner-architect-ha4h.md
  - 20261030T165922Z-contribution-auditor-q48r.md
  - 20261030T155116Z-governor-reviewer-ghgs.md
topic: d1-option-a-ratified-and-merge-3473
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# D1 option A ratified; merge of MR !3473 authorized

**Provenance.** The human governor gave these decisions in a live session on
2026-11-01 and told the planner-architect session (model: claude) to file this
record on the governor's behalf. The `from: governor-reviewer` field records the
decision-maker, not the typist. The governor may correct or supersede this record
by a later record.

## Decisions

1. **D1 is option A for the plan in MR !3426.** The generic rule keeps
   `MUST NOT`. Its classification is BREAKING under `downstream-impact-rule.md`.
   The implementation may not merge or ship before a separately approved
   major-release sequence exists. This decision names no version, milestone or
   date. It ratifies the bridge records
   `20261101T104342Z-planner-architect-2drg` and, for the earlier "D1 not yet
   chosen" state, supersedes `20261101T101947Z-planner-architect-rj1s` on that
   point only. The foundry direction in `ghgs` is unchanged and stays stricter.
2. **The merge of MR !3473 is authorized**, performed by the planner-architect on
   the governor's behalf. Scope: the MR head at merge time, no squash, with a
   passing pipeline at that exact head. The MR adds coordination records only.

## Not decided here

- **Gate 1 is not approved.** The plan in MR !3426 stays `pending-gate1`.
- Independent re-review of plan head `ff368bdb49341d67d71a5aad7e6cfee59836b289`
  is neither required nor waived by this record. The governor raised the
  question of what it would add. That question is open.
- No implementation, Gate 2, release, version, milestone or date authority.
