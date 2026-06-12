# Downstream Impact: Assess Before Committing to Framework Artifacts
#
# Framework principle: Propagation-aware governance
# See: framework.md — Principle 5 (Escalation over Improvisation)
# Theoretical grounding: Lehman (1980), Swanson (1976) / IEEE 14764-2006,
#   Parnas (1972), Šimon & Buhnova (2024)

<!-- CUSTOMIZE: This rule applies only to projects that distribute governance
artifacts to downstream users — framework forks, governance-as-a-product
projects, or projects that publish a rules-template or agent library for
others to adopt. A standard adopter (web app, CLI tool, data pipeline) using
this framework for its own governance should omit this rule: their
.claude/rules/ is their own project governance, not something they
distribute. -->

**Scope:** Any commit that touches one or more of these trigger paths:

<!-- CUSTOMIZE: Replace with your project's distributable artifact paths. -->
- `.claude/rules/**/*.md`
- `spec/rules-template/**/*.md`
- `agents-govern/agents/*.yaml`
- `agents-govern/src/agents_govern/`
- `spec/schemas/*.json`
- `docs/coordination/agov-config.yaml`
- `AGENTS.md`

A commit touching ONLY coordination channel entries, plan files, learning
records, or documentation outside these paths is exempt.

## Impact classification

Before proposing any non-exempt commit, classify it as one of:

| Classification | IEEE 14764 equivalent | Meaning | Required action |
|---|---|---|---|
| **BREAKING** | Corrective / Adaptive | Observable behavioral contract changes; adopters get different agent behavior silently | Write migration note before proposing the commit |
| **ADDITIVE** | Perfective / Adaptive | New capability, new optional file, new field; no existing contract removed | CHANGELOG entry recommended; no migration needed |
| **EDITORIAL** | Perfective | Wording, examples, formatting; observable behavioral contract unchanged (Parnas: implementation, not interface) | One-line in commit message: `editorial: [description]` |
| **PREVENTIVE** | Preventive | Proactive hardening; makes implicit constraint explicit | Treat as ADDITIVE unless it restricts previously-permitted behavior (then BREAKING) |

A commit containing both a breaking and an additive change is BREAKING.

The EDITORIAL boundary (Parnas): a change is editorial if and only if the
set of actions the rule *requires*, *forbids*, or *permits* is unchanged.
Adding a clarifying example without a new obligation is editorial. Narrowing
a previously implicit permission is BREAKING.

## Three-circle impact statement

For BREAKING, ADDITIVE, and PREVENTIVE changes, include this block in the
commit message or plan file:

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

<!-- CUSTOMIZE: Rename the three circles to match your project's propagation
model. A two-tier project (your project → your users) needs only two circles.
A four-tier project needs four. The invariant is: every tier that receives
your distributed artifacts must have an explicit entry. -->

For EDITORIAL changes, substitute the full statement with a single line:
`editorial: [description]`

## STOP condition

If the classification is BREAKING and no migration note exists: **do not
propose the commit**. Write the migration note first. It may live in the
commit message, a plan file, or `CHANGELOG.md`.

The Evidence field enforces the verifiable-claims-rule: a breaking change
that cannot cite the incident or learning that motivated it is not ready
to ship.
