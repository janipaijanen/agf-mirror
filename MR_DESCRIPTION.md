# feat: add agov coord graph command

Gate 1 approved. Plan: `docs/plans/feat-coord-graph-command.md`

## What changed

- Added `agov coord graph` to the Rust CLI.
- Reused the existing coordination reader so the graph is based on valid
  coordination messages from the channel.
- Counts `(from, to, kind)` communication edges and emits either:
  - Graphviz `dot`;
  - plain text lines for terminal review.
- Added role-aware node styling:
  - `governor-reviewer` is visually emphasized;
  - `all` is rendered as a broadcast target;
  - other roles render as ordinary role nodes.
- Added five focused Rust tests for empty directories, edge counts, node
  styling, text output, and missing-directory errors.
- Added a Concepts docs page for the coordination protocol, including `agov
  coord graph` usage.
- Added the new docs page to the Concepts nav and linked it from `README.md`.

## Verification

- `cargo check -p agov`: clean before rebase.
- `cargo test -p agov`: clean before rebase.
- `mkdocs build --strict`: clean before rebase.
- `agov coord graph` on the live foundry channel produced valid dot output.

## Review Focus

Please verify:

- [ ] The command output is useful for reviewing role-to-role communication.
- [ ] The command does not require the Graphviz binary; it only emits dot text.
- [ ] The docs page explains the coordination protocol without adding new
      governance rules.
- [ ] The README mention is discoverable but not over-promoted.
- [ ] The rebase preserved current `main` docs and figure changes.
