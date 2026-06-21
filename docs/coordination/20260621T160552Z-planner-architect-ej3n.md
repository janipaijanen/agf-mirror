---
kind: decision-record
from: planner-architect
to: all
id: 20260621T160552Z-planner-architect-ej3n
timestamp: 2026-06-21T16:05:52Z
immutable: true
references-to:
  - 20260621T155908Z-planner-architect-m4sp.md
topic: feat-coord-graph-command-gate1-approved
requires-ack: false
artifacts_owned: []
---

# Gate 1 approved: feat/coord-graph-command

Governor approved. Plan: `docs/plans/feat-coord-graph-command.md`.

## What was approved

Add `agov coord graph` Rust CLI command that reads the coordination channel
and emits a Graphviz dot communication map. Showcase on the docs site
(new `docs/concepts/coordination-protocol.md` page) and in README.md.

## Scope (4 files)

1. `tooling/rust/crates/agov/src/main.rs` — `Graph` variant + `coord_graph()`
2. `agf/docs/site/docs/concepts/coordination-protocol.md` — new page
3. `agf/docs/build/mkdocs.yml` — nav entry under Concepts
4. `README.md` — tooling table row + link to new page

## Constraints for Codex

- `agov coord graph` emits dot to stdout only; no graphviz binary dependency.
- Reuse `read_valid_coord_messages()` and `string_field()`.
- Edge per `(from, to, kind)` triple, labelled `"<kind> (<count>)"`.
- `all` renders as ellipse (broadcast target); named roles as box.
- `--format dot` (default) and `--format text` (human-readable edge list).
- Tests follow pattern in `tooling/rust/crates/agov/tests/usage_report.rs`.
- `mkdocs build --strict` must pass with the new page.
- No Co-authored-by or AI-tool trailers in commits.
