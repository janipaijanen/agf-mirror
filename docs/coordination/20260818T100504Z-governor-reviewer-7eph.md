---
kind: decision-record
from: governor-reviewer
to: all
id: 20260818T100504Z-governor-reviewer-7eph
timestamp: 2026-08-18T10:05:04Z
model: human-governor
immutable: true
references-to:
- 20260818T100116Z-planner-architect-3rat.md
- 20260818T093747Z-contribution-auditor-pk4k.md
- 20260818T093749Z-coder-tester-6kmj.md
- 20260818T054511Z-governor-reviewer-1gaj.md
- 20260818T085556Z-governor-reviewer-mk1p.md
- 20260818T090855Z-governor-reviewer-7b2r.md
topic: cl4-gate1-approved
requires-ack: false
artifacts_owned: []
---

# CL-4 immutable corpus binding Gate 1 approved

This record transcribes the human governor's binding instruction in the
active session: “Gate 1 approved. please proceed.” It approves the CL-4 Gate
1 plan at MR !1750 exact head
`91c98801fc19bca08204080b39de7e206bfa04c8`, following final planner handoff
`3rat`, contribution-auditor CLEAR `pk4k`, and coder-tester feasibility CLEAR
`6kmj`.

## Binding Gate 1 outcome

- **Plan approved:**
  `docs/plans/20260818T055436Z-cl4-immutable-corpus-binding-gate1-plan.md`
  at the exact head above.
- **Corpus boundary:** 142 included paths and 50 excluded paths: the 49
  inventory-derived exclusions governed by `1gaj` plus the individually
  governed post-inventory licence-text exclusion under `7b2r`.
- **No exemption class:** no `control_artifacts` class or other categorical
  shortcut is authorized. Every future matching path absent from the
  manifest requires its own included/excluded disposition.
- **Security-design applicability:** `mk1p` remains binding. The CI-only
  infosec-sentinel is not applicable to this design review; the plan's
  integrity properties and negative tests remain mandatory on the
  contribution-auditor and coder-tester surfaces.
- **Counsel condition:** qualified-counsel review remains mandatory before
  any real buyer offer or agreement execution.

## Implementation authority

Coder-tester is authorized to implement CL-4 on MR !1712's existing branch,
using MR !1723's reviewed inventory at the exact SHA named in the approved
plan. Implementation must preserve the approved staged, acyclic construction;
the enumerated included/excluded manifest; exact candidate predicate; blocking
schema-shape behavior; advisory current-tree and path-mutation behavior;
historical recovery behavior; Git-blob identity citations; documented
projection screen; and all positive and deliberate-negative fixtures.

The plan's front matter may now be synchronized to `status: approved` with
this decision ID and timestamp. That metadata update records this decision; it
does not create a second approval.

## Boundaries unchanged

This decision does not grant Gate 2, approve or merge MR !1712 or MR !1723,
authorize a licence offer or agreement execution, clear any of the 49
inventory-derived exclusions, waive counsel, approve a future corpus version,
or authorize a release, tag, deployment, branch deletion, or MR closure.
