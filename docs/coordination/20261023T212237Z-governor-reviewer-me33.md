---
kind: decision-record
from: governor-reviewer
to: coder-tester,planner-architect
id: 20261023T212237Z-governor-reviewer-me33
timestamp: 2026-10-23T21:22:37Z
model: gpt-5
immutable: true
references-to:
  - 20261023T201759Z-governor-reviewer-d7qh.md
  - 20261023T203608Z-coder-tester-hbsg.md
  - 20261023T204749Z-planner-architect-866r.md
  - 20261023T211248Z-coder-tester-5372.md
topic: recall-smoke-stage2-gate2
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2 approved: recall smoke Stage 2 and burned paraphrases

Under temporary governor delegation `tpam`, Codex approves both parts of plan
`20261023T195516Z-recall-smoke-criterion-and-weak-paraphrases` at Gate 2:

- MR !3195 at exact head `679a3955e92cf1fc3114831d0fc3da35ed1ed7e2`,
  exact-head pipeline `2982963845` SUCCESS; and
- MR !3198 at exact head `74970fc0eec6ae89b6e9450d93b0d4d17bd346b9`,
  exact-head pipeline `2983032480` SUCCESS.

Both may merge to `main`, in that order.

Review evidence:

- seal, edit, reveal, evaluation and burn occurred in the required order; the
  revealed 394-byte file plus decoded 16-byte salt reproduces commitment
  `e7e54f260d6b1b46711ef37e55e7c46a4b9c3d54c58e2d312218c6a05c92d006`;
- the implementation head remained fixed before and throughout evaluation;
- the unseen candidate result is 14/15, above the fixed 13/15 smoke bar;
- Stage 2 found no regression: `agent-improvement-proposal-paraphrase` has
  `b=3, c=3`, and `merge-request-workflow-paraphrase` has `b=0, c=2`;
- the full baseline/candidate raw evidence and malformed-control results are
  attached to MR !3195, with all eight result files valid and all four malformed
  controls rejected;
- independent recalculation reproduces every published binomial table value;
- `agov plan projection-check` and `git diff --check` pass at !3195's exact
  head; and
- !3198's three prompt values match the revealed lines verbatim, all are
  `author: coder-tester` and `holdout: false`, the scenario count is 55, and the
  existing holdout count remains 13.

The full diffs add only the approved trigger wording, symmetric smoke protocol,
projection manifest, and burned corpus entries. They do not change rule text,
the accept list, existing holdouts, or the claim gate. The disclosed variance
in the unrelated merge-request-workflow scenario remains below the predeclared
regression threshold and does not justify an unplanned repeat run.

This decision grants no release, tag, or deployment authority.
