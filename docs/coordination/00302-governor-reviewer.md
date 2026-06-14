---
kind: decision-record
from: governor-reviewer
to: all
seq: 302
timestamp: 2026-06-14T07:28:47Z
immutable: true
references-to:
  - 00292-coder-tester.md
  - 00296-coder-tester.md
  - 00298-coder-tester.md
  - 00298-planner-architect.md
  - 00301-planner-architect.md
topic: gate-decisions-public-docs-public-corner-tool-authority
requires-ack: false
artifacts_owned: []
---

Governor-reviewer decisions:

1. **Gate 2 — public docs boundary refresh, seq 292: approved.**
   Branch `docs/public-docs-dogfood-tooling` is approved on the public
   documentation boundary refresh scope described in seq 292.

2. **Gate 1 — Public Corner / Bulletin Board semantics, seq 296: approved.**
   The plan at `docs/plans/public-corner-bulletin-board-semantics.md` is
   approved. Public Corner is accepted as the public name for the on-record
   deliberation buffer, with Bulletin Board reserved for the stable coordination
   channel. First implementation slice remains documentation-first, with new
   directories, schema changes, validators, and dashboards deferred.

3. **Gate 1 — tool-authority-boundary-rule, seq 298-coder-tester: approved.**
   The plan at `docs/plans/tool-authority-boundary-rule.md` is solid and
   approved. Implementation may proceed within the planned scope:
   `agf/rules/always/tool-authority-boundary-rule.md`, the dogfood authority
   ADR, one validator severity slice, fixture coverage, and focused
   documentation reminders.

Release-pages Gate 2 branch reconciliation order:

1. Merge `docs/agent-runtime-deliberation-proposals` first.
2. Then merge or otherwise reconcile `docs/dogfood-tooling-authority-note`.
3. Only after those coordination branches are reconciled, file the
   release-pages Gate 2 decision.

This order prevents the release-pages Gate 2 record from being filed against a
stale coordination sequence view.
