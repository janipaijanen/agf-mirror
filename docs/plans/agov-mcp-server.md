---
id: agov-mcp-server
status: pending-gate1
topic: agov-mcp-server
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: not consulted in this single-agent Gate 1 planning session. A new process boundary (stdio MCP server) that agent runtimes talk to warrants an infosec-sentinel pass at Gate 2, even though it wraps an already-reviewed CLI and adds no new privilege — flagging rather than marking not-applicable.
  - role: contribution-auditor
    status: waived
    reason: single-agent Gate 1 planning session; no contribution-auditor available to consult. Flag for Gate 2 — this introduces a new component (new crate/binary) and the parity invariant below is exactly the kind of structural claim contribution-auditor review should verify.
---

# Plan: agov-mcp — MCP server wrapper over agov (GitLab #37)

**Date:** 2026-07-20T07:47:31Z
**Status:** PENDING GATE 1 APPROVAL
**Amendment note (2026-07-20T20:05:00Z):** the `dir` field-naming question
on `agov_session_resume` and five other tools — flagged by the Sol High
external assessment as a trust-relevant contract defect — is governed
separately by `docs/plans/mcp-coordination-dir-contract-fix.md`, not by
this plan's still-open `pending-gate1` status. Do not treat this plan's
stalled status as retroactive authorization for that specific change; see
the new plan for its own Gate 1 approval.
**Depends on:** `coord-cli-write-and-watch.md` (GitLab #32) — the issue
text is explicit: "an MCP wrapper over a read-only CLI has limited value."
This plan assumes `agov coord post` and `agov handoff take/complete` exist
before `agov-mcp` wraps them; it does **not** re-specify their behaviour
(see that plan for the write-path design).

---

## Intent

Ship a thin MCP (Model Context Protocol) server, `agov-mcp`, exposing
`agov`'s existing and planned operations (`validate`, `session resume`,
`handoff next/take/complete`, `coord post/list`) as MCP tools over stdio,
so agent runtimes that speak MCP (Claude Code and others) call the
framework natively instead of shelling out to the binary and parsing text
output.

## The load-bearing invariant

Per this repo's removability constraint (issue #39, README.md's "Git
history is a ledger, not a scratchpad" section) and this issue's own
acceptance criteria:

> **`agov-mcp` must never expose an operation the CLI lacks.**

This is not aspirational framing for this plan — it is the single design
constraint everything else in this plan serves. `agov-mcp` is a redundant
frontend: removing it loses ergonomics for MCP-speaking runtimes, never
capability. A full source checkout with only the `agov` binary (no MCP
server) remains equally valid and complete. Every tool this plan adds MUST
have a corresponding `agov` subcommand that predates it or ships in the
same change; this plan proposes **zero new capability**, only a new
transport for existing capability.

## Grounding in the current codebase

- `tooling/rust/Cargo.toml` is a workspace with `agov-core`, `agov-coord`,
  `agov-validators`, `agov` (the binary crate). All CLI logic that touches
  files lives in these library crates already (`agov/src/main.rs` is a
  thin dispatcher over `agov_coord::*` and `agov_validators::*` functions).
  This is exactly the shape an MCP wrapper wants: a new binary crate,
  `agov-mcp`, can depend on the same `agov-core`/`agov-coord`/
  `agov-validators` libraries `agov` depends on, and call the same
  functions `agov`'s `main.rs` calls — **not** shell out to the `agov`
  binary as a subprocess. Shelling out would duplicate argument-parsing
  and error-formatting logic in two places; calling the same library
  functions directly keeps one implementation.
- No MCP server exists in this repo today. `grep -ri mcp` across
  `agf/`, `docs/`, `tooling/` turns up exactly one mention: a hypothetical
  example in `agf/rules/always/tool-authority-boundary-rule.md`'s list of
  "every automated check, validator, CI job, MCP server..." — i.e. the
  rule already anticipates this component existing and already states
  that its output is advisory-by-default unless a named rule grants it
  blocking authority. `agov-mcp` inherits this: it is a transport, not a
  new authority; if a tool call reports a validation failure, that failure
  carries whatever severity the underlying `agov` command already carries
  today (e.g. `coord validate` failures are already blocking per CI
  wiring — the MCP tool surfaces the same structured result, it does not
  invent a new severity model).

## Scope

**In scope:**
- New workspace member crate `agov-mcp` (binary), depending on
  `agov-core`, `agov-coord`, `agov-validators` as libraries — added to
  `tooling/rust/Cargo.toml`'s `[workspace] members` and
  `[workspace.dependencies]`, matching the existing pattern for `agov`.
