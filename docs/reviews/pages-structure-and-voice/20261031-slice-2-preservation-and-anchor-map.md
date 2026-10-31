<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->

# Slice 2 ("Should I adopt?"): sentence-preservation report and anchor map

- **Status:** advisory review evidence for the slice 2 candidate. Not a complete semantic inventory and not a blocking check. Review unflagged safeguards too.
- **Plan:** `docs/plans/20261030T163253Z-pages-structure-and-voice.md`, work allocation item 4. Claim: `20261031T095048Z-planner-architect-6d8r`.
- **Reproduce:** from the repository root with the candidate checked out:

```
python3 -I docs/reviews/pages-structure-and-voice/slice-report-extractor.py origin/main \
  agf/docs/site/docs/start/should-you-adopt.md agf/docs/site/docs/start/adopter-faq.md \
  agf/docs/site/docs/adopt/downstream-impact.md agf/docs/site/use-cases.md
```

- **Extractor fix:** the slice 1 version compared base sentences with the final text before removing headings, so a sentence that spans a heading could be reported as missing when it was not. This version removes headings, tables and code from the final text the same way as from the base. The fix can only remove false "not found" results. It never adds a pass.
- **Limit:** it cannot see a meaning change caused by new text placed beside an old sentence. The verbatim count is evidence, not an invariant.

## Extractor output

```
Base ref: origin/main (6bd6cfb02b594ecc5a2ce94eb96f56374cb330cd)

## should-you-adopt.md

- Candidate sentences in base: 10; present verbatim in final: 10; not found: 0
- Headings before: 11; after: 11
- Heading ids before: should-you-adopt, what-you-are-deciding, the-practical-test, what-agf-is-is-not, what-ships-and-what-you-activate, adopt-now-if, wait-if, skip-if, what-adoption-asks-of-you, privacy-labor-and-provider-boundary-checklist, next
- Heading ids after:  should-you-adopt, what-you-are-deciding, the-practical-test, what-agf-is-is-not, what-ships-and-what-you-activate, adopt-now-if, wait-if, skip-if, what-adoption-asks-of-you, privacy-labor-and-provider-boundary-checklist, next
- Removed ids: none; added ids: none

## adopter-faq.md

- Candidate sentences in base: 23; present verbatim in final: 21; not found: 2
  - NOT FOUND: "Delegation does not transfer gate authority."
  - NOT FOUND: "A principal may authorize patch preparation, evidence collection, or test runs; Gate 1, Gate 2, release approval, and binding risk acceptance still require the governor-reviewer."
- Headings before: 23; after: 23
- Heading ids before: adopter-faq, do-we-need-agf-or-just-better-prompts, what-are-the-hard-rules, should-we-commission-the-tooling-too, does-agf-replace-our-code-review, how-do-adrs-fit-with-agf, who-is-the-governor, can-different-people-approve-gate-1-and-gate-2, can-ai-act-as-a-governor, how-much-governor-time-should-we-expect, when-should-we-write-a-learning-record, how-do-we-draft-a-learning, do-downstream-adopters-get-a-learning-template, when-should-a-learning-go-upstream, how-do-we-push-a-learning-upstream, can-one-ai-assistant-play-several-roles, can-an-assistant-work-on-behalf-of-a-human, can-several-ai-assistants-work-at-once, can-several-humans-and-several-ai-assistants-use-agf-together, how-should-a-mixed-human-and-ai-team-avoid-collisions, what-adoption-pattern-should-we-start-with, what-should-our-first-week-look-like, what-should-we-avoid-in-the-first-week
- Heading ids after:  adopter-faq, do-we-need-agf-or-just-better-prompts, what-are-the-hard-rules, should-we-commission-the-tooling-too, does-agf-replace-our-code-review, how-do-adrs-fit-with-agf, who-is-the-governor, can-different-people-approve-gate-1-and-gate-2, can-ai-act-as-a-governor, how-much-governor-time-should-we-expect, when-should-we-write-a-learning-record, how-do-we-draft-a-learning, do-downstream-adopters-get-a-learning-template, when-should-a-learning-go-upstream, how-do-we-push-a-learning-upstream, can-one-ai-assistant-play-several-roles, can-an-assistant-work-on-behalf-of-a-human, can-several-ai-assistants-work-at-once, can-several-humans-and-several-ai-assistants-use-agf-together, how-should-a-mixed-human-and-ai-team-avoid-collisions, what-adoption-pattern-should-we-start-with, what-should-our-first-week-look-like, what-should-we-avoid-in-the-first-week
- Removed ids: none; added ids: none

## downstream-impact.md

- Candidate sentences in base: 7; present verbatim in final: 7; not found: 0
- Headings before: 7; after: 7
- Heading ids before: downstream-impact-what-adoption-changes-in-your-project, files-added-to-your-repository, ci-jobs-added, workflow-obligations-per-change, ongoing-obligations, what-adoption-does-not-change, honest-summary
- Heading ids after:  downstream-impact-what-adoption-changes-in-your-project, files-added-to-your-repository, ci-jobs-added, workflow-obligations-per-change, ongoing-obligations, what-adoption-does-not-change, honest-summary
- Removed ids: none; added ids: none

## use-cases.md

- Candidate sentences in base: 5; present verbatim in final: 5; not found: 0
- Headings before: 5; after: 5
- Heading ids before: use-cases-and-non-use-cases, credible-use-cases, non-use-cases, maturity-honestly, next
- Heading ids after:  use-cases-and-non-use-cases, credible-use-cases, non-use-cases, maturity-honestly, next
- Removed ids: none; added ids: none

## Cross-file anchor links in the final files

- should-you-adopt.md -> ../concepts/roles-and-gates.md#which-roles-does-my-project-need: ok
- should-you-adopt.md -> ../../future/evolution.md#reduced-first-adoption-product: ok
- should-you-adopt.md -> ../adopt/install-and-layouts.md#step-4-review-rule-loading-and-customization: ok
- adopter-faq.md -> (same file)#do-we-need-agf-or-just-better-prompts: ok
- adopter-faq.md -> (same file)#who-is-the-governor: ok
- adopter-faq.md -> (same file)#should-we-commission-the-tooling-too: ok
- adopter-faq.md -> (same file)#can-several-ai-assistants-work-at-once: ok
- adopter-faq.md -> (same file)#what-should-our-first-week-look-like: ok
- adopter-faq.md -> ../concepts/roles-and-gates.md#approval-authority-and-optional-deputies: ok
- adopter-faq.md -> ../concepts/roles-and-gates.md#approval-authority-and-optional-deputies: ok
- downstream-impact.md -> ../start/adopter-faq.md#when-should-we-write-a-learning-record: ok
- downstream-impact.md -> ../start/adopter-faq.md#how-do-we-draft-a-learning: ok
- downstream-impact.md -> ../operate/day-to-day.md#coordination-hygiene: ok
- use-cases.md -> docs/start/should-you-adopt.md#wait-if: ok
```

