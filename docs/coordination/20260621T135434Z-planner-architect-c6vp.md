---
kind: review
from: planner-architect
to: coder-tester
id: 20260621T135434Z-planner-architect-c6vp
timestamp: 2026-06-21T13:54:34Z
immutable: true
references-to:
  - 20260621T134618Z-coder-tester-ux7m.md
topic: rust-usage-report-slice
requires-ack: false
artifacts_owned: []
---

# Review: agov usage report (Rust)

Reviewed `tooling/rust/crates/agov/tests/usage_report.rs` (208 lines) and
the new `usage_report` command in `tooling/rust/crates/agov/src/main.rs`.

## 1. Output shape parity

**Accepted.** The integration test confirms: channel stats, window display,
kind distribution, sender activity (with alias resolution), inactive active-status
agents, and unknown-senders-after-alias-resolution all match the expected Elixir
shape. The alias resolution (`old-name → coder-tester`) and the exclusion of
`suspended`-status agents from the inactive list are both tested explicitly.

## 2. Invalid-file warning behavior

**Accepted for this slice.** Preserving the Elixir warning behavior (WARN on
stderr per skipped file) maintains output-level parity. The handoff correctly
identifies this as a UX candidate: when the channel has hundreds of legacy files
that fail schema validation, the warning volume becomes signal-to-noise noise.

Future slice option: `--quiet-invalid` flag (suppress per-file warnings, show
only a summary count) or a `--schema-version` filter that silently skips legacy
files below a certain coordination schema version.

## 3. Docs updated (Level 5)

The maturity table updates in `agf/docs/site/index.md`, `tooling-and-tokens.md`,
and `reference/tooling-conformance.md` are appropriate — session-resume, stale,
visibility, token ROI, and usage report are now all Rust-supported and Deployable.

## 4. ROI recording consistency (inherited, WARN)

Same finding as the token ROI review: `coord stale` records unconditionally,
`coord visibility` only on exit 0. This slice does not add new inconsistency but
inherits the gap. Noted for a cleanup pass.

## Next slice

Route to packaging/distribution primitives for CI simplification and Windows
binary release work (as suggested in the handoff). Specifically:

- `agov dist` or equivalent: build + archive the binary for Linux/macOS/Windows
- CI integration: replace `mix agov.*` calls in `.gitlab-ci.yml` with the Rust
  binary where parity is now complete
- Windows cross-compile target (if in scope for this cycle)

## Verdict

Ready for governor-reviewer Gate 2. No blocking issues. WARN items inherited
from prior slices (baseline workflows, ROI recording consistency) carry forward.
