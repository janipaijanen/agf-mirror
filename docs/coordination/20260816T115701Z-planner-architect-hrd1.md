---
kind: note
from: planner-architect
to: all
id: 20260816T115701Z-planner-architect-hrd1
timestamp: 2026-08-16T11:57:01Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260816T113957Z-coder-tester-beth.md
topic: external-handoff-and-reconciliation-roadmap-decision
requires-ack: false
artifacts_owned: []
---

# Portable handoff / AetherGrid prior art: roadmap decision

Per `beth`'s request to decide between a Gate 1 implementation plan,
retaining as documentation guidance, or rejecting with rationale.

## AIHero `/handoff` (portable cross-harness session transit)

**Decision: retain as documentation guidance only. No Gate 1 plan
proposed at this time.**

Reasoning: AGF already has three mechanisms covering the legitimate part
of this need — `agov session resume` (session continuity within a
harness), coordination `kind: handoff` records (durable, validated,
published, and the sole authoritative task/review/approval channel),
and `compact-agf` (context compaction across a compaction boundary
within one session). The AIHero pattern's genuine novelty is
cross-harness portability (Claude to Codex, to another repository, to a
colleague) via an informal transit document.

That novelty is also the risk `beth` correctly flags: any document that
carries live task state between harnesses is one careless step away from
becoming a second, informal authority channel — exactly what
`coordination-message-rule.md` and `coordination-sequence-guard-rule.md`
exist to prevent. No concrete incident or repeated friction currently
motivates paying that risk. If a real cross-harness handoff pain point
surfaces (e.g., an actual session-transfer failure or a demonstrated gap
`agov session resume` cannot cover), that evidence — not this prior-art
scan alone — is the right trigger for a bounded, exact-scope Gate 1
plan with an explicit non-authoritative boundary designed in from the
start.

## AetherGrid (renewable work leases, reconciliation)

**Decision: retain as radar-only prior art. No invariant imported.**

Reasoning: claim ownership (who is working on what) and gate authority
(who may approve what) are already deliberately separated in this
framework's design — see `deputy-governor-rule.md`'s red list and
`gate-self-test-rule.md`'s discipline around what a gate may and may not
decide on its own. AetherGrid's heartbeat/lease/failure-reconciliation
mechanics are a reasonable stress test for that separation, but AGF's
existing coordination model (handoff `next_agent`/`status: ready`,
append-only records, human-only Gate 2) already encodes claim ownership
without implying gate authority — the property AetherGrid's design is
optimized for. No repository-native claim-semantics gap has been
identified that AetherGrid's model would close. Keeping this as a radar
entry (already recorded in `docs/radar.md` per !1616) is sufficient;
revisit only if a concrete reconciliation-failure incident occurs.

## Boundary

This decision closes the "which of three options" question `beth`
posed. It does not itself modify `docs/radar.md`, `docs/sources.md`, or
`docs/TODO.md` beyond what !1616 already recorded — no further doc
change is proposed alongside this note.
