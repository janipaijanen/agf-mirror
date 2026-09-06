---
id: 20260905T145109Z-route-all-jobs-gate-accompaniment
status: approved
gate1_approved_by: governor-reviewer
gate1_approved_at: 2026-09-06T07:56:04Z
topic: route-all-jobs-gate-accompaniment
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Exact-head review 20260906T073655Z-infosec-sentinel-kkph returned NO
      OBJECTION at 9e2a0ea8. Its four advisory controls for isolation posture,
      anonymous post-publication verification, secret scope, and supervised
      first-tag execution are incorporated below.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Exact-head review
      20260906T073638Z-contribution-auditor-4sb8 returned NO OBJECTION at
      9e2a0ea8 and confirmed p37b B1/B2/A2 resolved by the atomic unit. Its
      three documentation-consistency advisories are incorporated below.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Atomic v2.9 runner-routing and native-packaging release unit

## Governor-adopted correction (2026-09-06)

The governor adopted
`20260905T215114Z-planner-architect-gpss` in binding decision
`20260906T072233Z-governor-reviewer-jmsq` and assigned this Codex
planner-architect to correct the plan. This revision supersedes MR !2373's
old sequencing. One atomic implementation/Gate 2 candidate combines the
still-needed diffs from MR !2362 and MR !2392 with the six split-routing
overrides and both gate-accompaniment test families.

This atomic shape resolves contribution-audit findings `p37b` B1/B2 and A2:
the mandatory Linux packaging becomes native in the same head where the six
remaining nested jobs move to `[dind]`; neither invalid intermediate state
is merged. The old `4ghp` CLEAR and `p37b` review are historical inputs,
not approval of this corrected exact head. Fresh independent contribution-
auditor and infosec-sentinel reviews remain required before Gate 1.

Pipeline `2823154949` remains the motivating execution evidence: native
musl packaging succeeded on the ordinary runner, while nested OCI work failed
there. It must not be retried unchanged.

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

### Verified runner precondition

Authenticated GitLab runner API reads on 2026-09-06T06:57Z report:

| Runner | id | `tag_list` | `run_untagged` |
|---|---|---|---|
| `priv runner` | `55908035` | `['podman', 'linux', 'amd64']` | `true` |
| `dind` | `55975189` | **`['dind']`** | **`false`** |

Both are active, unpaused, and online. The tag-isolation precondition is
therefore satisfied, but volatile: implementation re-verifies it immediately
before the first acceptance pipeline and fails closed on drift. The operator
also reports the duplicate `/cache` target fixed. No post-fix `[dind]` job
has proved that claim, so executor preparation plus nested workload success
remain acceptance evidence, not a presumed fact.

The expected host posture is a Docker executor that permits the pinned
`docker:dind` service through `services_privileged` plus an exact
`allowed_privileged_services` entry; ordinary job containers do not gain
blanket privileged mode. Before the first acceptance run, the operator records
the effective executor privilege setting, allowed-service image/digest,
network-egress policy, and whether runner 55975189 shares the same managed
host/trust boundary as runner 55908035. Unknown or broader-than-declared
privilege/egress fails closed to governor and infosec disposition.

### Two runner classes

1. **Ordinary jobs** run on the ordinary private runner (`55908035`) via
   MR !2362's existing global default, unchanged:

   ```yaml
   default:
     tags: [podman, linux, amd64]
   ```

   The atomic implementation carries MR !2392's native mandatory Linux
   packaging into this same head: `.rust-tooling-package` removes DinD,
   Docker transport, and Cross; installs `musl-tools`; selects `musl-gcc`;
   adds the target with rustup; and builds `agov` plus `agov-mcp` with
   Cargo. This includes every job not named below — compile, clippy, unit and
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

### Atomic integration of MRs !2362 and !2392

- MR !2362 contributes the ordinary default
  `tags: [podman, linux, amd64]`.
- MR !2392 contributes native mandatory Linux packaging, a separate Windows
  Cross/DinD template, and its existing CI/gate regression tests.
- This plan contributes the six explicit `[dind]` job overrides and the
  live split-routing invariant.

The coder reconstructs those reviewed diffs from fresh `main` on one new
integration branch. MRs !2362 and !2392 remain evidence branches and are not
merged independently. Their final dispositions point to the atomic MR. This
is required because all three lanes overlap `.gitlab-ci.yml` and the same
two Rust test files; no merge order can independently earn valid exact-head
acceptance without temporarily misrouting either native or nested work.

