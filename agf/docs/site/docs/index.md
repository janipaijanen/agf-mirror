# Documentation

*Status: Provisional (v2.0 documentation).*

This documentation is a learning path, not a pile of pages. Each stage
assumes the previous one. Pick your entry point from the table, then follow
the path forward.

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

- [Roles and gates](concepts/roles-and-gates.md) — agents, authority
  boundaries, Gate 1/Gate 2, the coordination channel.
- [Coordination protocol](concepts/coordination-protocol.md) — message kinds,
  handoffs, acknowledgements, and the communication map that keeps roles
  reconstructible across sessions.
- [Evidence and learnings](concepts/evidence-and-learnings.md) — the
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

- [Day-to-day operation](operate/day-to-day.md) — claims, handoffs, gates,
  completions in an active project.
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

### 6. Evolution — explicitly non-binding

- [Where v2 could evolve](../future/evolution.md) — hypotheses under
  consideration, labelled as such. Nothing on that page is a commitment.

## Reference

- [Quick reference](https://gitlab.com/janipaijanen/agent-governance) —
  one-page roles/pipeline/rules summary ships in the distribution at
  `docs/reference/quick-reference.md`.
