---
id: 20260621T084000Z-coder-tester-p2dq
kind: completion
from: coder-tester
to: governor-reviewer
timestamp: 2026-06-21T08:40:00Z
immutable: true
references-to:
  - 20260621T082306Z-coder-tester-n1ns.md
topic: docs-adopter-friction-scan
requires-ack: false
artifacts_owned: []
model: gpt-5-codex
completion:
  status: complete
  completed_task:
    - Scanned adopter-facing documentation for manual setup friction similar to the first-governed-change template-copy issue.
    - Updated public docs so commissioning and first learning setup default to assistant-prepared diffs with human review.
    - Regenerated the adoption exit flow figure so the install step no longer says "copy rules".
  artifacts:
    - agf/docs/site/docs/adopt/install-and-layouts.md
    - agf/docs/site/docs/start/adopter-faq.md
    - agf/docs/site/docs/start/should-you-adopt.md
    - agf/docs/site/index.md
    - agf/docs/site/docs/index.md
    - agf/docs/site/evaluate.md
    - agf/docs/governance/manifest.md
    - agf/docs/governance/learning-system.md
    - agf/docs/figures/src/adoption-exit-flow.dot
    - agf/docs/figures/adoption-exit-flow.svg
    - agf/docs/site/reference/figures/adoption-exit-flow.svg
---

# Completion: adopter documentation friction scan

I scanned the public and shipped adopter documentation for the same adoption
friction that appeared in the first-governed-change page: asking the human to
perform clerical copying before they see framework value.

## Result

The main follow-up issue was the install and commissioning path. It still made
"copy files and customize rules" sound like the human's primary job. I changed
that path so the default instruction is: ask the assistant to prepare the
files, then have the human review the diff, make risk decisions, and resolve
local rule choices.

I also aligned the FAQ, landing page, documentation index, evaluation page,
learning-system guidance, and shipped manifest language with the same model.

## Verification

- `uvx --from 'mkdocs>=1.6,<2' --with 'mkdocs-material>=9.5,<10' --with 'mkdocs-awesome-pages-plugin>=2.9,<3' --with 'mkdocs-gen-files>=0.5,<1' mkdocs build -f agf/docs/build/mkdocs.yml --strict` passed.
- `git diff --check` passed.

The local shell emits a `mise` trust warning for this linked worktree before
commands run; it did not affect the docs build or diff check.
