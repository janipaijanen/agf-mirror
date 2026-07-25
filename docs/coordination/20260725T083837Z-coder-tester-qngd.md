---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260725T083837Z-coder-tester-qngd
timestamp: 2026-07-25T08:38:37Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260725T072657Z-planner-architect-rs4m
  - 20260725T083801Z-fact-checker-tr6m
topic: split-person-gates-faq-gate2
requires-ack: true
artifacts_owned:
  - agf/docs/site/docs/start/adopter-faq.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Review MR !515 implementation at exact SHA 1cff194334b855da618d1721b7156863a94c6647 and decide Gate 2"
  must_read:
    - docs/plans/split-person-gates-adopter-faq.md
    - docs/coordination/20260725T083801Z-fact-checker-tr6m.md
    - "GitLab MR !515"
  do_not_touch:
    - release-v2.4.2
---

# Split-person Gate FAQ ready for Gate 2

MR !515 implements approved plan MR !506 / issue #66 as one FAQ entry. It
answers yes in the first sentence, then bounds the pattern to explicit,
authorized, sequential ownership. The checklist preserves exact-plan
traceability, returns scope changes to Gate 1, names one disagreement owner,
and prevents an approver from approving implementation they personally
performed as coder-tester.

Fact-checker disposition is ADOPT with no ADAPT or REJECT findings at exact
implementation SHA `1cff194334b855da618d1721b7156863a94c6647`. Local evidence:

- `agov validate adopter-docs`: pass;
- umbrella `agov validate`: pass, with only pre-existing warnings;
- MkDocs 1.6.1 strict build: pass;
- relative link resolution: pass;
- rendered 390 px and 1440 px viewport inspection: pass;
- `git diff --check`: pass.

The fact-check and this handoff are coordination-only commits after the exact
implementation SHA. Governor-reviewer: review that exact implementation SHA
and record APPROVE or BLOCK for Gate 2 before merge.
