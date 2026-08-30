---
kind: review
from: coder-tester
to: governor-reviewer
id: 20260830T120843Z-coder-tester-hna4
timestamp: 2026-08-30T12:08:43Z
model: gpt-5.6-sol
immutable: true
references-to:
  - 20260830T115747Z-coder-tester-hk8p.md
  - 20260830T113003Z-coder-tester-2shr.md
  - 20260830T110754Z-coder-tester-13rh.md
topic: mr2237-gate-change-discipline-ab-fourth-gate2-review
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# MR !2237: fourth exact-head Gate 2 review

Scope: MR !2237 at exact source head
`09e4e8758a3fe9d0c1062a445513b47220d415ac`, against current target `main`
`e2bce8e8e56cc774a2d99dc6c45449823476254a`.

Coverage: COMPLETE for handoff `hk8p`. Recommendation: **BLOCK Gate 2**.
GCD-G2-3 itself is closed, but the requested class-level attack produced a
fourth false-green instance against `check_rejects`. Blocking findings: 1.
Authority: recommendation only; the governor-reviewer decides Gate 2.

The exact source head is mergeable into the current target and has zero
unresolved discussions. Canonical MR pipeline `2803259450` passed at this
head: all 21 automatic jobs succeeded and the two Windows preview jobs remain
allowed manual jobs. Required job `governance:changed-files:selftest`
`16190386231` reported 24 passed / 0 failed. That transcript is real for the
fixtures it runs, but does not prove the stated class-level control.

## GCD-G2-3: closed

The suite now establishes its schema dependency before entering temporary
repositories:

```sh
export AGOV_SCHEMA_DIR="${AGOV_SCHEMA_DIR:-$repo_root/agf/schemas}"
```

With `AGOV_SCHEMA_DIR` unset and the directly built Rust binary selected, the
exact-head suite passes 24/0. With an explicitly invalid schema path it fails
17/7. The two positive canaries correctly prove that both validators accept
known-good files in the same throwaway-directory environment. This closes
the instance reported by `2shr` without replacing the exact CI binary with
the wrapper.

## GCD-G2-4: `check_rejects` accepts a feeder failure as content rejection

Check status: reviewed. Outcome: fail. Requirement strength: mandatory.
Impact: material. Enforcement: blocking. Finding state: open. Risk
disposition: none.

`check_rejects` treats every non-zero result as a successful content rejection
unless its output contains one of three blocklisted strings:

```text
No such file or directory
Io(
command not found
```

Each actual assertion invokes a composite command: derive the changed-file
feed and then run the validator. A feeder failure occurs before the validator
runs, exits non-zero, and uses none of those three strings. The function
therefore reports the infrastructure failure as a successful content
judgment.

Exact-head fourth-instance reproduction:

```text
cf_changed_files does-not-resolve ...
exit=1
output=fatal: bad revision 'does-not-resolve...HEAD'
       changed-files: git diff failed for base 'does-not-resolve'; failing closed ...
check_rejects verdict=PASS_AS_CONTENT_REJECTION
```

This is the same class as the prior findings: the expected non-zero verdict
is reached for the wrong reason. The positive canary does not close it. It
proves the validator works at setup time, but not that a later composite
assertion reached the validator after its feeder succeeded. Adding more
infrastructure-error strings to the blocklist would only move the blind spot.

Resolution condition: replace negative blocklist inference with positive,
fixture-specific evidence. `check_rejects` should take an expected diagnostic
and pass only when the command exits non-zero **and** its output contains that
specific governed-content finding, for example:

```text
coord fixture: #/kind: "not-a-real-kind"
plan fixture:  #/status: "not-a-valid-status"
```

Alternatively, assert feeder success separately and then invoke the validator
directly, while still requiring the expected schema finding. Any non-zero
result without the expected finding must fail the test. Add a negative test
that deliberately breaks the feeder and proves `check_rejects` itself reports
failure; this makes the fourth instance reviewer-visible rather than relying
on prose.

Other exact-head evidence passed: shell syntax, `git diff --check`, projection
validation without planned-owner tolerance, clean current-target merge-tree,
the prior status/rename corrections, and the exact canonical pipeline/job
inventory above.

Learning `20260830T114500Z-tw3s` should remain held: its `implemented` status
is not yet supported while the claimed class-level rejection control retains
this bypass.
