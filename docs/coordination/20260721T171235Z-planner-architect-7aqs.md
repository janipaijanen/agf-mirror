---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - tooling/elixir/mix.lock
  must_read:
  - docs/plans/adr-tracker-coexistence-docs.md
  - agf/docs/site/docs/concepts/roadmap-and-tracker-coexistence.md
  next_agent: governor-reviewer
  status: ready
  task:
  - Review docs/plans/adr-tracker-coexistence-docs.md for Gate 1 approval
  - Review the new concept page for vendor-neutrality and evidence-boundary compliance
id: 20260721T171235Z-planner-architect-7aqs
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260721T170808Z-planner-architect-sps8.md
requires-ack: true
timestamp: 2026-07-21T17:12:35Z
to: governor-reviewer
topic: adr-tracker-coexistence-docs-gate1
---

# Gate 1: document AGF/ADR/tracker coexistence

Per governor request (chat with Claude, not yet in-channel): document how
AGF's plan-and-gate workflow coexists with roadmap/ticketing tools, since
the current workflow is iterative-incremental with no native roadmap-board
concept. Explicitly asked to name Asana and Trello alongside the more
commonly discussed GitLab/GitHub/Jira.

Branch `plan/adr-tracker-coexistence-docs`, commit `a709045`.

## What it does

New concept page `agf/docs/site/docs/concepts/roadmap-and-tracker-coexistence.md`
adds a third axis (*when*/priority) alongside the existing ADR (*why*) and
coordination-channel (*how*) split already stated in `adopter-faq.md` — links
to that split rather than restating it. Per-tool notes for GitLab Boards,
GitHub Projects, Jira, Asana, and Trello, with parallel/equal treatment (no
ranking). Explicit "what this page is not" statement up front: no
integration feature, no schema dependency, Level 1 conceptual pattern per
`implementation-honesty-rule` until real usage validates it.

Cross-linked from `adopter-faq.md`'s existing ADR section and `position.md`'s
"Next" list. New nav entry in `mkdocs.yml`.

## Verification performed

- `mkdocs build --strict --config-file agf/docs/build/mkdocs.yml` — clean,
  no new orphan pages, no broken links
- `agov plan validate docs/plans/adr-tracker-coexistence-docs.md` — OK
- `agov validate adopter-docs` — OK
- `git diff --check` — clean
- Self-check against `marketing-tone-rule`'s forbidden-pattern list: no
  vendor name within ~10 words of cost/complexity/bloat/lock-in/failure
  language; no unearned superlatives — both greps return zero matches

## Sub-agent/delegation concept note

Separately: the sub-agent/delegation concept thread (`k4mp`/`sg7t`) is
paused per governor reprioritization toward this work — not dropped,
`sg7t`'s review stands as the resumption point. See
`20260721T170808Z-planner-architect-sps8`.
