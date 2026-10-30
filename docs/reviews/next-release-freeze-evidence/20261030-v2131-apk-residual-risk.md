<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.13.1 publisher residual evidence and current disposition context

## Current source and authority context

The historical diagnostic body below is the independently reviewed report
SHA256 `7f2ad87905d77fb41e1049114bac0cef68ec2836c29361ecd8ebc63e533ead41`.
Its content is copied verbatim between source markers; that hash applies to
those payload bytes, not this enclosing tracked file. The evidence manifest
remains `fc8ce119707cb374d99cdcd3f0019d12d30c8cf13e479f8559684a822769a141`.
No old evidence, scanned input, package result or finding has been relabeled.

The diagnostic observation was at bb407c35969ccb251a32f5fe32179e58600223a7.
The approved product freeze is now **93e203466fbb4fdc50f926a7b7c7490bfbb0fdc9**.
Fresh source-continuity evidence
SHA256 `5816f7e3dfa3097ef85f43c7472c266c4603c088040b909d91bdd3657101f3cb`
proves all eight publisher/release inputs still byte-identical to that
observation and v2.13.0; only the separately governed README paragraphs and
audit records intervened. The exact-source estate addendum records those
parents, tree and six paths. No rescan or production execution is implied by
this continuity statement.

Decision `20261030T075341Z-governor-reviewer-bmaq`, published with the approved
release plan at d74d6f08e224f07cd54028f530ea26823351d56f, supplies the
**fresh conditional v2.13.1-only** acceptance of documented mutable repository/
package/tool/CA resolution, availability, unknown-vulnerability and non-atomic
publication residual. It does not waive a known vulnerability. It binds the
exact report/manifest, base/scanner/database and package/tool/CA/source inputs
below. This wrapper reports that decision; the report author does not accept
risk or extend its scope. ja28's v2.13.0-only acceptance is not reused.

The same bmaq record separately grants a v2.13.1-only exception for the three
named inherited authored SPDX-header omissions, retaining their current
CC-BY-SA-4.0 treatment. The all-file header checklist is not fully satisfied;
no header repair, license conversion, general exemption or third-party-license
waiver is authorized. tgrk remains outstanding. Final metadata Gate 2 must
recheck the actual three file hashes, omissions and unchanged licensing.

### Continuing controls and expiry

Acceptance ends **2026-10-30T18:00:00Z** and must remain active through terminal
success of the entire release:create job: both APK installs, every package
upload, Release POST and follow-up response/GET verification. A tag or POST
before expiry does not grandfather later execution. Before tag push, bmaq
requires at least max(two hours, twice the latest complete tag-pipeline wall
time plus current queue allowance); 16:00Z is the latest possible cutoff and
evidence can require earlier. Unknown or inadequate capacity stops before tag
creation. Monitor risk-bearing work and cancel/suspend queued or running work
before expiry if delay threatens completion, preserving immutable/partial
outputs and reconciling ambiguous outcomes read-only. Do not retry expired work.

Recheck frozen input continuity and bind the exact metadata head at Gate 2;
recheck source, namespace and effective tag protection immediately before push.
A material source/base/package/client/control change or any new known finding
requires fresh assessment/disposition. Preserve create-only/no-redirect/no-POST-
retry, positive access, artifact/provenance verification and partial-outcome
controls. No publisher, builder-image, runtime or CI edit is authorized here.
Separate merge/tag/publication/status gates remain. The original body's request
for a fresh risk decision is historical; bmaq supplies only the conditional
scope above, not a production success guarantee.

## Reviewed diagnostic report (verbatim)

<!-- BEGIN SOURCE publisher-diagnostic-20261030 sha256=7f2ad87905d77fb41e1049114bac0cef68ec2836c29361ecd8ebc63e533ead41 -->
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
# v2.13.1 publisher evidence refresh — 2026-10-30

Status: diagnostic evidence for independent review and a **new** governor
residual-risk disposition. This report accepts no risk and authorizes no
release implementation, merge, tag, upload, deployment or production POST.
No tracked file changed. Old v2.13.0 evidence remains unchanged.

## Result and exact coverage

Fresh Trivy 0.76.1 scans found **zero vulnerabilities at every reported severity**
(UNKNOWN, LOW, MEDIUM, HIGH, CRITICAL): 16 packages in the pinned base and 34 in
the actual captured post-install filesystem. Both full-severity scans exited 0
with no scanner warning/error. Original HIGH/CRITICAL-only scans also passed and
are retained, but the full-severity reports are the broader current evidence.
No finding was suppressed, ignored, reclassified or accepted. No retry was used
to turn a failed scan green; the second pair expanded severity coverage at the
independent reviewer's request, on the same inputs/database.

