---
kind: decision-record
from: governor-reviewer
to: all
id: 20261101T192401Z-governor-reviewer-qhjr
timestamp: 2026-11-01T19:24:01Z
model: claude
immutable: true
references-to:
  - 20261101T192358Z-contribution-auditor-mqrr.md
  - 20261101T191657Z-governor-reviewer-pr6j.md
  - 20261101T162840Z-governor-reviewer-srra.md
topic: gate1-approved-major-release-scope-49d0d52a
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 approved: major-release scope plan at `49d0d52a`

**Authority and disclosure.** Decided by the Claude session under the delegation
`20261101T162840Z-governor-reviewer-srra` (published before use; expires
2026-11-06T16:00:00Z). The version, target date, merge and tag authority were decided by the
Human Governor in `pr6j`, so this approval does not choose them. **The plan's author and
approver are the same session**; the delegation accepts that with the independent review
`mqrr` as the check. The governor may reverse this by a later record.

## Decision

Gate 1 is **approved** for `docs/plans/20261101T191112Z-major-release-scope.md` at exact head
`49d0d52a1cc4fa454105c2b4d1abd87009f31e43` (MR !3490), with one permitted follow-up commit:
the status and approval fields, plus wording fixes F1 to F3 from review `mqrr`. The commit may
touch the plan file only. Any other change returns to review.

Approval authorizes drafting the **publication plan** (its own Gate 1 and Gate 2). It
authorizes no version bump, changelog edit, tag, release or publication. The tag and
publication stay with the human governor (`pr6j`). The delegation ends before the 2026-11-08
target, so the publication plan's Gate 1 and Gate 2 need the governor or a renewed delegation.
