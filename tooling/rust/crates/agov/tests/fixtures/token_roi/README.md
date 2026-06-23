# Token ROI Fixtures

This directory contains committed snapshots of local token ROI telemetry for
development tests and evidence demos. These files are not live telemetry logs.

Live token ROI telemetry belongs under `tmp/` using stable per-assistant names:

- Codex: `tmp/codex-agov-token-roi.jsonl`
- Claude: `tmp/claude-agov-token-roi.jsonl`
- Other assistants: `tmp/<ai-abbrev>-agov-token-roi.jsonl`

When telemetry is needed as test evidence or showcase data, copy a dated
snapshot into an explicit fixture path and document its source, date, and
confidence limits.

## `codex-agov-token-roi-20260623.jsonl`

Source: Codex live log renamed from `tmp/agov-token-roi.jsonl` to
`tmp/codex-agov-token-roi.jsonl`.

Snapshot date: 2026-06-23 UTC.

Purpose:

- provide a realistic parser/report regression fixture;
- support confidence/uncertainty discussion using real foundry telemetry;
- preserve a small evidence sample for demos without committing the live log.

Confidence notes:

- `chars` fields are deterministic measured tool output sizes.
- `approx_tokens` uses the documented `ceil(chars / 4)` estimate.
- `approx_saved_*` values are baseline estimates, not exact billing data.
- Missing or future fields must remain tolerated by the reporter.
