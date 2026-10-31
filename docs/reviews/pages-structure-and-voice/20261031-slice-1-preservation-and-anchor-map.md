<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->

# Slice 1 ("Who does what?"): sentence-preservation report and anchor map

- **Status:** advisory review evidence for the slice 1 candidate. It is not a complete semantic inventory and not a blocking check. Review unflagged safeguards too.
- **Plan:** `docs/plans/20261030T163253Z-pages-structure-and-voice.md`, work allocation item 4. Claim: `20261031T055947Z-planner-architect-m2xq`. Corrections: governor decision `20261031T063133Z-governor-reviewer-5sh2`.
- **Reproduce:** from the repository root, with the candidate checked out:

```
python3 -I docs/reviews/pages-structure-and-voice/slice-report-extractor.py origin/main \
  agf/docs/site/docs/concepts/roles-and-gates.md agf/docs/site/docs/concepts/position.md
```

  The committed extractor `slice-report-extractor.py` (standard library only) holds the exact keyword expression, the sentence splitter and the heading-id rule. Its output follows, unedited, for base `origin/main` at the commit shown.
- **Limit:** it cannot see a meaning change caused by new text placed beside an old sentence. The independent factual review reads the new sections against the owners. The verbatim count is evidence, not an invariant: a needed faithful correction may replace a sentence with a documented disposition.

## Extractor output

```
Base ref: origin/main (705114ccb85d143065f56f72504d5edd4ab43e77)

## roles-and-gates.md

- Candidate sentences in base: 37; present verbatim in final: 36; not found: 1
  - NOT FOUND: "No agent both implements and approves its own work."
- Headings before: 10; after: 13
- Heading ids before: roles-and-gates, the-core-idea-separated-authority, the-pipeline-roles, approval-authority-and-optional-deputies, the-two-gates, the-coordination-channel, delegated-ai-work, why-gates-bind-ai-agents-in-particular, structural-and-advisory-enforcement, next
- Heading ids after:  roles-and-gates, roles-in-plain-terms, the-core-idea-separated-authority, the-pipeline-roles, approval-authority-and-optional-deputies, which-roles-does-my-project-need, ask-for-a-specialist-during-a-change, the-two-gates, the-coordination-channel, delegated-ai-work, why-gates-bind-ai-agents-in-particular, structural-and-advisory-enforcement, next
- Removed ids: none; added ids: ['ask-for-a-specialist-during-a-change', 'roles-in-plain-terms', 'which-roles-does-my-project-need']

## position.md

- Candidate sentences in base: 4; present verbatim in final: 4; not found: 0
- Headings before: 11; after: 11
- Heading ids before: agf-and-agent-runtimes, concrete-relationship, per-tool-notes, claude-code, codex-openai, cursor-windsurf, langgraph-crewai-autogen, how-agf-relates-to-other-frameworks, enterprise-boundary, what-agf-does-not-do, next
- Heading ids after:  agf-and-agent-runtimes, concrete-relationship, per-tool-notes, claude-code, codex-openai, cursor-windsurf, langgraph-crewai-autogen, how-agf-relates-to-other-frameworks, enterprise-boundary, what-agf-does-not-do, next
- Removed ids: none; added ids: none

## Cross-file anchor links in the final files

- roles-and-gates.md -> (same file)#approval-authority-and-optional-deputies: ok
- roles-and-gates.md -> (same file)#approval-authority-and-optional-deputies: ok
- position.md -> roles-and-gates.md#roles-in-plain-terms: ok
- position.md -> ../adopt/assistant-loader-configuration.md#verify-the-loader: ok
```

## Disposition of the one replaced sentence

| Base sentence | Where | Disposition |
|---|---|---|
| "No agent both implements and approves its own work." | `roles-and-gates.md`, "The core idea" | Replaced under 5sh2 by the opening "Planning, building and approving are separate responsibilities. An assistant may plan and implement a change; neither task gives it approval authority. ... This human role conflict does not permit an implementing assistant to approve its own work." Owner: `agf/ADOPTING.md` section 1.1 (the temporary human Coder-Tester and Governor-Reviewer conflict) and `agf/agents/governor-reviewer.yaml` (self-approval prohibition). The jurisdiction, authority and escalation sentence, the "acted outside its definition" sentence and the structural-versus-participant-discipline sentence are kept unchanged. |

## What the slice adds

- `roles-and-gates.md`: the new opening of "The core idea" and the sections "Roles in plain terms", "Which roles does my project need?" and "Ask for a specialist during a change". The anchor `#approval-authority-and-optional-deputies` is kept.
- `position.md`: five lines in the opening paragraph that define an agent as a role definition, name the people, assistants or sessions that play it, say that an orchestrator is an execution surface and not automatically the authorized role holder, and link to `roles-and-gates.md#roles-in-plain-terms`.

## What the slice does not do

It does not change the gate descriptions, the figures, or any canonical owner. It makes no policy change.
