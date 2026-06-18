Closes #2

**Framework version:** 2.0.4

## Summary

- Adopt four issue templates and three MR templates from agents-govern v1 into the AGF v2 repo; update all paths, tooling commands, and URLs for v2; remove all v1 references
- Fix issue #2: sequential IDs in coordination filenames and `seq` field cause guaranteed merge conflicts under parallel-branch and multi-agent workflows

## Changes

**GitLab templates (issue #2 unrelated)**
- `.gitlab/issue_templates/Bug Report.md`
- `.gitlab/issue_templates/External Review.md`
- `.gitlab/issue_templates/Learning.md`
- `.gitlab/issue_templates/Rule Proposal.md`
- `.gitlab/merge_request_templates/Default.md`
- `.gitlab/merge_request_templates/Learning Record.md`
- `.gitlab/merge_request_templates/Rule or Agent.md`

**Issue #2 fix — conflict-free coordination IDs**
- `agf/schemas/coordination-message.schema.json` — make `id` (ULID) required; deprecate `seq` with description noting it must not be used as a global counter
- `agf/templates/coord-handoff.md` — replace `seq:` with `id:` (pre-generated example ULID)
- `agf/templates/coord-decision-record.md` — same; update `references-to` to `COORD-YYYYMMDD-XXX` convention
- `agf/templates/coord-gate2-handoff.md` — same
- `agf/templates/coord-completion.md` — same
- `agf/templates/README.md` — replace sequential filename table with `COORD-YYYYMMDD-XXX.md` convention; explain the convention
- `agf/docs/site/docs/start/first-governed-change.md` — replace all `00001-...` hardcoded paths with `COORD-YYYYMMDD-XXX` examples; update copy instructions
- `agf/ADOPTING.md` — update `agov coord validate` example path

## Design decisions

**`seq` kept (deprecated) rather than removed** — existing adopter channels may have `seq` values; removing the field from the schema would break their validators. Marking it deprecated with an explicit warning in the schema description preserves backward compatibility while steering new adopters away from it.

**`COORD-YYYYMMDD-XXX` over role-in-filename** — sender role is already structured data in the `from:` YAML field. Embedding it in the filename (e.g. `PLANNER-YYYYMMDD-XXX.md`) would duplicate information. `COORD-` as the uniform prefix keeps filenames consistent and short.

**ULIDs over `PREFIX-YYYYMMDD-XXX` for the `id` field** — the schema already defined a ULID-pattern field. Reusing it avoids introducing a second ID format. The `COORD-YYYYMMDD-XXX` convention applies to filenames only, where human readability matters most.

## Checklist

- [x] No real project, product, or organisation names (anonymization policy)
- [x] All v1 repository references removed
- [x] `VERSION` references updated to `agf/VERSION`
- [x] `learnings/` path updated to `agf/learnings/`
- [x] Validation commands updated to `mix agov.validate.learnings` / `mix agov.validate.infosec`
- [x] `uv run agov-dist` removed — CI handles distribution builds in v2
- [x] `LESSONS-LEARNED.md` and `reviews/` references removed — paths do not exist in v2
- [x] All four updated coord templates pass `agov coord validate` (blocking: OK, advisory WARNs on missing `model` field — expected for example templates)
- [x] Changes do not relax governance rules, agent boundaries, or escalation triggers