## Gate accompaniment — two test families, combined

Both `.gitlab-ci.yml`-surface gates require a changed declared test path with
non-hollow content. RED against the pre-change tree, GREEN against the
combined implementation branch.

1. **`tooling/rust/crates/agov/tests/gate_change_check.rs`** — preserve MR
   !2392's executable negative proving a shared CI change cannot satisfy only
   one of the two active gates. Add the lighter `4ghp` A1 live-registry
   guard: `.gitlab-ci.yml` is a surface for exactly `gate-change-check` and
   `ci-needs-implication`, and every declared surface/test path resolves.

2. **`tooling/rust/crates/agov/tests/ci_needs_implication.rs`** — add
   `live_configuration_declares_split_runner_routing()` alongside MR
   !2392's native/container-free packaging invariant and mutation negative.
   Parse the live `.gitlab-ci.yml` and assert all three of:
   - top-level `default.tags == ["podman", "linux", "amd64"]`;
   - each of the six named jobs declares `tags: ["dind"]`;
   - no other job defines its own `tags:` key.
   This is a direct regression guard for the split: a future edit that drops
   the default, mis-tags a nested job, or lets an ordinary job self-tag onto
   the `dind` runner fails here instead of racing or failing nested workloads
   silently. It replaces the prior version's `default.tags`-only assertion.

## Acceptance criteria

1. `agov gate change-check --base-ref origin/main` exits 0 on the complete
   atomic diff: ordinary default, native/Windows template split, six nested
   overrides, and both declared test paths.
2. Each newly introduced invariant has recorded RED evidence against the
   appropriate known-bad pre-change fixture/tree and GREEN evidence on the
   combined head. Existing MR !2392 negative evidence may be reused only when
   its tested input and assertion are byte-equivalent; otherwise rerun it.
   `shared_surface_requires_each_active_gates_own_test_evidence` remains
   unchanged and green.
3. `cargo test --manifest-path tooling/rust/Cargo.toml -p agov` — full local
   suite green.
4. **Runner and executor preconditions re-verified at implementation time.** An
   authenticated GitLab runner API read shows `55975189` with
   `tag_list: ['dind']` and `run_untagged: false`, and `55908035`
   (`priv runner`) still with the ordinary tuple. Fail closed if `55975189`
   still carries `podman`/`linux`/`amd64` or `run_untagged: true`.
   The first `[dind]` job must also pass executor preparation without the
   former duplicate-`/cache` error.
5. One canonical exact-head MR pipeline in which:
   - the MR-pipeline `dind` jobs (`agov:oci:mr-build-scan`,
     `rust:tooling:oci-pull-test`) demonstrably **select runner `55975189`**
     — shown from the job log's runner line, not from API-reported
     availability — **and** their previously failing nested steps now
     succeed (nested Trivy runs; the rootless-Podman pull completes);
   - ordinary jobs still run on `priv runner` (`55908035`);
   - no ordinary job is observed selecting the `dind` runner.
6. `agov:oci:release`, `release:verify`, `rust:tooling:preview:windows-package`,
   and `rust:tooling:build:windows:preview` run only in the tag pipeline or as
   manual/allowed-failure jobs, so their `dind` routing cannot be proven in
   this MR pipeline. It is covered by test 2 (live-config assertion) and
   **deferred to the v2.9 tag pipeline for execution proof**. The governor
   separately authorizes that tag; routing tests do not substitute for the
   eventual release-job result.
7. No paid failed job is retried merely to seek green; the accepted run
   contains the correction.
8. Registry-push/signing variables are protected, masked where applicable,
   and scoped only to jobs that require them—preferably
   `agov:oci:release`—rather than inherited by every `[dind]` job.
9. `release:verify` is explicitly not an independent verification leg
   because it runs on the same runner that pushes. The download-based,
   anonymous `agov dist post-publication-check` remains unchanged and is the
   independent published-bytes check.

## Negative test (per `gate-self-test-rule.md`)

This plan adds and changes tests and adds `tags:` routing; it does not modify
either gate's own logic (`gate_change.rs`, `ci_rules.rs`). The negative-test
obligation is satisfied by RED-then-GREEN on the two revised tests (criterion
2), with the RED transcripts and expected reasons in the implementation MR
description, plus the canonical exact-head pipeline log.

