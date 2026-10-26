---
kind: completion
from: governor-reviewer
to: all
id: 20261026T070156Z-governor-reviewer-sntb
timestamp: 2026-10-26T07:01:56Z
model: codex
immutable: true
references-to:
  - 20261026T061833Z-governor-reviewer-speg
  - 20261026T053900Z-governor-reviewer-g7pj
topic: v2.11.0 release published and independently verified
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.11.0 published and independently verified

Release `v2.11.0` is complete. The immutable annotated tag object is
`d732e372f35bacd1f37981bb89031041d35121e4`; it peels to final authorized
`main` commit `c1a13f8cdf626a9c10df5ddc82cc8cf6ebefb48a`. The tagger and message
are the intended release identity and `Release v2.11.0`. The exact protected
tag remains restricted to Maintainers (access level 40; access-level object
`44387056`).

Tag pipeline `2988338354` passed all 23 jobs. The required publication jobs
passed as follows:

| Job | ID |
|---|---:|
| `agf:dist:release` | `17459246612` |
| `release:assets-verify-pre-oci` | `17459246615` |
| `agov:oci:release` | `17459246616` |
| `release:create` | `17459246613` |
| `release:verify` | `17459246614` |

GitLab generic package `agf` version `v2.11.0` is package ID `73744928`.
Published archive identities are:

| Archive | Bytes | SHA-256 | Provenance record |
|---|---:|---|---|
| `agf-2.11.0.tar.gz` | 1,233,427 | `9477aab6dd24ac7b15e9791fae0a809de447e61fbb57acc46af20c8c509aa939` | `agf-2.11.0.tar.gz-provenance.json` |
| `agf-2.11.0.zip` | 1,394,198 | `88adbed50d0353124505983d1aad5433a8200bb2e58e85e968e4a975f98b5d8c` | `agf-2.11.0.zip-provenance.json` |
| `agov-2.11.0-linux-x86_64.tar.gz` | 4,316,283 | `01a8f2cf63cda43f0eefd711b44ba67ffb263cfad7984cad802c3955349ebfce` | `agov-2.11.0-linux-x86_64.tar.gz-provenance.json` |

All four schema-v2.4 provenance records bind version `2.11.0`, tag
`v2.11.0`, and source commit `c1a13f8c`: the three records above plus
`agov-oci-2.11.0-provenance.json`. The OCI record binds
`registry.gitlab.com/janipaijanen/agents-govern-framework/agov:2.11.0` to
manifest digest
`sha256:baa57b6e460735b93b9f647d11f724d13c0e1d21096ce50c4d63ef6331a521d0`.

## Independent anonymous verification transcript

The operator ran the approved command with no authentication:

```text
agov dist post-publication-check \
  --releases-url https://gitlab.com/api/v4/projects/83321848/releases/v2.11.0 \
  --download-dir /tmp/agf-v211-postpub-Qdmx7D
```

Transcript location for this session:
`tmp/v211-post-publication-check.log`. It records successful anonymous
downloads of all ten linked release assets; validation of all four provenance
records; SHA-256 agreement for all three archives; a real anonymous pull of
the OCI manifest digest above; and agreement of the packaged `agov` binary's
self-reported version. The separate Git object check confirmed the annotated
tag object and peeled commit named above. The public Release API returned 14
assets and the intended release URL.

Ship-in issues #121, #122, #123, and #124 were closed after verification, each
with a note linking this release, tag pipeline, `release:verify`, and OCI
digest. Issues #131 and #132 were already closed before the cut.

Any roadmap or governance-manifest wording that still calls v2.11.0 pending
must be corrected forward in a separate reviewed MR; it does not alter this
published tag or its verified artifacts.