- Source baseline: verified Pages merge
  `bb407c35969ccb251a32f5fe32179e58600223a7`.
- Base requested/pulled by immutable linux/amd64 manifest:
  `docker.io/library/alpine@sha256:c26612752c78b18faacd63c7d50f4592c2e7b0f25e0e20e82bd74e465c57ab0d`.
- Alpine version: 3.24.3. Base image config:
  `sha256:b92841763c5c2140a20eeb2a44e94ebb0e80fd51d9d6b5895ea72fa999525d68`.
  Podman retains the index digest
  `sha256:be2656061b536c1883c4034790e21d0a09c5aaaf51968b89368a3c0371ea2c9a`
  as its display Digest; RepoDigests records both index and requested child.
- Scanner pulled by the previously reviewed immutable official image digest:
  `docker.io/aquasec/trivy@sha256:790fd66d1697395d8693d1acaa1ea702406a20f109b6fc7bac8db623eca6c06c`.
  Reported version 0.76.1; linux/amd64 config
  `ac7fdaf3e1f1442ec467a00d1ec900bc8dc20f694d172b11ff43d32f83162fb6`.
  Digest/content identity was checked; no new signature/source audit is claimed.
- Fresh DB UpdatedAt `2026-10-30T01:01:11.134360213Z`, DownloadedAt
  `2026-10-30T07:03:01.047807763Z`, NextUpdate
  `2026-10-31T01:01:11.134359572Z`; downloaded through Trivy's default
  `mirror.gcr.io/aquasec/trivy-db:2` repository.
- Database SHA256 `a12867e2335eb513a05e7c429798051d63cc8902a8c80f3dac48e1be97fd70c7`,
  size 1,666,846,720 bytes. Both scan pairs used this one database offline with
  updates disabled. Full-severity observations: 2026-10-30T07:06:02Z–07:06:03Z.

## Runtime replay and isolation

A disposable named Podman container used `--http-proxy=false`, no `--env`, no
host/project mounts and no credentials. Inspection records an empty mounts list
and only Podman's generated container/PATH/HOME/HOSTNAME environment; these
were not forwarded user/credential variables. Exact package commands were:

```sh
apk add --no-cache curl jq unzip
apk add --no-cache tar gzip
```

Installation and tool/CA inventory completed with exit 0 at
2026-10-30T07:02:41Z. The `/bin/sh` replay successfully executed the apk,
curl, jq, unzip, tar and gzip version commands. BusyBox's `command -v` invocation
recorded only `/bin/sh`; no separate path result is claimed for the other tools.
The installed filesystem was committed to a **temporary local evidence image**,
saved to Docker archive and scanned; it was never pushed or used as a shipped
builder/publisher. Its config was
`sha256:2b1a8ea6f33193ea7dd9c46e5f3fb668b2a5003bef693b80cfa5be2c0930732c`.
The evidence container and local evidence image were removed after scans.

| Observed package | Version |
| --- | --- |
| apk-tools | 3.0.8-r0 |
| curl / libcurl | 8.23.0-r0 |
| jq | 1.8.2-r0 |
| unzip | 6.0-r16 |
| tar | 1.35-r5 |
| gzip | 1.14-r3 |
| libcrypto3 / libssl3 | 3.5.9-r0 |
| ca-certificates-bundle | 20261020-r0 |

Curl reports OpenSSL 3.5.9 with c-ares/HTTP2/brotli support. Both
`/etc/ssl/cert.pem` and `/etc/ssl/certs/ca-certificates.crt` have SHA256
`5961c0a3c65459931c023e0075c09733f09aea4ca439cb85bc4a08bb4f3f568b`, owned by
ca-certificates-bundle-20261020-r0. Complete package/tool output is retained.
APK repositories were `https://dl-cdn.alpinelinux.org/alpine/v3.24/main` and
`https://dl-cdn.alpinelinux.org/alpine/v3.24/community`.

Scanner containers also used `--http-proxy=false`. The DB fetch mounted only
the owned temporary cache. Actual scans ran with `--network=none`, mounted only
the owned `/tmp` evidence inputs read-only and the owned cache, and used:

```text
image --cache-dir /cache --input /scan/{base,runtime}-image.tar
--skip-db-update --skip-java-db-update --offline-scan --scanners vuln
--list-all-pkgs --format json --exit-code 1
```

The full scans had **no severity filter**. Initial scans additionally passed
`--severity HIGH,CRITICAL`. No custom ignore file, ignore-unfixed option or
suppression/config was supplied. Scan logs detect Alpine OS packages and zero
language-specific files. Secret, misconfiguration and license scanners were
not enabled; this is a vulnerability-package assessment only.

