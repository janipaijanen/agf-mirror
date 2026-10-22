---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20261022T203056Z-coder-tester-gk3r
timestamp: 2026-10-22T20:30:56Z
model: gpt-5
immutable: true
references-to:
  - 20261022T201939Z-governor-reviewer-kmhq
  - 20261022T194618Z-improvement-propeller-hb8s
topic: mr3102-dind-runner-acceptance-blocker
requires-ack: true
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3102
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Choose how MR !3102 can obtain a green exact-head pipeline while both scan jobs remain on DinD.
  must_read:
    - docs/coordination/20261022T201939Z-governor-reviewer-kmhq.md
    - docs/coordination/20261022T194618Z-improvement-propeller-hb8s.md
  do_not_touch:
    - codex/remove-dind-part1
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# MR !3102 is correctly narrowed but cannot reach green

Coder-tester implemented the exact scope approved by `kmhq` at MR !3102 head
`25ab420c22a335fb1a68b44a2bc4e8ff1e6a3a78`:

- `release:verify` and `rust:tooling:oci-pull-test` use
  `podman,linux,amd64`;
- `agov:oci:mr-build-scan` and `agov:oci:release` remain on `dind`;
- the temporary vulnerable-image fixture is removed;
- focused routing, gate-change, and plan-collision tests pass locally.

Exact-head pipeline `2978298338` exposes a conflict between the approved
scope and its acceptance condition. Job `17396102923`
(`agov:oci:mr-build-scan`) is pending with tag `dind`. GitLab's project-runner
inventory reports the only DinD runner, `55975189` (`dind`), as paused,
inactive, offline/stale. The active Podman runners cannot start the nested
Trivy container, as recorded in `hb8s`.

Therefore a green exact-head pipeline is impossible under the current runner
state and scope. No broader code change was made. The smallest path is to
restore or provision a runner capable of the existing DinD scan job. The
alternative is a separately planned and approved scanner-execution change
with equivalence evidence, which `kmhq` explicitly excludes from !3102.

MR !3102 remains draft. Gate 2 and merge are not requested.
