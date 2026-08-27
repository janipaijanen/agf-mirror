---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260823T210426Z-contribution-auditor-kp8e
timestamp: 2026-08-23T21:04:26Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260823T205859Z-planner-architect-mkke.md
topic: research-maintenance-fast-path-plan-exact-head-review
requires-ack: true
artifacts_owned: []
---

# Research-maintenance fast path: COMPLETE / BLOCK

Scope: MR !1865 at exact head
`1c71bf1486e7c324608f2224e968a03d4cf32508`, including
`docs/plans/20260823T205706Z-research-maintenance-fast-path.md` and the
current governing workflow rules and framework explanation.

Coverage: COMPLETE

Recommendation: **BLOCK before Gate 1**

Blocking findings: 3 open

Advisory findings: 1

Open findings with accepted risk: 0

Authority: advisory contribution-auditor recommendation only. The
governor-reviewer decides Gate 1.

Independence: this Codex session did not author the plan or its implementation
design.

Independent checks: `agov plan validate` and both accompanying coordination
records passed at the exact head. The plan is structurally valid; the findings
below concern the proposed authority and projection contract.

## `CA-RMFP-1` — the Gate 1 waiver authority and timing are undefined

Check status: reviewed

Outcome: fail

Impact: critical

Enforcement: blocking

Basis: Gate discipline and `agf/framework.md` Pattern 6 require an explicit
waiver decision, including who waived Gate 1

Finding state: open

Risk disposition: none

The plan lets an implementer self-classify a change, begin work, and record the
eligibility assertion only in the commit message. The governor first acts at
Gate 2. This does not identify who has authority to waive Gate 1 or when that
binding decision occurs, while the shipped framework requires the fast-track
record to say who waived Gate 1. Exact paths and semantic disqualifiers bound
the candidate class but do not themselves delegate governor authority.

Resolution condition: choose and document one coherent authority model before
Gate 1. Either retain a pre-implementation governor waiver with a lightweight
record, or explicitly propose and justify a named rule-based delegation that
permits agent self-qualification. State the durable evidence, timing, and
escalation behavior, and align all authority surfaces. Do not leave Gate 2 to
retroactively authorize work that already skipped Gate 1.

## `CA-RMFP-2` — the planned projection leaves the distributed framework contradictory

Check status: reviewed

Outcome: fail

Impact: material

Enforcement: blocking

Basis: projection completeness and adopter documentation parity

Finding state: open

Risk disposition: none

The implementation table updates rules, loaders, operating docs, and
changelogs, but explicitly omits `agf/framework.md`. That distributed document
currently defines the fast-track waiver record, including “who waived Gate 1.”
The proposed workflow changes that contract, so leaving the framework text
untouched would ship two incompatible instructions. A projection check over
the listed six surfaces cannot resolve an omitted authoritative explanation.

Resolution condition: add `agf/framework.md` to the implementation and
projection scope, or give a precise evidence-backed disposition showing why
its Pattern 6 contract remains correct under the chosen authority model. Screen
other adopter entry points for the same old contract and record their explicit
updated/unchanged disposition.

## `CA-RMFP-3` — `waived` falsely represents an outstanding required review

Check status: reviewed

Outcome: fail

Impact: material

Enforcement: blocking

Basis: Gate 1 advisory-consultation metadata must truthfully describe the
consultation decision

Finding state: open

Risk disposition: none

The front matter marks contribution-auditor status as `waived`, then says the
review is outstanding, required before approval, and requested by the handoff.
No governor waiver is cited. This turns `waived` into a placeholder for
“pending,” even though the consultation has now occurred and produced this
review. A structurally valid but semantically false status weakens the Gate 1
evidence envelope.

Resolution condition: replace the draft-stage waiver with consultation
metadata that truthfully records this exact-head review and its unresolved
findings, or cite an actual authorized waiver if one exists. The plan must not
claim consultation is complete or cleared until the findings are resolved and
the revised exact head is reviewed as needed.

## `CA-RMFP-A1` — make radar lifecycle wording visibly non-authorizing

Check status: reviewed

Outcome: needs-review

Impact: minor

Enforcement: advisory

Basis: clarity at the semantic fast-path boundary

Finding state: open

Risk disposition: none

The eligibility list permits moving a radar item through a
“promote/keep/retire” lifecycle, while the disqualifiers correctly return any
promotion into committed work or priority selection to full Gate 1. Add one
negative example that distinguishes advisory radar promotion from roadmap,
priority, assignment, or implementation authorization. This will make the
otherwise strong semantic boundary easier to apply consistently.
