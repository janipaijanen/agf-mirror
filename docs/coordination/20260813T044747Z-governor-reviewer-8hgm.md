---
kind: decision-record
from: governor-reviewer
to: all
id: 20260813T044747Z-governor-reviewer-8hgm
timestamp: 2026-08-13T04:47:47Z
model: human
immutable: true
references-to:
  - 20260813T041529Z-planner-architect-4jhq.md
  - 20260813T042318Z-contribution-auditor-tjhg.md
  - 20260813T043455Z-contribution-auditor-6dn4.md
topic: release-sequence-scope-freeze-approved
requires-ack: false
artifacts_owned: []
---

# Decision: release sequence and frozen discretionary scope approved

The governor approved the counterproposal stated in-session: “your proposal
is approved. please proceed.” This decision fixes the following release
sequence and scope rules.

## Numbered release scope

- **v2.6.0 — coordination reliability:** issues #84, #85, #89, and the
  bounded #76 work only.
- **v2.7.0 — MCP and assistant integration:** issues #86, #87, #88, and #90
  only.

These are frozen discretionary scopes. Optional wording from earlier drafts
does not add !341, !810, #103, or any other item to either release.

## Unmilestoned backlog and next candidate

- Issues #34, #35, and #107 form the next **un-numbered release candidate**.
  No v2.8.0 milestone or commitment is created by this decision.
- !341, !810, #33, #102, #103, #105, CI/evidence work, learning recovery,
  regulatory work, and all agentic-actor/OpenClaw research remain numbered
  tracker items but **unmilestoned release backlog**.
- New work must receive a normal tracker identifier but remain unmilestoned;
  “un-numbered” means no release number, never untracked work.
- OpenClaw remains outside every numbered AGF release. Tier 0/0.5 research is
  not authorized by this decision; Tier 1+ still requires a separate trust
  and authority decision.

## Scope-change boundary

No new discretionary item may enter v2.6.0 or v2.7.0. The only exception path
is a newly discovered release blocker, security defect, mandatory
compatibility correction, or prerequisite strictly necessary to deliver an
already-approved item. Every exception requires an explicit governor decision
with evidence and should use a scope swap rather than net expansion where
practicable.

Release scope may shrink: an item shown to be already fixed, unsafe,
unnecessary, or too large may be deferred by governor decision.

## Revalidation cadence

Revalidate the frozen scope:

1. before implementation/Gate 1;
2. before release-candidate acceptance; and
3. after each release, when the remaining sequence and unmilestoned backlog
   are reconsidered for adjustment or a newly numbered future release.

Revalidation does not itself authorize scope movement. Any release addition,
removal, swap, or new release number remains a governor decision.
