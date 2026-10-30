<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Role and activation copy proposal (revision 2, for the Pages plan)

Status: proposal text for review. It is not site copy. This artifact makes no
policy change, and it changes no role definition, rule or manifest. Claim:
`20261030T185859Z-planner-architect-5msk` (revision 3; earlier claim `12qq`). Plan:
`docs/plans/20261030T163253Z-pages-structure-and-voice.md`. Revision 2 answers
advisory reviews `20261030T182931Z-planner-architect-6mt2` and
`20261030T185215Z-planner-architect-ehb8`. I checked each correction in those
reviews against the sources before using it. Where sources
disagree, this file shows the disagreement and does not choose a winner. The
prior governor delegation has expired, so nothing here is an approval.

## What the sources show (corrected)

1. Ten of the sixteen role definitions declare `activation_condition`. Six do
   not: academic-peer-reviewer, academic-visual-reviewer, capability-developer,
   deployment-engineer, documentation-writer, fact-checker [S1]. Revision 1 said
   "each role". That was wrong.
2. The schema says `activation_condition` is the trigger for the role's function
   being needed, and that `always` means needed from the first governed change
   [S2]. It is not a runtime launch and not the manifest status field.
3. Four different facts must stay apart: a **shipped definition** (the file), a
   **need or trigger** (its activation condition), an **assignment** (who or what
   plays it, recorded in the project's manifest and by the governor), and a
   **running participant** (an assistant, person or session doing the work) [S1]
   [S3][S4].
4. Every role the manifest template lists starts as `skipped` with a placeholder
   (the template omits one shipped definition, see D-3). It is inert, never read as adopter state [S3]. That does not make every role
   optional: the core roles are needed from the first governed change [S5].
5. By default the human governor decides at both gates. Delegating bounded
   approval authority to a deputy is a separate, opt-in arrangement, off by
   default, with a red list of things that can never be delegated [S6].
6. The day-to-day page lists role activation and deactivation as human-only
   authority [S7].

## Source disagreements to keep visible (do not resolve in copy)

- **D-1 core roles vs solo trial.** The framework core table says the
  planner-architect, coder-tester and governor-reviewer are always active from the
  first governed change [S5]. The adoption guide's minimum-role table offers a
  solo trial with only coder-tester and governor-reviewer [S8]. A public default
  must not pick one silently.
- **D-2 CI role.** The adoption guide's CI/security row lists contribution-auditor,
  infosec-sentinel and fact-checker but not build-engineer [S8]. The build-engineer
  definition and the framework text make it conditional on a CI pipeline [S1][S5].
  CI ownership and security review are separate responsibilities.
- **D-3 template gap.** Sixteen definitions ship; the manifest template has
  fifteen entries and omits documentation-estate-steward [S1][S3].

## Proposed copy

### What is a role?

A role is a responsibility with written limits: what it owns, what it may decide
and what it must hand to someone else. A role is not a separate program. An
assistant, a person or a session plays it. One person can play more than one
role. Roles are definitions in your repository; they do not run by themselves
[S1][S4].

### Phase 1 in one sentence

Phase 1 is the starting arrangement: related responsibilities are combined into a
few roles, and you split them later only when a specific problem tells you to
[S9]. Role files and the adoption guide use the word; entry pages do not need it.

### Who approves?

Choose the person accountable for approvals. In a solo project that is you. AGF
calls this person the human governor. Name the person explicitly, and record the
decision rather than leaving it implied in a merge [S4][S10]. By default the
human governor makes the plan decision and the merge decision. Any authority an
assistant has is delegated, bounded and revocable [S6][S10].

*Source tension (G5), not resolved here:* `roles-and-gates.md` also says the human
governor owns Gate 1, Gate 2, release approval and binding risk acceptance, and
calls them non-delegable unless a future AGF version changes the role model
[S12]. The deputy rule describes a bounded, opt-in delegation with a red list [S6].
Copy must distinguish the human governor's ultimate accountability from who may
exercise a bounded approval, and must not publish a blanket delegation claim.

### Planning, building and approving are separate

Planning, implementation and approval are separate responsibilities. The
documented common solo arrangement has an assistant prepare work and a named
human approve it. One person may temporarily hold several Phase 1 roles, but that
conflict is not independent review. An assistant that implemented a change does
not gain authority to approve that same work merely by switching role labels.
Deputy approval requires separate, bounded, recorded authority and stays subject
to the role's prohibitions. The human governor keeps ultimate authority and may
revoke a delegation [S4][S6][S8][S11].

(This paragraph is the source-bounded text proposed in review `6mt2`. I verified
its parts against the cited sources. It describes existing rules and changes no
policy. The governor decides whether the public page uses it.)

### Which roles do I need?

*Hold for disposition of D-1 and D-2 before any public use.* Sourced description:
the framework's core arrangement has three roles needed from the first governed
change: `planner-architect`, `coder-tester` and `governor-reviewer` [S5]. The
adoption guide also describes a lighter solo trial with `coder-tester` and a human
`governor-reviewer` [S8]. Add specialists when their trigger applies [S1][S9]:

| Need | Role | Trigger in its definition |
|---|---|---|
| Your project has a CI pipeline | `build-engineer` | "project has a CI pipeline" [S1] |
| Security screening of learning records | `infosec-sentinel` | "project submits learning records" [S1] |
| Screening of contributions | `contribution-auditor` | "first contribution to the framework or an adopter project" [S1] |
| Review of site changes | `web-ux-reviewer` | "any change to agf/docs/site/** or agf/docs/build/mkdocs.yml" [S1] |
| Other specialists | six roles have no declared trigger | read the role file and the adoption guide's growth signals [S1][S9] |

### The project manifest

When your project uses a manifest, it records which roles you actually use and who
or what plays each one. The shipped template is only a starting point: every entry
starts as skipped, and nothing in it is read as your state [S3]. The adoption guide
creates a manifest when your project needs one; do not present it as a new
prerequisite [S3][S8].

### Ask for a specialist during a change

Say what you need: "Ask for a security review of this plan." Then, in order:

1. **The assistant proposes.** It names the role that fits and why. Use the role's
   declared trigger where it has one [S1]. Six roles declare none; for them use the
   role's stated purpose and the adoption guide's growth signals [S1][S9], and ask
   the role's owner to clarify when that is still ambiguous.
2. **A person assigns.** The day-to-day page lists role activation and deactivation
   as human-only authority [S7]. The human records who or what plays the role in the
   manifest [S3]. Recording the role's limits as well is advisory practice; I found
   no source that requires it, so the owner should confirm before copy says "must".
3. **The specialist advises.** Findings are advice; an advisory reviewer does not
   approve a gate [S11].
4. **The gate decision.** By default the human governor decides, using the findings
   [S6][S11]. Only a recorded, bounded deputy delegation changes who decides [S6].

An already active specialist can be consulted within its current scope. That is
not a new activation.

The human-only presentation stays qualified by its source. The deputy rule has a
broader scope description and a red list that excludes rule and agent-definition
changes, including `agf/agents/*.yaml`; absence from a named red-list bullet is not
permission, and any deputy action needs a valid matching artifact [S6]. Do not infer
new activation or manifest-edit authority from these sources.

## Open points (not decided here)

1. D-1, D-2 and D-3 above need a scoped disposition from their canonical owners.
   See `role-source-gaps-proposal.md`.
2. Whether the public page uses the paragraph "Planning, building and approving are
   separate" as written: governor decision.
3. The gate wording on the day-to-day page and in `roles-and-gates.md` (lines 72 to
   75 and 196 to 199) must be reconciled with the deputy rule by their owners (G5).
   That is a separate source correction, not made here.

## Sources

- [S1] `agf/agents/*.yaml`: `phase`, `tier`, `activation_condition` (declared in 10
  of 16 role files), `forbidden_actions`.
- [S2] `agf/schemas/agent.schema.json`, `activation_condition` description.
- [S3] `agf/agents/MANIFEST.example.yaml`: header comment; 15 entries, all `skipped`.
- [S4] `agf/docs/site/docs/concepts/roles-and-gates.md`: "The core idea", pipeline
  roles, human governor paragraph.
- [S5] `agf/framework.md`, "Phase 1: Minimum Viable System", core agents "always
  active from the first governed change".
- [S6] `agf/rules/workflow/deputy-governor-rule.md`, sections 1 to 4.
- [S7] `agf/docs/site/docs/operate/day-to-day.md`, responsibility table, human-only
  authority row.
- [S8] `agf/ADOPTING.md`: "Pick the minimum role set" table; Step 1.1 guidelines.
- [S9] `agf/ADOPTING.md`, "Step 7: Grow When It Hurts".
- [S10] `roles-and-gates.md`: "Do not leave the governor role implicit".
- [S12] `roles-and-gates.md`, the passages on human governor ownership of gates
  (lines 72 to 75) and non-delegable authority (lines 196 to 199).
- [S11] `roles-and-gates.md`: independent-implementation-reviewer paragraph and the
  agent self-approval failure row; `coder-tester.yaml` forbidden actions (merging
  without governor approval); `governor-reviewer.yaml` denied capabilities.

## Limits

Read-only desk review. I did not check every page that mentions roles. The baseline
is a source-familiar expert assessment of whether answers can be located or derived
from the pages. It is not measured first-time-user comprehension and gives no task
times. Two things must stay separate. The web-UX reviewer authored part of the
current v2.13.1 site, so its critique is author-informed. The actual baseline walkthrough
(`73gh`) was done by a non-author session that disclosed prior plan and source
familiarity. Preserve both limits wherever results are quoted.
