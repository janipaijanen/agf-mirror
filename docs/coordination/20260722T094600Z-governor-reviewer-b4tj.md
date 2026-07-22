---
kind: handoff
from: governor-reviewer
to: planner-architect
id: 20260722T094600Z-governor-reviewer-b4tj
timestamp: 2026-07-22T09:46:00Z
immutable: true
references-to:
  - 20260722T093247Z-planner-architect-2jba
  - 20260722T094139Z-contribution-auditor-hr4p
  - 20260722T082058Z-coder-tester-2mgp
topic: v2-4-0-workstream-c-plan-revision-and-decision-routing
requires-ack: false
artifacts_owned:
  - docs/plans/v2.4.0-workstream-c-mcp-cli-telemetry.md
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Revise the Workstream C plan to resolve hr4p's parser and attribution-fallback contract findings.
    - Publish a concise governor decision request covering the real sequencing conflict with draft MR !363 and the infosec-sentinel consultation disposition.
    - Keep the plan pending Gate 1 until those binding governor choices are recorded, then route the exact revised head for focused re-review.
  must_read:
    - docs/plans/v2.4.0-workstream-c-mcp-cli-telemetry.md
    - docs/coordination/20260722T094139Z-contribution-auditor-hr4p.md
    - docs/coordination/20260722T083323Z-contribution-auditor-n8s6.md
    - docs/coordination/20260722T082058Z-coder-tester-2mgp.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Revise Workstream C and route the unresolved decisions

Planner-architect should take the response to `hr4p` now. Resolve the
non-authority-dependent plan defects directly: define the exact default-deny
token handling for equals-form flags, delimiters, positional/repeated/short
arguments, unknown or missing values, and `argv[0]`; make direct CLI explicit;
define coherent all-or-none MCP marker parsing; and remove the already-settled
historical-record question from the open list.

Do not infer either remaining binding choice. Draft !363 is currently held
until after v2.4.0 while Workstream C is a v2.4.0 blocker touching the same ROI
files. Present the governor with the concrete sequencing options and a
recommendation. Likewise, do not mark infosec-sentinel consulted without an
actual disposition: request either consultation or an explicit governor
waiver. Keep implementation blocked until the revised plan passes focused
review and receives Gate 1 approval.
