# agents-govern

*The framework that cannot govern itself has no business governing anything else.*

AI coding agents can produce changes you cannot reconstruct later.
agents-govern makes AI-assisted software work auditable without adding a hosted
service or runtime.

Your assistant can plan, patch, and test. Six months later, git history alone
often cannot answer why a change was requested, who accepted the risk, or what
the AI was supposed to do. agents-govern adds three file-based controls on top
of your existing repository:

- **Roles:** AI agents operate under named responsibilities; an agent cannot
  approve its own work, and a human governor holds gate authority.
- **Two gates:** Gate 1 approves a plan before implementation. Gate 2 approves
  the implementation before merge. CI validators provide evidence; the governor
  decides.
- **Decision log:** Plans, reviews, approvals, and handoffs are files in the
  repository, so later readers can reconstruct the chain of decisions.

No new accounts. No hosted service. No runtime. The framework is files in your
repo, instructions your AI assistant reads, and optional validators your CI can
run.

```text
AI writes a plan -> Governor approves -> AI implements + CI validates -> Merge
       Gate 1                                      Gate 2
```

Typical cost: an afternoon to install, plus about 5-15 minutes of governor
attention for a non-trivial change. Exit is documented: pause the gates, keep
only the rules you still want, or remove the AGF files in an ordinary cleanup
commit for a small default install.

Published docs: <https://janipaijanen.gitlab.io/agents-govern-framework/>

## Should you adopt?

Start with [Should you adopt?](agf/docs/site/docs/start/should-you-adopt.md).
It is the five-minute filter for deciding whether AGF is useful for your
project now.

Try AGF when:

- AI assistants write or review a meaningful share of your code.
- A human can serve as governor (approving plans before implementation and reviews before merge).
- Your project will live long enough for an audit trail to matter.
- Prompt-only discipline has not kept assistants consistently inside your
  quality, security, or review boundaries.

Wait or skip when:

- you need a hosted runtime or agent orchestrator;
- your project is disposable or exploratory;
- nobody can own gate decisions;
- you need every rule and extension point to be backed by multi-project evidence today.

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

## Maturity and evidence labels

AGF uses explicit labels so public claims do not overstate what exists or how
well it is proven.

Capability maturity answers "what exists?": Level 1 Planned, Level 2
Surface-defined, Level 3 Instantiated, Level 4 Validated, and Level 5
Deployable. A command implemented and tested in a branch is Level 4; it is not
Level 5 for an operator path or release until that selected binary includes it
and the documentation matches.

Evidence status answers "how strong is the proof?": Empirically validated,
Empirically informed, or Theoretical. Use both labels when a claim could
otherwise sound stronger than the evidence supports. See
[Maturity vocabulary](agf/docs/site/reference/maturity-vocabulary.md).

## Twenty-minute path

1. Read [Should you adopt?](agf/docs/site/docs/start/should-you-adopt.md).
2. Check cost and reversibility:
   [Downstream impact](agf/docs/site/docs/adopt/downstream-impact.md) and
   [Exit and decommission](agf/docs/site/docs/adopt/exit-and-decommission.md).
3. Install it — let your AI coding assistant do the work:
   1. Download `agf-<version>.tar.gz` (or `.zip`) from the
      [GitLab Releases page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases)
      into a `tmp/` directory in your project. Add the matching
      `agov-<version>-<platform>.tar.gz`/`.zip` from the **same release** if
      you want the `agov` validator without a full source checkout — both
      archives ship from the same tag, so "matching version" just means
      "same release page."
   2. Tell your coding assistant: *"There is a `.tar.gz` file in `tmp/`.
      Adopt it into the project."* It unpacks the archive, wires `agf/`,
      and prepares a diff for you to review — it does not commit or claim
      adoption is complete on its own.

   Full mechanics, and the longer commissioning prompt if you want more
   control over the first pass:
   [Install the `agf/` directory](agf/docs/site/docs/adopt/install-and-layouts.md).
4. Run one valid example:
   [Your first governed change](agf/docs/site/docs/start/first-governed-change.md).

