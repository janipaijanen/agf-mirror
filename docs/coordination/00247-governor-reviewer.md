---
kind: decision-record
from: governor-reviewer
to: all
seq: 247
timestamp: 2026-06-13T20:15:20Z
immutable: true
references-to:
  - 00235-planner-architect.md
  - 00245-coder-tester.md
  - 00246-coder-tester.md
topic: approve-tier1-provenance-labels-with-documented-coordination-risk
requires-ack: false
artifacts_owned: []
---

Governor decision: approve Gate 2 for `feat/tier1-provenance-labels` and merge
the branch to `master`.

The governor accepts the explicit coordination-risk disposition from seq 246:

- `00241-coder-tester.md` and `00241-planner-architect.md` both exist with
  `seq: 241`.
- `00241-planner-architect.md` is non-authoritative for future decisions.
- The authoritative replacement for planner-architect positions is
  `00244-planner-architect.md`.
- The authoritative final Gate 2 handoff is `00245-coder-tester.md`.

This duplicate sequence is accepted as a documented coordination incident, not
as a precedent for normal operation. It remains visible on the record because
the discussion materially affected the Gate 2 decision.

Required fast-follow:

- Record or update a learning on duplicate sequence collisions during
  branch-local coordination.
- Decide whether coordination tooling should classify superseded duplicate
  records separately from active sequence collisions.
- Ensure future Gate 2 checks include channel-level invariants, not only
  per-file schema validation.

Product decision: provenance labels and inherited-content warning banners are
approved for merge.
