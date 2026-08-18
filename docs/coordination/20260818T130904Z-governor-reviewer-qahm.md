---
artifacts_owned: []
from: governor-reviewer
id: 20260818T130904Z-governor-reviewer-qahm
immutable: true
kind: decision-record
model: human-governor
references-to:
- 20260818T124726Z-planner-architect-grsq.md
- 20260818T103816Z-contribution-auditor-gpq5.md
- 20260818T100504Z-governor-reviewer-7eph.md
requires-ack: false
timestamp: 2026-08-18T13:09:04Z
to: coder-tester
topic: cl4-imp-r1-topology-ratified
---

# CL4-IMP-R1 topology ratified: retain 48049759

This record transcribes the human governor's binding instruction in the
active session, responding to planner-architect's recommendation `grsq`
(MR !1771, merged to `main`): "Merge. Proposed topology is ok, proceed."

## Binding ratification

- **CL4-IMP-R1 is resolved**: `corpus_content_revision` in
  `LICENSES/commercial-corpus-manifest-v1.yaml` MUST be
  `48049759e8f6aa85b9878262ce6b2ab54a27e995` — MR !1712's exact commit
  immediately before CL-4's own implementation commits — not
  `b3a04178f149e287f82988ca90061a9153c13f3b`, the value currently
  recorded.
- The resulting `.gitlab-ci.yml` divergence between `48049759` and CL-4's
  own later commits is expected, ordinary advisory drift under the plan's
  section 4(b) current-tree check. It does not require special handling,
  a waiver, or a plan change.
- This does not reopen or reinterpret governor decision `7b2r`: the licence
  text's exclusion and the "no categorical exemption class" rule are
  unaffected.

## Implementation authority

Coder-tester is authorized to:

1. Regenerate `LICENSES/commercial-corpus-manifest-v1.yaml` and the bound
   licence digest against `corpus_content_revision: 48049759`.
2. Separately resolve CL4-IMP-R2 (the three missing acceptance/negative
   fixtures `gpq5` identified: missing-included-path advisory drift fixture;
   v1-to-v2 historical-vs-current-tree fixture; and a generation/acceptance
   fixture against the real MR !1712 base asserting the 192 = 142 + 50
   invariant).
3. Obtain a green exact-head pipeline, then request the next independent
   contribution-auditor review round.

## Boundaries unchanged

This decision resolves CL4-IMP-R1 only. It does not grant Gate 2, approve
or merge MR !1712, waive CL4-IMP-R2, authorize a licence offer or agreement
execution, clear any of the 49 inventory-derived exclusions, or waive
counsel review.
