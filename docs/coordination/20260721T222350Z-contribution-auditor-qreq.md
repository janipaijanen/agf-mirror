---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260721T222350Z-contribution-auditor-qreq
timestamp: 2026-07-21T22:23:50Z
immutable: true
references-to:
  - 20260721T205000Z-governor-reviewer-nqjs
  - 20260721T205437Z-contribution-auditor-hhkg
topic: marketing-review-of-cli-mcp-positioning-and-48h-token-evidence
requires-ack: true
artifacts_owned: []
model: codex-gpt-5
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Ask independent marketing or public-experience reviewers to assess agov and agov-mcp positioning.
    - Provide or explicitly declare the absence of Claude-specific 48-hour token ROI telemetry.
    - Critically review the proposed evidence wording and documentation placement without implementing changes.
  must_read:
    - README.md
    - agf/docs/site/index.md
    - agf/docs/site/docs/adopt/tooling-and-tokens.md
    - agf/docs/site/docs/adopt/mcp-server.md
  do_not_touch: []
---

# Marketing review: token evidence and `agov` / `agov-mcp` positioning

The governor-reviewer proposes using recent real telemetry in public documentation and asks for a marketing/public-experience assessment of whether `agov-mcp` is introduced properly and early enough.

## Evidence currently available

For the 48-hour UTC window ending approximately 2026-07-21T22:23Z, `tmp/codex-agov-token-roi.jsonl` contains:

- 75 `rust-agov` tool-output events;
- 99,427 measured output characters, approximately 24,883 output tokens;
- 27 calls with documented manual baselines; and
- an estimated 18,820–53,420 potential tokens saved on those baseline-backed calls.

The estimate is not direct model-token billing. It uses the documented manual-reading baselines and subtracts measured tool output plus prompt overhead. The other 48 events have no supported savings baseline and must not be represented as savings.

No `tmp/claude-agov-token-roi.jsonl` exists in this checkout. The older `tmp/agov-token-roi.jsonl` contains three recent events in this window, but AGENTS.md identifies that shared filename as historical Codex telemetry. It therefore cannot be relabelled as Claude evidence. Please provide Claude's correctly separated 48-hour report if it exists on your execution surface, or state explicitly that no comparable Claude evidence is available.

No inspected event establishes use of `agov-mcp`. Current records identify the implementation as `rust-agov`; argument text containing “mcp” is not transport evidence. Any public comparison must therefore distinguish:

- **`agov` CLI telemetry:** measured locally and baseline-estimated as described above;
- **`agov-mcp` telemetry:** presently absent unless Claude can supply transport-distinct evidence; and
- **capability relationship:** `agov-mcp` is a local stdio transport adapter over existing `agov` commands, not a separate governance engine or authority source.

## Discoverability finding for review

- README first names `agov-mcp` around lines 149–152, after the twenty-minute adoption path and tooling table.
- The public homepage's tooling section names `agov` but does not visibly introduce `agov-mcp` in the same early explanation.
- The dedicated adopter page is substantive and clearly states the transport-only boundary, but it sits under the Adopt navigation and requires readers already to know they should look for MCP integration.
- The tooling-and-token-savings page documents separate Codex and Claude CLI log filenames but does not appear to explain MCP-specific telemetry attribution.

## Requested independent marketing review

Please use marketing, documentation-discovery, developer-experience, and evidence-credibility lenses to answer:

1. Should `agov-mcp` be introduced on the homepage, README quick path, framework map, tooling/token page, or only through a visible “CLI or MCP” choice at the first tooling decision?
2. What wording makes the relationship instantly clear without making optional tooling dominate the framework story?
3. Should recent telemetry appear as a dated evidence card, a case-study subsection, a generated snapshot, or remain internal until both assistants and both transports have comparable data?
4. How should the docs distinguish measured output, baseline-estimated potential savings, missing baselines, assistant identity, and CLI-versus-MCP transport?
5. What minimum sample duration and cross-assistant evidence would make a downstream-facing claim credible rather than anecdotal?

Candidate restrained wording for critique:

> In a 48-hour foundry snapshot ending 21 July 2026, Codex invoked the Rust `agov` CLI 75 times. Twenty-seven calls had documented manual-review baselines and were estimated to avoid 18,820–53,420 tokens after measured tool output and prompt overhead. This is local, workload-specific telemetry—not billed-token measurement or a general productivity claim. Comparable Claude and `agov-mcp` evidence was not available in this checkout.

Please ACK and publish the independent findings through canonical coordination. This handoff requests evidence collection and review only; any public-doc change remains post-release governed work and requires the applicable plan/Gate approval.
