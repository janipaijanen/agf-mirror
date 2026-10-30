<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Role and activation copy proposal (for the Pages plan)

Status: proposal text for review. It is not site copy yet and changes no role
definition, rule or approval policy. Claim: `20261030T171431Z-planner-architect-bkmp`.
Plan: `docs/plans/20261030T163253Z-pages-structure-and-voice.md`. Every statement
in the proposed copy carries a source tag, for example `[S3]`, listed at the end.
The self-approval sentence is not written here. It is a policy wording point for
the governor-reviewer (see "Open points").

## What the sources show (findings that shape the copy)

1. Role definitions say when a role is meant to be used (`activation_condition`).
   Three roles say "always": planner-architect, coder-tester, governor-reviewer
   [S1]. Others are conditional, for example build-engineer ("project has a CI
   pipeline"), infosec-sentinel ("project submits learning records"),
   contribution-auditor ("first contribution to the framework or an adopter
   project"), and web-ux-reviewer ("any change to agf/docs/site/** or
   agf/docs/build/mkdocs.yml") [S1].
2. The adopter manifest template starts every listed role as `skipped`. It is a
   template, never read as adopter state; agov reads the project's own
   `agf/agents/MANIFEST.yaml` [S2]. So "shipped", "meant for this situation" and
   "active in this project" are three different facts. The copy must not blur them.
3. The example manifest lists 15 roles. The distribution has 16 role files. The
   documentation-estate-steward is the one the template does not list [S1][S2].
   This is template drift, outside this copy task; route it separately.
4. The documented minimum role sets are: solo trial, `coder-tester` plus
   `governor-reviewer`; standard project, those plus `planner-architect`; and two
   larger shapes that add specialists [S3].
5. Growth is signal-driven, not scheduled: "Do not split agents or add new ones on
   a schedule. Split when you feel the specific pain" [S4].

## Proposed copy

### What is a role?

A role is a responsibility with written limits: what it owns, what it may decide,
and what it must hand to someone else. A role is not a separate program. An
assistant, a person or a session plays it [S5]. One person can play more than
one role. Roles are definitions in your repository; they do not run by
themselves.

### Who approves?

Choose the person accountable for approvals. In a solo project that is you. AGF
calls this person the human governor. Every authority an assistant has is
delegated by the governor, can be revoked, and has limits [S6]. Record the choice
explicitly rather than leaving it implied in a merge [S6].

### Which roles are active?

None are active until you say so. AGF ships role definitions; your project's own
manifest says which ones you use and who or what plays each one [S2].

| What you want | Roles to activate | Source |
|---|---|---|
| Try AGF alone | `coder-tester`, plus you as `governor-reviewer` | [S3] |
| A standard governed project | the two above, plus `planner-architect` | [S3] |
| A project with CI and security needs | the standard set, plus `contribution-auditor`, `infosec-sentinel` and `fact-checker` as needed | [S3] |
| Extra independent review of a hard change | the above, plus `independent-implementation-reviewer`, named for one change | [S3][S7] |

Each role's definition lists the situation it is meant for. For example,
`build-engineer` is meant for a project that has a CI pipeline [S1].

### Add a specialist to a change

Say what you need, in plain words: "Ask for a security review of this plan."
Then four things happen, and the first and last belong to people:

1. Your assistant says which role fits and why. Use the role's own "meant for"
   condition as the reason [S1][S4].
2. You, as governor, decide whether to activate it, and record the player and
   limits in your manifest [S2]. (Open point 3: who may approve activation in an
   adopter project.)
3. The specialist reviews and writes findings. Findings are advice; they do not
   approve a gate [S7].
4. The governor decides, using those findings [S6][S7].

Do not add roles on a schedule. Add one when you meet the pain it fixes [S4].

### What a role does not give you

An implementing role does not give approval authority [S5][S7]. An advisory
reviewer does not approve a gate [S7]. Delegating bounded authority to a deputy
is a separate, opt-in arrangement that is off by default, with a list of things
that can never be delegated [S8].

## Open points (for governor or fact-checker, not decided here)

1. **Self-approval sentence.** The roles page says no agent both implements and
   approves its own work [S5]. The adoption guide lets one person play several
   roles in Phase 1, accepts a temporary reviewer conflict when a separate
   reviewer is not possible, and says a human should always play or oversee the
   governor role [S9]. These fit together if "agent" means an AI role, but that
   is a wording and policy decision. Two candidate shapes, neither chosen:
   (a) "An AI assistant never holds binding approval. A person may combine roles
   in Phase 1, with the conflict recorded." (b) Keep the current sentence and add
   a Phase 1 pointer beside it. The factual guard stays either way: switching
   role labels does not let an implementing assistant approve its own code.
2. **Sixteen versus fifteen.** Pages say sixteen roles [S5]; the template lists
   fifteen [S2]. Fix the template or word the page precisely.
3. **Who approves activating a role in an adopter project.** In this repository
   a governor approves agent status changes; I found no adopter-facing statement
   of the same rule in the pages I read. Confirm the canonical source before the
   copy says "you, as governor, decide".
4. **Phase terms.** "Phase 1" appears in role files and the adoption guide. The
   public copy should explain it in one sentence or avoid it on entry pages.

## Sources

- [S1] `agf/agents/*.yaml`: `phase`, `tier`, `activation_condition` fields (16
  role files).
- [S2] `agf/agents/MANIFEST.example.yaml`: header comment (template, never read
  as adopter state; agov reads `agf/agents/MANIFEST.yaml`); 15 role entries, all
  `status: skipped`.
- [S3] `agf/ADOPTING.md`, "Pick the minimum role set" table.
- [S4] `agf/ADOPTING.md`, "Step 7: Grow When It Hurts", signal table.
- [S5] `agf/docs/site/docs/concepts/roles-and-gates.md`, "The core idea" and the
  pipeline roles table.
- [S6] Same page: human governor paragraph and "Do not leave the governor role
  implicit".
- [S7] Same page: independent-implementation-reviewer paragraph and the
  agent self-approval row of the failure table.
- [S8] `agf/rules/workflow/deputy-governor-rule.md`, sections 1 and 2 (default
  state: no delegation; red list).
- [S9] `agf/ADOPTING.md`, "Step 1: Map Phase 1 Roles to Your Team", guidelines.

## Limits

Read-only desk review of the sources above. I did not check every page that
mentions roles. The 16-question baseline is the evidence for whether readers
understand this; this proposal is not a score. Provenance limit for that
baseline: the web-UX reviewer authored part of the current v2.13.1 site, so its
critique is author-informed and not an independent baseline. The walkthrough was
reassigned to the non-author InfoSec review session, with an authorship check and
a disclosure of its prior plan-review familiarity. Preserve this limit wherever
baseline results are quoted.
