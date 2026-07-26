---
id: otel-cost-telemetry-integration
status: pending-gate1
topic: otel-cost-telemetry-integration
advisory_consultation:
  - role: fact-checker
    status: consulted
    reason: "verified Claude Code (env-var based) and Codex (config.toml-based) OTel surfaces via live fetch against current primary documentation; see 'Reference material' section for citations and the corrected Codex model"
  - role: contribution-auditor
    status: consulted
    reason: "self-consulted in-session per the standing 'all roles in sane order' directive; reviewed scope boundary (step 1 docs-only vs step 2 deferred), dependency-minimalism argument for Option B, and downstream-impact classification"
  - role: infosec-sentinel
    status: not-applicable
    reason: "this plan's authorized scope (step 1) is documentation-only — no code, no new outbound network path. Step 2 (the OTLP bridge) explicitly requires its own future plan and its own infosec-sentinel consultation before implementation; not triggered by step 1"
---

# Plan: OpenTelemetry cost-telemetry integration for AGF tooling

## Trigger and observed problem

Both Claude Code and Codex (the two AI assistants doing AGF development work
in this repository) natively support OpenTelemetry (OTel) for reporting their
own token/cost usage, opt-in and disabled by default. Separately, AGF already
ships a bespoke, non-OTel telemetry mechanism: the "token ROI" JSONL log
(`agov-core::roi`, documented in `docs/reference/tooling-token-roi.md`),
which self-reports character/token-estimate savings for `agov`/`agov-mcp`
invocations, with default-deny argument redaction and self-reported (not
authenticated) MCP-vs-CLI transport attribution.

The user asked: should the AGF tooling (`agov`/`agov-mcp`) grow OpenTelemetry
support, either inside the existing crates or as new tooling, so that
adopters can better understand the **cost tradeoff of using AGF** — i.e. is
the governance overhead (rule loading, coordination reads, gate round-trips)
worth what it saves in tokens, time, or defect rate?

The user confirmed (in-conversation) that "cost" should cover all three of:

1. **AI assistant cost** — tokens/$ spent by Claude Code / Codex sessions,
   attributable to AGF overhead vs task work.
2. **`agov`/`agov-mcp` tool usage & latency** — how often each command runs,
   how long it takes, error rates.
3. **Coordination process overhead** — round-trips per Gate 1/Gate 2 cycle,
   time-to-approval, handoff latency — the "governance tax" independent of
   which AI tool is used.

And that placement (extend `agov`/`agov-mcp` vs new separate tooling) is
open — this plan is expected to analyze the tradeoff and recommend.

## What already exists (do not duplicate or contradict)

- `tooling/rust/crates/agov-core/src/roi.rs` — JSONL-based, synchronous,
  zero-async-runtime telemetry: `RoiRecord`, default-deny arg redaction,
  `transport`/`adapter`/`tool` self-reported MCP attribution, workflow
  baseline savings estimates (`workflow_baseline`), `agov token roi report`.
- `docs/reference/tooling-token-roi.md` — the documented rationale,
  reproduction steps, and "Remaining Gaps" section, which already lists
  `--format json` and workflow-aware reporting as known follow-ons.
- The entire `agov`/`agov-mcp`/`agov-core`/`agov-coord`/`agov-validators`
  workspace has **no async runtime dependency** (no `tokio`, no `hyper`) and
  a short, pinned dependency list (`serde`, `clap`, `jsonschema`, `sha2`,
  `time`, `zip`, `flate2`, `tar`, `ulid`, `regex`, `rayon`, `glob`). This is a
  deliberate simplicity/supply-chain posture, not an oversight.

This plan does not propose removing or replacing the existing ROI log; any
OTel work is additive and must remain compatible with it (per
`downstream-impact-rule`, this change is classified **ADDITIVE**).

## Reference material (fact-checked, live fetch, 2026-07-26)

- **Claude Code** — env-var configured, disabled by default. Enable with
  `CLAUDE_CODE_ENABLE_TELEMETRY=1`; per-signal exporter selection via
  `OTEL_METRICS_EXPORTER` (`otlp`/`prometheus`/`console`/`none`),
  `OTEL_LOGS_EXPORTER` (`otlp`/`console`/`none`), `OTEL_TRACES_EXPORTER`
  (beta, `otlp`/`console`/`none`, requires
  `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1`); transport config via standard
  `OTEL_EXPORTER_OTLP_PROTOCOL`/`OTEL_EXPORTER_OTLP_ENDPOINT`/
  `OTEL_EXPORTER_OTLP_HEADERS`; content redacted by default, opt-in via
  `OTEL_LOG_USER_PROMPTS`/`OTEL_LOG_ASSISTANT_RESPONSES`/
  `OTEL_LOG_TOOL_DETAILS`/`OTEL_LOG_TOOL_CONTENT`. Can be centrally pushed
  via an admin managed-settings JSON `env` block. Source:
  `code.claude.com/docs/en/monitoring-usage` (live fetch, this session).

