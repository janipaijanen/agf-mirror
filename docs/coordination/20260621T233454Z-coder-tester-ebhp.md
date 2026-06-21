---
kind: handoff
from: coder-tester
to: planner-architect
model: codex-gpt-5
id: 20260621T233454Z-coder-tester-ebhp
timestamp: 2026-06-21T23:34:54Z
immutable: true
references-to:
  - 20260621T231311Z-coder-tester-43eb
  - 20260621T233240Z-coder-tester-71sg
topic: principal-delegation-proposal-planner-follow-up
requires-ack: true
artifacts_owned:
  - docs/coordination/20260621T233454Z-coder-tester-ebhp.md
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Review whether MR !95 should remain exploratory and merge as-is, or become the basis for a Gate 1 plan for v2.x documentation conventions.
    - If planning follow-up work, preserve the docs-first direction and the anti-surveillance/residual-risk boundaries from the proposal.
  must_read:
    - docs/proposals/ai-on-behalf-of-human-principal.md
    - docs/coordination/20260621T231311Z-coder-tester-43eb.md
    - docs/coordination/20260621T233240Z-coder-tester-71sg.md
  do_not_touch:
    - agents-govern/
---

# Handoff: principal-bound delegation proposal follow-up

MR !95 is ready for planner/governor follow-up.

The proposal now recommends a docs-first v2.x path for principal-bound AI
delegation:

- human-authored delegation grants as `kind: note`;
- AI records reference grants instead of self-attesting a principal;
- `identifier_namespace` plus `identifier` are explicitly authored by the
  grant source;
- no inference from git metadata;
- contestation as an immutable note convention;
- anti-surveillance boundaries documented;
- residual surveillance and correlation risks documented, including identifier
  frequency, timestamps, and git history;
- no schema change proposed yet.

Please review whether this should become a Gate 1 plan for v2.x documentation
conventions, or whether MR !95 should remain exploratory and merge as-is.

Relevant branch:

- `docs/ai-on-behalf-principal-analysis`

Relevant MR:

- `!95 docs: analyze AI acting on behalf of human principals`

Latest pushed tip at handoff time:

- `b7f76aa docs: document residual delegation surveillance risks`

Pipeline status at handoff time:

- GitLab pipeline `2618150853`: success
- MR !95 detailed merge status: mergeable
