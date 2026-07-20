---
id: docs-audience-boundary-proposal
status: approved
topic: docs-audience-boundary-proposal
gate1_approved_by: "governor-reviewer; Codex-scribed decision in 20260720T143255Z-governor-reviewer-s6hg.md"
gate1_approved_at: "2026-07-20T14:32:55Z"
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: documentation-only taxonomy and audience-label proposal; no secrets, auth, or security boundary changes
  - role: contribution-auditor
    status: consulted
    reason: proposal reclassifies public-doc audience labels and page taxonomy; needs a scope/overstatement check before Gate 1, per the requesting handoff's explicit recommendation
  - role: fact-checker
    status: consulted
    reason: proposal asserts specific counts (page inventory, per-page Audience-line text) and status of two prior docs plans; needs factual verification before Gate 1
---

# Plan: Documentation audience-boundary proposal
#
# Timestamp: 2026-07-20T13:03:05Z
# Status: approved
# References: docs/coordination/20260720T125518Z-coder-tester-kpmn.md (requesting handoff)
# Non-goal: this plan proposes a taxonomy and a small-MR implementation split.
# It does not rewrite the documentation site.

## Problem statement

**Correction (2026-07-20, before Gate 1 review, twice): the first version of
this plan miscounted the nav, and the first self-correction was still
imprecise about which population it was counting.** The plan originally
claimed "32 nav pages" and "zero pages acknowledge an AI agent as executor."
A same-session self-review fixed the "32"/"zero" errors but introduced a new
imprecision (conflating "has a `*Status:*` line" with "has an `Audience:`
clause," which are different questions). An independent fact-checker pass
(Codex, separate session — see
`docs/coordination/20260720T135454Z-fact-checker-ag3k.md`) caught this and
was itself re-verified line-by-line before writing the numbers below.
**Method, stated explicitly per the fact-checker's request:** search the
first 20 lines of every nav page, case-insensitive, for the literal string
`audience:`, regardless of whether it appears on a line starting `*Status:`
or in a differently-formatted intro paragraph (`contribute.md` opens
`*Audience: ...*` with no "Status:" at all; `future/evolution.md`'s
`Audience:` clause is on the third line of a wrapped `*Status:*` sentence —
both were missed by the earlier single-line-anchored grep).

- The site has **34** nav pages.
- **22** of the 34 carry an `Audience:` clause in their first 20 lines; the
  other **12** carry none.
- Of those 22: **two already name "AI assistants"** alongside human roles —
  `docs/operate/run-agf-in-real-situations.md` ("Audience: governors,
  developers, and AI assistants operating an AGF-governed project") and
  `docs/start/framework-map.md` ("Audience: evaluators, adopters, and AI
  assistants that need a quick orientation before reading the detailed
  pages"). In this session's assessment, `run-agf-in-real-situations.md` is
  among the most AI-operator-oriented pages in the site — it opens "Run
  these before asking an assistant to act in a governed repository."

**The corrected claim, still the load-bearing finding:** the human role is
listed *first* in both of those two lines, and "AI assistants" reads as an
appended tertiary audience, not a reframing of who primarily executes the
page's procedure — neither page says "written for the AI agent; human
supervises." So the governor's underlying complaint still holds, precisely
stated this time: **20 of the 22 audience-labeled pages name only human
roles, and the 2 that do mention AI assistants still list the human role
first and don't flip the responsibility framing.** Corrected counts:

**Third correction (same review cycle): the phrase-count table below was
still stale after the 22/20/2 headline fix** — it carried the original,
first-pass per-phrase tallies (9/3/3/5/4/2), computed before the `Audience:`
clause search was widened to catch `contribute.md` and `future/evolution.md`.
Recounted manually, phrase by phrase, across all 22 audience-bearing pages —
disambiguating audience-noun uses of "evaluat-" (e.g. "evaluators.") from
verb uses describing an activity (e.g. "developers **evaluating** how AGF
fits," which names *developers* as the audience, not a separate "evaluators"
category):

| Audience phrase as written | Page count (of 22 audience-bearing pages) |
|---|---|
| "developers" (alone or combined) | 12 |
| "administrators" | 7 |
| "governors" | 7 |
| "evaluators" (as the audience noun; excludes verb uses like "developers evaluating") | 4 |
| "adopters" | 4 |
| "AI assistants" (always appended after a human role, never primary) | 2 |
| Non-standard phrasing naming neither a role noun nor "AI assistants" — `docs/start/adopter-faq.md` ("people deciding whether to commission AGF...") and `future/evolution.md` ("anyone evaluating long-term direction") | 2 |

(Rows overlap: most pages combine 2-3 roles, e.g. "evaluators and
administrators," so the phrase-count rows sum to 38 across 22 pages, not to
22 or 34. The 22 "has an Audience clause" figure is the exhaustive
population, verified by direct `head -20 <page> | grep -i audience:` over
all 34 nav pages, not by summing
this table.)

Yet the framework's own operating model — stated explicitly in
`docs/operate/day-to-day.md`'s "three responsibility modes" table and
figure (`operating-responsibility-map.svg`) — assigns most of the concrete
procedural content in these same "developer"-audience pages to **AI-executed
work**: running `agov plan validate`, writing YAML frontmatter, `git checkout
-b`, filing coordination messages. Two concrete examples:

- `docs/start/first-governed-change.md` (Audience: "developers who have
  completed install") mixes one AI-delegation pattern that works well
  ("Paste this into your coding assistant:" + a literal prompt block) with
  plain second-person procedural steps — `git checkout -b
  fix/sample-login-error-message`, `$EDITOR docs/coordination/<id>.md` — with
  no marker distinguishing "you personally type this" from "your assistant
  does this and you review the diff." A reader cannot tell which steps are
  theirs.
- `docs/operate/day-to-day.md` has the exact responsibility-mode breakdown
  this proposal needs — but it sits at path position 4 of 6 in the site's own
  stated learning order (Start → Concepts → **Adopt** → **Operate** →
  Advanced → Evolution). A first-time reader forms their impression of "how
  much do I personally have to do" from Start/Concepts/Adopt, three stages
  before the page that actually answers the question.

This is not a claim that the site never explains the human/AI boundary — it
does, well, in one place, late. The gap is that **the boundary explanation
is not surfaced early, and the per-page audience labels actively contradict
it** by naming a human role for content that is majority AI-executed
procedure.

## Relationship to prior, unresolved docs plans

Two earlier Gate 1-adjacent artifacts already cover adjacent ground and
should not be duplicated or silently ignored:

1. **`docs/plans/agents-govern-2.0-pedagogical-docs-restructure.md`**
   (`status: pending-gate1`, opened 2026-06-13, never resolved). Scope:
   reading-order and progressive-disclosure (Concepts before Adopt, figure
   first-reference prose, page moves). Spot-checked against current main:
   the Concepts-before-Adopt reorder is **already live** (`docs/index.md`'s
   path diagram matches its proposal exactly), and `roles-and-gates.md`
   already embeds first-reference figures. This plan's own decomposition
   table was never marked done and its four Gate 1 questions were never
   answered. **Recommendation: governor closes this plan explicitly —
   either superseded-by this proposal (if remaining items get folded in) or
   completed-via-untracked-work (if a decision is made that the remaining
   items are done and just need a completion entry).** Leaving it open
   indefinitely misrepresents work as outstanding that mostly already
   happened.
2. **`docs/plans/docs-assessment-2026-06-21.md`** (`status: active`, opened
   2026-06-21, 20 findings across four review angles). Spot-checked: items
   4, 17, and 20 are done in current main; item 6 (a dedicated
   `docs/operate/session-continuity.md` page) is still open and directly
   relevant to this proposal's "AI-operator instructions" bucket (session
   continuity — closeout notes, `agov session resume`, cold-start — is
   almost entirely AI-executed procedure that currently has no dedicated
   home). **Recommendation: governor reconciles the remaining open items
   (6, 12-16, 18, 19) explicitly — fold item 6 into this proposal's Task
   list below rather than tracking it twice, and either close or explicitly
   re-scope the rest.**

This proposal's distinct contribution is a third axis neither prior artifact
addressed as its primary concern: not reading *order*, not factual
*accuracy*, but **who the page assumes will act**, and whether that
assumption matches the framework's own responsibility model.

## Responsibility boundary matrix

| Actor | Owns / decides | Executes | Never does |
|---|---|---|---|
| **Human** (governor, or named reviewer role) | Gate 1/Gate 2 approval, merge decisions, risk acceptance, adopt/exit decision, role activation | Reads plans and diffs, writes decision records, revokes delegation | Cannot be replaced by an AI signature; an AI-authored "approval" is non-binding per `roles-and-gates.md` |
| **AI agent** (planner-architect, coder-tester, etc.) | Drafting: plans, docs, coordination messages, tests | Runs `agov`/git/CI commands, implements scoped changes, prepares Gate 2 evidence | Cannot approve its own Gate 1 or Gate 2, cannot accept risk, cannot activate/deactivate roles |
| **Deterministic tooling / CI** (`agov`, pre-commit, pipeline jobs) | Nothing — no decision authority | Validates schema shape, blocks malformed records, runs tests | Cannot decide "is this a good plan," only "is this a well-formed one" |
| **Repository artifact** (plan file, coordination message, decision record) | N/A | Is the durable record | Cannot be edited after the fact — corrections are new entries |

This matrix is the reference table the "shared conceptual pages" bucket
below should surface early (Concepts, not Operate), and the check every
procedural page should be written against: for each instruction, is the
actor a human or an AI agent, and does the page say so?

## Proposed documentation taxonomy

Four buckets, applied to every current nav page. "Inferred" rows are
classified from the page's title, path, and existing `*Status:*` line only
(not a full read this session) — flagged explicitly for confirmation during
implementation, not asserted as verified.

| Bucket | Definition | Primary reader |
|---|---|---|
| **H — Human decision** | Answers "should I / how do I decide" — no procedure | Human evaluator/governor |
| **A — AI-operator instructions** | Majority procedural content an AI agent executes; human supervises | AI agent (human reads to supervise) |
| **C — Shared concept** | Mental model both a human and an AI agent need to coordinate | Both |
| **R — Reference** | Command/schema/env-var/maturity precision, read after the concept is clear | Both, as a lookup |

| Page | Current audience label | Read this session? | Proposed bucket |
|---|---|---|---|
| `index.md` (Home) | none | no | H — landing/decision |
| `use-cases.md` | (has one, not shown above) | no | H |
| `evaluate.md` | none | no | H |
| `ai-evaluation-dossier.md` | none | no | R — third-party AI-evaluator artifact, not operator instructions; confirm during implementation |
| `docs/index.md` (learning path) | none | **yes** | C — this is the map itself; candidate home for the new documentation-map diagram |
| `docs/start/framework-map.md` | evaluators, adopters, **and AI assistants** | **yes** (verified during correction above) | C — already partially correct; human role still listed first, worth reordering or rephrasing rather than reclassifying |
| `docs/start/should-you-adopt.md` | evaluators | **yes** | H |
| `docs/start/adopter-faq.md` | people deciding | no (inferred) | H |
| `docs/concepts/position.md` | developers | **yes** | C |
| `docs/concepts/roles-and-gates.md` | developers and governors | **yes** | C — already good; candidate home for the responsibility matrix above |
| `docs/concepts/coordination-protocol.md` | developers and governors | partial (diff only) | C |
| `docs/concepts/evidence-and-learnings.md` | developers | no (inferred) | C |
| `docs/adopt/downstream-impact.md` | evaluators and administrators | **yes** | H — already good, clean audience match |
| `docs/adopt/install-and-layouts.md` | administrators | partial (diff only) | **Mixed → split.** Layout decisions (H) vs. file-by-file commissioning steps (A) |
| `docs/adopt/assistant-loader-configuration.md` | developers | no (inferred) | A |
| `docs/start/first-governed-change.md` | developers | **yes** | **Mixed → split.** See problem statement above; needs an explicit human-vs-assistant marker per step, not a bucket move |
| `docs/adopt/upgrade-from-v1.md` | adopters | no (inferred) | A |
| `docs/adopt/tooling-and-tokens.md` | developers | partial (diff only) | A |
| `docs/adopt/mcp-server.md` | adopters running MCP assistants | **yes** | A — already good: states maturity, disclaims authority explicitly |
| `docs/adopt/exit-and-decommission.md` | evaluators and administrators | no (inferred) | H |
| `docs/operate/day-to-day.md` | developers and governors | **yes** | C — has the responsibility-mode table this whole proposal leans on; under-promoted, not mis-labeled |
| `docs/operate/run-agf-in-real-situations.md` | governors, developers, **and AI assistants** | **yes** (verified during correction above) | A — already among the better-labeled AI-operator pages in the site; candidate reference example for MR-2's relabeling, not a page that itself needs fixing |
| `docs/operate/review-and-release.md` | governors | no (inferred) | H |
| `docs/advanced/contracts-and-validators.md` | administrators | partial (diff only) | A/R mixed — already flagged for a possible split in the 2026-06-13 pedagogical plan; this proposal does not re-open that decision |
| `docs/advanced/customizing-the-framework.md` | administrators | no (inferred) | R |
| `reference/*.md` (7 pages) | none/generated/draft | no | R — all seven, no change expected |
| `contribute.md` | **"Audience: adopters who have found something worth sharing..."** — missed in the first correction; opens `*Audience: ...*` with no "Status:" prefix at all, a different convention than every other page | **yes** (earlier this session) | H |
| `future/evolution.md` | **"...Audience: anyone evaluating long-term direction."** — missed in the first correction; the clause is on line 3 of a wrapped `*Status:*` sentence | no (inferred) | H — explicitly non-binding, already well-labeled |

Net picture: roughly a third of pages are already well-classified
(`roles-and-gates.md`, `downstream-impact.md`, `mcp-server.md`,
`evolution.md`, and — per the correction above — `run-agf-in-real-
situations.md`, a good existing example of an AI-inclusive audience
line); the highest-value fix is not moving pages between nav groups but
(a) adding the missing "AI agent executes, human supervises" audience label
to the ~9 pages currently mislabeled "developers" for majority-procedural
content (using `run-agf-in-real-situations.md`'s phrasing as the template,
reordered so the AI-executor role isn't always listed last), and (b)
splitting the two confirmed mixed pages (`first-governed-change.md`,
`install-and-layouts.md`).

## Acceptance-criteria coverage marker convention

To let a reviewer (or CI) check that the site actually answers every bullet
in the requesting handoff's "Acceptance criteria" section, without adding a
new schema, validator subcommand, or build step:

- Each section that answers one of the handoff's acceptance-criteria bullets
  carries an inline HTML comment immediately before it, e.g.:

  ```markdown
  <!-- audience: A (ai-operator); answers: "what agov validates mechanically
       and what it cannot decide" -->
  ## Validating your plan
  ...
  ```
- HTML comments in Markdown pass through to the rendered HTML source but are
  never displayed by a browser — there is nothing to actively strip before
  publishing. This is deliberately not a CI stripping step: an active
  strip pass is one more thing that can silently break the build for a
  problem comment-invisibility already solves for free.
- Coverage is checked by `grep -n '<!-- audience:' <page>.md` at review time,
  not by an `agov` validator. If per-page acceptance-criteria coverage needs
  machine enforcement later, that is a new schema and subcommand — worth
  proposing only once the taxonomy itself is in use, not built speculatively
  ahead of need.
- The marker is a contributor/reviewer aid, not reader-facing content — it
  does not replace the page's own `*Status:*`/`Audience:` line, which stays
  reader-visible prose at the top of the page.

This convention applies wherever MR-2 and MR-3 (audience relabeling and the
first-governed-change per-step split) add or touch a section; it is not a
retroactive requirement for pages this proposal doesn't touch.

## Recommended nav/page-map update

No structural nav reorder is proposed — the pedagogical-restructure plan's
Concepts-before-Adopt fix already landed. Two additions:

1. Add a short "How to read this site" callout near the top of
   `docs/index.md` stating the four-bucket taxonomy in one paragraph, before
   the existing stage diagram, so the audience question is answered before
   a reader picks an entry point — not after Operate.
2. Add the missing `docs/operate/session-continuity.md` page (still open
   from `docs-assessment-2026-06-21.md` item 6), classified **A**, since
   session-continuity procedure (closeout notes, `agov session resume`,
   cold-start) is exactly the kind of AI-executed content this taxonomy
   exists to label correctly from the start rather than retrofit later.

## Suggested rewrites, split into small implementation MRs

| MR | Scope | Depends on |
|---|---|---|
| MR-1 | Add the "How to read this site" taxonomy callout to `docs/index.md` | none |
| MR-2 | Add explicit `Audience:` reclassification (append "— written for your AI agent to execute; you supervise and review" where applicable) to the ~9 pages in the table above tagged **A** | MR-1 |
| MR-3 | Split `docs/start/first-governed-change.md`: add a per-step marker (human decision vs. assistant-executed) to each of its 8 numbered steps | none, can run parallel to MR-1/2 |
| MR-4 | Split `docs/adopt/install-and-layouts.md`: pull the layout-choice decision content into a short lead section separate from the file-commissioning steps | none |
| MR-5 | New `docs/operate/session-continuity.md`, closing `docs-assessment-2026-06-21.md` item 6 | MR-1 (taxonomy must exist first so the new page is labeled correctly at creation) |
| MR-6 | Reconcile and close `agents-govern-2.0-pedagogical-docs-restructure.md` and remaining open items in `docs-assessment-2026-06-21.md` with coordination completion entries | independent, can run any time |

Each MR is reviewable as its own yes/no question per the branch-and-worktree
sizing test, per this project's own rule.

## Recommended agent work split

This split is conditional on advisory review and final Gate 1 approval. It
does not authorize implementation before the governor-reviewer files the
binding approval. The AGF role named in coordination remains authoritative;
the assistant service names below only describe practical execution capacity.

| Phase | Primary service / role | Work | Boundary |
|---|---|---|---|
| Advisory scope review | Claude in a separate `contribution-auditor` role/session, if available | Check adoption-claim overstatement, scope hygiene, and whether folding prior docs plans into this plan is cleaner than parallel tracking | Files advisory `review`; does not approve Gate 1 |
| Advisory fact check | Codex or a separate AI session as `fact-checker`; if Claude is used, do not reuse the same planner session that authored the plan | Verify page counts, audience-label evidence, prior-plan status, and figure/source claims | Files advisory `review`; does not approve Gate 1 |
| Plan stewardship | Claude as `planner-architect` | Incorporate advisory findings, keep MR !227 coherent, and route blockers back to governor-reviewer | Cannot approve its own plan |
| MR-1 | Claude as `coder-tester`, with planner handoff context | Draft and implement the early taxonomy callout plus the new documentation-audience map; this is the most concept-heavy first implementation slice | Requires Gate 2 review before merge |
| MR-2 | Codex as `coder-tester` | Perform the grep-driven audience-line relabeling on pages tagged **A**, excluding pages intentionally handled in MR-3 to avoid file conflicts | Requires Gate 2 review before merge |
| MR-3 | Claude as `coder-tester` | Rewrite `first-governed-change.md` with per-step human-vs-assistant execution markers and undergraduate-level prose | Requires Gate 2 review before merge |
| MR-4 | Codex as `coder-tester` | Split `install-and-layouts.md` into human layout decisions versus AI-executed commissioning steps, with local docs validation | Requires Gate 2 review before merge |
| MR-5 | Claude as `coder-tester` | Draft the new `session-continuity.md` page, because it needs careful explanation of AI session resume and cold-start behavior | Requires Gate 2 review before merge |
| MR-6 | Codex as `coder-tester` | Reconcile and close prior docs plans with coordination completion records and validation evidence | Requires Gate 2 review before merge |

If one service is unavailable, reassign by coordination note before starting
the affected MR. Do not let two agents edit the same page in parallel without
a fresh claim or explicit split.

## Diagram requirements

Two of the four diagrams the requesting handoff asks for **already exist and
are fit for purpose** — reuse, do not regenerate:

- **Human/AI/tooling responsibility swimlane** → `operating-responsibility-map.svg`
  already exists and matches the ask closely. Action: promote it to
  `docs/index.md` (referenced early) in addition to its current home in
  `day-to-day.md`, not a new figure.
- **Adoption flow with human decision points vs. AI/tooling clerk work** →
  `adoption-exit-flow.svg` already exists; confirm during implementation
  that it visually distinguishes decision points from clerk work (not
  verified this session — a fact-checker or visual-inspection pass should
  confirm before assuming it satisfies the ask as-is).

Two are genuinely new:

- **Documentation map by audience and use** — no existing figure classifies
  pages by the four-bucket taxonomy above. New DOT source under
  `agf/docs/figures/src/`, rendered to `docs-audience-map.svg`, placed on
  `docs/index.md` next to the taxonomy callout (MR-1).
- **Gate 1/Gate 2 lifecycle diagram labeling the owner of each action** —
  `gate-pipeline.svg` exists but (per this session's read of
  `roles-and-gates.md`) shows the pipeline stages, not who acts at each
  stage. Governor-reviewer direction on 2026-07-20 is to try extending the
  existing DOT source with per-node owner labels first; add a companion figure
  only if extension makes the diagram crowded or misleading.

## Validation criteria

1. `mkdocs build -f agf/docs/build/mkdocs.yml --strict` passes after every MR.
2. `dot -Tsvg` regeneration for any new or modified DOT source, committed
   alongside the source per the existing figures.md convention.
3. Grep-based check per MR-2: every page tagged **A** in this plan's table
   carries an audience line naming the AI agent as executor, not only a
   human role.
4. Grep-based check per MR-2/MR-3: every new or touched section carries its
   `<!-- audience: ...; answers: "..." -->` marker (see the coverage-marker
   convention above), and the union of `answers:` values across the site
   covers every bullet in the requesting handoff's acceptance criteria.
5. Visual inspection (per `visible-artifact-verification-rule`) of the new
   `docs-audience-map.svg` and any modified `gate-pipeline.svg` before
   merge — a generated diagram is exactly the kind of artifact that rule
   requires a human or image-capable-model look, not just a strict build.

## Risks

- Reclassifying audience lines on ~9 pages touches wording other plans
  (`docs-assessment-2026-06-21.md` items 8-11) already flagged for prose
  cleanup — sequence MR-2 after checking those items don't conflict on the
  same paragraphs.
- Two prior plans left open for over a month (`pending-gate1` since
  2026-06-13, `active` since 2026-06-21) is itself a process signal worth
  naming to the governor directly, independent of this proposal's content.

## Rollback

All changes are documentation-only; rollback is reverting the relevant MR
commit(s). No schema, tooling, or CI behavior changes.

## Questions for Gate 1

1. Approve the four-bucket taxonomy (H / A / C / R) as the classification
   this project uses going forward, or propose different bucket names/cuts?
2. Close `agents-govern-2.0-pedagogical-docs-restructure.md` now (superseded
   or completed-via-untracked-work), or fold its remaining decomposition
   items into this plan's MR list?
3. Approve MR-5 (new `session-continuity.md`) as in-scope here, closing
   `docs-assessment-2026-06-21.md` item 6, rather than tracking it a third
   time?
4. Gate 1/Gate 2 lifecycle diagram direction: try extending the existing
   `gate-pipeline.svg` DOT source with owner labels first. Escalate only if
   that produces a crowded or misleading figure.
5. Should `contribution-auditor` and `fact-checker` review this plan file
   itself before Gate 1 sign-off (both are listed `status: active` in
   `agf/agents/MANIFEST.yaml` and were pulled in as advisory consultations
   per the requesting handoff)? Governor-reviewer direction on 2026-07-20:
   yes, advisory AI roles assess first; if they approve, governor-reviewer
   approval follows.

## Non-goal (restated)

This plan does not rewrite the documentation site. MR-1 through MR-6 are
proposed follow-on work, each requiring its own Gate 1 fast-track or full
review per its size, after this proposal itself is approved.
