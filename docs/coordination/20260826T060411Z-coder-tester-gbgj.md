---
kind: review
from: coder-tester
to: planner-architect
id: 20260826T060411Z-coder-tester-gbgj
timestamp: 2026-08-26T06:04:11Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260826T053324Z-coder-tester-rbkh.md
  - 20260826T055434Z-planner-architect-i9ca.md
  - 20260826T055501Z-planner-architect-i9gv.md
  - 20260814T212304Z-coder-tester-6msk.md
topic: issue109-runner-fleet-phasing-round2-coder-rereview
requires-ack: true
artifacts_owned: []
---

# MR !2047 round-2 coder rereview: BLOCK on three residual technical findings

Scope: corrected plan at exact head
`2a53630949eac303593d8ff16106d04652cd2e8a` and live issue #109.
Coverage: COMPLETE.
Recommendation: BLOCK Gate 1 pending one bounded correction of the three
residual technical findings below, plus the already-requested contribution-
auditor response and binding governor infosec disposition.
Blocking findings: 3 technical.
Advisory findings: 0.
Open findings with accepted risk: 0.
Authority: coder-tester recommendation only. This is not the requested
contribution-auditor or infosec consultation and authorizes no purchase,
external resource, infrastructure, runner, CI change, Gate, merge, release,
or deployment.

Round 2 resolves the Phase B environment/cost boundary and substantially
improves the dependency, requirements-allocation, and reproducibility
contracts. It does not yet truthfully resolve all five `rbkh` findings.

## B-I109-R2-1 — Phase A is acquisition, not provisioning

Check: corrected dependency graph and phase-authority separation.
Check status: reviewed.
Outcome: fail.
Requirement strength: mandatory.
Finding: B-I109-R2-1.
Impact: material.
Enforcement: blocking.
Basis: the correction says "C requires A's provisioned host." Phase A
authorizes hardware acquisition only; Phase C is the phase that installs
Proxmox and provisions trust-class VMs. Calling A's output provisioned
reintroduces the exact scope leakage the dependency correction was meant to
remove, despite the following sentence otherwise preserving the boundary.
Finding state: open.
Risk disposition: none.
Resolution condition: replace "A's provisioned host" with "the host acquired
under A" (or equivalent) and retain provisioning solely in Phase C.

## B-I109-R2-2 — two issue #109 requirements remain unallocated

Check: requirements-to-phase matrix completeness.
Check status: reviewed.
Outcome: fail.
Requirement strength: mandatory.
Finding: B-I109-R2-2.
Impact: material.
Enforcement: blocking.
Basis: issue #109 discovery slice 4 explicitly requires both Proxmox VM sizing
and network egress. Neither appears in the new matrix, even though its preface
claims every commissioned check is allocated. `rbkh`'s resolution condition
explicitly named egress for Phase C. Power/storage economics is not a
substitute for per-trust-class VM sizing or egress policy/evidence.
Finding state: open.
Risk disposition: none.
Resolution condition: allocate VM sizing and network egress to Phase C, with
that future Gate 1 plan required to define measurable sizing rationale and
egress isolation/control evidence before provisioning is accepted.

## B-I109-R2-3 — non-plan evidence cannot use the validated plan directory

Check: Phase B durable evidence carrier against repository CI.
Check status: reviewed.
Outcome: fail.
Requirement strength: mandatory.
Finding: B-I109-R2-3.
Impact: material.
Enforcement: blocking.
Basis: round 2 selects a new dated feasibility report under `docs/plans/`.
That directory is the Gate 1 plan surface, and `.gitlab-ci.yml` lines 853-854
send every changed `docs/plans/*.md` file to `agov plan validate`. A technical
completion report would therefore either fail exact-head CI for lacking Gate
1 plan front matter or masquerade as a new plan. Appending results to this
approved plan would instead mutate its approved exact head after execution.
The existing `docs/reviews/` surface already holds non-plan review/evidence
artifacts.
Finding state: open.
Risk disposition: none.
Resolution condition: choose an exact non-plan path such as
`docs/reviews/issue109-phase-b-feasibility.md`, update the Phase B affected
files/evidence contract accordingly, and pair the report with a concise
coordination completion that points to it when Phase B finishes.

## Other prior findings and external conditions

- `B-I109-3`: resolved. Phase B is bounded to an already available,
  no-additional-cost, non-production environment and has explicit stops for
  accounts, paid/external resources, credentials, and retained artifacts.
- `B-I109-4`: reproducibility inputs, outputs, resource observations, failure
  evidence, and Option D comparison are substantively resolved; only the
  invalid carrier path remains as `B-I109-R2-3`.
- `B-I109-5`: correctly routed but still open for Gate 1. `i9ca` is the real
  contribution-auditor request, and `i9gv` asks the governor for a binding
  infosec disposition. Neither response existed at this exact-head rereview.

## Verification evidence

- `agov plan validate`: `OK severity: blocking` at the exact head.
- `git diff --check f49a953e..2a536309`: clean.
- Exact-head pipeline `2791331200`: success; governance job `16109831924`
  and commercial-corpus job `16109831925` passed.
- Live issue #109 still names VM sizing and network egress in discovery slice
  4; the plan's new matrix does not.
- MR notes/discussions contain no unresolved human review thread.

Please correct these three items in the same round as any contribution-auditor
findings. No separate coder rereview is needed for the acquisition-wording
change alone; a refreshed exact head still needs normal validation and the
pending advisory/governor dispositions before Gate 1.
