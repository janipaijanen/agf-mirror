# Documentation

*Status: Released. Current framework version: see [Release information](../reference/release-info.md). Looking for current release status and what's next instead of a learning path? See the [roadmap](../roadmap.md).*

Follow **Decide → Set up → First governed change → Operate**. AGF adds
human-approved checkpoints and repository records around AI-assisted changes.
Your existing assistant executes the work; optional local `agov` tooling
supplies summaries and validation evidence.

**[Decide whether AGF fits](start/should-you-adopt.md)** ·
**[Set up AGF](start/commission.md)** ·
[Already using AGF? Open the operations runbook](operate/runbook.md).

!!! info "AGF in a Nutshell — one-page orientation flyer"
    The short version: an assistant proposes work, a human decides whether it
    starts and merges, and repository files preserve why. AGF calls those
    responsibilities roles, those checkpoints Gate 1 and Gate 2, and the
    durable decision log the coordination channel. For a printable overview:
    [view the full-size flyer (PNG)](../reference/figures/agf-nutshell-flyer.png).

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

## How to read this site

Every page below falls into one of four kinds. Knowing which kind you're
reading tells you who is actually meant to act on it — most of AGF's
procedural detail (running `agov`, writing YAML frontmatter, filing
coordination messages) is AI-executed work a human delegates and reviews,
not a personal to-do list:

- **Human decision** — pages that ask a human to decide something: adopt or
  not, approve or reject, accept a risk. No AI substitute exists for these.
- **AI action** — pages that are majority procedure for an AI
  agent to execute, with the human supervising and reviewing the result.
- **Shared concept** — the mental model both a human and an AI agent need,
  so they can coordinate using the same vocabulary.
- **Reference** — command, schema, and environment-variable precision, read
  after the concept is already clear.

## The path

### 1. Decide

<span id="1-start-decide"></span>

Use [Should you adopt?](start/should-you-adopt.md) for Adopt / Wait / Skip.
Read [Use cases](../use-cases.md) for examples and
[Downstream impact](adopt/downstream-impact.md) for files, costs, privacy, and
ongoing obligations. The [FAQ](start/adopter-faq.md) answers follow-up questions.

### 2. Set up

<span id="3-adopt-install-with-eyes-open"></span>

[Commission AGF](start/commission.md) is the ordered first-time setup path for
new and existing repositories, with optional tooling and a manual route.
[Install and layouts](adopt/install-and-layouts.md) supplies the detailed
layout, customization, validator, and evidence reference.
Verify [assistant loader configuration](adopt/assistant-loader-configuration.md)
before trusting a loader merely because its file exists.

### 3. First governed change

[Run one small real change](start/first-governed-change.md) through both gates.
A copied framework and a green validator are setup evidence; they do not
replace a governor's plan or merge decision.

### 4. Operate

<span id="4-operate-the-daily-loop"></span>

Use the [operations runbook](operate/runbook.md) to choose your next procedure:
[day-to-day work](operate/day-to-day.md),
[session continuity](operate/session-continuity.md),
[polling and coordination](operate/poll-and-coordinate.md),
[real situations](operate/run-agf-in-real-situations.md), or
[review and release](operate/review-and-release.md).
[Upgrade from v1](adopt/upgrade-from-v1.md) and
[exit and decommission](adopt/exit-and-decommission.md) cover transitions.

### Tools and customization

<span id="5-advanced-integration-and-customization"></span>

The [agov CLI](adopt/tooling-and-tokens.md) provides compact summaries and
validators. The official [agov MCP server](adopt/mcp-server.md) exposes local
CLI operations to an assistant; its page records the `dir` limitation.
[Third-party tooling](adopt/third-party-plugin-coexistence.md), including the
dated Ponytail and Caveman evidence, has a separate trust and authority review.
Use the [applicability catalog](adopt/applicability-catalog.md) and
[customization guide](advanced/customizing-the-framework.md) when choosing
local rules and roles.

### Concepts and reference

<span id="2-concepts-the-mental-model"></span>

Start with [roles and gates](concepts/roles-and-gates.md), then
[AGF and agent runtimes](concepts/position.md), the
[coordination protocol](concepts/coordination-protocol.md), and
[learning and evidence](concepts/evidence-and-learnings.md).
The [framework map](start/framework-map.md),
[contracts and validators](advanced/contracts-and-validators.md), and
[tooling conformance](../reference/tooling-conformance.md) give deeper detail.

### About AGF

<span id="6-contribute"></span>
<span id="7-evolution-explicitly-non-binding"></span>

[Evaluate the framework](../evaluate.md), inspect the
[AI evaluation dossier](../ai-evaluation-dossier.md), or
[contribute evidence](../contribute.md). The [roadmap](../roadmap.md) records
release direction; [evolution](../future/evolution.md) contains explicitly
non-binding hypotheses. Nothing there is an implementation commitment.

## Reference

- [Quick reference](https://gitlab.com/janipaijanen/agents-govern-framework) —
  one-page roles/pipeline/rules summary ships in the distribution at
  `docs/reference/quick-reference.md`.

## Choose your next page

Still deciding → [Should you adopt?](start/should-you-adopt.md). Already using
AGF → [Operations runbook](operate/runbook.md). Ready to install →
[Set up AGF](start/commission.md).
