# agents-govern

**Governance for AI-assisted software development.**

agents-govern is an open framework for running AI coding agents under
explicit, auditable governance: defined roles with bounded authority,
review gates before implementation and before merge, an append-only
coordination channel (the decision log), and a learning system that turns
operational incidents into framework evidence.

> **Status:** agents-govern **v1.x is canonical**. This site documents
> **v2.0.0, which is Provisional** — its structure and documentation are
> under evaluation and may change. The exact source commit for this build
> is recorded in [Release information](reference/release-info.md).
> Adopters who need stability today should adopt v1.x.

## Why this exists

AI coding assistants optimise for task completion. Without structural
gates they will push to main, skip review, and accumulate ungoverned
changes — not from malice, but because nothing stops them. agents-govern
adds the stopping structure: the same quality gates, audit trails, and
escalation paths that disciplined human teams use, expressed so that both
humans and AI agents follow them.

## What you get

- **Defined roles** — twelve agent definitions (planner, implementer,
  reviewer, auditor, security sentinel, and more) with explicit
  jurisdiction and authority boundaries. Role expansion is supported,
  but evidence-led: add a role when repeated work, incidents, or
  ownership conflicts show that an existing boundary is too broad.
- **Two gates** — plan approval before implementation (Gate 1, the
  "should we build this?" checkpoint) and review before merge (Gate 2, the
  "is this ready to land?" checkpoint), with a formal fast-track for small
  tasks.
- **A coordination channel** — an append-only, schema-shaped decision log
  that makes every handoff, decision, and completion reconstructible.
- **A rule library** — thirty-plus governance rules (TDD, immutable tags,
  merge-request workflow, security hardening, …) ready to copy into your
  AI assistant's configuration.
- **Evidence discipline** — learning records, the structured incident and
  pattern notes, with anonymization built in, so what you learn can flow
  upstream without leaking what you build.

## Where to go next

| Intent | Start here |
|---|---|
| Decide whether to adopt | [Should you adopt?](docs/start/should-you-adopt.md) |
| Understand cost and exit path | [Downstream impact](docs/adopt/downstream-impact.md) → [Exit and decommission](docs/adopt/exit-and-decommission.md) |
| Install and run one governed change | [Install and layouts](docs/adopt/install-and-layouts.md) → [Your first governed change](docs/start/first-governed-change.md) |
| Operate a project day to day | [Day-to-day operation](docs/operate/day-to-day.md) |
| Follow the full learning path | [Documentation index](docs/index.md) |

## Evidence, honestly stated

The framework's evidence base is its [learning records](docs/concepts/evidence-and-learnings.md):
operational incidents and validated patterns from the framework's own
development and from early adopters, recorded under codenames. Rules
shipped on single-adopter evidence are explicitly marked **Provisional**
in the rule text itself. We state maturity levels rather than claiming
capabilities — see the [evolution page](future/evolution.md) for what is
shipped versus hypothesised.

Version, source commit, copyright, and license details are listed in
[Release information](reference/release-info.md). agents-govern framework
content is licensed CC BY-SA 4.0; software tooling code is licensed MIT.
