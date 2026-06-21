---
id: 20260621T075410Z-coder-tester-lumo
kind: handoff
from: coder-tester
to: governor-reviewer
timestamp: 2026-06-21T07:54:10Z
immutable: true
references-to:
  - 00173-governor-reviewer.md
  - docs/plans/agents-govern-2.0-pedagogical-docs-restructure.md
topic: public-docs-one-page-introduction-polish
requires-ack: false
artifacts_owned: []
model: gpt-5-codex
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review and merge MR for the public docs copy polish derived from the Lumo one-page introduction draft.
    - Confirm the wording preserves provisional status, governance/runtime boundaries, and compliance caveats.
  must_read:
    - agf/docs/site/index.md
    - agf/docs/site/docs/start/should-you-adopt.md
  do_not_touch:
    - mise.toml
    - mise.lock
---

# Gate 2 handoff: public docs one-page introduction polish

I adapted the Lumo one-page introduction draft into the existing v2 public docs
without copying over its riskier claims.

## What changed

- Updated the home-page positioning line to "AI coding with human
  accountability."
- Added a compact "How it works" section covering role separation, the
  decision log, hard gates, and git-native operation.
- Expanded "Who benefits" to name solo maintainers and small teams explicitly.
- Added a practical adoption test to `Should you adopt?`.
- Added adopter-fit bullets for solo maintainers, small teams, and audit or
  compliance-sensitive work while preserving the caveat that AGF does not make
  a project compliant by itself.
- Added a skip condition for teams expecting AI self-regulation without a human
  governor.

## Claim discipline

I intentionally did not carry forward the Lumo draft's "Ready for Adoption"
status. The page still states v2.0 is Provisional. I also avoided claims that
AGF is a hosted service, agent runtime, security scanner, compliance product,
or universal AI-tool enforcement layer.

## Verification

- `uvx --from 'mkdocs>=1.6,<2' --with 'mkdocs-material>=9.5,<10' --with 'mkdocs-awesome-pages-plugin>=2.9,<3' --with 'mkdocs-gen-files>=0.5,<1' mkdocs build -f agf/docs/build/mkdocs.yml --strict` passed.
- `AGOV_SCHEMA_DIR=/home/devi/src/AGF-NG/.worktrees/docs-one-page-intro/agf/schemas AGOV_MANIFEST_PATH=/home/devi/src/AGF-NG/.worktrees/docs-one-page-intro/agf/agents/MANIFEST.yaml /home/devi/src/AGF-NG/tooling/rust/target/debug/agov coord validate docs/coordination/20260621T075410Z-coder-tester-lumo.md` passed.
- `git diff --check` passed.

The local shell emits a `mise` trust warning for this linked worktree before
commands run; it did not affect the MkDocs build result.
