---
id: dogfood-authority-boundary
date: 2026-06-14
status: accepted
deciders: [governor-reviewer]
references:
  - docs/coordination/00287-planner-architect.md
  - docs/coordination/00289-planner-architect.md
  - docs/plans/tool-authority-boundary-rule.md
---

# ADR: Dogfooding Authority Boundary for AGF-NG

## Context

AGF-NG is a self-governing repository: the framework it builds (`agf/`) is
also used to govern the project that builds it. This creates a potential
circular authority risk. A validator that checks framework files could be
perceived as the source of authority over those files, rather than as evidence
about them.

The project uses three layers:

1. **`agf/` — adopter-facing product and normative framework.** Rules, agent
   definitions, schemas, and documentation that adopters copy or reference.
   This is what AGF-NG ships.

2. **`tooling/` — implementation layer.** Validators, packagers, build scripts,
   and supporting code that exercises and distributes the framework. Tooling
   serves the framework; it does not govern it.

3. **Repository — first adopter and evidence corpus.** AGF-NG itself adopts
   the framework it produces. Coordination entries, gate decisions, and
   learning records are the live evidence of governance in practice.

## Decision

AGF-NG adopts `agf/rules/always/tool-authority-boundary-rule.md` for its own
operations.

The governing principle:

> A tool may block work only when a named rule in the adopter's rule set
> explicitly grants that check blocking authority. All other tool output
> is advisory evidence.

This means:

- `agov coord validate` and `agov plan validate` carry **blocking authority**
  for the schema checks defined in `agf/schemas/`. The schemas define the
  contracts; `agf/rules/always/tool-authority-boundary-rule.md` grants
  validators permission to block work when those contracts are violated. The
  validators surface schema violations as `severity: blocking`.

- Findings without a named backing rule are **advisory** (`severity: advisory`).
  They surface for role review but do not halt work autonomously.

- AI runtimes, evaluators, MCP servers, and local models produce **advisory
  evidence** regardless of model class, speed, or confidence score. A
  high-confidence model output becomes binding only when a named role accepts
  it through a documented gate.

## Why this rule belongs in `agf/rules/always/` rather than only `AGENTS.md`

`AGENTS.md` is a per-session instruction surface for AI coding assistants.
It is the right place for short operational reminders. It is the wrong place
for a normative rule that:

- must survive across sessions and model handoffs,
- must be adoptable by downstream projects as part of the framework,
- must be auditable as a framework artifact with its own evidence chain.

A rule in `agf/rules/always/` is a durable, schema-validated, provenance-tracked
artifact. `AGENTS.md` references it; it does not replace it.

## How this interacts with high-clockspeed deliberation

AGF-NG sessions can generate many coordination entries and advisory findings
quickly. The synthesis model:

- Agents and validators create evidence (advisory by default).
- Synthesis (coordination entries, plan files, review findings) lifts durable
  outputs from transient deliberation.
- Governor-reviewer decisions at Gate 1 and Gate 2 remain binding and cannot
  be substituted by tool volume.

The tool-authority-boundary-rule is the formal statement of this model. Without
it, a future automation author might promote a validator finding to blocking
without realizing they have changed the governance model.

## Why circular authority must be avoided

If `tooling/agov coord validate` were permitted to block merges without a named
rule granting that authority, then:

1. Someone could add a new validator check.
2. That check would immediately block all future merges.
3. The framework schema would be governed by the tooling that reads it, not by
   the framework rules and role decisions that define it.

This is the circular case. The boundary rule prevents it by requiring a named
rule before any blocking authority is exercised.

## Consequences

- Every validator that blocks work MUST name the rule granting that authority.
  Currently: `agov coord validate` and `agov plan validate` cite
  `agf/rules/always/tool-authority-boundary-rule.md` as the blocking authority
  rule and validate the contracts in
  `agf/schemas/coordination-message.schema.json` and
  `agf/schemas/gate1-plan.schema.json` respectively.
- New automated checks default to advisory until a rule update promotes them.
- `AGENTS.md` carries a one-line reminder for agents adding automation.
- This ADR is the citable record for the decision; coordination entries
  seq 287 and seq 289 are the deliberation record.
