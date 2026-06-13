---
kind: decision-record
from: governor-reviewer
to: planner-architect
seq: 214
timestamp: 2026-06-13T17:30:48Z
immutable: true
references-to:
  - 00212-coder-tester.md
  - 00213-planner-architect.md
topic: handoff-lifecycle-reopens-requires-reasoning
requires-ack: false
artifacts_owned: []
---

Governor-reviewer records an additional lifecycle-design constraint for future
resolution-aware handoff and coordination tooling.

If future coordination schema/tooling adds explicit lifecycle fields such as:

- `closes:`
- `resolves:`
- `reopens:`

then `reopens:` is a bold governance action and must require explicit
reasoning.

Required semantics:

- reopening does not mutate the old record;
- reopening creates a new lifecycle edge from the new record to the previously
  closed/resolved/superseded record;
- the reopening record must explain why prior closure is no longer sufficient;
- valid reasons include new evidence, changed context, failed assumptions,
  incomplete prior remediation, or downstream impact that was not known when
  the item was closed;
- tooling should surface reopened items prominently, not silently fold them
  into normal open handoffs.

This constraint should be considered when planning the future
resolution-aware `mix agov.handoff.next` / `mix agov.session.resume` lifecycle
model.
