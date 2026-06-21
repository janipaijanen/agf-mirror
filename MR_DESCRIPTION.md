# feat: add agov coord graph command

Gate 1 approved. Plan: `docs/plans/feat-coord-graph-command.md`

## What changed

- Added `agov coord graph` to the Rust CLI with `--dir` and `--format`
  arguments.
- Reused the existing coordination reader so the graph is based on valid
  coordination messages from the channel.
- Counts `(from, to, kind)` communication edges and emits either Graphviz `dot`
  text or human-readable terminal text.
- Added role-aware node styling:
  - `governor-reviewer` is visually emphasized;
  - `all` is rendered as a broadcast target;
  - other roles render as ordinary role nodes.
- Added five focused Rust tests for empty directories, edge counts, node
  styling, text output, and missing-directory errors.
- Added a Concepts docs page for the coordination protocol, including `agov
  coord graph` usage with a dated foundry-channel sample.
- Added the new docs page to the Concepts nav and linked it from `README.md`.

## Verification

- `git diff --check`: clean after rebase.
- `/home/devi/.cargo/bin/cargo test -p agov --test coord_graph`: 5 passed
  after rebase.
- `mkdocs build --strict`: clean after rebase.
- `agov coord graph` on the live foundry channel produced valid dot output.
- Rebased onto current `main` after !72 figure-legend restore.

## Review Focus

Please verify:

- [ ] `agov coord graph` reads only valid coordination messages.
- [ ] Dot output is deterministic and valid for Graphviz rendering.
- [ ] Text output is useful without requiring Graphviz.
- [ ] The docs do not imply Graphviz is a runtime dependency of `agov`.
- [ ] The README link and MkDocs navigation target the new protocol page.
- [ ] The rebase preserved current `main` docs and figure changes.
