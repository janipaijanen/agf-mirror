# agents-govern

*The framework that cannot govern itself has no business governing anything else.*

Plans reviewed before code. Code reviewed before merge. Not a hosted
service, not a runtime, not a coding agent itself — governance files
that live in your repository.

## Not this

- Not a hosted service — nothing to sign up for, nothing running outside your repo.
- Not an agent runtime or orchestrator — it does not execute or schedule agents.
- Not a root of trust by itself — see "Fit" below for what that distinction means for you.
- Not a CI/CD system — it governs decisions around your existing pipeline, not the pipeline itself.
- Not a coding assistant — it governs the assistant you already use.

## Fit

Full five-minute filter: [Should you adopt?](agf/docs/site/docs/start/should-you-adopt.md).
Check cost and reversibility before you start:
[Downstream impact](agf/docs/site/docs/adopt/downstream-impact.md) and
[Exit and decommission](agf/docs/site/docs/adopt/exit-and-decommission.md).

Try AGF when:

- AI assistants write or review a meaningful share of your code.
- A human can serve as governor — approving plans before implementation and
  reviews before merge.
- Your project will live long enough for an audit trail to matter.

Skip or wait when:

- you need a tamper-proof, identity-bound root of trust as the product — AGF
  is process scaffolding today, a different and weaker guarantee;
- your forge's built-in AI code review already covers your risk appetite;
- nobody can own gate decisions.

## Quick install

Pin a release and let your AI coding assistant do the mechanical work:

1. Download a pinned release archive into a `tmp/` directory in your project:

   ```bash
   curl -LO https://gitlab.com/api/v4/projects/janipaijanen%2Fagents-govern-framework/packages/generic/agf/vX.Y.Z/agf-X.Y.Z.tar.gz
   ```

   Replace both `vX.Y.Z` (path) and `X.Y.Z` (filename, no `v`) with the
   version shown on the
   [GitLab Releases page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases) —
   never fetch an unpinned or `latest` archive.
2. Tell your coding assistant: *"There is a `.tar.gz` file in `tmp/`. Adopt
   it into the project."* It unpacks the archive, wires `agf/`, and prepares
   a diff for you to review — it does not commit or claim adoption is
   complete on its own.

For ordered setup, prerequisites, and the full commissioning prompt, use
[Commission AGF](agf/docs/site/docs/start/commission.md). For layout,
customization, and validation details, see
[Install the `agf/` directory](agf/docs/site/docs/adopt/install-and-layouts.md).
Then run one valid example:
[Your first governed change](agf/docs/site/docs/start/first-governed-change.md).
First-run templates live in [agf/templates](agf/templates/).

## AI assistants

Read [`AGENTS.md`](AGENTS.md) before acting. The `agov` CLI is the universal
supported path. Recommended for MCP-capable assistants: connect
[agov MCP](agf/docs/site/docs/adopt/mcp-server.md) for structured access.

## Using AGF with other assistant tooling

Assistant tooling may help inside an approved change. It cannot approve AGF
gates or override applicable rules or human governor decisions. Required
coordination records remain part of the evidence. This is a proposed pattern:
Level 2 — Surface-defined, with Theoretical evidence;
compatibility has not been established. Review installers, hooks, and data
flows before enabling another tool. Read the
[dated evidence and limits](agf/docs/site/docs/adopt/third-party-plugin-coexistence.md).

## Evidence and maturity

Every qualifier below still applies — this table only relocates them out of
inline prose per AGF's own evidence-and-maturity practice.

| Claim | Status | Basis |
|---|---|---|
| Mechanical installation | No more than ten minutes | Maintainer's own deployments — empirically informed, not yet independent-adopter validated |
| Initial commissioning (review applicable rules, complete your first governed change) | Variable, no numeric estimate | Separate work from installation; duration depends on your project |
| Governor attention per non-trivial change | Often 5–15 minutes | Maintainer-informed planning estimate, not a benchmark |
| Exit duration | Unmeasured | Varies with your CI, hooks, and retained evidence — see [Exit and decommission](agf/docs/site/docs/adopt/exit-and-decommission.md) |

See [Maturity vocabulary](agf/docs/site/reference/maturity-vocabulary.md) for
the full label definitions (capability maturity levels and evidence-status
labels) referenced throughout this project's documentation.

## Map

| Path | Meaning |
|---|---|
| `agf/` | The adopter-facing AGF v2 product: roles, rules, schemas, docs, templates |
| `docs/` | This repository's own governed work: plans, coordination, proposals |
| `tooling/` | Implementation tooling for validation, build, release, and conformance checks |
| `AGENTS.md` | Contributor and agent orientation for this repository |

For product evaluation, start in `agf/` — `tooling/` and `docs/` are this
repository's own evidence and process, not the shipped product. The
normative files live in `agf/`; binding decisions come from the roles and
gates recorded in the coordination channel. Tool output is evidence; it
becomes blocking only when a named AGF rule or governor decision makes that
check blocking.

## Deep dive

