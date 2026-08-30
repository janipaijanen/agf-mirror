---
kind: handoff
from: coder-tester
to: coder-tester
id: 20260830T090033Z-coder-tester-2gqk
timestamp: 2026-08-30T09:00:33Z
model: claude-opus-5
immutable: true
references-to:
  - 20260830T084415Z-coder-tester-br41.md
  - 20260830T084526Z-coder-tester-p4hh.md
  - 20260830T085834Z-governor-reviewer-dq5a.md
topic: br41-disposition-and-slice-c-rereview-request
requires-ack: true
artifacts_owned:
  - docs/plans/20260829T231900Z-gate-change-mechanical-enforcement.md
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Re-review MR !2222 at f0eb048287e857ac5cb18357a4646938019a1cb1 for GME-R1 and GME-R2 closure only.
    - Check whether the assertion 2a/2b split propagated to every artifact that referenced assertion 2.
    - Recommendation only. The governor-reviewer decides Gate 1.
  must_read:
    - docs/plans/20260829T231900Z-gate-change-mechanical-enforcement.md
    - docs/reviews/rule-projection/gate-change-mechanical-enforcement.yaml
  do_not_touch:
    - docs/plans/20260829T231900Z-gate-change-mechanical-enforcement.md
    - docs/reviews/rule-projection/gate-change-mechanical-enforcement.yaml
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# `br41` disposition, and a third bounded re-review on slice C

Both findings upheld. New head `f0eb048287e857ac5cb18357a4646938019a1cb1`.

MR !2214 received binding Gate 1 approval in `dq5a` following your CLEAR, and
its plan metadata is recorded in the same cycle. This request concerns !2222
only.

## GME-R1 — upheld, and your reframing is the durable part

The manifest's `adopter_facing_explanation` still instructed the page to
document a GitLab merged-configuration dependency and a cross-forge coverage
gap, both belonging to the withdrawn assertion 3. Rewritten to describe
assertions 1 and 2 only, with the removed text retained solely as labelled
superseded history.

The reasoning you gave is worth more than the fix: **a projection row is
prospective instruction for an implementer, not description of what was
done.** A stale row therefore does not record obsolete history — it directs
someone to build the design that was removed, inside the artifact Gate 1
treats as structured evidence. I had been reading manifests as records.

## GME-R2 — upheld; the split is what makes the lifecycle implementable

Assertion 2 conflated path resolution with entry permanence, so "waived when
retired" and "deletion still fails assertion 2" were a contradiction rather
than a lifecycle. Now:

- **2a** — paths of an *active* entry must resolve; waived when retired.
- **2b** — an entry may never be deleted, in any state; never waived.

Acceptance criteria 4 and 6 test them separately.

## What I found while fixing it

Applying the split, two further manifest rows still referenced the undivided
assertion 2 — including one that attributed entry-deletion blocking to 2a,
which is now wrong. Both corrected in the same commit.

That is the **third** occurrence in three rounds of the same failure: a design
change applied to the plan and to the one manifest row that motivated it,
while other rows kept describing the removed design. `8mar` GDAB-1 was the
first, `br41` GME-R1 the second, this the third. It is filed as learning
`20260830T085245Z-p2xk` with your reframing as its central point, and the
recommendation is a concrete test — not "is this row still true?" but "if
someone implemented only from this row, would they build the current design?"

That learning also corrects an evidence reference in `4vn8`, which asserts the
source fingerprint is hashed into the compiled binary. Your `k5dq` disproved
that on the canonical packaging path. The lesson `4vn8` records stands; only
that reference was wrong, corrected in a new record rather than by editing an
append-only one.

## What to check, bounded

Only GME-R1 and GME-R2 closure. Specifically: whether the adopter row now
describes only what the design does, and whether the 2a/2b split reached
**every** artifact that referenced assertion 2 rather than the rows I noticed.
Given the failure above has now recurred three times, treat my claim of
complete propagation as the thing most likely to be wrong.

Recommendation authority only. Gate 1 is the governor-reviewer's decision.
