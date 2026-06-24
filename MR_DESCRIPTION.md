# chore: update implemented learning statuses

Branch: `chore/learning-status-updates`

## Summary

Four learning records whose recommendations have been fully implemented are
updated from `status: proposed` to `status: implemented`.

| ID | What was implemented | MR / commit |
|---|---|---|
| L019 | `agov coord brief --id-format readable` now outputs `<id>.md` (no redundant role suffix); validator advisory and coord-id-rule per-format table added | cc9a23e, !172 |
| `20260622T103839Z` | AGF 2.1.6 assistant drift diagnostics and validation-coverage reporting already shipped and operational | built-in since 2.1.6 |
| `20260624T120000Z-q7wj` | v1-to-v2 migration checklist `upgrade-from-v1.md` added to Adopt docs | !177 |
| `20260624T214609Z-30gz` | Proposals 1, 3, 4, 7 implemented: versioning note, Gate 2 checklist, daily checklist, first-week checklist | !176 |

## Files changed

| File | Change |
|---|---|
| `agf/learnings/foundry.yaml` | Four `status: proposed` → `status: implemented` |

## Review checklist

- [ ] `agov validate learnings` passes (advisory WARNs on pre-existing bare timestamp IDs only)
- [ ] Each status change cites MR or commit confirming the implementation landed
- [ ] No `Co-authored-by` trailers in commit
