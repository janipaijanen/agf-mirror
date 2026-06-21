---
date: 2026-06-21T19:39:00Z
authors: [planner-architect, four-agent assessment panel]
status: active
references-coordination: 20260621T193926Z-planner-architect-ah1n.md
---

# Public documentation site assessment — 2026-06-21

A four-angle assessment of the public docs (25 pages, 7 nav groups) triggered
by the `first-time-docs-session-continuity-review` handoff from coder-tester
(`20260621T192432Z-coder-tester-4sh2.md`). Assessment covered: information
architecture, prose/wording/tone, technical accuracy, and first-timer journey
and benefits.

The action split is in the coordination handoff. This plan file records the
evidence and answers Codex's 7 questions for PA judgment.

---

## Answers to Codex's 7 questions (from the handoff)

**Q1: Should FAQ be a top-level docs node? If not, what page carries that job?**

No — but the current answer (`should-you-adopt.md`) is not surfaced as the
FAQ-equivalent in the nav. The Adopt FAQ (`adopter-faq.md`) is the hidden
gem: it answers pre-commitment questions better than any other page. It should
be renamed or relabeled in the nav from "Adopter FAQ" to "Common questions
before you decide" and linked more prominently from `should-you-adopt.md` and
`index.md`. A top-level FAQ nav entry is not needed if FAQ content is reachable
from the evaluation flow.

**Q2: Should README prefer published docs links over source-doc links?**

Yes. The README's "five-minute path" points into `agf/docs/site/...` source
files. A public evaluator (not a repo contributor) expects polished published
URLs. Fix: put the published site URL first in the README quick-start; use
source-file links only in the "For contributors in a checkout" section.
This is a README fix, not a docs-site fix — out of scope for the current
docs assessment branch.

**Q3: Should Quick reference be visible in the site nav?**

Deferred — out of scope for this assessment. The `agf/docs/reference/
quick-reference.md` file was not in the site build at assessment time. If it
exists and is current, it should be added to the Reference nav section. Verify
its content before promoting.

**Q4: Where does session continuity belong?**

Public Operate page. The pattern (write a closeout note → `agov session resume`
→ read must-read files → continue from files not memory) is a first-class AGF
operation, not a foundry-only convention. It belongs in `docs/operate/` as
`session-continuity.md`, after `day-to-day.md`. The `/compact-agf` Claude
command is a tool-specific convenience that should be mentioned as an example
of the pattern, not as the pattern itself. The pattern is: durable closeout
note plus session resume.

**Q5: Is `/compact-agf` a foundry-local Claude command, an adopter-facing pattern, or both?**

Both, at different abstraction levels. The *pattern* is adopter-facing: write a
durable closeout note to the coordination channel before context loss; cold-start
by running session resume and reading that note. The `/compact-agf` command is a
Claude Code-specific convenience that implements the pattern for Claude Code users.
The public docs should describe the pattern; the Claude Code tool documentation
can reference `/compact-agf` as the convenience implementation.

**Q6: Should compact closeout use plain `kind: note`, or should active-work saves have a machine-detectable topic/schema convention?**

Plain `kind: note` with a conventional topic of `session-closeout-before-compact`
is sufficient for now. The `agov session resume` command already uses this topic
as a resume signal (it appears in the "Must read" output for the relevant role).
A new schema kind would need a validator schema and coordination protocol update;
the cost is not justified by the benefit at this adoption stage. Document the
`topic: session-closeout-before-compact` convention explicitly on the session
continuity page.

**Q7: What must be fixed before this can be presented to first-time humans without expert oral tradition?**

The Slice A items in the coordination handoff. The critical three are the Elixir
residue items (A1–A3) which present false information to adopters. After those,
the prose quick-wins (A4–A9) clean up the reader experience. The structural
items (Slice B nav reorder, first-governed-change file move) improve the journey
but are not embarrassing to a first-timer the way factual errors are.

---

## Full 20-item findings (evidence record)

### Critical — factual errors

**1. `agf/docs/site/index.md` ~L75 — false Elixir fallback claim**
> *"The Elixir Mix reference implementation covers the full command set and is
> always available as a fallback. If the Rust binary is not present, the `agov`
> wrapper falls through to Mix automatically — no configuration required."*

False for release-archive adopters. `reference/release-artifacts.md` confirms
no Elixir escript is published. The Rust binary is the only distributed tooling.

**2. `reference/tooling-conformance.md` — Elixir as reference implementation (inverted)**
The page presents an internal architectural view (Elixir reference, Rust secondary)
as the adopter contract. `agov coord graph` implemented and documented but absent
from the conformance table.

**3. `reference/environment-variables.md` — Mix-relative defaults**
`AGOV_SCHEMA_DIR` default `../../agf/schemas` and `AGOV_TOKEN_ROI_LOG` default
`../../tmp/agov-token-roi.jsonl` are relative to `tooling/elixir/`. `AGOV_ALLOW_ROOT`
mentions "AGF Mix tasks" as primary audience. Foundry-internal context exposed as
adopter-facing defaults.

