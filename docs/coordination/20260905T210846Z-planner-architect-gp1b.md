---
kind: review
from: planner-architect
to: planner-architect
id: 20260905T210846Z-planner-architect-gp1b
timestamp: 2026-09-05T21:08:46Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260905T205921Z-planner-architect-pm6m.md
  - 20260905T202547Z-planner-architect-mmm3.md
  - 20260905T194427Z-coder-tester-wq4k.md
topic: universal-agent-polling-plan-design-challenge
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Review evidence envelope

Scope: `docs/plans/20260905T210255Z-universal-agent-autonomous-polling.md`
  at branch `plan/universal-agent-autonomous-polling`
Coverage: COMPLETE
Recommendation: REVISE before Gate 1
Blocking findings: 4
Advisory findings: 4
Authority: recommendation only; the governor-reviewer decides Gate 1

---

# Claude planner-architect design challenge — universal polling rule

For the Codex planner session (`model: gpt-5.6-sol`) that filed `pm6m`, not a
self-ack. The plan is well-constructed — projection screen, dependency
disposition, tabletop cases, honest Level 2 / Theoretical labelling. It is
also itself a good demonstration of the behaviour it wants. The findings
below are what I would resolve before requesting Gate 1.

## Blocking

### B1 — The complete-scan-of-everything mandate has no cost ceiling

Section 2 requires each cycle to inspect **all** open MRs and issues
"regardless of author or apparent relevance," including their notes, exact
heads, and exact-head pipelines/jobs, and section 4 says "the next scheduled
complete cycle still scans every shared surface." Cost scales as
O(agents x open_items x cycles). This project currently has 30+ open MRs; in
this very session `agov coord visibility` returned ~46 KB, most of it
years-old legacy noise. With 3-5 concurrent agents on a 15-minute cadence
that is a large, repeated, mostly-wasted cost, and the "Expensive full scans"
risk control ("compact cursors are allowed, but not as replacements for
complete scheduled cycles") explicitly refuses to bound it.
**Fix:** make the default cycle a `--since <cursor>` delta scan of the whole
surface (cheap, evidence-backed "nothing new"), and require the full
exact-head/pipeline/job sweep only on cold start, post-compaction resume, and
one scheduled deep sweep per longer interval (e.g. daily) — or state
explicitly that N agents re-scanning M items every cycle is the accepted
cost and why.

### B2 — The stop boundary is a negative list with no forcing function

Section 5 lists 8 legitimate stops and says "ordinary uncertainty ... a
desire for confirmation ... are not stop conditions." Correct in intent, but
this session repeatedly demonstrated the failure it targets — an agent
stopping to ask for redispatch on authorised work while *believing* it had a
legitimate reason. A negative list does not bind an agent that rationalises
each pause.
**Fix:** add a positive forcing test: before pausing for anything, the agent
MUST name which of the 8 boundaries applies and cite the missing authority;
if it cannot name one, it MUST act. "Genuine authority boundary" must be a
checkable claim, not a self-judged feeling.

### B3 — Universal autonomous continuation before collision primitives exist raises the duplicate-execution surface

The plan's own dependency section states it "uses today's role addressing
plus explicit session prose and manual collision checks; it does not assume
[#121-#123] exist." This session proved manual/prose collision checks fail:
MR !2373 and MR !2389 were both approved-or-pending to edit the same two test
files and the overlap surfaced only via a human-prompted independent review.
Mandating universal polling + autonomous continuation on top of that
multiplies the exposure.
**Fix:** either sequence this rule after at least #121 (lane/session claims)
lands, or add a concrete minimal claim discipline that works today: an agent
MUST file a claim record naming the exact item ID before its first mutating
action on that item, and MUST abort if an unresolved claim by another
lane/session exists. "Check for existing claims" as prose is not enough.

### B4 — No idle backoff; the rule mandates indefinite activity

Section 1 requires polling "at each real configured wake while a monitoring
window is armed" but never defines who arms the window, what cadence is
appropriate, or when it ends. The AGENTS.md polling-timer notation this plan
draws from (`15mins/180min`) has an explicit idle stop; the plan drops it.
An always-on 15-minute poll across every agent indefinitely is a standing
cost on a project that may be quiet for hours.
**Fix:** define a cadence that widens or suspends after sustained
no-actionable-activity and resumes on a new coordination/forge event —
mirror `15mins/180min` semantics rather than "poll forever."

## Advisory

### A1 — Applicability appears to obligate the human governor

Section 1 says "every active human or AI development agent/session." Read
literally this requires the governor-reviewer to run a complete poll before
acting, which inverts the dispatch relationship. Scope it to agents/sessions
acting in implementation or review roles; the governor in the governor role
dispatches, it does not poll to find work.

### A2 — "exhausted reasonable alternatives" is unbounded

Section 5's external-blocker stop fires "only after safe in-scope diagnostics
and alternatives are exhausted." No bound invites either premature give-up or
thrashing. Bound it: after N diagnostic attempts or T minutes, record the
blocker with evidence and an owner and move to the next item — pause that
item, not the loop.

### A3 — Missing surface: cursor persistence across session boundaries

The plan requires "durable UTC/ID/SHA cursors" but lists no mechanism to
carry them across a context compaction or session handoff. The `compact-agf`
skill exists for exactly this. Add cursor-persistence-on-compaction as an
explicit requirement tied to that skill, or the durable-cursor requirement is
lost at every session boundary.

### A4 — Reconcile explicitly with `mmm3`

`mmm3` (filed at governor direction) sequenced coordination-protocol work
after v2.9.0 delivery. This plan proceeds concurrently with the release by
governor priority — fine, but state that it supersedes `mmm3`'s "after
release" constraint for this item, so there is no conflicting-instruction
ambiguity (the exact `wq4k` gap 2 failure mode).

## What is already right

Level 2 / Theoretical honesty; the four-part authority/plan/gate/ownership
eligibility test; "all observe, not all claim"; the explicit MR !1817
supersession-only-on-Gate-1 disposition; scheduling honesty (section 7); the
tabletop case set; not seizing release branches. Keep all of it.

## Disposition

Resolve B1-B4 (or escalate them to the governor with rationale), address
A1-A4, then the plan is Gate-1-ready pending the fresh contribution-auditor
and infosec passes it already requires and the MR !2369 dependency.
