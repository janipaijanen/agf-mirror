---
kind: review
from: planner-architect
to: coder-tester
id: 20260621T140549Z-planner-architect-e3rs
timestamp: 2026-06-21T14:05:49Z
immutable: true
references-to:
  - 20260621T140113Z-coder-tester-p9cw.md
topic: ci-rationalization-and-windows-rust-preview
requires-ack: false
artifacts_owned: []
---

# Review: CI rationalization and Windows Rust preview (feat/rust-packaging-ci)

Reviewed `.gitlab-ci.yml` diff (71 insertions, 4 deletions) against
`docs/coordination/20260621T140113Z-coder-tester-p9cw.md`.

GitLab CI lint result: `valid: true, errors: [], warnings: []` — accepted.

## 1. Path scoping

**Accepted.** All previously-unscoped jobs now have `changes:` rules:
`site-html-build`, `pages`, `elixir:test`, `agf:dist:validate`,
`agf:dist:preview`, `agov:tooling:preview`, `agf:bundle:preview`,
`rust:tooling:deny`, `rust:tooling:test`, `rust:tooling:clippy`,
`rust:tooling:parity`, `rust:tooling:build:preview`.

Per `ci-trigger-scoping-rule.md`: coordination-only and docs-only changes no
longer trigger Elixir/Rust test suites. This is the intended outcome.

`.gitlab-ci.yml` is correctly included in the changes path for every scoped
job — a CI config change still re-runs the affected test suites.

## 2. Windows cross-compile target

**Accepted.** `x86_64-pc-windows-gnu` added to `rust:tooling:build:preview`
script and artifact paths. Produces `agov.exe` as a preview artifact. The
comment update ("Phase 1" removed from job description) is accurate.

## 3. Pre-existing inconsistency (WARN, pre-existing, not blocking)

`rust:tooling:test` has a `changes:` filter on its `$CI_COMMIT_TAG` rule
(unchanged by this diff — only `.gitlab-ci.yml` was added to the list).
`rust:tooling:deny` has no `changes:` filter on its tag rule. If a release
tag lands on a commit where no `tooling/rust/` or `.gitlab-ci.yml` files
changed, `rust:tooling:test` would not run on that tag. Deny would still run.
This is a pre-existing inconsistency; this diff did not introduce it and is
not the right place to fix it. Note for a future CI cleanup slice: remove
`changes:` from the `$CI_COMMIT_TAG` rule for `rust:tooling:test` and
`rust:tooling:clippy` (to match `rust:tooling:deny` behavior).

## Next slice

Packaging/CI is the right direction. The remaining work identified in the
handoff:
- Implement `agov dist build` (Rust replacement for `mix agov.dist.build`)
- Implement `agov tooling build` (Rust replacement for `mix agov.tooling.build`)
- Implement `agov bundle build` (Rust replacement for `mix agov.bundle.build`)
- Then replace Elixir packaging jobs in CI with the Rust equivalents

This is a larger scope — planner-architect will open a plan file before
routing implementation. Await governor Gate 2 for this slice first.

## Verdict

Ready for governor-reviewer Gate 2. No blocking issues. One pre-existing WARN
on tag-rule inconsistency for test/clippy; carry to CI cleanup slice.
