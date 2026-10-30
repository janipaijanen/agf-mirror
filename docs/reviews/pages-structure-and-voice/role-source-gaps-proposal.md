<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Role source gaps: scoped follow-up proposal

Status: proposal for a scoped disposition. It changes no rule, schema, role file,
manifest, loader or ADOPTING text, and grants nothing. Claim:
`20261030T184503Z-planner-architect-12qq`. Origin: advisory review
`20261030T182931Z-planner-architect-6mt2` and `role-activation-copy-proposal.md`.
Each gap needs its own scope and binding authority; the Pages work does not
include them and must not decide them.

| Gap | What the sources say | Canonical owner to disposition | Smallest candidate disposition |
|---|---|---|---|
| G1 core roles vs solo trial | `framework.md`: planner, coder and governor are always active from the first governed change. `ADOPTING.md`: solo trial lists only coder-tester and governor-reviewer. | `framework.md` and `ADOPTING.md` owners | State that the solo trial is a documented reduction of the core arrangement, or add the planner to the trial row. One canonical sentence, then the Pages copy follows it. |
| G2 CI role omitted | The CI/security row in `ADOPTING.md` omits `build-engineer`; its definition and `framework.md` make it conditional on a CI pipeline. | `ADOPTING.md` owner | Split the row into CI ownership (build-engineer) and security review, or add build-engineer to it. |
| G3 template misses a role | 16 role files ship; the manifest template has 15 entries and omits `documentation-estate-steward`. | `agf/agents/MANIFEST.example.yaml` owner | Add the missing entry as `skipped`, or document why it is absent. A role-catalog change needs the agent-definition authority. |
| G4 activation_condition coverage | 10 of 16 definitions declare it; six do not (see the copy proposal). | Agent schema and role-file owners | Either require it for all roles or state in the schema description that it is optional. |
| G5 blanket gate wording | The day-to-day page lists gate approvals as human-only; the deputy rule allows bounded opt-in delegation. | Day-to-day page and `deputy-governor-rule.md` owners | Add one qualifying clause to the page that points to the deputy rule. A page-only wording fix, but it touches normative guidance. |

## What I am not proposing

No policy change, no new role, no activation, and no release version or date. The
self-approval, Phase 1 and deputy description stays source-bounded in the copy
proposal. Where the owner is the human governor, the disposition is theirs.

## Limits

Read-only comparison of the files named above on `main` at `544e4a9f`. I did not
trace every other page that repeats these statements.
