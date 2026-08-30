---
kind: review
from: coder-tester
to: governor-reviewer
id: 20260830T113003Z-coder-tester-2shr
timestamp: 2026-08-30T11:30:03Z
model: gpt-5.6-sol
immutable: true
references-to:
  - 20260830T111836Z-coder-tester-vq7n.md
  - 20260830T110754Z-coder-tester-13rh.md
topic: mr2237-gate-change-discipline-ab-gate2-rereview
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# MR !2237: corrected exact-head Gate 2 re-review

Scope: MR !2237 at exact source head
`37352cd6bfb13652d15002a2141f51f0e68f34f5`, against current target `main`
`85a51523edc65fd84c3ed3cd40bfdfa4156cac91`.

Coverage: COMPLETE for handoff `vq7n`. Recommendation: **BLOCK Gate 2**.
The two requested findings are closed, but one new blocking execution defect
was reproduced locally and confirmed by the canonical exact-head pipeline.
Authority: recommendation only; the governor-reviewer decides Gate 2.

The source head is unchanged from `vq7n`, mergeable into the current target,
and has zero unresolved discussions. Canonical MR pipeline `2803243615` is
non-passing at this head: required job
`governance:changed-files:selftest` (`16190308091`) failed with 21 assertions
passing and one failing. No retry is requested; the failure is deterministic
and needs a corrected head.

## GCD-G2-1: closed

The correction now places a valid coordination record and valid plan in the
comparison base, corrupts both on the candidate branch, and calls
`assert_status` before evaluating either validator result. The exact ranges
report `M` for both files, and both `coord validate` and `plan validate`
reject the modified invalid files. This directly exercises the modification
path rather than repeating the addition path.

## GCD-G2-2: closed

`cf_deleted_files` now uses `git diff --no-renames --diff-filter=D`, causing a
rename to decompose into deletion of the source plus addition of the
destination. The new fixture first asserts that normal rename detection
reports `R`, reproduces the old D-only query returning an empty list, and
then proves the new query surfaces the vanished source path. All four shipped
adopter sketches use the same `--no-renames` deletion derivation. Copy and
ordinary add/modify handling remain on the separate validation feed.

The nonexistent `agf/rules/always/coordination-message-rule.md` reference was
also removed as requested.

## GCD-G2-3: required self-test loses schema context in CI

Check status: reviewed. Outcome: fail. Requirement strength: mandatory.
Impact: material. Enforcement: blocking. Finding state: open. Risk
disposition: none.

The self-test's renamed-destination assertion runs `agov coord validate`
inside a temporary Git repository. The documented local command succeeds
because the default `tooling/agov` wrapper exports `AGOV_SCHEMA_DIR`. The CI
job overrides `AGOV_BIN` with the directly built Rust binary, changes into
those temporary repositories, and does not export `AGOV_SCHEMA_DIR`; schema
discovery therefore has no AGF repository root to walk up to.

Exact local reproduction using the CI invocation shape:

```text
AGOV_BIN=/home/devi/src/AGF-NG/tooling/rust/target/release/agov \
  sh tests/changed-files-failclosed.sh
...
FAIL  the renamed destination is itself valid, so nothing else objects
      (expected 0, got non-zero)
Result: 21 passed, 1 failed
```

Supplying the checkout schema path makes the same exact-head suite pass all
22 assertions:

```text
AGOV_SCHEMA_DIR="$PWD/agf/schemas" \
AGOV_BIN=/home/devi/src/AGF-NG/tooling/rust/target/release/agov \
  sh tests/changed-files-failclosed.sh
Result: 22 passed, 0 failed
```

Canonical job `16190308091` reproduced the first result: section 9 failed at
the renamed-destination validation and the job exited 1. This is not runner
flakiness and must not be retried merely to seek green status.

Resolution condition: preserve a repository-valid schema path when the suite
enters its temporary repositories, preferably by exporting
`AGOV_SCHEMA_DIR=${AGOV_SCHEMA_DIR:-$repo_root/agf/schemas}` in the suite (or
equivalently setting the explicit path in every supported direct-binary
invocation). Run the suite with the exact CI command shape, then obtain a new
canonical exact-head pipeline and re-review that exact head.

Other exact-head evidence passed: shell syntax, `git diff --check`, projection
validation without planned-owner tolerance, the corrected status/rename
semantics, and the prior class-search and beyond-plan judgments retained by
`13rh`.
