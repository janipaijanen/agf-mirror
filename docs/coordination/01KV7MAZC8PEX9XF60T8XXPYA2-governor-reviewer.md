---
kind: handoff
from: governor-reviewer
to: coder-tester
id: 01KV7MAZC8PEX9XF60T8XXPYA2
timestamp: 2026-06-16T07:10:07Z
immutable: true
references-to:
  - 01KV7MABYEEK6R1N18J9M4S8JQ-governor-reviewer.md
  - 01KV7HEHXGZYPJSRRYA313R1JA-governor-reviewer.md
topic: tooling-binary-bundle-implementation-plus-paper-mustfix
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Apply must-fix to docs/academic/governing-the-governors.tex (snapshot sentence)
      per 01KV7HEHXGZYPJSRRYA313R1JA before merging docs/academic-paper-review-revision.
    - Implement elixir-tooling-binary-bundle per approved plan in docs/plans/elixir-tooling-binary-bundle.md.
  must_read:
    - docs/plans/elixir-tooling-binary-bundle.md
    - docs/coordination/01KV7HEHXGZYPJSRRYA313R1JA-governor-reviewer.md
    - docs/academic/governing-the-governors.tex
  do_not_touch:
    - tooling/elixir/test/agov/claim_tracker_test.exs
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Implementation handoff: tooling binary + bundle, plus academic paper must-fix

Two tasks. Complete in order.

## Task 1 — Academic paper must-fix (branch: docs/academic-paper-review-revision)

The Gate 2 approval at `01KV7H1C6M9SGMK7230V74SSMP` was conditioned on fixing
the snapshot sentence before merge. See `01KV7HEHXGZYPJSRRYA313R1JA` for the
exact fix. In brief:

Replace the sentence containing "after the Elixir migration and stale-handoff
reconciliation work had closed" with one of the two options from the must-fix
note. Amend the commit on `docs/academic-paper-review-revision` then merge to
main.

## Task 2 — Tooling binary + bundle (new branch: feat/tooling-binary-bundle)

Implement per `docs/plans/elixir-tooling-binary-bundle.md`. Gate 1 approved at
`01KV7MABYEEK6R1N18J9M4S8JQ`.

### Summary of work

1. Add Mix release configuration in `tooling/elixir/` (target: `agov`).
2. Add `mix agov.tooling.build` task: runs `mix release agov`, tarballs the
   result as `agov-<version>-linux-x86_64-otp-<otp_major>.tar.gz`, writes
   SHA-256 and provenance JSON. Derive OTP major version at build time from
   `System.otp_release()`.
3. Add `mix agov.bundle.build` task: assembles bundle archive from framework
   archives already in `dist/` and tooling binary archive, writes `SHA256SUMS`
   and `bundle-provenance.json`.
4. Add CI jobs to root `.gitlab-ci.yml`:
   - `agov:tooling:preview` / `agov:tooling:release`
   - `agf:bundle:preview` / `agf:bundle:release`
   Release jobs need `agf:dist:release` in `needs:` to ensure framework archives
   exist before bundle assembly.
5. Add tests for both tasks and the failure path (bundle.build fails cleanly
   when tooling archive is missing).
6. Gate-self-test: run `mix agov.dist.check_tag` with a deliberate
   VERSION/tag mismatch and confirm the new jobs abort before archive
   production. Include transcript in Gate 2 handoff.

### Artifact naming

- `agov-<version>-linux-x86_64-otp-<otp_major>.tar.gz`
- `agov-<version>-linux-x86_64-otp-<otp_major>.sha256`
- `agov-<version>-linux-x86_64-otp-<otp_major>-provenance.json`
- `agf-<version>-bundle.tar.gz`
- `agf-<version>-bundle.sha256`

### Safety

- Tooling binary archive must not contain `agf/`, `.env`, `.git/`, `tmp/`,
  `agents-govern/`.
- Release jobs run only on tags.
- Failed `check_tag` must prevent upload.

### Validation before Gate 2 handoff

```bash
cd tooling/elixir
mix deps.get
mix compile --warnings-as-errors
mix test
mix agov.dist.validate
mix agov.tooling.build
mix agov.bundle.build
ls -lh ../../dist/
```
