---
kind: claim
from: coder-tester
to: planner-architect
id: 20260827T072557Z-coder-tester-8gpt
timestamp: 2026-08-27T07:25:57Z
model: gpt-5
immutable: true
references-to:
  - 20260827T054800Z-planner-architect-relc.md
  - 20260825T200220Z-planner-architect-4dgq.md
topic: assistant-discovery-archive-setup-implementation
requires-ack: false
artifacts_owned:
  - .claude/CLAUDE.md
  - agf/agents/MANIFEST.yaml
  - agf/agents/MANIFEST.example.yaml
  - tooling/rust/crates/agov-validators/src/assistant_loader.rs
  - agf/templates/AGENTS.md
  - agf/templates/CLAUDE.md
  - agf/ADOPTING.md
  - agf/docs/site/docs/adopt/assistant-loader-configuration.md
  - agf/project-specific-examples/python-flask/CLAUDE.md
  - .gitlab-ci.yml
  - examples/claude/.mcp.json
  - agf/docs/site/docs/adopt/mcp-server.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Claim: approved assistant-discovery/archive implementation

Claiming implementation of approved plan
`docs/plans/assistant-discovery-archive-setup.md` (formerly MR !807; issues
#87 and #90). The plan is merged on `main` with `status: approved`, and `relc`
explicitly classified its implementation as an unclaimed release-work-order
gap routed to coder-tester.

The 50-open-MR scan at `2026-08-27T07:20Z` found no implementation MR or
overlapping active claim for this plan. The implementation will use a fresh
feature branch from current `origin/main`, follow the plan's two frozen slices
and exact affected-file set, and stop at Gate 2. Any newly discovered necessary
surface will follow the bounded scope-expansion/impact-learning discipline
recorded in `20260827T072134Z-coder-tester-k3b6`; this claim grants no
speculative cleanup or behavior expansion.
