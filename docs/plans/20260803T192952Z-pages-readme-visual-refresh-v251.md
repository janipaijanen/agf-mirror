# Pages + README visual refresh — target v2.5.1

Timestamp: 20260803T192952Z (OS-generated, `date -u +%Y%m%dT%H%M%SZ`)
Branch: `plan/pages-readme-visual-refresh-v251`
Status: DRAFT — plan + visual mockup only, no implementation yet. Awaiting
human governor review before any real file changes land.

## Intent

The public Pages site (MkDocs Material, stock indigo palette, default
fonts, single-level `navigation.top`) and the README's messaging currently
read as visually and rhetorically flat next to comparable technical
projects. This plan drafts a redesign direction — palette, typography,
navigation, hero component, and README structure — inspired by the visual
register of a reference site the human governor pointed to
(near-monochrome base, confident minimal copy, generous whitespace,
flat navigation), adapted to AGF's own governed brand material rather
than copied wholesale.

Target release: **v2.5.1**. Current `agf/VERSION` is `2.4.11`;
`v2.5.0-rc.1` is in an active Gate 2 review cycle per recent coordination
records. This work does not touch `v2.5.0`'s scope and should land as its
own MR once 2.5.0 ships, targeting the next patch/minor release. No
version bump, CHANGELOG entry, or tag is part of this plan (per
`release-via-merge-request-rule.md`, those happen only in the release
workflow itself).

## How this was drafted

Per the request, four parallel drafting passes ran as independent
subagents, each briefed with a distinct persona, the real repository
content they needed (current README text, current `nav:` tree, the
governed diagram palette, the applicable project rules), and an explicit
instruction never to name any specific school or institution when
describing the target reader:

1. **Technical writing** (mirrors this project's own `documentation-writer`
   agent role — audience-first, honest status labelling) — README
   restructure.
2. **Developer experience / information architecture** — navigation
   flattening.
3. **Marketing / positioning** — hero copy, bound by this repo's
   `marketing-tone-rule.md` and `verifiable-claims-rule.md`.
4. **UX / visual design** — palette, typography, spacing, hero component,
   and nav feature-flag spec for MkDocs Material, bound by
   `accessibility-rule.md` and the existing contrast-checked palette in
   `agf/docs/site/reference/palette.md`.

Full drafts are reproduced in "Agent drafts (verbatim)" below. This
section is the synthesis.

The rendered visual mockup is saved alongside this plan at
`docs/plans/assets/20260803T192952Z-pages-readme-visual-refresh-v251-mockup.html`
(open directly in a browser) and was also published as a Claude Artifact for
interactive review during drafting. The repo copy is the durable record;
treat the Artifact link as a convenience, not the source of truth.

**Human governor checkpoint (2026-08-03):** direction approved as a starting
point — "I like the approach, but I am not 100% confident that's the final."
Recorded here, and in `docs/TODO.md`, as an open item pending confirmation.
No implementation has started.

### Target reader calibration (for this draft only, not for commit into
any adopter-facing file)

Roughly advanced-undergraduate software/ICT-engineering capability:
comfortable with git, CI/CD, running a test suite, and baseline security
concepts; not assumed to have prior exposure to formal governance/audit
terminology and not addressed as a beginner. No institution name is
recorded anywhere in this plan or in any drafted copy.

## Decisions

### 1. Palette — hybrid, not a full swap

Adopt the existing governed `blue-mid` (`#0077BB`) as the site's UI
`primary` (nav/header chrome) in both light and dark schemes — white text
on `blue-mid` is 4.82:1 in both directions. Do **not** reuse `blue-mid`
as the **accent** (link/button text on page background) in dark mode: the
UX draft computed it at ≈2.7:1 against Material's slate background, which
fails WCAG AA even at the large-text threshold. Dark-mode accent needs a
new lightened tint, added to `reference/palette.md` with its own
contrast entry and validated by the existing `check_palette_contrast.py`
CI hook — **not** invented ad hoc in CSS. The mockup below uses an
illustrative tint (`#5BB6EC`) to demonstrate the direction; it is not
proposed as the final validated value.

### 2. Typography

