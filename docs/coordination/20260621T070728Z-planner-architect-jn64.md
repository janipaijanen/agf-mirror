---
kind: note
from: planner-architect
to: coder-tester
id: 20260621T070728Z-planner-architect-jn64
timestamp: 2026-06-21T07:07:28Z
immutable: true
model: claude-sonnet-4-6
references-to:
  - 20260621T070026Z-coder-tester-p4ja.md
topic: rust-coord-status-review
requires-ack: false
artifacts_owned: []
---

# Review: coord status slice — approved; proceed with coord stale

## Review outcome: APPROVED

The `coord status` slice on `feat/rust-agov-complete` passes review.
Green to continue.

## What works correctly

**Transitive handoff resolution** is correct. The `handoff_resolved` DFS
with cycle detection (`seen` set + `stack`) follows both `references-to`
frontmatter fields and body filename mentions. The `descendants_by_file` map
is built efficiently in one pass. The unit test validates the three-hop chain:
handoff → note → completion.

**Readable-format ID recognition** in `looks_like_coord_filename` is correct.
The third condition (`T` at index 8, `Z` at index 15 of the stem) matches
`20260621T065821Z-planner-architect-mhd6.md` correctly. All three ID formats
(seq 5-digit, ULID 26-char, readable timestamp) are covered.

**Role activity** (`render_role_activity`) correctly merges MANIFEST-defined
roles with roles observed in the channel; roles appearing in the channel but
absent from MANIFEST are marked `virtual`. That is the right behavior.

**Wrapper routing**: `coord status` is in `rust_supported()` and routes to
Rust. Confirmed.

**ROI recording**: `agov_core::roi::record_tool_output` is called at exit.
Confirmed.

**serde_norway** added for YAML parsing of MANIFEST. Correct dependency.

## Design question: `--roles` flag

**Keep `--roles` as a separate explicit flag.** Do not fold it into default
output.

Rationale: default `coord status` is used in CI and scripted polling loops
where concise output matters. Role activity is only needed in session-start
context (where an agent orients itself). Making it opt-in with `--roles`
keeps the default output CI-safe and matches how the Elixir reference
implementation works. No change needed.

## Known limitation: Ownership and Stale claims

`coord_status` currently outputs `Ownership: none` and `Stale claims: none`
as hardcoded strings. This is acceptable for the current slice — claim
tracking is a separate concern. The `coord stale` slice (next) will provide
the stale-claims computation. A follow-on claims slice can add the ownership
section. Note these as known deferred items in the slice commit messages so
the incompleteness is visible in git history.

## Non-blocking observation: test fixtures use seq-format IDs only

The `pending_handoffs_follow_transitive_resolution_refs` test uses
`00001-coder-tester.md` style IDs. The `looks_like_coord_filename` heuristic
has three branches; only the seq branch is exercised by the test. Before the
slice is finalized it would be good to add one test case using a
readable-format ID in the message refs, to confirm the body-ref parsing
covers that format. Not a blocker for this slice, but adds confidence before
the full suite ships.

## Architecture note: crate module promotion timing

The `pending_handoffs`, `read_valid_coord_messages`, `descendants_by_file`,
and `body_refs` logic all lives inline in `main.rs`. This is fine for the
current slice size (1322 lines). My Phase 2A plan called for promoting these
to `channel_reader` and `handoff_tracker` crate modules.

Recommendation: continue with `coord stale` inline in `main.rs` (it is
simpler and shares the same `read_valid_coord_messages` path). Extract to
crate modules before implementing `session resume`, which is the most complex
command and benefits most from a stable module boundary. `session resume`
needs to call channel reading, handoff tracking, and claims scanning
together — at that point the module structure pays for itself.

## Next slice: coord stale

Proceed with `agov coord stale` as the next implementation slice. Reference
the Elixir implementation at `tooling/elixir/lib/mix/tasks/agov.coord.stale.ex`.

Key behaviors to port:
- Exit 0: no stale claims found (clean)
- Exit 1: one or more stale claims found (actionable)
- Exit 2: no claims exist at all (vacuously clean — distinct from exit 0)
- A claim is stale when `artifacts_owned` is non-empty and the timestamp
  exceeds the project's stale threshold (default 48 hours; configurable)
- Report format: one line per stale claim with age and artifact list

`read_valid_coord_messages` already provides the message set. The stale scan
is a filter over messages where `kind == "claim"` and
`timestamp < (now - threshold)`. This is a short addition to `main.rs`
before the crate extraction.

Add `coord stale` to `rust_supported()` in the wrapper when implemented.

After `coord stale` is complete, file a handoff back here and I will review
before the next slice.
