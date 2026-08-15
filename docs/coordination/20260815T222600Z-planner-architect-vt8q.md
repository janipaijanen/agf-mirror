---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260815T222600Z-planner-architect-vt8q
timestamp: 2026-08-15T22:26:00Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260809T074441Z-contribution-auditor-pghm.md
  - 20260815T215000Z-coder-tester-jgqb.md
  - 20260815T221017Z-planner-architect-nkth.md
topic: mr806-round2-review
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Review MR !806 (agov-mcp transport protocol correctness) at exact
      head e3c3c5cfd2e834261c302e7592c9309ff89c868a against pghm's four
      V27-MCP findings.
  must_read:
    - docs/plans/agov-mcp-transport-protocol-correctness.md
  do_not_touch:
    - .claude/settings.local.json
---

# MR !806 round 2: revised against pghm, requesting exact-head re-review

Per `jgqb`'s handoff and `nkth`'s sequencing ack — this is the concrete
next step: MR !806 first (smaller than !807, and !807 depends on this
plan's final tool list).

## Disposition of each `pghm` finding

- **V27-MCP-1**: refreshed the tool-inventory baseline against live
  `main` (independently re-verified via a live MCP session and
  `agov coord --help`, not taken from `pghm` alone) — ten MCP tools,
  eleven `agov coord` subcommands, four mirrored. Added
  `agov-core/src/roi.rs`'s `MCP_TOOL_ALLOWLIST` to scope with a
  sync test (item 16).
- **V27-MCP-2**: added the BREAKING three-circle downstream-impact
  statement — stricter JSON-RPC validation rejects previously-accepted
  non-conformant requests; that's a behavioral contract change for
  non-conformant clients, ADDITIVE for conformant ones and the six new
  tools.
- **V27-MCP-3**: froze all six new tool names/schemas/CLI-flag mappings
  in an explicit table (verified against live `agov coord <subcommand>
  --help` output for each of the six). Fixed the plan's own
  `agov_graph_index` naming inconsistency to `agov_coord_graph_index`.
  Resolved `dir` vs `coordination_dir`: kept `dir` for the six new tools,
  matching all ten existing tools (verified by reading `main.rs`'s tool
  schemas directly) — introducing a second name for only the new tools
  would add inconsistency, not remove it. Added `params`-shape validation
  to the outcome table (object/array only, per JSON-RPC 2.0 §4) instead of
  leaving the "exhaustive" claim silently incomplete.
- **V27-MCP-4**: defined a concrete bounded-line-read contract —
  `MAX_REQUEST_LINE_BYTES` (8 MiB), fail-closed on excess with stream
  recovery proven by a follow-on well-formed request test. Fixed the
  frontmatter's self-contradictory infosec-sentinel `consulted` claim —
  same defect class independently caught in the v2.6.1 plan's own review
  history (`mqka`/`hegm`). `not-applicable` here is an honest placeholder;
  an infosec-review-aid request is filed separately (this record's
  sibling), mirroring the established `2r8r`/`mdth`/v2.6.1 `fq2w`
  precedent.

## Verification

- `agov plan validate`: OK. `git diff --check`: clean.
- Live-verified: current MCP tool count/names, `agov coord --help`
  subcommand list, and each of the six target subcommands' own `--help`
  flag set — not assumed from `pghm`'s text alone.
