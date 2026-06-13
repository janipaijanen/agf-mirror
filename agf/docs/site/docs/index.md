# Documentation

*Status: Provisional (v2.0 documentation).*

This documentation is a learning path, not a pile of pages. Each stage
assumes the previous one. Pick your entry point from the table, then follow
the path forward.

## The path

```
Start ──► Adopt ──► Concepts ──► Operate ──► Advanced ──► Evolution
 (decide)  (install,   (mental     (daily      (contracts,   (future,
            impact,     model)      loop,       customize)    non-binding)
            exit path)              release)
```

### 1. Start — decide

- [Should you adopt?](start/should-you-adopt.md) — an honest decision page:
  who benefits now, who should wait, who should skip.

### 2. Adopt — install with eyes open

- [Install the `agf/` directory](adopt/install-and-layouts.md) — copying the
  files into the single supported v2.0 layout.
- [Downstream impact](adopt/downstream-impact.md) — **read before
  adopting**: every file, CI job, obligation, and ongoing cost adoption
  introduces.
- [Exit and decommission](adopt/exit-and-decommission.md) — **read before
  adopting**: how to leave cleanly, so you know adoption is reversible.
- [Your first governed change](start/first-governed-change.md) — one small
  change through plan approval, implementation, and review.

### 3. Concepts — the mental model

- [Roles and gates](concepts/roles-and-gates.md) — agents, authority
  boundaries, Gate 1/Gate 2, the coordination channel.
- [Evidence and learnings](concepts/evidence-and-learnings.md) — the
  learning system; upstream-owned vs adopter-owned records; anonymization.

### 4. Operate — the daily loop

- [Day-to-day operation](operate/day-to-day.md) — claims, handoffs, gates,
  completions in an active project.
- [Review and release](operate/review-and-release.md) — Gate 2 practice,
  release via merge request, upgrades, audit trail hygiene.

### 5. Advanced — integration and customization

- [Contracts and validators](advanced/contracts-and-validators.md) —
  JSON schemas, binding contracts, wiring `agov-validate-*` into CI.
- [Customizing the framework](advanced/customizing-the-framework.md) —
  what you may tune freely, and where customization breaks contract
  semantics.

### 6. Evolution — explicitly non-binding

- [Where v2 could evolve](../future/evolution.md) — hypotheses under
  consideration, labelled as such. Nothing on that page is a commitment.

## Reference

- [Quick reference](https://gitlab.com/janipaijanen/agent-governance) —
  one-page roles/pipeline/rules summary ships in the distribution at
  `docs/reference/quick-reference.md`.
