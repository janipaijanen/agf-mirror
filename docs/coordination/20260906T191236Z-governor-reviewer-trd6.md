---
kind: decision-record
from: governor-reviewer
to: all
id: 20260906T191236Z-governor-reviewer-trd6
timestamp: 2026-09-06T19:12:36Z
model: human
immutable: true
references-to:
  - 20260906T185444Z-capability-developer-m4hr
  - 20260906T185910Z-capability-developer-rn5h
  - 20260906T182331Z-governor-reviewer-q8m5
  - 20260906T183954Z-contribution-auditor-qr7b
topic: improvement-propeller-capability-review-disposition
requires-ack: true
artifacts_owned: []
---

# Propeller capability proposal disposition: proceed as proposed

The governor-reviewer directs the work to **proceed as proposed** after the
completed capability-developer review `m4hr`:

1. **Proposal 1 is approved for planning/implementation progression.** Build
   the already Gate-1-approved Propeller Stage 3 atomic role/rule/docs/
   projection unit in planned/inactive state. This approval accepts the
   bounded advisory-role design and its pilot telemetry recommendation.
2. **Proposal 2 is deferred to a separate planner-owned item.** The
   build-engineer and fact-checker maturity-bump reassessment must not enter
   the Propeller implementation branch and receives no implementation
   authority from this decision.
3. **Proposal 3 is routed to planner-architect for collision analysis.** The
   assistant-facing role-status projection mismatch must not add a competing
   polling owner or unrelated validator scope to the Propeller branch.

Claude's independent audit under `rn5h` remains requested and may raise normal
findings for governor disposition; it does not duplicate the capability review
or silently enlarge implementation scope.

The approved plan's dependency remains binding: MR !2405's revised canonical
universal-polling rule must pass Gate 1 and merge **before or atomically with**
Propeller implementation. Because !2405 is not yet Gate-1-ready, the immediate
authorized action is to advance that prerequisite and prepare only work that
does not cross its unresolved scope boundary. Do not implement the Propeller
unit on a standalone branch that could merge ahead of !2405.

This decision does not authorize Stage 4 activation/launch, a heartbeat, Gate
2, merge of an implementation candidate, version bump, tag, or release.
