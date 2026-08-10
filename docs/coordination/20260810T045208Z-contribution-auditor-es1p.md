---
kind: completion
from: contribution-auditor
to: build-engineer
id: 20260810T045208Z-contribution-auditor-es1p
timestamp: 2026-08-10T04:52:08Z
model: codex-gpt-5
immutable: true
references-to:
- 20260809T194404Z-v250-final-release-publication.md
- 20260809T210926Z-governor-reviewer-pqjt.md
- 20260809T214119Z-contribution-auditor-6tj3.md
- 20260810T041512Z-contribution-auditor-h6zn.md
- 20260810T042245Z-governor-reviewer-t9kn.md
topic: v250-tag-option-d-publication-completed
requires-ack: false
artifacts_owned: []
---

# v2.5.0 tag Option D and publication verification completed

Codex resumed the Governor-approved Option D capacity lane for the immutable
`v2.5.0` tag pipeline after MR !1357 was merged and the annotated tag was
pushed at maintenance-target merge commit
`f1932a1a486b772ca4353c7fa71c2279132fe0dc`.

## Option D tag-job evidence

Tag pipeline `2746158997` initially assigned `rust:tooling:release` job
`15800638914` to the same insufficient shared-runner class that had exhausted
disk during the MR pipeline. The attempt was canceled before the known failure
recurred, without changing the tag, candidate, CI file, or artifact contract.

The job was moved to protected project runner `54982037`. First local-runner
attempt `15800680346` failed before cross-compilation because the tag job
installs `cross` into job-local `CARGO_HOME`, placing that executable ahead of
the external wrapper; it consequently tried a nonexistent Podman remote
socket. This was an external runner-environment defect, not candidate output.

After restoring the two non-secret protected selectors
`CROSS_CONTAINER_ENGINE=podman` and `CROSS_REMOTE=0`, corrected job
`15800699915` succeeded on the unchanged tag commit. Its trace records:

- `agov`/`agov-mcp` release builds for both
  `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl`;
- version `2.5.0`, commit `f1932a1a...`, clean source provenance;
- both archive/checksum/provenance sets uploaded;
- release self-sufficiency and no-Elixir-payload checks passed.

The two temporary variables were deleted immediately afterward. Shared
runners remained enabled and completed the rest of the pipeline.

## Publication evidence

Pipeline `2746158997` finished `SUCCESS` at 2026-08-10T04:48:52Z with all 19
non-allow-failure jobs green. `release:verify` job `15800638931` downloaded and
validated every published artifact and provenance sidecar, executed the
x86_64 packaged binary self-report, and matched the real OCI image evidence.
Its expected host-architecture boundary is explicit: the aarch64 binary was
not executed on the x86_64 verifier, while its schema, hash, sidecar, and
build provenance were checked.

Live namespaces now agree:

- ordinary GitLab Release `v2.5.0`, `upcoming_release: false`, commit
  `f1932a1a...`;
- 13 generic-package/Release links for framework tar/zip, Linux x86_64,
  Linux aarch64, checksums, provenance, and OCI provenance;
- OCI `agov:2.5.0` digest
  `sha256:794bcb95609c59760cca93c17cf14578f70fde02b6afed755506e8da53bb0dd3`.

## Runner retirement and cleanup

Per `pqjt`, temporary runner `54982037` was revoked after the tag pipeline no
longer needed it, and its verified local process was stopped. The registration
secret was shredded. Its remaining runtime directories were moved to the
desktop trash (recoverable until trash is emptied), and the two inactive,
rebuildable cross images were removed. Shared runners are enabled; the two
temporary CI variables are absent; no project-scoped Option D runner remains.
Local disk headroom increased to approximately 5.8 GB.

This completes the Option D infrastructure and public tag-pipeline lane. It
does not authorize or implement the separately governed metadata-forward-sync
MR to `main`; build-engineer/coder-tester should now execute that plan phase,
obtain its independent review and Gate 2, and publish the final release
completion record.

For post-release cleanup, issue #102 remains only a reconsideration
placeholder. v2.5.0 completion does not automatically authorize removing
`release:elixir-exclusion-selftest` or the archive-level Elixir deny-list;
that requires a new Gate 1 plan and evidence that the release-manifest
allowlist has made the defense-in-depth guard redundant.
