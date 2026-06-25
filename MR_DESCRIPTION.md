# docs: address learning backlog batch 4

Branch: `docs/learning-batch4`

## Summary

Three learning records moved to `status: implemented` by making the
recommended concrete changes.

| Learning ID | Change |
|---|---|
| `20260621T182100Z-9x4m` | `figures.md` — numeric-label unit rule added to Usage rules; coordination map example caption now explains parenthesized numbers are message counts in a captured channel sample |
| `20260622T111100Z` | `ADOPTING.md` §5.5 (new) — CI artifact hygiene guidance: expiry conventions per artifact class, package-registry-then-expire pattern, GitLab storage monitoring |
| `20260623T212425Z-roi1` | Status update only — `environment-variables.md` already contains the per-assistant `AGOV_TOKEN_ROI_LOG` guidance from a prior MR |

## Files changed

| File | Change |
|---|---|
| `agf/docs/site/reference/figures.md` | Numeric label rule + coordination map caption |
| `agf/ADOPTING.md` | New §5.5 CI artifact hygiene |
| `agf/learnings/foundry.yaml` | Three status: proposed → implemented |

## Remaining proposed after this MR

- L009 — tooling task (`agov learning` subcommand); cannot implement as docs
- Batch2+3 lines (`proposed` in this tree, `implemented` on main after !180/!181 merge)

## Review checklist

- [ ] `agov validate learnings` passes
- [ ] Figures.md rule is additive; no existing guidance contradicted
- [ ] §5.5 is accurate for GitLab CI artifact model
- [ ] No `Co-authored-by` trailers in commit
