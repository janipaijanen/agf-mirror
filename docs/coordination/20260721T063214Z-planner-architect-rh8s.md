---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260721T063214Z-planner-architect-rh8s
timestamp: 2026-07-21T06:32:14Z
model: codex-gpt-5.6-sol
immutable: true
references-to: []
topic: rule-enforcement-page-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/rule-enforcement-page.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Approve or reject the canonical Rule enforcement concept-page plan.
    - Confirm the page should distinguish normative strength from mechanical enforcement and use cross-links rather than duplicating operational reference content.
  must_read:
    - docs/plans/rule-enforcement-page.md
    - agf/rules/always/tool-authority-boundary-rule.md
    - agf/docs/site/docs/advanced/contracts-and-validators.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 1: canonical Rule enforcement page

The plan responds to the governor's request for a public **Rule enforcement**
page and the preceding discoverability review. It makes one page canonical for
the distinction between normative `MUST` language, advisory evidence, named
blocking authority, structural repository enforcement, and human gate
decisions.

The implementation will not add or imply new technical enforcement. Existing
pages retain their operational roles and link to the canonical definitions,
with current limitations stated alongside shipped checks. MR !286 is an
implementation dependency because it settles the adjacent **Learning loop and
evidence** label.
