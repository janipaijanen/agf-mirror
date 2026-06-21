---
id: 20260621T090730Z-coder-tester-k4mj
kind: completion
from: coder-tester
to: governor-reviewer
timestamp: 2026-06-21T09:07:30Z
immutable: true
references-to:
  - 20260621T085210Z-coder-tester-v7ks.md
topic: docs-single-agf-carveout
requires-ack: false
artifacts_owned: []
model: gpt-5-codex
completion:
  status: complete
  completed_task:
    - Updated adopter documentation so framework-owned rules and skills stay under agf/.
    - Reframed ADOPTING.md as the assistant/operator execution guide and public docs as the human flow.
    - Replaced copied .claude/rules guidance with small tool-specific loader guidance.
  artifacts:
    - agf/ADOPTING.md
    - agf/docs/site/docs/adopt/install-and-layouts.md
    - agf/docs/site/docs/adopt/downstream-impact.md
    - agf/docs/site/docs/adopt/exit-and-decommission.md
    - agf/docs/governance/manifest.md
    - agf/rules/README.md
    - agf/skills/README.md
---

# Completion: keep AGF in a single removable carve-out

Updated the adopter and operator guidance to preserve the promised clean
carve-out: AGF-owned files stay under `agf/`. If an assistant needs root-level
configuration, that file is a small local loader that points at `agf/rules/`
or `agf/skills/`; it is not a copied framework rule or skill tree.

The human-facing docs now describe the flow and what the AI is expected to
produce. `ADOPTING.md` is explicitly framed as the assistant/operator practice
guide.

## Verification

- `uvx --from 'mkdocs>=1.6,<2' --with 'mkdocs-material>=9.5,<10' --with 'mkdocs-awesome-pages-plugin>=2.9,<3' --with 'mkdocs-gen-files>=0.5,<1' mkdocs build -f agf/docs/build/mkdocs.yml --strict` passed.
- `git diff --check` passed.
- A targeted scan for stale copy-rule/copy-skill instructions returned no matches.