Inter (display/body) + JetBrains Mono (labels, evidence/status values,
code) — both OFL-licensed and self-hostable. Real implementation must
vendor `.woff2` files into the repo (no font CDN — the project already
has no external runtime dependencies and Pages builds should stay that
way); the mockup uses a system-font fallback stack since it cannot ship
font binaries.

### 3. Navigation

Flatten the current single `Documentation` tree (5 nested groups, up to
6 levels deep) into 7 top-level tabs — Home, Evaluate, Learn, Adopt,
Operate, Reference, Contribute — using `navigation.tabs` +
`navigation.tabs.sticky` in place of `navigation.top`. Full proposed tree
is in the DX draft below. Two pages are demoted out of top-level nav
entirely (Philosophical lenses, AI evaluation dossier — both real pages,
kept and linkable, just not gating a new visitor's first path).
MkDocs page URLs are driven by file path, not nav position, so this
restructuring does not break any existing external link into the site.

### 4. Hero copy

Recommend **Variant A** from the marketing draft: *"Plans reviewed
before code. Code reviewed before merge."* — states the mechanism, not
an adjective; a reader can verify it by reading the workflow rather than
trusting a claim. Variant B (*"A decision record for every change an AI
makes."*) is a strong second option if the governor prefers a
record/audit framing over a gate framing. Variant C is included in the
draft for completeness but not recommended as primary.

### 5. README restructure

Adopt the technical-writing draft's structure: unchanged epigraph, a
tightened hero paragraph and bullet list with zero inline hedging, and a
new `## Evidence and maturity` section that consolidates every
qualifier currently interleaved in the "Typical cost" paragraph into a
claim/status/basis table. No claim is deleted or weakened — every
qualifier that exists today still exists, just relocated and linked from
the hero with one sentence. This keeps the change EDITORIAL/ADDITIVE
under `downstream-impact-rule.md`'s classification (no behavioral
contract changes; README.md is outside that rule's trigger-path list in
any case).

## Flagged for the human governor (not resolved here)

`.claude/rules/docs/documentation-audience-rule.md` states the public
docs site targets "business users (directors, managers, employees) and
system administrators." AGF's own site (`agf/docs/site/docs/index.md`,
"How to read this site") describes a different real audience model —
Human decision / AI action / Shared concept / Reference, i.e. developers
and technical evaluators overseeing AI agents, not business users doing
data entry. This mismatch predates this task and was not created or
resolved by it (per `uncertainty-and-scope-rule.md`, a rule/reality
conflict is surfaced, not silently resolved in either direction). All
four drafting agents were instructed to write for AGF's actual technical
audience rather than the rule template's business-user framing. If the
governor agrees, `documentation-audience-rule.md`'s "Public documentation
site" row should be customized for this project in a separate,
narrowly-scoped governance change — not folded into this visual-refresh
work.

## Affected files (once implementation is approved — none touched yet)

- `README.md` — hero/bullet restructure, new Evidence and maturity
  section.
- `agf/docs/build/mkdocs.yml` — `theme.palette` (custom primary/accent),
  `theme.features` (`navigation.tabs`, `navigation.tabs.sticky` replacing
  `navigation.top`), restructured `nav:` tree.
- `agf/docs/site/stylesheets/agf.css` — hero component styles, type
  scale, spacing scale.
- `agf/docs/site/overrides/main.html` — hero block markup for the
  homepage.
- `agf/docs/site/index.md` — hero copy source.
- `agf/docs/site/reference/palette.md` — new dark-mode accent tint entry
  with its own contrast row, if the UI-chrome unification in Decision 1
  is approved.
- New binary assets: self-hosted Inter + JetBrains Mono `.woff2` subsets
  (path TBD, likely `agf/docs/site/assets/fonts/`).

No migration needed (presentation-layer only, no schema/data changes).

## Test strategy

- `mkdocs build -f agf/docs/build/mkdocs.yml --strict` must still pass
  (broken internal links fail the build today; nav restructuring must not
  introduce any).
- `check_palette_contrast.py` must pass, including for any new dark-mode
  accent tint added to `reference/palette.md`.
- Per `accessibility-rule.md`: new hero CTAs and tab nav need visible
  keyboard focus states, accessible names, and must not rely on colour
  alone (the gate-ledger hero visual already carries shape distinctions
  from the existing governed responsibility palette, so this should
  carry over rather than being designed fresh).
