<section class="agf-hero" aria-labelledby="agf-hero-title" markdown>
<div class="agf-hero__copy" markdown>

<div class="agf-hero__eyebrow">Plans and decisions in your repository</div>

<h1 id="agf-hero-title" class="agf-hero__headline">Govern AI-assisted changes.</h1>

<p class="agf-hero__subhead">AGF provides repository files for planning, approving and recording software changes. Keep your existing assistant and tools. A named person or rotation is accountable for approvals: the human governor. In a solo project, that is you.</p>

<div class="agf-hero__actions">
  <a class="agf-hero__action agf-hero__action--primary" href="docs/start/should-you-adopt/" aria-label="Decide whether AGF fits your project">Decide whether AGF fits</a>
  <a class="agf-hero__action agf-hero__action--secondary" href="docs/start/commission/" aria-label="Set up AGF in your repository">Set up AGF</a>
</div>

<div class="agf-hero__foot">Use an assistant configured to load project instructions; tooling is optional.</div>

<p><strong>An illustrative change.</strong> A project needs a CSV export. The assistant writes a plan; the named human governor approves it (Gate 1). The assistant implements and tests. The governor reviews the result and decides whether it may merge (Gate 2). This is the default human-approval path. Checks supply evidence, not approval. An assistant cannot approve its own work.</p>

</div>

<figure class="agf-hero__visual">
  <a href="reference/figures/governed-change-flow.svg"><img src="reference/figures/governed-change-flow.svg" alt="Illustrative default human-approval path: a plan, plan approval, implementation and checks, result approval, then merge."></a>
  <figcaption>Default human-approval example. AI-generated figure; Publication remains the responsibility of the human maintainer.</figcaption>
</figure>
</section>

**What you take on.** Name your governor, budget review time, maintain decision
records, and keep engineering checks. Review
[costs and accountability](docs/start/should-you-adopt.md#cost-and-accountability),
[terms](docs/adopt/licensing.md),
[evidence limits](docs/concepts/evidence-and-learnings.md), and
[exit](docs/adopt/exit-and-decommission.md) before adopting.

Build version, source and release/development status: [Release information](reference/release-info.md).

<section class="agf-routes" aria-labelledby="agf-routes-title" markdown>

<h2 id="agf-routes-title">Choose your next step</h2>

Already using AGF? [Choose your next task](docs/operate/runbook.md).

<div class="agf-route-grid" markdown>

<article class="agf-route-card" markdown>

### [Decide](docs/start/should-you-adopt.md)

Start with your question:

- [Understand the basics](docs/start/should-you-adopt.md#understand-the-basics)
- [Cost and accountability](docs/start/should-you-adopt.md#cost-and-accountability)
- [Technical fit](docs/start/should-you-adopt.md#technical-fit)
- [Assistant workflows](docs/start/should-you-adopt.md#assistant-workflows)

</article>

<article class="agf-route-card" markdown>

<span id="adopt"></span>

### [Set up](docs/start/commission.md)

Follow the existing-project setup path; review the prepared changes.

</article>

<article class="agf-route-card" markdown>

### [First governed change](docs/start/first-governed-change.md)

The tutorial follows setup; installation alone does not demonstrate governed work.

</article>

<article class="agf-route-card" markdown>

### [Operate](docs/operate/runbook.md)

Find daily work, recovery, reviews, upgrades and exit procedures.

</article>

</div>
</section>

<span id="using-agf-with-other-tools"></span>

For optional tools, see [Rust tooling](docs/adopt/tooling-and-tokens.md), the
[local agov MCP server](docs/adopt/mcp-server.md), and
[third-party assistant tooling](docs/adopt/third-party-plugin-coexistence.md).
Third-party coexistence guidance is Level 2 — Surface-defined, with Theoretical
evidence and no adopter-validated coexistence for the evaluated tools.

## What you get

Written role definitions describe responsibilities, permissions and escalation.
Assigning an assistant execution or advisory responsibilities does not grant
approval authority. A definition does not install or run an assistant. AGF supplies no hosted service or agent runtime.
[Roles and approvals](docs/concepts/roles-and-gates.md) explain the boundaries;
the applicable rules state which checks block work and which rely on review.

The [deputy route](docs/concepts/roles-and-gates.md#approval-authority-and-optional-deputies)
is provisional and default-off. It covers only explicitly governed Gate 2
categories, never Gate 1 or self-approval; scope, severity, expiry and exclusions
remain binding.

Coordination records form an append-only decision log. Corrections add linked
records so readers can reconstruct a change; mutable plans are not append-only
records. One assistant across sessions or several assistants can perform assigned
non-approval execution or advisory responsibilities. Approval requires separate
valid authority under the applicable rules.
[Read the coordination protocol](docs/concepts/coordination-protocol.md).

Incidents can become learning records and inform framework changes. Follow
anonymization and submission requirements before sharing evidence upstream.
[Learning and evidence](docs/concepts/evidence-and-learnings.md) explains the
foundry and early-adopter evidence; single-adopter rules are marked Provisional.

## Evidence, maturity, and licensing

Capability maturity describes what exists; evidence status describes the
strength of proof. See the [maturity vocabulary](reference/maturity-vocabulary.md)
and [evolution proposals](future/evolution.md); future proposals are not shipped
capabilities.

Framework content is CC BY-SA 4.0, with separate commercial licensing available
for alternative terms. Tooling code is MIT. Read the
[licensing terms](docs/adopt/licensing.md) and
[release archives and binaries](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases).

For a compact overview, see the
[one-page orientation flyer](reference/figures/agf-nutshell-flyer.png).

*The framework that cannot govern itself has no business governing anything else.*
