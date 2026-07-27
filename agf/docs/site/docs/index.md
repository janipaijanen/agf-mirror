# Documentation

*Status: Released. Current framework version: see [Release information](../reference/release-info.md).*

AGF adds human-approved checkpoints and a durable decision trail around
AI-assisted changes; it does not run the assistant. This documentation is a
learning path, not a pile of pages. If you are deciding whether that trade is
worth it, start at **1. Start**. If AGF is already installed, jump to the stage
that matches the work in front of you, then follow the path forward.

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

```
Start ──► Concepts ──► Adopt ──► Operate ──► Advanced ──► Evolution
 (decide)   (mental      (impact,    (daily      (contracts,   (future,
             model)       install,    loop,       customize)    non-binding)
                          exit path)  release)
```

### 1. Start — decide

- [Should you adopt?](start/should-you-adopt.md) — an honest decision page:
  who benefits now, who should wait, who should skip.
- [Adopter FAQ](start/adopter-faq.md) — plain answers to the questions that
  usually appear while commissioning the framework: tooling, learnings,
  upstreaming evidence, and multi-agent team use.

### 2. Concepts — the mental model

- [AGF and agent runtimes](concepts/position.md) — how AGF relates to
  orchestrators (Claude Code, Codex CLI, Cursor, LangGraph, CrewAI);
  per-tool setup notes.
- [Roles and gates](concepts/roles-and-gates.md) — agents, authority
  boundaries, Gate 1/Gate 2, the coordination channel.
- [Coordination protocol](concepts/coordination-protocol.md) — message kinds,
  handoffs, acknowledgements, and the communication map that keeps roles
  reconstructible across sessions.
- [Learning loop and evidence](concepts/evidence-and-learnings.md) — the
  learning system; upstream-owned vs adopter-owned records; anonymization.

### 3. Adopt — install with eyes open

- [Downstream impact](adopt/downstream-impact.md) — **read before
  adopting**: every file, CI job, obligation, and ongoing cost adoption
  introduces.
- [Install the `agf/` directory](adopt/install-and-layouts.md) — commissioning
  the files into the single supported v2.0 layout with an assistant-prepared
  diff.
- [Your first governed change](start/first-governed-change.md) — one small
  change through plan approval, implementation, and review.
- [Tooling and token savings](adopt/tooling-and-tokens.md) — how the `agov`
  CLI replaces repeated AI reads with deterministic tool output, current
  command maturity, and how to wire it into CI.
- [Exit and decommission](adopt/exit-and-decommission.md) — how to leave
  cleanly, so you know adoption is reversible.

### 4. Operate — the daily loop

- [Operations runbook](operate/runbook.md) — choose the right procedure for
  session continuity, polling, ordinary work, review, upgrades, and exit.
- [Session continuity](operate/session-continuity.md) — checkpoint before
  compaction or reboot and recover honestly after abrupt loss.
- [Poll and coordinate](operate/poll-and-coordinate.md) — scan coordination,
  changes and comments, issues and comments, then advance authorized work.
- [Day-to-day operation](operate/day-to-day.md) — claims, handoffs, gates,
  completions in an active project.
- [Run AGF in real situations](operate/run-agf-in-real-situations.md) —
  choose the right AGF move for common and less-common project moments.
- [Review and release](operate/review-and-release.md) — Gate 2 practice,
  release via merge request, upgrades, audit trail hygiene.

### 5. Advanced — integration and customization

- [Contracts and validators](advanced/contracts-and-validators.md) —
  JSON schemas, binding contracts, wiring `agf` checks into CI.
- [Customizing the framework](advanced/customizing-the-framework.md) —
  what you may tune freely, and where customization breaks contract
  semantics.
- [Tooling conformance](../reference/tooling-conformance.md) — how the
  `agf` command surface remains language-neutral as implementations evolve.

### 6. Contribute

- [Contribute](../contribute.md) — three paths for feeding evidence back:
  submit a learning, propose a rule, propose an agent. Evidence thresholds,
  anonymization requirement, and independent-adopter gaps.

### 7. Evolution — explicitly non-binding

- [Where v2 could evolve](../future/evolution.md) — hypotheses under
  consideration, labelled as such. Nothing on that page is a commitment.

## Reference

- [Quick reference](https://gitlab.com/janipaijanen/agents-govern-framework) —
  one-page roles/pipeline/rules summary ships in the distribution at
  `docs/reference/quick-reference.md`.

## Choose your next page

Still deciding → [Should you adopt?](start/should-you-adopt.md). Already using
AGF → [Operations runbook](operate/runbook.md). Ready to install →
[Install the `agf/` directory](adopt/install-and-layouts.md).
