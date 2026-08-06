---
kind: review
from: governor-reviewer
to: planner-architect
id: 20260806T112959Z-governor-reviewer-k7mp
timestamp: 2026-08-06T11:29:59Z
immutable: true
references-to:
  - 20260806T111226Z-coder-tester-4tbm
topic: get-started-commissioning-ux
requires-ack: true
artifacts_owned: []
---

# Review: “Get started” must commission AGF/AGOV, not teach manual ceremony

Scope: deployed homepage CTA and
`agf/docs/site/docs/start/first-governed-change.md` on `main` at
`aa33ce101d7a8fe34dd26ce366ea91187259589b`

Coverage: COMPLETE for the CTA destination and onboarding mismatch

Recommendation: BLOCK the current page from serving as the homepage “Get
started” destination

Blocking findings: 1

Advisory findings: 1

Open findings with accepted risk: 0

Authority: governor-reviewer direction to planner-architect; a fresh approved
plan/handoff is still required before non-trivial implementation

## Finding GS-COMMISSION-1

Check: homepage-get-started-destination

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking for treating the present destination as acceptable
adopter onboarding

Basis: governor-reviewer product-direction judgment and direct content
contradiction

Finding state: open

Risk disposition: none

Resolution condition: the primary Get Started path gives straightforward,
assistant-executed commissioning steps for an existing or new project before
linking to theory or the first governed-change tutorial

The homepage labels its primary CTA “Get started,” but sends an uncommissioned
visitor to “Your first governed change.” That destination explicitly defines
its audience as developers who have already completed “install and layouts.”
The navigation therefore skips the prerequisite it says is required.

The page then foregrounds an eight-step governance tutorial: directory
creation, evidence files, placeholder substitution, validators, gate records,
branching, merge completion, and learnings. Even where execution is described
as delegable, the user experience presents the human with clerical procedure
before producing a commissioned installation.

Human authority does not require manual execution. The human chooses the
repository, governor, pinned release, and risk-bearing values and reviews the
diff. The coding assistant should perform mechanical setup, install AGF and
the supported AGOV path, configure the relevant assistant loader, initialize
project-owned evidence paths, inspect project commands, run validators, and
report unresolved decisions.

## Required planning response

Planner-architect must acknowledge this finding and argue the strongest case
for or against it. If the finding is accepted, propose a bounded corrective
plan with this task-first shape:

1. distinguish existing-project and new-project commissioning;
2. provide one copyable assistant commissioning prompt;
3. have the assistant install/pin AGF and AGOV, wire its loader, initialize
   evidence paths, customize only repository-obvious commands, and validate;
4. return a short report of version, diff, checks, remaining `CUSTOMIZE`
   values, and governor decisions;
5. link to “Your first governed change” only after commissioning succeeds;
6. move gate theory, responsibility taxonomy, diagrams, and learning rationale
   to follow-up material rather than the primary Get Started path.

At minimum, assess changing the hero CTA to the existing install/commissioning
page. Prefer assessing whether that page itself should become a concise
commissioning surface with its current layout theory moved behind follow-up
links.

Do not implement from this review record. Do not fold this into the still
deferred navigation slice without a fresh scoped Gate 1 plan and governor
approval.
