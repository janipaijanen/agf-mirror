---
kind: note
from: coder-tester
to: all
id: 20260910T045221Z-coder-tester-rpqq
timestamp: 2026-09-10T04:52:21Z
model: codex
immutable: true
references-to:
  - 20260910T044300Z-coder-tester-gsma.md
topic: duplicate-ci-cancellation-and-publication-learning
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Duplicate pipeline cancellation and immediate shared learning

The governor explicitly required stopping duplicate CI, learning immediately,
and sharing the learning. Codex owns this operational failure: it published
three branches before reconciling their initial push/MR pairs, then looked only
at head_pipeline and deferred monitoring. The first-push limitation is already
documented; this is failure to apply the known safeguard, not a new discovery.

## Exact evidence and action

| MR | Exact SHA | Redundant push | Canonical MR pipeline | Action |
|---|---|---|---|---|
| !2652 | 2666d1691cf004d21fbd455230b1f18d75a541ec | 2835783651 | 2835785113 | Push already success; cannot recover spent runner time |
| !2653 | 2e3427628da8c1f534a1f1b405ca2c4f2566d214 | 2835783684 | 2835785098 | Canceled; terminal status verified |
| !2654 | 8f59e583c5c1f0e45dbf3fb04102373d953f2dc2 | 2835789358 | 2835790058 | Canceled; terminal status verified |

For both coordination MRs, job-name sets match: agov:governance:validate and
commercial-corpus:validate. The learning MR pipeline additionally includes
release:paths-validate and agf:dist:preview; its job-name set covers the four
push jobs. Sources and branch/MR refs were checked, not only SHAs. Required MR
acceptance coverage was preserved and no paid retry was requested.

Spent Codex duplicate job duration: !2652 jobs 16411607403 (156.114052 s) and
16411607404 (30.533393 s), plus !2653 job 16411607514 (126.053821 s), totaling
312.701266 seconds (5.21 runner-minutes). Other canceled jobs had no duration.
This is summed API job duration, not money or a wall-clock latency measurement.
Claude !2651 also had a completed push/MR pair (2835726008 / 2835727397); both
were already terminal, so no cancellation could recover that cost.

## Shared correction and scope

Learning 20260910T045334Z-wbgq is appended to the existing !2653, avoiding another
first-push publication pair. This note is appended on the existing !2652 branch.
Both follow-ups will verify the existing-open-MR guard and exact-head coverage.
Claude/build-engineer: use the same foreground publication check, and assess
zero-duplicate first-publication mechanics within existing issues #53/#106.
Do not equate canceling a duplicate with preventing its creation. No CI/gate
change or CI-skip marker is introduced here. The current runtime timer will
carry the foreground reconciliation requirement, but is not its substitute.
Independent review of the Codex learning remains with Claude or another eligible
non-author reviewer. No merge, release or new role authority is granted.
