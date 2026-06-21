---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260621T220304Z-coder-tester-mhhr
timestamp: 2026-06-21T22:03:04Z
model: GPT-5
immutable: true
references-to: []
topic: ai-on-behalf-of-human-principal-analysis
requires-ack: true
artifacts_owned:
  - docs/proposals/ai-on-behalf-of-human-principal.md
  - docs/coordination/20260621T220304Z-coder-tester-mhhr.md
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Critically review whether AGF should support AI acting on behalf of a named human principal.
    - Decide whether this should become a proposal, docs FAQ item, schema extension, or deferred research topic.
    - Consider routing the included prompt to Lumo or another external AI reviewer.
  must_read:
    - docs/proposals/ai-on-behalf-of-human-principal.md
  do_not_touch:
    - tooling/elixir/
    - agents-govern/
---

# Handoff: AI acting on behalf of a human principal

I drafted an exploratory proposal note on the "AI works on behalf of person X"
pattern, including pros/cons, architecture, ethical issues, and EU regulatory
context.

Core recommendation in the note:

- support **principal-bound delegation** as an optional pattern;
- reject hidden AI impersonation of a human;
- keep AI role/session, human or service principal, and governor approval as
  separate concepts;
- avoid claiming that metadata equals legal compliance or meaningful human
  review.

The note includes a ready-to-send prompt for Lumo or another external AI
reviewer.

Sources consulted for the EU context:

- European Commission AI Act implementation page:
  <https://digital-strategy.ec.europa.eu/en/policies/regulatory-framework-ai>
- European Parliament Legislative Train on the withdrawn AI Liability
  Directive:
  <https://www.europarl.europa.eu/legislative-train/theme-a-europe-fit-for-the-digital-age/file-ai-liability-directive>
- EUR-Lex GDPR text:
  <https://eur-lex.europa.eu/eli/reg/2016/679/oj/eng>

This is deliberately not legal advice. It is a governance/product design prompt
for AGF.
