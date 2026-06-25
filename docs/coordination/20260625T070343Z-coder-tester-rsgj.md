---
kind: note
from: coder-tester
to: planner-architect
id: 20260625T070343Z-coder-tester-rsgj
timestamp: 2026-06-25T07:03:43Z
immutable: true
model: gpt-5-codex
references-to:
  - 20260625T065844Z-coder-tester-jmtg
topic: agov-version-unknown-dirty-ci-fix-timing
requires-ack: false
artifacts_owned: []
---

# Coordination request: GitLab baked `agov` reports `(unknown dirty)`

Jani reports that the GitLab-baked `agov` version `2.2.4` prints version
provenance as `(unknown dirty)`.

Local observation from this checkout:

```text
./tooling/agov --version
agov 2.2.4 (a71fa7df14a0 dirty)

./tooling/agov version --format json
{"version":"2.2.4","commit":"a71fa7df14a0","dirty":true}
```

That local output is expected for a binary built from a dirty checkout, but the
GitLab-baked binary should not report unknown/dirty when CI knows
`CI_COMMIT_SHA` and should be building from clean CI state.

Initial evidence:

- `tooling/rust/crates/agov/build.rs` embeds `AGOV_BUILD_COMMIT` and
  `AGOV_BUILD_DIRTY` when those env vars are present; otherwise it falls back
  to git `rev-parse` and `git status --porcelain`.
- `.gitlab-ci.yml` `.rust-base` currently exports
  `AGOV_BUILD_COMMIT="${CI_COMMIT_SHA:-unknown}"` and `AGOV_BUILD_DIRTY=false`.
- `.gitlab-ci.yml` `.rust-tooling-package` also exports those variables before
  cross-building release archives.
- `agf/CHANGELOG.md` 2.2.2 says this class of failure was previously fixed for
  Rust CI jobs because warm caches could serve stale binaries with
  `(unknown dirty)` provenance.

Coder-tester assessment:

This should be handled as a focused CI/tooling provenance follow-up, not mixed
into the current coordination-visibility/timer/changed-governance branch unless
the governor decides otherwise. The likely plan needs to audit which GitLab job
is producing or publishing the baked `agov`, whether that job actually inherits
`.rust-base`, whether Cargo rebuilds `agov` after provenance env vars change,
and whether cached target artifacts can bypass `build.rs` provenance refresh.

Jani's proposed timing:

Fix after the current branch's changes have settled/merged, so the CI
provenance fix can be isolated and reviewed cleanly.

Planner-architect request:

Please advise whether this should be:

1. deferred until after `chore/recover-root-git-metadata` settles, as Jani
   proposes;
2. planned as a small follow-up branch focused on GitLab `agov` provenance; or
3. pulled into the current branch because the `.gitlab-ci.yml` changed-file
   validation edit already touches CI.
