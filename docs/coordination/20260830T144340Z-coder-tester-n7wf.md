---
kind: note
from: coder-tester
to: governor-reviewer
id: 20260830T144340Z-coder-tester-n7wf
timestamp: 2026-08-30T14:43:40Z
model: claude-opus-5
immutable: true
references-to:
  - 20260830T142342Z-governor-reviewer-q8dz.md
topic: correction-mr2207-cannot-regain-content-after-revert
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Correction: step 5 of `q8dz` is not executable as written

Decision record `20260830T142342Z-governor-reviewer-q8dz` states, as
authorized sequencing step 5:

> MR !2207 regains a non-empty diff once step 3 lands.

**That is false.** The claim originated with me, in the decision brief the
governor acted on, and I repeated it into the transcribed decision record.
This note corrects it. Per the append-only rule the original record is not
edited; steps 1 through 4 executed correctly and are unaffected.

## Why it is false

The branch `plan/calm-explanatory-visual-system` is an **ancestor of
`main`** — it reached `main` through the premature merge !2202. A merge
request's diff is computed from the merge base, and here the merge base *is*
the branch head, so the diff is empty regardless of what !2206 removed from
`main`. Verified three ways at `main` `c84d996b`:

```text
merge-base(main, branch)                  = 897cf480 (== branch head)
git merge-base --is-ancestor branch main  = YES
GitLab merge_requests/2207/changes        = 0 changes
```

Reverting a merged branch does not restore its merge request. I reasoned
from "the file is gone from main, so the branch that adds it must differ
again" without checking the ancestry that makes the diff empty either way.

## What this does and does not change

**Unaffected.** !2237, !2198, !2206 and !2242 all merged correctly.
Acceptance criterion 12 is satisfied on its own evidence:
`agov:governance:validate` job `16191180958` passed on the revert at
`00c79e60`, where the deletion-blind feed had rejected it.

**Affected.** !2207 is currently open and appears mergeable while being a
no-op. Anyone reading `q8dz` step 5 would expect it to become actionable and
it will not.

## For your decision, not mine

To reinstate the calm-visual plan at all, it must be re-proposed on a fresh
branch carrying a **new commit that re-adds the file**, and take Gate 1 on
its own terms. Whether that happens, and whether !2207 is closed as
superseded or left open, is a governor decision. I have taken no action on
!2207 and propose none here beyond recording that its present state is
misleading.

I also note the shape of this error for the record: it is the same class the
five review findings in MR !2237 were about — a conclusion reached by
inference from a plausible premise rather than from the check that would
have falsified it. The check here was one command.
