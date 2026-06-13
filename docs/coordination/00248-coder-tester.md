---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 248
timestamp: 2026-06-13T20:22:00Z
immutable: true
references-to:
  - 00236-coder-tester.md
topic: tooling-conformance-cli-gate2-renumbered-after-seq236-collision
requires-ack: true
artifacts_owned:
  - .worktrees/tooling-conformance-cli
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review branch feat/tooling-conformance-cli.
    - Decide whether to merge the tooling conformance draft and initial bin/agf wrapper.
    - Confirm whether this should remain separate from feat/tier1-provenance-labels.
  must_read:
    - agf/docs/contracts/tooling-conformance.md
    - agf/docs/site/reference/tooling-conformance.md
    - bin/agf
  do_not_touch:
    - agents-govern/
    - .worktrees/tier1-provenance-labels
    - .claude/settings.local.json
---

This record preserves and renumbers the tooling-conformance Gate 2 handoff that
was previously filed as `docs/coordination/00236-coder-tester.md` on `master`.

Reason for renumbering: `feat/tier1-provenance-labels` also contains a
different `docs/coordination/00236-coder-tester.md`, and its later approved
records cite that path as the provenance Gate 2 handoff. During the provenance
merge, the file-path collision must resolve in favor of the provenance branch
so those references remain coherent.

The tooling-conformance handoff remains live here under seq 248.

## Branch commit

- `5837363 feat: add AGF CLI conformance draft`

## Scope

- Adds `agf/docs/contracts/tooling-conformance.md` as the language-neutral AGF
  tooling compatibility contract.
- Adds public site reference page `agf/docs/site/reference/tooling-conformance.md`.
- Adds `bin/agf`, an executable wrapper exposing the adopter-facing command
  shape while delegating to the current Elixir Mix reference implementation.
- Updates public docs to refer to `agf` as the downstream command surface and
  Mix tasks as reference implementation details.

## Verification

From `.worktrees/tooling-conformance-cli`:

```bash
./bin/agf coord validate docs/coordination/00234-coder-tester.md
./bin/agf plan validate tooling/elixir/test/fixtures/valid_plan.md
./bin/agf handoff next governor-reviewer
MISE_TRUSTED_CONFIG_PATHS=/home/devi/src/AGF-NG/.worktrees/tooling-conformance-cli/mise.toml \
  mise exec -- uvx --from 'mkdocs>=1.6,<2' \
  mkdocs build -f agf/docs/build/mkdocs.yml --strict
cd tooling/elixir
MISE_TRUSTED_CONFIG_PATHS=/home/devi/src/AGF-NG/.worktrees/tooling-conformance-cli/mise.toml \
  mise exec -- mix test
```

Results:

- `agf coord validate`: OK.
- `agf plan validate`: OK against known valid fixture.
- `agf handoff next governor-reviewer`: no handoff found, exit 2.
- MkDocs strict build: pass.
- Mix tests: 129 passed.
