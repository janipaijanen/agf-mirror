# fix: agov:governance:validate builds agov fresh to ensure clean CI provenance

Branch: `fix/agov-ci-provenance`

## Problem

`agov:governance:validate` used `before_script: []` which bypassed the
`.rust-base` before_script that sets `AGOV_BUILD_COMMIT` and
`AGOV_BUILD_DIRTY`. The job ran `./tooling/agov` (a shell dispatcher)
which preferred a pre-built binary from the Cargo cache. If that cached
binary had stale or dirty provenance (built from a local working tree
with uncommitted changes), it would show `(unknown dirty)` in CI pipeline
logs — making it impossible to audit which commit produced the binary.

Root cause path:
1. A previous build (local or CI) cached `tooling/rust/target/release/agov`
   with incorrect provenance.
2. `agov:governance:validate` with `before_script: []` skipped the env
   var setup, so `cargo run` fallback also lacked the vars.
3. The cached dirty binary was used directly by subsequent jobs.

## Fix

`.gitlab-ci.yml` — `agov:governance:validate`:

- Removed `before_script: []` override; `.rust-base` before_script now
  runs, setting `AGOV_BUILD_COMMIT="${CI_COMMIT_SHA:-unknown}"` and
  `AGOV_BUILD_DIRTY=false` before the job script executes.
- Job script builds `agov` from source first (`cargo build --release -p
  agov`). With the Cargo cache warm, this is a relink-only operation.
  Build flags are set correctly by the env vars.
- Uses the freshly-built binary directly via absolute path captured
  before `cd "$CI_PROJECT_DIR"`.
- CI logs now show `agov: <version> (<commit> clean)` at job start.

## Gate self-test (negative case)

```
$ AGOV_BUILD_COMMIT="a0fd82e3f1d1" AGOV_BUILD_DIRTY="false" \
  cargo build --release -p agov --manifest-path tooling/rust/Cargo.toml --quiet
$ tooling/rust/target/release/agov --version
agov 2.2.4 (a0fd82e3f1d1 clean)
```

If `AGOV_BUILD_DIRTY=true` were set, the output would show `dirty`.
The fix ensures `AGOV_BUILD_DIRTY=false` is set by `.rust-base` before
the build runs.

## Review checklist

- [ ] `.rust-base` before_script sets env vars before `cargo build` (lines 42-45).
- [ ] `cargo build --release -p agov` runs from `tooling/rust` (cwd after `.rust-base` cd).
- [ ] `$agov_bin` is absolute; `cd "$CI_PROJECT_DIR"` doesn't break it.
- [ ] `xargs -0r "$agov_bin" coord validate` runs from repo root.
- [ ] Path-scoped rules are unchanged.
- [ ] Gate 2 handoff: 20260625T080054Z-planner-architect-ajr1.md
