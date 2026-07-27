---
artifacts_owned: []
from: governor-reviewer
handoff:
  do_not_touch:
  - agf/VERSION
  - any release branch or tag
  must_read:
  - docs/plans/issue69-mcp-path-resolution-gate1-plan.md
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/69
  next_agent: coder-tester
  status: ready
  task:
  - "Slice A: add a PATH resolution fallback to agov_command() in tooling/rust/crates/agov-mcp/src/main.rs, between the checkout-build checks and the cargo-run fallback. Keep AGOV_MCP_AGOV_BIN as the first-checked override."
  - "Slice A: make agov-mcp fail loudly at startup (before the first request) if agov cannot be resolved at all, instead of deferring to the first tools/call."
  - "Slice A: add the negative fixtures the plan specifies (PATH-only success case, fully-unresolvable startup-failure case), alongside the existing AGOV_MCP_AGOV_BIN-set regression test in tests/stdio.rs."
  - "Slice B: update the README.txt printf in both the Linux and Windows archive blocks of .gitlab-ci.yml's .rust-tooling-package script to state agov-mcp is an MCP server and link agf/docs/site/docs/adopt/mcp-server.md."
  - "Slice B: add --help/-h handling to agov-mcp's main() with a short usage line and the same docs link."
  - Submit separate branches/MRs for Slice A and Slice B per the sizing test.
id: 20260727T072035Z-governor-reviewer-h6d2
immutable: true
kind: handoff
model: claude-sonnet-5
references-to: []
requires-ack: true
timestamp: 2026-07-27T07:20:35Z
to: coder-tester
topic: issue69-mcp-path-resolution-gate1-approved
---

# Issue #69 Gate 1 approved: agov-mcp PATH resolution and discoverability

Independently re-verified both defects in issue #69 against the current
source: `agov_command()` in `agov-mcp/src/main.rs` genuinely has no `PATH`
fallback (confirmed by reading the function -- checks
`AGOV_MCP_AGOV_BIN`, then checkout release/debug builds, then falls back to
`cargo run`), and the "install by placing on PATH" README text traced to an
exact `printf` in `.gitlab-ci.yml`'s `.rust-tooling-package` script. The
false-green failure shape (registration and `tools/list` succeed while every
`tools/call` fails) is real: the tool list is a static allowlist that never
invokes the child process.

One sub-claim needed correction: `--version` no longer "prints nothing" --
unrelated prior work already added `--version`/`-V` handling after v2.4.4.
`--help` remains unhandled and still needs Slice B's fix.

Decomposed into Slice A (PATH fallback + loud startup failure, release-
blocking) and Slice B (README.txt + `--help` discoverability, not blocking
but ships alongside). Full reasoning and verification table in the linked
plan.

Gate 1 approved under the governor's Gate 1 delegation to me (Gate 2 remains
the human governor's). This fix is scoped for inclusion in v2.4.5 -- see the
separate v2.4.5-scope coordination note.
