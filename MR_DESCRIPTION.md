# chore: tighten AGENTS.md template cold-start and filing discipline

Branch: `chore/agents-template-cold-start-fixes`

## Summary

Addresses four learning-cluster gaps in the AGENTS.md template:

**1. session resume before any reads (L020, 20260621T143908Z)**
Previous text said "Never start implementing before running session resume."
Gap: agents ran reads (git log, file inspection, coordination reads) before
resume, missing pending messages. Fix: "before anything else — before reading
coordination files, before git log, before any file inspection."

**2. Post-resume coordination note (20260623T192100Z-hab7)**
After a context resume or cold start, agents proceeded silently without filing
a current-state note. Other agents saw no baseline for the new cycle. Fix:
added instruction to file a brief `kind: note` after resume before proceeding.

**3. Filed = committed + pushed (20260621T084900Z, 20260622T111800Z)**
"File the entry... never as an afterthought" was ambiguous about what "filed"
means. An unpushed commit is invisible to other agents. Fix: explicit note
that a message is not filed until committed AND pushed.

**4. Timer claims require a real mechanism (L021)**
No instruction existed against stating "I will check in N minutes" without a
mechanism that fires. Fix: added to the "What NOT to do" list.

## Files changed

| File | Change |
|---|---|
| `agf/templates/AGENTS.md` | Cold start, coordination, and what-not-to-do sections updated |

## Review checklist

- [ ] Cold start section covers reads, not just implementation
- [ ] Post-resume coordination note instruction added
- [ ] Commit + push filing requirement is explicit
- [ ] Timer claims added to what-not-to-do
- [ ] No `Co-authored-by` trailers in commit
