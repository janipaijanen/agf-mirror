---
id: 20260905T145109Z-route-all-jobs-gate-accompaniment
status: pending-gate1
topic: route-all-jobs-gate-accompaniment
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: >-
      No infosec-sentinel review of this plan exists. Recorded as waived, not
      consulted. The revised scope moves the OCI build/scan and release-verify
      jobs, and the Windows Cross/DinD jobs, onto a separate privileged-nested
      runner. That is a routing and trust-boundary change, not merely test
      coverage, so an independent infosec pass covering runner isolation,
      privileged-nested execution, and release-pipeline job placement is
      required before binding Gate 1.
  - role: contribution-auditor
    status: waived
    reason: >-
      No contribution-auditor disposition exists for this revised plan. The
      prior narrow version received an advisory CLEAR
      (`20260905T193012Z-contribution-auditor-4ghp`); that review is
      SUPERSEDED because the scope now includes a substantive `.gitlab-ci.yml`
      routing change (six `tags:` additions) and release-pipeline job
      placement, not just two additive tests. An independent re-review of this
      revised plan — not by the planner-architect session that wrote it — is
      required before binding Gate 1.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Split ordinary / DinD runner routing, with `.gitlab-ci.yml` gate accompaniment

## Revision note (2026-09-05, per `20260905T195732Z-coder-tester-83nk`)

This plan was first written to accompany MR !2362's single global
`default: tags:` block with two additive tests, asserting "no further
`.gitlab-ci.yml` edit needed." That is no longer sufficient. After the
prior version's advisory CLEAR (`4ghp`), the runner state changed:

- A dedicated project runner (`55975189`, description `dind`) is now
  registered, online, active, and idle. The governor states it carries the
  `dind` tag; the project-level API does not expose its tag list, so that
  remains governor-supplied configuration evidence until a tagged job
  selects it.
- Canonical pipeline `2823154949` showed mandatory native-musl packaging
  succeeding on the ordinary runner (MR !2392), while `agov:oci:mr-build-scan`
  (job `16326536339`) built the image but could not run nested Trivy
  (`/proc` mount denied), and `rust:tooling:oci-pull-test` (job
  `16326536352`) failed in executor preparation after the ordinary runner's
  rootless Podman socket disappeared.

So one global default cannot serve both ordinary jobs and jobs that execute
nested container workloads. This plan is revised to specify a two-class
split. `4ghp`'s CLEAR is superseded; see `advisory_consultation` above.

## Why this is still a gate change

Unchanged from the prior version: `.gitlab-ci.yml` is a declared `surfaces:`
entry for both `gate-change-check` and `ci-needs-implication`
(`.agf/gates.yaml`), and `docs/plans/20260903T045619Z-ci-mandatory-needs-
implication-correction.md` (Gate 1 approved 2026-09-03) deliberately chose
the coarse file-level surface and disclosed the doubled accompaniment burden
as accepted cost. This revised plan does more `.gitlab-ci.yml` editing than
the first, so the accompaniment obligation is larger, not smaller. Narrowing
the surface registry remains rejected for the same reason: it reopens a
recent deliberate governor decision under pressure from a blocked MR.

## The design

### Two runner classes

1. **Ordinary jobs** run on the ordinary private runner via MR !2362's
   existing global default, unchanged:

   ```yaml
   default:
     tags: [podman, linux, amd64]
   ```

   This includes every job not named below — compile, clippy, unit and
   integration tests, `agov:governance:validate`, `commercial-corpus:validate`,
   the mandatory native-musl Linux preview/release packaging introduced by
   MR !2392, `agf:dist:*`, site build, and the coordination/self-test jobs.

2. **Nested-container jobs** — jobs that start a `docker:dind` service or run
   `cross`/rootless-Podman nested workloads — override the default with
   `tags: [dind]` so they select runner `55975189`:

   | Job | Why it needs `dind` |
   |---|---|
   | `agov:oci:mr-build-scan` | builds an image and runs nested Trivy |
   | `rust:tooling:oci-pull-test` | rootless-Podman pull of the built image |
   | `agov:oci:release` | tag-pipeline image build/push |
   | `release:verify` | tag-pipeline nested pull/verify of release image |
   | `rust:tooling:preview:windows-package` | Cross/DinD Windows packaging |
   | `rust:tooling:build:windows:preview` | Cross/DinD Windows build |

   All six are confirmed present in `.gitlab-ci.yml`. Each gets a
   `tags: [dind]` key on the job (not in a shared `default:`), overriding the
   global default for that job only.

