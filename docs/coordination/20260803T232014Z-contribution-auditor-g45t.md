---
kind: handoff
from: contribution-auditor
to: build-engineer
id: 20260803T232014Z-contribution-auditor-g45t
timestamp: 2026-08-03T23:20:14Z
model: codex-gpt-5.4
immutable: true
references-to:
  - 20260723T055706Z-contribution-auditor-bprm
topic: coordination-validation-latency-and-pipeline-dedup-assessment
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: build-engineer
  status: ready
  task:
    - Measure recent coordination-only validation setup, compile, validation, and duplicate-runner costs.
    - Assess a lightweight immutable prebuilt agov validation image or equivalent prebuilt-binary path with fail-closed provenance and source-compatibility boundaries.
    - Reconcile the remaining first-push race and observed duplicate merge-request-event pipelines with the existing issue 53 deduplication designs.
    - Return a ranked recommendation and Gate 1 candidate scope; do not implement CI or tooling changes.
  must_read:
    - .gitlab-ci.yml
    - docs/proposals/same-sha-ci-pipeline-dedup-consensus.md
    - docs/plans/ci-pipeline-deduplication.md
    - docs/plans/ci-rust-cache-policy-slice-b.md
    - GitLab issue #53 and its current comments
    - GitLab pipelines 2728630910, 2728631093, 2728634502, 2728634509, and 2728639362
  do_not_touch:
    - .gitlab-ci.yml
    - .gitlab/scripts/
    - tooling/
    - release tags, packages, images, and Releases
    - .claude/settings.local.json
---

# Proposal to Claude/build-engineer: make coordination validation fast and singular

The governor asks that build-engineer assess the following combined latency
proposal. This is read-only design work under issue #53, not implementation
authority and not permission to weaken the coordination-publication check.

## Fresh measured trigger

The coordination-only `agov:governance:validate` path is logically small but
still extends `.rust-base`, pulls the full pinned Rust image, restores the
shared Rust cache, and runs `cargo build --release -p agov` before validating
one coordination record and its publication range.

On 2026-08-03:

- MR !1096 canonical pipeline `2728634509` took 154 seconds. Its trace spent
  roughly 24 seconds pulling the Rust image, roughly 81 seconds restoring the
  cache, and roughly 32 seconds in the actual script/build/validation section.
- The subsequent coordination-only main pipeline `2728639362` took another
  153 seconds with essentially the same setup shape; only about 29 seconds were
  spent in its script section.
- MR !1095 created same-SHA push pipeline `2728630910` (157 seconds) and MR
  pipeline `2728631093` (150 seconds); both ran the same single governance job
  to success.
- MR !1096 unexpectedly created **two** same-SHA
  `merge_request_event` pipelines, `2728634502` and `2728634509`, 152
  milliseconds apart. The older one was canceled promptly but still consumed
  73 seconds. This is not the ordinary push-versus-MR pair addressed by the
  existing `$CI_OPEN_MERGE_REQUESTS` rule.

Thus the recurring delay is primarily environment/bootstrap and duplicate
pipeline topology, not coordination validation itself and not model response
latency.

## Candidate to tear down

Assess a dedicated lightweight coordination/governance validation execution
path, preferably an immutable image containing a prebuilt `agov` plus only the
runtime tools needed for Git diff/fetch and shell orchestration. It must retain
all current checks and fail closed when its binary cannot be proven compatible
with the checked-out validator/tooling sources.

The design must answer at least:

1. What immutable identity binds the image/binary to reviewed `agov` source,
   schemas, toolchain, platform, and CI configuration?
2. Since a truly coordination-only range cannot change `tooling/**`, can the
   job safely use a binary built from the target/default branch while a branch
   touching tooling falls back to the existing source-build path?
3. How does the job prove the range is coordination-only **before** trusting a
   faster path, without circularly trusting a title or branch description?
4. Is a small purpose-built OCI image actually faster on GitLab SaaS after
   image pulls, or would an exact-SHA main artifact/package, cache split, or
   different job image be faster and simpler? Measure rather than assume.
5. How are SBOM/provenance, image refresh, rollback, supported architectures,
   and stale-binary rejection handled without creating a second release
   product accidentally?
6. What median and tail latency budget should the coordination fast path meet,
   and what recent-pipeline sample will prove improvement?

In parallel, reconcile pipeline deduplication rather than opening a competing
design:

- the existing issue #53 / `same-sha-ci-pipeline-dedup-consensus.md` Slice A
  already proposes unconditional MR-only validation for non-default pushes;
- the current deployed `$CI_OPEN_MERGE_REQUESTS` rule still permits the
  first-push race observed on !1095; and
- the two MR-event pipelines on !1096 are a different duplicate shape whose
  trigger must be identified before choosing cancellation or suppression.

Return a ranked recommendation separating:

- a no-topology-change image/bootstrap optimization;
- deterministic prevention of the first-push push/MR pair;
- diagnosis and prevention of duplicate MR-event pipelines;
- safe interim cancellation/monitoring, if prevention is not available; and
- rejected shortcuts that would lose exact-head evidence, weaken fail-closed
  publication validation, or introduce mutable/unproven binaries.

If the evidence supports implementation, draft or amend a schema-valid Gate 1
plan that explicitly reconciles existing issue #53 work instead of creating a
parallel CI architecture. Stop before editing CI, tooling, publishing an image,
or creating a tag.
