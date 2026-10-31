# Roles and gates

*Status: Released. Current framework version: see [Release information](../../reference/release-info.md). Audience: developers and
governors building the mental model. After this page you can name the
roles, say what each gate decides, and read a coordination chain.*

!!! note "AGF v2 layout"
    Role definitions ship under `agf/agents/`. Gate evidence is recorded in the
    adopter-owned coordination and plan paths, `docs/coordination/` and
    `docs/plans/` by default.

## Roles in plain terms

A role is a responsibility with written limits: what it owns, what it may decide,
and what it must hand to someone else. A role is not a separate program, and it is
not necessarily another person. Your assistant, a person or a session plays it, and
one participant can play more than one role. Roles are definitions in your
repository, and they do not run by themselves.

Three responsibilities matter first. Someone plans the work, someone implements and
tests it, and someone approves it. Approval of the plan is called Gate 1. Approval
of the finished change is Gate 2. The person accountable for approvals is the
**human governor**. In a solo project, that is you.

## The core idea: separated authority

Planning, building and approving are separate responsibilities. An assistant may
plan and implement a change; neither task gives it approval authority. Prefer a
human approver who did not implement the change. Phase 1 is the starting
arrangement, in which related responsibilities are combined into a few roles. In
Phase 1, if that separation is not possible, the
[adoption guide, section 1.1](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/agf/ADOPTING.md) permits one person to play both the
Coder-Tester and Governor-Reviewer roles and accept the conflict temporarily.
Record the assignment and the approval explicitly. This human role conflict does
not permit an implementing assistant to approve its own work. Eligible Gate 2
delegation remains subject to the bounded deputy route described
[below](#approval-authority-and-optional-deputies).

Authority is split across role definitions, each a YAML contract stating jurisdiction (what
the role owns), authority (what it may decide), and escalation (what it
must hand upward). An implementing agent that "approves" its own merge has
acted outside its definition, and the audit trail shows it. Some checks are
structurally enforced by tooling; others, including reviewer approval, rely on
the participants following the recorded authority boundary.

## The pipeline roles

Table 1 introduces the three core pipeline roles before the gate figures:
planning, implementation, and approval are deliberately separated.

| Role | Owns | Typical artifact |
|---|---|---|
| **planner-architect** | Plans, decomposition, ADRs | Plan files, Gate 1 submissions |
| **coder-tester** | Implementation and tests | Branches, commits, Gate 2 submissions |
| **governor-reviewer** | Approvals at both gates | Decision records, review findings |

Around the core pipeline sit supporting roles — build engineering,
security sentinel, contribution auditing, and others. The shipped
[quick reference](https://gitlab.com/janipaijanen/agents-govern-framework)
(`docs/reference/quick-reference.md` in the distribution) lists all
sixteen with one-line responsibilities. The **human governor** retains
ultimate approval authority. Deputy approval authority is opt-in, revocable,
and bounded by the [deputy-governor rule](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/agf/rules/workflow/deputy-governor-rule.md)'s
full red list. That red list limits approval delegation; it does not by itself
prohibit an agent from performing otherwise-authorized implementation or
review work, which remains subject to its role boundaries and the applicable
gates.

One of those sixteen, **independent-implementation-reviewer**, is worth
naming here specifically: it performs a read-only, exact-head review of
an implementation against its approved Gate 1 plan and evidence, by a
session and model distinct from the implementer. It is not a fourth
binding gate — its `CLEAR`/`BLOCK` recommendation is advisory evidence
for the governor's Gate 2 decision, the same way any other domain
reviewer's findings are, and it never approves or rejects either Gate 1
or Gate 2 directly. A plan or the governor names it for a specific
non-trivial Gate 2 candidate when additional independent-review
separation is warranted; it is not activated for every change.

Do not leave the governor role implicit. In a solo project, the maintainer can
serve as governor, but the approval still needs to be a deliberate recorded
decision rather than an assumption hidden in the merge. In a team, name the
person or rotation that holds governor authority for the current period, and
name a backup before work blocks on unavailable review. "Everyone can approve"
usually becomes "no one owned the decision"; if a rotating governor model fits
your team, treat the rotation itself as part of the project convention.

For teams, record the operating model before the first busy week:

- primary governor for the current repository or milestone;
- backup governor and when the backup may act;
- rotation cadence, if any, and where the active governor is recorded;
- conflict-of-interest rule for work authored or sponsored by the governor;
- emergency override path and the follow-up review it requires;
- domain-specific reviewers, such as security, database, privacy, or release
  owners, whose advice may be required before the governor decides;
- revocation path for any delegated authority.

## Approval authority and optional deputies

AI assistants may prepare gate packets, compare diffs against plans, summarize
risks, or draft decision-record skeletons. That is AI-assisted review, not AI
governorship. The human governor retains ultimate approval authority. The
framework offers one narrow, opt-in route for eligible Gate 2 decisions: the
[deputy-governor rule](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/agf/rules/workflow/deputy-governor-rule.md). It is off by
default and provisional. It needs a written delegation with scope, severity
ceiling, expiry, audit trigger and revocation. Every deputized decision is
logged, the rule's full red list is never delegable, and role definitions still
forbid self-approval. Anything the rule does not expressly cover stays with the
human governor. This page does not make Gate 1, release approval, or binding
risk acceptance delegable. Ordinary task grants, described below, are distinct
from deputy approval.

## Which roles does my project need?

Your project uses only the roles it assigns. The definitions in `agf/agents/` are a
library, and having a definition does not make a role active. Start from your
operating model, then add a specialist when its trigger applies.

| Situation | Roles to assign | Notes |
|---|---|---|
| Solo trial | `planner-architect`, `coder-tester`, `governor-reviewer` | You are the human governor. One assistant may play both `planner-architect` and `coder-tester`. Planning and Gate 1 plan authorship still happen. |
| Standard governed project | `planner-architect`, `coder-tester`, `governor-reviewer` | The players may be separate. Name each one. |
| Project with a CI pipeline | add `build-engineer` | Trigger in its definition: "project has a CI pipeline". |
| Project that submits learning records | add `infosec-sentinel` | Trigger: "project submits learning records". Required once the project submits them. |
| Project that takes contributions | add `contribution-auditor` | Trigger: "first contribution to the framework or an adopter project". |
| Project that publishes factual claims | add `fact-checker` | Recommended from the first release, as needed. |
| Change to the public site or its build configuration | add `web-ux-reviewer` | Trigger: "any change to agf/docs/site/** or agf/docs/build/mkdocs.yml". |
| A non-trivial Gate 2 candidate that needs independent review | name `independent-implementation-reviewer` for that candidate | Not a gate for every change. Its findings are advice. |

Some shipped roles declare no activation condition. For those, read the role file
and the growth signals in `agf/ADOPTING.md`. The agent schema makes the
`activation_condition` field optional, so a missing one is not an error.

To see which roles your own project uses, read your project's manifest,
`agf/agents/MANIFEST.yaml`, if it keeps one. It records each role's status and who
or what plays it. The shipped `agf/agents/MANIFEST.example.yaml` is only a template:
every entry starts as `skipped`, and `agov` reads only `MANIFEST.yaml`. Do not read
the template as your roster.

## Ask for a specialist during a change

Say what you need: "Ask for a security review of this plan." What happens next
depends on the kind of specialist:

- **An already active role.** The assistant consults it within its current scope.
  That is not a new activation.
- **A shipped role your project has not activated.** A request alone does not
  activate it or assign its player. The assistant proposes the role and says why,
  using the role's declared trigger, or its stated purpose when it declares none.
  The authorized human decides whether to activate the role and who or what plays
  it. The project records the player, the responsibilities, the limits and the
  review handoff in its governed record path, and in a manifest entry where the
  project keeps a manifest. The assistant then routes a bounded handoff to the
  specialist, and the findings return to the requester and the approver named in
  that handoff.
- **An informal adviser.** The [adoption guide, section 1.1](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/agf/ADOPTING.md) allows
  project-specific advisers who take part in planning without a YAML definition or
  framework tracking. They have no pipeline authority.

Specialist output is not always advice. A role can own work: `build-engineer`, for
example, owns CI configuration and build automation. A role can carry a named
requirement: the `web-ux-reviewer` definition sends accessibility findings rated
"Needs work" or "Poor" to the `governor-reviewer`, because WCAG AA findings block
under `ui/accessibility-rule.md`. Tool output does not become a mechanical gate
unless a named rule grants that authority. An advisory reviewer never approves a
gate. The authorized approver makes the gate decision, by default the human
governor, using the findings.

Designing a new role, or changing a role's authority, follows the approval process
in [customizing the framework](../advanced/customizing-the-framework.md).

## The two gates

The two gates sit at opposite ends of the same unit of work, not side by
side: Gate 1 reviews the plan before any code exists, and Gate 2 reviews
the diff after implementation but before merge. Figure 1 traces that
single path — plan in, implementation in the middle, merge at the end —
so the cost claim above (redirection at Gate 1 costs minutes; the same
redirection after implementation costs the implementation) is visible as
a shape, not just an assertion. The point to retain: there is exactly one
plan-review checkpoint and exactly one merge-review checkpoint per
change, and nothing skips between them.

![Request path through plan approval, implementation review, and merge](../../reference/figures/gate-pipeline.svg)

*AI-generated figure. Publication remains the responsibility of the human maintainer.*

**Gate 1 — plan approval.** Before implementation. The reviewer reads the
plan, not code. Redirection here costs minutes; the same redirection
after implementation costs the implementation. Gate 1 is waived by the
fast track for small, low-risk work (≤3 files, no shared interface, data
model, or security boundary change, no gate-mechanics change, no claim
conflict) — the waiver is recorded, not silent.

**Gate mechanics never take the fast track**, whatever the diff size. A
change to a gate, schema, validator, CI check, merge policy, or release
policy takes the full path, and — per the gate self-test rule — carries a
negative test proving the gate still fires after the change, not only when
it was first introduced. The reason is that at diff level a weakening and a
fix look the same; the surviving negative test is the only thing that tells
them apart. See `rules/workflow/simplicity-rule.md` and
`rules/always/gate-self-test-rule.md` for the governing text.

**Gate 2 — review before merge.** After implementation, before the merge
request lands. Evidence-based: tests green, checklist run, diff scoped to
the approved plan, visual inspection where the output is visible.
Findings are filed as review entries; approval is a decision record that
may carry conditions, and the merge must honour them.

## The coordination channel

Where Figure 1 showed the two checkpoints a change passes through, Figure
2 shows what accumulates between them: every claim, handoff, review,
decision, and completion is its own file, and each new file references
the ones it responds to rather than replacing them. Figure 2 draws that
referencing structure as a chain, not a list, because the order and the
pointers are what let a reviewer walk backward from a decision to the
evidence that justified it. The point to retain: nothing in the channel
is edited in place — corrections are new links added to the chain, which
is why the chain, not any single file, is the audit trail.

<div class="figure-scroll" markdown>
![Coordination message kinds grouped by their routing and authority effects](../../reference/figures/coordination-channel.svg)
</div>

*AI-generated figure. Publication remains the responsibility of the human maintainer.*

Every handoff, decision, and completion is a file:
`docs/coordination/<YYYYMMDDThhmmssZ-role-slug-suffix>.md`, YAML frontmatter
plus a short body. This is the current preferred convention. Earlier projects
used sequence numbers (`00001-sender.md`) or ULIDs; those records are
immutable once filed —
corrections are new entries that reference the old, never edits. Nine
message kinds cover the lifecycle: claim, handoff, review,
decision-record, completion, note, ack, fast-track-waiver,
lesson-proposal.

The channel is the audit trail. A reviewer reconstructing a decision
reads the chain: handoff → review → decision-record → completion, each
referencing its predecessor by filename. An action without an entry is a
gap — the coordination-message rule makes filing the entry part of the
action itself.

## Delegated AI work

Sometimes an assistant works under authority delegated by a human, team, or
service principal. AGF treats that as delegated agency, not impersonation. The
AI session and AGF role remain visible; the principal is recorded separately;
Gate 1 and Gate 2 approval still belong to the governor-reviewer.

In v2.x this is a docs-only convention. It uses existing `kind: note`
coordination messages. The optional `delegation` and `contestation` objects are
not semantically enforced by AGF tooling in v2.x, so adopters should treat them
as an audit convention rather than a validator-backed contract.

A delegation grant is written by the human or service authority that grants it.
An AI agent must not write a grant that asserts a human principal for itself.
AI-authored records that act under the grant reference the grant through
`references-to`; they do not repeat or infer the principal from git committer,
email, SSH key, or branch metadata.

```yaml
kind: note
from: governor-reviewer
to: coder-tester
id: 20260115T101500Z-governor-reviewer-ab12
timestamp: 2026-01-15T10:15:00Z
immutable: true
references-to: []
topic: delegation-grant-docs-branch
requires-ack: true
artifacts_owned: []
note_type: delegation-grant
delegation:
  principal_kind: human
  identifier_namespace: "gitlab.com/example-group/example-project"
  identifier: "@example-member"
  delegation_scope:
    - "prepare documentation patch"
    - "run documentation checks"
  valid_until: "2026-01-16T10:15:00Z"
  cannot_approve:
    - "Gate 1"
    - "Gate 2"
    - "release tag"
```

Minimum useful convention fields are `principal_kind`,
`identifier_namespace`, `identifier`, and `delegation_scope`. `valid_until` is
strongly recommended for every grant; open-ended delegation is high-risk
because it leaves old authority too easy to reuse. `cannot_approve` is
defense-in-depth wording, not the source of gate authority. These grants never
delegate Gate 1, release approval, or binding risk acceptance. A grant alone
never delegates a Gate 2 decision. The only delegated Gate 2 approval route
described here is the narrow opt-in deputy route summarized
[above](#approval-authority-and-optional-deputies) and defined in the
[deputy-governor rule](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/agf/rules/workflow/deputy-governor-rule.md).

If a principal disputes a grant or believes an AI exceeded the delegated scope,
file a later immutable `kind: note` with `note_type: contestation` and a
reference to the contested record. Do not edit the old message. The correction
belongs in the chain.

Delegation metadata improves accountability, but it also makes activity more
legible to anyone with repository access. Named identifiers, timestamps, commit
authorship, review comments, and merge events can be correlated. AGF does not
ship native productivity reports, principal ranking, review-depth scores, or
HR exports, and adopters should treat that boundary as intentional.

For privacy-sensitive teams, use identity minimization where audit precision
does not require a named person: team aliases such as `@docs-team`, rotating
duty aliases such as `@on-call-reviewer`, batch grants scoped to a sprint, and
documented retention limits. These are privacy-risk mitigations, not a
compliance checklist or a substitute for local privacy, labor, and legal
review.

## Why gates bind AI agents in particular

AI assistants optimise for task completion; under pressure they will
take the shortest path that satisfies the request. Written rules give
them the constraint set; the gates give the constraints teeth, because
the artifact the gate demands (a plan, a decision record, evidence)
either exists or does not. In the framework's own operation, the gates
have caught scope drift, branch collisions, and unverifiable claims that
prompting alone did not.

## Structural and advisory enforcement

Not every gate has equal enforcement strength. The distinction matters for
adopters deciding where to invest in automation.

A **structural gate** enforces regardless of agent cooperation — the check
runs outside the agent's control flow. Pre-commit hooks and CI pipeline jobs
are structural: the commit is refused or the merge is blocked before anyone
decides whether to cooperate.

An **advisory gate** relies on the agent reading and following the rule. The
coordination-message rule, the review checklist, and Gate 1 plan approval are
advisory: they are woven into the agent's briefing and enforced through review
and the immutable audit trail. An agent that skips an advisory gate leaves a
visible audit gap — not an automated block.

| Gate | Enforcement type |
|---|---|
| Pre-commit hook (linter, secret scan) | Structural — adopter configures |
| CI schema validation (`agov coord validate`) | Structural — see [contracts and validators](../advanced/contracts-and-validators.md) |
| Gate 1 plan approval | Advisory — governor verifies |
| Gate 2 review checklist | Advisory — agent self-checks, reviewer verifies |
| Gate 2 decision record | Advisory + social — governor must file before merge |

The rule files in `agf/rules/` are advisory by nature. What makes the advisory
layer effective is discipline, review, and the audit trail. What makes the
structural layer effective is automation. Add structural checks wherever the
automation cost is low; accept the advisory model where it is not — and make
the distinction visible so adopters know what to reinforce.

Advisory gates fail by leaving gaps, not by stopping the commit. Common failure
modes look like this:

| Failure mode | What happened | How to detect or recover |
|---|---|---|
| Skipped Gate 1 | Implementation started without an approved plan or fast-track waiver | Gate 2 reviewer asks for the plan or waiver before merge; if missing, file a review finding and reconstruct or reject the work |
| Silent scope drift | The diff no longer matches the approved plan | Gate 2 compares the diff against the Gate 1 plan and records the drift as a review finding |
| Missing decision record | A merge happened after informal approval in chat or memory | File a corrective coordination note; record the gap as learning if the pattern could repeat |
| Agent self-approval | The implementing agent writes text that looks like approval | Treat it as non-binding; governor-reviewer must file the actual decision record |

These are audit failures before they are tooling failures. Validators can check
message shape and some repository contracts, but the governor still has to ask:
"Does the evidence chain exist, and does it support this merge?"

## Next

- How evidence accumulates and becomes rules:
  [learning loop and evidence](evidence-and-learnings.md)
- Wiring validators as CI blockers: [contracts and validators](../advanced/contracts-and-validators.md)
- The gates in daily practice: [day-to-day operation](../operate/day-to-day.md)