No other job may declare its own `tags:`. The invariant the accompaniment
tests assert is: **global default is the ordinary tuple; exactly the six
named jobs override with `[dind]`; nothing else self-tags.**

### Interaction with MR !2392 and MR !2362

- MR !2362's `default:` block is the base this overrides; it is not
  re-edited.
- MR !2392 (`build: remove DinD from mandatory Linux packaging`) removes the
  `docker:dind` service from `.rust-tooling-package` for the native-musl
  Linux target. That is complementary: after MR !2392 the mandatory Linux
  packaging is container-free and correctly stays on the ordinary runner via
  the default. The Windows packaging jobs keep their Cross/DinD template and
  gain `tags: [dind]` here.
- **Collision:** this plan and MR !2392 both modify `.gitlab-ci.yml` and both
  touch `tooling/rust/crates/agov/tests/ci_needs_implication.rs` and
  `gate_change_check.rs` (issue #123 — no primitive surfaces this). Sequencing
  disposition: **land this plan's implementation first**, then rebase MR !2392
  onto the new `main` and obtain a fresh exact-head canonical pipeline. The
  implementer must preserve MR !2392's native-musl test assertions and this
  plan's split-routing assertions in the same files after the rebase.

## Gate accompaniment — two tests, revised

Both `.gitlab-ci.yml`-surface gates require a changed declared test path with
non-hollow content. RED against the pre-change tree, GREEN against the
combined implementation branch.

1. **`tooling/rust/crates/agov/tests/gate_change_check.rs`** — add
   `live_gates_registry_matches_declared_shape()`: read this repository's
   actual `.agf/gates.yaml` via `repo_root()` and assert its parsed gate
   ids, surfaces, and declared test paths.
   *Per `4ghp` finding A1:* a full hard-coded registry literal is broad
   accompaniment surface. Prefer the lighter form — assert (a) `.gitlab-ci.yml`
   is a surface for exactly `gate-change-check` and `ci-needs-implication`,
   and (b) every declared surface and test path in the registry resolves on
   disk. This still catches accidental mis-scoping and dangling paths, with
   far less update churn on legitimate future registry growth. The
   implementer chooses the form and records the choice; either satisfies the
   accompaniment.

2. **`tooling/rust/crates/agov/tests/ci_needs_implication.rs`** — add
   `live_configuration_declares_split_runner_routing()`: parse the live
   `.gitlab-ci.yml` and assert all three of:
   - top-level `default.tags == ["podman", "linux", "amd64"]`;
   - each of the six named jobs declares `tags: ["dind"]`;
   - no other job defines its own `tags:` key.
   This is a direct regression guard for the split: a future edit that drops
   the default, mis-tags a nested job, or lets an ordinary job self-tag onto
   the `dind` runner fails here instead of racing or failing nested workloads
   silently. It replaces the prior version's `default.tags`-only assertion.

## Acceptance criteria

1. `agov gate change-check --base-ref origin/main` exits 0 on the combined
   implementation diff (`.gitlab-ci.yml` six `tags:` additions plus both
   declared test paths).
2. Both new tests fail first against the pre-change tree (RED, with the
   documented expected reason) and pass against the implementation branch
   (GREEN). `shared_surface_requires_each_active_gates_own_test_evidence`
   stays unchanged and green.
3. `cargo test --manifest-path tooling/rust/Cargo.toml -p agov` — full local
   suite green.
4. One canonical exact-head MR pipeline in which:
   - the MR-pipeline `dind` jobs (`agov:oci:mr-build-scan`,
     `rust:tooling:oci-pull-test`) demonstrably **select runner `55975189`**
     — shown from the job log's runner line, not from API-reported
     availability — **and** their previously failing nested steps now
     succeed (nested Trivy runs; the rootless-Podman pull completes);
   - ordinary jobs still run on the ordinary runner;
   - no ordinary job is observed selecting the `dind` runner.
5. `agov:oci:release`, `release:verify`, `rust:tooling:preview:windows-package`,
   and `rust:tooling:build:windows:preview` run only in the tag pipeline or as
   manual/allowed-failure jobs, so their `dind` routing cannot be proven in
   this MR pipeline. It is covered by test 2 (live-config assertion) and
   **deferred to the next release's tag pipeline for execution proof**. This
   deferral is explicit and must be stated in the implementation MR.
6. No paid failed job is retried merely to seek green; the accepted run
   contains the correction.

## Negative test (per `gate-self-test-rule.md`)

This plan adds and changes tests and adds `tags:` routing; it does not modify
either gate's own logic (`gate_change.rs`, `ci_rules.rs`). The negative-test
obligation is satisfied by RED-then-GREEN on the two revised tests (criterion
2), with the RED transcripts and expected reasons in the implementation MR
description, plus the canonical exact-head pipeline log.

## Affected files

- `.gitlab-ci.yml` — add `tags: [dind]` to the six named jobs. MR !2362's
  `default:` block is the unmodified base.
- `tooling/rust/crates/agov/tests/gate_change_check.rs` — new test.
- `tooling/rust/crates/agov/tests/ci_needs_implication.rs` — new test
  (replacing the prior version's planned `default.tags`-only assertion).

No schema, migration, or release-metadata change. No new dependency. No
source change in `agov` / `agov-mcp`.

## Risks

- **Runner state is volatile.** `55975189` registered mid-session and its
  `dind` tag is governor-asserted config, not API-verified. Per
  `environment-interrogation-rule`, the running system is the source of
  truth: criterion 4's tagged-selection-plus-nested-success proof is the real
  gate, and the governor should confirm `55975189`'s configuration (tag list,
  privileged/nested support, isolation) is stable before Gate 1 rather than
  approving against config that changed twice today.
- **`tags: [dind]` moves the failure if the runner cannot actually run
  nested workloads.** Criterion 4 requires demonstrated nested success, not
  merely selection, precisely to catch this.
- **Three-way `.gitlab-ci.yml` collision** (MR !2362 base, this plan, MR
  !2392) — issue #123. Sequencing is dispositioned above; MR !2392 rebases
  after and its native-musl assertions must be preserved alongside this
  plan's.
- **Release-pipeline job placement.** Moving `agov:oci:release` and
  `release:verify` to a different runner is release-integrity-relevant and is
  the reason infosec review is required (see `advisory_consultation`).
- **Deferred tag-pipeline verification** (criterion 5) means a
  mis-configuration on `55975189` affecting only the release jobs would not
  surface until the next tag pipeline. Accepted, stated, and bounded by test
  2.

## Rollback

Revert the two test files and the six `tags: [dind]` additions in one commit.
MR !2362's `default:` block reverts independently per its own rollback note.
No runner configuration is changed by this plan, so there is nothing to undo
on the runner host.

## Evidence

- `docs/coordination/20260905T123338Z-coder-tester-6bkq.md` — original
  finding: dedicated-runner fix confirmed, !2362 gate-blocked.
- `docs/coordination/20260905T195732Z-coder-tester-83nk.md` — the revision
  handoff: split-routing requirement, the six jobs, the acceptance bar, the
  sequencing ahead of MR !2392.
- `docs/coordination/20260905T193012Z-contribution-auditor-4ghp.md` — the
  now-superseded CLEAR of the prior narrow scope; findings A1 (registry
  literal brittleness) and A2 (a fresh exact-head Gate 2 handoff for MR !2362
  is owed after its branch is amended) still apply.
- `docs/coordination/20260905T200607Z-coder-tester-hqps.md` and issue #123 —
  the plan/workstream collision-detection gap this plan hits three ways.
- `docs/plans/20260903T045619Z-ci-mandatory-needs-implication-correction.md`
  — the deliberate coarse-surface decision this plan follows.
- Pipeline `2823154949`, jobs `16326536339` / `16326536352` — the nested-
  workload failures on the ordinary runner that motivate the split.
- `.agf/gates.yaml` — `.gitlab-ci.yml` declared a surface for both gates.
- `.gitlab-ci.yml` — the six named jobs confirmed present.
