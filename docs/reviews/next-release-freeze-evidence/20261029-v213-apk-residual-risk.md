<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.13 publisher: mutable APK residual risk — 2026-10-29

Status: evidence prepared for explicit governor disposition at freeze Gate 2.
This report does not accept risk or authorize publication by itself.

Combined implementation baseline: `32e384e74959d456acf85435fcd7d7c7194d8cad`.
Publisher !3358 merged as `c7ea3cfb6585789395ee8eaeac2d98ffa0bd43be`, from
`fb338ba09f4bb61734034f734e1d9f8f7c3b6e56`, under pre-published Gate 2 `pb4a`.
Freeze-preparation Gate 1: `20261029T205351Z-governor-reviewer-mbsk`.

## What changed since the historical record

The immutable [2k4p note](../../coordination/20261028T210122Z-planner-architect-2k4p.md)
correctly identified unpinned APK dependencies, but described a release-cli base
and curl/unzip followed by tar/gzip. The accepted implementation replaces that
obsolete image and client with official Alpine and curl/jq. The current job runs
`apk add --no-cache curl jq unzip`, then `apk add --no-cache tar gzip` for artifact
verification. This report updates those assumptions without rewriting the note.

The job's immutable linux/amd64 base is:

`alpine:3.24.3@sha256:c26612752c78b18faacd63c7d50f4592c2e7b0f25e0e20e82bd74e465c57ab0d`.

Its registry index is
`sha256:be2656061b536c1883c4034790e21d0a09c5aaaf51968b89368a3c0371ea2c9a`;
verified image configuration is
`b92841763c5c2140a20eeb2a44e94ebb0e80fd51d9d6b5895ea72fa999525d68`.
The image has PATH only, a `/bin/sh` command and no entrypoint or project credentials.

## Dated runtime and vulnerability evidence

[MR !3358 evidence note and attached reports](https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3358#note_4119889717)
preserve the scans, runtime inventory and independent evidence review. Disposable
containers used `--http-proxy=false`, no forwarded environment and no project
mounts. Replaying the installed package set made shell, apk, curl, jq, unzip, tar
and gzip available. Observed curl 8.23.0, jq 1.8.2 and OpenSSL 3.5.9; CA bundle
SHA-256 `5961c0a3c65459931c023e0075c09733f09aea4ca439cb85bc4a08bb4f3f568b`.
The replay installed curl/unzip/jq in a different argument order from CI, with the
same package set, followed by tar/gzip. It was a credential-free tool check, not
execution of the production publish job.

Trivy 0.76.1 official image digest:
`sha256:790fd66d1697395d8693d1acaa1ea702406a20f109b6fc7bac8db623eca6c06c`.
Vulnerability database updated `2026-10-29T13:09:46Z`, downloaded `19:41:25Z`.
Scans used `trivy image --input /scan/image.tar --scanners vuln --severity
HIGH,CRITICAL --list-all-pkgs --format json` on saved base and actual installed
runtime filesystems. Both detected Alpine 3.24.3 with no scanner warnings/errors.

| Evidence | Coverage/result | SHA-256 |
|---|---|---|
| Base scan JSON | 16 packages, zero HIGH/CRITICAL findings | `5395eb2086c98c268be89c18c99d53bce639542de76151eb37f8fd44e8c779df` |
| Installed-runtime scan JSON | 34 packages, zero HIGH/CRITICAL findings | `2aeab803bb0572f7612713c773b68d42fccfaa2fa39d48837335d79a564fa834` |
| Runtime tools inventory | Installed commands, versions and CA hash | `f56395a4c4b003f63027d254ce33f7e2472f34e1aef18a70e03b97e4e47676f7` |

Earlier Trivy 0.66.0 scans warned about their built-in Alpine EOL list; they are
not the acceptance scans. The current-scanner rerun and independent review are
the evidence above. The temporary runtime evidence image was removed; it is not
a shipped custom builder image. Old release-cli findings were not waived,
suppressed or remediated in place. The implementation replaced that component.

These results cover HIGH/CRITICAL package findings known to that database for
those observed bytes. They do not establish vulnerability freedom, coverage of
lower severities, complete supply-chain security or later APK resolutions.

## Controls and remaining uncertainty

The base digest is immutable, but APK repository metadata, resolved package
versions and the trust/CA bundle are mutable. A later tag pipeline can resolve
different tools even from the same base. Registry/repository availability can
also prevent publication. Neither full-job reproducibility nor an automatic
scan of every later publisher runtime is established by this evidence.

Existing controls preserve artifact-set/checksum/provenance verification before
uploads; check the existing tag against the pipeline SHA and require
positive Releases collection access before accepting a missing Release; and
repeat tag/absence checks before one POST without `ref`. The client accepts one
JSON document and verifies exact returned/re-fetched metadata and asset-link
bijection. It does not follow redirects, overwrite/delete a Release, create a
tag or retry POST. Tokens use stdin, not external process arguments. Ambiguous
POST results stop for operator reconciliation; a single diagnostic GET is not
permission to retry. Package uploads remain their existing separate step and
are not transactional with Release creation, so a partial outcome needs review.

A protected, operationally immutable release tag is a separate publication
prerequisite. The helper checks tag commit identity, not the API's `.protected`
field. A tag can move between precheck and POST; postconditions detect identity
disagreement observed during verification, but cannot make those operations atomic.
The publication plan must establish the tag-protection prerequisite explicitly.

Canonical MR pipeline `3005846742` passed all 23 jobs at the final source head.
Rust job `17589585071` passed its real job-token GET probe for existing protected
tag, exact Release and collection at `2026-10-29T20:54:28.647668Z`.
Trace SHA-256: `de1f01cab60e7a792d079194aadbef3288ad5e48a4b2cf83609ff9e19a06a470`.
That probe establishes read access and identity at that time. It does not prove
production POST success; no production write, throwaway tag or Release was used
for testing. Local mocks cover the create-only behavior and failure boundaries.

## Requested bounded disposition and stop conditions

Request governor acceptance of mutable APK resolution for the planned v2.13
publication only, with these disclosed limits. Package pinning remains explicitly
excluded by the approved v2.13 scope; this request does not schedule or reopen
builder-image work. The governor must record the final disposition separately.

Any new vulnerability finding requires remediation or its own explicit governor
disposition before freeze completion. Changed image/runtime assumptions or failed
artifact, tag, API or verification checks require reassessment rather than a
claim that this report covers the new state. Preserve failed/ambiguous outcomes;
do not retry a production POST merely to obtain a green pipeline.

Rollback never restores `latest` or the known obsolete publisher. Use only a
separately reviewed, verified last-known-good digest/client, or suspend release
publication. Published tags, packages and Releases are not rewritten by this
report. A later publication plan owns the tag/pipeline/artifact/anonymous
verification procedure and must carry this risk disposition explicitly.
