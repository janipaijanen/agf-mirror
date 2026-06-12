# Value proposition

*Status: Provisional (v2.0 documentation). For canonical status see the
[landing page](index.md).*

## The situation

Teams adopting AI coding assistants gain speed and lose certainty. Changes
arrive faster than review capacity; assistants complete tasks in ways that
bypass the team's quality conventions; and when something breaks, the trail
of *who decided what, and why* is thin. Most teams respond with prompt
instructions — "always write tests", "never push to main" — and discover
that instructions without enforcement degrade under pressure.

This is not a tooling failure. It is a governance gap: the assistant has
production-level write access and intern-level accountability.

## What agents-govern enables

**Accountability structure for AI agents.** Each agent operates under a
written definition: what it owns, what it may change, what it must escalate.
An implementing agent cannot approve its own work; a reviewing agent cannot
implement. The separation is structural, not aspirational.

**Decisions you can reconstruct.** Every plan approval, review finding, and
merge decision is a file in an append-only coordination channel. Six months
later, you can answer "why was this merged?" by reading the chain, not by
archaeology on commit messages.

**Quality gates that bind assistants too.** The rule library expresses your
engineering discipline (test-first, migration hygiene, immutable tags,
merge-request workflow) in a form AI assistants load as instructions and
validators enforce in CI. The discipline survives the speed.

**Learning that compounds.** Incidents become structured learning records;
recurring patterns become rules; rules ship to every adopter. The framework
grows from evidence, and the anonymization policy means your learnings can
flow upstream without revealing your project.

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

The framework occupies a focused niche: governance for AI-assisted
development workflows. It is not a CI system, not an agent runtime, and
not a replacement for your existing review culture — it is the structure
that makes that culture enforceable when agents do the typing.
