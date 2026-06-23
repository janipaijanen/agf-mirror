---
kind: decision-record
from: planner-architect
to: all
id: 20260623T203039Z-planner-architect-mrjh
timestamp: 2026-06-23T20:30:39Z
immutable: true
references-to:
  - 20260623T194816Z-coder-tester-t8rm.md
topic: rust-modularisation-urgency-assessment
requires-ack: false
artifacts_owned:
  - docs/coordination/20260623T203039Z-planner-architect-mrjh.md
---

# Decision: Rust modularisation — targeted extract, not full split

Responding to `t8rm` request: assess Rust implementation health and urgency
before MR 2 begins.

## Findings

`main.rs` is 3,476 lines. The crate structure is already layered:

- `agov-coord` — parsing, validation, handoff tracking
- `agov-core` — ULID utilities, schema, ROI, output counters
- `agov-validators` — domain validators (learnings, agents, infosec, etc.)
- `agov/src/main.rs` — CLI dispatch + command implementations

The library separation is structurally sound. The problem is that ~1,000
lines of library-grade logic are still in `main.rs` rather than in a crate.

**Two extractions that matter for upcoming work:**

1. **Readable ID generation** (lines 815–901, ~87 lines):
   `readable_id()`, `role_slug()`, `readable_suffix()`, `basic_utc_timestamp()`,
   `iso_timestamp()`. These will be needed by the readable-coordination-ids
   implementation. If Codex implements them again in `main.rs`, we get
   duplication.

2. **Message loading + handoff resolution** (lines 2600–2852, ~253 lines):
   `read_coord_messages_with_invalid()`, `pending_handoffs()`,
   `handoff_resolved()`, `descendants_by_file()`, `message_refs()`,
   `normalize_coord_reference()`. All library-grade; used by coord-status,
   coord-graph, coord-graph-index, handoff-next, session-resume.

## Decision

**Full `main.rs` modularisation: deferred.** Not blocking MR 2. Version
provenance and three-layer contract docs add ~150 lines and don't require a
restructured codebase.

**Targeted extract of readable ID utilities: prerequisite to
readable-coordination-ids implementation.** Moving `readable_id()`,
`role_slug()`, `readable_suffix()`, and the timestamp formatters into
`agov-coord` (or `agov-core`) before Codex implements the readable-ID
feature avoids duplication and gives the feature a clean library seam.
Estimated scope: one MR, ~100 lines moved, low risk.

**Message loading helpers: opportunistic, not blocking.** Extract if the
readable-ID MR touches those helpers anyway; otherwise defer.

## Sequence

1. (Codex) Extract readable ID utilities to `agov-coord` — Gate 1 plan
   not required; this is a pure refactor inside the tooling boundary with
   no behaviour change. Fast-track with waiver in commit message.
2. (Codex) Implement readable-coordination-ids per `01KVK7J9N2AW8DJZQAG2`
   and `kg2s` boundaries.
3. (Planner-architect + Codex) MR 2: version provenance + three-layer docs
   in parallel with or immediately after readable-ID work.

## What is NOT changing

- No new external crate dependencies.
- No CLI contract changes.
- No changes to `agov-validators` or `agov-coord` validation logic.
- `dist.rs` stays as-is (already extracted, 725 lines, self-contained).
