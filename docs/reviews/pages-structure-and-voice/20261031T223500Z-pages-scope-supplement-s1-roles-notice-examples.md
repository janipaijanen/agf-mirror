<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->

# S1 supplement: preservation report and anchor map

- **Status:** advisory review evidence. Not a complete semantic inventory and not a blocking check.
- **Plan:** `docs/plans/20261031T223500Z-pages-scope-supplement-s1-roles-notice-examples.md` (Gate 1 `20261031T224419Z-governor-reviewer-jmkh`). Allocation: jmkh. Acknowledgement `20261031T225203Z-planner-architect-r8qm`, claim `20261031T225205Z-planner-architect-mh2s`.
- **Immutable base:** `5b4c3f110347a1e5fafd15647becee07dafbe652`.
- **Author limit:** the author wrote this change and is not independent of it. The author cannot review it.
- **Reproduce:** `python3 -I docs/reviews/pages-structure-and-voice/slice-report-extractor.py 5b4c3f110347a1e5fafd15647becee07dafbe652 agf/docs/site/docs/concepts/roles-and-gates.md agf/docs/site/docs/start/should-you-adopt.md agf/docs/site/docs/concepts/coordination-protocol.md`

## Extractor output

```
Base ref: 5b4c3f110347a1e5fafd15647becee07dafbe652 (5b4c3f110347a1e5fafd15647becee07dafbe652)

## roles-and-gates.md

- Candidate sentences in base: 48; present verbatim in final: 48; not found: 0
- Headings before: 13; after: 13
- Heading ids before: roles-and-gates, roles-in-plain-terms, the-core-idea-separated-authority, the-pipeline-roles, approval-authority-and-optional-deputies, which-roles-does-my-project-need, ask-for-a-specialist-during-a-change, the-two-gates, the-coordination-channel, delegated-ai-work, why-gates-bind-ai-agents-in-particular, structural-and-advisory-enforcement, next
- Heading ids after:  roles-and-gates, roles-in-plain-terms, the-core-idea-separated-authority, the-pipeline-roles, approval-authority-and-optional-deputies, which-roles-does-my-project-need, ask-for-a-specialist-during-a-change, the-two-gates, the-coordination-channel, delegated-ai-work, why-gates-bind-ai-agents-in-particular, structural-and-advisory-enforcement, next
- Removed ids: none; added ids: none

## should-you-adopt.md

- Candidate sentences in base: 17; present verbatim in final: 17; not found: 0
- Headings before: 16; after: 16
- Heading ids before: should-you-adopt, what-you-are-deciding, the-practical-test, adopt-now-if, wait-if, skip-if, what-adoption-asks-of-you, privacy-labor-and-provider-boundary-checklist, understand-the-basics, cost-and-accountability, technical-fit, assistant-workflows, selected-use-cases, what-agf-is-is-not, what-ships-and-what-you-activate, next
- Heading ids after:  should-you-adopt, what-you-are-deciding, the-practical-test, adopt-now-if, wait-if, skip-if, what-adoption-asks-of-you, privacy-labor-and-provider-boundary-checklist, understand-the-basics, cost-and-accountability, technical-fit, assistant-workflows, selected-use-cases, what-agf-is-is-not, what-ships-and-what-you-activate, next
- Removed ids: none; added ids: none

## coordination-protocol.md

- Candidate sentences in base: 42; present verbatim in final: 40; not found: 2
  - NOT FOUND: "A complete four-file thread from Gate 1 approval to task closure."
  - NOT FOUND: "**Step 1 — Gate 1 plan approved** (`20260614T120000Z-pa-add-export-feature.md`) **Step 2 — Governor approves** (`20260614T121500Z-gr-add-export-feature.md`) **Step 3 — Coder-tester files Gate 2** (`20260614T150000Z-ct-add-export-feature.md`) **Step 4 — Closure** (`20260614T160000Z-ct-add-export-feature-done.md`) The four files form a navigable chain: plan → approval → implementation → cl
- Headings before: 19; after: 19
- Heading ids before: coordination-protocol, what-the-coordination-channel-is, writing-messages, publication-is-independent-of-gate-2-but-mechanically-verified, ending-the-approval-chain-for-finished-merges, lifecycle-what-a-records-kind-promises, how-agents-communicate, miniature-example-thread, message-kinds, requires-ack-obligation-semantics, review-bodies, parallel-work-and-conflicts, generating-the-communication
- Heading ids after:  coordination-protocol, what-the-coordination-channel-is, writing-messages, publication-is-independent-of-gate-2-but-mechanically-verified, ending-the-approval-chain-for-finished-merges, lifecycle-what-a-records-kind-promises, how-agents-communicate, miniature-example-thread, message-kinds, requires-ack-obligation-semantics, review-bodies, parallel-work-and-conflicts, generating-the-communication
- Removed ids: none; added ids: none

## Cross-file anchor links in the final files

- roles-and-gates.md -> (same file)#approval-authority-and-optional-deputies: ok
- roles-and-gates.md -> (same file)#approval-authority-and-optional-deputies: ok
- should-you-adopt.md -> ../concepts/roles-and-gates.md#approval-authority-and-optional-deputies: ok
- should-you-adopt.md -> ../concepts/roles-and-gates.md#roles-in-plain-terms: ok
- should-you-adopt.md -> (same file)#what-agf-is-is-not: ok
- should-you-adopt.md -> ../concepts/roles-and-gates.md#which-roles-does-my-project-need: ok
- should-you-adopt.md -> (same file)#what-adoption-asks-of-you: ok
- should-you-adopt.md -> ../concepts/roles-and-gates.md#roles-in-plain-terms: ok
- should-you-adopt.md -> adopter-faq.md#can-one-ai-assistant-play-several-roles: ok
- should-you-adopt.md -> adopter-faq.md#can-several-ai-assistants-work-at-once: ok
- should-you-adopt.md -> ../../use-cases.md#credible-use-cases: ok
- should-you-adopt.md -> ../concepts/roles-and-gates.md#which-roles-does-my-project-need: ok
- should-you-adopt.md -> ../../future/evolution.md#reduced-first-adoption-product: ok
- should-you-adopt.md -> ../adopt/install-and-layouts.md#step-4-review-rule-loading-and-customization: ok
- coordination-protocol.md -> ../../reference/tooling-conformance.md#coordination-visibility-scope: ok
- coordination-protocol.md -> ../../reference/tooling-conformance.md#unresolved-handoff-reply-evidence: ok
```

