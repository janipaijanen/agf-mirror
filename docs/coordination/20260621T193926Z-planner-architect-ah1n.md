---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260621T193926Z-planner-architect-ah1n
timestamp: 2026-06-21T19:39:26Z
immutable: true
references-to:
  - 20260621T192432Z-coder-tester-4sh2.md
topic: docs-assessment-split-proposal
requires-ack: true
artifacts_owned:
  - docs/coordination/20260621T193926Z-planner-architect-ah1n.md
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Review the proposed slice split below and confirm or counter-propose.
    - Once agreed, implement Slice A as a single focused branch (pre-release blocker).
    - Hold Slice B until I confirm the nav ordering decision in a follow-up note.
    - Slices C and D are deferred — do not implement without a separate handoff.
  must_read:
    - docs/coordination/20260621T192432Z-coder-tester-4sh2.md
  do_not_touch:
    - agf/docs/site/docs/start/first-governed-change.md  (Slice B — pending PA decision)
    - agf/docs/build/mkdocs.yml nav ordering  (Slice B — pending PA decision)
---

# Docs assessment: proposed fix split

A four-angle assessment of the public documentation site was run in response
to your `first-time-docs-session-continuity-review` handoff. Twenty findings
across information architecture, prose/wording, technical accuracy, and
first-time reader journey. Full results are in the session transcript (not in
a file; ask the human governor to share if needed).

The findings naturally split into four slices by who decides and who
implements.

---

## Slice A — Codex implements now (pre-release blockers)

These are well-scoped mechanical edits with no architectural judgment needed.
They block release because items 1–3 are factual errors.

### A1. Elixir residue in `index.md`

Around line 75, remove or replace the paragraph claiming an automatic
Elixir fallback wrapper:

> *"The Elixir Mix reference implementation covers the full command set
> and is always available as a fallback. If the Rust binary is not present,
> the `agov` wrapper falls through to Mix automatically — no configuration
> required."*

This is false for any adopter using the release archive (no Elixir toolchain).
Replace with a single-sentence note that the Rust binary is the only
distributed tooling surface; source-checkout users with a full Elixir checkout
can use Mix commands directly. Keep it short — the surrounding tooling
description already covers this adequately.

### A2. `reference/tooling-conformance.md` — Elixir as reference implementation

The entire page presents Elixir as the primary implementation and Rust as a
secondary binary, with a `mix agov.*` command column. This is the internal
architectural view, not the adopter-facing contract.

Rework the page to describe the Rust binary as the adopter's surface.
Remove the Elixir Mix command column or move it to a developer/contributor
note clearly scoped to foundry checkout users. Also: add `coord graph` to
the command table — it is implemented and documented in `coordination-protocol.md`
but absent from the conformance page.

### A3. `reference/environment-variables.md` — Mix-relative defaults

`AGOV_SCHEMA_DIR` default (`../../agf/schemas`) and `AGOV_TOKEN_ROI_LOG`
default (`../../tmp/agov-token-roi.jsonl`) are relative to `tooling/elixir/`
and won't resolve for a release-binary adopter. `AGOV_ALLOW_ROOT` description
mentions "AGF Mix tasks" as the primary audience.

Fix: either state the correct defaults for a binary adopter (likely empty /
no default, or relative to the repo root), or note that defaults are
development-context values and adopters should set these explicitly.

### A4. Prose — delete "More precisely" paragraph in `should-you-adopt.md`

The paragraph beginning "More precisely: it gives AI coding agents explicit
role definitions, Gate 1..." introduces five jargon terms parenthetically in
one compound sentence, before any of them have been defined. The paragraph
immediately before it gives a cleaner plain-English explanation and is
sufficient. Delete the "More precisely" sentence.

### A5. Prose — remove orphan concepts in `tooling-and-tokens.md`

Two concept clusters appear once with no referent elsewhere in the docs:
- "gateway assistants", "chat bridges" (in the install-guide subsection)
- "policy engine", "budget caps" (in the "Why meter at the governance gate" section)

These appear to be aspirational features that leaked into operational
documentation. Remove them. If either concept is planned and has a home in
`future/evolution.md`, a forward reference to that page is acceptable; an
unexplained in-line claim is not.

### A6. Prose — cut duplicate "six months later" claim on `index.md`

The audit trail benefit is stated twice on the same page with different
wording. Keep one canonical statement; remove the other.

> Version A: *"...by reading the chain instead of relying on chat history or memory."*
> Version B: *"...by reading the chain, not by archaeology on commit messages."*

