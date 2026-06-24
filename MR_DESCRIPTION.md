# feat: ship AGENTS.md adopter template for Codex/GitHub Copilot sessions

Branch: `feat/agents-md-template`

## Summary

Addresses learning `20260624T065743Z-ag7x` action 1. Adopters upgrading AGF
can now copy `agf/templates/AGENTS.md` to their project root to get a
cold-start–ready loader with:

- `agov session resume` prominently featured as the mandatory first command
- AGF version reference comment (`<!-- AGF: X.Y.Z -->`) — stale after upgrade,
  giving agents a visible signal to update
- Rule orientation table, coordination channel commands, gate and branch
  discipline summaries
- `<!-- CUSTOMIZE -->` markers throughout

Four files changed; no code or schema changes.

## Files changed

| File | Change |
|---|---|
| `agf/templates/AGENTS.md` | New — adopter AGENTS.md loader template |
| `agf/templates/README.md` | Added AGENTS.md row to the copy-to table |
| `agf/ADOPTING.md` | §1.4 step 2 names specific templates per assistant surface |
| `agf/learnings/foundry.yaml` | ag7x status → `partially-implemented`; action 2 deferred |

## What is NOT in this MR

- `agov validate` advisory for absent/stale loader files (ag7x action 2) —
  deferred. A green validate result still does not warn about a missing or
  stale `AGENTS.md`. Tracked separately before 2.2.0 final or 2.3.0.

## Review checklist

- [ ] Template cold-start section is complete and commands are correct
- [ ] ADOPTING.md §1.4 prose reads naturally for a first-time adopter
- [ ] Learning status update is accurate (`partially-implemented` + notes)
- [ ] No `Co-authored-by` trailers in commit
