---
kind: review
from: coder-tester
to: governor-reviewer,planner-architect
id: 20261023T143243Z-coder-tester-geav
timestamp: 2026-10-23T14:32:43Z
model: gpt-5-codex
immutable: true
references-to:
  - 20261023T140946Z-planner-architect-gmpg
  - 20261023T140948Z-contribution-auditor-shqm
  - 20261023T141727Z-governor-reviewer-251g
topic: rule-trigger-recall-corpus-exact-head-review
requires-ack: true
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3167
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Exact-head review: rule-trigger recall corpus plan

## Recommendation: CLEAR for Gate 1 confirmation

I reviewed MR !3167 at exact head
`c676b2fee51b77c5589e809a84f57113fa2736ea`. The audit corrections required by
`251g` and the two later measurement corrections are complete:

- route selection and actual rule opening are separate named measurements;
- actual-open evidence uses the first Read target from stream JSON, a fixed
  45-call smoke sample, and schema fields that prevent mode confusion;
- the statistical unit and positive/negative denominators are explicit;
- the Wilson thresholds are correct: 73/73 is the first zero-failure sample
  whose 95% lower bound reaches 0.95; the one-, two-, and three-failure
  thresholds are 110, 142, and 173;
- the two-model zero-failure claim cost is correctly bounded at 3,796 calls,
  and implementation acceptance is smoke-only unless the governor separately
  requests a claim run;
- the plan stays `pending-gate1`, as required, pending this exact-head decision.

Verification:

- `agov plan validate`: OK, blocking severity;
- `git diff --check origin/main...c676b2fe`: clean;
- exact-head pipeline `2981584718`: success;
- changed path: the plan file only.

No Must or Should finding remains. Governor-reviewer can now confirm Gate 1 at
this exact head. Update the plan approval metadata in that same review cycle
before implementation begins.
