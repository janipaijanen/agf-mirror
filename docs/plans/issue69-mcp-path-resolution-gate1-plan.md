---
id: issue69-mcp-path-resolution-gate1
status: approved
topic: issue69-mcp-path-resolution-gate1
gate1_approved_by: governor-reviewer
gate1_approved_at: "2026-07-27T07:19:45Z"
advisory_consultation:
  - role: contribution-auditor
    status: not-applicable
    reason: "issue 69 is maintainer dogfooding evidence (explicitly labeled by its author, same kantele commissioning run as issue 67), not an external contribution; no adversarial-steering or false-precedent surface to audit"
  - role: infosec-sentinel
    status: not-applicable
    reason: "binary-resolution fallback order and README/--help text; no secret, credential, permission, or trust-boundary change. Startup-time validation is a hardening improvement, not a new attack surface."
---

# Plan: Issue #69 — agov-mcp PATH resolution and discoverability

## Trigger

Issue #69 (`agov-mcp: PATH install per archive README yields a server that
registers but cannot execute any tool`) reports two defects from the same
v2.4.4 kantele commissioning run as issue #67, kept in a separate issue
because they are unrelated to #67's documentation slices. No coordination
handoff had been filed for it yet; routed here directly on governor
instruction ("coordinate and proceed as proposed").

The reporter qualifies this as maintainer dogfooding, not independent adopter
evidence — preserved here per `docs/implementation-honesty-rule.md`.

## Independent verification (performed before planning)

| # | Claim | Verification | Result |
|---|---|---|---|
| 1 | `agov-mcp` never resolves `agov` from `PATH` | Read `agov_command()` in `tooling/rust/crates/agov-mcp/src/main.rs`: checks `AGOV_MCP_AGOV_BIN`, then `target/release/agov`, then `target/debug/agov` in a checkout, then falls back to `cargo run` — no `PATH`/`which` step at any point | Confirmed |
| 1 | Error message is hardcoded to "could not execute agov" regardless of which resolution step actually failed | Read `run_agov()` — the `Err(err)` arm always formats `"could not execute agov: {err}"` regardless of `command.program` | Confirmed; matches the reported reproduction exactly |
| 1 | `initialize`/`tools/list` succeed even when every `tools/call` will fail | Read `call_tool()` / tool listing — the tool list is a static allowlist (`agov_core::roi::MCP_TOOL_ALLOWLIST`), never invokes the child process | Confirmed — false-green failure shape is real, not a reporting artifact |
| — | Archive `README.txt` says "Install by placing the agov and agov-mcp executables on PATH." | Found the exact source: `.gitlab-ci.yml`'s `.rust-tooling-package` script, `printf "AGF agov Rust tooling\nVersion: %s\nTarget: %s\n\nInstall by placing the agov and agov-mcp executables on PATH.\n"` | Confirmed — exact text match, single source location (used for both Linux archives; Windows archive block further down uses the same pattern and needs the same fix) |
| 2 | `agov-mcp --help`/`--version` print nothing, exit 0, at the v2.4.4 release the reporter used | Diffed `main.rs` between tag `v2.4.4` and current `main`: at `v2.4.4`, `main()` had no argument handling at all — any args (or none) fell through to `run_stdio()`, which on a closed/empty stdin reads EOF and returns cleanly, hence "prints nothing, exits 0" | Confirmed accurate for the version examined |
| 2 (current state) | Same behavior still present today | Current `main` already added `--version`/`-V` and `version --format json` handling (unrelated prior work, not a response to this issue) but **`--help` is still unhandled** — any other argument, including `--help`, now hits the `!args.is_empty()` branch and exits 2 with `"agov-mcp: unsupported arguments"` on stderr | Partially superseded: `--version` now works; `--help` still gives no usage text or doc link, just an unhelpful error |

Both defects confirmed. No count or claim in issue #69 was found inaccurate;
one sub-claim (`--version` prints nothing) has since been superseded by
unrelated prior work and is corrected in the table above rather than treated
as still-open.

## Gate 1 decomposition

Two slices, on the same risk/reviewability axis used for issue #67:

### Slice A — PATH resolution fallback + loud startup failure
**Question a reviewer can answer yes/no:** "Does a PATH-only install (no env
var, no checkout, no cargo) now work, and does a fully-unresolvable install
fail at startup instead of at first tool call?"

- Add a `PATH` lookup (e.g. via `which`-equivalent resolution, not shelling
  out to `which(1)`, per `security-hardening-rule.md`'s spirit of trusting
  the process's own PATH search rather than an external shell) as the
  resolution step between the checkout-build checks and the `cargo run`
  fallback in `agov_command()`. This makes the archive's own README
  instruction literally true, rather than choosing to weaken the promise.
- At server startup (`run_stdio()` entry, before accepting the first
  request), resolve the `agov` binary once and confirm it is executable
  (e.g. invoke `agov --version` or equivalent and check exit status), and
  exit non-zero with a clear stderr message identifying the resolution
  order tried and which step failed, rather than deferring the failure to
  the first `tools/call`.
- Keep `AGOV_MCP_AGOV_BIN` as the explicit override; it remains first in
  resolution order.

**Negative fixtures (per `gate-self-test-rule.md` / `boundary-shape-testing-rule.md`):**
- PATH-only fixture: `AGOV_MCP_AGOV_BIN` unset, no checkout build present,
  a fake `agov` script placed only on `PATH` — `tools/call` must now succeed
  via the same pattern `stdio.rs`'s existing `stdio_server_lists_tools_and_calls_agov`
  test uses for the env-var case.
- Fully-unresolvable fixture: `AGOV_MCP_AGOV_BIN` unset, no checkout build,
  no `agov` on `PATH`, no `cargo` on `PATH` — server must exit non-zero at
  startup with a message naming the resolution order, not merely fail the
  first `tools/call`.
- Existing `AGOV_MCP_AGOV_BIN`-set fixture must continue to pass unchanged
  (regression guard per `feature-regression-guard-rule.md`).

**Release-blocking?** Yes — this is the "registers but cannot execute" false
green from a documented, currently-shipping install instruction.

### Slice B — Discoverability: README.txt and `--help`
**Question a reviewer can answer yes/no:** "Does the shipped archive, and the
binary itself, tell an adopter what `agov-mcp` is and where the docs live?"

- Update the `printf` in `.gitlab-ci.yml`'s `.rust-tooling-package` script
  (both the Linux-archive block and the Windows-archive block further down,
  which share the same README pattern) to: state that `agov-mcp` is an MCP
  server, note `AGOV_MCP_AGOV_BIN` as an optional override once Slice A's
  fallback exists, and link `agf/docs/site/docs/adopt/mcp-server.md`.
- Add `--help`/`-h` handling to `agov-mcp`'s `main()` (currently absent —
  falls into the generic "unsupported arguments" branch) that prints a short
  usage line and the same docs link, matching the pattern used for
  `--version`.

**Release-blocking?** No — purely discoverability; Slice A's fix removes the
silent-failure risk on its own. Ships in the same release as Slice A since
both are small and already scoped together, but does not block Slice A if
sequencing requires it.

## What this plan does NOT authorize

- Any change to `agf/VERSION`, a release branch, or a tag.
- Writing a kantele learning record (separate governed addition, same
  constraint as the issue-67 plan).
- Any change to the MCP protocol surface, tool allowlist, or ROI/telemetry
  marker behavior (`agov_core::roi`) — out of scope, untouched by this issue.

## Release inclusion

Per governor instruction, this fix is scoped **for inclusion in the next
release (v2.4.5)**, alongside the already-approved issue #68 (token ROI
project-local) and the Elixir-exclusion cleanup. Coordination note filed
separately recording v2.4.5 scope inclusion.

## Next step

Gate 1 approved (governor-reviewer authority delegated to me for Gate 1 per
2026-07-27 session delegation; Gate 2 remains the human governor's). Handoff
to coder-tester for Slice A + Slice B implementation on separate branches per
`branch-and-worktree-rule.md`'s sizing test.
