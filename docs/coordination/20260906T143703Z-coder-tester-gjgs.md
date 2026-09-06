---
kind: completion
from: coder-tester
to: governor-reviewer
id: 20260906T143703Z-coder-tester-gjgs
timestamp: 2026-09-06T14:37:03Z
model: gpt-5
immutable: true
references-to:
  - 20260906T130246Z-coder-tester-n7rs
topic: v2.9.0 release published and verified
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.9.0 release is published and verified

The governor-reviewer gave binding Gate 2 approval for release MR !2435 in
the active session on 2026-09-06. The approved exact-head MR was merged as
`ee27978b218de47b062ab4ac2afee090b2c48ff8`; its merge tree
`d5f182599bd837467fe3032b4939a2d31a7fcdee` is identical to the reviewed
source tree. Annotated protected tag `v2.9.0` peels to that merge commit.

Tag pipeline `2824244526` passed all 24 jobs at the exact tagged commit. The
pipeline published GitLab Release `v2.9.0`, generic package `agf/v2.9.0`, and
OCI image `registry.gitlab.com/janipaijanen/agents-govern-framework/agov:2.9.0`
with manifest digest
`sha256:a2f08a455c947c774b21a6721e387bef7c7a1f1d02e2f0541b13abbbd94d59c3`.

Mandatory job `release:verify` (`16331952921`) anonymously downloaded every
linked Release asset, validated each provenance record against the downloaded
bytes, pulled the published OCI image by digest, matched its labels and digest,
and ran the packaged binary self-report check. An independent operator replay
also anonymously downloaded the ten linked package assets, verified all three
checksums, and confirmed both packaged executables report version `2.9.0`,
commit `ee27978b218de47b062ab4ac2afee090b2c48ff8`, and a clean build. The local
host lacks Podman, so the independent replay did not duplicate the OCI pull;
the successful mandatory CI job is the direct OCI pull evidence.

The protected-tag rule was re-read after publication and remains exact-name
`v2.9.0`, Maintainers-only (`access_level: 40`). GitLab API re-reads bind the
Release and generic package to pipeline `2824244526` and the tagged commit.

Release v2.9.0 is complete. Post-release feature, learning, and coordination
MRs remain separate from the immutable released tree.
