---
id: downstream-impact-rule
status: pending-gate1
topic: framework-governance
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: no security boundaries, auth, secrets, or credential-adjacent content
  - role: contribution-auditor
    status: waived
    reason: foundry-originated governance addition; no external contribution pattern to audit
---

# Plan: downstream-impact-rule — Impact assessment for framework artifact changes

**Date:** 2026-06-11T20:14:22Z
**Status:** PENDING GATE 1 APPROVAL
**Priority:** Tier 3 — Framework Maintainability

---

## Intent

When the foundry changes a rule, an agent definition, a schema, or a
framework document, that change propagates to every downstream project that
has adopted the framework. The agent proposing the change can reason about
whether the change is correct in isolation without ever asking: "what does
this mean for someone who already adopted this?"

Currently no rule forces that question. The result is that behavioral
changes can ship without a migration note, additive changes go unannounced,
and the downstream perspective is considered only when someone remembers.

This plan adds a rule that fires before any commit touching framework
artifacts, requiring an explicit three-circle impact statement:

1. **Foundry (self)** — does this change break the foundry's current setup?
2. **Direct adopters** — is the change breaking / additive / editorial for
   a project that has already copied these artifacts?
3. **Adopters' downstream** — does this change alter the artifacts that
   adopters produce, or the behavior their agents exhibit toward their own
   users and stakeholders?

Rules and schemas are first-class artifacts in this analysis. A change to
`workflow/tdd-rule.md` propagates to every AI coding session in every
adopting project. A schema change alters what every adopter's validator
accepts. Both require the same explicit downstream thinking as an agent
YAML or framework.md change.

---

## Affected files

| File | Action |
|---|---|
| `.claude/rules/always/downstream-impact-rule.md` | New — foundry working rule (active immediately) |
| `spec/rules-template/always/downstream-impact-rule.md` | New — distributable template with CUSTOMIZE scope note |

---

## Theoretical grounding

The rule formalises three well-established software-engineering principles
applied to governance artifact propagation:

**Lehman's Laws of Software Evolution (Lehman, 1980):** The Law of
Continuing Change holds that any system used in a real-world environment
must continually be adapted, or it becomes progressively less satisfactory.
The Law of Increasing Complexity holds that as a system evolves, its
complexity increases unless work is done to maintain or reduce it. Applied
here: framework rules will inevitably change; without systematic downstream
impact analysis, accumulated untracked changes silently erode the adoption
contract for every project that copied an earlier version.

**Swanson's maintenance taxonomy (Swanson, 1976; extended to four types by
IEEE 14764-2006):** Software maintenance divides into corrective (fixing
faults), adaptive (adapting to environment changes), perfective (improving
performance or maintainability), and preventive (preventing future faults).
Our BREAKING / ADDITIVE / EDITORIAL / PREVENTIVE classification maps
directly onto this established taxonomy, giving reviewers a shared
vocabulary grounded in existing software engineering practice.

**Parnas's interface/implementation distinction (Parnas, 1972):** A change
is editorial — no behavioral contract change — only if the rule's
*observable behavioral contract* (what the rule requires agents to do or
not do) is unchanged. Improvements to examples, clarification of wording,
and formatting are editorial. Adding, removing, or narrowing an obligation
is not.

**Quality gates (Šimon & Buhnova, 2024):** A quality gate is an explicit,
documented criterion that work must satisfy before advancing. The STOP
condition in this rule (BREAKING without a migration note blocks the
commit) is a quality gate in the formal sense — it prevents propagation
of undocumented breaking changes into the distribution channel.

---

## Rule design

### Trigger paths

The rule fires on any commit that touches:

- `.claude/rules/**/*.md` — working rule set (all categories)
- `spec/rules-template/**/*.md` — distributable rule templates
- `agents-govern/agents/*.yaml` — agent definitions
- `agents-govern/src/agents_govern/` — validator and tooling (behavioral)
- `spec/schemas/*.json` — schema definitions
- `docs/coordination/agov-config.yaml` — coordination configuration
- `AGENTS.md` — project orientation document

A commit that touches ONLY coordination channel entries, plan files,
learning records, or documentation outside the trigger paths is exempt.

### Impact classification

Every non-exempt commit produces one of four classifications before the
commit is proposed. The four types align with the IEEE 14764-2006
maintenance taxonomy:

