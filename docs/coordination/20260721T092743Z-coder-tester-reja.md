---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260721T092743Z-coder-tester-reja
timestamp: 2026-07-21T09:27:43Z
immutable: true
references-to:
  - 20260721T080952Z-planner-architect-sdsk
  - 20260721T081448Z-coder-tester-djss
  - 20260721T083158Z-coder-tester-h1pk
topic: v2-3-0-release-implementation-gate2
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
    - independent review (Codex) of the diff and evidence below, per sdsk's
      responsibility split
    - "after merge: tag v2.3.0 on the exact merge commit and push immediately
      (release-via-merge-request-rule; no second confirmation needed)"
  must_read:
    - docs/plans/v230-release-preparation.md
    - "https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/306"
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
    - refs/tags/v2.3.0
---

# Gate 2: v2.3.0 release implementation ready for review

Implements the handoff in `sdsk`. **MR !306** (`release-v2.3.0` -> `main`),
exact head `4de5b09f3e0758bc3049749cc4f43fe8d6daa5fb`, cut from the frozen
commit `39aa4d263c43ab3823597208f1190ace7cc500bc`, rebased forward to
`9eb27c5` after the release-blocking packaging defect fix (MR !303, see
`h1pk`) merged.

## What changed

`agf/VERSION` 2.2.6 -> 2.3.0; Rust workspace versions + regenerated
`Cargo.lock` (workspace packages only, no external dependency upgrades);
`agf/templates/AGENTS.md` version marker; full `[2.3.0]` CHANGELOG entry
covering the diff since the last *published* tag (`v2.2.5`) — coordination
write primitives, the `agov` OCI image release job, `agov-mcp`, the
adopter applicability catalog, `deprecation-policy-rule`,
`agent-improvement-proposal-rule`; `[2.2.6]` entry annotated
prepared/merged-but-never-tagged-or-published per `dpm8`; stale `2.2.6` OCI
tag examples corrected to `2.3.0`; two stale inventory-count claims found
during this release's own validation corrected (README: 44->46 rules;
applicability catalog: "8 of 57"/"49 remaining" -> "8 of 58"/"50
remaining" — both verified against actual current file counts).

Excludes MR !293, !294, !295, !297, !299, !300, and the Gate-1-approved-
but-not-Gate-2-implemented `philosophical-lenses-learning-additions` plan,
per the approved plan's frozen scope.

## Evidence

**Local validation (this exact head):** `cargo build/test/clippy
--workspace -D warnings`, `agov validate`, `agov validate adopter-docs`,
`agov dist validate/build`, `mkdocs build --strict`, `git diff --check` —
all pass. `CI_COMMIT_TAG=v2.3.0 agov dist check-tag` succeeds;
`CI_COMMIT_TAG=v9.99.99` fails as required.

**MR pipeline (id `2692970887`), green on the exact head:**
`agf:dist:validate`, `agf:dist:preview`, `agf:flyer:verify`,
`agov:governance:validate`, `dependency:cve-scan`, `rust:agov:validate`,
`rust:tooling:clippy/deny/preview/test`, `site-html-build` all `success`.
`rust:tooling:build:windows:preview` is `manual`/`allow_failure: true` by
design, not a required gate.

**Downloaded and independently inspected the pipeline's own preview
artifacts** (not just local output):
- `agf-2.3.0.tar.gz`/`.zip`, `agov-2.3.0-x86_64-unknown-linux-musl.tar.gz`,
  `agov-2.3.0-aarch64-unknown-linux-musl.tar.gz` — zero forbidden/local
  paths (`.git/`, `__pycache__`, `.pyc`, etc.) in any of them; all
  checksums verified by direct recomputation, matching the shipped
  `.sha256` files exactly.
- Provenance JSONs: `agf-2.3.0-provenance.json` has real
  `source_commit: 4de5b09...`, `pipeline_identity` with the real
  pipeline/job IDs (not the `"unknown"` placeholder a local build
  produces outside CI); same for both Rust tooling provenance JSONs.
- Extracted and **executed** the pipeline's own x86_64 musl `agov`
  binary: `agov version --format json` ->
  `{"version":"2.3.0","commit":"4de5b09...","dirty":false}`. The aarch64
  binary is not executable on this x86_64 host — approved limitation per
  the plan, tag-pipeline-only for that target.

**OCI candidate — built and scanned locally** (musl toolchain and podman
became available mid-review, so this was verified directly rather than
deferred):
- `podman build` against `tooling/rust/Containerfile.agov`, replicating
  `agov-oci-image.sh build`'s staging layout.
- Smoke test: `podman run --rm agov-candidate:2.3.0 version --format
  json` -> matches version/commit/`dirty: false`.
- OCI labels (version, revision, title, source, licenses) correct;
  non-root `USER agov:agov`; `ENTRYPOINT ["agov"]`.
- **Real Trivy scan**, the exact invocation `agov-oci-image.sh scan`
  uses (same digest-pinned `docker.io/aquasec/trivy@sha256:086971aa...`,
  same flags): **0 HIGH/CRITICAL vulnerabilities found**, exit 0.
- Not locally verified: registry push (needs CI credentials), Windows
  cross target (no Docker-in-Docker locally) — tag-pipeline-only,
  consistent with `b1e35b7`'s accepted gap for those specific items.

## Residual risks / what's still pending

Per the plan's own state model (`prepared -> reviewed -> merged -> tagged
-> tag pipeline passed -> assets published -> public artifacts verified ->
completed`), this MR reaching `merged` is not evidence of publication. Tag
push, tag-pipeline jobs (`agf:dist:release`, `rust:tooling:release`,
`agov:oci:release`, `release:create`), and post-publication verification
from public surfaces are all still ahead, per the plan's Gate 2/
publication sequence — none of that is claimed as done here.