## Affected files

- `.gitlab-ci.yml` — reconstruct MR !2362's ordinary default, MR !2392's
  native Linux / Windows DinD template split, and the six `[dind]` overrides.
- `tooling/rust/crates/agov/tests/gate_change_check.rs` — preserve the MR
  !2392 negative and add the live-registry guard.
- `tooling/rust/crates/agov/tests/ci_needs_implication.rs` — preserve MR
  !2392's native packaging tests and add split-routing tests.
- one implementation completion/Gate 2 handoff coordination record.

No schema, migration, or release-metadata change. No new dependency. No
source change in `agov` / `agov-mcp`.

## Downstream impact

Classification: **BREAKING** for foundry CI because observable runner
placement and mandatory Linux packaging change atomically. Foundry migrates
through the one integration diff and its atomic rollback; direct adopters and
downstream consumers receive no shipped `agf/` contract or migration change.
Evidence is pipeline `2823154949`, the `gpss` atomic-unit analysis, and
binding decision `jmsq`.

## Risks

- **Runner state is volatile.** Current API evidence satisfies tag isolation,
  but criterion 4 rechecks immediately before acceptance and criterion 5
  requires actual selection plus nested success. The operator-reported
  `/cache` repair is not treated as proven before that job.
- **`tags: [dind]` moves the failure if the runner cannot actually run
  nested workloads.** Criterion 4 requires demonstrated nested success, not
  merely selection, precisely to catch this.
- **Three-way `.gitlab-ci.yml` collision** (MR !2362, MR !2392, and the old
  !2373 plan) — issue #123. One Codex-owned integration branch is the sole
  implementation lane; the old MRs are evidence/superseded inputs.
- **Release-pipeline job placement.** Moving `agov:oci:release` and
  `release:verify` to a different runner is release-integrity-relevant and is
  the reason infosec review is required (see `advisory_consultation`).
  Because both run on runner 55975189, a compromise there is treated as a
  release-integrity incident, not merely a CI failure; same-runner
  `release:verify` cannot clear that incident.
- **Deferred tag-pipeline verification** (criterion 5) means a
  mis-configuration on `55975189` affecting only the release jobs would not
  surface until the next tag pipeline. Accepted, stated, and bounded by test
  2. The first v2.9 tag pipeline is supervised: runner selection,
  provenance/scan output, registry push, and the later anonymous
  post-publication check are watched to terminal state before the release is
  trusted.

## Rollback

Revert the atomic `.gitlab-ci.yml` and two-test-file diff together. Do not
revert only the native packaging or only the routing overrides, because either
partial rollback recreates an invalid runner assignment. No runner-host
configuration is changed by this plan; host rollback remains an operator
decision.

## Evidence

- `docs/coordination/20260905T123338Z-coder-tester-6bkq.md` — original
  finding: dedicated-runner fix confirmed, !2362 gate-blocked.
- `docs/coordination/20260905T195732Z-coder-tester-83nk.md` — the revision
  handoff: split-routing requirement, the six jobs, the acceptance bar, the
  now-superseded sequencing ahead of MR !2392.
- `docs/coordination/20260905T193012Z-contribution-auditor-4ghp.md` — the
  now-superseded CLEAR of the prior narrow scope; findings A1 (registry
  literal brittleness) remains incorporated, while A2 is resolved by
  superseding rather than amending or merging MR !2362.
- `docs/coordination/20260905T200607Z-coder-tester-hqps.md` and issue #123 —
  the plan/workstream collision-detection gap this plan hits three ways.
- `docs/plans/20260903T045619Z-ci-mandatory-needs-implication-correction.md`
  — the deliberate coarse-surface decision this plan follows.
- Pipeline `2823154949`, jobs `16326536339` / `16326536352` — the nested-
  workload failures on the ordinary runner that motivate the split.
- `docs/coordination/20260905T215114Z-planner-architect-gpss.md` and
  `docs/coordination/20260906T072233Z-governor-reviewer-jmsq.md` — atomic
  unit proposal and binding adoption/assignment.
- `.agf/gates.yaml` — `.gitlab-ci.yml` declared a surface for both gates.
- `.gitlab-ci.yml` — the six named jobs confirmed present.
