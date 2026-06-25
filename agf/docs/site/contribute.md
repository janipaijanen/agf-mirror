<!--
SPDX-License-Identifier: CC-BY-SA-4.0
-->
# Contribute

*Audience: adopters who have found something worth sharing — a gap, a
validation, a tension, a pattern that worked, or a pattern that didn't.*

The framework grows by accumulating evidence from real projects. This page
describes the three paths for feeding evidence back and what each one
requires.

## Three paths

### 1. Submit a learning

The lightest path. Use this when you have a finding from your project:
a gap the framework didn't cover, a validation that a rule works as
described, a tension between two rules, an adaptation you made, or a part
of the framework that turned out to be unnecessary overhead.

**How:** File an issue on the [framework issue tracker](https://gitlab.com/janipaijanen/agents-govern-framework/-/issues)
with the label `learning`. Include:

- A brief description of what you observed (one to three sentences).
- The context: which rule, agent, or pattern does it relate to?
- The outcome: did it help, fail, need adaptation, or prove unnecessary?

Single-adopter evidence is acceptable. The framework maintainer will
integrate it as a learning record entry and credit the codename.

### 2. Propose a rule

Use this when a pattern in your project would benefit other adopters and
is general enough to belong in the shared rules template.

**How:** Open an issue on the framework tracker with the label `rule-proposal`.
The proposal should include:

- The rule body, written as a markdown file following the existing
  `rules-template/` style.
- `<!-- CUSTOMIZE -->` markers for anything adopters need to adapt to
  their own stack or project.
- The evidence that motivated it — what happened, in which project
  (codename), at what point in adoption.
- A self-assessed status: **Provisional** (single-adopter evidence) or
  **Recommended** (2+ adopter corroboration). Default to Provisional.
- A suggested target directory: `always/`, `workflow/`, `ui/`, or `docs/`.

Provisional rules ship with a prominent status header. Promotion to
Recommended requires independent corroboration from at least one other
adopter.

### 3. Propose an agent

The highest bar. Agent definitions carry more long-term cost than rules:
they define a role's jurisdiction, communication channels, and authority
boundaries, which downstream adopters build workflow assumptions on top of.

**Requirements before filing a proposal:**

- 2+ independent adopter projects have encountered the pattern the
  agent addresses.
- The pattern cannot be adequately handled by a rule alone (you've
  tried, or can explain why a rule is insufficient).
- You have a draft agent YAML following the `agf/agents/` format.

If your evidence is single-adopter, propose a rule first. The agent can
follow once the rule's enforcement turns out to be insufficient for the
pattern.

## Anonymization requirement

All submissions use a **codename**, never a real project name, product
name, organisation name, or domain descriptor. You keep a private
mapping; the public corpus contains only codenames.

When drafting external content (issue bodies, MR descriptions, learning
records), treat the prompt itself as external: anonymize before writing,
not after. See [Evidence and learnings](docs/concepts/evidence-and-learnings.md)
for the full anonymization policy.

The framework maintainer screens every submission for leaks (paths,
internal hostnames, customer names, cumulative-detail signatures) before
integrating it.

## Independent adopters wanted

The framework's strongest current weakness is single-developer evidence.
Most adopter projects in the corpus involve the same human Governor-Reviewer
who designed the framework. Independent adopters — teams and individuals who
did not build the framework — are explicitly wanted.

Evidence gaps where independent corroboration is most valuable:

- **Overkill findings** — has any part of the framework added overhead
  without delivering value? Zero overkill findings across all current
  projects is suspicious; single-developer bias cannot be ruled out.
- **Multi-person team dynamics** — does the Governor-Reviewer role work
  when it is a different person from the Coder-Tester?
- **Non-Claude-Code tools** — does the rule and agent content work with
  Cursor, Windsurf, Codex CLI, or other AI coding assistants?
- **Phase 2+ role splits** — has anyone needed to split the combined
  Planner-Architect or Coder-Tester roles into separate agents? What
  triggered it?

If any of these match your experience, a learning submission is the most
valuable thing you can contribute.

## Next

- How evidence is structured: [Evidence and learnings](docs/concepts/evidence-and-learnings.md)
- Install the framework: [Install the `agf/` directory](docs/adopt/install-and-layouts.md)
- Framework issue tracker: [gitlab.com/janipaijanen/agents-govern-framework](https://gitlab.com/janipaijanen/agents-govern-framework/-/issues)
