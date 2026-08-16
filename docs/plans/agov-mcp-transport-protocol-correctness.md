---
id: agov-mcp-transport-protocol-correctness
status: pending-gate1
topic: agov-mcp-transport-protocol-correctness
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >
      Round 5 refresh: the original `rk2k`/`bhhh` citations (2026-07-30)
      predated `pghm`'s fresh BLOCK (2026-08-09) and every fix made since.
      Non-self contribution-auditor re-review at the current exact head
      landed CLEAR, zero findings:
      `20260816T002624Z-contribution-auditor-n2j3.md` — "resolves `pghm`'s
      prior contribution-audit findings without expanding the two-issue
      boundary" — exact head `0d0d66fd062332fc320d7923ad3ee9c3cdeb045a`.
      Per `n2j3`'s own note, this citation-only correction does not
      require a further substantive contribution-auditor pass unless
      scope or reviewed substance changes.
  - role: infosec-sentinel
    status: not-applicable
    reason: >
      Round 2 correction, per `pghm`'s V27-MCP-4: the prior `consulted`
      status was the same self-contradictory pattern later independently
      found in the v2.6.1 plan — claiming a status whose own reason text
      only requests a future pass with no pinned result. infosec-sentinel
      has no configured human or AI player (`played_by: "ci (agov validate
      infosec, on commit)"` per `agf/agents/MANIFEST.yaml`), so there is no
      role-holder to "consult" the way contribution-auditor has one. This
      plan touches the MCP stdio transport's request-validation boundary
      (#88) — the one place in `agov-mcp` that parses externally-supplied
      input before dispatch, with a genuine bounded-input/DoS-surface
      question (see Slice 1's malformed-input handling and the input-safety
      risk item below). Per the established precedent (frozen-scope plan's
      `2r8r`/`mdth`; v2.6.1's `fq2w`), an infosec-review-aid pass from
      coder-tester plus a governor waiver decision is the path to
      resolution — requested separately, not asserted here before it
      exists. **This plan is not ready for Gate 1 while this entry is
      unresolved.**
---

# Plan: agov-mcp transport completeness and protocol correctness (#86, #88)

## Revision history

**Round 5 (this revision): citation-only refresh.** Round 4's bounds-check
unification was CLEAR per coder-tester
(`20260815T232500Z-coder-tester-85hh.md`) and per a fresh non-self
contribution-auditor pass (`20260816T002624Z-contribution-auditor-n2j3.md`,
replacing the 2026-07-30 `rk2k`/`bhhh` citations, which predated `pghm`'s
fresh BLOCK). No design change. infosec-sentinel remains the sole
outstanding prerequisite — a governor decision.

**Round 4, per coder-tester's exact-head review
`20260815T224602Z-coder-tester-83tn.md` (requested via `p3xn`, MR !1595):
BLOCK, 1 blocking (`B-V27-MCP-R3-1`).** Round 3's bounded-reader algorithm
split into separate "newline found" and "no newline found" branches with
inconsistent bounds checks — the newline-found branch never defined the
ordinary "this exact chunk's payload pushes the line over the cap for the
first time" case (the one-byte-over test itself). Fixed by unifying the
bounds check into one step applied identically regardless of whether a
newline was found in the current chunk — see Slice 1's bounded-line-read
contract, revised, and the new chunk-boundary test.

**Round 3, per coder-tester's exact-head review and infosec-review-aid
pass `20260815T222810Z-coder-tester-h6qr.md` (requested via `vt8q`/`hzwc`,
MR !1593): BLOCK, 1 blocking (`B-V27-MCP-R2-1`).** `V27-MCP-1`, `V27-MCP-2`,
and `V27-MCP-3` all confirmed `pass` — no design changes needed. The one
blocking finding: round 2's bounded-line-read design specified checking
`buf.len()` *after* `BufRead::read_until()` returns, but `read_until`
itself appends the entire delimiter-free line before returning — the
unbounded allocation the fix was meant to prevent already happened by the
time the check ran. Fixed by replacing `read_until` with a genuinely
bounded incremental reader built on `fill_buf()`/`consume()` directly,
which never allows the line buffer itself to exceed the cap even while
still consuming a hostile, arbitrarily-long, delimiter-free stream — see
Slice 1's bounded-line-read contract, revised, and validation-plan item
15. Still awaiting the separate governor decision on the plan-specific
infosec consultation waiver this aid-review's findings are evidence for
(frontmatter unchanged this round).

**Round 2, per contribution-auditor's fresh exact-head review
`20260809T074441Z-contribution-auditor-pghm.md` (governor decision `2gbg`):
COMPLETE/BLOCK against current `main` (`72b23940`), superseding the
2026-07-30 CLEAR evidence, which had gone stale. Four findings
(`V27-MCP-1..4`), all addressed in this revision:

- `V27-MCP-1`: tool-inventory baseline refreshed — current `main` has ten
  MCP tools (one more than this plan's original count: `agov_coord_pending`
  shipped from the coordination read-path plan since this plan was
  drafted) and eleven `agov coord` CLI subcommands (one more:
  `pending`), four of which are mirrored. Both counts independently
  re-verified against live `main` before writing this revision, not taken
  from `pghm` alone. `MCP_TOOL_ALLOWLIST`
  (`tooling/rust/crates/agov-core/src/roi.rs`) added to scope — see
  "Affected files".
- `V27-MCP-2`: added the BREAKING three-circle impact statement — see
  "Downstream impact" below.
- `V27-MCP-3`: froze all six new tool names, schemas, and CLI-flag mappings
  explicitly — see "Slice 2" and the new "Frozen tool contract" table.
  Corrected the plan's own inconsistent `agov_graph_index` reference (no
  `coord_` infix) to `agov_coord_graph_index`, matching the established
  `agov_coord_*` namespace throughout. Resolved the `dir` vs
  `coordination_dir` question: every one of the ten existing tools uses
  `dir` (confirmed by reading `main.rs`'s tool schemas directly); the six
  new tools use `dir` too, for consistency — introducing `coordination_dir`
  for only the new tools would create a second inconsistency instead of
  resolving the first. If a project-wide `dir`→`coordination_dir` rename is
  ever adopted, it applies to all sixteen tools in one coordinated change,
  not selectively. Added `params`-shape validation to Slice 1's outcome
  table instead of leaving the "exhaustive" claim silently incomplete.
- `V27-MCP-4`: defined a bounded-line-read contract with an explicit
  maximum, error behavior, and oversized-input test (see Slice 1). Fixed
  the frontmatter's self-contradictory infosec-sentinel `consulted` claim —
  the same defect class independently caught in the v2.6.1 plan's own
  review history; `not-applicable` here is an honest placeholder pending a
  real infosec-review-aid pass, not an assertion the plan doesn't need one.
  Refreshed the stale v2.4.6 exclusion language to the current v2.7.0 child-
  plan relationship (see "Out of scope").

## Trigger

Filed 2026-07-30 from the same session as the coordination read-path
issues (see `agov-coordination-read-path-reliability.md`): an agent
scanning the MCP tool list found no tool for `agov coord visibility` or
`agov coord graph-index`, both of which `AGENTS.md` names as the
prescribed cross-branch/cold-start tools. Independently reproduced by
contribution-auditor (`rk2k`), which additionally found and filed #88
(JSON-RPC request validity) while reproducing #86.

## Issues in scope

### #86 — `agov-mcp` exposes 10 of 11 `agov coord` subcommands' worth of the CLI, missing 6 tools

**Refreshed against current `main` per `pghm`'s `V27-MCP-1`** (round-2
correction from the original "nine tools ... three of ten"): live MCP
session confirms ten tools total (`agov_coord_list`, `agov_coord_pending`,
`agov_coord_post`, `agov_coord_validate`, `agov_handoff_complete`,
`agov_handoff_next`, `agov_handoff_take`, `agov_plan_validate`,
`agov_session_resume`, `agov_validate`) against the CLI's `agov coord`
subcommand list (`validate, brief, post, list, status, stale, visibility,
graph, graph-index, publication-check, pending` — 11 total, 4 mirrored:
`list`, `post`, `validate`, `pending`). State as "ten MCP tools total, four
of eleven coordination subcommands exposed." Additionally, the one
`agov_session_resume` tool that *is* exposed omits the CLI's `--manifest
<MANIFEST>` flag — confirmed at
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
| Structurally invalid for any reason (bad/missing `jsonrpc`, missing/wrong-type `method`, malformed JSON, an `id` shape B3 rejects, or a `params` shape rejected below) — with or without `id` present | `-32600 Invalid Request` (or `-32700` for malformed JSON specifically) with `id: null` in the response |

**`params` shape validation, added per `pghm`'s `V27-MCP-3`** (round 2 —
the outcome table above was labeled exhaustive while leaving `params`
unvalidated; this closes that gap rather than narrowing the claim). Per
JSON-RPC 2.0 §4: "if present, parameters for the rpc call MUST be provided
as a Structured value" — an Object or Array, never a bare string, number,
boolean, or `null`. The current `params: Option<Value>` field accepts any
JSON value. Add validation rejecting non-object, non-array `params` (when
present) as part of "structurally invalid," row three of the table above.
Test: `params` present as a string, a number, and `true` — three
additional negative cases, each `-32600`/`id: null`. `params` absent, an
empty object, and an empty array all remain valid (unchanged).

"Structurally valid" means: parses as JSON, `jsonrpc` is exactly `"2.0"`,
`method` is present and a string, `params` if present is an object or
array, and if `id` is present it is a string or integer (B3). Per the
JSON-RPC 2.0 spec's normative examples, an
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

**Bounded-line-read contract, corrected in round 3 per coder-tester's
infosec-review-aid finding `B-V27-MCP-R2-1`
(`20260815T222810Z-coder-tester-h6qr.md`, requested via `hzwc`).**
`run_stdio`'s current `stdin.lock().lines()`
(`tooling/rust/crates/agov-mcp/src/main.rs:112`) uses
`std::io::BufRead::lines()`, which allocates an unbounded `String` for any
single line with no length check before the line is even handed to the
JSON parser — a client (or a corrupted/adversarial stream) sending
megabytes of non-newline-terminated bytes grows that allocation without
bound. Define `MAX_REQUEST_LINE_BYTES` (8 MiB — comfortably larger than
any real coordination message, plan file, or tool-call payload this
project produces, while still bounding worst-case memory).

**Round 2's first design was itself insufficiently bounded** —
`BufRead::read_until(b'\n', &mut buf)` does not return merely because
`buf` crossed the cap; it appends the entire delimiter-free line to `buf`
before returning at all, so checking `buf.len()` *after* the call is too
late: the same unbounded allocation the fix is meant to prevent has
already happened by the time the check runs. Round 3 replaces it with a
genuinely bounded incremental reader using `BufRead::fill_buf()`/
`consume()` directly, never `read_until`:

**Round 4, corrected per `h6qr`'s `B-V27-MCP-R3-1`**: round 3's algorithm
split into a "newline found" branch and a "no newline found" branch with
*separate, inconsistent* bounds checks — the newline-found branch never
defined what happens when the line was not yet oversized but this exact
chunk's payload (the bytes up to the newline) pushes it over the cap for
the first time, which is precisely the ordinary "one byte over" case the
test matrix names. Unifying the bounds check removes the inconsistency
instead of patching the missing branch:

1. Loop calling `fill_buf()` to get the next available chunk from the
   underlying buffered reader (this returns whatever is already buffered
   or does one bounded underlying read — it never itself reads an
   unbounded amount).
2. Within each returned chunk, scan for `b'\n'`. The **payload** for this
   step is the chunk's bytes up to (not including) the newline if one was
   found, or the entire chunk if not.
3. **Bounds check, applied identically regardless of whether a newline
   was found** (this is the fix — round 3 applied this check only in the
   no-newline branch): if the line is not yet marked oversized, and
   `accumulated_len + payload.len()` would exceed
   `MAX_REQUEST_LINE_BYTES`, mark the line **oversized** and append
   nothing further (the line buffer stays at whatever length it already
   had, never exceeding the cap). If the line is not yet oversized and
   the payload fits, append it and update `accumulated_len`. If the line
   is already oversized, append nothing (payload is discarded either
   way).
4. `consume()` the chunk up through the newline if one was found, or the
   whole chunk if not — always advances the stream position, whether or
   not the payload was retained in step 3.
5. If a newline was found: the line is complete. If it was never marked
   oversized, process it normally (parse/dispatch as today). If it was
   marked oversized, respond `-32600 Invalid Request` with `id: null`
   (the line was never successfully parsed as JSON, so it cannot be
   attributed to a request `id`). Either way, reset all per-line state
   (`accumulated_len`, the oversized flag, the line buffer) before
   starting the next line. If no newline was found, continue the loop
   with another `fill_buf()` call — the line is still in progress.
6. EOF while a line is in progress (no trailing newline): treat exactly
   like reaching a newline for whatever state the line was in (complete
   normal line, or oversized) — same step-5 handling, then the read loop
   ends.

This bounds the **line buffer itself** to at most `MAX_REQUEST_LINE_BYTES`
at every point during the read, including while still consuming a
hostile, arbitrarily long, delimiter-free stream — the discard path never
retains the discarded bytes, only advances past them. Whether the
`MAX_REQUEST_LINE_BYTES` count includes the trailing `\n`/`\r\n` itself is
implementation-defined but must be stated explicitly in the code comment
and matched by the exactly-at-cap test below.

Test: a line of exactly `MAX_REQUEST_LINE_BYTES` (accepted, normal
processing); one byte over the cap, delivered as a normal (non-hostile)
single write (rejected, loop continues); **the chunk-boundary case added
per `h6qr`'s `B-V27-MCP-R3-1`** — a line whose first over-cap byte and
its terminating newline arrive in the *same* `fill_buf` chunk (this is the
exact gap round 3 left undefined: not-yet-oversized, newline present,
payload alone pushes it over) — rejected with `-32600`/`id: null`, not a
panic or an inconsistent state; a **delimiter-free stream many times
larger than the cap** — the case round 2's design failed to actually
bound — with an assertion on the line-buffer's length staying capped
throughout the read, not only on the final `-32600` response; EOF while
mid-line and already oversized (treated as line-complete, not a hang);
and a well-formed request line immediately following an oversized one
(processed normally, proving recovery).

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
`agov_coord_graph_index` tool (corrected name — see below) does **not**
expose `--output`; it always requests stdout output and returns the JSON
body as the tool result. This is an intentional, documented exclusion, not
an oversight, and the CLI/MCP contract test below must assert it
explicitly (a passing test that merely doesn't cover `--output` is not the
same as a test proving it's absent).

**Frozen tool contract, added per `pghm`'s `V27-MCP-3`** (round 2 — the
six mappings were named only as CLI subcommands, not as frozen MCP tool
names/schemas). Corrects the plan's own prior `agov_graph_index` reference
(missing the `coord_` infix used by every other coordination tool). Every
new tool takes an optional `dir` string parameter, matching the CLI's
`--dir` (default `docs/coordination`) and the existing ten tools'
convention — no `coordination_dir` rename for these six only (see
"Revision history"):

| MCP tool name | CLI subcommand | Parameters (beyond `dir`) |
|---|---|---|
| `agov_coord_status` | `agov coord status` | `recent` (integer), `roles` (boolean) |
| `agov_coord_visibility` | `agov coord visibility` | `base_branch` (string), `branch` (string), `format` (enum: text, json) |
| `agov_coord_stale` | `agov coord stale` | none |
| `agov_coord_graph_index` | `agov coord graph-index` | none — `--output` deliberately not exposed (B5 above) |
| `agov_coord_publication_check` | `agov coord publication-check` | `base_ref` (string, **required**), `subject_prefix` (string), `format` (enum: text, json) |
| `agov_coord_brief` | `agov coord brief` | `id_format` (enum: readable, ulid), `from`, `to`, `kind`, `topic`, `model` (all string) |

All six mirror the CLI's own flag names and defaults exactly (verified
against `agov coord <subcommand> --help` output on current `main`, not
assumed). `agov_coord_brief` generates a skeleton and does not write to
disk (unlike `agov_coord_post`) — confirmed read-only, consistent with
this plan's "every new tool is read-only" scope.

Add the missing `--manifest` forwarding to `agov_session_resume`. Do not
claim full parity from a hand-maintained count: add an executable test
that enumerates the CLI's `agov coord --help` subcommands and the MCP
registry's tool list and asserts the intended coverage set matches, so
future CLI additions cannot silently drift out of sync with the MCP
surface again. Any CLI command or flag deliberately left MCP-unexposed
(`graph`, `graph-index --output`) must be named in that test with its
reason, not just absent.

## Affected files

- `tooling/rust/crates/agov-mcp/src/main.rs` — `JsonRpcRequest` struct,
  `run_stdio`'s bounded-line-read loop, and `params`/`id` validation
  (#88); `tools()` and the CLI-arg-building match arms for all six new
  tools (#86); `agov_session_resume` arm specifically for `--manifest`.
- `tooling/rust/crates/agov-core/src/roi.rs` — **added per `pghm`'s
  `V27-MCP-1`**: `MCP_TOOL_ALLOWLIST` gains the six new tool names
  (`agov_coord_status`, `agov_coord_visibility`, `agov_coord_stale`,
  `agov_coord_graph_index`, `agov_coord_publication_check`,
  `agov_coord_brief`) — every dispatched tool is checked against this
  allowlist before dispatch, so a tool added to `tools()` without a
  matching allowlist entry would be listed but silently rejected at call
  time. Add a test asserting the allowlist and the tool registry stay in
  sync (same drift risk `V27-MCP-1` flagged for the tool-count claim
  generally).
- `tooling/rust/crates/agov-mcp/tests/stdio.rs` — existing transport
  tests; add JSON-RPC validity cases (`jsonrpc`, `id`, and `params`
  shape validation; bounded-line-read cases).
- `tooling/rust/crates/agov-mcp/tests/roi_transport_attribution.rs` —
  check for interaction with new tool entries and the allowlist addition.
- New: a CLI-registry/MCP-registry contract test (exact location to be
  decided at implementation time — likely alongside the existing
  `tool_list_contains_only_cli_backed_tools` test at `main.rs:715`).
- `AGENTS.md` and `agf/docs/site/docs/adopt/mcp-server.md` — update the
  documented tool-to-CLI mapping once the new tools ship.
- `docs/reference/tooling-token-roi.md` — mentioned by `roi.rs`'s own doc
  comment as documenting `MCP_TOOL_ALLOWLIST`; update if the six new
  entries change any documented figure there.

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
14. **`params` shape validation, added per `pghm`'s `V27-MCP-3`** — a
    string, a number, and `true` as `params`, each with an otherwise-valid
    request: `-32600`, `id: null`. `params` absent, an empty object, and
    an empty array: all still valid, unchanged behavior.
15. **Bounded-line-read contract, corrected per `h6qr`'s
    `B-V27-MCP-R2-1` and `B-V27-MCP-R3-1`** — a line of exactly
    `MAX_REQUEST_LINE_BYTES`: accepted, processed normally. A line one
    byte over, delivered as a single normal write: rejected (`-32600`,
    `id: null`). **The chunk-boundary case (`B-V27-MCP-R3-1`)**: the
    first over-cap byte and the terminating newline in the same
    `fill_buf` chunk — rejected consistently, not left undefined. A
    **delimiter-free stream many times larger than the cap** (the actual
    hostile case): the line buffer's length is asserted to stay at or
    below `MAX_REQUEST_LINE_BYTES` throughout the read, not only checked
    after the fact — this is the test that would have caught round 2's
    `read_until`-based design failing to actually bound memory. EOF while
    mid-line and already oversized: treated as line-complete, no hang. A
    well-formed request line immediately following an oversized one:
    processed normally, proving the transport recovers rather than
    desyncing.
16. **`MCP_TOOL_ALLOWLIST` sync, added per `pghm`'s `V27-MCP-1`** — each
    of the six new tool names is present in `MCP_TOOL_ALLOWLIST`; a tool
    call for a name present in `tools()` but absent from the allowlist is
    rejected, not silently dispatched (proves the two lists are actually
    checked against each other, not just both hand-updated).

## Downstream impact (per `downstream-impact-rule.md`, `tooling/**` trigger path)

**Added per `pghm`'s `V27-MCP-2`.** Round 1 treated this plan as purely
additive (new tools only). It is not: Slice 1 makes the transport *reject*
requests that are accepted today (missing/wrong `jsonrpc`, invalid `id`
shapes, invalid `params` shapes) — existing MCP clients that have been
sending non-conformant requests and getting away with it will start
receiving `-32600` errors instead of silent dispatch. That is a behavioral
contract change, not purely additive.

    Impact:
      Foundry:          Any of this project's own AI-assistant sessions
                         using `agov-mcp` that send non-`"2.0"` `jsonrpc`,
                         non-string/integer `id`, or non-object/array
                         `params` will start seeing `-32600` where they
                         previously got a normal response. No current
                         Foundry client is known to do this (the transport
                         only ever emitted spec-correct requests itself),
                         but this has not been exhaustively audited.
      Direct adopters:  BREAKING for any adopter's MCP client sending
                         non-conformant JSON-RPC 2.0 requests; ADDITIVE for
                         the six new tools. A conformant client (the normal
                         case — every mainstream MCP client library sends
                         valid `jsonrpc`/`id`/`params`) sees no behavior
                         change on existing tools, only new tools appearing
                         in `tools/list`.
      Downstream:       No schema, storage, or data-model change. No
                         change to any tool's read/write authority.
      Migration:         Adopters whose MCP client sends non-conformant
                         JSON-RPC 2.0 requests must fix the client to send
                         valid `jsonrpc: "2.0"`, a string-or-integer `id`
                         (or omit it for a notification), and an
                         object-or-array `params` (or omit it). This is
                         conformance to an existing external spec
                         (JSON-RPC 2.0 / MCP), not a new AGF-specific
                         requirement.
      Evidence:          Issues #86, #88; `agov-core/src/roi.rs`'s
                         `MCP_TOOL_ALLOWLIST` cross-check; `pghm`'s
                         `V27-MCP-2` finding.

Gate 2 must include the exact `Adopter docs parity` outcome per
`downstream-impact-rule.md` — whether `agf/docs/site/docs/adopt/mcp-server.md`
and any other adopter-facing MCP documentation needed a matching update, and
confirmation that update shipped in the same change or is explicitly
tracked as a fast-following documentation commit.

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
- Any release/tag/version action. This plan is one of the two governor-
  selected (`8hgm`) child plans of the v2.7.0 MCP/assistant-integration
  lane (alongside MR !807, issues #87/#90); it does not request authority
  to cut, tag, or publish v2.7.0 itself — that remains a separate, later
  decision.
