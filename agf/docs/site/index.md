<section class="agf-hero" aria-labelledby="agf-hero-title" markdown>
<div class="agf-hero__copy" markdown>

<div class="agf-hero__eyebrow">Two gates, one repo</div>

<h1 id="agf-hero-title" class="agf-hero__headline">Plans reviewed before code.<br>Code reviewed before merge.</h1>

<p class="agf-hero__subhead">Two human gates sit inside your normal git workflow — one before an AI implements a plan, one before the tested result merges. Every governed decision leaves a timestamped, append-only record in your repo.</p>

<div class="agf-hero__actions">
  <a class="agf-hero__action agf-hero__action--primary" href="docs/start/commission/" aria-label="Get started: commission AGF and AGOV in your repository">Get started</a>
  <a class="agf-hero__action agf-hero__action--secondary" href="docs/" aria-label="See the agents-govern documentation overview">See the docs</a>
</div>

<div class="agf-hero__foot">No new accounts · No hosted service · No runtime — files in <code>your-repo/agf/</code></div>

</div>

<figure class="agf-hero__visual">
  <a href="reference/figures/governed-change-flow.svg"><img src="reference/figures/governed-change-flow.svg" alt="Governed change flow from a written plan through human Gate 1, AI implementation, automated validation, human Gate 2, and merge"></a>
  <figcaption>Human decisions, AI action, and automated checks remain visibly distinct. AI-generated figure; publication remains the responsibility of the human maintainer.</figcaption>
</figure>
</section>

*The framework that cannot govern itself has no business governing anything else.*

> **Status:** This site documents the released **agents-govern framework**.
> The exact framework content version and source commit for this build are
> recorded in
> [Release information](reference/release-info.md). Release archives and
> platform-specific Rust binaries are published on the
> [GitLab Releases page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases).
> When evaluating or citing AGF, use the framework content version on the
> Release information page, which also states whether this build is exact
> release documentation or later development documentation.

<section class="agf-routes" aria-labelledby="agf-routes-title" markdown>

<h2 id="agf-routes-title">Choose your next step</h2>

Start with the decision you need to make. Each route points to the canonical
guidance instead of repeating it here.

<div class="agf-route-grid" markdown>

<article class="agf-route-card" markdown>

### [Decide](docs/start/should-you-adopt.md)

Use the adopt / wait / skip filter, including the real governance cost,
evidence limits, and exit path.

[See the one-page orientation flyer](reference/figures/agf-nutshell-flyer.png)
if you need a compact overview first.

</article>

<article class="agf-route-card" markdown>

### [Adopt](docs/adopt/install-and-layouts.md)

Install the `agf/` directory, [commission AGF and AGOV](docs/start/commission.md),
then run [your first governed change](docs/start/first-governed-change.md).

</article>

<article class="agf-route-card" markdown>

### [Operate](docs/operate/day-to-day.md)

Follow the daily workflow and [operations runbook](docs/operate/runbook.md).
Add the optional [Rust tooling](docs/adopt/tooling-and-tokens.md) when compact,
deterministic validation output helps.

</article>

<article class="agf-route-card" markdown>

### [Using AGF with other tools](docs/adopt/third-party-plugin-coexistence.md)

Set authority, instruction, hook, and data-flow boundaries before combining
assistant tooling. Guidance only: Level 2 — Surface-defined; evidence is
Theoretical, with no adopter-validated coexistence for the evaluated tools.

</article>

</div>
</section>

## What you get

**Accountability structure for AI agents.** Each agent operates under a
written definition: what it owns, what it may change, what it must escalate.
Those definitions do not authorize an implementing agent to approve its own
work, or a reviewing agent to implement the change it approves. The audit trail
makes violations reviewable; whether a particular check is structurally
blocked or relies on reviewer discipline is stated by the applicable rule.

**Decisions you can reconstruct.** Every plan approval, review finding, and
merge decision is a file in the coordination channel — the append-only decision
log. Six months later, you can answer "why was this merged?" by reading the
chain, not by archaeology on commit messages. Because the channel is just files
in a repository, it works whether one AI assistant fills all roles across sessions
or several assistants collaborate in parallel.

**Quality gates that bind assistants too.** The rule library expresses your
engineering discipline (test-first, migration hygiene, immutable tags,
merge-request workflow) in a form AI assistants load as instructions. Gate 1
and Gate 2 provide human-reviewed checkpoints with CI validators supplying
evidence; the governor makes the binding decision.

**Learning that compounds.** Incidents become learning records — structured
incident and pattern notes; recurring patterns become rules; rules ship to every
adopter. The framework grows from evidence, and the anonymization policy means
your learnings can flow upstream without revealing your project.

## Evidence, maturity, and licensing

The framework's evidence base is its
[learning records](docs/concepts/evidence-and-learnings.md): operational
incidents and validated patterns from the framework's own development and from
early adopters, recorded under codenames. Rules shipped on single-adopter
evidence are explicitly marked **Provisional** in the rule text itself.

Capability maturity answers “what exists?” using Level 1 Planned through Level
5 Deployable. Evidence status separately answers “how strong is the proof?”
using Empirically validated, Empirically informed, and Theoretical. See the
[maturity vocabulary](reference/maturity-vocabulary.md) and
[evolution page](future/evolution.md) for what is shipped versus hypothesised.

Version, source commit, copyright, and license details are listed in
[Release information](reference/release-info.md). Framework content is CC BY-SA
4.0, with separate commercial licensing available for alternative terms.
Tooling code is MIT.
