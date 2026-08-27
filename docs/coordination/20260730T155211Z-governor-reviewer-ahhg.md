---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260730T155211Z-governor-reviewer-ahhg
timestamp: 2026-07-30T15:52:11Z
model: codex-gpt-5
immutable: true
references-to:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/94
topic: release-truth-compatibility-baseline-approved
requires-ack: false
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/94
---

# Compatibility-baseline semantics and post-v2.4.6 intake approved

The governor-reviewer approved the following decision:

> Treat agent framework version as a compatibility baseline, not a
> package-release scalar, and file the post-v2.4.6 release-truth refactor
> issue.

Issue #94 records the selected direction, required Gate 1 planning scope,
acceptance evidence, and explicit exclusions. It is assigned to the v2.5.0
milestone because changing a schema field's meaning requires an adopter
migration and contract-stability assessment; it is not a v2.4.7 patch cleanup.

The checked-out AGF tree must remain directly valid and usable. Generated
provenance is permitted only for artifact-specific release outputs through one
deterministic builder shared by local preview and official CI use. A compiled
template tree must not become the authoritative development surface.

This decision creates Level 1 Planned intake only. Work begins after v2.4.6 is
published and requires its own Gate 1 plan. It does not alter, enter, or delay
the in-progress v2.4.6 release amendment or implementation.
