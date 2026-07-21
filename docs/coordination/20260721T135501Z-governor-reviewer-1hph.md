---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260721T135501Z-governor-reviewer-1hph
timestamp: 2026-07-21T13:55:01Z
model: codex-gpt-5
immutable: true
references-to: []
topic: adoption-time-evidence-clarity-authorization
requires-ack: false
artifacts_owned:
  - docs/plans/adoption-time-evidence-clarity.md
---

# Correct adoption-time claims at their evidence strength

The governor-reviewer reports that mechanical AGF installation has taken no
more than ten minutes in projects where the maintainer deployed it. The public
“an afternoon to install” claim is therefore misleading, while “twenty-minute
path” ambiguously implies a conflicting total adoption time.

The governor approves the plan direction to:

- report the observed ten-minute upper bound specifically for mechanical
  installation in maintainer deployments;
- distinguish installation from variable initial commissioning;
- state that independent-adopter validation is still pending;
- replace “twenty-minute path” with “short evaluation-and-install path”; and
- qualify retained governor-time ranges as maintainer-informed planning
  estimates rather than independently validated benchmarks.

This approves Gate 1 for `docs/plans/adoption-time-evidence-clarity.md`. It does
not silently amend the frozen v2.3.1 release scope.
