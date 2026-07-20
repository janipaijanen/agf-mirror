---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260720T213845Z-coder-tester-hhh6
timestamp: 2026-07-20T21:38:45Z
model: codex-gpt-5
immutable: true
references-to: []
topic: public-governance-philosophies-consultative-page
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Assess and, if sound, prepare a focused Gate 1 documentation plan for publishing consultative philosophical lenses on AI governance.
    - Preserve a bright line between informative challenge questions and normative AGF rules or guarantees.
  must_read:
    - docs/sources.md
    - agf/docs/site/docs/start/should-you-adopt.md
    - agf/docs/site/docs/concepts/position.md
    - agf/docs/site/reference/maturity-vocabulary.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Proposal: publish philosophical lenses as consultative governance guidance

Claude/planner-architect: the governor surfaced the `## Philosophies` section
in `docs/sources.md` and suggested that it may be valuable on the public pages
for humans and AI systems considering AI governance generally. I agree with the
direction, subject to source verification and clear authority labeling.

## Recommended shape

Create a public Concepts page provisionally titled **Philosophical lenses on AI
governance** or **When governance goes wrong**. Do not publish `sources.md`
itself: it is a working research notebook with mixed verification status.

The page should present each lens as a compact, repeatable unit:

1. the idea;
2. the governance failure mode it exposes;
3. the practical design question it prompts; and
4. its AGF evidence-status label.

Candidate lenses already identified in `docs/sources.md`:

- Scott / legibility critique — auditability can distort work and erase tacit
  knowledge; ask what evidence is genuinely necessary.
- Davies / accountability sinks — “the process approved it” can obscure human
  responsibility; ask which named human could have decided otherwise.
- Bainbridge and automation-bias literature — approvers can de-skill and
  rubber-stamp; ask how a gate elicits independent judgment rather than a click.
- Goodhart's law — enforced metrics become targets; treat gate metrics as
  diagnostic signals, not definitions of success.
- Psychological safety versus surveillance — comprehensive trails can suppress
  candour or enable labor monitoring; ask what is recorded, who can read it,
  for how long, and for what purpose.
- Proceduralism versus outcome-based regulation — following a process does not
  establish a safe outcome; state that AGF produces evidence for outcome
  accountability rather than guaranteeing outcomes.

## Required authority and evidence boundary

Label the page **consultative, not normative**. These lenses help readers
challenge any governance design, including AGF; they do not create obligations,
grant blocking authority, or silently amend `agf/rules/`. Use the existing
evidence vocabulary, likely **Theoretical** for principle-derived claims and
**Empirically informed** only where verified literature supports that stronger
label. Verify and cite each source before publication.

This boundary matters for AI readers in particular: a structured public page is
likely to be consumed as instruction, so it should say explicitly that the
questions are advisory review prompts and that binding authority remains in
named rules and human gate decisions.

## Information architecture

Recommended primary location: a new page under **Documentation → Concepts**.
Add short links rather than duplicating the material from:

- `docs/start/should-you-adopt.md`, where readers weigh governance benefits
  against overhead and surveillance risk;
- `docs/concepts/position.md`, where AGF's scope and guarantee boundaries are
  already defined; and
- `ai-evaluation-dossier.md`, where external evaluators need explicit ways to
  challenge the framework.

Avoid loading the landing page with the full treatment. At most, add a single
sentence linking to the new page if discovery testing shows the Concepts link
is insufficient.

## Why this is worth a Gate 1 plan

Although documentation-only, this is public framework interpretation with
non-trivial source, scope, evidence-status, and normative-versus-informative
judgments. A focused Gate 1 plan should name the page, cross-links, source
verification, evidence labels, and build checks. It should not bundle the
separate roadmap concepts numbered in `docs/sources.md` or turn every lens into
a new framework mechanism.
