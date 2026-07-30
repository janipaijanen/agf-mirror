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

Exact-head review (`20260730T083948Z-contribution-auditor-1dq4`, findings
B3/B4) found this slice's first draft addressed only half of #88's own
stated contract and got one behavioral detail wrong. Both are corrected
here, not treated as new scope — #88's own Recommendation section
already required both.

**`jsonrpc` validation (as originally scoped).** Add a `jsonrpc: String`
field to `JsonRpcRequest` (or an equivalent pre-dispatch check) and
reject any request where it is absent or not exactly `"2.0"`, returning
`-32600 Invalid Request` with `id: null` (see below for why `id: null`
specifically) rather than silently dispatching.

**Request-ID type validity (B3, previously missing).** #88's
Recommendation explicitly also requires "Validate `id` against the
JSON-RPC 2.0 request-id types used by MCP" — the current
`id: Option<Value>` accepts any JSON value, including `null`, booleans,
arrays, objects, and fractional numbers. Per the MCP specification, a
request ID must be a string or a non-null, non-fractional-seeming
integer-valued number; `null` is reserved for error responses to
requests that themselves lack a valid ID and is not itself a legal
request ID. Add validation rejecting `id` shapes outside {string,
integer} with `-32600`, and add explicit test coverage for null,
boolean, array, object, and fractional-number IDs (five distinct
negative cases, not one generic "invalid ID" test). MCP's within-session
ID-uniqueness requirement is explicitly bounded out of this plan: the
transport is stateless per-request today and adding uniqueness tracking
would be new statefulness, not a request-shape validation fix: filed as
a follow-up recommendation, not silently dropped.

**Notification semantics, corrected (B4, then re-corrected per R2).** The
first draft conflated "any request with `id` absent" with "a
notification." The first correction fixed that but introduced a new,
sharper error: its "every other shape — valid or invalid, with or
without `id` — receives the applicable JSON-RPC error response" wording
literally routes a fully valid, ID-*bearing* request to an error
response too, which is wrong — a valid ID-bearing request must receive
its normal success or method-level response, exactly as today.

Frozen as three disjoint, exhaustive outcomes, replacing both prior
attempts:

| Request shape | Outcome |
|---|---|
| Structurally valid, `id` present (string or integer, per B3) | Normal response (success result or method-level error), echoing the same `id` |
| Structurally valid, `id` absent — a Notification | No response sent, ever, regardless of what the method would have returned |
| Structurally invalid for any reason (bad/missing `jsonrpc`, missing/wrong-type `method`, malformed JSON, or an `id` shape B3 rejects) — with or without `id` present | `-32600 Invalid Request` (or `-32700` for malformed JSON specifically) with `id: null` in the response |

"Structurally valid" means: parses as JSON, `jsonrpc` is exactly `"2.0"`,
`method` is present and a string, and if `id` is present it is a string
or integer (B3). Per the JSON-RPC 2.0 spec's normative examples, an
ID-less object that fails any of those checks is not a notification —
it's an invalid Request object, evaluated by the same rules as an
ID-bearing one, and returns row three's outcome with `id: null`, not "no
response." MCP's own notification definition likewise requires
`jsonrpc: "2.0"` and a string `method`. Test all three rows separately,
plus the specific edge case R2 named: a **valid** ID-bearing request
must not be misrouted to row three by an implementation that only checks
"is `id` present" without also checking overall structural validity.
Normative references checked 2026-07-30:
`https://www.jsonrpc.org/specification` and
`https://modelcontextprotocol.io/specification/2025-06-18/basic/index`.

Must fail closed on malformed JSON itself (parse error, `-32700`)
without panicking or hanging the stdio loop, per the infosec-sentinel
note above.

### Slice 2 — coordination subcommand parity (#86)

Extend the MCP registry (`tools()` at `main.rs:523` and the matching
dispatch arms) to add MCP tools for the CLI's remaining `agov coord`
subcommands: `status`, `visibility`, `stale`, `graph-index`,
`publication-check`, `brief`, prioritizing `visibility` and
`graph-index` first since `AGENTS.md` explicitly names those two for AI
cold-start and cross-branch discovery. `graph` (Graphviz/dot output)
stays CLI-only per `bhhh`'s own scoping. Correction from exact-head
review (A1): the original rationale here — "a binary graph-rendering
format" — is factually wrong; Graphviz DOT is a text format, not binary.
The actual reason to keep it CLI-only is that `graph-index`, already in
scope, is the compact machine/LLM-oriented result for the same
underlying graph, and full DOT output is comparatively large/verbose for
a tool-call result — not that DOT can't be represented as text.

