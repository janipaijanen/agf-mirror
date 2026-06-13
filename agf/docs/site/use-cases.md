# Use cases and non-use cases

*Status: Provisional (v2.0 documentation). Use cases below trace to
learning records in the framework's evidence base; codenames refer to
anonymized adopter projects.*

## Credible use cases

**Solo maintainer with AI assistants doing most implementation.**
The framework's own development (codename *foundry*) runs this way: a
human governor approves plans and merges; AI agents plan, implement, and
review under role definitions. The gates replace the review a second
human would provide.

**Small team adding AI assistants to an existing quality culture.**
Adopters in the evidence base (e.g. *silver-birch*, *copper-rapids*)
used the rule library to make existing conventions — migration hygiene,
test tiers, CI scoping — binding on assistants across Elixir and Python
application stacks. The value is consistency: the assistant follows the same
written rules in every session.

**Projects with audit or compliance pressure.**
The append-only coordination channel and audit-log rules give projects
under regulatory regimes a reconstructible decision trail for
machine-generated changes.

**Projects producing visible or generated artifacts.**
Adopters generating CAD drawings and documents (*blue-marlin*) used the
visual-verification and file-generator rules to catch defect classes that
unit tests structurally miss.

## Non-use cases

Be skeptical of adopting if any of these describe you:

- **Throwaway or exploratory code.** Governance overhead is waste when the
  artifact's lifespan is days. Use your assistant directly.
- **No AI assistants in the loop.** The framework's enforcement targets
  AI-assisted workflows; a human-only team gets more value from
  conventional code review and CI alone.
- **Need for turnkey enforcement.** The framework is files, rules, and
  conventions — not a hosted service. If nobody on the team will act as
  governor, the gates will not gate.
- **Expecting a runtime.** agents-govern does not execute agents; it
  governs them. Orchestration and execution come from your assistant
  tooling (Claude Code, Codex, or similar).
- **Very large organisations seeking org-wide policy.** The framework is
  validated on single projects with one governor. Multi-project,
  multi-governor federation is a [hypothesis](future/evolution.md), not a
  capability.

## Maturity, honestly

Per the framework's implementation-honesty discipline, capability claims
carry maturity levels. The single-project, single-governor workflow is the
validated core. Several rules are explicitly **Provisional** (single-adopter
evidence) and say so in their text. If a page on this site does not state a
maturity level for a claim, treat that as a documentation bug and report it.
