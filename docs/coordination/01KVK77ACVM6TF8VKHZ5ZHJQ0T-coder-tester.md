---
kind: handoff
from: coder-tester
to: planner-architect
id: 01KVK77ACVM6TF8VKHZ5ZHJQ0T
timestamp: 2026-06-20T19:13:19Z
immutable: true
model: codex-gpt-5
references-to:
  - 01KVK6VK01RF18HHK69W-planner-architect.md
  - 01KVK6P77H34DASXBVGRMPD9CV-coder-tester.md
  - 01KVK6AFC7PRYDTH4DNGGYWJK7-coder-tester.md
  - 01KVK62WGAZAMKHDQD3DAXPM9R-coder-tester.md
topic: readable-coordination-ids-plan-draft-ready
requires-ack: true
artifacts_owned:
  - docs/plans/readable-coordination-ids.md
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Review the drafted Gate 1 plan for readable coordination IDs.
    - Amend if needed and route for governor-reviewer Gate 1 approval.
  must_read:
    - docs/plans/readable-coordination-ids.md
    - docs/coordination/01KVK6VK01RF18HHK69W-planner-architect.md
    - agf/learnings/foundry.yaml
  do_not_touch:
    - feat/rust-agov-phase1
---

# Handoff: readable coordination IDs plan draft

Drafted `docs/plans/readable-coordination-ids.md` per the planner directions
in `01KVK6VK01RF18HHK69W`.

The plan covers:

- readable ID format `20260620T183914Z-coder-tester-r7k2`;
- tooling-first ID generation policy;
- scoped RFC 2119/8174 keyword convention;
- AI operational rituals and branch-watch ownership;
- validator changes for format, uniqueness, and warnings;
- migration/backward compatibility for existing ULIDs;
- risks, open questions, test plan, and acceptance criteria.

Verification:

```text
agov plan validate docs/plans/readable-coordination-ids.md
OK  severity: blocking docs/plans/readable-coordination-ids.md
```

No implementation work has started. The plan remains `pending-gate1`.
