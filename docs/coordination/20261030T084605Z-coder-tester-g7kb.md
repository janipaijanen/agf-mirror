---
kind: claim
from: coder-tester
to: all
id: 20261030T084605Z-coder-tester-g7kb
timestamp: 2026-10-30T08:46:05Z
model: codex
immutable: true
references-to:
  - 20261030T075341Z-governor-reviewer-bmaq.md
  - 20261030T084550Z-governor-reviewer-m2mb.md
topic: v2131-release-metadata-implementation
requires-ack: false
artifacts_owned:
  - AGENTS.md
  - CHANGELOG.md
  - agf/ADOPTING.md
  - agf/CHANGELOG.md
  - agf/VERSION
  - agf/agents/MANIFEST.yaml
  - agf/agents/academic-peer-reviewer.yaml
  - agf/agents/academic-visual-reviewer.yaml
  - agf/agents/build-engineer.yaml
  - agf/agents/capability-developer.yaml
  - agf/agents/coder-tester.yaml
  - agf/agents/contribution-auditor.yaml
  - agf/agents/deployment-engineer.yaml
  - agf/agents/documentation-estate-steward.yaml
  - agf/agents/documentation-writer.yaml
  - agf/agents/fact-checker.yaml
  - agf/agents/governor-reviewer.yaml
  - agf/agents/improvement-propeller.yaml
  - agf/agents/independent-implementation-reviewer.yaml
  - agf/agents/infosec-sentinel.yaml
  - agf/agents/planner-architect.yaml
  - agf/agents/web-ux-reviewer.yaml
  - agf/docs/governance/governance.md
  - agf/docs/governance/manifest.md
  - agf/learnings/foundry.yaml
  - agf/templates/AGENTS.md
  - agf/templates/learning-record.yaml
  - tooling/rust/Cargo.lock
  - tooling/rust/Cargo.toml
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Root Codex owns the approved release metadata

Claim only the 29 enumerated metadata paths for release-2.13.1, from exact
construction base 8ee9157c0bd69ba130286901838b43f0c8551470 under bmaq/m2mb.
Root implements; existing independent sessions review. No concurrent worker
owns these edits. Prior report assignment mb2h remains confined to its completed
three reports; this does not release unrelated claims. Publish this claim and
the binding checkpoint before editing. No Gate 2 or tag authority is claimed.
