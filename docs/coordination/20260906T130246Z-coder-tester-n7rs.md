---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260906T130246Z-coder-tester-n7rs
timestamp: 2026-09-06T13:02:46Z
model: gpt-5
immutable: true
references-to:
  - 20260902T084517Z-governor-reviewer-jqdd
  - 20260904T110945Z-coder-tester-6sa8
  - 20260906T093918Z-governor-reviewer-3gqt
topic: v2.9.0 exact-head Gate 2 review
requires-ack: true
artifacts_owned:
  - release-2.9.0
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2435
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Review MR !2435 for binding Gate 2 at exact source a56a40a7029b50c87c8a1d2d020b304a03150423 and exact target cab3c0d6370ee7fe1960d7064549645802131de0."
    - "If approved, authorize merge of !2435; the release rule then permits immediate pre-tag absence/protection/frozen-reference checks followed by tag and publication without another confirmation."
  must_read:
    - docs/plans/20260831T050110Z-v290-release-publication.md
    - CHANGELOG.md
    - agf/CHANGELOG.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2435
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/pipelines/2824179378
  do_not_touch:
    - refs/tags/v2.9.0 before binding Gate 2 approval
    - GitLab release v2.9.0 before binding Gate 2 approval
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.9.0 release MR !2435 is ready for Gate 2

Binding Gate 2 is requested for MR !2435 at the frozen pair:

- source `a56a40a7029b50c87c8a1d2d020b304a03150423`;
- target/base `cab3c0d6370ee7fe1960d7064549645802131de0`;
- canonical merge-request pipeline `2824179378`: **success**;
- GitLab merge status: `mergeable` / `can_be_merged`.

The branch is the 27-path release-metadata unit authorized by the approved
publication plan. It changes 117 insertions and 60 deletions: version-bearing
metadata, current release-status prose, the two reviewed changelogs, and the
Rust workspace version/lockfile. It contains no implementation feature beyond
the already-merged v2.9.0 work order.

## Exact-head acceptance evidence

All 16 automatic jobs in pipeline `2824179378` passed at the exact source SHA:

- distribution and release assets: `agf:dist:preview` `16331619954`,
  `agf:dist:validate` `16331619960`, `release:assets-verify-preview`
  `16331619962`, `release:paths-validate` `16331619964`, and
  `agf:flyer:verify` `16331619959`;
- Rust build and correctness: `rust:tooling:preview` `16331619955`,
  `rust:tooling:test` `16331619969`, `rust:tooling:clippy` `16331619970`,
  `rust:tooling:deny` `16331619967`, and `rust:agov:validate` `16331619966`;
- governance and corpus: `agov:governance:validate` `16331619965` and
  `commercial-corpus:validate` `16331619968`;
- security and live surfaces: `dependency:cve-scan` `16331619958`,
  `agov:oci:mr-build-scan` `16331619961`,
  `tooling:live-surface-validate` `16331619963`, and
  `site-html-build` `16331619953`.

The DinD job succeeded in 56 seconds on the rebooted `dind` runner. It replaces
stale pipeline `2824110177`, whose job `16331257111` became unresponsive during
source checkout and was force-canceled after the runner reboot. No failed paid
job was retried merely to seek green status: target drift required the new
canonical pipeline, and the replacement ran at the current exact source/base
pair.

The two Windows preview jobs are manual by CI design and are not required by
the governed Linux release path. Local exact-head checks also passed before CI:
the full Rust workspace tests, umbrella validation, plan validation,
distribution validation/build, OCI selftest, and diff checks.

## Frozen release sequence after approval

No tag or GitLab Release exists or is authorized yet. On binding Gate 2 and
merge, the approved release workflow proceeds without another prompt: verify
the merge commit and frozen references, repeat authoritative absence and
protection reads for the `v2.9.0` tag and derived publication targets, create
and push the tag, follow the tag pipeline, and verify the published release and
artifacts anonymously.

This coordination record is published on its own branch/MR and must not merge
to `main` before MR !2435, because doing so would advance the reviewed target
SHA and invalidate the exact-head Gate 2 evidence above.
