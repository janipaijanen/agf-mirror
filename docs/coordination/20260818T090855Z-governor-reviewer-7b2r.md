---
artifacts_owned: []
from: governor-reviewer
id: 20260818T090855Z-governor-reviewer-7b2r
immutable: true
kind: decision-record
model: human-governor
references-to:
- 20260818T054511Z-governor-reviewer-1gaj.md
- 20260818T074544Z-contribution-auditor-hsrk.md
- 20260818T080749Z-contribution-auditor-5mjn.md
- 20260818T084146Z-planner-architect-8hms.md
- 20260818T085556Z-governor-reviewer-mk1p.md
requires-ack: false
timestamp: 2026-08-18T09:08:55Z
to: planner-architect
topic: cl4-r8-licensing-artifact-corpus-scope-decision
---

# Decision: explicitly exclude the licence instrument; require future per-file dispositions

This record transcribes the human governor's binding instruction in the active
session: “approved as proceed.” It approves the exact CL4-R8 disposition
presented after planner escalation `8hms` narrowed the current candidate delta
to `LICENSES/LicenseRef-AgentsGovern-Commercial.txt`.

## Options considered

1. Treat the licence text and every future artifact labeled
   “licensing-control artifact” as categorically outside CL-3, allowing the
   generator to exempt that class without another per-file decision.
2. Explicitly exclude the current commercial licence text because it is the
   licensing instrument rather than Framework Content, while requiring every
   future post-inventory file that matches the candidate rule to receive its
   own included/excluded disposition.

**Chosen: option 2.**

## Binding disposition

- `LICENSES/LicenseRef-AgentsGovern-Commercial.txt` is excluded from the
  commercial Framework Content corpus. It is the instrument defining the
  commercial licence, not content granted under that instrument.
- The initial included corpus remains the 142 Licensor-authored paths approved
  by `1gaj`.
- The current implementation-base boundary is therefore 142 included paths
  and 50 explicitly excluded paths: the 49 inventory-derived
  `generated/vendor` paths plus this one post-inventory licence-instrument
  disposition. The licence text's provenance must remain distinguishable from
  the 49 inventory rows rather than being presented as if it existed in that
  inventory.
- No categorical exemption is granted for unspecified future
  “licensing-control artifacts.” Any later tracked file that matches the
  candidate predicate and is absent from the current manifest must stop
  generation or surface drift until an authorized included/excluded
  disposition is recorded for that exact file.
- The agreement template does not currently match the predicate and receives
  no prospective exemption through this decision. The same rule applies to
  future manifests, pointer files, schemas, templates, or other control
  artifacts if their eventual bytes match the predicate.

## Strongest objection and response

The strongest objection is operational friction: case-by-case disposition is
slower than a reusable class exemption. The response is that a broad,
open-ended class would weaken the closed-list property CL-4 exists to enforce
and could silently hide future files from corpus review. One explicit decision
per new matching path preserves provenance, reviewability, and the governor's
authority over corpus changes.

## Required planner action

Planner-architect may resolve CL4-R8 by updating the plan and manifest design
to record the licence text as an explicit post-inventory exclusion citing this
decision. Generation and validation must account for all 192 candidates on the
MR !1712 implementation base without conflating the 191-row source inventory
with the later disposition. Future matching deltas must remain individually
gated as described above.

## Boundaries unchanged

This is a technical/governance corpus-scope decision, not legal advice.
Qualified-counsel review remains mandatory before any real buyer offer. This
decision does not itself approve the CL-4 plan at Gate 1, authorize
implementation or Gate 2, merge MR !1712 or MR !1723, offer a licence, or
execute an agreement. CL4-R11 remains governed separately by `mk1p`.
