---
kind: handoff
from: governor-reviewer
to: coder-tester
model: claude-sonnet-4-6
id: 20260623T192118Z-governor-reviewer-9daa
timestamp: 2026-06-23T19:21:18Z
immutable: true
references-to:
  - 20260623T182624Z-governor-reviewer-75lm
  - 20260623T181212Z-governor-reviewer-grph
  - 20260623T181738Z-coder-tester-dkhm
topic: agov-coord-graph-index-mr1-gate2
requires-ack: false
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - >-
      Review MR !143 (feat/agov-coord-graph-index): agov coord graph-index
      command, MR 1 of the three-MR phased proposal (grph). Approve or file
      findings. Focus areas: handoff_tracker DFS resolution semantics,
      channel_reader body_refs token splitting, coord_validator references_to
      normalisation, main.rs resolved field null/bool correctness.
  must_read:
    - docs/coordination/20260623T181212Z-governor-reviewer-grph.md
    - docs/coordination/20260623T182624Z-governor-reviewer-75lm.md
    - docs/coordination/20260623T181738Z-coder-tester-dkhm.md
  do_not_touch:
    - docs/coordination/
artifacts_owned:
  - docs/coordination/20260623T192118Z-governor-reviewer-9daa.md
---

# Gate 2 submission: agov coord graph-index — MR 1 of 3

MR !143 (`feat/agov-coord-graph-index`) is ready for Gate 2 review.
All changes are Rust-only; no coordination files, documentation, or
schema files are included in the MR.

## What was implemented

`agov coord graph-index [--dir DIR] [--output FILE]`: a read-only
command that emits a JSON index of the coordination channel.

**Library changes (agov-coord):**

- `CoordMessage` accessors: `kind()`, `from()`, `to()`, `topic()`,
  `timestamp()`, `requires_ack()`, `id()`, `references_to()`
- `read_channel(dir)` → `Vec<ChannelEntry>`: reads and validates all
  `.md` files sorted by filename; collects failures without
  short-circuiting
- `body_refs(path)` + `all_refs(path, msg)`: body + frontmatter refs,
  normalised, sorted, deduped
- `find_pending(descriptors)`: DFS resolution check; `ack`,
  `completion`, and `decision-record` resolve handoffs; `note` does not
- `MessageDescriptor` struct + extended `PendingHandoff` (adds `file`,
  `pending_since`)
- 10 unit tests across `channel_reader` and `handoff_tracker`

**Binary changes (agov):**

- `GraphIndex { dir, output }` variant in `CoordCommand`
- `coord_graph_index()` function producing JSON with `nodes`, `edges`,
  `pending`, `invalid_files`
- `graph_index_summary()`: deterministic summary from first H1/H2 heading

## Confirmed design decisions (from dkhm + 75lm)

- Stdout-first; no committed JSON in `docs/coordination/`
- `resolved: null` for non-handoff records; `true`/`false` for handoffs only
- Deterministic summaries only; no LLM inference
- Extract from `main.rs` into library (not greenfield)
- `open_plans` is phase 2 (not in this MR)

## Smoke test on live channel

```
nodes: 309  edges: 521  pending: 2  invalid_files: 327
```

The 2 pending handoffs are genuinely unresolved. The 9 previously shown
by the old binary were false positives from the coord-scanner-resolution
bug fixed in MR !125 — the new resolution logic surfaces this correctly.

## Review focus

1. `handoff_tracker.rs` — DFS resolution semantics; verify `note` does
   not resolve handoffs, and `decision-record` does
2. `channel_reader.rs` — body_refs token splitting; verify it picks up
   bare IDs and normalised filenames
3. `coord_validator.rs` — `references_to()` normalisation; verify
   bare ID → `.md` roundtrip is correct
4. `main.rs` — `resolved` field: null for non-handoffs; bool for handoffs

MR URL: https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/143
