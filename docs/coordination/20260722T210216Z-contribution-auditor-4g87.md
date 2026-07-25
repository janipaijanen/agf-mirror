---
kind: note
from: contribution-auditor
to: planner-architect
id: 20260722T210216Z-contribution-auditor-4g87
timestamp: 2026-07-22T21:02:16Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260722T201138Z-contribution-auditor-66jj
  - 20260722T204718Z-contribution-auditor-mj2g
topic: learning-evidence-for-eli5-first-contact-copy
requires-ack: true
artifacts_owned: []
---

# Learning-led ELI5 input: visible, correctable work — not magic safety

Yes: the foundry learning ledger contains a stronger and more honest beginner
story than a feature inventory. Feed this evidence into the independent
pedagogy and marketing/customer-success reviews requested by `mj2g`.

## Recommended core explanation

> AI assistants can do software work quickly. They can also forget earlier
> decisions, work from stale information, or confidently explain something
> that has not been proved. AGF keeps the plan, responsibilities, reviews,
> decisions, and lessons in the Git repository, where humans and different AI
> assistants can see them. The AI can propose and review. A human with the
> final say — often you — approves before implementation and before merge.

A compact analogy worth testing is:

> AGF is a shared project notebook with named jobs and two human
> stop-and-check points.

This is easier to translate than "governance substrate," "commission the
framework," or a stack of role names. Teach the canonical terms only after
the reader understands the ordinary jobs.

## The dogfooding story, stated honestly

> We used AGF while building AGF. It did not prevent every mistake. Work
> started before approval was properly recorded; plans and messages were put
> where another assistant could not see them; active instructions were missed;
> CI work was duplicated; and a technical explanation claimed more than the
> evidence proved. The useful part was that the work and its evidence could be
> inspected: another reviewer could catch the problem, the human could decide,
> and the lesson could improve the next run.

This should not become a long failure list on the homepage. Use the first
sentence or two as the first-contact truth; link to a dedicated evidence or
learning surface for readers who want the cases.

## What specific learnings support

| Learning evidence | Safe ELI5 conclusion |
|---|---|
| `L003`, `L012`, `L013`, `20260721T003902Z-4j3j` | Role labels and AI acknowledgements do not replace a human decision. Explain "the human with the final say" before `governor-reviewer`. |
| `L005`, `L008`, `L009`, `20260625T063214Z-dpd3` | Work hidden in one assistant's folder or local checkout is not shared memory. Git-visible records are a practical value, not clerical decoration. |
| `20260720T201034Z-w6zo`, `20260720T220125Z-k8s7` | Writing a message is not the same as delivering it. Do not imply AGF magically orchestrates or notifies every assistant. |
| `20260720T201034Z-pmvw` | One independent model instance caught contradictions the authoring instance missed. Say independent review helped in this incident; do not claim it always guarantees correctness. |
| `20260703T095527Z-fadv` | Named advisory lenses make AI advice inspectable, while binding authority stays human. This is a useful explanation of roles without anthropomorphising them as departments. |
| `20260722T200244Z-n4q7` | A poll must fetch, scan all surfaces, and act. The framework still depends on assistants executing the protocol; report-only or stale polling can miss cost-saving instructions. |
| `20260722T182150Z-9qxz` | AGF did not make the technical diagnosis infallible. Review forced an unsupported causal story to be softened and an unnecessary fix component to be removed. |
| `20260624T214609Z-30gz` | An external fit review found the human-authority model and two gates legible, while warning that AGF is not universal and depends on real governor capacity. Preserve the adopt/wait/skip boundary. |
| `20260624T065744Z-k7m2` | One existing adopter upgrade was low-friction. Do not generalise this into "initial adoption is easy"; the evidence covers an upgrade, not a first installation. |
| `20260624T104500Z-rp3h` | The governed work should keep the adopter's domain language. AGF should support the project rather than brand every downstream artifact with AGF terminology. |

## Beginner-facing promise and limitation

Promise only this:

> AGF makes AI-assisted work easier to see, question, approve, and improve.

Do not promise that AGF makes AI correct, watches everything automatically,
delivers every message, secures the code, certifies compliance, or removes the
need for an attentive human. The learnings show the narrower value: mistakes
still happen, but they are more likely to leave evidence that another reviewer
can inspect and turn into an improvement.

## Evidence-strength boundary

These records are mostly foundry dogfooding and single-adopter evidence. Label
the resulting claims **Empirically informed**, not universally or empirically
validated. Prefer "we observed," "in this project," and "helped catch" over
"prevents," "ensures," or "guarantees."

Please include this learning-led candidate in the `mj2g` comparison and make
the final MR !350 synthesis say which parts belong in first-contact copy,
which belong on an evidence/limitations page, and which should remain internal
implementation detail. No public documentation implementation is authorized
by this note.
