---
id: 20260621T085210Z-coder-tester-v7ks
kind: completion
from: coder-tester
to: governor-reviewer
timestamp: 2026-06-21T08:52:10Z
immutable: true
references-to:
  - 20260621T084000Z-coder-tester-p2dq.md
topic: docs-ai-acts-human-decides-rule
requires-ack: false
artifacts_owned: []
model: gpt-5-codex
completion:
  status: complete
  completed_task:
    - Clarified adopter install guidance with the rule of thumb "let the AI act; make the human decide."
    - Reworded the layout and rule-install steps so the assistant performs clerical setup and the human reviews decisions.
  artifacts:
    - agf/docs/site/docs/adopt/install-and-layouts.md
---

# Completion: AI acts, human decides wording

Updated the install guide to make the adoption rule of thumb explicit:
let the AI act; make the human decide.

The page already defaulted to assistant-prepared commissioning, but one later
step still sounded like manual file copying was the human's task. The revised
wording says the assistant moves files, creates directories, installs rules,
and prepares skeleton evidence; the human reviews the diff, chooses the
governor, accepts the codename, and owns risk-bearing rule choices.

## Verification

- `uvx --from 'mkdocs>=1.6,<2' --with 'mkdocs-material>=9.5,<10' --with 'mkdocs-awesome-pages-plugin>=2.9,<3' --with 'mkdocs-gen-files>=0.5,<1' mkdocs build -f agf/docs/build/mkdocs.yml --strict` passed.
- `git diff --check` passed.
