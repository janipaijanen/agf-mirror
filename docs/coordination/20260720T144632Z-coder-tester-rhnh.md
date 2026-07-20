---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260720T144632Z-coder-tester-rhnh
timestamp: 2026-07-20T14:46:32Z
model: codex-gpt-5
immutable: true
references-to:
  - docs/plans/docs-audience-boundary-proposal.md
  - 20260720T143255Z-governor-reviewer-s6hg.md
  - 20260720T144310Z-coder-tester-3h8h.md
topic: docs-audience-install-layouts-mr4-gate2
requires-ack: false
artifacts_owned:
  - agf/docs/site/docs/adopt/install-and-layouts.md
  - docs/coordination/20260720T144305Z-coder-tester-hpsg.md
  - docs/coordination/20260720T144310Z-coder-tester-3h8h.md
  - docs/coordination/20260720T144632Z-coder-tester-rhnh.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the MR-4 implementation branch docs/install-layout-boundary-mr4 for Gate 2.
    - Confirm the install-and-layouts page now separates the human layout decision from AI-executed commissioning steps.
    - Confirm the coordination handoff to Claude for MR-1 and Codex MR-4 claim are acceptable collaboration records.
  must_read:
    - docs/plans/docs-audience-boundary-proposal.md
    - docs/coordination/20260720T143255Z-governor-reviewer-s6hg.md
    - docs/coordination/20260720T144305Z-coder-tester-hpsg.md
    - docs/coordination/20260720T144310Z-coder-tester-3h8h.md
    - agf/docs/site/docs/adopt/install-and-layouts.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 2 handoff: MR-4 install-and-layouts split

Codex implemented MR-4 from the approved documentation audience-boundary plan
on branch `docs/install-layout-boundary-mr4`.

Implementation summary:

- moved the supported repository-layout decision before the assistant
  commissioning prompt;
- kept the `agf/` layout table and loader/copy-boundary warnings, but made them
  part of the human decision section instead of a later file-review step;
- reframed Step 3 as review of the assistant-prepared layout;
- added a visible "Human decision, assistant execution" note that separates
  human authority from AI clerical work;
- published a coordination handoff for Claude to start MR-1 and a Codex claim
  for MR-4.

Validation:

- `agov coord validate docs/coordination/20260720T144305Z-coder-tester-hpsg.md docs/coordination/20260720T144310Z-coder-tester-3h8h.md` passed.
- `git diff --check` passed.
- `uvx --from 'mkdocs>=1.6,<2' --with 'mkdocs-material>=9.5,<10' --with 'mkdocs-awesome-pages-plugin>=2.9,<3' --with 'mkdocs-gen-files>=0.5,<1' --with 'pymdown-extensions>=10.7,<11' mkdocs build -f agf/docs/build/mkdocs.yml --strict` passed.

MkDocs emitted only the existing Material for MkDocs warning and the already
known informational notices for unnaved `reference/figures.md` and
`reference/palette.md`.
