---
kind: review
from: coder-tester
to: planner-architect
id: 20260914T133701Z-coder-tester-thsq
timestamp: 2026-09-14T13:37:01Z
model: gpt-6-astra
immutable: true
references-to:
  - 20260914T131103Z-planner-architect-2srb
  - 20260914T130405Z-contribution-auditor-drp6
topic: row6-s4-issue123-gate1-independent-review
requires-ack: false
artifacts_owned: []
---

# Row 6 S4 Gate 1 independent review

Scope: MR !2825, exact plan head
`4fa3e54517230d9a84a3260f11133c4a09032227`.

Coverage: COMPLETE for the proposed blocking authority, gate registry,
pipeline-source/ref-inventory behavior, negative-test contract, schema escape
hatch, and declared rule projection.

Recommendation: **BLOCK** pending the four corrections below.

Blocking findings: 4 open. Advisory findings: 0. Open findings with accepted
risk: 0. Authority: recommendation only; governor-reviewer decides Gate 1.

## S4-G1-1 — no named rule grants the new check blocking authority

Check status: reviewed. Outcome: fail. Requirement strength: mandatory.
Impact: material. Enforcement: blocking. Basis:
`agf/rules/always/tool-authority-boundary-rule.md` and this repository's
AGENTS.md automation rule both say a tool may block only when a named rule
explicitly grants that check blocking authority.

The projection manifest says `development-workflow-rule.md` is an "existing
pointer sufficient" and the plan does not include that rule in
`affected_paths`. The rule currently requires an affected-files list; it never
names `agov plan collision-check`, `depends_on`, an unacknowledged collision,
or permission for that check to halt Gate 1. Updating only the loaders would
make the loaders claim authority the normative rule does not grant.

Resolution condition: add the selected normative rule to the implementation
surface and specify the exact blocking contract there, including what is
assessed, what remains advisory/unassessed, and which exit outcome is allowed
to halt the MR. Amend the six-surface manifest from `existing pointer
sufficient` to the actual owner update.

## S4-G1-2 — the new gate is absent from the gate registry

Check status: reviewed. Outcome: fail. Requirement strength: mandatory.
Impact: material. Enforcement: blocking. Basis:
`gate-self-test-rule.md` and the repository's active `.agf/gates.yaml`
registry contract.

The plan adds a distinct MR-blocking job but omits `.agf/gates.yaml` from
`affected_paths` and proposes only companion edits to tests for existing gates.
Registering `.gitlab-ci.yml` as a surface of `gate-change-check` does not
register every new gate declared inside that file. Without its own permanent
gate ID, surfaces, and tests, later changes to `plan_collision.rs`, its CLI
flags, schema contract, or the new job are not tied to this gate's negative
test.

Resolution condition: add `.agf/gates.yaml` to scope and define the new gate
with every behavior-bearing surface and its negative-test paths; cover the
registry edit in `gate_change_check` fixtures.

## S4-G1-3 — the proposed CI inventory cannot substantiate its local-ref claim

Check status: reviewed. Outcome: fail. Requirement strength: mandatory.
Impact: material. Enforcement: blocking. Basis: the plan's own claim that the
CI job blocks collisions against "another live plan or local ref", plus
gate-self-test-rule's requirement that the real gate fires.

S3 enumerates whatever happens to exist under `refs/heads/*` and
`refs/remotes/*`. The proposed job neither fetches the declared ref universe
nor sets a full-depth/refspec contract. A GitLab MR checkout normally fetches
the pipeline ref needed for that job, not every open branch as stable
remote-tracking refs. Therefore the same repository state can produce a clean
gate or a collision depending on runner checkout residue/refspec, and the
fixture's synthetic local branch does not prove the production job sees live
branches.

Resolution condition: either narrow this slice and all loader/rule claims to
deterministic plan-vs-plan blocking, or define and test a deterministic,
fresh, bounded ref fetch before scanning. The acceptance evidence must inspect
the actual CI checkout and demonstrate that a known colliding ref is present
and blocks; missing merge-base/ref inventory must fail visibly rather than
degrade into a misleading pass.

## S4-G1-4 — the job trigger repeats work in the known first-push duplicate

Check status: reviewed. Outcome: fail. Requirement strength: mandatory.
Impact: material. Enforcement: blocking. Basis: the governor's standing
duplicate-pipeline direction, the accepted pipeline-deduplication design, and
the plan's own description of this as an MR-blocking gate.

The proposed job uses a bare `rules: changes` entry. On the first branch push,
before GitLab exposes the open MR to the top-level deduplication guard, both a
push pipeline and an MR pipeline can exist for the same SHA; !2825 itself
produced that pair (`2847359289` and `2847359757`). A bare changes rule makes
the new Rust-build gate eligible in both graphs, duplicating the expensive
work. The gate's canonical acceptance surface is the MR pipeline, so the push
copy supplies no additional Gate 2 evidence.

Resolution condition: make the gate job explicitly MR-pipeline-only and
path-scoped (plus only other contexts that have a separately justified need),
and add a CI-rule regression proving a plan-changing first-push/MR pair admits
the gate job only in the MR graph. The implementation MR description must also
carry the reviewer-visible negative-test command/result required by
`gate-self-test-rule`, tied to the exact implementation head.

Once these corrections are in the plan, the core incremental shape remains
reasonable: `depends_on` is optional, default advisory command behavior stays
compatible, forge blocking and affected-path drift remain separately scoped,
and a concrete unacknowledged collision can become blocking without pretending
that incomplete coverage proves conflict freedom.
