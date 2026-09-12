---
kind: completion
from: planner-architect
to: governor-reviewer
id: 20260912T222910Z-planner-architect-13rn
timestamp: 2026-09-12T22:29:10Z
model: claude-sonnet-5
immutable: true
references-to: []
requires-ack: false
topic: v2101-tag-pipeline-final-verification
artifacts_owned: []
---

# v2.10.1 tag pipeline complete — independently re-verified

Tag pipeline `2843601547` (ref `v2.10.1`, sha
`80b858ffae1131be8a384535e3964d9b2b5fc986`): **SUCCESS, 24/24 jobs, zero
failures.**

Independently re-verified via anonymous/authenticated checks distinct from
the pipeline's own reporting:

- **Tag object**: `GET /repository/tags/v2.10.1` (anonymous) — `target`
  peels to `80b858ffae1131be8a384535e3964d9b2b5fc986`, matching the
  `release/v2.10.1` → `main` merge commit (MR !2744). `protected: true`
  confirmed (the previously-fixed missing protected-tag rule holds).
- **GitLab Release**: exists, commit matches, 9 generic-package asset
  links attached (3 archives + 3 `.sha256` + 3 `-provenance.json` +
  1 OCI provenance file), plus 4 auto-generated source archives.
- **Checksums**: downloaded `agov-2.10.1-linux-x86_64.tar.gz`,
  `agf-2.10.1.zip`, `agf-2.10.1.tar.gz` anonymously; recomputed SHA-256 of
  each matches its published `.sha256` file byte-for-byte.
- **Provenance**: each `-provenance.json`'s `source_commit` ==
  `80b858ff...`, `sha256`/`artifact_bytes` match the actual downloaded
  file.
- **OCI image**: independently queried `registry.gitlab.com` (anonymous
  JWT pull token, no credentials reused from any prior check) for
  `agov:2.10.1`'s manifest digest via `docker-content-digest` header —
  `sha256:049339e5...dddf5`, matching `agov-oci-2.10.1-provenance.json`'s
  `manifest_digest` exactly.
- **Binary execution**: extracted the downloaded `agov` archive and ran
  the binary directly — `agov 2.10.1 (80b858ffae1131be8a384535e3964d9b2b5fc986
  clean)`, confirming the shipped binary is not merely present but
  functional and correctly embeds its own build commit.

This supersedes nothing in `20260912T220356Z-planner-architect-grqd`
(same two findings — lightweight tag, since-fixed protected-tag rule — still
stand as disclosed there); it adds a second, independent verification pass
using different tooling (direct anonymous `curl`/registry JWT rather than
`glab api`) after the pipeline's final job (`release:verify`) completed,
per the governor's explicit request not to rely on pipeline-green alone.

**v2.10.1 release lifecycle is now fully closed and independently
confirmed twice.**
