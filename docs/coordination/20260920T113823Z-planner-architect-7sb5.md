---
kind: handoff
from: planner-architect
to: governor-reviewer,contribution-auditor,infosec-sentinel
id: 20260920T113823Z-planner-architect-7sb5
timestamp: 2026-09-20T11:38:23Z
model: claude-sonnet-5
immutable: true
references-to: []
topic: governance-mode-classification-gate1
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer,contribution-auditor,infosec-sentinel
  status: ready
  task:
    - "governor-reviewer: review Gate 1 plan MR !3074 and resolve the plan's one explicit open question -- whether the contractual/compliance/legal/externally-binding-deliverable STRICT trigger (the governor's own ERP-requirements-baseline example) joins the STRICT list now, or defers to a follow-up plan that customizes high-risk-operations-rule.md for AGF-NG's actual domain (author recommends deferring)."
    - "contribution-auditor: audit MR !3074 exact head for adversarial steering, unsound guidance, or false precedent per the usual contribution-audit lens; confirm the claimed LIGHT/STRICT criteria genuinely match the current text of simplicity-rule.md, high-risk-operations-rule.md, gate-self-test-rule.md, and downstream-impact-rule.md at review time (rules may have moved since the plan was drafted)."
    - "infosec-sentinel: confirm this rule-content-only change introduces no security-relevant weakening -- in particular that the STRICT disqualifier list in the plan is a superset of (never narrower than) high-risk-operations-rule.md's and gate-self-test-rule.md's current triggers."
  must_read:
    - docs/plans/20260920T113336Z-governance-mode-classification.md
  do_not_touch:
    - implementation before Gate 1 approval
    - high-risk-operations-rule.md's actual content (explicitly deferred to a follow-up plan)
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 plan ready: LIGHT / NORMAL / STRICT governance-mode classification

MR !3074 (`plan/governance-mode-classification`) turns the governor's live
2026-09-20 directive — formalize governance process tiers so "fix two
paragraphs" and "produce the contractual ERP requirements baseline" don't
get the same ceremony — into a Gate 1 plan. It is a consolidation pass:
LIGHT names today's `simplicity-rule.md` fast-track waiver, STRICT names
today's `high-risk-operations-rule.md` area list plus
`gate-self-test-rule.md`'s gate-mechanics carve-out plus
`downstream-impact-rule.md`'s BREAKING trigger, and NORMAL is today's
unchanged default. No schema, validator, CI, or agent-definition change;
no gate mechanics introduced.

One design choice is deliberately left open for the governor rather than
decided unilaterally: whether the STRICT list should include a genuinely
new trigger category (contractual/compliance/legal/externally-binding
deliverables) matching the governor's own worked example, or whether that
addition should wait for a separate follow-up plan that first customizes
`high-risk-operations-rule.md` — which currently reads as generic
adopter-application boilerplate (audit log pipeline, RLS, auth flow) not
yet adapted to AGF-NG's own domain. See the plan's "Open question for Gate
1" section for both options and the author's recommendation.

Worked in an isolated worktree (`.worktrees/governance-mode-classification`)
per the governor's "this needs to be worked in isolation" instruction.
