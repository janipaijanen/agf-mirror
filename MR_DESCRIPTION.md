# Summary

Implements the approved tooling binary and release bundle slice for
`feat/tooling-binary-bundle`.

This MR adds a linux-x86_64 Mix release archive for the `agov` tooling and a
single AGF bundle archive that wraps the framework source archives plus the
tooling binary archive.

The framework source archive remains usable on its own. The tooling archive and
combined bundle are optional convenience artifacts, not prerequisites for
adopting AGF.

# Scope

- Adds Mix release configuration for `agov` with bundled ERTS.
- Adds a release overlay so unpacked `bin/agov` runs `Agov.CLI` directly.
- Adds `Agov.ReleasePackage` for tooling archive and bundle assembly.
- Adds `mix agov.tooling.build`.
- Adds `mix agov.bundle.build`.
- Adds CI preview jobs:
  - `agov:tooling:preview`
  - `agf:bundle:preview`
- Adds CI release jobs:
  - `agov:tooling:release`
  - `agf:bundle:release`
- Adds tests for tooling archive layout/provenance/checksum, bundle layout and
  `SHA256SUMS`, and missing-tooling-archive failure.
- Updates adopter-facing release/install wording so framework-only,
  tooling-only, and bundle release lanes are described consistently.
- Adds the Gate 2 handoff record:
  `docs/coordination/01KV7NDSZEMZ1G87JTBRGGWAVC-coder-tester.md`.

# Governance Trail

- Gate 1 approval:
  `docs/coordination/01KV7MABYEEK6R1N18J9M4S8JQ-governor-reviewer.md`.
- Implementation handoff:
  `docs/coordination/01KV7MAZC8PEX9XF60T8XXPYA2-governor-reviewer.md`.
- Gate 2 handoff:
  `docs/coordination/01KV7NDSZEMZ1G87JTBRGGWAVC-coder-tester.md`.
- Framework-only adoption note:
  `docs/coordination/01KVA0V3DTSXCGRS67GEH0SVQ1-coder-tester.md`.
- Review-fix note:
  `docs/coordination/01KVJZ98VPESN1YB9ATJQ3KXY4-coder-tester.md`.

# Implementation Notes

`mix agov.tooling.build` runs `mix release agov --overwrite` with `MIX_ENV=prod`,
then packages the release as:

`agov-<version>-linux-x86_64-otp-<otp_major>.tar.gz`

with:

- `agov-<version>-linux-x86_64-otp-<otp_major>.sha256`
- `agov-<version>-linux-x86_64-otp-<otp_major>-provenance.json`

`mix agov.bundle.build` wraps existing `dist/agf-<version>.tar.gz`,
`dist/agf-<version>.zip`, and the tooling archive as:

`agf-<version>-bundle.tar.gz`

with:

- `agf-<version>-bundle.sha256`
- bundled `SHA256SUMS`
- bundled `bundle-provenance.json`

The tooling binary archive was smoke-tested after unpacking:

```text
bin/agov --help
```

and printed the expected AGOV CLI usage.

# Verification

- `git diff --check` passed.
- `mix compile --warnings-as-errors` passed.
- `mix test` passed: 200 passed, 0 failures.
- `mix agov.validate` passed.
- `mix agov.dist.validate` passed.
- `mix agov.dist.build` passed.
- `mix agov.tooling.build` passed.
- `mix agov.bundle.build` passed.
- Unpacked `bin/agov --help` passed.
- `agov coord validate
  docs/coordination/01KVJZ98VPESN1YB9ATJQ3KXY4-coder-tester.md` passed.
- Gate self-test passed for both new release command sequences:
  - `CI_COMMIT_TAG=v9.99.99 mix agov.dist.check_tag && mix agov.tooling.build`
    exited 1 at the tag guard.
  - `CI_COMMIT_TAG=v9.99.99 mix agov.dist.check_tag && mix agov.bundle.build`
    exited 1 at the tag guard.

# Review Notes

- Generated archives are not committed.
- The tooling archive content check found no forbidden `agf/`, `agents-govern/`,
  `tmp/`, `.git/`, or `.env` paths.
- Release jobs run only on tags.
- Preview artifacts expire in one week.
- Release artifacts use `expire_in: never`.
- The full test run still emits two pre-existing warnings:
  - unused `@tag` before `describe` in `test/agov/coord_config_test.exs`
  - unused default argument in private `claim/3` in
    `test/agov/claim_tracker_test.exs`
- Local MkDocs strict build was not rerun after the review-fix docs commit
  because `mkdocs` is not installed in this shell.

# Out of Scope

- macOS, Windows, or ARM binaries.
- Burrito, escript, or Nerves packaging.
- Package registry publishing.
- Installer scripts or PATH manipulation.
- Release signing or Sigstore.
- SBOM generation.
