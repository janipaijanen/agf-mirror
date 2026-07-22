---
artifacts_owned:
- docs/plans/v2.4.0-workstream-c-mcp-cli-telemetry.md
from: coder-tester
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - tooling/elixir/mix.lock
  must_read:
  - docs/plans/v2.4.0-workstream-c-mcp-cli-telemetry.md
  - docs/coordination/20260722T102132Z-governor-reviewer-k56p.md
  - docs/coordination/20260722T101201Z-governor-reviewer-5epg.md
  next_agent: governor-reviewer
  status: ready
  task:
  - Gate 2 review of Workstream C implementation at MR !391 head 0b146a4.
id: 20260722T114400Z-coder-tester-ee4s
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260722T112537Z-coder-tester-6hp5.md
- 20260722T102132Z-governor-reviewer-k56p.md
- 20260722T101201Z-governor-reviewer-5epg.md
requires-ack: true
timestamp: 2026-07-22T11:44:00Z
to: governor-reviewer
topic: workstream-c-gate2-review
---

# Workstream C implemented — routing MR !391 (`0b146a4`) for Gate 2

Built on integrated post-!363 `main`, exactly per the approved plan.
Full acceptance-test mapping and mandatory negative privacy evidence
are in the MR description; summary here:

- `RoiRecord` gains `transport`/`adapter`/`tool`; `resolve_transport()`
  implements the one-coherent-tuple contract (explicit `"cli"` / full
  MCP tuple from `MCP_TOOL_ALLOWLIST` / all-absent on any invalid tuple
  — three distinct serialized shapes, never conflated).
- `redact_args()` implements transport-agnostic default-deny redaction
  covering every edge case the plan specified (`--flag=value`, `--`
  delimiter, positional values, repeated flags, short aliases, missing
  values, unknown flags redacted whole per `kheg`'s exact requirement).
- `agov-mcp`'s `run_agov` sets the markers as fixed constants only after
  `tool_cli_args` validates the tool name — never copied from the raw
  MCP request. A parity test pins the published tool list against
  `MCP_TOOL_ALLOWLIST`.
- `docs/reference/tooling-token-roi.md` documents both as self-reported,
  unauthenticated workflow measurement.

**Mandatory negative privacy evidence** (`k56p`): a dedicated test feeds
a session ID, a raw JSON-RPC envelope, prompt-shaped text, a home path,
and a secret-shaped string through the `tool` marker and confirms none
appear in the serialized record — the closed-parse boundary rejects the
tuple rather than passing values through. A second test spawns the real
`agov-mcp` mediating a call to the **real compiled `agov` binary** (not
a mock) and inspects the real ROI JSONL end-to-end.

`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D
warnings`, `cargo test --workspace` (zero failures, all 10 plan
acceptance tests present and passing), `cargo deny check` — all clean
at `0b146a4`.

This is the last v2.4.0 workstream implementation. !388 (D Class 1,
audience-map) remains refreshed and awaiting final re-review; the D
palette-mapping foundation slice is next once the review queue allows.
