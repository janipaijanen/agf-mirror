---
id: 20260905T145109Z-route-all-jobs-gate-accompaniment
status: pending-gate1
topic: route-all-jobs-gate-accompaniment
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: >-
      No infosec-sentinel review of this plan exists. Recorded as waived, not
      consulted, so the metadata does not assert a review that did not
      happen. The change adds test coverage only; it does not touch secrets,
      credentials, or access control.
  - role: contribution-auditor
    status: waived
    reason: >-
      No contribution-auditor disposition exists for this plan. This is gate
      mechanics (a `.gitlab-ci.yml`-touching change), so an independent
      review is appropriate regardless; the governor may hold Gate 1 for an
      auditor pass.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate-accompaniment disposition for MR !2362 (route all jobs to the dedicated runner)

Disposes the `agov:governance:validate` failure recorded in
`docs/coordination/20260905T123338Z-coder-tester-6bkq.md`: MR !2362's
`default: tags:` addition to `.gitlab-ci.yml` is a changed surface for both
`ci-needs-implication` and `gate-change-check` (`.agf/gates.yaml:3-16`), and
neither gate's declared test path changed in that MR, so `agov gate
change-check` correctly blocked it.

**This is a gate change.** Per `simplicity-rule.md`, any edit to
`.gitlab-ci.yml` is gate mechanics and ineligible for fast-track regardless
of diff size or how unrelated it looks to the gates' own logic — that is by
design, not an edge case. `docs/plans/20260903T045619Z-ci-mandatory-needs-
implication-correction.md` (Gate 1 approved 2026-09-03) explicitly chose to
register `.gitlab-ci.yml` as a shared surface for *two* active gates and
disclosed the resulting doubled accompaniment burden as an accepted cost,
specifically to prevent "this change doesn't really affect the gate"
reasoning from becoming an unstated exception. !2362 is exactly that
scenario arriving two days later. The correct disposition is therefore to
satisfy the accompaniment requirement with genuine, non-hollow test
evidence for both gates — not to propose narrowing the surface registry,
which would reopen a decision the governor made deliberately and recently
with the tradeoff stated.

## What already exists (verified before proposing new work)

- `tooling/rust/crates/agov/tests/ci_needs_implication.rs`'s
  `shared_surface_requires_each_active_gates_own_test_evidence` (added by
  the 2026-09-03 plan) already proves the *generic* mechanism — touching one
  gate's declared test alone fails, touching both passes — against synthetic
  fixture content. This does not need to be re-proven; it is not the gap.
- `gate_change_check.rs` has no test that reads the **real**,
  live `.agf/gates.yaml` in this repository — every existing test constructs
  a synthetic temp-repo registry (`BASE_REGISTRY`, `shared-ci-surface-
  registry.yaml`, etc.). This is a genuine, previously-uncovered gap: nothing
  currently catches accidental drift in the actual registry governing this
  project's own gates.
- `ci_needs_implication.rs` already has the live-configuration pattern to
  follow (`live_configuration_preserves_every_mandatory_needs_implication`,
  line 29), but nothing in it asserts on `default:` — only on `needs:`
  edges. `default: tags:` is exactly the kind of routing config today's
  runner incident showed has no regression guard at all today: MR !2362
  exists because no job declared `tags:`, so every job silently raced
  shared vs. dedicated runners with no test catching it.

## The design

Two new tests, each touching one gate's declared test path with content
genuinely tied to what !2362 changes — not a token edit:

1. **`tooling/rust/crates/agov/tests/gate_change_check.rs`** — add
   `live_gates_registry_matches_declared_shape()`: read this repository's
   actual `.agf/gates.yaml` (via `repo_root()`, already used elsewhere in
   this file for `AGOV_SCHEMA_DIR`) and assert its parsed gate ids, surfaces,
   and declared test paths equal a hard-coded expected value. This catches
   accidental drift in the registry itself — adding, removing, or
   mis-scoping a gate or surface without anyone noticing — the same class of
   gap `ci_needs_implication.rs`'s own live-configuration test closes for
   the needs-graph.

2. **`tooling/rust/crates/agov/tests/ci_needs_implication.rs`** — add
   `live_configuration_declares_the_expected_default_runner_tags()`: read
   the live `.gitlab-ci.yml` and assert its top-level `default.tags` equals
   `["podman", "linux", "amd64"]`. This is a direct regression guard for
   !2362's own change: if a future edit silently drops or narrows the
   default tag routing, this test fails instead of jobs quietly racing
   shared runners again.

Both land in the same commit as !2362's `default: tags:` addition, so the
implementation MR's changed-file set includes `.gitlab-ci.yml` plus both
declared test paths, satisfying `agov gate change-check` for both gates with
evidence that is actually about the change, not merely present to pass a
linter.

## Rejected alternative

**Narrowing `.agf/gates.yaml`'s surface declarations to be section- rather
than file-granular** (so a `default:`-only edit wouldn't count as touching
either gate). Rejected because the 2026-09-03 plan already considered and
disclosed this exact tradeoff — coarse, file-level surfaces catch changes a
finer-grained declaration could miss, at the cost of accompaniment burden on
every edit — and the governor accepted the coarse version explicitly two
days ago. Reopening it now, under pressure from one blocked MR, is the
quiet-weakening pattern `gate-self-test-rule.md` exists to prevent. If the
governor wants to revisit the granularity question, that is a separate,
explicit decision, not a side effect of unblocking !2362.

## Affected files

- `tooling/rust/crates/agov/tests/gate_change_check.rs` (new test)
- `tooling/rust/crates/agov/tests/ci_needs_implication.rs` (new test)
- `.gitlab-ci.yml` (already changed by !2362; no further edit needed here)

No schema, migration, or release-metadata change. No new dependency.

## Test and validation strategy

1. Add both tests; confirm each fails first against the pre-!2362 tree
   (RED — proving they test something real) and passes against !2362's
   branch (GREEN).
2. `agov gate change-check --base-ref origin/main` on the combined branch
   (!2362's `.gitlab-ci.yml` change plus these two test files) — must exit 0.
3. `cargo test --manifest-path tooling/rust/Cargo.toml -p agov` — full
   local suite green, including the pre-existing
   `shared_surface_requires_each_active_gates_own_test_evidence` (unchanged,
   confirms no regression to the generic mechanism).
4. One canonical exact-head pipeline on the (rebased) !2362 MR.

## Negative test (per `gate-self-test-rule.md`)

This plan does not modify either gate's own logic (`gate_change.rs`,
`ci_rules.rs`) — it adds test coverage only. The negative-test obligation is
therefore satisfied by RED-then-GREEN on the two new tests themselves (step
1 above), not by a separate transcript against the gate mechanism, which is
unchanged and already covered by its own existing suite.

## Risks

- **The live-registry test is brittle to future legitimate registry
  growth.** Any future gate addition or surface change requires updating
  `live_gates_registry_matches_declared_shape()`'s expected value in the
  same commit — this is the intended accompaniment behavior, not a defect,
  but is stated so it isn't mistaken for test flakiness later.
- **Scope creep risk:** this plan is deliberately narrow (two additive
  tests). It must not be used to justify further, unrelated changes to
  `.gitlab-ci.yml` on the same branch.

## Rollback

Revert both new test files. No effect on `.gitlab-ci.yml`'s own content,
which reverts independently per !2362's own rollback note (delete the
`default:` block).

## Evidence

- `docs/coordination/20260905T123338Z-coder-tester-6bkq.md` — the original
  finding: dedicated-runner fix confirmed working, !2362 gate-blocked.
- `docs/plans/20260903T045619Z-ci-mandatory-needs-implication-correction.md`
  — the prior, deliberate design decision this plan follows rather than
  reopens.
- `.agf/gates.yaml:3-16` — the current registry, verified to declare
  `.gitlab-ci.yml` as a surface for both gates.
- `tooling/rust/crates/agov/tests/ci_needs_implication.rs:29,134-155` and
  `tooling/rust/crates/agov/tests/gate_change_check.rs` (full file) — read
  in full before writing this plan; confirmed no existing test covers either
  gap this plan closes.
