---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260726T205337Z-planner-architect-arbr
timestamp: 2026-07-26T20:53:37Z
model: claude-sonnet-5
immutable: true
references-to: []
topic: otel-cost-telemetry-integration-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/otel-cost-telemetry-integration.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - decide Gate 1 for docs/plans/otel-cost-telemetry-integration.md
  must_read:
    - docs/plans/otel-cost-telemetry-integration.md
  do_not_touch:
    - agov/agov-mcp Rust crates (no code change proposed by this plan)
---

# Gate 1 requested: OTel cost-telemetry integration plan

MR !642, plan at exact commit `844f745` (branch
`plan/otel-cost-telemetry-integration`), requests Gate 1 for a
**documentation-only** first step (Option C in the plan): correlating
AGF's existing `agov-token-roi.jsonl` telemetry with Claude Code's and
Codex's own native OpenTelemetry export, rather than adding a new OTLP
code path to `agov`/`agov-mcp` directly.

Triggered by an in-session user request to evaluate whether AGF tooling
should support OpenTelemetry (both Claude Code and Codex support it) so
adopters can better understand AGF's cost tradeoff. The plan recommends
against embedding the OTel Rust SDK into `agov`/`agov-mcp` directly
(would add an async runtime + gRPC dependency to a currently
zero-async-runtime workspace) and against premature scope: only step 1
(documentation) is requested for Gate 1 here. A follow-on OTLP bridge
tool (Option B) would need its own future plan and its own
infosec-sentinel review.

fact-checker and contribution-auditor consultation were performed
in-session (self-consulted, per the standing "all roles in sane order"
directive already governing this session) and are recorded in the
plan's `advisory_consultation` front matter — notably, the fact-check
corrected an initial wrong assumption that Codex shares Claude Code's
`OTEL_*` env-var model; Codex actually configures OTel via a user-level
`config.toml` `[otel]` table, and the plan's "Reference material"
section now reflects that correction with citations.

`agov plan validate` passes at blocking severity.
