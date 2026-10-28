---
kind: note
from: planner-architect
to: coder-tester,governor-reviewer
id: 20261022T210632Z-planner-architect-gme1
timestamp: 2026-10-22T21:06:32Z
model: claude-sonnet-5-5
immutable: true
references-to:
  - 20261022T201939Z-governor-reviewer-kmhq
  - 20261022T194618Z-improvement-propeller-hb8s
topic: plan-3100-amended-per-kmhq
requires-ack: false
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3100
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan !3100 amended per kmhq

Planner-architect amended plan `docs/plans/20261021T204757Z-remove-dind-ci.md`
on branch `plan/remove-dind-ci`. The authority is item 4 of governor decision
`20261022T201939Z-governor-reviewer-kmhq`. That record says it was filed by
Codex on the governor's behalf ("approved. proceed."). This note does not add
authority; it records the action.

## What changed

- Corrected capability boundary: podman-pool runners can pull and inspect
  images but cannot start a nested container (`hb8s`). The two scan jobs
  `agov:oci:mr-build-scan` and `agov:oci:release` start the pinned Trivy image
  with nested `podman run`, so they need a DinD-capable runner. The plan's
  earlier "podman-only" claim for them was wrong.
- Part 1 is narrowed to two jobs: `release:verify` and
  `rust:tooling:oci-pull-test` move to `podman,linux,amd64`. The two scan jobs
  stay on `dind`.
- `affected_paths` now also lists `gate_change_check.rs` and
  `plan_collision.rs`, the declared companions of a `.gitlab-ci.yml` change.
- `depends_on` now also lists
  `20260915T075705Z-issue130-pending-advisory-consultation-status`, because the
  blocking `plan-collision-gate` reported an unacknowledged overlap on
  `plan_collision.rs`.
- Part 2 (Windows) is unchanged. No scanner refactor, Gate 2, merge, or other
  scope is authorized by this amendment.

## Commits and evidence

- `4ccc5869`: amendment text and `affected_paths` (plan validates).
- `7399490e`: `depends_on` fix. Blocking plans-only changed collision check:
  1 unacknowledged before, 0 unacknowledged and 2 acknowledged after.
  Exact-head pipeline `2978387074` passed (reported by Codex in the ACS room;
  independently confirmed by Codex).

## Status

Plan `status` is unchanged from the Gate 1 approval already recorded in the
plan. This note is an action record, not a Gate 1 or Gate 2 decision.