- **Codex — corrected model, NOT env-var-parity with Claude Code.** Codex
  configures OTel through its **user-level `config.toml`**, under an
  `[otel]` table — **not** via `OTEL_*` environment variables, and the
  reference explicitly states OTel configuration is user-level only and
  **cannot be overridden by a project-scoped `.codex/config.toml`** (so it
  cannot be checked into this repository to configure contributors'
  sessions the way an env var convention could). Key fields: `otel.exporter`
  (`none`/`otlp-http`/`otlp-grpc`), `otel.metrics_exporter` (defaults to
  `"statsig"` — Codex's own default metrics destination is not OTel at all
  unless explicitly repointed), `otel.trace_exporter`, `otel.environment`
  (an environment tag on emitted events), `otel.log_user_prompt` (opt-in raw
  prompt export), plus per-exporter `endpoint`/`headers`/`protocol`
  (`binary`/`json`)/TLS certificate fields. Source:
  `developers.openai.com/codex/config-reference` → resolved via redirect to
  `learn.chatgpt.com/docs/config-file/config-reference` (live fetch, this
  session).

**Consequence for this plan:** the original assumption that AGF's docs
could point both assistants at "the same OTEL_ env vars" is **wrong for
Codex** and must not appear in the step-1 documentation deliverable. The
correlation doc must present two distinct configuration procedures (Claude
Code: shell/managed-settings env vars; Codex: per-user `config.toml` `[otel]`
table) rather than one unified one. This also affects Option B's rationale
in "Architectural options": reusing "the same environment-variable family
Claude Code already uses" only reduces cognitive load for Claude Code users,
not Codex users — the bridge tool's own config surface should still use
`OTEL_EXPORTER_OTLP_*` (matching the OTel ecosystem convention and Claude
Code), independent of whichever assistant produced the JSONL records it
reads.

## Architectural options considered

### Option A — Embed an OTel SDK directly into `agov`/`agov-mcp`

Add `opentelemetry`/`opentelemetry-otlp` (Rust SDK) as dependencies of
`agov-core` or `agov`, instrumenting commands with spans/metrics directly.

- **Cost:** the Rust OTel SDK's OTLP exporter pulls in an async runtime
  (`tokio`) and gRPC/HTTP stack (`tonic` or `reqwest`) — a materially larger
  dependency and supply-chain surface added to every build of the core CLI,
  including users who never enable telemetry (feature-gating reduces but
  does not eliminate this: `Cargo.lock` still names the crates, and CI
  vulnerability scanning per `security-hardening-rule` must cover them).
  Conflicts with the workspace's current zero-async-runtime posture.
  Violates `simplicity-rule`'s "generic systems when a specific solution
  would take 10% of the effort" caution unless the existing JSONL scheme is
  demonstrably insufficient for what adopters need.

### Option B — New, separate, opt-in bridge tool (recommended)

A small new crate/binary (e.g. `agov-otel-bridge`) that:

- Reads the **existing** `tmp/agov-token-roi.jsonl` log (no format change
  needed — `record_tool_output` already captures workflow, chars,
  approx_tokens, saved-token estimates, and MCP/CLI transport attribution)
  and exports it as OTLP metrics/logs on request or on a tail/watch loop.
- Optionally exposes a Prometheus scrape endpoint as a lower-dependency
  alternative to full OTLP push (Prometheus text exposition needs no async
  runtime or gRPC stack — a plain HTTP/1.1 listener suffices).
- Is disabled by default, ships as a separate binary/crate so the
  always-invoked `agov`/`agov-mcp` binaries carry zero new dependencies,
  and can be versioned/released independently of the core tooling.
- Reuses the **same environment-variable family** Claude Code already uses
  (`OTEL_EXPORTER_OTLP_ENDPOINT`, `OTEL_EXPORTER_OTLP_PROTOCOL`,
  `OTEL_EXPORTER_OTLP_HEADERS`) so an adopter who has already pointed Claude
  Code at a collector does not learn a second configuration vocabulary.

**Recommendation: Option B.** It satisfies the "all three cost dimensions"
request without weakening `agov`/`agov-mcp`'s dependency-minimalism, and it
is reversible (deleting the bridge crate has zero effect on core tooling).

### Option C — No new export path; only documentation + correlation guidance

Ship no new code. Instead, document how an adopter can:

- Enable Claude Code's/Codex's own native OTel export (assistant-side cost).
- Correlate assistant-side OTel spans with AGF's existing JSONL ROI records
  by shared `workflow` labels (already present in both) via a join done in
  the adopter's own observability stack (Grafana/Loki, etc.), not in AGF.
