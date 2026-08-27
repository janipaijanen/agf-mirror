---
artifacts_owned:
- tooling/rust/crates/agov-mcp/src/main.rs
- tooling/rust/crates/agov-core/src/roi.rs
- tooling/rust/crates/agov-mcp/tests/stdio.rs
- tooling/rust/crates/agov-mcp/tests/roi_transport_attribution.rs
from: planner-architect
id: 20260825T200906Z-planner-architect-j4jp
immutable: true
kind: claim
model: claude-sonnet-5
references-to:
- 20260825T200220Z-planner-architect-4dgq.md
requires-ack: false
timestamp: 2026-08-25T20:09:06Z
to: all
topic: mr806-implementation-claim
---

# Claim: implement MR !806 (agov-mcp transport completeness and protocol correctness)

Claiming implementation of the Gate-1-activated plan
`docs/plans/agov-mcp-transport-protocol-correctness.md` (exact head
`77ccdb1b`), per Gate 1 activation record `4dgq` and the human governor's
"proceed, gate is approved" instruction. Work proceeds on the existing
`plan/agov-mcp-transport-protocol-correctness` branch, implementing exactly
the two slices the plan specifies:

1. Slice 1 — JSON-RPC 2.0 request validity (#88): `jsonrpc` field
   validation, request-ID shape validation, notification semantics per the
   frozen three-row outcome table, `params` shape validation, and the
   bounded-line-read contract (`MAX_REQUEST_LINE_BYTES`).
2. Slice 2 — coordination subcommand parity (#86): six new MCP tools
   (`agov_coord_status`, `agov_coord_visibility`, `agov_coord_stale`,
   `agov_coord_graph_index`, `agov_coord_publication_check`,
   `agov_coord_brief`), `MCP_TOOL_ALLOWLIST` sync, `--manifest` forwarding
   for `agov_session_resume`, and the CLI/MCP registry contract test.

Following the plan's exact algorithm, error codes, and 16-item test matrix
verbatim — no design deviation. TDD throughout, per this project's
`tdd-rule.md`. Will not implement !807 in the same branch or commit.
Stops at Gate 2, per `tgjs`'s own boundary — does not merge, tag, release,
or claim Gate 2 authority.
