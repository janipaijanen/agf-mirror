---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260828T040852Z-governor-reviewer-s3tg
timestamp: 2026-08-28T04:08:52Z
model: human-directed-gpt-5
immutable: true
references-to:
  - 20260827T192914Z-governor-reviewer-hsj8
  - 20260827T210552Z-governor-reviewer-2h2s
  - 20260827T212604Z-coder-tester-mrrq
topic: v270-rc1-exact-head-gate2-approved
requires-ack: false
artifacts_owned:
  - release-v2.7.0-rc.1
  - reconstruct/v2.7
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2163
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Binding Gate 2 approval: v2.7.0-rc.1 exact head

This record transcribes the governor-reviewer's in-session decision,
"gate 2 approved", given in response to exact-head handoff `mrrq`.

## Decision

MR !2163 receives binding Gate 2 approval at exactly:

- source `release-v2.7.0-rc.1`:
  `c39f9bccb2432a13ca2960049ce3c414fe96e56f`;
- source tree: `7d94b6c327ff837aa3f33aa072068c4335e80ddf`;
- target `reconstruct/v2.7`:
  `f780ac0ea37061e0ad3ee6fb1f00c6958b996fc0`;
- exact-head pipeline `2797915827`: success, 20 required jobs passed,
  zero failures, two optional Windows preview jobs manual;
- zero unresolved discussions and an empty `closes_issues` response.

The coder-tester is authorized to perform the plan-prescribed merge of that
exact head into that exact target through the GitLab merge API with
`squash=false` and source-branch retention. Immediately before merging,
re-read both branch heads, both protection objects, the MR head and target,
and the exact-head pipeline. Pass the exact approved source SHA to the merge
API. Immediately afterward, verify first parent = reviewed target, second
parent = reviewed source, merge tree = reviewed source tree, source branch
retained, and target/source protection unchanged.

Any head, target, pipeline, protection, discussion, or mergeability drift
invalidates this approval and requires a return to the governor before merge.

## Not authorized

This decision does not authorize creating the `v2.7.0-rc.1` tag, publishing a
GitLab Release, generic package, or OCI image, starting the 48-hour hold, or
promoting stable `v2.7.0`. Those remain separate governed steps after the
merge proof is recorded.
