---
kind: decision-record
from: governor-reviewer
to: all
id: 20260722T164331Z-governor-reviewer-smmj
timestamp: 2026-07-22T16:43:31Z
model: human-governor
immutable: true
references-to:
  - 20260722T162730Z-governor-reviewer-ek13
  - 20260722T162730Z-governor-reviewer-ph8p
  - 20260722T163038Z-governor-reviewer-sjr8
  - 20260722T163910Z-contribution-auditor-r2hr
topic: v2-4-0-release-priority
requires-ack: false
artifacts_owned: []
---

# Governor decision: prioritize the v2.4.0 release

Release-critical v2.4.0 work is now the highest project priority. This
decision supersedes `ek13` only where it described CI deduplication as the
highest immediate priority; the CI assignment and `r2hr` findings remain
valid, but they must not delay the release.

Priority order:

1. Complete the already approved v2.4.0 workstreams, their exact-head Gate 2
   evidence, release-build verification, and governor decisions needed to
   reach a releasable state.
2. Continue the CI-deduplication plan/revision when it does not displace
   release-critical work. Do not edit `.gitlab-ci.yml` until the revised plan
   clears `r2hr` and receives Gate 1 approval.
3. Keep the dossier reference-layer implementation authorized by `sjr8`
   queued outside v2.4.0. It must not delay or alter the release.

Claude, acting as `coder-tester`, should resume the remaining approved
Workstream D Class 1 release work under MR !393 now. Claude, acting as
`build-engineer`, should prioritize v2.4.0 release-build and CI evidence needed
for the release; the deduplication plan remains next in that role's queue.

This priority decision does not itself authorize a version bump, tag, release
publication, deployment, or scope expansion. Those binding actions remain
subject to their existing governor gates and explicit release authorization.
