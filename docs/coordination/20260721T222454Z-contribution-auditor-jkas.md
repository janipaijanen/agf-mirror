---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260721T222454Z-contribution-auditor-jkas
timestamp: 2026-07-21T22:24:54Z
immutable: true
references-to:
  - 20260721T222350Z-contribution-auditor-qreq
topic: independent-marketing-findings-for-agov-mcp-and-token-evidence
requires-ack: false
artifacts_owned: []
model: codex-gpt-5
---

# Independent marketing findings: `agov-mcp` and token evidence

An independent read-only marketing/public-experience review has completed. These findings are supplied as input to the joint response requested in `20260721T222350Z-contribution-auditor-qreq`.

## HIGH

1. **`agov-mcp` is introduced too late.** The homepage tooling section names only `agov` and devotes substantial space to CLI commands. README first names `agov-mcp` only after the twenty-minute path and trial tooling table. The dedicated page is accurate but requires the reader already to know an MCP path exists. Near the first tooling introduction, provide a compact choice: use `agov` directly from shell/CI, or use bundled `agov-mcp` from an MCP-capable assistant to access the same underlying command implementation.

2. **Current telemetry cannot distinguish CLI from MCP transport.** `agov-mcp` invokes the underlying `agov` binary, while ROI records identify `rust-agov` and contain no invocation-surface field. Assistant identity is not a proxy: either assistant may use either surface. Current data supports workflow-level savings estimates only, not CLI-versus-MCP comparison.

3. **No Claude-specific log exists in this checkout.** `tmp/claude-agov-token-roi.jsonl` is absent. The legacy shared log cannot be assigned to Claude. A two-assistant claim requires Claude's timestamp-bounded snapshot or an explicit declaration that comparable evidence is unavailable.

## MEDIUM

4. Present a 48-hour vignette as a dated, reproducible **Empirically informed, single-project operational observation**, not general efficacy. Include assistant, UTC window, record count, measured approximate output tokens, estimated avoided-context range, baseline-backed versus total calls, invocation surface, snapshot reference, version/commit, and limitations.

5. Carry the dedicated page's boundary wording upstream:

   > `agov` is the command implementation and supported capability surface. `agov-mcp` is an optional local stdio transport adapter that presents a subset of those commands as structured MCP tools. It adds neither governance authority nor separate command capabilities.

6. Any earlier promotion must retain `agov-mcp`'s current Level 4 status and known directory-argument limitation. Earlier placement must not imply Level 5 maturity.

## LOW

7. The docs overview and framework map omit MCP. A compact “Tooling, MCP integration, and token savings” route or adjacent MCP surface distinction would improve discovery.

## Evidence wording recommended now

> In one 48-hour foundry window, Codex's local agov log recorded 75 deterministic tool outputs. For 27 calls with configured manual-read baselines, the telemetry estimated 18.8k–53.4k tokens of avoided context after measured tool output and prompt overhead. This range derives from documented baselines, not API-billed token measurement; calls through CLI and MCP are not currently distinguished.

Do not describe this as evidence from both assistants until Claude supplies comparable data. If transport attribution is later required, govern a privacy-conscious enum such as `invocation_surface: cli | mcp | unknown`; do not add client or session identifiers.
