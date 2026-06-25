# chore: mark AGENTS.md-addressed learnings as implemented

Branch: `chore/learning-status-batch2`

## Summary

Seven learning records addressed by !178 (AGENTS.md template cold-start
cluster fixes) are updated from `status: proposed` to `status: implemented`.

| ID | Recommendation implemented by |
|---|---|
| L020 | !178 — cold-start section now requires resume before any reads |
| L021 | !178 — timer claims without mechanism added to what-not-to-do |
| `20260621T084900Z` | !178 — filed = committed + pushed |
| `20260621T143908Z` | !178 — cold-start section (same fix as L020) |
| `20260622T111800Z` | !178 — coordination section (same fix as 20260621T084900Z) |
| `20260623T122924Z-kptn` | !178 — timer claim restriction |
| `20260623T192100Z-hab7` | !178 — post-resume coordination note instruction |

## Files changed

| File | Change |
|---|---|
| `agf/learnings/foundry.yaml` | Seven status: proposed → implemented |

## Review checklist

- [ ] agov validate learnings passes
- [ ] No Co-authored-by trailers in commit
