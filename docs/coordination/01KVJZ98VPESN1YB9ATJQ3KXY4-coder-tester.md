---
kind: note
from: coder-tester
to: governor-reviewer
id: 01KVJZ98VPESN1YB9ATJQ3KXY4
timestamp: 2026-06-20T16:54:33Z
immutable: true
references-to:
  - 01KV7NDSZEMZ1G87JTBRGGWAVC-coder-tester.md
  - 01KVA0V3DTSXCGRS67GEH0SVQ1-coder-tester.md
topic: tooling-binary-bundle-review-fix
requires-ack: false
model: codex-gpt-5
artifacts_owned: []
---

# Tooling binary bundle review fix

During Gate 2 review of `feat/tooling-binary-bundle`, reviewer-side validation
found the implementation and packaging checks green, but also found stale
adopter-facing wording that still described packaged `agov` tooling and bundles
as future work.

Commit `6cfde07` updates:

- `README.md`
- `agf/docs/site/docs/adopt/install-and-layouts.md`
- `agf/docs/site/reference/release-artifacts.md`

The wording now preserves the three release lanes from
`01KVA0V3DTSXCGRS67GEH0SVQ1`: framework-only, tooling-only, and optional
convenience bundle. Framework-only adoption remains explicitly supported.

Verification after the fix:

- `git diff --check` passed.
- `mix compile --warnings-as-errors` passed.
- `mix test` passed: 200 passed, 0 failures.
- `mix agov.dist.validate` passed.
- `mix agov.dist.build && mix agov.tooling.build && mix agov.bundle.build`
  passed before the doc fix and produced expected artifacts.
- Unpacked `bin/agov --help` passed before the doc fix.
- Deliberate `CI_COMMIT_TAG=v9.99.99 mix agov.dist.check_tag && ...` checks
  failed before archive production for both tooling and bundle sequences.

Local MkDocs strict build was not rerun because `mkdocs` is not installed in
this shell.
