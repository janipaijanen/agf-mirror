---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260815T181933Z-governor-reviewer-5g84
timestamp: 2026-08-15T18:19:33Z
model: human
immutable: true
references-to:
  - 20260815T173111Z-governor-reviewer-kpkk.md
  - 20260815T174820Z-governor-reviewer-jj5s.md
topic: v260-construction-authorized
requires-ack: false
artifacts_owned: []
---

# Authorize the approved v2.6.0 construction through Gate 2 evidence

Jani approved the proposal and directed the coder-tester to proceed. This
binding decision authorizes implementation of the already Gate-1-approved plan
`docs/plans/20260813T051840Z-v260-release-publication.md` at MR !1492 head
`bc8a975e3f73b50e72db100d227752a7ed4da094`, subject to the following exact
boundary:

1. Create `reconstruct/v2.6` and `release-v2.6.0` from peeled annotated
   `v2.5.0` base `f1932a1a486b772ca4353c7fa71c2279132fe0dc`, with the
   branch protections and evidence required by the approved plan.
2. On the source branch, apply exact allowlist commits
   `3456c1fac7ee3112674fd2b4ebfd884e56501896` and
   `bcb0fadd` using the approved conflict-resolution procedure, followed only
   by the plan's bounded release-only metadata change set.
3. Open the non-squash, non-rebase release MR from `release-v2.6.0` to
   `reconstruct/v2.6`, run the complete pre-Gate-2 validation, and publish the
   exact-head evidence and coder-tester Gate 2 handoff.

The effective allowlist includes exact `3456c1fa` under the post-test product
decision in `jj5s`: `aarch64-unknown-linux-musl` is not a mandatory v2.6.0
release target even though the bounded necessity job succeeded. The test result
must remain reported accurately.

This authorization stops at presentation for Gate 2. It does not approve Gate
2, merge the release MR or MR !1492, create or publish `v2.6.0`, create a tag or
GitLab Release, upload packages or OCI artifacts, deploy, close issues, or
forward-sync release metadata to `main`. Each remains behind the approved
plan's later explicit governor decision and exact-state checks.