Full narrative — audience routes, roles and gates, coordination protocol,
adoption steps, and the complete reference catalog — lives on the
[published docs site](https://janipaijanen.gitlab.io/agents-govern-framework/);
the [roadmap](https://janipaijanen.gitlab.io/agents-govern-framework/roadmap/)
tracks what's next.

<!-- public-feature-summary:start -->
## What AGF gives you

- **Human plan and merge decisions.** A named human governor decides whether
  implementation may start (Gate 1) and whether the tested change may merge
  (Gate 2); validators provide evidence but never approve a gate.
- **Named roles, explicit authority.** Proposing, implementing, reviewing, and
  approving are separate, named responsibilities, so each participant's remit
  is visible.
- **Durable, reconstructible records.** Plans, handoffs, reviews, and decisions
  stay as repository files tied to exact revisions, preserving the reasoning
  alongside the work.
- **Evidence and maturity labels.** Public capability claims distinguish what
  is planned, implemented, validated, or deployable and whether the supporting
  evidence is theoretical, empirically informed, or empirically validated.
- **Reviewed learning, separately governed improvement.** Operational evidence
  can become a reviewed learning record, and a learning may motivate an
  improvement proposal; neither step changes rules, roles, or scope
  automatically, because improvement begins behind a fresh plan-approval gate.
- **Repository-native ownership, bounded exit.** AGF ships as files, rules, and
  conventions, with no runtime or service holding adopter data. You can pause,
  downgrade, or remove it; optional validation tooling supports consistency but
  is required neither to adopt nor to leave.
<!-- public-feature-summary:end -->

![AGF in a Nutshell — a one-page orientation covering roles, the two gates, and the coordination channel](agf/docs/figures/agf-nutshell-flyer.png)

*AI-generated figure. Publication remains the responsibility of the human maintainer.*

```text
AI writes a plan -> Governor approves -> AI implements + CI validates -> Merge
       Gate 1                                      Gate 2
```

## What a trial adds to your project

Default POC layout:

| Artifact | Default path |
|---|---|
| AGF distribution | `agf/` |
| Assistant rules | `agf/rules/` |
| Optional assistant loader | Tool-specific root file that points at `agf/rules/` |
| Working plans | `docs/plans/` |
| Coordination channel | `docs/coordination/` |
| Coordination write path | `agov coord post` / `agov handoff take` / `agov handoff complete` |
| Learning record | `agf/learnings/<codename>.yaml` |
| Current core validators | `agov` from a full checkout |
| Communication map | `agov coord graph` — Graphviz dot map of role-to-role coordination flows |
| Coordination index | `agov coord graph-index` — JSON index of handoffs and references |
| Coordination updates | `agov coord list --since <id-or-timestamp>` |
| Coordination obligations | `agov coord pending` — unresolved `requires-ack: true` records of any kind other than `handoff`/`ack` |
| Tool provenance | `agov --version` / `agov version --format json` |

The `agf/` distribution ships **51 governance rules**, **16 agent definitions**,
and **9 skills**.

The framework source archive is usable on its own. Adopters who want the
current core validators can use a full checkout or the optional packaged
`agov` tooling archive; the combined bundle is a convenience artifact, not a
prerequisite for adopting AGF.

For MCP-capable assistant runtimes, the Rust tooling archive also includes
`agov-mcp`, a local stdio adapter that exposes `agov` workflow commands as MCP
tools without changing the file-based coordination protocol. See
`agf/docs/site/docs/adopt/mcp-server.md`.

Container-based CI can use the OCI image associated with a published entry on
the [GitLab Releases page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases),
using `registry.gitlab.com/janipaijanen/agents-govern-framework/agov:<version-from-release-page>`.
Pin that exact version or its digest, never a floating tag.

## Local learning and upstream proposals

Two concepts recur across the docs and are easier to follow as a diagram
than as prose: how a project's own operational evidence becomes a
learning record, and how a pattern worth sharing travels upstream as a
rule or agent proposal. Both are explained with an embedded figure on
the published docs site, kept in sync with the text:

- [Evidence and learnings](https://janipaijanen.gitlab.io/agents-govern-framework/docs/concepts/evidence-and-learnings/)
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

## Acknowledgement of AI assistance

AI assistants have contributed to this project, including planning, drafting,
implementation, testing, documentation, and review. Their work is subject to
the project's governed review process.

AI assistants do not approve gates or releases and do not hold project
authority. The human maintainer and governor, Jani Päijänen, retains approval
authority, authorizes publication, and takes responsibility for the published
content and project decisions.

For accessibility and transparency, every figure created with AI assistance
must have descriptive alternative text and a visible AI-generation disclosure
at each published embed. The project applies this rule to technical diagrams
as well as synthetic artwork. This publication control supports the
transparency and accessibility objectives of
[Article 50 of the EU AI Act](https://eur-lex.europa.eu/eli/reg/2024/1689/oj),
but is not by itself a certification of legal compliance.

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

## Start development in this checkout

This section is for contributors to AGF's own codebase, not for adopters
evaluating AGF for their project — the rest of this README is written for
the latter. Build the supported Rust helper, put the repository wrapper on
your path, and ask it for the current project state before reading
coordination files by hand:

```bash
cargo build --release --manifest-path tooling/rust/Cargo.toml -p agov
export PATH="$PWD/tooling:$PATH"
agov session resume --base-branch main
```

Then read [`AGENTS.md`](AGENTS.md) for the branch, gate, and contribution
rules that apply to changes in this repository. The `agov` helper is optional
for adopters, but it is the supported operator path for developing AGF itself.

> **CI expectation for the first merge-request push:** The initial branch push
> and MR creation can currently produce both `push` and
> `merge_request_event` pipelines for one commit. Later pushes normally retain
> only the MR pipeline once GitLab can see the open MR. Treat the MR pipeline
> as canonical acceptance evidence; two same-SHA pipelines alone are not a
> reason to retry the push, close and reopen the MR, or assume CI failed. See
> [Agent Cold-Start Habits](docs/reference/agent-cold-start-habits.md#predict-first-push-ci)
> for the comparison and cancellation checklist.

## Next

Contributing here → read [`AGENTS.md`](AGENTS.md), then run the development
start commands immediately above. Evaluating adoption → read
[Should you adopt?](agf/docs/site/docs/start/should-you-adopt.md).