**`graph-index` authority boundary (B5, previously undecided).** The CLI
supports `agov coord graph-index --output <file>`, which writes to a
caller-chosen filesystem path. This plan's stated "every new MCP tool is
read-only" claim did not decide whether that flag is exposed — if it
were, the MCP surface would let a caller write to an arbitrary host
path, which is a mutation, not a read. Frozen decision: the MCP
`agov_graph_index` tool does **not** expose `--output`; it always
requests stdout output and returns the JSON body as the tool result.
This is an intentional, documented exclusion, not an oversight, and the
CLI/MCP contract test below must assert it explicitly (a passing test
that merely doesn't cover `--output` is not the same as a test proving
it's absent).

Add the missing `--manifest` forwarding to `agov_session_resume`. Do not
claim full parity from a hand-maintained count: add an executable test
that enumerates the CLI's `agov coord --help` subcommands and the MCP
registry's tool list and asserts the intended coverage set matches, so
future CLI additions cannot silently drift out of sync with the MCP
surface again. Any CLI command or flag deliberately left MCP-unexposed
(`graph`, `graph-index --output`) must be named in that test with its
reason, not just absent.

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

1. Valid `"jsonrpc": "2.0"`, ID-bearing requests — normal success/method
   response, same `id` echoed back (row one of the outcome table; the
   exact case R2 found misrouted by an earlier draft).
2. Missing `jsonrpc` field, `id` present — `-32600`, `id: null` (row
   three; response `id` is `null`, not the request's `id`, since the
   request is structurally invalid).
3. Wrong `jsonrpc` value (e.g. `"1.0"`), `id` present — `-32600`,
   `id: null`.
4. Malformed JSON — `-32700`, `id: null`, no panic, no hang.
5. Invalid request-ID shapes (B3), otherwise-valid request — five
   separate cases: `null`, boolean, array, object, fractional number —
   each rejected with `-32600` and response `id: null`.
6. Valid notification (row two) — `id` absent, otherwise fully valid
   request object — no response sent, regardless of what the method
   would have returned.
7. Invalid ID-less object (row three, distinct from item 6) — `id`
   absent *and* another validity failure (malformed JSON, missing/wrong
   `jsonrpc`, missing `method`) — `-32600` (or `-32700` for the JSON
   case) with `id: null`, not silently dropped.
8. Request IDs — echoed correctly in row-one responses only, for both
   string and integer ID shapes; rows two and three never echo the
   request's own `id` (no response, or `id: null`, respectively).
9. Tool discovery (`tools/list`) — includes all newly-added tools with
   correct schemas.
10. Argument forwarding — each new tool's MCP arguments map to the
    correct CLI flags (mirroring the existing `push_optional_flag`/
    `push_required_flag` pattern).
11. `graph-index` authority boundary (B5) — the MCP tool's schema and
    dispatch path contain no `output`/file-path argument; a test
    attempting to pass one is rejected or ignored, proving exclusion
    rather than merely not exercising it.
12. CLI/MCP result equivalence — same input via CLI and via MCP tool
    call produces the same output for each newly-added tool.
13. Authority preservation for mutating operations — `coord post` and
    any other mutating tool's authority/validation behavior is unchanged
    by this plan; this plan only adds read-path tools plus the JSON-RPC
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
- MCP's within-session request-ID uniqueness requirement is explicitly
  not implemented by this plan (see Slice 1) — a client reusing an ID
  within a session is not detected. This is a known, named gap, not a
  silent omission; tracked as a follow-up recommendation since it would
  require new session-scoped state the transport doesn't have today.

## Rollback

Revert the governed commit/MR as a whole. The transport remains a stdio
JSON-RPC proxy with no persisted state; rollback has no data/migration
concern.

## Out of scope

- #84, #85, #89 (coordination read-path) — separate plan.
- #87, #90 (assistant discovery/archive) — separate plan. Sequencing
  correction (A2, both plans now consistent): that plan's shipped
  `.mcp.json` template contains no tool list, so there is no standing
  dependency on this plan finalizing first. The only coupling is
  conditional: if that plan's README or adopter docs ever name specific
  tool names in prose, that text must be re-checked against this plan's
  final tool list at that time.
- `graph` (Graphviz/dot) MCP exposure — explicitly not added.
- Any v2.4.6 release action.