The two "not found" lines for `coordination-protocol.md` are the four replaced examples and the "four-file" sentence. They are the intended replacement.

## What changed

| Path | Change | Disposition of old text |
|---|---|---|
| `roles-and-gates.md` | One paragraph inserted after the first paragraph of "Roles in plain terms". | Nothing removed. All 48 keyword sentences are present verbatim. No heading id added or removed. |
| `should-you-adopt.md` | One sentence and a link added at the end of "Understand the basics". | Nothing removed. All 17 keyword sentences are present verbatim. The section is not in the A1 Evaluate range. |
| `coordination-protocol.md` | The "Miniature example thread" section: four records replaced by five. | Replaced on purpose. The old records lacked required fields (`timestamp`, `immutable`, `requires-ack`, the `handoff` object) and used a `status` field the schema does not define. The thread ended without a Gate 2 decision. The old closure said "Merged... at commit abc1234". |
| `overrides/main.html` | A Material `announce` block added: one conditional-purpose sentence linking to the dossier on every public page except the dossier. The guard hides the notice only when page metadata positively identifies the dossier source file; absent page metadata (the 404 page) shows the notice. Whitespace control keeps the dossier page free of an empty banner. | The template had no announce block. Nothing removed. |

## Evidence from the final source

- The five records were extracted from the final page source and each passes `agov coord validate` (exit 0). The thread is labelled illustrative, with one clearly labelled illustrative full commit SHA `0123456789abcdef0123456789abcdef01234567` in the Gate 2 handoff and the Gate 2 decision.
- The sixteen role-definition YAML files in `agf/agents/` (excluding the two manifest files) and the sixteen skipped starter entries in `MANIFEST.example.yaml` were counted on this base. This change does not edit the manifest.
- `which-roles-does-my-project-need` resolves in `roles-and-gates.md` (heading "Which roles does my project need?").
- Normative pointers kept: every existing link to the deputy-governor rule, the approval-authority section and the roles anchors remains (the extractor reports all anchors "ok").

## Not yet shown

- Strict build, hook results and the original A1 recount on the CI artifact. None of the edited sections is in a counted range, so the counter should not move.
- The notice's measured visible height on desktop and mobile in light and dark, keyboard order to the link, primary calls to action in the first screen, the dossier page with no `md-banner` element, and the 404 page with the notice. The notice is outside the article, so the original counter does not count it. It is still visible reading and it still moves content down.
- Independent source review and the distinct-model rendered web-UX review. Neither is supplied by the author.
