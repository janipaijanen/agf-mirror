<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Role and activation copy proposal (revision 2, for the Pages plan)

Status: proposal text for review. It is not site copy and changes no role
definition, rule, manifest or approval policy. Claim:
`20261030T184503Z-planner-architect-12qq`. Plan:
`docs/plans/20261030T163253Z-pages-structure-and-voice.md`. Revision 2 answers
advisory review `20261030T182931Z-planner-architect-6mt2`. I checked each
correction in that review against the sources before using it. Where sources
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
4. The manifest template lists every role as `skipped` with a placeholder. It is
   inert, never read as adopter state [S3]. That does not make every role
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

1. **The assistant proposes.** It names the role that fits and why, using the role's
   own trigger [S1][S9].
2. **A person assigns.** Role activation and deactivation are human-only authority.
   The human records the player and limits [S3][S7].
3. **The specialist advises.** Findings are advice; an advisory reviewer does not
   approve a gate [S11].
4. **The gate decision.** By default the human governor decides, using the findings
   [S6][S11]. Only a recorded, bounded deputy delegation changes who decides [S6].

An already active specialist can be consulted within its current scope. That is
not a new activation.

## Open points (not decided here)

1. D-1, D-2 and D-3 above need a scoped disposition from their canonical owners.
   See `role-source-gaps-proposal.md`.
2. Whether the public page uses the paragraph "Planning, building and approving are
   separate" as written: governor decision.
3. The day-to-day page's gate wording must not override the deputy rule
   (`6mt2`, point 5). That is a separate source correction, not made here.

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
- [S11] `roles-and-gates.md`: independent-implementation-reviewer paragraph and the
  agent self-approval failure row; `coder-tester.yaml` forbidden actions (merging
  without governor approval); `governor-reviewer.yaml` denied capabilities.

## Limits

Read-only desk review. I did not check every page that mentions roles. The
16-question baseline measures whether readers understand the pages, not this
proposal. Provenance limit for that baseline: the web-UX reviewer authored part of
the current v2.13.1 site, so its critique is author-informed. The walkthrough was
done by a non-author session with disclosed familiarity, as an expert walkthrough
and not a first-time-user study. Preserve this limit wherever results are quoted.
