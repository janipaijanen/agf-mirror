---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20261029T195322Z-coder-tester-amkh
timestamp: 2026-10-29T19:53:22Z
model: gpt-6-astra
immutable: true
references-to:
  - 20261029T162805Z-governor-reviewer-dn8m.md
  - 20261029T190118Z-governor-reviewer-nqsp.md
  - 20261029T192400Z-governor-reviewer-nkkg.md
  - 20261029T194912Z-governor-reviewer-nskj.md
topic: distribution-profiling-implementation-gate2
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review profiling for Gate 2 after acceptance amendment merge and exact-head CI.
  must_read:
    - docs/plans/20260920T083259Z-distribution-scan-profiling-slice-b.md
    - docs/reviews/dist-scan-profiling/20261029T194217Z-baseline.md
  do_not_touch:
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Feature-gated distribution profiling and a qualified baseline

Implementation source 08ae26b8c80812bef8848a67780a5a40df908b80 adds invocation-local
counters and timers, the feature-only diagnostic, a generated tracked/untracked/
hidden/in-root-symlink fixture, and the bounded telemetry-vocabulary correction.
Default behavior and command inventory remain unchanged. Report-only commit
11e96a285210fc23f23ff3ee35c1a1f4c37a0088 preserves the exact clean measured source,
raw samples, environment, reproduction steps and limitations. Subsequent merges
bring current approved plans/decisions into the candidate; no code changes result.

Core tests: 37 default / 38 feature. CLI tests: 497 default / 498 feature.
Workspace all-features: 839. Manual fixture baseline: 1. Existing three OCI tests
remain ignored; the manual baseline is explicitly run rather than counted as a
normal-suite execution. The full feature audit initially failed for missing ROI
vocabulary, then passed unchanged after the approved nkkg correction. Shared-gate
controls and CI-pin test-delta compatibility pass.

Independent exact-head review of 11e96a28 found no source/report issues, but BLOCKED
on the original cold/warm requirement. The explicit nskj amendment in !3371 was
reviewed CLEAR at 1abcfffc0de2ed1a8f790e8ad32ea3de5ad3d261. It accepts honest
fresh-process samples with uncontrolled caches and defers controlled comparison;
no cache-effect, speedup, optimization, CI-critical-path or pristine-checkout claim
is supported. The first sample is not cold. Gate 2 must await that amendment's
merge, final candidate review and successful exact-head MR CI.

The measurements show stable call counts and dominant aggregate filter duration
on this host; they do not isolate causal cost or establish a faster implementation.
No optimization, matcher behavior, CI wiring, dependency, outside-symlink policy,
release/tag action or builder-image work is included.