- stdio transport only (per the issue text: "stdio transport first").
- Tool set: one MCP tool per already-existing (or, per the dependency
  above, concurrently-shipping) `agov` subcommand relevant to an agent
  runtime's workflow: `validate` (all validators + per-file coord/plan
  validate), `session resume`, `handoff next`, `handoff take`, `handoff
  complete`, `coord post`, `coord list`. Read-only commands not directly
  relevant to a single agent's workflow loop (`dist build`, `usage
  report`) are deferred — not because they are hard, but because the
  acceptance criterion is "full role handoff cycle via MCP tools only,"
  which does not require every `agov` subcommand to have a tool on day
  one. Additional tools are additive follow-ups, always subject to the
  parity invariant.
- Structured error responses: every tool call that fails returns a
  structured error object (error kind, message, and — for schema
  validation failures — the same `(message, json_pointer)` pairs
  `agov_coord::coord_validator::validate_file` already returns) rather
  than a raw stderr string.
- Integration guide: one page, under one page per the acceptance
  criterion.

**Out of scope (explicit, per the issue text):**
- HTTP/SSE transport.
- A hosted/always-on service.
- Any operation without a pre-existing CLI equivalent (this is the
  parity invariant, restated as a scope boundary).

## Design

### Crate layout

```
tooling/rust/crates/agov-mcp/
├── Cargo.toml
├── src/
│   └── main.rs        # stdio MCP server: tool registry + dispatch
```

`agov-mcp`'s `Cargo.toml` depends on `agov-core`, `agov-coord`,
`agov-validators` via the workspace's existing path+version dependency
pattern (see `agov/Cargo.toml` for the template) plus one new dependency:
an MCP server library for Rust (stdio transport, tool registration,
JSON-RPC framing). This is the one new third-party dependency this plan
introduces; per `no-auto-package-update.md` it is added via the manifest
at a pinned version, not floated, and per `security-hardening-rule.md`'s
supply-chain section it goes through the same `cargo deny` check
(`rust:tooling:deny` CI job) as every other workspace dependency —
no CI change needed, the existing job already covers new workspace
members' `Cargo.lock` entries.

### Tool registry — parity table (this table IS the parity invariant,
kept current on every `agov-mcp` change per the acceptance criterion)

| MCP tool | Wraps `agov` subcommand | New capability? |
|---|---|---|
| `agov_validate` | `agov validate [adoption\|agents\|learnings\|infosec\|contributions\|framework-config\|readme-stats]` | No |
| `agov_coord_validate` | `agov coord validate <files>` | No |
| `agov_plan_validate` | `agov plan validate <files>` | No |
| `agov_session_resume` | `agov session resume [--role]` | No |
| `agov_handoff_next` | `agov handoff next <role>` | No |
| `agov_handoff_take` | `agov handoff take <id> --from <role>` | No (depends on coord-cli-write-and-watch.md) |
| `agov_handoff_complete` | `agov handoff complete <id> --from <role>` | No (depends on coord-cli-write-and-watch.md) |
| `agov_coord_post` | `agov coord post ...` | No (depends on coord-cli-write-and-watch.md) |
| `agov_coord_list` | `agov coord list --since <cursor>` | No (depends on coord-cli-write-and-watch.md) |

Every row cites the CLI command it wraps. A Gate 2 reviewer (or the
contribution-auditor consultation flagged above) can check this table
against `agov --help`'s subcommand tree directly — if a row's CLI column
does not exist, the tool must not ship.

### Dispatch

Each tool call:
1. Deserialises MCP tool-call arguments into the same argument shape the
   CLI's `clap`-derived struct would parse (reuse the `clap::Parser`
   structs from `agov/src/main.rs` where feasible by extracting them to
   a shared location both `agov` and `agov-mcp` depend on, rather than
   redefining argument shapes twice — see Affected files).
2. Calls the same library function `agov/src/main.rs`'s dispatcher calls
   (e.g. `agov_coord::coord_validator::validate_file`,
   `agov_coord::handoff_tracker::find_pending`).
3. Serialises the result (or structured error) as the MCP tool response.

This means `agov-mcp` and `agov` are two thin frontends over one set of
library functions — exactly the shape that keeps the parity invariant
true by construction rather than by discipline alone: a new capability
can only reach `agov-mcp` by first existing as a public function in
`agov-core`/`agov-coord`/`agov-validators`, which is also where `agov`'s
own dispatcher gets it from.

## Affected files

| File | Change |
|---|---|
| `tooling/rust/Cargo.toml` | Add `agov-mcp` to `[workspace] members`; add the chosen MCP server library to `[workspace.dependencies]` (pinned version) |
| `tooling/rust/crates/agov-mcp/Cargo.toml` | New — binary crate manifest |
| `tooling/rust/crates/agov-mcp/src/main.rs` | New — tool registry + stdio dispatch loop |
| `tooling/rust/crates/agov/src/main.rs` | Possible refactor: extract `clap`-derived argument structs shared with `agov-mcp` into a location both binaries can use (e.g. a small shared module in `agov-core`), if duplicating argument shapes proves error-prone in practice. Not required for a correct v1 — flagged as an implementation-time judgment call. |
| `tooling/rust/crates/agov-mcp/tests/` | New — integration tests exercising the parity invariant (see Test strategy) |
| `agf/docs/site/docs/` (exact page TBD at implementation time — likely under an "Integrations" or "Adopt" section matching existing site structure) | New — one-page integration guide: install, configure a runtime's MCP client to launch `agov-mcp`, example tool calls for a full Gate 1→Gate 2 handoff cycle |
| `agf/docs/build/mkdocs.yml` | Add the new page to nav |
| `README.md` | One paragraph + link, matching how `coordination-protocol.md` is referenced today |
| `.gitlab-ci.yml` | Extend `rust:tooling:test`/`clippy`/`deny` path-scoping (`tooling/rust/**/*`) already covers the new crate with no change; extend `rust:tooling:preview`/`release` packaging (`.rust-tooling-package` template) to also build and archive the `agov-mcp` binary alongside `agov`, since adopters installing the CLI archive should be able to get the MCP server the same way (see #18's OCI-image plan for the parallel packaging concern) |

## Migration needed

No schema change. New crate, new binary, new docs page, CI packaging
extension. No existing command's behaviour changes.

## Test strategy

| Tier | What | Where |
|---|---|---|
| Unit | Each tool's argument deserialisation matches the corresponding CLI struct's accepted shapes (including rejecting the same invalid shapes the CLI would reject) | `tooling/rust/crates/agov-mcp/src/main.rs` |
| Integration | Structured error shape: an intentionally invalid coordination file, fed through `agov_coord_validate`, returns a structured error containing the same `(message, json_pointer)` data the CLI's `coord validate` would print — not a bare string | `tooling/rust/crates/agov-mcp/tests/` |
| Integration (acceptance criterion) | Full role-handoff cycle exercised via MCP tool calls only, no shell access: `agov_session_resume` → `agov_handoff_next` → `agov_handoff_take` → `agov_coord_post` (file a Gate 2 handoff) → `agov_handoff_complete`, asserting the resulting coordination directory matches what the equivalent CLI-driven sequence would produce | `tooling/rust/crates/agov-mcp/tests/` (fixture-directory pattern, mirroring `tests/coord_visibility.rs`) |
| Parity check (documentation, not automated in v1) | The parity table above is manually diffed against `agov --help`'s full subcommand tree at each `agov-mcp` change; a follow-up could script this diff (e.g. parse `clap`'s command tree and the MCP tool registry and assert set-equality-or-subset), but that tooling is not required for v1 given the small number of tools |

TDD order: write the "structured error, not raw string" test first (RED —
crate does not exist), scaffold the crate + one tool (`agov_validate`)
to GREEN, then add tools one at a time, each with its own parity-table
row added in the same commit as the tool.

## Risks

| Risk | Mitigation |
|---|---|
| Parity invariant drifts silently as tools are added over time | Parity table lives in this plan and should be copied into the crate's own module doc comment (`agov-mcp/src/main.rs` top-of-file) so it travels with the code, not just the plan; Gate 2 review checklist item: "does every `agov-mcp` tool row cite a real CLI command?" |
| New MCP-server dependency turns out unmaintained or has a breaking API change | Pin the version per `no-auto-package-update.md`; the crate is thin (dispatch only), so a future swap to a different MCP library is a contained rewrite of one crate, not a framework-wide change |
| Duplicated argument-parsing logic between `agov` (clap) and `agov-mcp` (MCP schema) drifts | Flagged in Affected files as an implementation-time refactor candidate; not blocking for v1 given the small tool count, but should be revisited once the tool list grows past the current 9-row table |
| stdio-only transport limits discoverability for non-Claude-Code runtimes | Explicitly out of scope per the issue; HTTP/SSE is a documented future extension, not a v1 gap |

## Rollback

`agov-mcp` is a new, independent crate and binary. Removing it (delete the
crate, remove it from `[workspace] members`) has zero effect on `agov`,
any existing command, or any coordination file already on disk — this is
the parity invariant's rollback corollary: since `agov-mcp` owns no state
of its own (it only calls into `agov-core`/`agov-coord`/`agov-validators`,
which write only the same plain files the CLI would), deleting the crate
loses only the MCP transport, never any data or capability.

## Assumption flagged for the identity/accountability cluster

Every MCP tool that wraps a mutating command (`agov_coord_post`,
`agov_handoff_take`, `agov_handoff_complete`) inherits that command's
`--from <role>`-style explicit-identity requirement from
`coord-cli-write-and-watch.md` — this plan does not add any MCP-specific
identity mechanism (e.g. deriving `from:` from the MCP client's own
identity metadata, if the protocol exposes any). If the identity and
accountability cluster (#35/#45/#34/#47) changes how sender identity is
supplied to mutating commands, `agov-mcp`'s tool argument schemas should
be revisited in lockstep with the CLI's own flag changes — the parity
invariant requires it.
