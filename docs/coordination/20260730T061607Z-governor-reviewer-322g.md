---
kind: handoff
from: governor-reviewer
to: build-engineer
id: 20260730T061607Z-governor-reviewer-322g
timestamp: 2026-07-30T06:16:07Z
model: human
immutable: true
references-to:
  - 20260730T061607Z-governor-reviewer-q6hj
  - 20260729T191059Z-planner-architect-eqh8
  - 20260729T183734Z-contribution-auditor-b16q
  - 20260729T175843Z-governor-reviewer-m15q
topic: mr792-post-merge-first-push-monitoring
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: build-engineer
  status: ready
  task:
    - Observe and record the exact coordination-only first-push job set after MR !792 is on main.
    - Observe and record the exact Rust-source-only first-push job set after MR !792 is on main.
    - Publish a completion record only when both assertions meet the closure condition below.
  must_read:
    - 20260730T061607Z-governor-reviewer-q6hj
    - 20260729T183734Z-contribution-auditor-b16q
    - docs/plans/first-push-ci-changes-fallback.md
  do_not_touch:
    - .gitlab-ci.yml
    - agf/VERSION
    - agf/CHANGELOG.md
---

# Tracker: MR !792 post-merge first-push monitoring

Owner: `build-engineer`

State: OPEN; becomes executable only after MR !792 is merged to `main` at its
governor-approved exact head. This tracker exists before that merge as
required by `q6hj` and the contribution-auditor safeguard in `eskm`.

## Assertion A — coordination-only new branch

From a current `main` that contains MR !792, create a disposable new branch
whose first commit touches only `docs/coordination/*.md`. Push it before
opening an MR so the relevant pipeline source is `push` and its
`before_sha` is all zero.

Expected complete job set:

- `agov:governance:validate`

No other job may be present. This proves the new `compare_to` rule selects
the one matching path set and the terminal `when: never` guard prevents all
non-matching jobs from falling through to the legacy zero-`before_sha`
behavior.

## Assertion B — Rust-source-only new branch

From the same qualifying `main`, create a separate disposable new branch
whose first commit touches only
`tooling/rust/crates/agov/src/main.rs` (a non-functional comment is the
approved fixture). Push it before opening an MR.

Expected complete job set, exactly these four:

- `agov:governance:validate`
- `rust:tooling:deny`
- `rust:tooling:test`
- `rust:tooling:clippy`

In particular, `rust:agov:validate`, `dependency:cve-scan`,
`site-html-build`, `agf:flyer:verify`, `agf:dist:validate`, and the three
`release:*-selftest` jobs must not be present. This records both the
must-run and must-not-run halves of the approved assertion.

## Evidence fields required for each assertion

- MR !792 merge commit and qualifying `main` SHA used as the branch base;
- disposable branch name and first commit SHA;
- pipeline ID, URL, source, ref, all-zero `before_sha`, status, and creation
  timestamp;
- complete observed job-name list with each job's final status;
- explicit expected-versus-observed comparison and any deviation;
- branch-deletion confirmation after evidence capture.

Record evidence in an immutable coordination completion or review entry and
bridge it to MR !792. Pipeline history remains the permanent runtime source
after disposable branches are deleted.

## Closure condition and sequencing

Close this tracker only when both first-push pipelines complete successfully
and their complete observed job sets exactly equal the sets above. Any extra,
missing, failed, or unexpectedly skipped job keeps the tracker open and must
be routed as a new finding; do not silently accept a near match.

Capture both fixtures against `main` after MR !792 and before a later
`.gitlab-ci.yml` change is merged, so the observations isolate the approved
first-push fix. This tracker does not block MR !792's Gate 2 decision or
merge—the governor explicitly converted the impossible pre-merge assertions
to post-merge monitoring—but it must remain visible and owned until its
closure condition is met.
