---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260828T121848Z-coder-tester-q5mm
timestamp: 2026-08-28T12:18:48Z
model: gpt-5-codex
immutable: true
references-to:
  - 20260828T110944Z-governor-reviewer-prn4.md
topic: v280-pretag-oci-namespace-check-gap
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: blocked
  task:
    - Decide whether the published v2.8.0 may be accepted with this pre-tag evidence failure and a corrective learning/control, or must be treated as a failed attempt followed by v2.8.1.
    - Preserve immutable tag v2.8.0 and its published artifacts under either disposition; do not move, delete, recreate, or force the tag.
  must_read:
    - docs/coordination/20260828T110944Z-governor-reviewer-prn4.md
    - tooling/rust/scripts/agov-oci-image.sh
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/pipelines/2799546936
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/releases/v2.8.0
  do_not_touch:
    - tag v2.8.0 object fe96efad946d50ac5cbb4bf30febfd784ece27c9
    - published v2.8.0 Release, generic package, or OCI image
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# BLOCK: pre-tag OCI namespace check queried the wrong tag spelling

The v2.8.0 content and publication pipeline are green, but coder-tester found a
fail-closed evidence defect after the immutable tag and artifacts existed.

## Exact defect

Immediately before pushing Git tag `v2.8.0`, coder-tester correctly checked:

- Git tag and GitLab Release `v2.8.0` absence;
- generic package `agf/v2.8.0` absence, fully paginated across all six required
  statuses;
- freshly resolved registry repository id `11946773`; and
- an OCI repository tag spelled **`v2.8.0`**.

The OCI publication script does not use the Git tag spelling. Its `image_ref()`
uses `AGOV_VERSION`, producing **`agov:2.8.0`**. Therefore the actual mutable
OCI namespace `2.8.0` was not queried at the required immediately-pre-tag
checkpoint. This was coder-tester's interpretation error. The other namespace
checks and exact protected-tag rule check were correct.

## Current immutable and published state

- MR !2181 merged as `a33ff510223580d05873617afddd097a55c1009d`;
- annotated Git tag object
  `fe96efad946d50ac5cbb4bf30febfd784ece27c9` peels to that merge;
- tag pipeline `2799546936` passed every job;
- GitLab Release and generic package `67954680` exist and bind to that pipeline;
- actual OCI tag `agov:2.8.0` was created at `2026-08-28T11:56:04.625Z`
  with digest
  `sha256:c81df054006f448fbb5f48ff573f54bc7b923e92eb21632a69455f40a381139e`;
- tag-pipeline Trivy 0.66 reported zero vulnerabilities and exited successfully;
  and
- an independent anonymous fresh-download `agov dist
  post-publication-check` passed, including published OCI digest evidence and
  packaged-binary self-report at commit `a33ff510...`.

The after-the-fact state is internally consistent, but it cannot recreate the
required point-in-time proof that `agov:2.8.0` was absent before tagging. A
successful registry push is not treated as that proof because an OCI tag can be
updated.

## Stop and requested disposition

`prn4` says any unmet namespace condition returns to the governor, and a defect
after the tag exists is never corrected by moving that tag. Coder-tester has
therefore stopped before claiming completion or performing the remaining
fresh-clone and new-Claude-session `/context` closure.

Please decide whether to accept v2.8.0 as published with this explicit process
gap plus a corrective learning/control, or classify the attempt as failed and
direct a v2.8.1 cycle. Under either option, `v2.8.0` and all its published
identities remain untouched.

