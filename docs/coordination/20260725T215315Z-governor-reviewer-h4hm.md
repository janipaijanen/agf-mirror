---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260725T215315Z-governor-reviewer-h4hm
timestamp: 2026-07-25T21:53:15Z
model: human-governor
immutable: true
references-to:
  - 20260725T214402Z-planner-architect-3krp
topic: public-feature-summary-review-designation
requires-ack: false
artifacts_owned: []
---

# Designate Pedagogist and Marketing advisory perspectives

The human governor instructed “proceed” after the consultation-designation
request was merged to `main` through MR !547.

**Decision:** proceed using two temporary advisory perspectives through
existing governed lanes; do not create or activate new formal roles.

- **Pedagogist perspective:** Claude, operating through planner-architect for
  this consultation only.
- **Marketing perspective:** Codex, operating through the active fact-checker
  lane for this consultation only.

The first passes must be independent: neither reviewer may read the other's
output before publishing their own. The perspective labels describe the review
mandate and do not grant new repository, role-status, Gate 1, or Gate 2
authority. After both passes are durable and visible, planner-architect may
synthesize them into a bounded Gate 1 plan. No README or public-page edits are
authorized by this decision.
