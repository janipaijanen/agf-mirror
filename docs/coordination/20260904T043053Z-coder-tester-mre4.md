---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260904T043053Z-coder-tester-mre4
timestamp: 2026-09-04T04:30:53Z
model: claude-sonnet-5
immutable: true
references-to: []
topic: agents-md-agov-mcp-build-guidance-gate2
requires-ack: false
artifacts_owned:
  - AGENTS.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Gate 2 review of a single-file, additive documentation change to AGENTS.md.
    - Confirm the classification (ADDITIVE, no existing obligation changed) and the fast-track waiver.
    - Merge to main via the MR if accepted.
  must_read:
    - AGENTS.md ("Build agov first, then use it heavily" section)
    - agf/docs/site/docs/adopt/mcp-server.md
  do_not_touch:
    - .gitlab-ci.yml
    - .agf/gates.yaml
    - agf/templates/AGENTS.md
    - release branches, tags, published artifacts
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2 handoff: AGENTS.md — build and wire the local `agov-mcp` binary

## What changed

`AGENTS.md`, "Build agov first, then use it heavily" section only. Three
additions, no existing line altered:

1. A paragraph stating the `tooling/rust` workspace ships two binaries
   (`agov`, `agov-mcp`) and that `agov-mcp` should be built when the session
   will use the structured MCP tools.
2. One build command in the existing code block:
   `cargo build --release --manifest-path tooling/rust/Cargo.toml -p agov-mcp`.
3. A paragraph on registering the local `agov-mcp` build as a stdio MCP server
   (`.mcp.json` / settings → `tooling/rust/target/release/agov-mcp`,
   `AGOV_MCP_AGOV_BIN` if needed), pointing to
   `agf/docs/site/docs/adopt/mcp-server.md` for the full client-config,
   `agov`-resolution-order, and known-limitation detail.

## Why

The existing text tells an `agf`/`agov` developer to build `agov` at session
start but is silent on `agov-mcp`, even though the workspace ships the crate
and AGENTS.md's own "Supported commands" section says to prefer the
`mcp__agov__*` tools when available. A developer following AGENTS.md literally
builds only `agov` and never produces the MCP server. Governor directed this
edit live and authorized a single-file direct edit under supervision;
this handoff files it through the gate.

## Classification (downstream-impact-rule)

- **ADDITIVE.** New optional build step and wiring guidance. No rule,
  obligation, gate, schema, validator, CI, or interface changed. Nothing
  adopters must do; the guidance is opt-in ("build `agov-mcp` when this
  session will use the MCP tools"). No migration note required.
- Not a gate mechanic (`.agf/gates.yaml` untouched); `gate-self-test-rule`
  does not apply.
- No MUST/SHOULD/blocking language added, so
  `documentation-rule-projection-rule` screening is not triggered; the change
  is build-habit guidance, not normative operating guidance.

## Fast-track waiver

Ordinary documentation update, one file, low risk, no gate/schema/CI/data-model
change. Per `workflow/simplicity-rule.md` the lighter process (branch + MR +
this coordination record, no separate Gate 1 plan) applies. If review finds
the change is larger than stated, stop and escalate to full Gate 1.

## Not in scope (separate follow-ups)

- `agf/templates/AGENTS.md` (adopter AI loader template) still omits both the
  `agov` and `agov-mcp` build steps.
- `agf/docs/site/docs/adopt/mcp-server.md` Install section still assumes the
  release archive rather than a from-source build.
- Local `mise.toml` / `mise.lock` env changes (drop erlang+elixir, relax glab
  pin) are held uncommitted on the operator checkout, not part of this MR.

## Branch / MR

- Branch: `chore/agents-md-agov-mcp-build-guidance` (from `main`).
- MR: opened against `main`; exact-head pipeline is the Gate 2 acceptance path.
