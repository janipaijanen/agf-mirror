---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 01KV7P02E16RFK4343JF0V8WB2
timestamp: 2026-06-16T08:50:02Z
immutable: true
references-to:
  - 01KV7NDSZEMZ1G87JTBRGGWAVC-coder-tester.md
  - 01KV7MABYEEK6R1N18J9M4S8JQ-governor-reviewer.md
topic: tooling-binary-bundle-gate2-approved
requires-ack: false
artifacts_owned:
  - tooling/elixir/lib/agov/release_package.ex
  - tooling/elixir/lib/mix/tasks/agov.tooling.build.ex
  - tooling/elixir/lib/mix/tasks/agov.bundle.build.ex
  - tooling/elixir/test/agov/release_package_test.exs
  - tooling/elixir/mix.exs
  - tooling/elixir/rel/overlays/bin/agov
  - tooling/elixir/rel/vm.args.eex
  - .gitlab-ci.yml
---

# Gate 2 approved — elixir-tooling-binary-bundle

**Branch:** `feat/tooling-binary-bundle`
**Commit:** `1aca304`
**Decision:** APPROVED — merge to main.

## Verified against plan `elixir-tooling-binary-bundle`

All plan requirements satisfied:

- **Mix release config**: `include_erts: true`, `strip_beams: true`, `overlays:
  ["rel/overlays"]`, target named `agov`. No `agf/` framework content embedded.
- **`bin/agov` overlay**: invokes `Agov.CLI.main/1` via `-eval`/`-noshell`. Correct
  for a CLI tool. Executable bit asserted in tests.
- **`mix agov.tooling.build`**: thin Mix task wrapper, exits `{:shutdown, 1}` on
  failure, calls `Agov.ReleasePackage.build_tooling/1`.
- **`mix agov.bundle.build`**: thin Mix task wrapper, exits `{:shutdown, 1}` on
  failure, calls `Agov.ReleasePackage.build_bundle/1`.
- **`Agov.ReleasePackage`**: full implementation in `release_package.ex`. Handles
  archive staging, forbidden-entry assertions, checksums, provenance JSON, and
  bundle assembly. Clean with-chain error propagation throughout.
- **Provenance fields**: `otp_version`, `elixir_version`, `target_platform`,
  `packaging` all present. OTP major derived at runtime from `System.otp_release()`.
- **Artifact naming**: `agov-<version>-linux-x86_64-otp-<otp_major>.{tar.gz,sha256,-provenance.json}`,
  `agf-<version>-bundle.{tar.gz,sha256}` — matches plan exactly.
- **Bundle contents**: `agf-<version>.{tar.gz,zip}`, tooling tar.gz, `SHA256SUMS`,
  `bundle-provenance.json` under `agf-<version>-bundle/` prefix. Wraps archives
  as-is, no re-expansion.
- **Forbidden-entry check**: `assert_no_forbidden_entries/1` guards `agf/`, `.env`,
  `.git/`, `tmp/`, `agents-govern/`.
- **Four CI jobs**: `agov:tooling:preview`, `agov:tooling:release`,
  `agf:bundle:preview`, `agf:bundle:release` — all present with correct triggers.
  Release jobs: `$CI_COMMIT_TAG` only. Preview jobs: MR + default branch.
- **`check_tag` guard**: both release jobs call `mix agov.dist.check_tag` as first
  script step — archive production cannot run without passing the guard.
- **`needs:` wiring**: `agf:bundle:release` needs both `agf:dist:release` and
  `agov:tooling:release` with artifacts. `agf:bundle:preview` needs both preview jobs.
- **Tests**: 3 new tests covering tooling archive layout/provenance/checksum, bundle
  layout+SHA256SUMS+provenance, and clean failure when tooling archive is missing.
  200 tests total, 0 failures.
- **Gate-self-test**: both release sequences abort at `check_tag` before archive
  production. Transcript in Gate 2 handoff.
- **Non-goals respected**: no macOS/Windows/ARM, no Burrito/escript, no registry,
  no signing, no SBOM.
- **Security**: no secrets in CI, no `.env` read, no hardcoded tokens.

## `vm.args.eex`

Scheduler busy-wait tuning flags only (`+sbwt none`, `+sbwtdcpu none`,
`+sbwtdio none`). Correct for a CLI tool that should not spin.

## Two pre-existing test warnings

Both noted in the handoff:
1. Unused `@tag` in `coord_config_test.exs`
2. Unused default argument in `claim_tracker_test.exs`

These are pre-existing and not introduced by this branch. Not a blocker.
