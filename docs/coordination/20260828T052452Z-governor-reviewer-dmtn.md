---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260828T052452Z-governor-reviewer-dmtn
timestamp: 2026-08-28T05:24:52Z
model: human-directed-claude-sonnet-5
immutable: true
references-to:
  - 20260827T204327Z-coder-tester-ksq6.md
  - 20260827T210552Z-governor-reviewer-2h2s.md
  - 20260827T203738Z-governor-reviewer-2h4k.md
  - 20260828T050103Z-planner-architect-6rsd.md
topic: ksq6-superseded-two-line-projection-already-decided
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# `ksq6` resolved: the two-line projection question was already decided and executed

`ksq6` remains `status: blocked` in the governor-reviewer queue, asking
whether to supersede `2h4k`'s one-line authorization with the proven
two-line projection declaration. That question is **already answered**;
this record closes the obligation so the queue reflects reality.

## Disposition

`ksq6` is **superseded, not rejected**. Its recommendation was correct
and was independently reached twice:

- coder-tester reached it in `ksq6` / `drrk`;
- planner-architect reached it independently through the read-only
  full-job pre-flight recorded in `jb13`, which proved the
  `surfaces.distribution.paths` obligation sat behind the reported
  `changed_pages_or_runbooks` finding, hidden by fail-fast.

The governor decided it in **`2h2s`** (2026-08-27T21:05:52Z), which
explicitly widened the authorized content from `2h4k`'s one line to two
and stated that approving `5d3k` verbatim was insufficient. `2h2s` is
later than `2h4k` on the same subject and supersedes it.

It is executed: commit `c39f9bcc82…` — the exact source head that
received binding Gate 2 approval in `s3tg` and merged as
`25e6d752b4a7f6964fecec2d5daebd0a3d8c7660` — carries **both** lines, the
`changed_pages_or_runbooks` entry and the `surfaces.distribution.paths`
entry, plus the origin comment. Path count remained 45; no `surfaces:`
disposition changed.

No further action is required from coder-tester on `ksq6`. The active
obligation is `qt1a` (v2.7.0-rc.1 publication), which is separate.

## Why this record exists at all

`ksq6` went stale because it and the decision that answered it lived on
different unmerged branches, so neither the tooling nor either agent
could link them. That is the same visibility failure recorded in `6rsd`,
which also documents the related hazard: `2h4k` and `2h2s` are two
binding governor records for one decision, transcribed independently by
two assistants that could not see each other's work.

Both are now on `main` and `agov coord visibility --base-branch main`
reports no recent off-main coordination. The underlying mitigation —
publish a transcription before acting on it, or make one role the sole
transcriber for a decision thread — remains an open governance item
clustered with `#47` and `#82` for v2.10.0 planning. It is not a
v2.7.0-rc.1 blocker and nothing about the RC's content or proofs is
affected.
