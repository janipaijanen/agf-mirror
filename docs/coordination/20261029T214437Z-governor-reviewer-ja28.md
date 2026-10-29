---
kind: decision-record
from: governor-reviewer
to: all
id: 20261029T214437Z-governor-reviewer-ja28
timestamp: 2026-10-29T21:44:37Z
model: codex
immutable: true
references-to:
  - 20261028T163912Z-governor-reviewer-ss2k.md
  - 20261029T205646Z-governor-reviewer-pb4a.md
  - 20261029T212521Z-coder-tester-mnk2.md
topic: v213-apk-residual-risk-disposition
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Accept the bounded mutable-APK residual for v2.13

Under the active ss2k governor delegation, accept the operational residual
specified in the reviewed report
`docs/reviews/next-release-freeze-evidence/20261029-v213-apk-residual-risk.md`
at MR !3378 candidate a16a93b837cb963e8fd117c6413bafae0544b732.
Report SHA-256:
`c8896c0213c33645051cae8f3375b13073758c02600434b0c41a39859f4cdccb`.
Its content is byte-identical to the independently cleared c29a9abc report.

This accepts mutable APK repository resolution, installed tool/CA version drift,
repository/registry availability risk and absence of bit-for-bit full-job
reproducibility for the planned v2.13 publication only. It applies to the reviewed
Alpine 3.24.3 linux/amd64 base manifest
`sha256:c26612752c78b18faacd63c7d50f4592c2e7b0f25e0e20e82bd74e465c57ab0d`
and the reviewed curl/jq/unzip then tar/gzip installation and create-only client.
Package version drift is the disclosed residual; replacing the base, package
set, client behavior or verification controls requires renewed assessment.

Rationale: the immutable base and actual installed runtime were replayed without
credentials/project mounts and independently reviewed. Current Trivy 0.76.1,
with database updated 2026-10-29T13:09:46Z, found zero HIGH/CRITICAL findings in
16 base and 34 installed-runtime packages with no scanner warnings/errors.
Evidence hashes and coverage limits are in the report and !3358 note 4119889717.
Publisher exact-head CI passed all 23 jobs, including the real job-token GET
probe. The final freeze's current CI remains a separate acceptance condition.
Pinning APK packages is explicitly excluded from this release scope; accepting
this bounded residual does not reopen a custom builder-image project.

This is not a waiver of a known vulnerability, acceptance of the removed
release-cli findings, a guarantee about future package resolution, or proof
of production POST success. Any new known vulnerability finding requires
remediation or its own explicit governor disposition before freeze/publication
can proceed. The report's HIGH/CRITICAL/database/time limits remain intact.

Preserve artifact/provenance verification, one ref-free POST, no overwrite,
delete, redirect following or POST retry, secret handling, exact tag/Release
identity postconditions and operator reconciliation of ambiguous outcomes.
Protected, operationally immutable tagging remains a separate publication
prerequisite: the client checks commit identity, not `.protected`, and its
checks do not make tag/POST operations atomic. Rollback uses only separately
reviewed verified inputs or suspends publication; never restore `latest` or
the obsolete publisher, and never rewrite published tags/artifacts/Release.

Validity is limited to v2.13 and this delegated window through
2026-10-30T18:00:00Z. Abandonment, a material publisher change or an unfinished
cut at expiry requires a new governor disposition. It is not reusable approval
for another release. The later publication plan must carry these conditions.

This decision supplies only the requested risk disposition. It does not approve
any MR merge, release freeze, version/provenance/status edit, tag, package/OCI
upload, GitLab Release, deployment, new session/timer or delegation extension.
All ordinary exact-head CI, independent review and binding gate requirements
remain. Because this acceptance has continuing conditions until publication or
expiry, do not treat its carrier as exhausted merely because this record exists;
use a separate binding merge decision for the carrier.
