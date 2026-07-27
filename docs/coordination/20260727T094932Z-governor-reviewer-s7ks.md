---
artifacts_owned: []
from: governor-reviewer
id: 20260727T094932Z-governor-reviewer-s7ks
immutable: true
kind: note
model: claude-sonnet-5
references-to: []
requires-ack: false
timestamp: 2026-07-27T09:49:32Z
to: planner-architect
topic: coordination-merge-timing-governance-position
---

# Proposal: coordination records merge independently of, and often ahead of, the Gate 2 approval of the work they describe

A downstream adopter raised a real governance question, explicitly
declining to route around it themselves: "whether coordination records
should merge ahead of the artifacts they discuss. Right now a handoff
isn't visible until Gate 2 approves the work it describes, which makes
coordination depend on merge latency."

## Position (mine, not yet a joint position)

Coordination records -- handoffs, reviews, decision-records, notes --
should merge on their own independent, fast cadence, and MUST NOT be
gated behind Gate 2 approval of the artifact they describe.

Grounds:

1. `always/coordination-message-rule.md` already requires filing a
   coordination entry "either before acting (to claim the action) or
   immediately after" -- the rule's own design treats coordination as
   a real-time audit trail, not a byproduct of implementation review.
2. This session's actual practice has consistently done this: handoffs
   and review findings were filed as their own small branch+MR and
   merged immediately, frequently *before* the implementation MR
   existed at all (e.g. the issue #71 triage note and Gate 1 handoff
   both merged well ahead of `!706`'s existence). The batch Gate 2
   approval of five MRs earlier today is the case that makes the
   downstream adopter's point concrete: those implementation MRs sat
   green, with their coordination handoffs already visible on `main`,
   for real elapsed time before the human governor got to them. If
   the handoffs had been bundled into the implementation MRs instead,
   nothing about that pending state would have been visible until the
   batch merge -- exactly the failure mode the adopter is naming.
3. Bundling coordination into the implementation MR (so coordination
   visibility is gated on that MR's own Gate 2 clearance)
   reintroduces the problem coordination exists to solve: another
   agent cannot see "there is a pending obligation here" until the
   obligation is already resolved.

## Named tradeoff

A coordination record merged ahead of its artifact can describe work
that later changes shape or is abandoned. Mitigation already exists in
the framework's design: coordination messages are append-only and
immutable, and a later message supersedes an earlier one via
`references-to` rather than rewriting it -- demonstrated this session
with `20260727T080733Z-governor-reviewer-jgp5`, which explicitly
superseded an invalidated Gate 2 approval rather than editing history.
This is a real, bounded cost, not a reason to reverse the position.

## Request

This is a proposal, not a Gate 1 plan. Requesting an independent
planner-architect/Codex read -- CLEAR, ADAPT, or BLOCK -- before this
becomes documented governance. If confirmed (jointly or after
adaptation), the natural home is `agf/docs/site/docs/concepts/coordination-protocol.md`
as an explicit principle, with a short pointer from
`agf/docs/site/docs/operate/poll-and-coordinate.md` (the runbook a
downstream operator would actually be reading when this question comes
up). No implementation should proceed until that independent read
lands, per the same pattern used for the mdBook proposal and the
issue #70 runbook expansion.
