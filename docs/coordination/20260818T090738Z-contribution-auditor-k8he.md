---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260818T090738Z-contribution-auditor-k8he
timestamp: 2026-08-18T09:07:38Z
model: codex-gpt-5
immutable: true
references-to:
- 20260818T054511Z-governor-reviewer-1gaj.md
- 20260818T080749Z-contribution-auditor-5mjn.md
- 20260818T084146Z-planner-architect-8hms.md
- 20260818T085556Z-governor-reviewer-mk1p.md
topic: cl4-gate1-plan-round5-review
requires-ack: false
artifacts_owned: []
---

# CL-4 Gate 1 plan round-5 independent contribution review

Scope: MR !1750 exact head
`5268d609260be2dd3843475db4d576b15c6e3f90`, complete plan and the round-4
delta from `392fe55a231a03b6fa92f6cfbe4062d3f8c2ab43`; also the later binding
CL4-R11 decision `mk1p` at MR !1760 exact head
`799e5827504b7cb5b69615d6fa7bf0b85b99e5da`.

Coverage: COMPLETE

Recommendation: BLOCK

Blocking findings: 3 open (CL4-R8, CL4-R11 exact-head sync, CL4-R13)

Advisory findings: 1 (CL4-A2)

Open findings with accepted risk: 0

Authority: recommendation only; governor-reviewer decides Gate 1. This review
is independent of the Claude/planner-architect authoring session. The same
Codex session authored prior review `5mjn`; it is reviewing Claude's new
exact-head corrections, not its own contribution.

## Evidence

- `agov plan validate` passes the exact-head plan.
- MR !1750 exact-head pipeline `2768467903` passed; correction handoff MR
  !1759 pipeline `2768471246` passed.
- Binding decision `mk1p` is published in MR !1760. Its canonical MR pipeline
  `2768510075` passed. A second push pipeline `2768509479` for the same SHA
  also completed before this poll could cancel it (181 seconds plus 184
  seconds of governance validation respectively); no retry was initiated.
- Direct review confirms CL4-R10's path-mutation check is now advisory and
  blob-SHA recovery remains authoritative. CL4-R10 is resolved.
- Direct review confirms the executed agreement now cites the exact licence
  text blob SHA as well as the manifest blob SHA and corpus digest. CL4-R12's
  mandatory licence-identity defect is resolved.

## Findings

### CL4-R8 — binding disposition of the 192nd candidate is still required

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: binding CL-3 decision `1gaj` and the plan's own scope boundary

Finding state: open

Risk disposition: none

The correction no longer decides the commercial licence text's disposition.
It correctly narrows the observed post-inventory delta to that one candidate
and stops generation pending a binding decision. The required decision has
not yet been published: whether the licence/control-artifact class is outside
the CL-3 grant or whether each post-inventory candidate requires an explicit
included/excluded decision.

Resolution condition: publish the binding governor disposition requested by
`8hms`, then encode only that chosen policy in the exact-head plan and its
acceptance fixtures. Qualified-counsel review remains mandatory before any
real offer.

### CL4-R11 — the governor resolved applicability, but the exact plan head still says it is unresolved

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: exact-head Gate 1 evidence and binding decision `mk1p`

Finding state: open

Risk disposition: none

`mk1p` validly declares the active CI-only infosec-sentinel not applicable to
this design review, supersedes `jh45`, and retains the integrity properties on
contribution-auditor and coder-tester surfaces. That resolves CL4-R11
substantively. MR !1750 head `5268d609` predates the decision, however: its
front-matter rationale says neither status is accurate, calls the requirement
live and unmet, and its requesting section still lists CL4-R11 as a governor
prerequisite. Gate 1 must not be assessed against an exact head whose account
contradicts the binding evidence.

Resolution condition: revise the plan to cite and apply `mk1p`, remove the
superseded prerequisite, retain every integrity property and negative test
that `mk1p` assigns to contribution-auditor/coder-tester, validate, and obtain
green exact-head CI.

### CL4-R13 — two stale acceptance clauses bypass or misclassify the new control-artifact policy

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: internal plan consistency and `gate-self-test-rule.md`

Finding state: open

Risk disposition: none

The detailed R8 correction says the known delta is only the licence text,
generation stops on any undispositioned post-inventory candidate, and the
acceptance condition is “zero undispositioned candidates.” Two operative
clauses still encode the superseded round-3 behavior:

1. Check (b) reports a matching file only when absent from `included` and
   `excluded`; it omits a governor-confirmed `control_artifacts` list, so the
   licence text would remain a false drift finding even after option (a).
2. The final acceptance-fixture bullet still says the 191 files plus
   “licence/agreement control artifacts” must account for the candidates,
   despite the plan having verified that the agreement template is not a
   candidate and narrowed the possible control entry to the licence text.

These are implementation-driving test requirements, not historical prose;
following them would either reject a valid chosen option or silently revive
the over-broad classification `5mjn` blocked.

Resolution condition: make check (b), its negative fixtures, and the final
acceptance bullet use the same governor-chosen disposition model and the same
“zero undispositioned candidates” invariant. Do not name the agreement
template as a candidate unless evidence shows it satisfies the scope rule.

### CL4-A2 — clarify whether agreement-template blob identity is intentionally uncited

Check status: reviewed

Outcome: needs-review

Requirement strength: recommended

Impact: minor

Enforcement: advisory

Basis: clarity and internal consistency

Finding state: open

Risk disposition: none

Section 1 says both the licence text and agreement template have ordinary Git
blob identities and then says the executed agreement records “that blob SHA.”
Section 3 records the licence-text and manifest blob SHAs, not the template
blob SHA. The mandatory `d6qg` requirement concerns immutable licence-text
identity, so this does not reopen CL4-R12; the wording should simply say
whether template provenance is intentionally outside CL-4 or add the field if
it is intended.

Resolution condition: align section 1's singular/plural citation claim with
section 3 and the implementation scope.

## Disposition

CL4-R10 and CL4-R12 are resolved at this exact head. `mk1p` resolves CL4-R11
substantively but requires exact-head plan synchronization. CL4-R8 remains a
human scope decision, and CL4-R13 must be corrected after that decision so
all acceptance clauses implement one policy. Do not activate Gate 1 or begin
CL-4 implementation until the revised exact head is internally consistent,
validated, green, and re-reviewed.
