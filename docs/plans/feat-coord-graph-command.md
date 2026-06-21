---
id: feat-coord-graph-command
status: approved
gate1_approved_by: "governor-reviewer"
gate1_approved_at: "2026-06-21T16:05:52Z"
topic: feat-coord-graph-command
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: read-only CLI command + documentation; no secrets, auth, or security boundary changes
  - role: contribution-auditor
    status: not-applicable
    reason: no governance structure, role definition, or coordination schema changes
---

# Plan: agov coord graph — coordination channel communication map

## Intent

Add `agov coord graph` to the Rust CLI. The command reads the coordination
channel and emits a Graphviz dot graph showing role-to-role communication
flows, annotated by message kind. Showcase the command and the coordination
protocol on the documentation site and in README.md.

Motivation: the coordination channel already encodes every message's `from:`,
`to:`, and `kind:`. A visual map makes the multi-agent protocol tangible for
evaluators and adopters reading the docs site — it is a concrete proof of
structure rather than a prose description.

## Affected files

| File | Change |
|---|---|
| `tooling/rust/crates/agov/src/main.rs` | Add `Graph` variant to `CoordCommand`; add `coord_graph()` function |
| `agf/docs/site/docs/concepts/coordination-protocol.md` | New page: coordination channel concept, `agov coord graph` usage, embedded dot example |
| `agf/docs/build/mkdocs.yml` | Add new page to Concepts nav |
| `README.md` | Add `agov coord graph` to tooling quick-reference table; add one-paragraph coordination protocol description |

## Migration needed

No. New command, new docs page, README addition only.

## Command design: agov coord graph

```
agov coord graph [--dir <coord-dir>] [--format dot|text]
```

- Default `--dir`: `docs/coordination`
- Default `--format`: `dot`
- `dot` format: valid Graphviz dot output on stdout — pipe to `dot -Tsvg`
- `text` format: human-readable edge list (fallback for environments without graphviz)

### What the graph encodes

**Nodes:** every distinct role value seen in `from:` or `to:` fields across all
valid coordination messages. The sentinel value `all` is a valid node.

**Edges:** directed from `from:` to `to:`, one edge per `(from, to, kind)` triple,
labelled with the kind and message count. Example:
```
"planner-architect" -> "coder-tester" [label="handoff (12)"]
"coder-tester" -> "governor-reviewer" [label="handoff (8)"]
"governor-reviewer" -> "all" [label="decision-record (8)"]
```

Multiple kinds between the same pair are separate edges (different labels).

**Node styling:**
- `planner-architect`: shape=box
- `coder-tester`: shape=box
- `governor-reviewer`: shape=box, style=filled (distinguished)
- `all`: shape=ellipse (broadcast target, not a role)
- Unknown/custom roles: shape=box (default)

**Graph attributes:**
```dot
digraph agf_coordination {
  rankdir=LR;
  node [fontname="Helvetica", fontsize=11];
  edge [fontname="Helvetica", fontsize=9];
  // nodes and edges follow
}
```

### Implementation notes (for Codex)

- Reuse `read_valid_coord_messages()` (already used by `coord_status`, `coord_stale`)
- Extract `string_field(&message.raw, "from")` and `string_field(&message.raw, "to")`
  and `string_field(&message.raw, "kind")` — same helpers used elsewhere
- Build a `BTreeMap<(String, String, String), usize>` counting `(from, to, kind)`
- Emit dot to stdout (no file write — caller pipes)
- ROI telemetry: `record_tool_output("agov", args, "coord-graph", output.chars)`
- No new dependencies needed

### Dispatch (in `run()`)

```rust
Command::Coord {
    sub: CoordCommand::Graph { dir, format },
} => coord_graph(dir, format, &args),
```

## Documentation: new page

**File:** `agf/docs/site/docs/concepts/coordination-protocol.md`

Page structure:
1. **What the coordination channel is** — append-only YAML-frontmatter markdown
   files; every message has `from:`, `to:`, `kind:`, `topic:`, `timestamp:`
2. **How agents use it** — handoffs, decision records, notes, completion entries;
   one agent writes, another reads via `agov session resume`
3. **Generating the communication map** — `agov coord graph | dot -Tsvg > map.svg`
   with a pre-rendered SVG inline or a fenced dot code block as example
4. **Message kinds reference** — table of `handoff`, `decision-record`, `note`,
   `completion`, `review`

**Tone:** concepts audience (developers and adopters). Per documentation-audience-rule:
technical, factual, honest about maturity.

The pre-rendered example should use actual message counts from the AGF-NG foundry
channel so the page is grounded in real data, not invented numbers.

**mkdocs.yml nav addition:**
```yaml
  - 'Concepts':
      - Roles and gates: docs/concepts/roles-and-gates.md
      - Coordination protocol: docs/concepts/coordination-protocol.md
      - Evidence and learnings: docs/concepts/evidence-and-learnings.md
```

## README.md changes

1. In the tooling quick-reference table (around line 71), add a row:
   ```
   | `agov coord graph` | Emit Graphviz dot map of role-to-role coordination flows |
   ```
2. Below the existing "coordination channel" sentence in the intro block, add
   one sentence linking to the docs page:
   > See [Coordination protocol](https://janipaijanen.gitlab.io/agents-govern-framework/docs/concepts/coordination-protocol/)
   > for how agents communicate and how to render the channel as a communication map.

## Test strategy

- Unit: `agov coord graph` on an empty channel emits a valid (empty) dot graph
  (no panic, exit 0)
- Unit: `agov coord graph` on a fixture with known messages emits expected edges
- `cargo check -p agov` passes
- `mkdocs build --strict` passes with new page
- Validate the dot output is parseable: pipe through `dot -Tsvg /dev/stdin` in
  the test or check for `digraph` sentinel

The test fixture approach mirrors `tooling/rust/crates/agov/tests/usage_report.rs`
(creates temp dir with sample files, calls the function, asserts on output).

## Risks

- None blocking. No schema change, no coordination format change, no security
  surface. Pure read path + new doc page.
- `dot` binary availability: the command emits dot text only; it does not shell
  out to graphviz. Rendering is the caller's responsibility. Document this clearly.

## Rollback

Revert is trivial: remove the `Graph` variant, the function, and the doc page.
No migration needed.