| Classification | IEEE 14764 equivalent | Meaning | Required action |
|---|---|---|---|
| **BREAKING** | Corrective / Adaptive | Observable behavioral contract changes; adopters get different agent behavior silently | Write migration note before proposing the commit |
| **ADDITIVE** | Perfective / Adaptive | New capability, new optional file, new field; no existing contract removed | CHANGELOG entry recommended; no migration needed |
| **EDITORIAL** | Perfective | Wording, examples, formatting; observable behavioral contract unchanged (Parnas: implementation, not interface) | One-line statement in commit message: `editorial: [description]` |
| **PREVENTIVE** | Preventive | Proactive hardening against foreseeable drift; makes an implicit constraint explicit | Treat as ADDITIVE unless it restricts previously-permitted behavior (then BREAKING) |

A commit that contains BOTH a breaking change and an additive change is
classified as BREAKING.

The EDITORIAL boundary (Parnas): a change is editorial if and only if the
set of actions the rule *requires*, *forbids*, or *permits* is unchanged.
Adding a clarifying example that does not introduce a new obligation is
editorial. Narrowing a previously implicit permission is BREAKING.

### Three-circle statement format

For BREAKING, ADDITIVE, and PREVENTIVE changes, the impact statement is
part of the commit message or plan file (for changes covered by an
existing plan):

```
Impact:
  Foundry:          [does this break our own current setup? yes/no + detail]
  Direct adopters:  [BREAKING / ADDITIVE / PREVENTIVE — what they must or may do]
  Downstream:       [does adopter-produced artifact shape change? does agent
                     behavior toward adopter's users change?]
  Migration:        [required if BREAKING — what adopters must do]
  Evidence:         [for BREAKING: learning record ID or incident that motivated
                     the change — per verifiable-claims-rule]
```

For EDITORIAL changes, the one-line fast-track in the commit message
(`editorial: [description]`) substitutes for the full statement.

The Evidence field enforces the framework's existing verifiable-claims
discipline: a breaking change that cannot cite the incident or learning
that motivated it is not yet ready to ship.

### STOP condition

If the classification is BREAKING and no migration note exists: do not
propose the commit. Write the migration note first. The migration note
may live in:

- The commit message (for small, self-contained breaking changes)
- A plan file (if the breaking change is part of a planned task)
- `CHANGELOG.md` (for release-bound breaking changes)

### Scope note for the distributable template

The downstream-impact rule is only applicable to projects that themselves
distribute governance artifacts to downstream users: framework forks,
governance-as-a-product projects, projects that publish a rules-template
or agent library for others to adopt.

For a standard adopter (a web app, a CLI tool, a data pipeline) using
AGF-NG, this rule does not fire because their `.claude/rules/` is their
own project governance, not something they distribute. The CUSTOMIZE block
in the distributable template makes this explicit.

---

## Implementation steps

**Step 1** — Write `.claude/rules/always/downstream-impact-rule.md` with
the trigger paths, classification table, three-circle statement format,
STOP condition, and examples.

**Step 2** — Write `spec/rules-template/always/downstream-impact-rule.md`
with the same content plus a CUSTOMIZE block at the top stating:
`<!-- CUSTOMIZE: this rule applies only to projects that distribute
governance artifacts downstream. Standard adopters should omit it. -->`

**Step 3** — Self-apply the new rule to the commit that introduces it:
- Classification: ADDITIVE (new rule, no existing behavior changed)
- Foundry: no breakage (rule didn't exist before; nothing relied on its absence)
- Direct adopters: opt-in (they copy spec/rules-template/ when they choose to)
- Downstream: no change (new optional file)

---

## Verification

1. Both files exist with consistent content.
2. Rule correctly distinguishes the three trigger path categories.
3. Rule does not overlap with existing always/ rules.
4. CUSTOMIZE block in spec/rules-template/ version makes scope explicit.

---

## Risks

| Risk | Mitigation |
|---|---|
| Rule too broad — fires on every minor edit | EDITORIAL fast-track provides low-friction path for wording changes |
| Rule scope unclear (which projects must follow it) | CUSTOMIZE block explicitly scopes to framework-distributing projects |
| Three-circle analysis becomes formulaic | Rule requires specific content per circle, not just checkbox acknowledgement |