- Per `visible-artifact-verification-rule.md` (this is visible output a
  human is meant to look at): before merge, a human opens the rendered
  site via `mkdocs serve` in both light and dark theme and at desktop and
  mobile widths, and confirms it looks correct — the Artifact mockup
  produced alongside this plan is a stand-in for early-stage visual
  review, not a substitute for that final check against the real
  MkDocs Material build.

## Risks

- Dark-mode accent contrast gap (Decision 1) must be resolved with a
  real validated token before shipping, not deferred or shipped with a
  guessed value.
- Font self-hosting adds binary assets to the repo; confirm OFL licence
  terms permit redistribution and add attribution if the licence
  requires it.
- Nav flattening is a real information-architecture change; the DX
  draft's "first 60 seconds" journey should be sanity-checked against
  actual adopter feedback if any exists, not treated as validated by
  this drafting pass alone.

## Rollout

Implementation (once approved) proceeds as its own branch/MR per
`branch-and-worktree-rule.md` and `merge-request-workflow-rule.md`,
sized so a reviewer can answer one yes/no question per MR (likely split:
one MR for README, one for the Pages theme/nav/hero, since those are
independently reviewable and revertible). No commit in this plan's
branch should be tagged or version-bumped.

---

## Agent drafts (verbatim)

### 1. Technical writing — README restructure

<!-- Full draft output preserved for audit; see coordination/session
record for the raw subagent transcript if this section needs
re-verification. Synthesized into Decision 5 above. -->

Restructured hero: unchanged epigraph, tightened "What AGF gives you"
bullets (each cut to one independent clause), new "## Evidence and
maturity" table with three rows (install time, governor attention, exit
duration), one linking sentence from the hero. Rationale: nothing
deleted, only relocated; the causal clause on "Reviewed learning,
separately governed improvement" was shortened but kept because it is
load-bearing for how the improvement process actually works, not
decorative hedging.

### 2. Developer experience / IA — navigation

<!-- Synthesized into Decision 3 above. -->

Proposed 7 top-level tabs (Home, Evaluate, Learn, Adopt, Operate,
Reference, Contribute) replacing the single nested "Documentation" tree.
"Use cases" and "Evaluate" merge into one page (both answer "does this
fit my situation"). "First 60 seconds" journey: Home's primary CTA reaches
a working first governed change in 1 click; a self-check-first reader
reaches it in 2 (Evaluate → should-you-adopt → Adopt → first-governed-
change). Demoted from top-level nav: Philosophical lenses and AI
evaluation dossier (narrow audiences, still linkable), Disclaimer
(footer-only regardless of section), Roadmap (folded into Evaluate).

### 3. Marketing / positioning — hero copy

<!-- Synthesized into Decision 4 above. -->

Three headline variants, all mechanism-first with no unsupported
superlative or vendor comparison:

- **A** — "Plans reviewed before code. Code reviewed before merge."
- **B** — "A decision record for every change an AI makes."
- **C** — "Governance that lives in your repo."

A draft line playing on "without spinning up new infrastructure" was
deliberately rewritten to the positive "added to your existing repo"
framing before being placed in Variant B, to avoid reading as an implicit
knock on hosted governance tools. A draft headline claiming AGF is "the
only" framework of its kind was discarded outright as an unsupported
superlative.

### 4. UX / visual design — MkDocs Material spec

<!-- Synthesized into Decisions 1-2 and the CSS/component notes above. -->

Palette: `blue-mid` for chrome in both themes; stock indigo (not
`blue-mid`) for dark-mode accent until a validated lightened tint exists.
Type: Inter + JetBrains Mono, three-tier scale (56px/32px/16px desktop
hero/heading/body). Spacing: 4/8/16/32/64px scale, 760px max body-text
column. Hero: two-column flex (text left, figure right) built in
`overrides/main.html`, collapsing to single-column below 960px. Nav:
`navigation.tabs` + `navigation.tabs.sticky`. All proposed text/fill
pairings re-checked against the existing 4.5:1 normal-text / 3:1
large-text-or-fill AA thresholds; the one pairing that fails
(`blue-mid` accent on dark slate) is explicitly excluded rather than
shipped.
