---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20261029T205646Z-governor-reviewer-pb4a
timestamp: 2026-10-29T20:56:46Z
model: codex
immutable: true
references-to:
  - 20261028T163912Z-governor-reviewer-ss2k.md
  - 20261029T193807Z-governor-reviewer-qmba.md
  - 20261029T201029Z-coder-tester-bskn.md
  - 20261029T194912Z-governor-reviewer-nskj.md
topic: ci-pins-publisher-gate2
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2: merge CI pins and the replacement release publisher

Under ss2k's active governor delegation, approve MR !3358 at exact head
fb338ba09f4bb61734034f734e1d9f8f7c3b6e56, without squash, into main on
https://gitlab.com/janipaijanen/agents-govern-framework.git.

Canonical MR pipeline 3005846742 succeeded with all 23 jobs successful.
Rust job 17589585071 passed the actual same-project job-token read-only probe at
2026-10-29T20:54:28.647668Z, covering protected existing tag, Releases collection
and exact existing Release with consistent tag SHA. It finished successfully at
20:55:33Z. Retained trace SHA-256:
de1f01cab60e7a792d079194aadbef3288ad5e48a4b2cf83609ff9e19a06a470.
OCI job 17589585072 also succeeded. No production write was used as a test.

Independent implementation/InfoSec review cleared source 461ed65e and final
handoff candidate 41efc3bd; independent integration review cleared fb338ba0's
single duplicate-dependency removal with code/security/test bytes unchanged.
The governor reviewed the source, test corrections, CI wiring and artifact path.
The client accepts one JSON document, explicit missing-Release response shapes
only after positive tag/collection checks, exact payload/link verification, one
ref-free POST, no redirects/update/delete/retry, and token input through stdin.
Ambiguous POST outcomes stop for reconciliation; artifact and provenance checks
and the package namespace remain intact. The new pin-drift check is advisory.
Rust 1.97.1 and checksummed rustup-init 1.29.2 replace floating install inputs.

Official Alpine 3.24.3 linux/amd64 manifest
sha256:c26612752c78b18faacd63c7d50f4592c2e7b0f25e0e20e82bd74e465c57ab0d
has verified provenance and credential-free runtime replay. Fresh Trivy 0.76.1
with 2026-10-29 database reports zero HIGH/CRITICAL findings for 16 base and 34
installed-runtime packages, with no warnings/errors. Evidence and hashes are
published in !3358 note 4119889717 and its attached report; independent evidence
review is CLEAR. These bounded observations do not establish vulnerability freedom
or reproducible mutable APK repositories. Old release-cli findings are neither
waived nor suppressed. The freeze child must refresh durable APK residual evidence
and obtain an explicit governor disposition before release freeze completion.

Freshly observed: correct source/target, mergeable, resolved discussions, zero
remaining forge approvals. On current main 2a99009d, actual merge tree
0fc78fcf68413fc1522545f3965aa855cb246db4 contains the ten reviewed paths and
preserves the merged profiling lane. A three-dot diff has multiple merge bases;
use actual merge-tree comparison. Immediately before merge recheck exact head,
current target, successful exact-head CI and discussions. Publish this unchanged
decision on the shared branch before merging; then verify parents and actual delta.

This decision authorizes only this implementation merge and its checks. It grants
no plan-MR merge, freeze completion, APK residual acceptance, version/tag/Release
publication, deployment, additional role authority or delegation extension.
