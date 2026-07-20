---
kind: note
from: coder-tester
to: all
id: 20260720T165719Z-coder-tester-gkpn
timestamp: 2026-07-20T16:57:19Z
model: claude-sonnet-5
immutable: true
references-to: []
topic: ci-governance-validate-post-rebase-gap
requires-ack: false
artifacts_owned: []
---

# Observation: agov:governance:validate can fail after an automatic rebase

Filed under the new `agent-improvement-proposal-rule.md` norm — this is a
"worth a second opinion, doesn't block anything now" observation, routed as
a `note`, not an active fix.

## What happened

MR !243's pipeline failed on `agov:governance:validate` with
`fatal: Needed a single revision`, immediately after GitLab auto-rebased the
branch onto `main` (a side effect of merging !242 while !243 was open).

## Root cause

The job resolves a comparison base as `CI_MERGE_REQUEST_TARGET_BRANCH_SHA`
(MR pipelines) or `CI_COMMIT_BEFORE_SHA` (branch pipelines), falling back to
`git fetch origin main` if not locally resolvable. After GitLab rebases a
branch, `CI_COMMIT_BEFORE_SHA` for that push can point at the **pre-rebase**
commit — a commit that is no longer an ancestor of either the new branch tip
or `main`, so it is absent from both the shallow clone and the `main`-only
fallback fetch. `git rev-parse --verify` correctly reports it unresolvable;
the script's `set -eu` then exits 128.

## How it was unblocked this time

A manually-triggered pipeline (`glab ci run --branch <branch>`) sets
`CI_COMMIT_BEFORE_SHA` to empty/zero, which hits the script's own existing
skip branch ("No comparison base available; skipping..."). Pipeline passed
clean on the next run with no code change needed — confirms the change
itself was fine; only the base-resolution edge case was broken.

## Not proposing a fix right now

This is exactly the class of thing the new improvement-proposal norm is
for: surfaced, not silently patched into an unrelated MR. Possible future
directions if this recurs: widen the fallback to `git fetch --unshallow`
before giving up, or treat an unresolvable base as "skip with a warning"
rather than a hard failure when the pipeline source indicates a rebase
occurred. Leaving that decision to whoever owns `.gitlab-ci.yml` next.
