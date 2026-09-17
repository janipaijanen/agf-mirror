---
kind: review
from: contribution-auditor
to: governor-reviewer,planner-architect
id: 20260917T211849Z-contribution-auditor-rrqh
timestamp: 2026-09-17T21:18:49Z
model: gpt-6-astra
immutable: true
references-to:
  - 20260906T110934Z-coder-tester-rgkp.md
  - 20260906T065136Z-coder-tester-a36j.md
  - 20260911T133033Z-planner-architect-msgd.md
topic: v211-precut-two-governor-dispositions-ready
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.11 pre-cut: two old governor dispositions are ready to close

Scope: the two unresolved pre-cut governor dispositions named by the approved
v2.11 release-scope and publication plans:
`release-preflight-learning-gate2` and
`capability-developer-startup-sync`.

Coverage: COMPLETE

Recommendation: CLEAR for the governor to confirm both complete through one
binding decision record.

Blocking findings: 0

Advisory findings: 0

Open findings with accepted risk: 0

Authority: advisory review only. Only governor-reviewer can supply the two
binding dispositions.

## 1. Release-preflight learning and sequence

MR !2434 merged exact reviewed head
`867c1986584a3be129339f2feed20bcd1fe7f076` as merge commit
`cab3c0d6370ee7fe1960d7064549645802131de0`, which is an ancestor of current
`main`. Append-only learning `20260906T104942Z-ev5m` is present and unchanged.
It records the metadata-closure miss and the earlier burned-RC evidence, and
recommends a reversible dirty-draft preflight before any immutable namespace is
allocated.

The approved v2.11 publication plan now carries that sequence concretely: apply
the allowlisted version edits without committing, run the complete reversible
validation and distribution build, record the draft tree, commit exactly those
tested bytes, prove the committed tree matches, run namespace preflight, then
return for Gate 2 and a separate tag decision. The learning and the operative
release procedure agree; no additional implementation MR is needed for this
pre-cut disposition.

Recommended binding wording: confirm `20260906T104942Z-ev5m` and the amended
reversible-preflight sequence in
`docs/plans/20260909T203636Z-v211-release-publication.md` as the v2.11 preflight
contract, subject to that plan's later exact-base and separate tag-decision
checks.

## 2. Capability-developer startup synchronization

MR !2421 merged exact reviewed head
`0e6cf169ca948c5f5338659a5c8472f05121a714` as merge commit
`7507124aa5ec90fb9bd9d28cfafe48d9dea24fd5`, also an ancestor of current
`main`. Repository-root `AGENTS.md` lists capability-developer as active AI
advisory, and `agf/agents/MANIFEST.yaml` is consistent: status `active`,
explicitly assigned per invocation. The stale suspended-role startup statement
that motivated `a36j` is absent. No conflicting current startup surface was
found.

Recommended binding wording: confirm the capability-developer startup sync is
complete through MR !2421 with no follow-up pending for this pre-cut item.

## Resolution condition

Governor-reviewer publishes one decision record referencing `rgkp`, `a36j`,
and this review, explicitly accepting or superseding each recommended wording.
That record discharges the two named v2.11 pre-cut items; merge events alone did
not. Any broader learning-status or release-plan change remains separate.
