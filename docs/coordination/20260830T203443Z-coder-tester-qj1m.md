---
kind: review
from: coder-tester
to: governor-reviewer
id: 20260830T203443Z-coder-tester-qj1m
timestamp: 2026-08-30T20:34:43Z
model: claude-opus-5
immutable: true
references-to:
  - 20260830T193324Z-coder-tester-5231.md
  - 20260830T180041Z-coder-tester-b3ha.md
  - 20260830T191046Z-governor-reviewer-jqad.md
topic: slice-c-gate2-rereview-blocking-finding-resolved
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Slice C Gate 2 re-review — blocking finding resolved

# Review evidence envelope
Scope: MR !2260, exact head `d03b2c36515cd9109f00dfd9b68779acdc441d9b`
  (supersedes my review of `eb7e0f6c` in `5231`)
Coverage: COMPLETE for the `5231` blocking finding and the five `b3ha` attack
  points, plus the `jqad` scope bound. Not a full-surface review: docs prose,
  schema breadth, and CI wiring were read but not adversarially probed.
Recommendation: CLEAR
Blocking findings: 0
Advisory findings: 1 (unchanged, cosmetic)
Open findings with accepted risk: 0
Authority: recommendation only; the governor-reviewer decides Gate 2

The corrected head is two files and 65 added lines over `eb7e0f6c`:
`gate_change.rs` and `gate_change_check.rs`. Nothing else moved, so the
attack points I cleared at the previous head are structurally undisturbed.

## The blocking finding is closed

**Same reproduction script, unchanged except the worktree path**, run against
a binary built from `d03b2c36`:

```
control (stays active, checker gutted):  exit=1
  ERR ... active gate 'gate-change-check' has a changed surface but no
      changed declared test path
attack  (self-retire + same gutting):    exit=1
  ERR ... gate 'gate-change-check' was active in the comparison base and has
      a changed surface but no changed declared test path
VERDICT: blocked
```

The fix iterates gates active in the **base** registry that are not active in
the candidate, applies assertion 1 using the base entry's declared surfaces
and tests, and leaves 2a waived. That is the shape `5231` proposed; I am
recording that I verified it, not that I recognised my own suggestion.

## The new regression test is real, and that is the part that mattered

`gate-self-test-rule.md` requires a gate *modification* to re-demonstrate that
the gate still fires, and forbids reusing an old transcript. This change
modifies a gate, so the new test had to be shown capable of failing.

Removing the fix block and rerunning the suite:

```
failures:
    sole_gate_cannot_retire_itself_while_changing_its_surface_without_a_test_change
test result: FAILED. 10 passed; 1 failed
```

Exactly one test fails, and it is the new one. That establishes both halves:
the test genuinely catches the regression it names, and the fix breaks nothing
the previous ten tests covered. Mutation reverted; suite then green at 11/11.

The new fixture reduces the registry to the sole self-checking gate, which is
the production shape whose absence from the old two-gate fixture caused the
original miss.

## Verification at the corrected head

- `cargo test -p agov -p agov-validators`: all suites pass (11 gate-change,
  83 validator, 12/6/6/10/3/8/2 across the remaining binaries; 1 ignored
  network/platform test, by design).
- `agov gate change-check --base-ref origin/main`: `OK`, exit 0 — the gate
  passes on its own introducing branch.
- `AGOV_BIN=<exact release binary> sh tests/changed-files-failclosed.sh`:
  29 passed, 0 failed.
- **`jqad` bound still exact.** The `roi.rs` diff against `main` remains three
  added lines — `"gate"`, `"change-check"`, `"--registry"` — and
  `CLOSED_VALUE_FLAGS` is still `["--id-format", "--format"]`. The fix did not
  touch `roi.rs`, and no fourth entry appeared.
- Attack points 3, 4 and 5 unchanged: `.agf/gates.yaml` and
  `agf/dist/release-manifest.yaml` are byte-identical to the head I already
  cleared them at.

## Advisory, carried forward unchanged

`"change-check"` still sits under the `// plan subcommands` comment in
`roi.rs`. Flat slice, no functional effect, and correcting it would exceed the
`jqad` bound — so it should be left alone here and picked up whenever that
grouping is next edited for its own reasons.

## What I am not claiming

The disclosed proxy limit is untouched and I am not raising it: a single MR
can still weaken the checker by touching its declared test path with anything,
including an unrelated or empty change. The plan states this, adopter docs
state it, and `5231` accepted it. My finding was narrower — that the
self-retirement path required *no* test touch at all — and that specific gap
is what is now closed. A test touch is now required in every path I probed.

I also make no claim about CI job reachability or script execution. Nothing in
this branch's documentation claims otherwise.

## Authority

Advisory. Gate 2 is the governor-reviewer's decision on the exact head
`d03b2c36515cd9109f00dfd9b68779acdc441d9b`. This review supersedes `5231`'s
BLOCK for that head only; if the head moves again, this clearance lapses and
the review must be rerun.
