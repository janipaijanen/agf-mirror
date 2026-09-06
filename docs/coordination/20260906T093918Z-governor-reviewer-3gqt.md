---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260906T093918Z-governor-reviewer-3gqt
timestamp: 2026-09-06T09:39:18Z
model: human
immutable: true
references-to:
  - 20260906T090930Z-coder-tester-g7mh.md
  - docs/plans/20260905T145109Z-route-all-jobs-gate-accompaniment.md
topic: v290-atomic-runner-routing-gate2-approved
requires-ack: false
artifacts_owned:
  - .gitlab-ci.yml
  - tooling/rust/crates/agov/tests/ci_needs_implication.rs
  - tooling/rust/crates/agov/tests/gate_change_check.rs
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2 approved and merged: MR !2430 — atomic v2.9 runner routing and native packaging

The governor's direct statement in the active session (2026-09-06,
transcribed by this Claude contribution-auditor session): **"gates
approved.. proceed."** On the governor's explicit answer to a scope
confirmation, this decision covers **MR !2430 only** — not the `a36j`
root `AGENTS.md` startup sync and not the universal-polling plan
(MR !2405), which stays behind its unresolved `p6j5` review.

## What was approved

MR !2430 exact source head
`91cb81067f14b6f5b250382367abf3a177f60b4b`, implementing the Gate 1-approved
atomic unit in `docs/plans/20260905T145109Z-route-all-jobs-gate-accompaniment.md`
(Gate 1 approved 2026-09-06T07:56:04Z, citing `4sb8` / `kkph` NO OBJECTION).
Handoff `20260906T090930Z-coder-tester-g7mh`.

Acceptance evidence at that head:

- Canonical MR pipeline `2823963799` terminal green (22 required jobs; the
  two Windows preview jobs remain manual/allowed-failure).
- Runner-selection evidence recorded in the handoff: `agov:oci:mr-build-scan`
  (job `16330500985`) and `rust:tooling:oci-pull-test` (job `16330500998`)
  both selected runner `55975189` (`dind`) and passed, executor preparation
  completed without the former duplicate-`/cache` error; every observed
  ordinary job selected `55908035` (`priv runner`); no ordinary job selected
  the DinD runner.
- Gate-mechanics self-test present (per `always/gate-self-test-rule.md` for a
  modification): RED before the six `[dind]` overrides —
  `live_configuration_declares_split_runner_routing` failed naming all six
  jobs `found None`; the registry mutation negative rejects an unresolved
  declared path. Both re-run GREEN at `91cb8106`. The two changed Rust test
  files are those accompaniment guards.

## Stale-target disposition

Pipeline `2823963799` ran with `main` at `9fc2f63a`; `main` was at
`826811ca` at merge time. The intervening advance (MRs !2431, !2432) is
coordination-documents only and touches neither `.gitlab-ci.yml` nor any
path MR !2430 changes. On the governor's decision, the merge proceeded on
the existing green pipeline without a fresh exact-head run.

## Merge

Merged via the platform API with `sha` pinned to
`91cb81067f14b6f5b250382367abf3a177f60b4b`, `squash=false`.

- Merge commit: `75266d663883a2cb073078bc7984729479ea1f96`
- First parent `826811ca66391c2e801d51a24e3bc56fdee2394d` (reviewed target).
- Second parent `91cb81067f14b6f5b250382367abf3a177f60b4b` (reviewed source).
- Diff vs first parent limited to `.gitlab-ci.yml` and the two
  gate-accompaniment test files (373 insertions, 12 deletions).
- `agf/VERSION` unchanged at `2.8.0`.

## What this does NOT approve

No version bump, tag, release, or publication. The v2.9.0 release-publication
plan (`docs/plans/20260831T050110Z-v290-release-publication.md`) and its
Gate 1/Gate 2 remain separate and pending. This decision also does not
approve MRs !2362, !2392, or !2373 (superseded evidence inputs — close
separately), the `a36j` startup sync, or any Propeller / universal-polling
gate.
