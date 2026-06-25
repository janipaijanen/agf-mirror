---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260625T080054Z-planner-architect-ajr1
timestamp: 2026-06-25T08:00:54Z
immutable: true
references-to:
  - 20260625T070343Z-coder-tester-rsgj
topic: fix-agov-ci-provenance
requires-ack: false
artifacts_owned:
  - .gitlab-ci.yml
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review CI provenance fix for agov:governance:validate job.
  must_read:
    - .gitlab-ci.yml (agov:governance:validate section, lines 294-350)
  do_not_touch: []
---

# Gate 2 handoff: fix/agov-ci-provenance

## Root cause

`tooling/agov` is a shell dispatcher that prefers a pre-built binary at
`tooling/rust/target/release/agov`. In CI, the Cargo cache (`tooling/rust/target`)
contains binaries from previous pipeline runs. The cached binary may have been
built with stale or incorrect provenance (`unknown dirty`) if the cache was
populated by a local build or an earlier CI run without the provenance env vars.

`agov:governance:validate` had `before_script: []` which bypassed the
`.rust-base` before_script that sets `AGOV_BUILD_COMMIT` and `AGOV_BUILD_DIRTY`.
So even when `cargo run` was used as a fallback, the env vars were absent and
`build.rs` had to rely on git interrogation — which worked in CI but failed
silently when git history was shallow or unavailable.

## Fix

1. Removed `before_script: []` override so `.rust-base` sets `AGOV_BUILD_COMMIT`
   and `AGOV_BUILD_DIRTY` before the script runs.
2. Changed the script to `cargo build --release -p agov` first (uses Cargo cache,
   fast), then run the freshly-built binary directly. This ensures correct
   provenance regardless of what was previously in the cache.
3. Uses `$CI_PROJECT_DIR` to return to repo root after the cargo build (more
   robust than `cd ../..`).

## Gate self-test

The negative test: if `AGOV_BUILD_DIRTY=true` were set, the binary would show
`(hash dirty)`. With `AGOV_BUILD_DIRTY=false` (set by `.rust-base`), it shows
`(hash clean)`. Verified locally:

```
$ AGOV_BUILD_COMMIT="a0fd82e3f1d1" AGOV_BUILD_DIRTY="false" \
  cargo build --release -p agov --manifest-path tooling/rust/Cargo.toml --quiet
$ tooling/rust/target/release/agov --version
agov 2.2.4 (a0fd82e3f1d1 clean)
```

The CI job now prints `agov: <version>` at the start so the provenance is visible
in pipeline logs.

## Checklist for governor-reviewer

- [ ] `.rust-base` before_script (lines 42-45) sets `AGOV_BUILD_COMMIT` and
      `AGOV_BUILD_DIRTY=false` before `cargo build` runs in the job script.
- [ ] `cargo build --release -p agov` runs from `tooling/rust` (correct dir
      after `.rust-base` cd step) and produces `target/release/agov`.
- [ ] `$agov_bin` captures an absolute path before `cd "$CI_PROJECT_DIR"`.
- [ ] `xargs -0r "$agov_bin" coord validate` and `... plan validate` run from
      repo root, so file paths in the diff output resolve correctly.
- [ ] Rules block is unchanged (path-scoped triggers preserved).
