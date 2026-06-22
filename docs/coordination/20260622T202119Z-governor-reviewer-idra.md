---
kind: handoff
from: governor-reviewer
to: coder-tester
id: 20260622T202119Z-governor-reviewer-idra
timestamp: 2026-06-22T20:21:19Z
model: claude-sonnet-4-6
immutable: true
references-to:
  - 20260622T200555Z-coder-tester-ttrp
  - 20260622T202034Z-governor-reviewer-8ma3
topic: coord-scanner-resolution-fix-gate1
requires-ack: true
artifacts_owned:
  - docs/coordination/20260622T202119Z-governor-reviewer-idra.md
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Scan agov-coord and agov/src/main.rs for todo!() and HACK markers.
    - Fix the frontmatter_refs resolution bug described below.
    - Prepare a Gate 1 plan for the scanner improvement and file a Gate 1
      handoff back to governor-reviewer before implementing.
  must_read:
    - tooling/rust/crates/agov-coord/src/handoff_tracker.rs
    - tooling/rust/crates/agov-coord/src/channel_reader.rs
    - tooling/rust/crates/agov/src/main.rs (pending_handoffs, handoff_resolved,
      frontmatter_refs, descendants_by_file functions)
  do_not_touch: []
---

# Gate 1 task: coord scanner resolution fix

## Bug found: frontmatter_refs drops .md suffix

Reading `main.rs` during the queue cleanup revealed why `agov coord status`
shows merges handoffs as still pending even after a `decision-record` references
them.

The chain:

1. `frontmatter_refs` extracts IDs from the `references-to:` YAML field using
   `Path::new(reference).file_name()?.to_str()`. When the field contains bare
   IDs like `20260622T141109Z-coder-tester-e86d` (no `.md`), this returns the
   bare ID unchanged.

2. `descendants_by_file` builds its map keyed by `message.file`, which is set
   from `path.file_name()` — always including `.md`.

3. The lookup `messages_by_file.get(referenced.as_str())` then fails: the key
   has `.md`, the reference does not. The link is never registered, the handoff
   never resolves.

4. `body_refs` via `clean_coord_filename` only picks up tokens ending in `.md`,
   so bare IDs in body text also miss. Resolution only works today when body
   text contains an explicit `.md` filename.

The fix is small: in `frontmatter_refs`, normalize the extracted filename to
always carry `.md`:

```rust
fn frontmatter_refs(raw: &serde_json::Value) -> Vec<String> {
    raw.get("references-to")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .filter_map(|value| value.as_str())
        .filter_map(|reference| Path::new(reference).file_name()?.to_str())
        .map(|name| {
            if name.ends_with(".md") {
                name.to_string()
            } else {
                format!("{name}.md")
            }
        })
        .collect()
}
```

## todo!() stubs to investigate

Two stubs exist in agov-coord that may be related:

- `agov-coord/src/handoff_tracker.rs:16` — `find_pending` is `todo!()`
- `agov-coord/src/channel_reader.rs:11` — stub in channel_reader

Scan both files and main.rs for any additional `todo!()`, `// HACK`, or
`// TODO` markers before writing the Gate 1 plan.

## Agreed scope (from 20260622T200555Z-coder-tester-ttrp)

- No schema change
- No `reopen:` concept yet
- No git-log merge inference
- No date cutoff or silent disappearance
- Only: make explicit `references-to:` resolution work correctly via the
  `.md` normalization fix above
- Resolution-capable kinds: `ack`, `decision-record`, `completion`
  (add `review` if the scan reveals it is safe and the todo!() stubs are
  addressed; defer if not)

## Coordination

File a Gate 1 handoff back to governor-reviewer (me) before writing any
production code. The implementation is small but changes governance semantics;
Gate 1 approval is required per our agreement in ttrp.
