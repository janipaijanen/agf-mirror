---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260728T201031Z-planner-architect-jqbb
timestamp: 2026-07-28T20:10:31Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260728T200855Z-governor-reviewer-331n
  - 20260728T195654Z-planner-architect-qjr6
  - 20260728T192317Z-planner-architect-msds
topic: version-controlled-roadmap-page-implementation
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Implement the approved version-controlled roadmap-page plan from MR !758 head 9bc2147 on a feature branch based on current origin/main.
    - Before finalizing prose, challenge the planner position below, publish any dissent with evidence, and converge only where the source record supports agreement.
    - Run the plan's strict documentation, adopter-doc, diff, source/link, claim, and narrow-viewport checks; then request exact-head contribution-auditor and fact-checker review before Gate 2.
  must_read:
    - docs/plans/version-controlled-roadmap-page.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/755
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/757
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/758
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/57
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/78
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/81
  do_not_touch:
    - agf/VERSION
    - agf/CHANGELOG.md
    - any release branch or tag
    - v2.5 SoW scope, status, or disposition
    - issue or MR state
    - .claude/settings.local.json
---

# Implementation handoff: version-controlled roadmap page

Gate 1 is approved by `20260728T200855Z-governor-reviewer-331n`. The governor
also explicitly authorized the earlier uncertainty snapshot allowed by the
plan. Implementation may start now, but the page must not imply that the
pending and challenged MR !755 SoW is approved.

## Planner position to challenge

The first page should make the current uncertainty useful rather than hide it:

1. **Released baseline:** v2.4.5 is the current released baseline. State its
   maturity and link the exact tag/release evidence without calling the release
   wholly healthy while issue #81's clean-archive runbook-test defect remains
   open.
2. **Active train:** say plainly that no next-release train is approved. The
   MR !755 candidate may be summarized only as a proposed T0 + v2.4.6 +
   v2.5.0-rc.1 + conditional rc.2 + v2.5.0 sequence, paired with its open
   contribution-audit findings and pending role-lens consultations. If that
   summary is more likely to mislead than help, link the proposal without
   reproducing the train and explain why.
3. **Current work:** distinguish stabilization and current-contract repair
   from adopter-facing v2.5 product scope. Do not market delivery-enabling work
   as shipped product capability.
4. **Parked/radar/excluded:** prefer named source links and revisit triggers;
   do not copy the full issue/MR ledger or turn absence from the active train
   into rejection.
5. **Freshness:** use UTC observed-at data and a source revision that is honest
   at authoring time. Do not claim the page's containing commit hash inside the
   same commit; use a non-circular source identity such as the reviewed source
   snapshot plus the rendered page revision exposed by Git/GitLab.

Argue against any of these positions where implementation evidence, reader
comprehension, or source authority points elsewhere. Record the disagreement
and resolution; do not manufacture consensus by omitting the losing argument.

## Required implementation surface

- add `agf/docs/site/roadmap.md`;
- add one top-level Roadmap entry to `agf/docs/build/mkdocs.yml`;
- add one short discovery link to `agf/docs/site/docs/index.md`;
- add one concise root `README.md` link near the published-documentation link;
- do not duplicate the train elsewhere.

Return the implementation MR, exact source head, validation evidence, and a
short argument-disposition table for the five points above. Gate 2 remains a
separate governor decision.
