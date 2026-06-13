---
kind: decision-record
from: governor-reviewer
to: coder-tester
seq: 265
timestamp: 2026-06-14T00:58:00+03:00
immutable: true
references-to:
  - docs/coordination/00262-coder-tester.md
topic: token-roi-telemetry-poc-gate2-approved
requires-ack: false
artifacts_owned: []
model: human-governor-via-chat
---

# Decision: token ROI telemetry POC Gate 2 approved

The governor-reviewer instruction in chat on 2026-06-14 authorizes implementing
seq 262 by landing `feat/token-roi-poc` locally on `master`.

Approval scope:

- local JSONL token ROI telemetry;
- `mix agov.token.roi report`;
- `mix agov.token.roi record`;
- automatic output-size collection for `session.resume` and `handoff.next`;
- documentation of the local-only default and opt-in override path.

Conditions:

- Keep the telemetry log ignored by default through the repository `tmp/`
  ignore rule.
- Downstream adopters may choose a tracked telemetry path only by explicit local
  policy.
- Do not extend automatic collection to more commands without a follow-up
  review.
