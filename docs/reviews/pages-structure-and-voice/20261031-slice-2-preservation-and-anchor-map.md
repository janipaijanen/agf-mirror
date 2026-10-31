<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->

# Slice 2 ("Should I adopt?"): sentence-preservation report and anchor map

- **Status:** advisory review evidence for the slice 2 candidate after the batched correction in review `20261031T101616Z-governor-reviewer-8ksb`. Not a complete semantic inventory and not a blocking check. Review unflagged safeguards too.
- **Plan:** `docs/plans/20261030T163253Z-pages-structure-and-voice.md`, work allocation item 4. Claim: `20261031T095048Z-planner-architect-6d8r`.
- **Reproduce:** from the repository root with the candidate checked out:

```
python3 -I docs/reviews/pages-structure-and-voice/slice-report-extractor.py origin/main \
  agf/docs/site/docs/start/should-you-adopt.md agf/docs/site/docs/start/adopter-faq.md \
  agf/docs/site/docs/adopt/downstream-impact.md agf/docs/site/use-cases.md
```

- **Limit:** the extractor cannot see a meaning change caused by new text placed beside an old sentence. The verbatim count is evidence, not an invariant.

## Extractor output

```
Base ref: origin/main (6bd6cfb02b594ecc5a2ce94eb96f56374cb330cd)

## should-you-adopt.md

- Candidate sentences in base: 10; present verbatim in final: 8; not found: 2
  - NOT FOUND: "In plain terms: it is a set of files that tell your AI assistant how to behave, a review process that a human approves before anything merges, and a decision log that makes those approvals reconstructible later."
  - NOT FOUND: "Evaluators should judge both the complete distributed surface and whatever smaller configuration a given adopter actually runs; neither measurement substitutes for the other, and the smaller configuration is not guaranteed to exist. - AI assistants write a significant share of your code, **and** you have (or are willing to be) a human governor who approves plans and merges. - You are a solo maintainer who wants a repeatable review discipline around AI-generated work before you approve it yourself. - You are a small team that needs the same conventions to hold across tools, sessions, and contributors. - Your project will live long enough for an audit trail to matter — months, not days. - You already have quality conventions you want assistants to follow consistently, and prompting alone has not held. - You need reconstructible evidence for internal audit, security review, or compliance work."
- Headings before: 11; after: 16
- Heading ids before: should-you-adopt, what-you-are-deciding, the-practical-test, what-agf-is-is-not, what-ships-and-what-you-activate, adopt-now-if, wait-if, skip-if, what-adoption-asks-of-you, privacy-labor-and-provider-boundary-checklist, next
- Heading ids after:  should-you-adopt, what-you-are-deciding, the-practical-test, adopt-now-if, wait-if, skip-if, what-adoption-asks-of-you, privacy-labor-and-provider-boundary-checklist, understand-the-basics, cost-and-accountability, technical-fit, assistant-workflows, selected-use-cases, what-agf-is-is-not, what-ships-and-what-you-activate, next
- Removed ids: none; added ids: ['assistant-workflows', 'cost-and-accountability', 'selected-use-cases', 'technical-fit', 'understand-the-basics']

## adopter-faq.md

- Candidate sentences in base: 23; present verbatim in final: 20; not found: 3
  - NOT FOUND: "Use this minimum checklist: - record who owns each gate, including the repository, change, and time period covered by that authority; - give the Gate 2 reviewer the exact approved plan, Gate 1 decision, conditions, and accepted risks; - at Gate 2, compare the diff and evidence with that exact plan; - send scope changes back to Gate 1 instead of silently rewriting the plan at Gate 2; - name one primary governor or project authority who resolves disagreement; - do not let either approver approve implementation they personally performed while acting as coder-tester."
  - NOT FOUND: "Delegation does not transfer gate authority."
  - NOT FOUND: "A principal may authorize patch preparation, evidence collection, or test runs; Gate 1, Gate 2, release approval, and binding risk acceptance still require the governor-reviewer."
- Headings before: 23; after: 23
- Heading ids before: adopter-faq, do-we-need-agf-or-just-better-prompts, what-are-the-hard-rules, should-we-commission-the-tooling-too, does-agf-replace-our-code-review, how-do-adrs-fit-with-agf, who-is-the-governor, can-different-people-approve-gate-1-and-gate-2, can-ai-act-as-a-governor, how-much-governor-time-should-we-expect, when-should-we-write-a-learning-record, how-do-we-draft-a-learning, do-downstream-adopters-get-a-learning-template, when-should-a-learning-go-upstream, how-do-we-push-a-learning-upstream, can-one-ai-assistant-play-several-roles, can-an-assistant-work-on-behalf-of-a-human, can-several-ai-assistants-work-at-once, can-several-humans-and-several-ai-assistants-use-agf-together, how-should-a-mixed-human-and-ai-team-avoid-collisions, what-adoption-pattern-should-we-start-with, what-should-our-first-week-look-like, what-should-we-avoid-in-the-first-week
- Heading ids after:  adopter-faq, do-we-need-agf-or-just-better-prompts, what-are-the-hard-rules, should-we-commission-the-tooling-too, does-agf-replace-our-code-review, how-do-adrs-fit-with-agf, who-is-the-governor, can-different-people-approve-gate-1-and-gate-2, can-ai-act-as-a-governor, how-much-governor-time-should-we-expect, when-should-we-write-a-learning-record, how-do-we-draft-a-learning, do-downstream-adopters-get-a-learning-template, when-should-a-learning-go-upstream, how-do-we-push-a-learning-upstream, can-one-ai-assistant-play-several-roles, can-an-assistant-work-on-behalf-of-a-human, can-several-ai-assistants-work-at-once, can-several-humans-and-several-ai-assistants-use-agf-together, how-should-a-mixed-human-and-ai-team-avoid-collisions, what-adoption-pattern-should-we-start-with, what-should-our-first-week-look-like, what-should-we-avoid-in-the-first-week
- Removed ids: none; added ids: none

## downstream-impact.md

- Candidate sentences in base: 7; present verbatim in final: 7; not found: 0
- Headings before: 7; after: 8
- Heading ids before: downstream-impact-what-adoption-changes-in-your-project, files-added-to-your-repository, ci-jobs-added, workflow-obligations-per-change, ongoing-obligations, what-adoption-does-not-change, honest-summary
- Heading ids after:  downstream-impact-what-adoption-changes-in-your-project, what-adoption-asks-of-the-people-involved, files-added-to-your-repository, ci-jobs-added, workflow-obligations-per-change, ongoing-obligations, what-adoption-does-not-change, honest-summary
- Removed ids: none; added ids: ['what-adoption-asks-of-the-people-involved']

## use-cases.md

- Candidate sentences in base: 5; present verbatim in final: 5; not found: 0
- Headings before: 5; after: 5
- Heading ids before: use-cases-and-non-use-cases, credible-use-cases, non-use-cases, maturity-honestly, next
- Heading ids after:  use-cases-and-non-use-cases, credible-use-cases, non-use-cases, maturity-honestly, next
- Removed ids: none; added ids: none

## Cross-file anchor links in the final files

- should-you-adopt.md -> ../concepts/roles-and-gates.md#approval-authority-and-optional-deputies: ok
- should-you-adopt.md -> ../concepts/roles-and-gates.md#roles-in-plain-terms: ok
- should-you-adopt.md -> (same file)#what-agf-is-is-not: ok
- should-you-adopt.md -> (same file)#what-adoption-asks-of-you: ok
- should-you-adopt.md -> adopter-faq.md#can-one-ai-assistant-play-several-roles: ok
- should-you-adopt.md -> adopter-faq.md#can-several-ai-assistants-work-at-once: ok
- should-you-adopt.md -> ../../use-cases.md#credible-use-cases: ok
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
- downstream-impact.md -> ../start/adopter-faq.md#how-much-governor-time-should-we-expect: ok
- downstream-impact.md -> ../start/should-you-adopt.md#privacy-labor-and-provider-boundary-checklist: ok
- downstream-impact.md -> ../start/adopter-faq.md#when-should-we-write-a-learning-record: ok
- downstream-impact.md -> ../start/adopter-faq.md#how-do-we-draft-a-learning: ok
- downstream-impact.md -> ../operate/day-to-day.md#coordination-hygiene: ok
- use-cases.md -> docs/start/should-you-adopt.md#wait-if: ok
```

