# Customizing the framework

*Status: Released (v2.1 documentation). Audience: administrators and
governors adapting the framework to a project. After this page you know
the three customization zones — free, marked, and contracted — and the
costs of crossing them.*

!!! note "AGF v2 layout"
    Framework-owned files live under `agf/`; adopter-owned evidence lives in
    root `docs/` by default. Keep that boundary visible when customizing.

## The three zones

**Free to change.** Your codename's learning records, your project's
coordination entries, plan files, rules you wrote yourself, rules you
deleted as inapplicable. These are adopter-owned; upgrades never touch
them, and upstream makes no assumptions about their content beyond
schema validity where schemas apply.

**Marked for customization.** Rules and configuration carrying
`CUSTOMIZE` markers: locale sets, migration commands, CI path patterns,
audit function names, target-consumer lists for generated files,
delegable categories in the deputy-governor rule (which ship
deliberately empty). Customizing these is not deviation — it is the
intended completion of the install. An uncustomized `CUSTOMIZE` block is
the defect, not the edit.

**Contracted.** Schema shapes, validator semantics, the
upstream-owned/adopter-owned boundary, role authority separations, the
red list of never-delegable categories. You *can* edit these — it is
your repository — but you forfeit the upgrade path for the edited files
and, for authority boundaries, the structural guarantees the framework
exists to provide. An implementer that can approve its own merges is not
a customization; it is the absence of the framework with its files still
present.

| Zone | Examples | Upgrade effect | Review posture |
|---|---|---|---|
| Free | Local coordination records, adopter learnings, local-only rules, deleted inapplicable rules | Preserved | Normal project review |
| Marked | `CUSTOMIZE` blocks, local command names, locale sets, path patterns, empty delegable categories | Preserved if kept in adopter-owned space; may need re-check after upstream updates | Expected completion work |
| Contracted | Schemas, validator exit semantics, authority boundaries, upstream/adopter ownership boundary | May lose compatibility with upstream updates | Treat as a governed fork or upstream contribution candidate |

## Customizations that strengthen

- **Adding project-domain rules.** Stack-specific rules alongside the
  shipped library (your migration tool, your audit schema, your CI) are
  the most valuable customization; the shipped
  `project-specific-examples/` directory exists as a starting pattern.
- **Tightening defaults.** Lower severity ceilings, shorter delegation
  expiries, stricter fast-track criteria — tightening never breaks the
  contract.
- **Adding roles.** New agent definitions for project-specific concerns
  (a data-quality reviewer, a domain modeller) extend the pipeline; the
  framework's evidence base includes adopter-originated roles that later
  surfaced gaps no shipped role covered.

## Customizations that weaken

- **Loosening gates without evidence.** Widening fast-track criteria or
  delegable categories because ceremony feels heavy inverts the evidence
  discipline: the framework loosens only on corroborated learning, and
  so should your instance of it.
- **Forking contracted files silently.** If a contracted surface does
  not fit, file the learning upstream — the contract changes on
  evidence, and your case may be the corroboration a pending change
  needs. A silent fork gets you a one-version framework nobody else
  maintains.
- **Keeping rules you do not enforce.** A rule visibly ignored teaches
  agents (and people) that rules are decorative. Delete what you will
  not enforce; deletion is honest.

## Sending customizations upstream

Generalizable rules, learning records, and gap reports flow upstream
under the anonymization rule: codename only, domain-neutral generics for
anything revealing. The contribution path is the same learning-record
mechanism the framework uses internally — see
[learning loop and evidence](../concepts/evidence-and-learnings.md).

## Next

- Where the framework itself may go: [evolution](../../future/evolution.md)
