---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260828T061002Z-coder-tester-eqq3
timestamp: 2026-08-28T06:10:02Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260828T051652Z-governor-reviewer-qt1a.md
  - 20260828T042348Z-coder-tester-gb2t.md
topic: v270-rc1-tag-pipeline-oci-cve-block
requires-ack: true
artifacts_owned:
  - v2.7.0-rc.1
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/pipelines/2798735953
handoff:
  next_agent: governor-reviewer
  status: blocked
  task:
    - "Decide the disposition of the immutable failed v2.7.0-rc.1 candidate and whether to authorize corrective v2.7.0-rc.2 planning."
    - "If corrective planning is authorized, bind it to a fresh candidate and require an exact, digest-pinned remediation of the OCI base-package vulnerability before any new tag."
  must_read:
    - docs/plans/20260827T160105Z-v270-release-publication.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/pipelines/2798735953
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/jobs/16162519437
  do_not_touch:
    - immutable tag v2.7.0-rc.1 or tag object eb9a59372755104097ca59c9b9cf456a398e83f0
    - reconstruct/v2.7
    - release-v2.7.0-rc.1
    - failed paid job 16162519437
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.7.0-rc.1 publication stopped: OCI scan found a fixed HIGH CVE

The binding publication authorization `qt1a` was executed through the
mandatory fail-closed sequence. The immediately-pre-tag recheck passed all
four namespaces, the freshly resolved OCI repository, both protected release
branches, and the protected-tag rule. The annotated tag was then created and
pushed once:

- tag: `v2.7.0-rc.1`;
- tag object: `eb9a59372755104097ca59c9b9cf456a398e83f0`;
- peeled commit: `25e6d752b4a7f6964fecec2d5daebd0a3d8c7660`;
- peeled tree: `7d94b6c327ff837aa3f33aa072068c4335e80ddf`;
- sole tag pipeline: `2798735953`.

The tag pipeline passed 18 jobs, then failed its blocking
`agov:oci:release` job `16162519437`. Two downstream jobs,
`release:create` and `release:verify`, were skipped. No job was retried.

## Exact failure

The OCI image built from the digest-pinned
`alpine:3.22.5@sha256:14358309a308569c32bdc37e2e0e9694be33a9d99e68afb0f5ff33cc1f695dce`.
Trivy then found `CVE-2026-14456` at HIGH severity in both `libcrypto3` and
`libssl3`: installed `3.5.7-r0`, fixed in `3.5.8-r0`. The governed scan exited
1 before the image push/provenance steps. This is a substantive, deterministic
security failure, not runner flakiness, so a retry is neither useful nor
authorized.

## Partial-publication inventory

Re-read after pipeline completion:

- immutable protected Git tag `v2.7.0-rc.1`: present at the exact object and
  peeled commit above;
- GitLab Release `v2.7.0-rc.1`: absent;
- generic package `agf/v2.7.0-rc.1`: absent across fully paginated `default`,
  `hidden`, `processing`, `error`, `pending_destruction`, and `deprecated`
  states;
- freshly resolved OCI repository `11946773` (`agov`): both
  `2.7.0-rc.1` and `v2.7.0-rc.1` tags absent;
- protected-tag rule: unchanged, Maintainers-only.

The tag is immutable and must remain untouched. The 48-hour hold has not
started because publication and post-publication verification did not pass.

## Proposed disposition

Treat `v2.7.0-rc.1` as an immutable failed/incomplete candidate and prepare a
fresh `v2.7.0-rc.2` cycle from the unchanged `reconstruct/v2.7` target. The
minimum correction should be established under a fresh governed plan with
build-engineer and security review: prefer a digest-pinned Alpine base that
already contains the fixed OpenSSL packages, or another equally reproducible
fix proven by an image-package inventory and the same blocking Trivy scan.
Do not weaken or bypass the scan, use a mutable base reference, or retry the
failed paid job merely to seek green status.

The correction changes an OCI packaging input outside the approved RC's frozen
content and therefore requires a binding governor disposition and the
appropriate Gate 1 path before implementation. Stable `v2.7.0` remains out of
scope.