## Disposition of every "not found" sentence

| Base sentence or block | File | Disposition |
|---|---|---|
| "In plain terms: it is a set of files ... a review process that a human approves before anything merges, and a decision log ..." | should-you-adopt.md | Reworded to "a review process in which, by default, a human approves before anything merges". A new paragraph states the default human path and points to the narrow, default-off, provisional deputy route. Finding 3 of review 8ksb. Owner: `agf/rules/workflow/deputy-governor-rule.md`. |
| "Evaluators should judge both the complete distributed surface ... - AI assistants write a significant share of your code, **and** you have ... a human governor who approves plans and merges. ..." | should-you-adopt.md | One extractor block spans two sections that are now separated by the reorder. The first sentence is unchanged. In "Adopt now if", "a human governor who approves plans and merges" is now "a human governor accountable for plans and merges" (same word count). Finding 3. |
| "Use this minimum checklist: ... do not let either approver approve implementation they personally performed while acting as coder-tester." | adopter-faq.md | Shortened to prose. Kept: record who owns each gate and the period, give the Gate 2 reviewer the exact approved plan and accepted risks, compare the diff with that plan, send scope changes to Gate 1, name one primary governor. The blanket conflict prohibition is replaced by the canonical rule: prefer an approver who did not implement the change; ADOPTING section 1.1 permits one person to play both Coder-Tester and Governor-Reviewer temporarily in Phase 1; record it; an implementing assistant never approves its own work. Finding 2. Owners: `agf/ADOPTING.md` section 1.1 and `roles-and-gates.md`. |
| "Delegation does not transfer gate authority." and the "A principal may authorize ..." sentence | adopter-faq.md | Unchanged from the first slice 2 candidate: reworded as recorded in b87a. Same meaning. Deputy rule is the only delegated Gate 2 route. |

