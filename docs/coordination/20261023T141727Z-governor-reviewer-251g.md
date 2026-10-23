---
kind: decision-record
from: governor-reviewer
to: planner-architect,coder-tester,contribution-auditor
id: 20261023T141727Z-governor-reviewer-251g
timestamp: 2026-10-23T14:17:27Z
model: human-governor
immutable: true
references-to:
  - 20261023T140946Z-planner-architect-gmpg
  - 20261023T140948Z-contribution-auditor-shqm
topic: recall-corpus-conditional-gate1-and-split-closure
requires-ack: false
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3163
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3167
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Recall-corpus audit required; split plan closure authorized

The Human Governor answered the three live questions with: "gate 1 approved,
audit. close." This record ratifies bridge
`20261023T140946Z-planner-architect-gmpg` with the constraints below and accepts
the requested audit as mandatory rather than waived.

## Recall-corpus plan: approved direction, corrections still required

The Governor approved the design direction of plan
`20261023T140050Z-rule-trigger-recall-corpus` as reviewed at MR !3167 head
`bb6e66ce6f4cb24e9e49fec0a8380d5303ef3918` and instructed the planner to run
the advisory audit.

That required audit, record
`20261023T140948Z-contribution-auditor-shqm`, found one WARN that expressly
requires a plan edit before approval, plus two INFO corrections. Under the
pending-advisory consultation rule, the current head cannot yet be represented
as `approved` and supplies no implementation authority.

The planner must make these bounded corrections:

1. estimate the claim-gate cost and state that the claim gate runs only when
   the Governor requests it;
2. use the single classification `STRICT`; and
3. require `corpus.yaml` to record Codex as the hold-out author.

After the corrected exact head passes plan validation and its required advisory
rows are no longer pending, return it for a short exact-head Governor
confirmation. Only that confirmation may set the plan to `approved` and release
implementation. Gate 2 and merge remain with the Human Governor.

This treatment preserves both parts of the live answer: the Governor accepted
the plan direction and explicitly required, rather than waived, the audit. It
does not silently discard the audit's required correction.

## Split plan MR !3163: closure authorized

Close MR !3163 at exact head
`a7ea040fc971cbd13823dc7f4ffa991e7248ed2c`. Preserve its branch. The plan
remains pending Gate 1 and may be revived only through a new request after the
documented scope, collision, baseline, and adopter-migration findings are
resolved.

The closure may execute only after this record is visible on `main`. No release,
tag, deploy, implementation, or additional plan approval is authorized. The
expired `qpda` delegation is not used.
