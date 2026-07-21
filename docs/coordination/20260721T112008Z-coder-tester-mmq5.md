---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260721T112008Z-coder-tester-mmq5
timestamp: 2026-07-21T11:20:08Z
immutable: true
references-to:
  - 20260721T080952Z-planner-architect-sdsk
  - 20260721T092743Z-coder-tester-reja
  - 20260721T093448Z-coder-tester-7mhk
  - 20260721T094458Z-coder-tester-k3er
  - 20260721T094905Z-coder-tester-tbrj
  - 20260721T100749Z-coder-tester-24jp
topic: v2-3-0-release-implementation-gate2-final
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
    - "after merge: tag v2.3.0 on the exact merge commit and push immediately (release-via-merge-request-rule; no second confirmation needed)"
  must_read:
    - docs/plans/v230-release-preparation.md
    - "https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/306"
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
    - refs/tags/v2.3.0
---

# Gate 2 (final): v2.3.0 release, checksum fix rebased in, evidence regenerated

Supersedes `7mhk`. Responds to `k3er`'s BLOCK (checksum consumer-path defect)
and its two corrections: the checksum fix and the stale branch-tip statement
in `7mhk`.

## What changed since `7mhk`

1. Fixed the checksum defect on a separate branch/MR (**MR !308**,
   `fix/rust-checksum-consumer-path`), same governed pattern as MR !303 —
   not committed directly onto the frozen release branch. Independently
   cleared by Codex (`24jp`) against MR !308's own CI-built artifacts before
   merge.
2. MR !308 merged to `main` at `545f3862bcfe5214d6aea6caa2f497290519c2bd`.
3. Rebased `release-v2.3.0` onto the new `main` tip. New exact content head:
   `9d8388fef91d5e564614507bc19ce410f2353276` (`release: prepare v2.3.0`).
   Diff against `main` is unchanged from `7mhk`'s description — same files,
   same content — only the base moved.
4. **Caught my own process error while pushing the rebase**: the initial
   force-push landed with a `coordination:`-titled commit as the new HEAD,
   which correctly triggered `.gitlab-ci.yml`'s coordination-only skip rule
   — but incorrectly, because the *content* commit underneath had never
   itself been pipelined against the new (checksum-fixed) base. The old
   green pipeline (`2692970887`) was evidence for the pre-fix tree only.
   Corrected by pushing the content commit alone as the branch tip first,
   forcing a real pipeline (`2693255388`) against
   `9d8388fef91d5e564614507bc19ce410f2353276`, then restacking this
   coordination trail on top only after that pipeline was green.
5. `rust:tooling:clippy` failed once on the first run of `2693255388` with
   `failure_reason: no_updates_running` (empty trace — a runner stall, not a
   lint failure) — contradicted by a clean local `cargo clippy --workspace
   --all-targets -- -D warnings` run on the identical content head. Retried
   the job (`15451620507`); it passed. Pipeline `2693255388` is green.

## Exact head and pipeline

Content head `9d8388fef91d5e564614507bc19ce410f2353276`. Pipeline
`2693255388`, all required jobs `success` (`rust:tooling:clippy` green after
one infra-stall retry): `agf:dist:validate`, `agf:dist:preview`,
`agf:flyer:verify`, `agov:governance:validate`, `dependency:cve-scan`,
`rust:agov:validate`, `rust:tooling:clippy/deny/preview/test`,
`site-html-build`. `rust:tooling:build:windows:preview` is `manual`/
`allow_failure: true` by design.

## Evidence (regenerated against this exact head)

Downloaded and independently inspected pipeline `2693255388`'s own preview
artifacts (job `15450040093` Rust, job `15450040092` framework) — this is
the specific evidence `k3er`/`tbrj` asked for, freshly generated with the
checksum fix, not carried over from `reja`/`7mhk`:

- `agov-2.3.0-x86_64-unknown-linux-musl.tar.gz` and
  `agov-2.3.0-aarch64-unknown-linux-musl.tar.gz`: both `.sha256` files now
  record only the archive basename (previously `../../dist/<archive>`).
  Copied both archives and checksums into one fresh directory and ran
  `sha256sum -c` — **both `OK`**, exit 0. Zero forbidden/local paths in
  either archive. Extracted and executed the x86_64 binary: `agov version
  --format json` -> `{"version":"2.3.0","commit":"9d8388fef91d...","dirty":false}`.
  Provenance JSONs carry `source_commit: 9d8388fef91d...` and
  `pipeline_identity: {pipeline_id: 2693255388, job_id: 15450040093}`.
- `agf-2.3.0.tar.gz`: `sha256sum -c` -> `OK`. Zero forbidden/local paths.
  Provenance carries the same exact head and `job_id: 15450040092`.

OCI build/smoke/scan evidence from `reja`/`7mhk` is unaffected by this
rebase (the Rust checksum defect was specific to the `.gitlab-ci.yml`
tar.gz/zip packaging step, not the OCI image) and is not re-run here.

## Correcting `7mhk`'s stale statement

`7mhk` said "the branch tip is now `98c3b6f`" — at the moment `7mhk` itself
was committed, its own commit (`bc57265`) was already the tip, not
`98c3b6f`. Not repeating that framing here: the exact head for review is
`9d8388fef91d5e564614507bc19ce410f2353276` (the content commit); this
handoff and its predecessors are coordination-only commits stacked on top
and do not change that head or re-trigger CI.

## Residual risks

Unchanged from `reja`/`7mhk`: registry push and the Windows/`aarch64`
execution-verification gap remain tag-pipeline-only by design. No product
feature work is on this branch.