## Other changes in this batch

- `should-you-adopt.md`: sections reordered so the decision comes before the inventory: intro, practical test, Adopt/Wait/Skip, what adoption asks of you, then new short sections **Understand the basics**, **Cost and accountability**, **Technical fit**, **Assistant workflows**, **Selected use cases**, then the retained inventory (**What AGF is / is not**, **What ships and what you activate**) and **Next**. The new sections are pointers (about 50 words each) to the canonical owners. No existing heading text or id is removed. New ids: `understand-the-basics`, `cost-and-accountability`, `technical-fit`, `assistant-workflows`, `selected-use-cases` (the Home audience links target the first four).
- `adopter-faq.md`: the AI-governor answer now puts the deputy route's written scope, severity ceiling and expiry, red list, default-off and provisional status, no self-approval, and ultimate human authority in one paragraph. Finding 4. The gate-split answer is shorter and points to the roles page. No heading change.
- `downstream-impact.md`: new section "What adoption asks of the people involved" before the implementation inventory (attention, repository changes, responsibility and privacy, terms and exit) with links to the owners. One id added: `what-adoption-asks-of-the-people-involved`. The earlier sentence pointing to Licensing stays.
- `use-cases.md`: no change. The detailed examples stay there; Decide links to them.

## A1 Evaluate answer-key sections (should-you-adopt.md)

The five counted sections are unchanged in length in the source: 65, 139, 132, 73 and 281 words (same counts as the base). Only one phrase changed, in "Adopt now if" (8 words replaced by 8 words). Reordering the page does not change the heading-to-heading ranges. The rendered counter result for this head is recorded in the handoff record for the corrected head.

## What this batch does not do

No rule, role definition, figure, CSS or navigation change. No heading removed or renamed. No policy change. No Home edit (Home links to the new Decide sections stay with the Home slice).
