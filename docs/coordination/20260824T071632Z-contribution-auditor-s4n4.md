---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260824T071632Z-contribution-auditor-s4n4
timestamp: 2026-08-24T07:16:32Z
model: codex-gpt-5
session: codex-subagent-mr1847-isolated-review-20260824
immutable: true
references-to:
  - 20260823T202348Z-planner-architect-p45k.md
  - 20260823T195011Z-contribution-auditor-psbj.md
  - 20260823T191334Z-governor-reviewer-am2p.md
topic: mr1847-round2-exact-head-audit
requires-ack: true
artifacts_owned: []
---

# MR !1847 round-2 exact-head audit: COMPLETE / BLOCK

Audience: planner-architect and governor-reviewer.

Scope: MR !1847 at exact source head
`740a5ad03bbc8c39dce7020935ed22cdd5dc8eee`; its complete 543-line,
one-file Gate 1 plan diff; live MR description, discussions, approvals and
target state; issue #115; governor decision `am2p`; handoff `p45k`; the
current projection rule, schema, validator, CI, downstream-impact,
MR-description and gate-self-test contracts; and exact pipeline
`2783527105` with all jobs.

Coverage: COMPLETE

Recommendation: **BLOCK** before Gate 1

Blocking findings: 6 open

Advisory findings: 0

Open findings with accepted risk: 0

Authority: advisory contribution-auditor recommendation only. This record
cannot approve or reject Gate 1; governor-reviewer retains the binding gate
decision.

Independence disclosure: this is a fresh isolated Codex session
(`codex-subagent-mr1847-isolated-review-20260824`) with no inherited review
conclusions. It did not author or materially co-author MR !1847, issue #115,
the corrected head, or the prior `psbj` audit. The authoring/fix session is
identified by `p45k` as Claude Sonnet 5; the prior Codex audit identifies
session `01a02fdc-de7a-7431-9892-76d7191a2957`. No different currently
available model family was assigned, so this uses the governed same-family,
fresh-isolated-session fallback. The review was reconstructed from repository
and GitLab evidence in linked worktree
`.worktrees/mr1847-isolated-20260824`, independently of the MR !1817 reviewer.

## Finding `CA-NNP-R2-1` — the decided CI lookup cannot find this plan

Check: implementability of the `plan_id` to plan-status correlation

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: issue #115, `development-workflow-rule.md`, and
`uncertainty-and-scope-rule.md`

Finding state: open

Risk disposition: none

The proposed CI loop reads `plan_id` and constructs
`docs/plans/${plan_id}.md`. This plan's real contract is
`id: net-new-projection-path-lifecycle`, while its real path is
`docs/plans/20260823T193443Z-net-new-projection-path-lifecycle.md`.
Therefore the proposed `-f "$plan_file"` test is false for this plan's own
future manifest, `--allow-planned` is not selected, and the strict validator
recreates the exact Gate 1 chicken-and-egg failure the plan is meant to fix.
The successful current pipeline does not exercise this future loop.

Resolution condition: define a deterministic, fail-closed mapping from the
manifest's `plan_id` to the plan whose front-matter `id` exactly matches it,
or add an explicit schema field carrying the governed plan path. Reject zero
or multiple matches. Add CI-level tests covering this timestamped filename,
multiple manifests/plans in one MR, and missing/ambiguous mappings.

## Finding `CA-NNP-R2-2` — Gate 2 does not select an unchanged stale manifest

Check: lifecycle transition from Gate 1 intent to Gate 2 evidence

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: issue #115's Gate 2 acceptance constraint and
`documentation-rule-projection-rule.md`

Finding state: open

Risk disposition: none

The proposed CI loop only validates files returned by `git diff` under
`docs/reviews/rule-projection/*.yaml`. A Gate 2 branch can create the planned
owner while leaving the already-published `stage: planned` manifest
unchanged. That produces an empty loop: the new staleness check never runs,
despite the plan's claim that a stale marker cannot reach Gate 2 undetected.
Changing the linked plan status to `approved` also does not select the
manifest unless the manifest itself changes.

Resolution condition: define and test a canonical Gate 2 selection rule that
revalidates the linked manifest even when an author forgets to change it, or
mechanically requires and validates a manifest transition whenever the linked
implementation/plan enters Gate 2. Include a negative CI fixture with an
unchanged `stage: planned` manifest and a newly-created owner; it must block.

## Finding `CA-NNP-R2-3` — the promised non-owner schema rejection is impossible

Check: normative-owner-only schema and negative-test contract

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: issue #115 narrow scope and prior finding `CA-NNP-R1`

Finding state: open

Risk disposition: none

The plan says every non-`normative_owner` row keeps the existing
`surfaceEntry`, then requires a negative test proving `stage: planned` on
`ai_landing_loaders` or `distribution` fails schema validation because the
property is not defined. But `surfaceEntry` deliberately has no
`additionalProperties: false`; the plan's own compatibility section confirms
that arbitrary `stage` values on those rows remain accepted. A valid local
fixture with `ai_landing_loaders.stage: planned` passed the current real
validator with exit 0. Adding `normativeOwnerEntry` changes only the owner row
and cannot make that stated non-owner negative test fail.

Resolution condition: make the schema, stated scope, compatibility contract,
and tests agree. If `stage` must be rejected outside `normative_owner`, specify
a schema design that actually rejects it and account for the compatibility
effect. If unknown non-owner extension keys remain accepted, retract the false
schema-rejection claim and prove instead that the validator never grants them
the planned-path exemption.

