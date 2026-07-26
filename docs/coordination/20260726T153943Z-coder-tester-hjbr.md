---
artifacts_owned: []
from: coder-tester
id: 20260726T153943Z-coder-tester-hjbr
immutable: true
kind: review
model: claude-sonnet-5
references-to: []
requires-ack: false
timestamp: 2026-07-26T15:39:43Z
to: governor-reviewer
topic: public-docs-eli5-new-adopter-assessment
---

# Public-docs ELI5 / new-adopter assessment (read-only, no Gate 1)

Per user request: assessed the live v2.4.4 public docs site (rebuilt from
`103930d`, confirmed live via WebFetch of the homepage) for content order,
ELI5-ness, and missing/thin content for new adopters. Ran five parallel
subagents against the exact `mkdocs.yml` nav order, reading the actual
files under `agf/docs/site/`: four personas (skeptical engineering lead,
non-technical director, solo developer doing a first install, and an
AI-assistant operator) plus one structural nav-order/forward-reference
audit. This is assessment only — no files changed, no Gate 1 needed.

## Highest-confidence findings (corroborated by 2+ independent agents)

1. **Forward-reference problem, the single most corroborated finding.**
   Core vocabulary ("Gate 1"/"Gate 2", role names — `contribution-auditor`,
   `fact-checker`, `planner-architect`/`coder-tester`/`governor-reviewer`,
   "maturity level"/"Level 5") is used substantively many pages before it
   is actually defined. Concretely: `contribution-auditor`/`fact-checker`/
   `infosec-sentinel`/`academic-peer-reviewer` first appear in a figure
   footnote on `framework-map.md` (nav #6) but aren't explained until
   `applicability-catalog.md` (#16) and `upgrade-from-v1.md` (#19) — 13
   pages later. "Level 5 — Deployable" is used on `index.md` (#1) 29 pages
   before its canonical definition page. `adopter-faq.md` uses "Gate
   1"/"Gate 2" as settled vocabulary ~9 pages before `roles-and-gates.md`
   defines them. Found independently by the skeptical-eng-lead persona,
   the non-technical-director persona, and the structural audit.

2. **`exit-and-decommission.md` self-contradicts its own nav position.**
   It opens "Read this page **before** adopting" but is the *last* page in
   the Adopt section — after install, first-governed-change,
   upgrade-from-v1, tooling-and-tokens, and mcp-server. A linear reader
   hits this instruction only after doing everything it says to read
   first. Structural audit's clearest ordering defect.

3. **`upgrade-from-v1.md` interrupts the new-adopter narrative.**
   Placed between `first-governed-change.md` (a brand-new adopter's first
   real change) and `tooling-and-tokens.md` (general reference), even
   though it's a v1→v2 migration checklist irrelevant to anyone without an
   existing v1 install. Found independently by the solo-dev persona and
   the structural audit.

4. **Install-path fragmentation for a brand-new adopter with no checkout.**
   Three competing `agov` install paths (full checkout, release archive,
   OCI image) are spread across `install-and-layouts.md` and
   `tooling-and-tokens.md`, three pages apart, with no page foregrounding
   the "new repo, no framework checkout" case — `install-and-layouts.md`'s
   "symlink `tooling/agov`" instruction only makes sense if you already
   have the AGF-NG monorepo checked out. **Partially addressed in-flight:**
   MR !599 (this session, awaiting Gate 2) already reorders
   `tooling-and-tokens.md` to lead with the release archive — the
   remaining gap is `install-and-layouts.md` not yet reflecting the same
   priority, and no page explicitly branching on "do you have a checkout
   or not."

## Other significant findings

- **Case-study credibility gap (skeptical eng lead's top finding).** The
  `use-cases.md` codenamed vignettes (silver-birch, copper-rapids,
  blue-marlin, etc.) read like independent case studies but appear to all
  trace back to the same maintainer's own projects — no independently
  verifiable third-party account exists anywhere in the reviewed pages.
  This is in tension with the site's own citation-discipline rules.
- **Repeated "what ships vs what you activate" box** copy-pasted
  near-verbatim across `index.md`, `evaluate.md`,
  `ai-evaluation-dossier.md`, and `should-you-adopt.md` — a direct
  instance of the project's own message-integrity principle ("state a
  claim once, canonical location, others link to it") being violated by
  the docs about that very principle.
- **Cold-start command-order contradiction (AI-operator persona).**
  `day-to-day.md`, `run-agf-in-real-situations.md`, and
  `coord-graph-index.md` each state a different canonical "first command
  after cold start" sequence, with none marked as superseding the others
  — a real risk of inconsistent AI-assistant behavior at session start.
- **Wrong-audience placement for a non-technical decision-maker.**
  `evaluate.md` and `ai-evaluation-dossier.md` are explicitly written for
  AI assistants/independent reviewers (the pages say so themselves) but
  sit early in the track a director would naturally follow, at nav
  positions #3–#4; they provide zero decision-relevant content for that
  reader. Risk-if-it-goes-wrong is answered only very late (disclaimer.md,
  #35; should-you-adopt.md's privacy checklist).
- **No explicit "initial setup is complete" checkpoint** anywhere in the
  adopt flow (solo-dev persona) — `install-and-layouts.md`'s "Verify"
  checklist and `first-governed-change.md`'s ending both function as soft
  done-signals but neither states it outright.
- **No example coordination-message filename shown** anywhere in
  `first-governed-change.md`'s walkthrough, despite the walkthrough
  repeatedly referencing `<gate-1-handoff-id>` etc. as if self-evident
  (solo-dev persona).
- **n=1 self-reported evidence used as headline claims**: the "installation
  has taken no more than ten minutes" line (repeated 4+ times) and the
  "one to two orders of magnitude" token-savings claim are both honestly
  labeled as maintainer/foundry estimates, but repetition across many
  pages lends them more apparent weight than the n=1 evidence supports.

## Minor / thin-section findings

- `docs/index.md`'s "Reference" section links to
  `docs/reference/quick-reference.md`, which is not part of the published
  `reference/` nav at all — a dangling reference to a distribution-only
  file.
- `reference/tooling-conformance.md`'s "Conformance suite" section is
  entirely aspirational ("a future `agf/conformance/` directory
  **should**...") with no current fixtures — should be labeled at an
  explicit maturity level rather than read as already-existing.
- `ai-evaluation-dossier.md`'s "Authoritative Repository References" and
  "Full Source Page Appendix" sections are build-generated placeholders
  with no visible content in the raw source — self-disclosed but visually
  looks like a broken/empty section to a raw-file reader.
- `applicability-catalog.md` self-discloses "8 of 58 governed artifacts
  classified" up front — transparently incomplete, not deceptive, but the
  page a new adopter needs most for a safe/unsafe judgment is 86% empty.
- The MCP `dir`/`coordination_dir` known bug (already tracked in
  `docs/plans/mcp-coordination-dir-contract-fix.md`) was independently
  re-confirmed as a live foot-gun by two agents; not new, but worth
  noting it surfaced organically from both the eng-lead and AI-operator
  personas without being prompted toward it.

## Disposition

No files changed. If useful, findings #1-4 above (the corroborated ones)
are compact enough to seed a bounded Gate 1 plan — several map directly
onto the already-approved MR !591 slices (public-docs projection; the
cold-start contradiction fits the foundry/adopter executable-guidance
slice). Awaiting governor direction on whether/how to fold any of this
into v2.4.5 or route to v2.5.0.
