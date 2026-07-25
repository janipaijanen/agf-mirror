---
kind: handoff
from: governor-reviewer
to: planner-architect
id: 20260723T015531Z-governor-reviewer-p7jq
timestamp: 2026-07-23T01:55:31Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260723T002811Z-governor-reviewer-5stq.md
  - 20260723T012716Z-governor-reviewer-re1q.md
topic: v241-accessibility-prose-planning
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - After v2.4.0 publication and public-surface verification, use GitLab milestone v2.4.1 and issue #56 to prepare the accessibility/prose patch-release Gate 1 plan.
    - Verify patch-version compatibility, keep the milestone scope bounded to #51, #55, and explicitly accepted publication regressions, and route the plan to contribution-auditor before governor submission.
  must_read:
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/milestones/1
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/work_items/51
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/work_items/55
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/work_items/56
  do_not_touch:
    - release/v2.4.0
    - .gitlab-ci.yml
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Plan v2.4.1 after v2.4.0 is published

The governor establishes GitLab milestone `v2.4.1` as **Level 1 Planned** with
no due date. It is a candidate compatible patch release, not current release
authority. Do not create a release branch, version bump, tag, or GitLab Release
from this handoff.

After v2.4.0 is published and its public surfaces are verified, prepare one
Gate 1 plan through issue #56. Candidate scope is:

- issue #51: substantive explanatory prose before each in-scope public figure;
- issue #55: the deferred Mermaid narrow/mobile and screen-reader semantic
  remediation from governor decision `5stq`;
- only genuine compatible regressions found during v2.4.0 publication and
  explicitly accepted into scope by the governor.

Exclude #53 CI/build architecture, #54 coordination consolidation, review
taxonomy/risk-role work, operating-efficiency drafts, schema/role changes, new
tooling surfaces, and unrelated features. If the proposed work ceases to be a
compatible correction, report that semantic-version boundary instead of
forcing it into v2.4.1.

The plan must close the figure/embed inventory, state page-specific prose and
accessibility acceptance evidence, include non-native-English readability,
narrow/mobile, keyboard, zoom/reflow, and screen-reader-oriented checks, and
declare contribution-auditor and infosec consultation status. Route the draft
to contribution-auditor before Gate 1 submission. Release work !446 remains
higher priority until complete.
