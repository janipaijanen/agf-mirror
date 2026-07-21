---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - tooling/elixir/mix.lock
  must_read:
  - docs/plans/v2.4.0-workstream-b-provenance-contract.md
  - agf/schemas/release-provenance.schema.json
  next_agent: governor-reviewer
  status: ready
  task:
  - Review MR !351 for Gate 2 approval.
  - Confirm the discriminated-union schema, cardinality rule, and legacy
    classification match the approved plan scope.
  - Decide whether the flagged full-scale `agov dist build` gap (below)
    needs to be closed before merge or can be verified in CI instead.
id: 20260721T211300Z-coder-tester-p8pq
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260721T204707Z-governor-reviewer-r4hg.md
- 20260721T204904Z-coder-tester-3ak5.md
requires-ack: true
timestamp: 2026-07-21T21:13:00Z
to: governor-reviewer
topic: v2-4-0-workstream-b-implementation-complete
---

# Workstream B implementation complete: MR !351

Implements the approved unified release-provenance contract
(`docs/plans/v2.4.0-workstream-b-provenance-contract.md`) on branch
`feat/v240-workstream-b-provenance-contract`, per handoff `r4hg`.

**MR:** https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/351

## What changed

- `agf/schemas/release-provenance.schema.json` — closed discriminated
  union keyed by `artifact_kind` (`file` / `oci-image`),
  `additionalProperties: false` throughout, embedded into the `agov`
  binary at compile time (`include_str!`).
- `agov-core::schema::compile_str` — compiles a schema from an in-memory
  string, no filesystem access, for the embedded-schema path.
- New `agov::provenance` module: builders, schema validation, legacy
  (`assurance_level: legacy`) classification, and the cross-record checks
  the schema alone can't express (artifact/sha256 agreement, checksum-
  sidecar agreement, duplicate-filename detection, OCI label agreement).
  25 unit tests, including all seven negative cases named in the plan.
- `dist.rs::write_provenance` rewritten: one `file`-kind record per
  published archive format, not one aggregate record.
- New CLI: `agov dist provenance-record file|oci` (generator, replacing
  the previous shell `printf` JSON construction) and
  `agov dist validate-provenance <files...> [--artifact-dir <dir>]`
  (validator, build-time or post-publication).
- `.gitlab-ci.yml` / `agov-oci-image.sh`: the Rust tooling-archive job now
  generates real schema-valid records via a host-native `agov` binary;
  the OCI release job captures the actual pushed image digest
  (`podman push --digestfile`) and publishes a matching `oci-image`
  provenance record as a GitLab Release asset.
- `release-assets.sh` mandatory-asset set and `selftest` updated for the
  new `<artifact_filename>-provenance.json` naming (framework provenance
  filenames split from the old aggregate `agf-X.Y.Z-provenance.json` into
  one per format); `release-artifacts.md` documents the unified contract.

## Verification

`cargo test --workspace` (all green, including the 25 new tests),
`cargo clippy --workspace -- -D warnings` (clean), `cargo fmt --all
--check` (clean), `agov plan validate` (OK), `git diff --check` (clean),
`release-assets.sh selftest` (OK), and a manual CLI smoke test against
real files exercising both the positive and negative (dangling-reference)
paths for `validate-provenance`, plus OCI and legacy-record cases. Full
detail and the one open verification gap (a full-scale `agov dist build`
against the real 205-file manifest timed out in this sandbox for
unrelated I/O-speed reasons; the equivalent fixture-based unit test
passes) are in the MR description.

CHANGELOG entry deferred to v2.4.0 release-branch prep per
`20260721T165358Z-governor-reviewer-5pgp`, as already decided.

Requesting Gate 2 review.
