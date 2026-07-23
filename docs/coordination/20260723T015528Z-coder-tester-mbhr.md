---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - tooling/elixir/mix.lock
  must_read:
  - docs/plans/v2.4.0-release.md
  - docs/coordination/20260723T012716Z-governor-reviewer-re1q.md
  next_agent: governor-reviewer
  status: ready
  task:
  - Gate 2 review for release/v2.4.0 truth-surface commit (MR !446)
id: 20260723T015528Z-coder-tester-mbhr
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260723T012716Z-governor-reviewer-re1q.md
- 20260723T012435Z-coder-tester-nnh3.md
requires-ack: true
timestamp: 2026-07-23T01:55:28Z
to: governor-reviewer
topic: v240-release-gate2
---

# Gate 2: release/v2.4.0 truth-surface commit (MR !446)

Exact SHA: `a9ec2a77a81d88af16c9461b271db9ff46ad2c4b`, branch
`release/v2.4.0`, frozen base `main` `d4163098696758e14e53587793ba83e19329bca4`
(main's own merge pipeline `2698804817` verified green before this branch
was created, per `re1q`'s condition).

## Scope

Release-truth surfaces only, per `re1q` and `docs/plans/v2.4.0-release.md`'s
acceptance criteria. Ten files changed, no product/feature code:

- `agf/VERSION`: `2.3.1` -> `2.4.0`.
- `tooling/rust/Cargo.toml`: workspace package version and the three
  internal path-dependency versions (`agov-core`, `agov-coord`,
  `agov-validators`) bumped to `2.4.0`. Individual crate `Cargo.toml`
  files use `version.workspace = true` and needed no direct edit.
- `tooling/rust/Cargo.lock`: regenerated via `cargo build --workspace` and
  `cargo build --release -p agov`. Diff is version-string-only for the
  five in-house crates (`agov`, `agov-coord`, `agov-core`, `agov-mcp`,
  `agov-validators`) — no third-party dependency changed, confirmed by
  `git diff tooling/rust/Cargo.lock` showing only `version = "2.3.1"` ->
  `"2.4.0"` hunks.
- `agf/templates/AGENTS.md`: assistant loader marker comment synced to
  `agf/VERSION` (`<!-- AGF: 2.3.1 -->` -> `<!-- AGF: 2.4.0 -->`), per the
  file's own self-documented update instruction.
- `README.md`, `agf/docs/site/docs/adopt/install-and-layouts.md`,
  `agf/docs/site/docs/adopt/tooling-and-tokens.md`,
  `agf/docs/site/docs/advanced/contracts-and-validators.md`: OCI image tag
  references `agov:2.3.1` -> `agov:2.4.0` (5 occurrences across 4 files).
  Left `agf/schemas/release-provenance-legacy.schema.json` untouched — its
  "2.3.1" is a historical description of which published records the
  legacy schema was verified against, not a current-version marker.
- `CHANGELOG.md`, `agf/CHANGELOG.md`: new `[2.4.0] - 2026-07-23` entries.
  Drafted from the actual merge history since `v2.3.1`
  (`git log v2.3.1..d416309 --merges`), not from the umbrella plan's
  aspirational scope — content and merge-request numbers were
  cross-checked against the real diffs (`git show <merge-sha> -m --stat`)
  for Workstreams A, B, C, D and five repository-level fixes (MRs !328,
  !330, !363, !398, !408) plus the CI-dedup change (MR !410). The
  `Changed (breaking)` asset-rename entry states the Phase 0
  zero-known-dependent rationale per the deprecation-policy rule.

No historical coordination record, plan file, or immutable tag was
touched, per the plan's "Explicitly out of scope" / "Historical
rewriting" boundary.

## What this handoff does NOT claim

Per `re1q`'s explicit scope boundary and the honest-capability discipline
this project runs under, this handoff does not claim the full
release-wide acceptance criteria in `v2.4.0-release.md` are verified.
Not yet exercised: OCI image build/publish and runtime smoke test,
downloaded-artifact checksum/provenance re-verification against the
public Release surface, packaging path-hygiene inspection of the actual
built archives, host release preflight, and Pages post-deploy
verification. Those require the release pipeline's build/publish jobs and
a subsequent governor-authorized tag, which `re1q` explicitly reserves for
a separate exact-SHA decision after this Gate 2.

## Verification run locally

- `mkdocs build --strict` (`agf/docs/build/mkdocs.yml`): clean, 0.76s,
  same two pre-existing "not in nav" INFO notices as every prior build
  this release cycle (unrelated).
- `cargo test --workspace` (`tooling/rust/`): 0 failed across every test
  binary in the workspace (21 `test result:` lines, 2 ignored, 0 failed),
  confirmed via `cargo test --workspace 2>&1 | grep "test result:\|FAILED\|error\["`
  showing no `FAILED` and no `error[`.
- `cargo build --workspace` and `cargo build --release -p agov`: both
  clean, only the five in-house crates recompiled at the new version.

## Pipeline

Exact-SHA pipeline `2698823782` (MR !446, merge-request-event trigger),
all required jobs `success`:

| Job | Status |
|---|---|
| site-html-build | success |
| agf:dist:preview | success |
| rust:tooling:preview | success |
| agf:dist:validate | success |
| agov:governance:validate | success |
| rust:agov:validate | success |
| dependency:cve-scan | success |
| agf:flyer:verify | success |
| rust:tooling:test | success |
| rust:tooling:clippy | success |
| rust:tooling:deny | success |

`rust:tooling:build:windows:preview` and
`rust:tooling:preview:windows-package` show `manual` — optional
cross-compile triggers, not part of this pipeline's required job set, not
run.

## Scope note

No tag, GitLab Release, upload, or public artifact created. Per `re1q`,
that step waits for your separate exact-SHA Gate 2 decision. If approved,
I'll tag the merge commit and push immediately per the standing release
workflow rule (no further confirmation needed on that specific step, per
prior standing instruction) — but will hold for your decision on this
handoff first given the size of this release and the deliberately
incomplete acceptance-criteria coverage noted above.
