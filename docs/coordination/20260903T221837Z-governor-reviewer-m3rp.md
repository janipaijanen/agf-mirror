---
kind: decision-record
from: governor-reviewer
to: all
id: 20260903T221837Z-governor-reviewer-m3rp
timestamp: 2026-09-03T22:18:37Z
immutable: true
model: human-directed-claude-sonnet-5
references-to:
  - 20260827T203743Z-governor-reviewer-nsne.md
  - 20260828T143959Z-coder-tester-gkah.md
  - 20260903T204232Z-governor-reviewer-p7hj.md
  - 20260903T221134Z-coder-tester-p9wq.md
topic: v290-governed-roadmap-work-order-gate1-approved
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 approved: v2.9.0 governed roadmap and release work order

Binding human decision. Given by the governor-reviewer in the live project
session on 2026-09-04 ("gate 1 approved. proceed as proposed"); transcribed by
claude-sonnet-5 acting under direct governor authorization. The decision is the
governor's; the boundary notes are the transcriber's.

## Decision

Plan `docs/plans/20260830T214452Z-v290-governed-roadmap-and-work-order.md`
(MR !2269), prepared under `nsne` and `gkah`, is **approved at Gate 1**. Its
`status` moves `pending-gate1` -> `approved` on the plan branch in the same MR.

The approval covers the plan as written:

- create `docs/roadmap.md` as the single governed internal roadmap;
- retain `agf/docs/site/roadmap.md` and GitLab milestones as projections that
  must not assert more than the owner;
- the bounded `docs/roadmap.md` maintenance path, modelled on the existing
  `docs/sources.md` / `docs/radar.md` research fast path, with the
  disqualifier list resolving all ambiguity toward full Gate 1;
- the issue dispositions as **recommendations** (#32, #37 shipped-closeout;
  #36 split), with closure still requiring per-issue verification at execution;
- the v2.9-v2.12 clustering as planning baseline only, not commitments.

## What this authorizes

Preparation of the roadmap artifacts, the rule-projection manifest
(`docs/reviews/rule-projection/v290-governed-roadmap.yaml`, all six rows
disposed), the bounded-maintenance workflow rule, and the maintenance-path
negative test, through **normal Gate 2**. Implementation authority for these
artifacts is assigned to the planner-architect session that filed `w7kq`,
claim-first.

## What this does NOT authorize

No version bump, no `docs/roadmap.md` content before Gate 2 merge, no
adopter-projection change before Gate 2, no milestone/label/issue-state
mutation, no schema or validator change, no release branch, tag, or
publication. Gate 2 requires the strict `agov plan projection-check` output the
plan names as evidence; a session without the Rust toolchain (`p9wq`) may
prepare the artifacts but must not represent Gate 2 as met without that
validator evidence.

## Open question carried by this plan

The `release-v2.7.0-rc.2` disposition that `m2pr` deferred and this plan lists
as blocking its scope-completion is answered in companion decision `t9ng`.
With `t9ng` filed, this plan has no remaining governor-blocking open question.
