<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Role source gaps: scoped follow-up proposal

Status: proposal for a scoped disposition. It changes no rule, schema, role file,
manifest, loader or ADOPTING text, and grants nothing. Claim:
`20261030T185859Z-planner-architect-5msk` (revision 3). This artifact makes no policy
change. Origin: advisory reviews
`20261030T182931Z-planner-architect-6mt2` and `20261030T185215Z-planner-architect-ehb8` and `role-activation-copy-proposal.md`.
Each gap needs its own scope and binding authority; the Pages work does not
include them and must not decide them.

| Gap | What the sources say | Canonical owner to disposition | Candidate dispositions (not decisions) |
|---|---|---|---|
| G1 core roles vs solo trial | `framework.md`: planner, coder and governor are always active from the first governed change. `ADOPTING.md`: the solo trial lists only coder-tester and governor-reviewer. | `framework.md` and `ADOPTING.md` owners | Ask who owns planning and Gate 1 plan authorship in the solo trial, or which changes that trial can govern. A label saying "reduction" alone leaves that gap open. Alternative candidate: include the planner in the trial row. |
| G2 CI role omitted | The CI/security row in `ADOPTING.md` omits `build-engineer`; its definition and `framework.md` make it conditional on a CI pipeline. | `ADOPTING.md` owner | Split the row into CI ownership (build-engineer) and security review, or add build-engineer to it. |
| G3 template misses a role | 16 role files ship; the manifest template has 15 entries and omits `documentation-estate-steward`. Every listed role starts `skipped`. | `agf/agents/MANIFEST.example.yaml` owner | Add the missing entry as `skipped`, or document why it is absent. A role-catalog change needs the agent-definition authority. |
| G4 activation_condition coverage | 10 of 16 definitions declare it. The schema does not require it (`required` lists name, id, version, framework_version, tier). | Agent schema and role-file owners | Ask whether the six omissions are intentional and what guidance applies. Making the field mandatory would be a separate schema and policy proposal, not made here. |
| G5 gate and delegation wording | The day-to-day page lists gate approvals as human-only. `roles-and-gates.md` lines 72 to 75 say the human governor owns Gate 1, Gate 2, release approval and binding risk acceptance. Lines 196 to 199 call them non-delegable unless a future AGF version changes the role model. The deputy rule allows bounded, opt-in delegation with a red list. | Day-to-day page, `roles-and-gates.md` and `deputy-governor-rule.md` owners | Reconcile the three passages and distinguish the human governor's ultimate accountability from who may exercise a bounded approval. Do not publish a blanket delegation claim. |

## What I am not proposing

No policy change, no new role, no activation, and no release version or date. The
self-approval, Phase 1 and deputy description stays source-bounded in the copy
proposal. Where the owner is the human governor, the disposition is theirs.

## Limits

Read-only comparison of the files named above on `main` at `544e4a9f`. I did not
trace every other page that repeats these statements.
