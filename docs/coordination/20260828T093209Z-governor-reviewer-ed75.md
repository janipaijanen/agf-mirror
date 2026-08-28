---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260828T093209Z-governor-reviewer-ed75
timestamp: 2026-08-28T09:32:09Z
model: human-directed-claude-sonnet-5
immutable: true
references-to:
  - 20260828T092331Z-coder-tester-nrdg.md
  - 20260828T085508Z-governor-reviewer-j37k.md
  - 20260828T085514Z-planner-architect-rmna.md
topic: v280-exact-head-gate2-approved
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Binding Gate 2 approval: v2.8.0 exact head

This record transcribes the governor-reviewer's in-session decision,
"gate 2 approved", given in response to coder-tester's exact-head
candidate report `nrdg`.

## Approved

MR !2181 receives binding Gate 2 approval at exactly:

- source `release-2.8.0`: `a8c8e407c3cb9c0254f56054282671cb4d43056f`
- source tree: `601a9d70d9cc1a7555e608485561fdea440e8c99`
- target `main`: `ca8086077c216d4dc9f6940f0d2c58f58065f010`
- canonical project: `janipaijanen/agents-govern-framework`

**Conditional on the exact-head pipeline passing.** At the time of this
decision, MR !2181's head pipeline `2799215081` had failed with every job
reporting `failure_reason: ci_quota_exceeded` and `started_at: null` — no
job executed and no runner time was consumed. CI quota has since been
topped up and that pipeline was retried at the same SHA
`a8c8e407c3cb…`. Retrying it is not "retrying a failed paid job to seek
green status": the jobs never ran, the failure was diagnosed as
project-level quota state, and the candidate content is unchanged.

If the retried pipeline fails on substance, this approval lapses and the
work returns to the governor. If the head SHA changes for any reason,
this approval is void and a fresh exact-head decision is required.

## Evidence accepted

Coder-tester's local exact-head evidence in `nrdg`:

- full locked Rust workspace tests passed;
- `agov validate`, distribution validation and build, and release-script
  self-tests passed (pre-existing advisories only);
- exact musl binaries report `v2.8.0`, full source SHA, clean and
  source-clean;
- **mandatory local OCI build** succeeded with **MIT labels**, exact
  revision, **OpenSSL `3.5.8-r0`**, image digest
  `sha256:83cab93e72d0ea653f89e0953313684dbac3da184ed73f43ab7f942d73bac252`;
- **digest-pinned Trivy 0.66 HIGH/CRITICAL scan: zero findings, exit 0.**

That last pair is the decisive evidence. It confirms both defects that
defeated the abandoned reconstruction are absent here: `CVE-2026-14456`
is remediated (`3.5.8-r0`, versus the `3.5.7-r0` that stopped
`v2.7.0-rc.1`), and the tooling licence metadata is **MIT**, matching
binding decision `7b9b69c1` and root `LICENCE`, versus the frozen-out
`CC-BY-SA-4.0` values that blocked `v2.7.0-rc.2`.

The mandatory pre-tag OCI build and scan required by `j37k` is therefore
satisfied, performed before anything immutable exists — which is exactly
the control that was missing when `-rc.1` was tagged.

## Authorized action

Merge the exact approved source head into `main` through the platform
merge API with `sha` set to `a8c8e407c3cb9c0254f56054282671cb4d43056f`
and `squash=false`. Immediately before merging, re-read both branch heads
and the MR's head pipeline. Immediately after, verify the merge commit's
parents and resulting tree against the reviewed values.

## Not authorized by this record

**Publication is a separate decision and is not granted here.** No tag,
GitLab Release, generic package, or OCI publication may be created under
this approval. `qhk8` remains suspended per `j37k` and was
`v2.7.0-rc.2`-scoped in any case; it confers nothing on `v2.8.0`.

Present the post-merge proof and request publication authorization for
`v2.8.0` specifically.

No change to the immutable `v2.7.0-rc.1` tag. `reconstruct/v2.7` and
`release-v2.7.0-rc.2` remain retained, unmerged and untagged.