The first-run templates live in [agf/templates](agf/templates/).

## What a trial adds to your project

Default POC layout:

| Artifact | Default path |
|---|---|
| AGF distribution | `agf/` |
| Assistant rules | `agf/rules/` |
| Optional assistant loader | Tool-specific root file that points at `agf/rules/` |
| Working plans | `docs/plans/` |
| Coordination channel | `docs/coordination/` |
| Learning record | `agf/learnings/<codename>.yaml` |
| Current core validators | `agov` from a full checkout |
| Communication map | `agov coord graph` — Graphviz dot map of role-to-role coordination flows |
| Coordination index | `agov coord graph-index` — JSON index of handoffs and references |
| Tool provenance | `agov --version` / `agov version --format json` |

The `agf/` distribution ships **44 governance rules**, **12 agent definitions**,
and **7 skills**.

The framework source archive is usable on its own. Adopters who want the
current core validators can use a full checkout or the optional packaged
`agov` tooling archive; the combined bundle is a convenience artifact, not a
prerequisite for adopting AGF.

## Local learning and upstream proposals

Two concepts recur across the docs and are easier to follow as a diagram
than as prose: how a project's own operational evidence becomes a
learning record, and how a pattern worth sharing travels upstream as a
rule or agent proposal. Both are explained with an embedded figure on
the published docs site, kept in sync with the text:

- [Evidence and learnings](https://janipaijanen.gitlab.io/agents-govern-framework/concepts/evidence-and-learnings/)
  — the learning-loop diagram, and the ownership table distinguishing
  framework-owned learning records from your own
  `agf/learnings/<codename>.yaml` (never touched on upgrade).
- [Contribute](https://janipaijanen.gitlab.io/agents-govern-framework/contribute/)
  — the three upstream paths (submit a learning, propose a rule, propose
  an agent) and the evidence bar each one requires.

Figure sources are Graphviz DOT files under
[agf/docs/figures/src/](agf/docs/figures/src/); see
[figures.md](agf/docs/figures/figures.md) for the full inventory,
including the governance loop, gate pipeline, coordination channel, and
exit-flow diagrams referenced elsewhere in these docs.

## How to cite

If you use agents-govern in research or a published project:

**APA 7th edition:**

> Päijänen, J. (2026). *agents-govern: Governance for AI-assisted software
> development* [Computer software]. https://gitlab.com/janipaijanen/agents-govern-framework

For version-specific citations, see the `reference/release-info/` page on the
[published docs site](https://janipaijanen.gitlab.io/agents-govern-framework/)
for the APA and BibTeX entry with the exact version number.

## Git history is a ledger, not a scratchpad

This repository's git history, and the coordination trail under
`docs/coordination/`, are append-only by policy: we do not rewrite, squash,
or purge past commits or coordination messages to satisfy a later request.
Treat them like a journal or ledger — corrections are new entries, not
edits to old ones (see `agf/docs/governance/governance.md` for the
coordination channel's immutability rule, and
`.claude/rules/workflow/immutable-tags-rule.md` for the equivalent policy
on release tags).

This has a direct data-protection implication: an append-only trail that
carries author identities cannot honor an erasure request by rewriting
history without destroying the audit trail it exists to provide. Projects
that need an erasure story handle it upstream of history — identity
minimisation and a documented lawful basis for retention — not by rewriting
committed history. See the [disclaimer](agf/docs/governance/disclaimer.md)
for the fuller statement that regulatory compliance is the adopter's
responsibility, not something this framework guarantees.

## Safety and independence

Only run AI agents and tooling on systems, repositories, and data where you
have the necessary authorization. AGF guidance is not legal, security, or
compliance advice; it is a governance framework you must adapt to your own
context. See the
[disclaimer](https://janipaijanen.gitlab.io/agents-govern-framework/reference/disclaimer/)
for the fuller statement of responsibility and tooling limits.

AGF-NG is developed independently. The project has not received external
funding from, and has no dependency or endorsement relationship with, the
companies or brands whose tools, models, or runtimes may be discussed in the
documentation.