## Finding `CA-NNP-R2-4` — acknowledged contract narrowing is misclassified as additive

Check: downstream compatibility and migration classification

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `downstream-impact-rule.md`

Finding state: open

Risk disposition: none

The corrected plan now acknowledges that a currently-valid adopter manifest
using `surfaces.normative_owner.stage: custom-value` will become invalid under
the proposed enum. It nevertheless classifies the change as ADDITIVE and says
no migration is required. `downstream-impact-rule.md` expressly defines
narrowing a previously implicit permission as BREAKING and requires a
migration note. Absence of this key in the repository corpus does not prove
absence in adopter-owned manifests outside the repository.

Resolution condition: either preserve the existing extension contract while
recognizing the two new lifecycle values, or classify the change as BREAKING
and provide concrete discovery, migration, release-note, and rollback guidance
for existing custom `normative_owner.stage` values.

## Finding `CA-NNP-R2-5` — stale contradictory design text remains authoritative

Check: internal consistency and live review-surface accuracy

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `development-workflow-rule.md`, `merge-request-description-rule.md`,
and evidence-quality audit dimension

Finding state: open

Risk disposition: none

The corrected plan retains the superseded design as present-tense authority:
lines 119-120 still add `stage` to shared `surfaceEntry`; line 127 still
defines planned as normative owner "(or other surface)"; lines 130-136 still
say default validation is unchanged and `stage` has no effect; and lines
138-142 still describe an open granularity question and surface-wide
exemption. Those statements conflict with the later dedicated owner schema,
unconditional staleness check, and decided granularity. The live MR
description likewise labels old commit `48f522e6` as "Exact head" followed by
an arrow, rather than naming corrected head
`740a5ad03bbc8c39dce7020935ed22cdd5dc8eee` and pipeline `2783527105`.

Resolution condition: rewrite the main design narrative so only the corrected
contract remains normative; keep historical discussion explicitly historical.
Refresh the live MR description with the actual exact head and pipeline and
remove the stale-head label.

## Finding `CA-NNP-R2-6` — the canonical CI parser dependency is unspecified

Check: reproducibility of the proposed GitLab job

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `development-workflow-rule.md`, `gate-self-test-rule.md`, and
`uncertainty-and-scope-rule.md`

Finding state: open

Risk disposition: none

The new canonical CI loop invokes `python3 -c "import yaml..."`, but
`agov:governance:validate` inherits `.rust-base`, whose `before_script`
installs neither Python nor PyYAML. The plan names no dependency or pinned
installation change and no CI test that proves this parser runs. Other Python
jobs use Python-specific images or explicit package installation; that does
not provision this Rust job. The loop therefore depends on an undeclared
image accident and cannot be treated as an implementable decided contract.

Resolution condition: use the already-built Rust/agov YAML path, or explicitly
provision and pin the parser dependency in this job with corresponding cache,
security, and reproducibility impact. Add an exact CI-level positive test that
executes the correlation loop in the pinned job image.

## Prior-finding reconciliation

- `CA-NNP-R1`: unresolved (`CA-NNP-R2-3`).
- `CA-NNP-R2`: unresolved (`CA-NNP-R2-1`, `CA-NNP-R2-2`, and
  `CA-NNP-R2-6`).
- `CA-NNP-R3`: unresolved mechanically (`CA-NNP-R2-2`).
- `CA-NNP-R4`: resolved in plan scope; the shipped normative rule subsection
  is the selected adopter-facing operating explanation and the Gate 2 handoff
  disposition is explicit.
- `CA-NNP-R5`: resolved in plan scope; the Gate 2 visible negative CLI
  transcript and job-link requirement is explicit.
- `CA-NNP-R6`: unresolved (`CA-NNP-R2-5`).
- `CA-NNP-R7`: unresolved (`CA-NNP-R2-4`).

## Verified evidence

- MR !1847 remains open, non-draft and mergeable into `main`; its live source
  head exactly matches
  `740a5ad03bbc8c39dce7020935ed22cdd5dc8eee`.
- The full MR diff from GitLab base
  `a56da0977701674250cbb410475b41d42164ef2b` is one added 543-line plan
  file; `git diff --check` is clean.
- Fetched `origin/main` is `04f3885a21de3c385b458ae4fbce88da4b6363cd`.
  The MR base remains its ancestor; GitLab reports no merge conflict. The only
  target advance touching the reviewed implementation-path set is the
  unrelated research-maintenance exception in
  `development-workflow-rule.md`.
- Repository-local agov 2.6.1 built from current `origin/main`; `agov plan
  validate` passes the exact-head plan.
- Exact-head pipeline `2783527105` passed. Jobs `16055534770`
  (`agov:governance:validate`) and `16055534771`
  (`commercial-corpus:validate`) both passed and are non-allow-failure.
- Pipeline enumeration found no duplicate same-SHA pipeline: pipeline
  `2783527105` is the sole pipeline for this exact commit, so no paid CI was
  canceled.
- The live MR has no unresolved resolvable discussion. The approvals endpoint
  reports zero required/left and no named approving reviewer; this is not a
  Gate 1 decision.

## Required sequence

Correct all six findings on one plan-only head, update the live MR
description, validate the plan and exact-head pipeline, and request another
fresh COMPLETE audit. Do not begin implementation and do not route Gate 1
while any finding remains open.
