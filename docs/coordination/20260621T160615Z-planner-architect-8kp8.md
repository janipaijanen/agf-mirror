---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260621T160615Z-planner-architect-8kp8
timestamp: 2026-06-21T16:06:15Z
immutable: true
references-to:
  - 20260621T160552Z-planner-architect-ej3n.md
topic: feat-coord-graph-command
requires-ack: true
artifacts_owned:
  - docs/plans/feat-coord-graph-command.md
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Add agov coord graph subcommand to tooling/rust/crates/agov/src/main.rs
    - Create agf/docs/site/docs/concepts/coordination-protocol.md (new page)
    - Add Coordination protocol nav entry to agf/docs/build/mkdocs.yml
    - Add agov coord graph row and coordination protocol sentence to README.md
  must_read:
    - docs/plans/feat-coord-graph-command.md
    - tooling/rust/crates/agov/src/main.rs
    - agf/docs/build/mkdocs.yml
    - agf/docs/site/docs/concepts/roles-and-gates.md
    - README.md
  do_not_touch:
    - tooling/rust/Cargo.toml
    - agf/docs/site/docs/coordination/
    - agents-govern/
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 1 handoff: feat/coord-graph-command

Gate 1 approved. Plan: `docs/plans/feat-coord-graph-command.md`.
Implement on branch `feat/coord-graph-command` from `main`.

## What to build

### 1. `agov coord graph` command

**File:** `tooling/rust/crates/agov/src/main.rs`

Add to `CoordCommand` enum:

```rust
/// Emit a Graphviz dot communication map of the coordination channel.
Graph {
    #[arg(long, default_value = "docs/coordination")]
    dir: PathBuf,
    #[arg(long, default_value = "dot", value_parser = ["dot", "text"])]
    format: String,
},
```

Add dispatch in `run()`:

```rust
Command::Coord {
    sub: CoordCommand::Graph { dir, format },
} => coord_graph(dir, format, &args),
```

Implement `coord_graph()`:

- Call `read_valid_coord_messages(&dir)` (already used by `coord_status`)
- Extract `string_field(&message.raw, "from")`, `string_field(&message.raw, "to")`,
  `string_field(&message.raw, "kind")` for each message
- Count into `BTreeMap<(String, String, String), usize>` keyed by `(from, to, kind)`
- For `--format dot`: emit valid Graphviz dot to stdout:
  ```dot
  digraph agf_coordination {
    rankdir=LR;
    node [fontname="Helvetica", fontsize=11];
    edge [fontname="Helvetica", fontsize=9];
    // one node per distinct role, quoted
    "planner-architect" [shape=box];
    "coder-tester" [shape=box];
    "governor-reviewer" [shape=box, style=filled, fillcolor="#e8f0fe"];
    "all" [shape=ellipse];
    // one edge per (from, to, kind) triple
    "planner-architect" -> "coder-tester" [label="handoff (12)"];
  }
  ```
  - `governor-reviewer` gets `style=filled, fillcolor="#e8f0fe"` (approver distinguished)
  - `all` gets `shape=ellipse`
  - all other roles: `shape=box`
  - skip messages where `from` or `to` is missing
- For `--format text`: emit one line per edge:
  ```
  planner-architect -> coder-tester  handoff  (12)
  ```
- ROI telemetry: `agov_core::roi::record_tool_output("agov", args, "coord-graph", output.chars)`
- Exit 0 on success, 1 if directory not found

### 2. Documentation: new concepts page

**File:** `agf/docs/site/docs/concepts/coordination-protocol.md`

Page sections (audience: developers and adopters; tone: technical, factual):

```markdown
# Coordination protocol

*Status: Deployable (v2.0). Audience: developers and governors who want
to understand how agents communicate. After this page you can read a
coordination thread, run the graph command, and explain the message kinds.*

## What the coordination channel is

[append-only, YAML-frontmatter markdown files, one per message, in
`docs/coordination/`. Every message carries: `from:`, `to:`, `kind:`,
`topic:`, `timestamp:`, `id:`. No server, no database — the channel
is the directory.]

## How agents communicate

[Describe the handoff → implementation → Gate 2 → decision-record flow.
One agent writes a `handoff`; another reads it via `agov session resume`,
implements, files a Gate 2 `handoff` to governor-reviewer; governor
files a `decision-record`. Diagram with the kinds table below.]

## Message kinds

| Kind | Direction | Purpose |
|---|---|---|
| `handoff` | agent → agent | Pass work with deliverables |
| `decision-record` | governor → all | Gate approval or rejection |
| `note` | any → any or all | Status, polling, broadcast |
| `completion` | agent → all | Close a task |
| `review` | reviewer → all | Gate 2 findings |

## Generating the communication map

`agov coord graph` reads the channel and emits Graphviz dot:

    agov coord graph | dot -Tsvg > coordination-map.svg
    agov coord graph --format text   # no graphviz needed

Include a fenced dot block showing example output (use real output from
`agov coord graph --format dot` against the AGF-NG foundry channel, then
trim to a representative subset — 6–10 edges).
```

### 3. mkdocs.yml nav

**File:** `agf/docs/build/mkdocs.yml`

Add under Concepts:

```yaml
  - 'Concepts':
      - Roles and gates: docs/concepts/roles-and-gates.md
      - Coordination protocol: docs/concepts/coordination-protocol.md
      - Evidence and learnings: docs/concepts/evidence-and-learnings.md
```

### 4. README.md

**File:** `README.md`

a. In the tooling table (near line 71), add a row for `agov coord graph`:
   ```
   | `agov coord graph` | Emit Graphviz dot map of role-to-role coordination flows |
   ```

b. After the sentence "an append-only coordination channel" in the intro
   (around line 7), add one sentence:
   > The coordination protocol is file-based: no server required.
   > `agov coord graph` renders the channel as a Graphviz communication map —
   > see [Coordination protocol](https://janipaijanen.gitlab.io/agents-govern-framework/docs/concepts/coordination-protocol/)
   > in the docs.

   (Keep it tight — do not expand into a paragraph. The docs page carries the detail.)

## Verification checklist

- [ ] `cargo check -p agov` passes
- [ ] `agov coord graph` on an empty dir exits 0, emits `digraph agf_coordination { ... }`
  with no edges
- [ ] `agov coord graph` on a fixture with 3 known messages emits expected edges
- [ ] `agov coord graph --format text` emits human-readable lines
- [ ] `uv run --project agents-govern --group docs mkdocs build -f agf/docs/build/mkdocs.yml --strict` passes
- [ ] `./tooling/agov coord validate <new-coordination-files>` passes
- [ ] No Co-authored-by or AI-tool trailers in commits

## Do not

- Do not add a graphviz binary dependency to Cargo.toml
- Do not shell out to `dot` from the binary
- Do not modify the coordination schema or frontmatter format
- Do not touch `agents-govern/` (legacy v1 directory)
- Do not change `docs/coordination/` directory structure
