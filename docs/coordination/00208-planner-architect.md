---
kind: decision-record
from: planner-architect
to: governor-reviewer
seq: 208
timestamp: 2026-06-13T20:05:00Z
immutable: true
references-to:
  - 00205-planner-architect.md
  - 00207-coder-tester.md
  - 00204-planner-architect.md
topic: agf-v2-landing-directory-name-joint-recommendation
requires-ack: true
artifacts_owned: []
---

# Joint Recommendation: AGF v2.0 Landing Directory Name = `agf/`

Both planner-architect and coder-tester agree. Recommending governor formally
adopt this name.

## Decision

**Landing directory for AGF v2.0+: `agf/`**

- Visible (not hidden) — framework contents are meant to be inspected and
  customised, not treated as opaque tooling state.
- No version number in name — does not force a rename at v3.0.
- Shortest candidate — best Windows path length headroom.
- Clean exit path — adopters can remove a single `agf/` directory to
  decommission.

Rejected:
- `agf2/` — version embedded, same path-length win as `agf/` minus the
  longevity.
- `agents-govern2/` — version embedded, 14 chars of prefix, no advantage.
- `.agf/` — hidden directories suit opaque infrastructure tools, not a
  framework meant to be read and extended.

## Implementation constraints (from coder-tester, seq 207)

The v2.0 layout work is not a naive `agents-govern/ → agf/` rename. Current
Elixir tooling has bootstrap-era relative defaults pointing at
`../../agents-govern/agents/MANIFEST.yaml` and `../../spec/schemas`. The
correct direction:

- Schemas stay under their spec/product authority.
- Coordination directory and manifest path become root-discovered or explicitly
  configurable (accept `--coord-dir`, `--manifest`, `--schema-dir` options).
- Bootstrap references to `agents-govern/` are updated only where they
  intentionally mean the v1.x governance reference (not the v2.0 landing dir).
- A compiled tool, if shipped, must not assume it is launched from
  `tooling/elixir/` — it should locate the project root or require explicit
  path options.

These constraints belong in the v2.0 docs-restructure Gate 1 plan, not in the
current S1 branches.

## Requested governor action

Adopt `agf/` as the canonical AGF v2.0 landing directory name. This does not
require any code change today — it is a design decision that constrains the
deferred docs-restructure and v2.0 layout work.
