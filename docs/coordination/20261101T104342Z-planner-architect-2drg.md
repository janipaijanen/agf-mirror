---
kind: note
from: planner-architect
to: governor-reviewer
id: 20261101T104342Z-planner-architect-2drg
timestamp: 2026-11-01T10:43:42Z
model: claude
immutable: true
references-to:
  - 20261101T101947Z-planner-architect-rj1s.md
  - 20261101T100352Z-planner-architect-ha4h.md
  - 20261030T165922Z-contribution-auditor-q48r.md
  - 20261030T155116Z-governor-reviewer-ghgs.md
topic: bridge-d1-option-a-selected
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Bridge record: the governor selected D1 option A (live decision)

This is a bridge record. In a live session on 2026-11-01 the governor selected
D1 option A for the plan in MR !3426: keep `MUST NOT` in the generic rule. This
note transcribes that decision. It is **provisional** and is not binding
authority until a `governor-reviewer` record (an `ack`, a `decision-record` or a
ratification) references it. It follows the earlier bridge record
`20261101T101947Z-planner-architect-rj1s`, which said D1 was not yet chosen.

## What the live decision covers

Only the D1 choice. Option A means the evidence clause and the new owner's
front-matter strength are `MUST NOT` / `MUST`, the change is classified
BREAKING, and the implementation may not merge or ship before a separately
approved major-release sequence exists (the plan's option A path).

## What it does not cover

- **Gate 1 is not approved.** The governor did not approve Gate 1 in this
  session. The plan stays `status: pending-gate1`. Its `gate1_approved_by` and
  `gate1_approved_at` fields stay unset.
- The plan file is not edited to record D1. The Gate 1 record must record the
  choice, per the plan.
- Both advisory rows (`infosec-sentinel`, `contribution-auditor`) stay
  `consulted` and are not cleared. Independent re-review of plan head
  `ff368bdb49341d67d71a5aad7e6cfee59836b289` is still requested by
  `20261101T100352Z-planner-architect-ha4h`.
- No implementation, Gate 2, release, version, milestone or date authority.

## Request

Please publish a `governor-reviewer` record that references this note and either
ratifies the D1 choice or corrects it. Gate 1 approval, when you give it, comes
in a governor record that names the exact reviewed head.
