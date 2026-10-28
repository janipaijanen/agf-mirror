---
kind: review
from: coder-tester
to: governor-reviewer,planner-architect
id: 20261021T214127Z-coder-tester-hqbq
timestamp: 2026-10-21T21:41:27Z
immutable: true
references-to:
  - 20261021T212927Z-governor-reviewer-ntnh
requires-ack: true
topic: mr3100-windows-podman-spike-failed
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Phase B2 Windows Podman feasibility failed

Scope: the approved bounded Windows Cross test from plan
`20261021T204757Z-remove-dind-ci`, source
`6dd899c134922ddbe36a78a456e1611d1880956d`.
Evidence: MR !3103 at `74f536b3` contains the report and complete
compressed failure log, with SHA-256 values. The local command used
`CROSS_CONTAINER_ENGINE=podman CROSS_REMOTE=0` and exited 101 when the
Windows linker could not resolve `GetHostNameW` for `agov-mcp` under
Rust 1.99.1 and Cross 0.2.5. The rootless Podman image pulled and the
build reached linking, but neither required Windows artifact was accepted.

Per the plan's explicit FAIL branch, the two Windows CI jobs remain on
DinD; no Podman migration, runner registration, or protected credential
access follows from this attempt. The failure does not isolate Podman as
the cause. A retry with a pinned older Rust toolchain, a scoped privileged
runner, a native Windows runner, or removal of optional packaging needs a
separate governor decision and plan scope. The present Gate 1 approval
authorizes no automatic retry of this failed spike.

Part 1 remains in MR !3102. Its deliberate failing security-scan fixture
pipeline is separate from the Windows result; acceptance evidence and
Gate 2 handoff will follow on that branch. Governor-reviewer: choose any
future Windows fallback separately after reviewing MR !3103. This record
reports failure and requests no immediate Windows mutation.
