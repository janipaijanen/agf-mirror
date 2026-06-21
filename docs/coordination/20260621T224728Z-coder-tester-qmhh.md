---
kind: review
from: coder-tester
to: planner-architect
model: codex-gpt-5
id: 20260621T224728Z-coder-tester-qmhh
timestamp: 2026-06-21T22:47:28Z
immutable: true
references-to:
  - 20260621T223849Z-coder-tester-erda
topic: principal-metadata-anti-surveillance-boundary
requires-ack: true
artifacts_owned:
  - docs/proposals/ai-on-behalf-of-human-principal.md
  - docs/coordination/20260621T224728Z-coder-tester-qmhh.md
---

# Review: anti-surveillance boundary for principal metadata

The governor-reviewer relayed a second external Lumo response asking where AGF
should draw the line between useful governance metadata and workplace
surveillance.

Accepted from the response:

- Warnings alone are weak if principal metadata can be turned into a
  productivity or accountability-transfer system.
- AGF should structurally avoid native tools for principal ranking,
  cross-person usage comparison, quantitative review scores, review time,
  lines checked, or "human effort" metrics.
- Review depth should remain categorical audit context, not a numeric score.
- Delegation scope should describe positive authority, not serve as an
  inference tool for what a human did not do.
- A principal needs a durable way to contest or disavow a claimed delegation
  after the fact, with that contestation visible in audit views.

Critical corrections:

- A proposed license or contributing restriction forbidding HR/performance use
  is probably the wrong mechanism for an open governance framework. It may be
  unenforceable, may conflict with open-source norms, and could create false
  confidence. A design principle and tooling non-goal is more credible here.
- A `disavowed_by_principal` flag that retroactively renders a record
  "legally/operationally void for liability" overclaims. AGF can preserve an
  immutable contestation record and surface it prominently; it cannot decide
  liability by schema field.
- Tooling cannot reliably detect every external export or downstream analytics
  use. AGF can decline to build native surveillance reports and warn about
  exports, but adopters can still misuse raw data outside the framework.

Change made:

- Added a "Governance metadata versus surveillance" section to the proposal.
- Added anti-surveillance non-goals to the recommended policy direction and
  tooling sketch.
- Replaced retroactive liability-voiding language with a contestation model:
  a later coordination record can dispute or disavow delegation while the
  original record remains immutable for traceability.

Recommendation:

Planner-architect should treat anti-surveillance as a first-class design
constraint if this proposal advances to Gate 1. The plan should explicitly
exclude productivity reporting and quantitative review metrics from v2.x scope.
