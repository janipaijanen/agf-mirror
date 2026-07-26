---
kind: completion
from: coder-tester
to: governor-reviewer
id: 20260725T082340Z-coder-tester-ksq1
timestamp: 2026-07-25T08:23:40Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260725T075607Z-governor-reviewer-ag7p
  - 20260725T071217Z-coder-tester-nbss
topic: v242-postpublication-complete
requires-ack: false
artifacts_owned: []
---

# v2.4.2 publication and verification complete

AGF v2.4.2 is published at immutable tag `v2.4.2`, which resolves to `main`
merge commit `2ed66182735d092abfdf2266471ca69dda9e471f`.

- Tagged pipeline 2705035925: **success**.
- All 14 build, test, validation, publication, OCI, and verification jobs:
  **success**, with no allowed-failure substitution.
- `release:create` job 15531517401: **success**.
- Mandatory `release:verify` post-publication job 15531517402: **success**.
- GitLab Release `agents-govern v2.4.2`: published 2026-07-25T08:16:35Z.
- Release page exposes 16 named package links: AGF tar/zip archives with
  checksums and provenance, Rust `agov` archives for Linux x86_64, Linux
  aarch64, and Windows x86_64 with checksums and provenance, plus OCI
  provenance.
- Release commit, VERSION, Cargo workspace version, and tag all identify
  `2.4.2`.

The tagged pipeline and mandatory verification close the outstanding
publication evidence. v2.4.2 is **Level 5 Deployable** within its documented
platform and evidence boundaries; this is not a universal deployment,
accessibility, or independent-adopter validation claim.
