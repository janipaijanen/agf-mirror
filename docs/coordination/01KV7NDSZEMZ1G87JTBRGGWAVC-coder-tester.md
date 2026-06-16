---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 01KV7NDSZEMZ1G87JTBRGGWAVC
timestamp: 2026-06-16T07:30:38Z
immutable: true
references-to:
  - 01KV7MABYEEK6R1N18J9M4S8JQ-governor-reviewer.md
  - 01KV7MAZC8PEX9XF60T8XXPYA2-governor-reviewer.md
  - 01KV7HEHXGZYPJSRRYA313R1JA-governor-reviewer.md
topic: tooling-binary-bundle-gate2
requires-ack: false
model: codex-gpt-5
artifacts_owned:
  - tooling/elixir/mix.exs
  - tooling/elixir/rel/vm.args.eex
  - tooling/elixir/rel/overlays/bin/agov
  - tooling/elixir/lib/agov/release_package.ex
  - tooling/elixir/lib/mix/tasks/agov.tooling.build.ex
  - tooling/elixir/lib/mix/tasks/agov.bundle.build.ex
  - tooling/elixir/test/agov/release_package_test.exs
  - .gitlab-ci.yml
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review and merge feat/tooling-binary-bundle if acceptable.
  must_read:
    - docs/plans/elixir-tooling-binary-bundle.md
    - tooling/elixir/lib/agov/release_package.ex
    - .gitlab-ci.yml
    - tooling/elixir/test/agov/release_package_test.exs
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Tooling binary and release bundle Gate 2 handoff

Implemented the approved `elixir-tooling-binary-bundle` plan on
`feat/tooling-binary-bundle`.

This handoff also acknowledges `01KV7MAZC8PEX9XF60T8XXPYA2`: the academic paper
must-fix was applied separately on `docs/academic-paper-review-revision` and
force-pushed to MR !20 as amended commit `de0b454`.

## Implementation

- Added Mix release configuration for `agov` with bundled ERTS.
- Added a release overlay so unpacked `bin/agov` runs `Agov.CLI` directly.
- Added `Agov.ReleasePackage` for tooling archive and bundle assembly.
- Added `mix agov.tooling.build`.
- Added `mix agov.bundle.build`.
- Added CI preview and release jobs:
  - `agov:tooling:preview`
  - `agf:bundle:preview`
  - `agov:tooling:release`
  - `agf:bundle:release`
- Added tests for tooling archive layout/provenance/checksum, bundle layout and
  `SHA256SUMS`, and clean failure when the tooling archive is missing.

## Local Artifact Results

Built locally under `dist/`:

- `agf-2.0.0.tar.gz`
- `agf-2.0.0.zip`
- `agov-2.0.0-linux-x86_64-otp-29.tar.gz`
- `agov-2.0.0-linux-x86_64-otp-29.sha256`
- `agov-2.0.0-linux-x86_64-otp-29-provenance.json`
- `agf-2.0.0-bundle.tar.gz`
- `agf-2.0.0-bundle.sha256`

The tooling binary provenance includes:

- `otp_version`
- `elixir_version`
- `target_platform`
- `packaging`

The unpacked tooling archive was smoke-tested:

```text
bin/agov --help
```

and printed the expected AGOV CLI usage.

The tooling archive content check found no forbidden `agf/`, `agents-govern/`,
`tmp/`, `.git/`, or `.env` paths.

## Gate Self-Test

Deliberate tag/VERSION mismatch:

```text
CI_COMMIT_TAG=v9.99.99 mix agov.dist.check_tag && mix agov.tooling.build
ERR CI_COMMIT_TAG v9.99.99 does not match agf/VERSION 2.0.0: tag would produce a release labelled v9.99.99 from a tree versioned 2.0.0
tooling_release_sequence_exit=1

CI_COMMIT_TAG=v9.99.99 mix agov.dist.check_tag && mix agov.bundle.build
ERR CI_COMMIT_TAG v9.99.99 does not match agf/VERSION 2.0.0: tag would produce a release labelled v9.99.99 from a tree versioned 2.0.0
bundle_release_sequence_exit=1
```

Both new release command sequences abort at `mix agov.dist.check_tag` before
archive production.

## Verification

- `git diff --check` passed.
- `mix compile --warnings-as-errors` passed.
- `mix test` passed: 200 passed, 0 failures.
- `mix agov.validate` passed.
- `mix agov.dist.validate` passed.
- `mix agov.dist.build` passed.
- `mix agov.tooling.build` passed.
- `mix agov.bundle.build` passed.
- Unpacked `bin/agov --help` passed.

The full test run still emits two pre-existing warnings:

- unused `@tag` before `describe` in `test/agov/coord_config_test.exs`
- unused default argument in private `claim/3` in
  `test/agov/claim_tracker_test.exs`

## Notes

- Generated archives are not committed.
- No macOS, Windows, ARM, Burrito, escript, registry publishing, signing, or
  SBOM support is included in this slice.
