---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260722T204718Z-contribution-auditor-mj2g
timestamp: 2026-07-22T20:47:18Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260722T201138Z-contribution-auditor-66jj
  - 20260722T201740Z-contribution-auditor-r4ag
  - 20260722T203708Z-planner-architect-j7gp
  - 20260722T204233Z-contribution-auditor-4qq2
topic: first-contact-pedagogy-marketing-consensus-consultation
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Convene independent pedagogical and marketing/customer-success advisory reviews of AGF's first-contact experience.
    - Synthesize agreements and disagreements into one implementation proposal on existing marketing MR !350.
  must_read:
    - docs/coordination/20260722T201138Z-contribution-auditor-66jj.md
    - docs/coordination/20260722T201740Z-contribution-auditor-r4ag.md
    - agf/agents/documentation-writer.yaml
    - .claude/rules/docs/marketing-tone-rule.md
    - .claude/rules/docs/documentation-audience-rule.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Consultation: make AGF's first contact easy, warm, and teachable

The governor requests a genuine multi-lens consultation before implementation.
The question is broader than the word "governor": how should a person feel and
learn when first encountering AGF through the root README, GitLab project,
GitLab Pages homepage, evaluation path, adoption pages, or another public
entry point?

The target includes readers for whom English is not their first language and
readers unfamiliar with AI-assisted software development, governance
frameworks, or AGF's role names. Treat this as the normal global audience, not
an accessibility edge case.

## Advisory structure authorized by the governor

There is no manifest role literally named `pedagogist` or `marketing`.
Use the existing `documentation-writer` definition as a task-scoped
pedagogical advisor, following the precedent in coordination `00094` and
`00172`. Use independent marketing and customer-success advisory lenses under
`planner-architect`. This consultation changes no manifest status and grants
no gate authority; binding decisions remain with `governor-reviewer`.

Claude should have each lens produce its own critique before synthesis. Do not
write one answer and relabel it as several departments.

## Pedagogical review

Assess the accepted approach from `j7gp`/`4qq2`: explain the familiar job
first ("the human with the final say, often you"), then introduce AGF's term
`governor`.

Compare it against credible alternatives, including:

- outcome-first: begin with what the reader gains;
- scenario-first: begin with one recognisable repository change;
- task-first: begin with the smallest action the reader can take;
- progressive vocabulary: delay AGF terms until the reader needs them;
- choose-your-path: separate evaluator, adopter, human operator, and AI
  assistant routes;
- worked-example-first: teach the loop through a short concrete example.

Evaluate cognitive load, translation friendliness, prerequisite assumptions,
progressive disclosure, recall, reader confidence, and the risk of sounding
condescending. Recommend what to teach once, what to repeat, and what should
remain a canonical linked definition.

## Marketing and customer-success review

Assess how AGF can create a low-friction, reassuring first impression without
hype, fake friendliness, fear, or hiding its real process cost. In particular:

- Does the reader recognise a problem they actually have before seeing AGF's
  machinery?
- Can the reader answer "what is this, is it for me, and what do I do next?"
  within a short first visit?
- Does the copy make a small, reversible pilot feel safe?
- Is "you" welcoming and accurate on each surface, or does it become
  presumptuous?
- Can warmth come from respect, permission to skip, honest limitations,
  concrete examples, and visible exit paths rather than promotional claims?
- Which single value statement is canonical, and where should other surfaces
  link instead of paraphrasing it?

Include a cold-reader journey across the root README, Pages landing page,
`evaluate/`, `should-you-adopt`, and the first governed-change path. Flag any
public entry point that strands a reader in unexplained terminology or asks
for commitment before giving confidence.

## Required synthesis on MR !350

After the independent reviews, publish one validated synthesis on the existing
marketing MR !350. It must distinguish:

1. consensus shared by pedagogy, marketing/customer success, and governance;
2. material disagreements and the trade-off behind each;
3. exact recommended first-contact wording, including the first definition of
   `governor`;
4. a surface-by-surface responsibility map for README, Pages homepage,
   evaluation, adoption decision, and first governed change;
5. the smallest evidence test with real humans, preferably including
   non-native English readers;
6. proposed Gate 1 implementation slices, avoiding new pages and duplicate
   claims unless the evidence demonstrates a genuine navigation gap;
7. claims or choices that still require governor input.

Do not edit public documentation during the consultation. The final proposal
must remain part of MR !350 and must not displace the active v2.4.0 release
chain. Advisory work may proceed in parallel only if it does not interrupt the
coder-tester's release obligation on MR !405.
