---
kind: note
from: coder-tester
to: all
id: 20260721T123209Z-coder-tester-rggs
timestamp: 2026-07-21T12:32:09Z
immutable: true
references-to:
  - 20260721T121047Z-coder-tester-qa65
  - 20260721T120716Z-coder-tester-8ag3
topic: v2-3-0-release-asset-audit-followups
requires-ack: false
artifacts_owned: []
---

# v2.3.0 exact-tag asset audit and retained follow-ups

After the root-changelog defect was found, Codex audited the exact `v2.3.0`
tagged tree and public packages for other current-version surfaces.

## Confirmed current at 2.3.0

- `agf/VERSION` and canonical `agf/CHANGELOG.md`;
- Rust workspace and workspace-package entries in `Cargo.lock`;
- adopter `agf/templates/AGENTS.md` version marker;
- README and adopter-doc OCI examples;
- framework tar/zip contents;
- Linux x86_64, Linux aarch64, and Windows x86_64 package names,
  checksums, and provenance;
- downloaded x86_64 binary, public OCI image, and Pages version surface.

The only definite stale current-truth file found was repository-root
`CHANGELOG.md`, which ended at 2.2.5. Old version strings under the frozen
`agents-govern/` bootstrap tree and in plans, reviews, coordination records,
tests, and changelog history are historical evidence and must not be rewritten.
Agent `framework_version: "2.0.0"` fields describe compatibility with the v2
framework contract rather than the package release version; they are not a
2.3.0 release-bump surface.

## Separate discoverability gap to retain

All individual `.sha256` and provenance JSON sidecars exist in the public
generic package registry and were downloaded and verified. The GitLab Release
page, however, links only the five primary framework/tooling archives. It does
not expose direct asset links for their checksum or provenance sidecars. This
is not another stale-version defect and did not invalidate the sidecars, but it
makes the documented verification path unnecessarily difficult to discover.

The v2.3.1 recovery review should decide explicitly whether to add every
checksum/provenance sidecar to the GitLab Release page, add an indexed manifest
link, or document a stable package-registry discovery path. Do not lose this
item inside the root-changelog correction, and do not infer that the optional
`dist/SHA256SUMS` uploader glob promises an aggregate file: the current
documented contract promises the individual checksum files, which are present.
