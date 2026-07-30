---
id: agov-mcp-transport-protocol-correctness
status: pending-gate1
topic: agov-mcp-transport-protocol-correctness
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >
      Issues #86, #88 independently reproduced and confirmed by
      contribution-auditor (`20260730T072926Z-contribution-auditor-rk2k`);
      #88 is an additional defect contribution-auditor found while
      reproducing #86. Decomposition and Gate 2 evidence scope from
      `20260730T074742Z-contribution-auditor-bhhh`.
  - role: infosec-sentinel
    status: consulted
    reason: >
      This plan touches the MCP stdio transport's request-validation
      boundary (#88) — the one place in `agov-mcp` that parses
      externally-supplied input before dispatch. No credential, secret,
      or permission-model change; the transport already only proxies to
      the same `agov` CLI subcommands with the same authority. Flagging
      for a pass on whether stricter JSON-RPC validation could itself
      become a new denial-of-service surface (e.g. malformed-input
      handling must fail closed with a JSON-RPC error, not panic or hang).
---

# Plan: agov-mcp transport completeness and protocol correctness (#86, #88)

## Trigger

Filed 2026-07-30 from the same session as the coordination read-path
issues (see `agov-coordination-read-path-reliability.md`): an agent
scanning the MCP tool list found no tool for `agov coord visibility` or
`agov coord graph-index`, both of which `AGENTS.md` names as the
prescribed cross-branch/cold-start tools. Independently reproduced by
contribution-auditor (`rk2k`), which additionally found and filed #88
(JSON-RPC request validity) while reproducing #86.

## Issues in scope

### #86 — `agov-mcp` exposes 9 of ~16 `agov` CLI commands

Confirmed via a live MCP session (9 tools:
`agov_coord_list`, `agov_coord_post`, `agov_coord_validate`,
`agov_handoff_complete`, `agov_handoff_next`, `agov_handoff_take`,
`agov_plan_validate`, `agov_session_resume`, `agov_validate`) against the
CLI's `agov coord` subcommand list (`validate, brief, post, list, status,
stale, visibility, graph, graph-index, publication-check` — 10 total, 3
mirrored). Correction from `rk2k`: state as "nine MCP tools total, three
of ten coordination subcommands exposed" rather than the looser "9 of
~16" framing. Additionally, the one `agov_session_resume` tool that *is*
exposed omits the CLI's `--manifest <MANIFEST>` flag — confirmed at
`tooling/rust/crates/agov-mcp/src/main.rs:390-396`, which forwards only
`role`, `base_branch`, `dir`.

### #88 — `agov-mcp` does not validate the JSON-RPC `jsonrpc` request member

`tooling/rust/crates/agov-mcp/src/main.rs:652-657`:

```rust
#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}
```

No `jsonrpc` field exists on the request struct, so it is never read or
checked. A request with `jsonrpc` absent entirely, or set to `"1.0"` or
any other value, is accepted and dispatched identically to a spec-correct
`"jsonrpc": "2.0"` request. This is a protocol-correctness gap against the
JSON-RPC 2.0 / MCP request contract, not a security boundary — the
transport still only reaches the same `agov` subcommands with the same
authority regardless of this field.

## Gate 1 decomposition

One plan, two reviewable implementation slices in sequence — per `bhhh`,
validate the transport contract first, then extend tool coverage on top
of a transport that is already contract-correct.

### Slice 1 — JSON-RPC 2.0 request validity (#88)

Add a `jsonrpc: String` field to `JsonRpcRequest` (or an equivalent
pre-dispatch check) and reject any request where it is absent or not
exactly `"2.0"`, returning a JSON-RPC error response (`-32600 Invalid
Request` per the spec) rather than silently dispatching. Must handle
notifications (`id` absent) per the spec — no response is sent for a
malformed notification, since notifications never receive a response by
definition. Must fail closed on malformed JSON itself (parse error,
`-32700`) without panicking or hanging the stdio loop, per the
infosec-sentinel note above.

### Slice 2 — coordination subcommand parity (#86)

Extend the MCP registry (`tools()` at `main.rs:523` and the matching
dispatch arms) to add MCP tools for the CLI's remaining `agov coord`
subcommands: `status`, `visibility`, `stale`, `graph-index`,
`publication-check`, `brief`, prioritizing `visibility` and
`graph-index` first since `AGENTS.md` explicitly names those two for AI
cold-start and cross-branch discovery. `graph` (Graphviz/dot output)
stays CLI-only per `bhhh`'s own scoping — a binary graph-rendering format
is not a natural MCP tool result shape. Add the missing `--manifest`
forwarding to `agov_session_resume`. Do not claim full parity from a
hand-maintained count: add an executable test that enumerates the CLI's
`agov coord --help` subcommands and the MCP registry's tool list and
asserts the intended coverage set matches, so future CLI additions cannot
silently drift out of sync with the MCP surface again. Any CLI command
deliberately left MCP-unexposed (e.g. `graph`) must be named in that test
with its reason, not just absent.

## Affected files

- `tooling/rust/crates/agov-mcp/src/main.rs` — `JsonRpcRequest` struct and
  request-handling loop (#88); `tools()` and the CLI-arg-building match
  arms (#86); `agov_session_resume` arm specifically for `--manifest`.
- `tooling/rust/crates/agov-mcp/tests/stdio.rs` — existing transport
  tests; add JSON-RPC validity cases.
- `tooling/rust/crates/agov-mcp/tests/roi_transport_attribution.rs` —
  check for interaction with new tool entries.
- New: a CLI-registry/MCP-registry contract test (exact location to be
  decided at implementation time — likely alongside the existing
  `tool_list_contains_only_cli_backed_tools` test at `main.rs:715`).
- `AGENTS.md` and `agf/docs/site/docs/adopt/mcp-server.md` — update the
  documented tool-to-CLI mapping once the new tools ship.

## Test strategy (Gate 2 evidence)

Per `bhhh`'s minimum evidence list — exercise all of:

1. Valid `"jsonrpc": "2.0"` requests (unchanged behavior).
2. Missing `jsonrpc` field — rejected with `-32600`.
3. Wrong `jsonrpc` value (e.g. `"1.0"`) — rejected with `-32600`.
4. Malformed JSON — rejected with `-32700`, no panic, no hang.
5. Notifications (`id` absent) — no response sent, per spec, for both
   valid and invalid notification requests.
6. Request IDs — echoed correctly in both success and error responses.
7. Tool discovery (`tools/list`) — includes all newly-added tools with
   correct schemas.
8. Argument forwarding — each new tool's MCP arguments map to the correct
   CLI flags (mirroring the existing `push_optional_flag`/
   `push_required_flag` pattern).
9. CLI/MCP result equivalence — same input via CLI and via MCP tool call
   produces the same output for each newly-added tool.
10. Authority preservation for mutating operations — `coord post` and any
    other mutating tool's authority/validation behavior is unchanged by
    this plan; this plan only adds read-path tools plus the JSON-RPC
    validity gate, it does not touch mutation authority.

## Risks

- Stricter JSON-RPC validation is itself a new code path that parses
  untrusted stdio input before any existing logic runs — must be tested
  against malformed/truncated/oversized input to confirm it fails closed
  rather than panicking (see infosec-sentinel note).
- Adding six new MCP tools increases the registry's maintenance surface;
  the contract test in Slice 2 is required specifically to prevent this
  plan's own fix from drifting out of sync again after the next CLI
  change.

## Rollback

Revert the governed commit/MR as a whole. The transport remains a stdio
JSON-RPC proxy with no persisted state; rollback has no data/migration
concern.

## Out of scope

- #84, #85, #89 (coordination read-path) — separate plan.
- #87, #90 (assistant discovery/archive) — separate plan; Slice C of that
  plan consumes this plan's final supported MCP tool list per `bhhh`'s
  sequencing note, so this plan's tool list should be finalized before
  that plan's Gate 2.
- `graph` (Graphviz/dot) MCP exposure — explicitly not added.
- Any v2.4.6 release action.
