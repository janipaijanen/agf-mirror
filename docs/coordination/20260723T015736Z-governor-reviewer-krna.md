---
kind: handoff
from: governor-reviewer
to: planner-architect
id: 20260723T015736Z-governor-reviewer-krna
timestamp: 2026-07-23T01:57:36Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260723T015531Z-governor-reviewer-p7jq.md
  - 20260723T012105Z-governor-reviewer-r3pq.md
topic: v250-portfolio-discovery
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - When higher-priority release work permits, use GitLab milestone v2.5.0 and issue #57 to inventory and critically shape the large early-stage portfolio.
    - Recommend one coherent minor-release character or split the ideas into smaller milestones; route the portfolio screen to contribution-auditor before drafting any Gate 1 release plan.
  must_read:
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/milestones/2
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/work_items/57
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/434
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/work_items/53
  do_not_touch:
    - release/v2.4.0
    - .gitlab-ci.yml
    - agf/schemas/
    - agf/rules/
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Discover v2.5.0 before planning a massive release

The governor establishes GitLab milestone `v2.5.0` as a no-deadline **Level 1
Planned discovery portfolio** for large ideas that may require a minor version.
The milestone is not a promise that every candidate ships together.

Use issue #57 as the intake and decomposition surface. Inventory review/risk
taxonomy, #53 build and correction architecture, public-experience/evidence
presentation, the open governance/tooling roadmap, and evidence-qualified
operating guidance. Trace every candidate to its issue, MR, or coordination
record. Listing is not adoption: explicitly adopt, split, defer, or reject each
candidate.

Group work by user outcome and dependency. Separate shipped product capability,
governance contract, internal process, research, and documentation maintenance.
Assess semantic-versioning impact, evidence strength, user value, risk,
reversibility, implementation and maintenance cost, specialist/research needs,
and stop conditions. If one bundle would become an unreviewable massive
release, recommend smaller milestones instead of forcing artificial cohesion.

Route the portfolio screen to contribution-auditor for adversarial review
before creating any umbrella Gate 1 plan. Do not implement, change schemas or
rules, create `release/v2.5.0`, bump versions, tag, or create a GitLab Release
from this handoff.

Priority remains: finish v2.4.0, then bounded v2.4.1 planning and authorized
post-release obligations. v2.5.0 discovery may use genuine waits but must not
delay those lanes.
