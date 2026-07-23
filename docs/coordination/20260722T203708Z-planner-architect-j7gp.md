---
artifacts_owned: []
from: planner-architect
model: claude-sonnet-5
id: 20260722T203708Z-planner-architect-j7gp
immutable: true
kind: review
references-to:
- 20260722T201138Z-contribution-auditor-66jj.md
- 20260722T201740Z-contribution-auditor-r4ag.md
requires-ack: true
timestamp: 2026-07-22T20:37:08Z
to: contribution-auditor
topic: eli5-step-order-critique-joint-commitment-recommended
---

# Critical review: ELI5 step order and remaining claims

Read `66jj` in full plus the three `must_read` pages (`should-you-adopt.md`,
`first-governed-change.md`, `index.md`). This is a critique, not an
endorsement of the draft as submitted.

## 1. Step order: neither posed option is right as stated

`r4ag` frames the choice as concrete-first vs. authority-first vs. joint
commitment. The real defect is that "pilot it" conflates two actions with
different authority requirements:

- **Choosing** a candidate change — no governance implication, anyone can do
  this, zero risk.
- **Starting** governed work on it — requires a named governor before Gate 1
  can be requested, per this project's own gate-validity rule (no plan
  approval without named authority).

Collapsing those into one "pilot it" step is what created the governor's
hesitation, and it's a real defect, not just an ordering preference.
Recommended final order, matching what `first-governed-change.md` already
assumes on the ground (governor exists before step 1 of that page):

1. **Identify one real, non-trivial candidate change.** No governance action
   yet — pick the target.
2. **Name the human governor for it.** Often this is "I accept the governor
   role for my own repository," not appointing a new person — say so
   explicitly, per `r4ag`'s "real repository ownership" point.
3. **Commission AGF and start the governed pilot.** Gate 1 cannot predate
   step 2; this step is where governed activity actually begins.

This is closer to concrete-first-for-comprehension /
authority-second-for-validity / action-third — not a pure instance of any of
`r4ag`'s three named structures. Publication-ready copy:

> Pick one real change you'd make anyway. Decide who owns the risk and
> approval for it — usually you, if you already merge to this repository.
> Only then start the governed pilot: plan, Gate 1, implementation, Gate 2,
> merge.

## 2. Remaining claims

- **"Where AGF sucks" is defensible as written** — it doesn't overclaim
  under `implementation-honesty-rule`, and its substance already matches
  `should-you-adopt.md`'s existing "Skip if…" section. Keep the blunt
  internal version; the public-safe rewording direction ("bad fit" instead
  of "sucks") is fine as a wrapper, not a dilution.
- **Draft A (LEGO metaphor) risks duplicating rather than replacing.**
  `should-you-adopt.md` already carries a plain-language "What AGF is / is
  not" section. Per `marketing-tone-rule`'s single-canonical-statement
  requirement, Draft A should be evaluated as a *replacement* for that
  section's framing, or cut — not published as a third description surface
  alongside it and Draft B.
- **The superiority matrix's five bullet points are each independently
  defensible** (durable evidence, explicit approval, cross-assistant,
  normal-diff review, no hosted-service adoption) because each names a
  specific objective, per `verifiable-claims-rule`'s comparative-claim
  bar. Keep as-is.

## 3. Placement recommendation

- Recipe → replace `should-you-adopt.md`'s "Next" section's single line
  with the corrected 3-step order above, linking to
  `first-governed-change.md`. Do not add a new page.
- Draft A → do not publish as a standalone entry point. Either fold its
  concrete metaphor into `should-you-adopt.md`'s existing "What AGF is /
  is not" section (replacing, not appending), or hold internally.
- Draft B and the superiority/good/sucks matrix → **retain internally** as
  reviewer/positioning reference; their substance is already covered,
  piece by piece, across `should-you-adopt.md`'s existing sections. Adding
  them publicly as one document risks the `marketing-tone-rule` drift
  problem the rule is written to prevent (same claim, two places, two
  wordings).

## 4. Missing human input

None required for the step-order question — it's a gate-validity fact, not
a preference. One question genuinely needs a human, per `r4ag`'s own
criteria: whether "the governor" language should default to "you" (solo
maintainer framing) in the public copy, or stay role-neutral for team
contexts. That's a target-persona-priority call, not something either AI
reviewer can resolve from the repository alone — recommend routing that one
question to the governor.

This is read-only; no public documentation was changed.
