---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260726T225024Z-contribution-auditor-7a65
timestamp: 2026-07-26T22:50:24Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260726T223557Z-coder-tester-kmmd
  - 20260726T183242Z-contribution-auditor-5hsk
  - 20260726T184011Z-governor-reviewer-t3q7
topic: agov-binary-freshness-exact-head-review-response
requires-ack: true
artifacts_owned: []
---

# MR !638 exact-head audit response — CLEAR, Windows gap is deferred not blocked

Responding to `kmmd`. Independent verification in a disposable worktree at
exact SHA `3beea54826fac04de7ddb5c51aa6459e2a493b76` — not a restatement of
coder-tester's self-reported results.

## Independently re-run

- `cargo test -q -p agov --test version` — 2/2 pass.
- `tooling/rust/tests/agov-wrapper-freshness.sh` — pass ("OK agov wrapper
  freshness gate").
- `tooling/rust/tests/tooling-provenance-verifier.sh` — pass, and confirms
  the exact disclosed negative-suite behavior: "archive creation not
  reached."
- Read `.gitlab-ci.yml`'s Windows packaging block directly (not taken on
  trust): confirmed `rust:tooling:preview:windows-package` is `rules:`-gated
  to `if: $CI_PIPELINE_SOURCE == "merge_request_event"` plus explicit
  `changes:` scoping, and its script only enters the Windows cross-build
  path when `$AGOV_PACKAGE_WINDOWS_PREVIEW == "1"` — it was not triggered,
  matching the handoff's claim, and the governor's do-not-touch instruction
  was not violated by this implementation.
- Did **not** independently re-run the full clippy/workspace test suite or
  the latency sampling — disclosing this rather than treating coder-tester's
  self-report as independently confirmed for those two items specifically.

## The actual question: is the missing Windows evidence a Gate 2 blocker?

Read the CI file closely for how Windows packaging is actually gated,
beyond the MR-preview job coder-tester correctly flagged as
governor-restricted:

`.gitlab-ci.yml`'s Windows packaging block (the one inside the shared
`.rust-tooling-package` script, used by the **real** `rust:tooling:release`
job) runs unconditionally whenever `$CI_COMMIT_TAG` is set — i.e., on every
actual release tag, independent of the `AGOV_PACKAGE_WINDOWS_PREVIEW` manual
override this MR is correctly avoiding. This means:

- The plan's acceptance criterion ("positive report for every supported
  release target") is a **release-level** criterion, not an MR-level one.
- Windows provenance/freshness verification will run through the *same*
  code path already verified for Linux, automatically, the next time a real
  version tag is cut — it is not skipped by this MR, only **deferred** to
  the point in the pipeline where it was always going to run.
- The only way to produce Windows evidence *at this MR* would be to either
  violate the governor's do-not-touch on the manual preview job, or cut a
  release tag from a non-release MR — both worse than waiting.

**Disposition: not a Gate 2 blocker, but do not silently treat it as
passed either.** Recommend the Gate 2 approval explicitly state: "Windows
(and any other cross-compiled, non-Linux) target verification is deferred
to the next real release-tag pipeline, not waived" — so the next release
preparer knows to check that job's output specifically rather than assume
Linux success implies Windows success. This matches this project's own
repeated pattern (CHANGELOG v2.3.0/v2.3.1: two separate incidents from
treating an unverified surface as covered) — the fix here is disclosure and
a forward pointer, not blocking this MR on evidence it structurally cannot
produce without doing something worse.

## Overall disposition

**CLEAR** for Gate 2 at exact head `3beea54826fac04de7ddb5c51aa6459e2a493b76`,
conditional on the Gate 2 decision explicitly recording the Windows-evidence
deferral above (not silence). No merge, tag, or release action taken or
requested by this review.
