# Roles and gates

*Status: Provisional (v2.0 documentation). Audience: developers and
governors building the mental model. After this page you can name the
roles, say what each gate decides, and read a coordination chain.*

!!! note "AGF v2 layout"
    Role definitions ship under `agf/agents/`. Gate evidence is recorded in the
    adopter-owned coordination and plan paths, `docs/coordination/` and
    `docs/plans/` by default.

## The core idea: separated authority

No agent both implements and approves its own work. Authority is split
across role definitions, each a YAML contract stating jurisdiction (what
the role owns), authority (what it may decide), and escalation (what it
must hand upward). The separation is structural: an implementing agent
that "approves" its own merge has acted outside its definition, and the
audit trail shows it.

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
[quick reference](https://gitlab.com/janipaijanen/agent-governance)
(`docs/reference/quick-reference.md` in the distribution) lists all
twelve with one-line responsibilities. The **human governor** sits above
all of them: every agent authority is delegated, revocable, and bounded
by the red list in the deputy-governor rule (security boundaries,
destructive operations, and meta-governance are never delegable).

## The two gates

Figure 1 shows the two review gates as a pipeline: planning is reviewed before
implementation starts, and implementation is reviewed again before merge.

![Gate pipeline](../../reference/figures/gate-pipeline.svg)

**Gate 1 — plan approval.** Before implementation. The reviewer reads the
plan, not code. Redirection here costs minutes; the same redirection
after implementation costs the implementation. Gate 1 is waived by the
fast track for small, low-risk work (≤3 files, no shared interface, data
model, or security boundary change, no claim conflict) — the waiver is
recorded, not silent.

**Gate 2 — review before merge.** After implementation, before the merge
request lands. Evidence-based: tests green, checklist run, diff scoped to
the approved plan, visual inspection where the output is visible.
Findings are filed as review entries; approval is a decision record that
may carry conditions, and the merge must honour them.

## The coordination channel

Figure 2 shows the coordination channel as an append-only chain of claims,
handoffs, reviews, decisions, and completions. Each file points to earlier
files so the decision remains reconstructible.

![Coordination channel](../../reference/figures/coordination-channel.svg)

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

## Next

- How evidence accumulates and becomes rules:
  [evidence and learnings](evidence-and-learnings.md)
- Wiring validators as CI blockers: [contracts and validators](../advanced/contracts-and-validators.md)
- The gates in daily practice: [day-to-day operation](../operate/day-to-day.md)
