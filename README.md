# AGF-NG

AGF-NG is the next-generation agents-govern framework: a file-based governance
system for AI-assisted software development.

It gives AI coding agents explicit roles, bounded authority, two review gates,
an append-only coordination channel, and structured learning records. The aim
is simple: keep AI-assisted changes fast while making decisions, handoffs, and
risk acceptance reconstructible from files.

Published docs: <https://janipaijanen.gitlab.io/agents-govern-framework/>

## Should you try it?

Try AGF when:

- AI assistants write or review a meaningful share of your code.
- A human can own the governor role and approve plans and merges.
- Your project will live long enough for an audit trail to matter.
- Prompt-only discipline has not kept assistants consistently inside your
  quality, security, or review boundaries.

Wait or skip when:

- you need a hosted runtime or agent orchestrator;
- your project is disposable or exploratory;
- nobody can own gate decisions;
- you need v2.0 stability today. AGF v2.0 is still provisional.

## Repository map

| Path | Meaning |
|---|---|
| `agf/` | The adopter-facing AGF v2 product: roles, rules, schemas, docs, templates |
| `tooling/` | Implementation tooling for validation, build, release, and conformance checks |
| `docs/` | This repository's own governed work: plans, coordination, proposals |
| `agents-govern/` | Legacy/bootstrap v1.x reference used to govern this repo; not the v2 product |
| `AGENTS.md` | Contributor and agent orientation for this repository |

For product evaluation, start in `agf/`, not `agents-govern/`.

`tooling/` helps exercise the framework, but it is not the source of
governance authority. The normative files live in `agf/` and the binding
decisions come from the roles and gates recorded in the coordination channel.
Tool output is evidence; it becomes blocking only when a named AGF rule or
governor decision makes that check blocking.

## Five-minute path

1. Read [Should you adopt?](agf/docs/site/docs/start/should-you-adopt.md).
2. Check cost and reversibility:
   [Downstream impact](agf/docs/site/docs/adopt/downstream-impact.md) and
   [Exit and decommission](agf/docs/site/docs/adopt/exit-and-decommission.md).
3. Try the first-run path:
   [Install the `agf/` directory](agf/docs/site/docs/adopt/install-and-layouts.md).
4. Run one valid example:
   [Your first governed change](agf/docs/site/docs/start/first-governed-change.md).

The first-run templates live in [agf/templates](agf/templates/).

## What a trial adds to your project

Default POC layout:

| Artifact | Default path |
|---|---|
| AGF distribution | `agf/` |
| Assistant rules | `.claude/rules/` or your tool's equivalent |
| Working plans | `docs/plans/` |
| Coordination channel | `docs/coordination/` |
| Learning record | `agf/learnings/<codename>.yaml` |
| Current core validators | `agov` from a full checkout |

The framework source archive is usable on its own. Adopters who want the
current core validators can use a full checkout or the optional packaged
`agov` tooling archive; the combined bundle is a convenience artifact, not a
prerequisite for adopting AGF.

## How to cite

If you use agents-govern in research or a published project:

**APA 7th edition:**

> Päijänen, J. (2026). *agents-govern: Governance for AI-assisted software
> development* [Computer software]. https://gitlab.com/janipaijanen/agents-govern-framework

For version-specific citations, see the `reference/release-info/` page on the
[published docs site](https://janipaijanen.gitlab.io/agents-govern-framework/)
for the APA and BibTeX entry with the exact version number.

## Safety and independence

Only run AI agents and tooling on systems, repositories, and data where you
have the necessary authorization. AGF guidance is not legal, security, or
compliance advice; it is a governance framework you must adapt to your own
context.

AGF-NG is developed independently. The project has not received external
funding from, and has no dependency or endorsement relationship with, the
companies or brands whose tools, models, or runtimes may be discussed in the
documentation.