### Major — information architecture

**4. Concepts sub-group comes after Adopt; Adopt pages assume the mental model.**
Install page (step 5) sends readers to `roles-and-gates.md` for the mental model
after asking them to commission role definitions. The docs index ASCII diagram
labels Concepts "mental model" — which implies it should precede Adopt. Proposed
fix: Start → Concepts → Adopt → Operate → Advanced.

**5. `first-governed-change.md` at `docs/start/` but placed in Adopt nav.**
Page explicitly requires install to be complete. File path says "start"; nav says
"adopt." Fix: move file to `docs/adopt/first-governed-change.md`.

**6. No session continuity page.**
`agov session resume`, compact closeout, cold-start, multi-session handoffs —
scattered across `day-to-day.md` and `tooling-and-tokens.md`. High-frequency
pain for the target audience. New page: `docs/operate/session-continuity.md`.

**7. Coordination protocol absent from docs index Concepts section.**
`coordination-protocol.md` (message kinds table, mermaid diagram, `agov coord graph`)
exists in Concepts nav but not in the `docs/index.md` learning path. Readers
following the documented path will miss it.

### Major — prose / wording

**8. "More precisely" paragraph in `should-you-adopt.md` — worst paragraph in the set.**
Five jargon terms defined parenthetically in one compound sentence, before any
have been introduced. The plain-English paragraph before it is sufficient.
Delete the "More precisely" sentence.

**9. Orphan concepts in `tooling-and-tokens.md`: "policy engine", "budget caps",
"gateway assistants", "chat bridges".**
Each appears once with no referent elsewhere. Appear to be aspirational features
that leaked into operational docs. Remove or move to `future/evolution.md` with
a forward reference.

**10. Claim drift on `index.md` — "six months later" stated twice with different wording.**
Version A (~L89): "...relying on chat history or memory."
Version B (~L112): "...archaeology on commit messages."
Keep Version B (sharper); remove Version A.

**11. Jargon used before definition: "fast track" and "claim".**
"Fast track" first on `index.md` / `should-you-adopt.md`; defined in
`first-governed-change.md` and `roles-and-gates.md`. "Claim" used in
`should-you-adopt.md` fast-track criteria before `coordination-protocol.md`
defines it. Add one-clause gloss at first mention on `index.md`.

### Moderate — benefits and first-timer journey

**12. Blocking-urgent-work objection unanswered.**
Fast-track waiver buried in `first-governed-change.md` step 8. The scenario
"production is down, gate blocks work" has no clear answer anywhere. Deferred.

**13. Manager landing page gap.**
No page speaks primarily to a non-technical manager. "What it costs" on `index.md`
lists overhead without completing the trade. Deferred.

**14. Missing benefits: onboarding acceleration; review consistency as outcome.**
New contributor orientation via explicit rule library not mentioned. "Same rules
every session" implied but not stated as a benefit. Deferred.

**15. `adopter-faq.md` undersold.**
Labeled "Answer commissioning questions" in the nav/CTA. Better: "Common
questions before you decide." Deferred.

**16. `evaluate.md` not linked from primary evaluation flow.**
First-time evaluators reading Home → Should you adopt → (decide) don't encounter
`evaluate.md`. A "not sure yet?" link on `should-you-adopt.md` would close the
gap. Deferred.

### Minor — quick wins

**17. Visual palette and Figures in public Reference are contributor artifacts.**
Remove `reference/palette.md` and `reference/figures.md` from the public nav.
SVGs are already linked inline. Reference should contain only adopter-operational
content.

**18. No visual on the home page.**
Three-box pipeline diagram (plan → Gate 1 → implement → Gate 2 → merge) near
"How it works" would let a scanning visitor understand the model in 2 seconds.
Currently text-only. Deferred pending format decision (mermaid vs SVG).

**19. Exit/decommission nav position disrupts install flow.**
Currently 2nd item in Adopt, between downstream-impact and install. Readers who
have committed to adopting have to scroll past exit to find install. Move to end
of Adopt. (Slice B — pending PA decision on nav reorder.)

**20. "One to two orders of magnitude shorter" claim unverified.**
In `tooling-and-tokens.md`, this measurement is stated without telemetry
citation. Add "(foundry estimate; your channel size will vary)".

---

## Slice assignments summary

| # | Assigned to | When |
|---|-------------|------|
| 1–3 | Codex (Slice A) | Now — pre-release blocker |
| 4 | Codex (Slice B) | After PA nav-order decision |
| 5 | Codex (Slice B) | After PA file-move decision |
| 6 | PA authors → Codex integrates (Slice C) | Deferred |
| 7 | Codex (Slice B) | After PA sign-off (low-risk, likely quick win) |
| 8–11 | Codex (Slice A) | Now |
| 12–16 | Deferred (Slice D) | Post-release |
| 17 | Codex (Slice A) | Now |
| 18 | Deferred (Slice C) | After PA diagram spec |
| 19 | Codex (Slice B) | After PA nav decision |
| 20 | Codex (Slice A) | Now |
