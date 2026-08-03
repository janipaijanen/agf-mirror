---
id: nasa-llis-influence-attribution
status: approved
topic: nasa-llis-influence-attribution-and-acknowledgements
gate1_approved_by: "governor-reviewer; decision 20260803T233430Z-governor-reviewer-ts77"
gate1_approved_at: "2026-08-03T23:34:30Z"
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Documentation and source-attribution changes only. No secrets, personal
      data, access-control behavior, learning corpus, schema, or validator is
      changed.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Contribution-auditor assessed MR !981 and recommended replacing its
      protocol-level comparison with narrow, traceable attribution. The
      recommendation preserves the reviewed research while distinguishing
      design influence, ordinary citation, watch-list classification, and
      standards. The governor explicitly approved that recommendation.
  - role: fact-checker
    status: consulted
    reason: >-
      MR !981's completed review chain primary-verified NASA LLIS Lesson 30101
      and corrected historical/current-policy overclaims. The replacement may
      reuse only the verified Lesson 30101 and AGF repository facts; a bounded
      exact-head factual review remains required before Gate 2.
---

# Plan: NASA LLIS influence attribution and acknowledgements

## Decision and purpose

Replace MR !981's 424-line protocol comparison with concise, traceable design
provenance. NASA's Lessons Learned Information System (LLIS), particularly
public Lesson 30101, prompted an AGF review that led to AGF-specific learning
record changes. Credit that influence without implying protocol adoption,
equivalent stakes, NASA endorsement, or comparable assurance.

## Scope

1. Add NASA LLIS and Lesson 30101 to `docs/sources.md` as material design
   influence and primary evidence for the source record, with the reviewed
   factual boundary.
2. Add concise credit near the public learning-system documentation in
   `agf/docs/site/docs/concepts/evidence-and-learnings.md`.
3. Add `agf/docs/site/acknowledgements.md` and navigation. The page must state
   the project's credit threshold and make already-cited material influences
   visible without turning into protocol-by-protocol comparisons.
4. After the replacement MR is visible, exact-head validated, independently
   reviewed, and merged, close MR !981 as superseded. Preserve its branch,
   discussions, review history, and commits in GitLab.

## Required wording boundary

The NASA credit must convey this substance:

> NASA's Lessons Learned Information System, particularly public Lesson
> 30101, prompted AGF's review of learning-record structure and lifecycle.
> AGF subsequently adopted its own append-safe supersession model and
> formalized its existing status convention. This acknowledgement describes
> design influence, not protocol equivalence, NASA endorsement, or comparable
> assurance stakes.

The implementation may edit for local flow but must retain every limitation.

## Credit threshold

- Materially influenced shipped design: inline design-provenance credit plus
  the acknowledgements page.
- Supports a factual or theoretical claim: ordinary citation near the claim.
- Competitor, sibling, partner candidate, or watch item: source registry only,
  without implying adoption.
- Standard or regulation: formal citation, not kudos or endorsement language.

The acknowledgements page should consolidate existing, already-supported
influences: Argyris and Schön, Deming's PDSA, Leveson's CAST, Lehman,
Swanson/ISO/IEC 14764, and Parnas. ISO/IEC 42001 remains a formally cited
standard and comparison boundary, not an endorsed influence claim.

## Impact

```
Impact:
  Foundry:          Yes. Adds source provenance and replaces an oversized,
                    still-pending comparison plan with a bounded public credit.
  Direct adopters:  Editorial documentation only; no rule, schema, command,
                    field, archive, or workflow behavior changes.
  Downstream:       Documentation navigation gains one page. Existing URLs and
                    behavior remain unchanged.
  Migration:        None.
  Evidence:         NASA LLIS Lesson 30101; MR !981's reviewed research and
                    fact-check chain; merged Path B implementation commits
                    437a5089/e6dbc0a3 and its approved plan.
```

## Validation and review

- `agov plan validate docs/plans/nasa-llis-influence-attribution.md`
- `agov validate adopter-docs`
- `mkdocs build --strict --config-file agf/docs/build/mkdocs.yml`
- `git diff --check`
- Bounded fact-check/docs review at the exact implementation head.
- Negative wording scan: no claim that AGF adopted NASA's protocol, shares
  NASA's stakes, is NASA-endorsed, or conforms to NASA governance.

## Stop conditions

- Do not copy the full !981 comparison into public documentation.
- Do not revive obsolete NASA governance/process claims.
- Do not close !981 before the narrower replacement is merged.
- Do not modify learning schema, validator, or historical learning records in
  this scope.

## Rollback

Revert the documentation MR. Reopen !981 only if the governor explicitly
decides the protocol comparison itself is again needed; its GitLab history is
retained regardless of closure.