## Disposition of the two replaced sentences (both in `adopter-faq.md`)

| Base sentence | Where | Disposition |
|---|---|---|
| "Delegation does not transfer gate authority." | "Can an assistant work on behalf of a human?" | Reworded to "A delegation grant does not transfer gate authority." Same meaning. |
| "A principal may authorize patch preparation, evidence collection, or test runs; Gate 1, Gate 2, release approval, and binding risk acceptance still require the governor-reviewer." | same | Split. The first half is kept. The second half no longer says all four always require the governor-reviewer, because the deputy rule permits a narrow opt-in Gate 2 route. It now says: these grants never delegate Gate 1, release approval or binding risk acceptance; a grant alone never delegates a Gate 2 decision; the only delegated Gate 2 approval route described here is the narrow opt-in deputy route. This is the wording the governor approved for the roles page under decision n4rq. Owner: `agf/rules/workflow/deputy-governor-rule.md`. No category is added. |

## A1 Evaluate answer-key sections (should-you-adopt.md)

The five counted sections (The practical test, Adopt now if, Wait if, Skip if, What adoption asks of you) are textually identical
before and after (65, 139, 132, 73 and 281 words in the source). The edits are in "What ships and what you activate" and "Next",
which are outside the answer key. The rendered Evaluate total is therefore expected to stay 982, but only the rendered counter
can confirm it.

## What the slice adds

- `should-you-adopt.md`: the solo trial and standard project sentence now matches the owners (three roles for the solo trial,
  one assistant may play two), points to the roles page triggers, and a "Next" line points to Licensing.
- `adopter-faq.md`: one paragraph in "Can AI act as a governor?" and one rewritten paragraph in the delegation answer, both pointing to
  `roles-and-gates.md#approval-authority-and-optional-deputies`.
- `downstream-impact.md`: one sentence at the end of "Honest summary" pointing to the licensing page.
- `use-cases.md`: no change. Its five keyword sentences are unchanged.

## What the slice does not do

No heading or id changes. No rule, role definition, figure, CSS or navigation change. No policy change.
