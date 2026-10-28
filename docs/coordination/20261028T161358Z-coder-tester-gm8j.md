---
kind: completion
from: coder-tester
to: all
id: 20261028T161358Z-coder-tester-gm8j
timestamp: 2026-10-28T16:13:58Z
model: codex
immutable: true
references-to:
  - docs/coordination/20261028T153858Z-governor-reviewer-eq8h.md
topic: v212-release-publication-complete
requires-ack: false
artifacts_owned: []
---

# AGF v2.12.0 publication complete

AGF v2.12.0 is published and independently verified.

## Immutable identity

- Annotated tag: `v2.12.0`
- Tag object: `008ec0c3c633db62ff7e593aa9fb12c405cf9b27`
- Peeled commit: `e045d20b073ce31341cd0d460469fac60cdbab5a`
- Tagger time: `2026-10-28T15:48:02Z`
- Release: `agents-govern v2.12.0`, released at
  `2026-10-28T16:12:00.241Z`
- Generic package: `agf` / `v2.12.0`, package ID `74135079`, status
  `default`
- OCI reference:
  `registry.gitlab.com/janipaijanen/agents-govern-framework/agov:2.12.0`
- OCI manifest digest:
  `sha256:65fedc0afecd36f9a23a29d847a5c38d3128b1ddbcfc87d908697a647015751f`

## Published bytes

- `agf-2.12.0.tar.gz`:
  `b44c1fc619f0efcff458b45c7f824f98eb4f32ea9c1bf72669888119e615d2c1`
- `agf-2.12.0.zip`:
  `4f4f2a95039e10101c95e1c06f25ae38aadd6b428b439d79206a57e091a0f17e`
- `agov-2.12.0-linux-x86_64.tar.gz`:
  `b5d47d0b2991fb106db09ac795f77cef05bd12317b85a9da149dedd5e8b2fc9f`

Each archive has a published checksum sidecar and v2.4 provenance record.
All provenance records bind source commit
`e045d20b073ce31341cd0d460469fac60cdbab5a`, tag `v2.12.0`, pipeline
`3000042011`, and their producing job.

## Verification

Tag pipeline `3000042011` succeeded with all 23 mandatory jobs green. The
publication jobs were tooling build `17546210043`, framework distribution
`17546210061`, Release creation `17546210062`, Release verification
`17546210063`, pre-OCI asset verification `17546210064`, and OCI publication
`17546210065`.

`agov dist post-publication-check` then used the anonymous GitLab Release API
and fresh directory `/tmp/agf-v212-public.p4QFrP` to download all ten linked
assets. It validated the downloaded bytes against their downloaded sidecars
and provenance, checked the packaged binary self-report, and pulled and
verified the real published OCI image by digest. No credential was supplied.
The temporary path is local evidence only and is not committed.

The exact protected-tag rule was re-read after publication and still restricts
`v2.12.0` creation to Maintainers, access level 40, with no user, group, or
deploy-key exception. The tag object, message `AGF v2.12.0`, peeled commit,
Release commit, package source, OCI labels, and all provenance identities
agree.

The separate publication-status synchronization MR required by the approved
release plan is now the next release task. This completion record does not
perform that status edit, Pages deployment, or announcement.
