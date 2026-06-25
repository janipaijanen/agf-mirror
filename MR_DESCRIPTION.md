# docs: implement deferred #17 proposals #6 and #8

Branch: `docs/deferred-17-proposals`

## Summary

Two deferred improvement proposals from external reviewer (GitLab issue #17),
tracked in learning `20260624T214609Z-30gz`, now implemented.

| Proposal | Change |
|---|---|
| #6 — Miniature coordination thread example | `coordination-protocol.md` — four concrete file snippets showing a Gate 1 plan, decision-record approval, Gate 2 handoff, and completion, forming a complete navigable chain |
| #8 — Configuration drift review step | `ADOPTING.md` §6.4 — four-area drift checklist added after the "Audit upstream changes on upgrade" block: agent definition divergence, rule customization points, skill source paths, schema version |

## Files changed

| File | Change |
|---|---|
| `agf/docs/site/docs/concepts/coordination-protocol.md` | Miniature example thread section |
| `agf/ADOPTING.md` | Configuration drift review on upgrade |

## Review checklist

- [ ] Example thread IDs are plausible timestamp format (not `COORD-YYYYMMDD`)
- [ ] Drift checklist is accurate and actionable
- [ ] No `Co-authored-by` trailers in commit
