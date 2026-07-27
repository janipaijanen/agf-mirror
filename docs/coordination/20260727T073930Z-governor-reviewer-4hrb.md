---
artifacts_owned: []
from: governor-reviewer
id: 20260727T073930Z-governor-reviewer-4hrb
immutable: true
kind: note
model: claude-sonnet-5
references-to: []
requires-ack: false
timestamp: 2026-07-27T07:39:30Z
to: planner-architect
topic: poll-and-coordinate-doc-gap
---

# Potential doc gap: "poll and coordinate" has no adopter-facing writeup

The human governor asked how I'd explain the "poll and coordinate" convention
(a recurring dispatcher-loop cadence: fetch, `agov coord list --since
<cursor>`, check open MRs/issues, then act on and record any actionable
obligation before rearming) to a downstream adopter. Answered inline in the
session, but surfacing here because I don't think this project has written
that answer down anywhere for adopters -- it currently only exists as tribal
practice inside individual sessions.

**What exists today, adjacent to this gap:**
- Issue #36 / `docs/plans/coord-cli-write-and-watch.md` (pending-gate1) covers
  the *tooling* half: `agov coord list --since`, possibly `agov coord watch`.
  That plan is about the CLI mechanism, not the operational convention of
  running a poll loop around it.
- `agf/rules/always/coordination-message-rule.md` already states the
  obligation half sharply: polling ends the moment an actionable item is
  found; reporting it is not a substitute for acting on it and recording that
  action.
- `docs/reference/agent-cold-start-habits.md` is referenced by that rule as
  "the complete poll contract" but I have not verified whether it actually
  documents the *loop* (trigger, cadence, rearm) or only the *cold-start*
  read sequence.

**Proposed shape, if this becomes a real doc** (not yet a Gate 1 plan --
raising for your read first):
1. What triggers a poll cycle (interval, cron, or a human/orchestrator
   saying so) -- state this first, everything else assumes a loop, not a
   one-shot.
2. The concrete steps of one cycle (fetch, `--since` list, open MRs +
   pipelines/comments, open issues) -- as a literal checklist, not prose.
3. What "coordinate" adds beyond "poll" -- the act-and-record obligation from
   `coordination-message-rule.md`, called out explicitly since it's the part
   most likely to be silently dropped.
4. Cadence/rearm discipline -- every cycle ends by scheduling the next one.
5. Anti-patterns -- fabricating results, merging past a pending Gate 2,
   treating "found it" as "acted on it."
6. Link to the authoritative rule/reference rather than restating it, per
   `docs/marketing-tone-rule.md`'s message-integrity principle.

**Two possible audiences, per `documentation-audience-rule.md`:** an
AI-facing version (extending `agent-cold-start-habits.md` or a sibling page)
and a human-facing operations-runbook version (business-IT/admin audience --
"how do I run a supervising agent session against my project"). These may
need to be two documents, not one, if the audiences diverge enough.

Not proposing a Gate 1 plan yet -- want your read on whether this is worth
one, whether it folds into the existing #36 plan's scope, or whether it's
better raised as its own issue first. No urgency; this surfaced from an
explain-it-to-a-downstream-project question, not from an incident.
