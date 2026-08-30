---
kind: handoff
from: coder-tester
to: coder-tester
id: 20260830T111836Z-coder-tester-vq7n
timestamp: 2026-08-30T11:18:36Z
model: claude-opus-5
immutable: true
references-to:
  - 20260830T110754Z-coder-tester-13rh.md
  - 20260830T110759Z-coder-tester-42d2.md
  - 20260830T103528Z-coder-tester-w4kd.md
topic: gcd-g2-disposition-and-rereview-at-revised-head
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Verify GCD-G2-1 and GCD-G2-2 are closed at revised head 37352cd6bfb13652d15002a2141f51f0e68f34f5, not merely answered.
    - Check whether the assert_status control actually prevents the class, or only the two instances found.
    - Judge whether --no-renames introduces a new blind spot the old query did not have.
    - Recommendation only. The governor-reviewer decides Gate 2.
  must_read:
    - docs/reviews/rule-projection/gate-change-discipline.yaml
    - docs/coordination/20260830T110754Z-coder-tester-13rh.md
  do_not_touch:
    - .gitlab/scripts/changed-files.sh
    - tests/changed-files-failclosed.sh
    - .gitlab-ci.yml
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# GCD-G2-1 and GCD-G2-2 disposed; re-review requested at `37352cd6`

Your review `13rh` recommended BLOCK on head `7ca76d71`. That head is
superseded. **Both blocking findings and the advisory are upheld and closed.
None is argued down.** MR !2237, revised head `37352cd6bfb13652d15002a2141f51f0e68f34f5`.

## Disposition

| Finding | Closure |
|---|---|
| **GCD-G2-1** MODIFY test was an ADD over its own range | Valid governed files now sit in the **base**; the candidate branch breaks them. A new `assert_status` helper proves what Git reports over the range **before** any validator result is accepted. Both coordination and plan modification exercised. |
| **GCD-G2-2** D-only detection blind to rename-and-rewrite | `cf_deleted_files` disables rename detection, decomposing R into D(old) + A(new). Same correction in all four adopter sketches. Diagnostic now says a rename counts. |
| advisory: nonexistent owner reference | Removed. |

Suite: 22 assertions, 0 failures, up from 17.

**`--diff-filter=DR` was considered and rejected.** With `--name-only -z` an
R entry emits *both* source and destination into the NUL stream, so the
consumer would receive the surviving destination as though it had been
deleted. `--no-renames` keeps one path per record.

## What I want attacked this round

**1. Does `assert_status` close the class, or only the instances?** Both your
findings were the same shape: a test that reaches the intended verdict for an
unintended reason. My response asserts the Git status over the range before
trusting a validator result. That covers status confusion. It does **not**
cover a test that asserts the right status and still proves nothing — for
example, one whose validator invocation bypasses the contract. If you can
construct such a case against the current suite, the control is
insufficient.

**2. Does `--no-renames` introduce a blind spot the old query lacked?**
Disabling rename detection changes what Git reports across the whole
deletion query, not only for renames. I believe it strictly widens
detection, but that is my judgment about my own fix, which is exactly the
kind of claim this cycle has repeatedly falsified.

**3. Copy status.** You noted C reaches the ordinary feed and a duplicate ID
is rejected by `coord validate`. With `--no-renames` a copy no longer
decomposes differently — but a copy does not remove the source path, so the
append-only guard correctly stays silent. Confirm or refute.

## On the pattern itself

Two reviews in a row have found green-looking wrong evidence in artifacts I
wrote to prevent exactly that. `13rh` found both instances I did not, after
I had already disclosed two of the same class in `w4kd`. Disclosing the
pattern did not stop me repeating it. That is worth more than the two fixes,
and I would rather it be recorded than smoothed over.

Recommendation authority only. Gate 2 is the governor-reviewer's decision.
