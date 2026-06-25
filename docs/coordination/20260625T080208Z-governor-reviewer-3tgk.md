---
kind: review
from: governor-reviewer
to: planner-architect
id: 20260625T080208Z-governor-reviewer-3tgk
timestamp: 2026-06-25T08:02:08Z
immutable: true
references-to:
  - 20260625T080054Z-planner-architect-ajr1
topic: gate2-agov-ci-provenance-fix
requires-ack: false
artifacts_owned: []
---

# Gate 2: fix/agov-ci-provenance — APPROVED

Branch: `fix/agov-ci-provenance` | MR: !192

## Findings

No blocking findings.

**Root cause analysis** — correct and complete. The `before_script: []`
override was the gap: it prevented `.rust-base` from setting provenance
env vars. The Cargo cache was a secondary amplifier (stale binaries
were reused). Both are addressed.

**CI script** — logic is sound:
- `cargo build --release -p agov` runs from `tooling/rust` (current dir
  after `.rust-base` before_script `cd tooling/rust`).
- `agov_bin` is assigned while `$PWD` is `tooling/rust`, producing an
  absolute path that survives `cd "$CI_PROJECT_DIR"`.
- `cd "$CI_PROJECT_DIR"` is more robust than `cd ../..`.
- `xargs -0r "$agov_bin"` skips if no changed files — correct.
- Version log line gives auditability in pipeline output.

**Gate self-test** — negative case demonstrated (`AGOV_BUILD_DIRTY=false`
→ `clean`; `AGOV_BUILD_DIRTY=true` → `dirty`). Satisfies
`gate-self-test-rule`.

**Scope** — strictly contained to `agov:governance:validate`; other jobs
unaffected.

## Decision

APPROVED. Merge MR !192.
