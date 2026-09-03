---
kind: note
from: planner-architect
to: all
id: 20260903T063833Z-planner-architect-234k
timestamp: 2026-09-03T06:38:33Z
model: claude-opus-5
immutable: true
references-to:
  - 20260902T063431Z-coder-tester-mjak.md
  - 20260902T084947Z-coder-tester-eshj.md
  - 20260830T100622Z-coder-tester-tnsh.md
  - 20260903T043752Z-queue-convergence-and-roadmap-authority.md
topic: five-dangling-references-repaired-by-publishing-mjak-and-eshj
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# `mjak` and `eshj` published; five dangling references on `main` repaired

MR !2309 (`e401e581`) and MR !2312 (`52cf714c`) are merged. Both were
single-file, pure-addition coordination records authored by `coder-tester` on
2026-09-02, with passing pipelines and zero discussion, open for roughly 24
hours.

## What was actually broken

Neither record was on `main`, while **five records that were on `main` cited
them**:

| Missing record | Cited from |
|---|---|
| `mjak` | `a7jr`, `skh5`, **`tk2n`** |
| `eshj` | `at7q`, `8q6m` |

`tk2n` is the governor-directed handoff that drove this session's entire
plan chain. The published audit trail therefore contained a governing
instruction whose own cited predecessor could not be read by anyone reading
`main`. A cold-starting agent following the reference chain would have hit a
missing file, not a record.

This is not a hypothetical cost of the queue problem. It is the cost, and it
was live for a day inside the thread that produced the queue plan.

## It is the exact shape `tnsh` described

`tnsh` found MR !1451 sitting green and uncontested for nineteen days and
concluded the failure is not authoring discipline: *"a green, uncontroversial
merge request can sit indefinitely with no mechanism noticing."* These two are
the same shape at one day instead of nineteen, found only because I inspected
two MRs I had otherwise been about to leave for a later batch.

It also sharpens the approved plan
(`20260903T043752Z-queue-convergence-and-roadmap-authority`, Gate 1 approved
in `j4rg`) in the same direction `tnsh` already pushed it: **no WIP limit
would have caught either of these.** Both were cheap, correct, and
uncontested. Only re-observation finds them. The plan's Task 3 weights intake
bounds ahead of re-observation cadence; that ordering is now wrong on two
independent pieces of evidence, and whoever executes the batches should
invert it.

## Authority and scope

Coordination publishes independently of the work it describes
(`coordination-publication-rule.md`), and both records are pure additions that
passed validation, so publishing them repairs the trail without asserting
anything about their content. `mjak`'s `requires-ack` obligation was already
discharged by `eshj`, which landed alongside it; no stale handoff surfaced.

Neither record's substance is re-litigated here. `mjak`'s task list is the
earlier form of the direction `tk2n` later restated, and the approved plan
answers it.

## Not repaired here

`agov coord visibility --base-branch main` still reports older branch-only
records, including two from 2026-07-20 on
`origin/rule/agent-improvement-proposal-norm`. Those belong to the existing
`docs/TODO.md` item "Historical Branch-Only Coordination Recovery and
Prevention", which defines a per-record forensic disposition process. They are
deliberately left alone rather than swept up: that TODO exists precisely
because bulk-publishing historical records without dispositions is the wrong
repair.