## Source continuity and inherited headers

`source-and-headers.json` compares bytes at the named merge with immutable
v2.13.0 (annotated object `a75803f2f791779fb1ec82aad92eb150cdfab56b`,
peeled commit `a46c2e1c8754103fce45b6bdabe023c3ee79b2b6`). All eight
checked inputs are identical: `.gitlab-ci.yml`,
`create-release.sh`, `release-assets.sh`, `release-namespace-check.sh`,
`agov-oci-image.sh`, `release-targets.txt`, `Containerfile.agov`, and the
`agf/dist/release-manifest.yaml`. Full paths and all SHA256 values are in that JSON.
Key exact hashes:

- `.gitlab-ci.yml`:
  `5a9fd4ff1cc8f7c279e0d6f5cde90c34c26d3c1506b464ad99a3b4c163af8d2e`.
- `tooling/rust/scripts/create-release.sh`:
  `28b504308627e9b2581156100a59d486bd0148d30e469a8e59895720ace8663e`.

All three inherited authored header omissions are re-observed:
`CHANGELOG.md`, `agf/CHANGELOG.md`, `agf/templates/learning-record.yaml` contain
no SPDX identifier in their first 20 lines and are whole-file byte-identical
to v2.13.0. This corroborates the existing omission finding; it does not make
those files header-compliant or supply a new checklist exception. dta3 and
ja28 remain v2.13.0-only; tgrk remains a separate unimplemented repair proposal.

## Evidence files and hashes

All paths below are under `/tmp/agf-v2131-publisher-refresh-20261030/`.
`all-evidence.sha256` is the complete manifest, SHA256
`fc8ce119707cb374d99cdcd3f0019d12d30c8cf13e479f8559684a822769a141`.

| Evidence | SHA256 |
| --- | --- |
| base-all-severity-scan.json | `45f30799a8e8ef98d365376ab30258d6625509f388ebb60ab18c5f032929bcb6` |
| runtime-all-severity-scan.json | `59cf4ba150b764b30bf2a87d3b7acaeb85bb19693c9b7d25a89b238844914fb1` |
| all-severity-summary.json | `32e1da907c71f5941a562c15226c45b7e45103745ac71b8953a061434248d704` |
| runtime-install-inventory.txt | `74350c6e4102462622392c8be04e2c4969d681106c62d91a36b41a4f273db9d9` |
| database-metadata.json | `667c582ccbd266cc7e28e586fa27bd923a84b42ef1a62f9066934207ad35410d` |
| source-and-headers.json | `d04d526c21bf0a751e1996fe05f965bc8b6a99b25ad9c76f211bca0e3099ffde` |
| base-image.tar | `32550407d937ed2318e8f71a1e23831630d9dc86ce83b79fa004706fc6be6a30` |
| runtime-image.tar | `d4e4b4174553bc4bfd97a42e3924085a232d7f67b85aaa3b260cf22892893a76` |

Scanned tar inputs, full JSON, logs, inventory and identities are retained for
review. The disposable 1.6GB DB cache was hashed and removed to restore disk
space; metadata/content identity remain. Exact database replay would require
matching that DB hash; a later fresh DB scan can legitimately differ. No
pre-existing cache, report, worktree or image was removed.

## Replay evidence versus production guarantees

This replay proves the named install resolved these observed packages and tools
on this architecture and that the dated scanner reported no findings for the
captured filesystems. It does **not** execute the production publisher, exercise
an authenticated current API, validate a future namespace/protection rule, or
prove a future APK install will resolve identically. APK repositories/tool/CA
versions and availability remain mutable despite the base digest. No full-job
bit-for-bit reproducibility or vulnerability freedom is claimed.

Unchanged reviewed code preserves archive/provenance checks before uploads,
positive Release collection access, exact tag commit checks, one ref-free POST,
no redirects/POST retry/update/delete, stdin token transport, and returned plus
re-fetched metadata/asset-link checks. These are source-continuity observations
against prior reviewed code, not newly executed production proofs. The helper
checks tag identity, not `.protected`; operator protected-tag verification is
still required. Checks and POST are not atomic. Generic-package upload, OCI and
Release creation are separate steps with possible partial outcomes. Historical
v2.13.0 production success is evidence for that cut only.

Any later source/base/package/control change, new scanner finding, failed API or
artifact check, or expired authority requires a new assessment/disposition.
Zero scanner findings do not waive future lower-severity findings or eliminate
unknown vulnerabilities. Independent review and a fresh exact-v2.13.1 governor
risk record remain required; this report supplies neither gate approval nor
risk acceptance.
<!-- END SOURCE publisher-diagnostic-20261030 -->
