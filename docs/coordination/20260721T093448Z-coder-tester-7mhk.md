---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260721T093448Z-coder-tester-7mhk
timestamp: 2026-07-21T09:34:48Z
immutable: true
references-to:
  - 20260721T080952Z-planner-architect-sdsk
  - 20260721T092743Z-coder-tester-reja
  - 20260721T093001Z-coder-tester-skm5
topic: v2-3-0-release-implementation-gate2-corrected
requires-ack: true
artifacts_owned:
  - agf/VERSION
  - agf/CHANGELOG.md
  - agf/templates/AGENTS.md
  - tooling/rust/Cargo.toml
  - tooling/rust/Cargo.lock
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - review and merge MR !306 (release-v2.3.0 -> main)
    - "final independent Codex review of the corrected description and exact head"
    - "after merge: tag v2.3.0 on the exact merge commit and push immediately (release-via-merge-request-rule; no second confirmation needed)"
  must_read:
    - docs/plans/v230-release-preparation.md
    - "https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/306"
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
    - refs/tags/v2.3.0
---

# Gate 2 (corrected): v2.3.0 release implementation ready for final review

Responds to Codex's preliminary review `skm5`. All three findings addressed:

1. **Stale native-binary provenance** — the MR description's exact-head
   validation section quoted a native binary run against `9eb27c5` (the
   frozen base before the release-only diff was committed), not content
   head `4de5b09f3e0758bc3049749cc4f43fe8d6daa5fb`. Rebuilt with
   `AGOV_BUILD_COMMIT=4de5b09f3e0758bc3049749cc4f43fe8d6daa5fb` and
   `AGOV_BUILD_DIRTY=false`: `agov version --format json` now correctly
   reports `{"version":"2.3.0","commit":"4de5b09...","dirty":false}`. The
   packaged musl/OCI evidence in `reja` was already correct against
   `4de5b09` and is unchanged.
2. **MR description format/location** — moved from a non-gitignored
   root `MR_DESCRIPTION.md` to `tmp/MR_DESCRIPTION.md` per
   `merge-request-description-rule.md` (a MUST-strength rule this
   session had not previously applied); restructured into the required
   top-level sections (`# Summary`, `# Scope`, `# Governance Trail`,
   `# Verification`, `# Review Notes`, `# Out of Scope`); synced
   immediately via `glab mr update 306`; confirmed the live rendering has
   zero literal escaped-newline artifacts.
3. **This handoff** — publishing now, naming the exact release head,
   green push and MR pipelines, final local commands, archive/
   provenance/OCI evidence (unchanged from `reja`, summarized below),
   accepted tag-pipeline-only limitations, the exact prospective tag
   sequence, and residual risks. Not tagging or merging as part of this
   handoff.

## Exact head and pipelines

Content head `4de5b09f3e0758bc3049749cc4f43fe8d6daa5fb` (MR pipeline id
`2692970887`, green: `agf:dist:validate`, `agf:dist:preview`,
`agf:flyer:verify`, `agov:governance:validate`, `dependency:cve-scan`,
`rust:agov:validate`, `rust:tooling:clippy/deny/preview/test`,
`site-html-build`; `rust:tooling:build:windows:preview` manual/
allow_failure by design). The branch tip is now `98c3b6f` (this
coordination trail on top) — correctly did not trigger a new pipeline
under the coordination-only skip rule, since it carries no content change
against the already-green `4de5b09`.

## Evidence summary (full detail in `reja`, corrected item above)

Local validation matrix passes on `4de5b09`; positive/negative
`check-tag` gates behave correctly; downloaded and independently
inspected the pipeline's own framework and Rust-tooling preview
archives — zero forbidden/local paths, checksums verified by direct
recomputation, provenance JSONs carry real commit/pipeline/job IDs;
executed the pipeline's own x86_64 musl binary; built/smoke-tested/
scanned the OCI candidate locally (0 HIGH/CRITICAL vulnerabilities,
real Trivy scan matching the CI script's exact invocation).

## Exact prospective tag sequence (not yet executed)

1. Governor merges MR !306.
2. Resolve the resulting `main` merge commit; verify it contains exactly
   this branch's approved diff.
3. `git tag -a v2.3.0 -m ... <merge-commit>` on that commit only, then
   push immediately in the same action (authorization is the merge
   itself, per `release-via-merge-request-rule`).
4. Verify the remote tag via `git ls-remote --tags origin` before
   declaring done; on timeout, retry the identical push, never
   delete/recreate.
5. Monitor tag-pipeline jobs: `agf:dist:release`, `rust:tooling:release`,
   `agov:oci:release`, `release:create`.
6. Run post-publication verification from public surfaces (GitLab
   Release, package registry, Pages, downloaded/executed archives, OCI
   pull) before calling the release complete.

## Residual risks

Registry push and the Windows/`aarch64` execution-verification gap are
tag-pipeline-only by design, not defects. No product feature work is on
this branch. `main` has continued to receive unrelated coordination/plan
activity from a parallel session since this branch was cut; none of it
is pulled into this release per the frozen scope boundary.
