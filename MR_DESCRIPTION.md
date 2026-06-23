# feat: agov coord graph-index — channel graph-index command (MR 1 of 3)

Branch: `feat/agov-coord-graph-index`

## Summary

Implements `agov coord graph-index [--dir DIR] [--output FILE]`:
a read-only command that emits a JSON graph of all coordination channel
messages, their causal edges, pending handoffs, and invalid files.

This is MR 1 of the three-MR phased proposal approved at
`20260623T181212Z-governor-reviewer-grph`. It covers only read-only
tooling and CoordMessage accessors; the session-resume integration and
`agov learnings index` follow in MR 2.

### Library changes (agov-coord)

- **`coord_validator.rs`** — added `CoordMessage` accessors: `kind()`,
  `from()`, `to()`, `topic()`, `timestamp()`, `requires_ack()`, `id()`,
  `references_to()`. Added public `normalize_coord_reference()` and
  `looks_like_coord_filename()` for reuse across crates.

- **`channel_reader.rs`** — implemented `read_channel(dir) → Vec<ChannelEntry>`:
  reads all `.md` files sorted by filename, validates each, collects
  failures without short-circuiting. Added `body_refs(path)` (scans body
  for coordination filename tokens) and `all_refs(path, msg)` (frontmatter
  + body, sorted, deduped). Four unit tests.

- **`handoff_tracker.rs`** — implemented `find_pending(descriptors) →
  Vec<PendingHandoff>`: finds handoffs with no resolving descendant.
  Resolution semantics: `ack`, `completion`, and `decision-record` kinds
  resolve a handoff; `note` does not. Uses DFS over a descendant map.
  Extended `PendingHandoff` with `file` and `pending_since` fields.
  Added `MessageDescriptor` struct for graph-index use. Four unit tests.

- **`Cargo.toml`** — added `tempfile` dev-dependency for tests.

### Binary changes (agov)

- Added `GraphIndex { dir, output }` variant to `CoordCommand`.
- Implemented `coord_graph_index()`: reads channel, separates valid/invalid,
  builds node/edge/pending lists, serialises to pretty-printed JSON.
- Added `graph_index_summary()`: deterministic one-line summary from first
  H1/H2 heading in body, falling back to `topic`.

### Output shape

```json
{
  "generated_at": "...",
  "nodes": [{ "id", "file", "kind", "from", "to", "topic", "timestamp",
               "requires_ack", "resolved", "summary" }],
  "edges": [{ "from", "to", "type": "references-to" }],
  "pending": [{ "id", "file", "from", "to", "topic", "pending_since" }],
  "invalid_files": [...]
}
```

`resolved` is `null` for non-handoff records, `true`/`false` for handoffs.
No `graph-index.json` is committed to `docs/coordination/` (append-only
audit trail constraint, confirmed in `75lm`).

### Smoke test (live channel, 636 files)

```
nodes: 309  edges: 521  pending: 2  invalid_files: 327
```

The 327 invalid files are legacy sequence-numbered messages that fail
current schema validation (same set the existing `agov coord status` skips).
The 2 pending handoffs are genuinely unresolved; the 9 previously shown
in the old binary were false positives from the coord-scanner-resolution bug
fixed in MR !125.

## Test plan

- [x] `cargo test -p agov-coord` — 10 tests pass
- [x] `cargo build -p agov` — clean build
- [x] `agov coord graph-index --dir docs/coordination` — live channel output correct
- [x] Pending handoff count consistent with post-!125 resolution semantics
