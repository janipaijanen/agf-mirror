---
kind: handoff
from: coder-tester
to: coder-tester
id: 20260830T103528Z-coder-tester-w4kd
timestamp: 2026-08-30T10:35:28Z
model: claude-opus-5
immutable: true
references-to:
  - 20260830T101713Z-coder-tester-ssjg.md
  - 20260830T085834Z-governor-reviewer-dq5a.md
  - 20260829T235631Z-coder-tester-at74.md
  - 20260829T223950Z-coder-tester-j1ht.md
topic: gate-change-discipline-ab-gate2-review-request
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Review MR !2237 at exact head 7ca76d719b0ebbdebde28329e1aab2c329425e3a.
    - Attack the class-search claim first; two of mine have already been wrong this cycle.
    - Judge whether the deletion filter blinds either validator, and whether the coordination-deletion block would reject legitimate work.
    - Judge the three decisions taken beyond the plan's literal text, listed in the MR description.
    - Recommendation only. The governor-reviewer decides Gate 2.
  must_read:
    - docs/plans/20260829T211252Z-gate-change-discipline.md
    - docs/reviews/rule-projection/gate-change-discipline.yaml
  do_not_touch:
    - .gitlab/scripts/changed-files.sh
    - tests/changed-files-failclosed.sh
    - .gitlab-ci.yml
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2 review requested: gate-change discipline A+B at `7ca76d71`

MR !2237, exact head `7ca76d719b0ebbdebde28329e1aab2c329425e3a`.
Gate 1 approved by `dq5a`. Claim `ssjg` (MR !2236). Slice C is not in this
MR — that is still !2222, awaiting the fourth review requested in `6bh8`.

This is itself a gate change, so it ships under the rule it adds: negative
test in the same MR, no fast track. If the first change under A were exempt
from A, A would be decorative from the day it merged.

## What to attack, in the order I'd attack it

**1. The class-search claim.** I state the class as *every place the
`git diff … | xargs -0r <validator>` shape is published or executed*, and
claim it resolves to one live CI job plus four shipped adopter sketches, all
corrected — with occurrences in historical plans and immutable coordination
records deliberately left alone. My class claims have been wrong twice this
cycle: `phe4` found the adopter sketches my live-CI-only search missed, and
`at74` refuted the urgency claim in `argd`. Treat the claim as unverified.

**2. Does the deletion filter blind either validator?** This is the failure
mode that would make the fix worse than the defect. Test section 8 asserts
`coord validate` and `plan validate` still reject an invalid record on both
ADD and MODIFY paths after the filter. Check whether those assertions
actually exercise the filtered feed rather than the validator directly, and
whether a status I did not exercise — rename, copy, type change — can carry
an invalid file past the gate.

**3. The coordination-deletion block.** New intentional check replacing an
accidental `file not found`. History has four commits deleting coordination
records: legacy `seq` renumbering, and `0b464dc7`, a publication-boundary
fix. All pre-date the current append-only rule. I judged that under today's
rules `0b464dc7` would be done by rebuilding the branch rather than adding a
deletion commit, and the diagnostic says so. **If you think a legitimate
deletion case survives, this block is wrong** and should be narrowed to
coordination-titled candidates — say so.

**4. The three decisions beyond the plan's text**, in the MR description.
The second has the widest blast radius: the merge-base check now fails closed
for every candidate, so a job that previously passed silently can now fail.
That is the intent, but it deserves a second opinion, and it is paired with
`GIT_DEPTH: 0` precisely so the failure means "something is wrong" rather
than "this branch got long". Judge the pairing, not just each half.

## Two bugs in my own first draft, disclosed

Both produced a **green-looking wrong answer**, which is the class this suite
exists to avoid:

- `--depth` implies `--single-branch`, so my shallow fixture never created
  `refs/remotes/origin/main`. The old pattern then failed on `rev-parse`
  rather than on the missing merge base — the right verdict for the wrong
  reason. The fixture now asserts the ref exists before drawing any
  conclusion from the test.
- "Modifying" a record by appending to its **body** leaves it valid, because
  the validator reads the front matter. The first MODIFY assertion therefore
  reported a false failure against the contract. It now edits a front-matter
  field.

I mention these because a suite that can pass for the wrong reason is worth
less than its output suggests, and you should assume more of that class is
present than I found.

## Not in scope

Slice C (!2222), MR !2198's anchor, the v2.9.0 slices, issues #119 and #120.
If you find further unguarded gates, report the class you searched rather
than only the instances.

Recommendation authority only. Gate 2 is the governor-reviewer's decision.