- Derive coordination-process overhead (dimension 3) from timestamps already
  present in `docs/coordination/*.md` frontmatter (`timestamp`,
  `references-to`) with a small offline script — no new instrumentation
  needed, since Gate 1/Gate 2 round-trip time is fully reconstructable from
  the existing append-only channel today.

This is the **lowest-cost, lowest-risk starting point** and could ship ahead
of Option B as a documentation-only PREVENTIVE/ADDITIVE change while Option
B is still under Gate 1 review, since Option C requires no new dependency
and no infosec review.

## Recommended sequencing

1. **Now (this plan, low risk):** Option C — write
   `docs/reference/otel-cost-correlation.md` documenting how to enable
   Claude Code's/Codex's native OTel export, how to correlate it with the
   existing `agov-token-roi.jsonl` via the `workflow` field, and a
   coordination-timestamp-based script for dimension 3 (process overhead).
   No code change to `agov`/`agov-mcp`; no new dependency; no infosec
   review required beyond the standard docs review.
2. **Follow-on (separate plan, Gate 1 required):** Option B — the
   `agov-otel-bridge` crate, scoped and reviewed on its own, once dimension-3
   correlation demand from (1) shows adopters actually want live-exported
   AGF telemetry rather than offline correlation.

This plan's Gate 1 approval, if granted, covers **only step 1** (the
documentation deliverable) plus the analysis above. Step 2 requires its own
subsequent plan and Gate 1 decision — it is not pre-authorized here.

## Affected files (step 1 only)

- New: `docs/reference/otel-cost-correlation.md`
- Edit: `docs/reference/tooling-token-roi.md` — add a cross-reference link
  in "Remaining Gaps" pointing to the new correlation doc (one line, per
  `marketing-tone-rule`'s message-integrity rule: link, don't restate).
- Edit: `docs/feature-flags.md` — **not applicable**; this is documentation
  of an external, already-existing capability (Claude Code/Codex's own
  OTel), not a new AGF feature flag, so `feature-flagged-docs-rule` does not
  apply to step 1. Re-evaluate for step 2 if the bridge tool ships behind a
  flag.

## Test / acceptance strategy

Step 1 is documentation-only — no unit/integration tests apply. Acceptance:

- The new doc's Claude Code env-var claims are cited against a live fetch of
  current primary Anthropic documentation (fact-checker), not conversational
  recall — done, see "Reference material" above.
- The new doc's Codex claims are cited against Codex's own current primary
  documentation (fact-checker) and correctly describe the `config.toml`
  `[otel]` model, not an env-var model — done, see "Reference material"
  above.
- The correlation script (offline, reads existing coordination timestamps)
  is runnable against this repository's own `docs/coordination/` and
  produces plausible round-trip durations for at least one already-closed
  Gate 1→Gate 2 cycle, as a smoke check.
- `docs/implementation-honesty-rule` maturity labeling: the correlation
  capability is Level 3 (Instantiated) — a worked example exists in this
  repo's own coordination history — not Level 4/5, since no automated
  validation or deployment of the correlation script exists yet.

## Risks

- **Scope creep into Option B without a second Gate 1.** Mitigated by this
  plan explicitly authorizing only step 1.
- **Codex/Claude Code configuration-model asymmetry.** Resolved during
  fact-check: Codex's OTel is `config.toml`-based and user-level-only
  (cannot be project-pinned), unlike Claude Code's env-var model. The step-1
  doc must present these as two distinct procedures, never a single unified
  env-var table, or it will mislead Codex users into looking for `OTEL_*`
  variables that Codex does not read for this purpose.
- **Codex `otel.metrics_exporter` defaults to `"statsig"`, not OTel.** A
  user who only sets `otel.exporter` may still not get OTLP metrics unless
  `otel.metrics_exporter` is separately repointed. The step-1 doc must call
  this out explicitly so adopters don't silently get partial export.
- **New outbound network path in step 2.** Flagged for infosec-sentinel now
  so the follow-on plan is written with SSRF/egress review in mind from the
  start, per `security-hardening-rule`.

## Rollback

Step 1 is a documentation addition; rollback is a plain revert. No schema,
migration, or released-artifact impact.

## Downstream impact statement

```
Impact:
  Foundry:          no behavioral change to agov/agov-mcp; documentation only
  Direct adopters:  ADDITIVE — new optional reference doc; no existing
                     contract changes
  Downstream:       none — no artifact shape change, no agent-behavior change
  Migration:        not applicable (additive)
  Evidence:         this plan; motivated by user request in-session
                     2026-07-26, not a prior incident or learning record
```
