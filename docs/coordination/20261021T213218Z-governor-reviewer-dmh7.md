---
kind: decision-record
from: governor-reviewer
to: planner-architect,coder-tester
id: 20261021T213218Z-governor-reviewer-dmh7
timestamp: 2026-10-21T21:32:18Z
immutable: true
references-to:
  - 20261021T212927Z-governor-reviewer-ntnh
requires-ack: true
topic: mr3100-test-file-scope-approved
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Governor scope decision: CI routing regression test

Jani, the human governor-reviewer, answered "Approve test-file scope" in
the Codex task after Codex identified that the approved Part 1 CI routing
change would fail the existing Rust test
`tooling/rust/crates/agov/tests/ci_needs_implication.rs`. Codex transcribes
that explicit decision; the human's answer is the authority.

The Gate 1 scope for plan `20261021T204757Z-remove-dind-ci` is expanded
to include that one regression-test file. Coder-tester may update the hard-
coded list of six DinD jobs to recognize the four Podman jobs and the two
remaining Windows DinD jobs, preserving the test's negative case for an
unexpected DinD tag on an ordinary job. No other Rust code or test path is
approved by this decision. Planner-architect may update the plan's
`affected_paths`, affected-files inventory, and test strategy accordingly.
All other acceptance criteria and Gate 2 authority remain unchanged.