Version B is sharper. Keep it; remove Version A.

### A7. Prose — add one-clause gloss for "fast track" and "claim" at first use

"Fast track" first appears on `index.md` and `should-you-adopt.md` without
definition. "Claim" (the coordination artifact kind) appears in
`should-you-adopt.md` fast-track criteria without prior context.

At the first use of "fast track" on `index.md`, add a parenthetical or
inline gloss: "(trivial edits of three files or fewer, no shared interface
change)." The full criteria appear in `roles-and-gates.md` and
`first-governed-change.md`; this is just enough for the reader to not be
confused.

For "claim," the first use on `should-you-adopt.md` can read: "no agent
holds an active claim (an exclusive reservation of a task in the coordination
channel)."

### A8. Reference nav — remove Visual palette and Figures

`reference/palette.md` and `reference/figures.md` are documentation
contributor artifacts (color tokens, SVG catalog). They are not useful to
framework adopters. Remove both from the mkdocs nav. The SVGs are already
linked inline where needed.

### A9. Numbers — frame "orders of magnitude" claim as foundry estimate

In `tooling-and-tokens.md`, the claim that `agov coord brief` output is
"one to two orders of magnitude shorter than reading the underlying
coordination files with an AI agent" is stated as a measurement. It should
read as a foundry estimate: add "(foundry estimate; your channel size will
vary)" after the figure.

---

## Slice B — PA decides ordering first, then Codex implements

These require a planner-architect architectural call before any file moves
or nav changes. I will post a follow-up note with decisions once I've worked
through them. Do not start these until that note arrives.

Pending decisions:

- **Nav order** — should Concepts move before Adopt? The assessment finding
  is that Adopt pages assume the mental model that Concepts teaches. My
  current inclination is yes (Start → Concepts → Adopt → Operate → Advanced),
  but this requires updating the docs index ASCII diagram and section
  descriptions simultaneously. I'll confirm.

- **`first-governed-change.md` file location** — the file is at
  `docs/start/first-governed-change.md` but placed in the Adopt nav group.
  Options: move the file to `docs/adopt/` (nav and URL consistent), or move
  it in the nav to Start (simpler, no file move, but path still says "start").
  I'll decide.

- **Exit/decommission nav position** — currently 2nd item in Adopt, which
  disrupts the install flow for committed adopters. Options: move to end of
  Adopt, or as a sub-item of downstream-impact. I'll decide.

- **Coordination protocol in docs index** — small: add a line for
  `coordination-protocol.md` to the Concepts section of `docs/index.md`.
  This is low-risk and I'll probably include it in my follow-up as a
  confirmed quick win.

---

## Slice C — PA authors, deferred

These require new content that the planner-architect will draft or outline
before Codex implements. Not blocking the current release.

- **Session continuity page** (`docs/operate/session-continuity.md`) —
  compact-before-context-loss, cold-start protocol, `agov session resume`,
  multi-session handoffs. I will write a draft or detailed outline; Codex
  refines and integrates into the nav.

- **Home page visual** — a simple three-box pipeline diagram near "How it
  works." Needs a decision on format (mermaid vs SVG from the figures catalog)
  and exact placement. I'll propose the diagram spec.

---

## Slice D — Deferred, not blocking release

These are benefits improvements and CTA refinements that are good-to-have
but not errors:

- Blocking-urgent-work objection unanswered (emergency fast-track scenario
  not documented anywhere)
- Manager landing page gap (no page speaks primarily to a non-technical manager)
- Missing benefits: onboarding acceleration, review consistency as an outcome
- `adopter-faq.md` undersold ("Answer commissioning questions" → "Common
  questions before you decide")
- `evaluate.md` not linked from primary evaluation flow (should-you-adopt.md
  needs a "not sure yet?" link)

---

## Proposed branch structure

For Slice A, I suggest a single branch (`docs/fix-pre-release-blockers` or
similar) with all nine edits, since they are all docs-only prose changes that
touch different files and do not interact. Gate 2 for the combined branch will
be fast — no logic changes.

Counter-proposal welcome. If you prefer to split Slice A into "Elixir residue"
and "prose fixes" as separate branches, that works too.

Please confirm:
1. Slice A scope is correct and complete (or propose additions/removals)
2. Preferred branch structure for Slice A
3. Any findings from your `first-time-docs-session-continuity-review` handoff
   that you think I've miscategorized

I'll post the Slice B decisions as a follow-up note to this handoff.
