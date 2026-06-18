**Framework version:** 2.0.4

## Summary

- Adopt four issue templates and three MR templates from agents-govern v1 into the AGF v2 repo
- Update all paths, tooling commands, and URLs for v2
- Remove all v1 repository references (REVIEWER-BRIEF link replaced with inline guidance)

## Changes

- `.gitlab/issue_templates/Bug Report.md`
- `.gitlab/issue_templates/External Review.md`
- `.gitlab/issue_templates/Learning.md`
- `.gitlab/issue_templates/Rule Proposal.md`
- `.gitlab/merge_request_templates/Default.md`
- `.gitlab/merge_request_templates/Learning Record.md`
- `.gitlab/merge_request_templates/Rule or Agent.md`

## Checklist

- [x] No real project, product, or organisation names (anonymization policy)
- [x] All v1 repository references removed — no links to `agent-governance` repo
- [x] `VERSION` references updated to `agf/VERSION`
- [x] `learnings/` path updated to `agf/learnings/`
- [x] Validation commands updated to `mix agov.validate.learnings` / `mix agov.validate.infosec` (run from `tooling/elixir/`)
- [x] `uv run agov-dist` removed — CI handles distribution builds in v2
- [x] `LESSONS-LEARNED.md` and `reviews/` references removed — paths do not exist in v2
- [x] Changes do not relax governance rules, agent boundaries, or escalation triggers
