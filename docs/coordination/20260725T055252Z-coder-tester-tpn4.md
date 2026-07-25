---
kind: handoff
from: coder-tester
to: fact-checker
id: 20260725T055252Z-coder-tester-tpn4
timestamp: 2026-07-25T05:52:52Z
model: gpt-5-codex
immutable: true
references-to:
  - 20260724T174750Z-governor-reviewer-j55n
  - 20260724T205538Z-governor-reviewer-mpea
topic: v242-public-experience-exact-text-fact-check
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: fact-checker
  status: ready
  task:
    - Review the exact public text in MR !488 at SHA 861aed0bdd1570ac7d321967aa73cfa893b0741b before Gate 2.
    - Return ADOPT, ADAPT, or REJECT for each implementation commit, with the byline-only commit explicitly marked outside factual-review scope if no factual claim needs review.
    - Check that simplification preserves every feature, maturity, evidence, cost, privacy, enforcement, data-boundary, command-inventory, dossier-link, and homepage-outcome boundary named in the approved plan.
  must_read:
    - docs/plans/v2.4.2-public-experience-and-evidence-presentation.md
    - MR !488 at 861aed0bdd1570ac7d321967aa73cfa893b0741b
    - commit 5ffd98b (factual and trust-boundary corrections)
    - commit 6a4ab12 (homepage tooling inventory)
    - commit 861aed0 (dossier reference layer)
    - commit 18e9639 (six-file plain-language pass)
  do_not_touch:
    - MR !488 implementation branch
    - version, release, tag, or publication state
    - role definitions, CI configuration, schemas, or adopter contracts
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Exact-text fact-check request for v2.4.2 public experience

### Codex · coder-tester

Please review MR !488 at exact SHA
`861aed0bdd1570ac7d321967aa73cfa893b0741b`. The implementation is complete,
but Gate 2 is intentionally not requested until the approved fact-checker
consultation returns a disposition against this exact text.

The review scope is bounded by the approved plan:

- data handling and external-provider anonymization wording;
- capability maturity and canonical command-inventory claims;
- the homepage proof block's claims about current `agov` outcomes;
- exact-build dossier repository-reference claims; and
- every simplification in the six-file plain-language inventory, ensuring
  shorter wording does not erase feature, evidence, cost, privacy, or
  enforcement limits.

The byline convention in `ff844df` is a documentation process and is outside
the plan's factual-review scope. The adoption-exit terminology correction is a
consistency repair and is likewise outside scope. Please still account for
each of the five commits in the response so the Gate 2 evidence is complete.

Local evidence at the exact head is green: focused hook tests, CI-equivalent
strict MkDocs build, adopter-docs validation, README-stat validation, numeric
palette contrast validation, exact-SHA dossier-link rendering, and
`git diff --check`. MR pipeline evidence is running separately.
