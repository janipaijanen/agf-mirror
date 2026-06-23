---
kind: ack
from: governor-reviewer
to: coder-tester
id: 20260623T225813Z-governor-reviewer-qjpm
timestamp: 2026-06-23T22:58:13Z
immutable: true
references-to:
  - 20260623T225000Z-coder-tester-6mk5.md
  - 20260623T223730Z-governor-reviewer-g6ng.md
topic: version-provenance-contract-docs-gate2-approved
requires-ack: false
artifacts_owned:
  - docs/coordination/20260623T225813Z-governor-reviewer-qjpm.md
---

# Gate 2 approved: version provenance + contract docs (MR !157)

MR !157 (`feat/version-provenance-contract-docs` → `main`) reviewed.
Merging after CI green.

## Findings

**No BLOCKs.**

**WARN (non-blocking):** `agov --help` output still shows bare `CARGO_PKG_VERSION`
without SHA — clap's built-in help string is set at compile time from the attribute,
not from `version_text()`. The critical operator path (`agov --version`) is correct.
This is a known limitation of the early-intercept pattern; document or fix in a
future cleanup MR if it causes confusion.

## Review summary

- **`build.rs` correct:** Provenance baked in at compile time via `git rev-parse --short=12 HEAD`
  and `git status --porcelain`. Env var overrides (`AGOV_BUILD_COMMIT`, `AGOV_BUILD_DIRTY`)
  allow reproducible CI builds. `rerun-if-changed` correctly wired to `.git/HEAD` and `.git/index`.

- **`--version` output correct:** `agov --version` intercepted before clap parsing in
  `main()`, emitting `agov {version} ({sha} {dirty|clean})`. Output shape matches `g6ng`
  specification. `clean` is correct inverse of `dirty`.

- **`agov version --format json` correct:** `VersionInfo { version, commit, dirty }` struct
  with `Serialize`, `build_version_info()`, `version_command()`. JSON output shape:
  `{"version":"…","commit":"…","dirty":true|false}`. Error path returns exit code 1.

- **Tests cover both paths:** `top_level_version_includes_commit_and_dirty_state` asserts
  `agov {version} ({sha} dirty|clean)` shape on the binary; `version_json_reports_structured_provenance`
  parses JSON and verifies field types via `serde_json::Value`. Both are integration tests
  against the compiled binary — meaningful assertions.

- **`tooling/agov` launcher updated:** `rust_supported()` now routes `--version`, `-V`,
  and `version` to Rust. Help text updated to document `agov version [--format text|json]`.
  `coord graph-index` was already routed via `graph-index` case; now also matches `graph`.

- **Three-layer `coord-graph-index.md` complete:** Human ops (what/when/how), AI cold-start
  guidance (session resume pattern, `pending` queue first, interpretation rules for `resolved`
  values, anti-pattern against pasting full JSON), output contract (full field tables, exit
  codes, stability guarantees). Scope matches `g6ng` specification exactly.

- **`agent-cold-start-habits.md` update correct:** Brief mention of `agov coord graph-index`
  after `coord status`, with `pending` and `nodes[].resolved` interpretation.

## Step 4 — next

MR !157 merged. Step 3 of the RC-1 sequence is complete.
Step 4 is the RC readiness audit MR. Separate handoff to follow.
