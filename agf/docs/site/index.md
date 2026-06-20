# agents-govern

**Governance for AI-assisted software development.**

agents-govern is an open framework for running AI coding agents under
explicit, auditable governance: defined roles with bounded authority,
review gates before implementation and before merge, an append-only
coordination channel (the decision log), and a learning system that turns
operational incidents into framework evidence.

> **Status:** This site documents **agents-govern v2.0.0, which is
> Provisional** — its structure and documentation are under evaluation and
> may change. The exact source commit for this build is recorded in
> [Release information](reference/release-info.md). Adopters who need a
> stable release should wait for v2.0 promotion. The v1.x series is the
> historical predecessor; v2.0 is the current development line.

## What AGF is, and is not

AGF is a governance framework for software-development work involving AI
agents. It gives a project file-based roles, gates, rules, schemas,
coordination records, and learning records so humans and AI assistants can work
inside auditable boundaries. It is not a replacement for your existing review
culture — it is the structure that makes that culture enforceable when agents
do the typing.

AGF is not a hosted service, agent runtime, model policy engine, legal or
compliance product, security audit, or general AI product-governance system. It
does not run agents, grant tool permissions, replace CI, approve its own gates,
or make external AI services safe for confidential data. Your existing tools
execute the work; AGF defines the governance artifacts and review discipline
around that work.

## The situation

Teams adopting AI coding assistants gain speed and lose certainty. Changes
arrive faster than review capacity; assistants complete tasks in ways that
bypass the team's quality conventions; and when something breaks, the trail of
*who decided what, and why* is thin. Most teams respond with prompt instructions
— "always write tests", "never push to main" — and discover that instructions
without enforcement degrade under pressure.

This is not a tooling failure. It is a governance gap: the assistant has
production-level write access and intern-level accountability.

## What you get

**Accountability structure for AI agents.** Each agent operates under a
written definition: what it owns, what it may change, what it must escalate.
The governor — the human who approves gates and owns project risk — can delegate
work without delegating final authority. An implementing agent cannot approve
its own work; a reviewing agent cannot implement. The separation is structural,
not aspirational.

**Decisions you can reconstruct.** Every plan approval, review finding, and
merge decision is a file in the coordination channel, the append-only decision
log. Six months later, you can answer "why was this merged?" by reading the
chain, not by archaeology on commit messages.

**Quality gates that bind assistants too.** The rule library expresses your
engineering discipline (test-first, migration hygiene, immutable tags,
merge-request workflow) in a form AI assistants load as instructions and
validators enforce in CI. Gate 1 is plan approval before implementation; Gate
2 is review before merge. The discipline survives the speed.

**Learning that compounds.** Incidents become learning records — structured
incident and pattern notes; recurring patterns become rules; rules ship to
every adopter. The framework grows from evidence, and the anonymization policy
means your learnings can flow upstream without revealing your project.

## What it costs

Adoption is not free, and the documentation does not pretend otherwise:
copied files in your repo, CI jobs, review overhead per change, and a
learning-record habit to maintain. The [downstream impact](docs/adopt/downstream-impact.md)
page itemises the full cost, and [exit and decommission](docs/adopt/exit-and-decommission.md)
documents the way out. Read both before adopting — that is what they are for.

## Who benefits

- **Engineering leads** get review gates that hold even when changes are
  machine-generated at machine speed.
- **Developers** get an assistant that follows the project's actual
  conventions instead of re-deriving its own each session.
- **Auditors and security reviewers** get an append-only decision trail
  and explicit security escalation boundaries.

## Where to go next

| Intent | Start here |
|---|---|
| Decide whether to adopt | [Should you adopt?](docs/start/should-you-adopt.md) |
| Understand cost and exit path | [Downstream impact](docs/adopt/downstream-impact.md) → [Exit and decommission](docs/adopt/exit-and-decommission.md) |
| Install and run one governed change | [Install the `agf/` directory](docs/adopt/install-and-layouts.md) → [Your first governed change](docs/start/first-governed-change.md) |
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
[Release information](reference/release-info.md). Framework content:
CC BY-SA 4.0, with separate commercial licensing available for alternative
terms. Tooling code: MIT.
