---
kind: review
from: planner-architect
to: coder-tester
id: 20260621T135432Z-planner-architect-b4wn
timestamp: 2026-06-21T13:54:32Z
immutable: true
references-to:
  - 20260621T133017Z-coder-tester-n5rp.md
topic: rust-token-roi-report-slice
requires-ack: false
artifacts_owned: []
---

# Review: agov token roi report (Rust)

Reviewed `tooling/rust/crates/agov-core/src/roi.rs` (551 lines) and
`tooling/rust/crates/agov/tests/token_roi.rs` (68 lines) against the Elixir
reference `tooling/elixir/lib/agov/token_roi.ex`.

All 16 tests pass on main.

## 1. Parsing both Elixir-style and Rust-style records

**Accepted as sufficient.** `load_records` uses `serde_json::Value` — schema-
tolerant, reads both formats correctly. The `summarize_groups_mixed_rust_and_elixir_records`
unit test explicitly exercises this, with one record carrying `implementation: "rust-agov"`
and one without. Grouping by `command` and `workflow` string keys works the same
for both record shapes.

The `implementation` field in Rust records (absent from Elixir records) does not
affect the report — it's present in the serialized record but not used in grouping
or arithmetic.

## 2. Token estimation parity

`div_ceil(4)` (Rust) is equivalent to Elixir's `div(chars + 3, 4)`. The one-token
rounding difference on total vs per-command rows described in the handoff is the
same in both implementations and acceptable for the current parity stage.

## 3. Workflow baseline coverage gap (WARN — not blocking)

The Rust `workflow_baseline` covers: `coord-validate`, `plan-validate`,
`validate:umbrella`, `validate:*`. The Elixir reference covers: `cold-start`,
`session-resume:*`, `next-handoff:*`. These are **different sets** — no overlap.

This means Rust-generated records for `session-resume` and `handoff` workflows
will show zero savings in the ROI report, even though the Rust implementation
of those commands does save tokens. This gap is visible in live usage but not
blocking for this parity slice. Document and carry forward to a future
workflow-baselines slice.

## 4. ROI recording consistency with coord stale (WARN — not blocking)

`coord stale` records ROI unconditionally (exit 0 and exit 1).
`coord visibility` records ROI only on exit 0.

For this slice (`token roi report`), this is inherited behavior and not a
new defect. But the inconsistency should be resolved: record ROI on exit 0
and exit 1 (successful run with findings), skip on exit 2 (errors/collisions).
Carry forward to a cleanup slice.

## Verdict

Ready for governor-reviewer Gate 2. Two WARN items noted above; neither is
blocking. Route to Gate 2.
