---
id: pages-readme-visual-refresh-v251
status: pending-gate1
topic: pages-readme-visual-refresh-v251
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: Reviewed the exploratory draft 2026-08-03
      (docs/coordination/20260803T201537Z-contribution-auditor-3kpe.md);
      found 5 pre-Gate defects (F-VIS-1..5), all addressed in this
      revision, and recommended holding any Gate 1 submission until the
      v2.5.0-rc.1 observation hold is lifted.
  - role: infosec-sentinel
    status: not-applicable
    reason: Presentation-layer documentation/CSS/navigation change only;
      no credentials, access control, schema, or security-boundary
      change.
---

# Pages + README visual refresh — exploratory draft, not an active v2.5.1 target

Timestamp: 20260803T192952Z (OS-generated, `date -u +%Y%m%dT%H%M%SZ`)
Branch: `plan/pages-readme-visual-refresh-v251`
Status: DRAFT / ON HOLD — plan + visual mockup only, no implementation.
Not eligible for Gate 1 submission or any release-planning obligation
while the v2.5.0-rc.1 observation hold
(`docs/coordination/20260803T200227Z-governor-reviewer-5m41.md`) is in
effect. Held as exploratory material only.

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

Target release: **provisional, not v2.5.1 as an active obligation.**
`v2.5.0-rc.1` has since published, been independently verified (three-track
external evaluation, `docs/coordination/20260803T204257Z-planner-architect-sjqm.md`),
and is now under a **binding governor observation hold**
(`docs/coordination/20260803T200227Z-governor-reviewer-5m41.md`, 2026-08-03):
"Do not begin a v2.5.0 final, v2.5.0-rc.2, v2.5.1-rc.1, or v2.5.1-rc.2 plan
or release process yet." This plan does not create or advance a v2.5.1
release obligation while that hold is in effect — it stays exploratory
material on its own branch until the governor lifts the hold and a
release target is explicitly reopened. No version bump, CHANGELOG entry,
or tag is part of this plan in any case (per `release-via-merge-request-rule.md`,
those happen only in the release workflow itself).

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

### 3. Navigation — superseded by a kind-based two-lane resolution

**Status: this subsection replaces the original flat 7-tab draft below**
(kept in "Agent drafts (verbatim)" for audit only — do not implement it
as written). The flat-tab draft was a real improvement over the current
6-level nested tree but treated all pages as one undifferentiated list.
A later exchange in the same review thread (with a human governor and
an independent external model, Grok, both weighing in) found a sharper
structure and converged on it collaboratively — recorded here as the
resolved decision, not as one party's unilateral call.

**The problem the flat draft missed:** a pure two-lane split ("adopter
path" vs. "evaluate/nosey path") was proposed first and is *almost*
right, but applied literally it leaves pages like "Roles and gates" and
"Coordination protocol" homeless — not adopter-only (a new visitor
doesn't need them before day one) and not evaluator-only (a working
adopter needs them by day two, not as theory). The fix does not invent
a third category — this site already states one, in prose, in
`agf/docs/site/docs/index.md`'s "How to read this site" section, and
has simply never built it into navigation:

> Human decision · AI action · Shared concept · Reference

**Resolved structure:** keep two reading contracts (critical adopt path
vs. evaluate/nosey path) but assign pages to a lane by their existing
four-kind classification, not by a fresh binary:

| Kind (existing taxonomy) | Lane | Rule |
|---|---|---|
| Human decision | Adopter path, front | Adopt, approve, accept risk, exit — ordered toward the first governed change |
| AI action | Adopter path, front | Procedure an assistant executes under supervision |
| Shared concept | Adopter path, **early** — not stranded under Evaluate | Needed by day one/two *and* by evaluators; pulling it into the adopt path is the one-word fix for the flat-lane version's biggest gap |
| Reference | Evaluate/reference lane, after the concept is clear | Precision — contracts, provenance, full inventories |
| Evaluate/theory (adjacent to Reference) | Nosey lane only | Dossier, philosophical lenses, external-review posture, non-binding future direction |

**Per-page inventory** (every page in the current `nav:` tree,
classified; supersedes the flat draft's tab assignment):

| Page | Kind | Lane |
|---|---|---|
| Home (`index.md`) | entry / lane selector | both — the front door, links into each lane explicitly |
| Use cases + Evaluate (merged, per DX draft) | Human decision | Adopter path, front |
| Roadmap | Human decision (informs adopt/not) | Adopter path, front |
| Should you adopt? | Human decision | Adopter path, front |
| Adopter FAQ | Human decision | Adopter path, front |
| Exit and decommission | Human decision | Adopter path, front — exit is a front-loaded decision, not a footnote |
| Install the agf directory | AI action | Adopter path, front |
| Assistant loader configuration | AI action | Adopter path, front |
| Your first governed change | AI action | Adopter path, front — the core worked example |
| agov MCP server | AI action (setup step) | Adopter path, front; Reference-depth content cross-linked |
| Downstream impact | AI action / Human decision | Adopter path, front |
| Operations runbook | AI action | Adopter path, day-2 |
| Session continuity | AI action | Adopter path, day-2 |
| Poll and coordinate | AI action | Adopter path, day-2 |
| Day-to-day operation | AI action | Adopter path, day-2 |
| Run AGF in real situations | AI action / Shared concept | Adopter path, day-2 |
| Review and release | AI action | Adopter path, day-2 |
| Upgrade from v1 | AI action | Adopter path, day-2 |
| Framework map | Shared concept | Adopter path, early |
| AGF and agent runtimes | Shared concept | Adopter path, early |
| Roles and gates | Shared concept | Adopter path, early — the page the flat two-lane split lost |
| Coordination protocol | Shared concept | Adopter path, early — same |
| Learning loop and evidence | Shared concept | Adopter path, early |
| Adopter applicability catalog | Reference | Evaluate/reference lane, linked from Adopt |
| Tooling and token savings | Reference (evidence-heavy) | Evaluate/reference lane, linked from Adopt |
| Contracts and validators | Reference | Evaluate/reference lane |
| Customizing the framework | Reference | Evaluate/reference lane, linked from Adopt once minimal path is done |
| Environment variables | Reference | Evaluate/reference lane |
| Release artifacts | Reference | Evaluate/reference lane |
| Maturity vocabulary | Reference | Evaluate/reference lane |
| Coordination graph index | Reference | Evaluate/reference lane |
| Review findings | Reference | Evaluate/reference lane |
| Tooling conformance | Reference | Evaluate/reference lane |
| Release information | Reference | Evaluate/reference lane |
| Disclaimer | Reference | Evaluate/reference lane (footer-weight regardless) |
| AI evaluation dossier | Evaluate/theory | Nosey lane only |
| Philosophical lenses on AI governance | Evaluate/theory | Nosey lane only |
| Evolution | Evaluate/theory (non-binding future direction) | Nosey lane only |
| Contribute | mixed / community | Nosey lane, footer-weight |

**Implementation rules:**

1. Adopter-path nav = Human decision + AI action pages, ordered toward
   the first governed change, using `navigation.tabs` +
   `navigation.tabs.sticky`.
2. Shared concept pages are pulled early into the adopter path — not
   left stranded under an "Evaluate" or "Learn" tab, which was the flat
   draft's actual defect.
3. Nosey/evaluate lane = the dossier, philosophical lenses, Evolution,
   and Reference-depth pages. Kept as a real top-level tab (`Evaluate`),
   not deleted — evaluators must not be funneled through a simplified
   adopter path that hides Track 3 findings, non-goals, or evidence
   status.
4. Per-page badge: one of the four kind labels, plus the line *"Not on
   the critical adopt path"* on any page whose kind is Reference or
   Evaluate/theory.
5. Cross-links both ways: Shared concept pages link into both lanes;
   nosey pages link back to the minimal adopt path. Neither lane is a
   dead end.

This is **not** two products or two truth levels — one framework, two
reading contracts, four kinds already on record in this site's own
docs, now actually built into navigation instead of asserted in prose
one reader has to already be on `docs/index.md` to see.

MkDocs page URLs are driven by file path, not nav position, so none of
this restructuring breaks any existing external link into the site.

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

### 5a. README structure — refined, supersedes the loose hero+bullets shape above

**Frame (new, adopted):** README is the forge door, doing triage — the
published Pages site is where persuasion and teaching happen. The
earlier hero+bullets+evidence-section draft above still describes the
right *content*, but not the right *division of labor* between the two
surfaces. Concretely: README should point, not carry the full pitch.

**Structure (replaces the loose shape in Decision 5):**

| Section | Max length | Content |
|---|---|---|
| Title + one-liner | 3 lines | What it is, one sentence; explicitly not a runtime |
| Not this | 5 bullets | Not Microsoft AGT, not an orchestrator, not a root of trust alone |
| Fit | 6 bullets | Try when / skip when — including the root-of-trust and forge-native-AI-review skip cases from the competitive-positioning section above |
| 60-second install | commands | Pinned `vX.Y.Z` archive, link to published docs |
| Map | short table | `agf/` = shipped product; `docs/` = this repo's own evidence/process |
| Deep dive | one line | Link to the published docs site — full narrative lives there, not in README |

**Correction, verified false (2026-08-03):** an earlier claim that
README's docs links are "repo-native paths (`agf/docs/site/docs/...`),
not the published URL first" is checked and **wrong** for the current
README — `grep` confirms six separate links to the actual
`https://janipaijanen.gitlab.io/agents-govern-framework/...` site,
starting at the top ("Published docs:" / "Current roadmap:"). Dropped,
not carried forward. The broader point (README currently carries too
much narrative density for a repo door) stands on its own evidence and
doesn't need that specific claim to hold.

**TODO — unverified, needs a citation check before use:** a claim that
"AGF" collides with other unrelated uses (session finders, papers, 5G,
funds) was offered with no source and no verification method — same
unverified-specific-claim shape as Lumo's fabricated schema filenames
earlier in this review. Not dropped outright, because the underlying
instinct (short acronyms are usually overloaded; disambiguate on first
screen regardless) is plausible on priors — but not to be repeated as
fact, and not to inform the "Not this" bullets above, until someone
actually searches and confirms specific real collisions.

## Audience-assumption stress test (protocol + first pass)

**Protocol, going forward:** a single imported reader-capability baseline
must not be applied uniformly to public pages without an adversarial pass
per page (or per page-group, at the granularity the plan is currently
operating at). Each pass argues *against* applying the baseline to that
specific page, independently. Disagreements and convergent findings are
synthesized and put to the human governor for a decision — the synthesis
is not self-resolved by AI consensus. This is now standing practice for
this workstream as it advances from hero/nav/README scope into per-page
content drafting.

**The baseline being stress-tested** (paraphrased from a named technical
university's published bachelor's-in-ICT-engineering graduate-outcomes
page — the institution is deliberately not named in this file):
full-stack build skill (front end, back end, DB schema, deployment);
Linux/Windows infrastructure and networking; cybersecurity practice
(risk assessment, vulnerability scanning, hardening, incident response);
relational DB design/SQL; scripting/automation/CI-CD; ML/AI
implementation if specialized; TCP/IP and network architecture; IoT
sensor/edge basics; plus Agile project management, teamwork, engineering-
mindset problem decomposition, independent research, and stakeholder
communication — framed around "DevSecOps competence," foundational
across development, security, and operations by year three.

**Correction (contribution-auditor finding, `3kpe`, 2026-08-03):** the
subsections below were originally framed as "six independent adversarial
critiques" reaching "independent" conclusions. That overstates what
happened: one orchestrator ran six structured prompts from one shared
source packet and framing. This is **one AI-assisted adversarial
exercise with six page-group angles**, not six independent evidence
sources, and not adopter consensus — corrected throughout this section.
The raw per-group drafts were not preserved as a durable, citable
artifact (they existed only in a session transcript, which is not a
governed evidence source); any future claim that reuses these findings
must not cite "the session transcript" as if it were retrievable
evidence. If the underlying arguments are needed again, they should be
regenerated and committed as a real file, not asserted as already
existing.

**First pass — one exercise, six page-group angles** (Home, README,
"Should you adopt?" as the Human-decision kind, the Shared-concept
group including Philosophical lenses, the AI-action group
[Adopt/Operate], and the Reference group). The converging structural
finding and each group's specific counter-proposal, as originally
recorded:

| Page group | Why the baseline's *axis* is wrong for this page (not just too high/low) | Proposed axis instead |
|---|---|---|
| Homepage | Assumes the reader already self-selected as an implementer before the page has earned that | "Has used or heard of PR/code-review workflows" — nothing deeper |
| README | Assumes one reader; a README structurally draws several incompatible ones, including an AI agent reader with no human-style "capability level" at all | No single baseline; mixed-audience prose + a separately labeled AI-instructions section |
| Should you adopt? (Human decision) | The person who decides is often the one representing the DevSecOps gap the framework fills, not someone who already embodies it | "Can read a technical proposal and make a resourcing/risk decision" — generalist, not practitioner |
| Concepts / Philosophical lenses (Shared concept) | Measures build/operate skill; these pages need organizational-authority literacy and tolerance for non-convergent argument — orthogonal to an engineering curriculum, not a subset of it | Decision/escalation literacy + comfort with normative argument |
| Adopt/Operate (AI action) | The AI does the hands-on work; the human's job is judging the output — a narrower, more teachable skill the baseline never names | "Can read a diff/YAML/CI log well enough to judge correctness" — verification, not construction |
| Reference | Not too high or too low — irrelevant axis. Gates on this project's own vocabulary, which no external curriculum teaches | "Has read the relevant Concepts page" — an in-project prerequisite, not an external credential |

All six angles landed on the baseline's *axis* being wrong for their
page, not merely its level — zero overlap in the specific objection,
one shared structural conclusion within this single exercise: reader-
capability is a per-page-kind derived attribute here, not a single
site-wide constant (same shape as `concept-modelling-rule.md`'s
intrinsic-vs-derived distinction, applied to "assumed reader" instead
of a data-model field). This is a theoretically useful critique from
one AI-assisted pass, not independently corroborated evidence.

**Accepted follow-up, not yet executed (per `3kpe`):** model each page
group by primary and secondary audience, user task, prerequisite
knowledge, and action/verification duty — not by a single capability
level. Split `Adopt` from `Operate` (they were merged above). Split
Reference material by task (installation, supervision, troubleshooting,
command lookup, schema lookup do not share one reader contract). Note
explicitly that an AI-action page still needs a human troubleshooting
and verification path, which the single-axis table above does not
capture.

**Status: proposed, not decided, and due for the re-model above before
it is treated as settled.** This table is a single AI-assisted-exercise
output, not multi-source evidence and not a governor decision.
drafting.

## Blocking prerequisite: the documentation-audience-rule mismatch

**Correction (contribution-auditor finding, `3kpe`, 2026-08-03):**
originally recorded below as a flagged-but-parallel note. That
undersells it. `.claude/rules/docs/documentation-audience-rule.md` is
the **active, adopted project rule** — not the uncustomized template
under `agf/rules/` — and this plan's own drafting agents were
instructed to write against a different audience model that conflicts
with it. A visual implementation must not knowingly proceed against a
conflicting active rule. This is now a **hard prerequisite, not a
parallel side-item**: the rule mismatch needs its own narrow governance
change, reviewed and merged on its own, before this plan depends on the
result — not resolved inline as part of the presentation implementation.

Original finding, preserved: `.claude/rules/docs/documentation-audience-rule.md`
states the public docs site targets "business users (directors,
managers, employees) and system administrators." AGF's own site
(`agf/docs/site/docs/index.md`, "How to read this site") describes a
different real audience model — Human decision / AI action / Shared
concept / Reference, i.e. developers and technical evaluators
overseeing AI agents, not business users doing data entry. This
mismatch predates this task and was not created or resolved by it (per
`uncertainty-and-scope-rule.md`, a rule/reality conflict is surfaced,
not silently resolved in either direction). All four drafting agents
were instructed to write for AGF's actual technical audience rather
than the rule template's business-user framing.

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

## Competitive positioning and exit criteria — draft input for "Who should not adopt"

**Status: exploratory draft content, not reviewed for accuracy or
publication, not to be advanced while the v2.5.0-rc.1 observation hold
is in effect.** Different artifact class from this plan's other
sections and from the framework's `learnings/` schema — captured here
because it's draft-ready input for a documentation section, not
operational evidence from running the framework. The one operational
angle (no self-reported enforcement profile → overclaim risk) is
already recorded as learning `20260803T212421Z-t8mz`; do not duplicate
it here.

Produced via the same cross-model exchange pattern as the rest of this
session (Grok proposed, corrected, and integrated through direct
critique — not independent corroboration; see learning
`20260803T212419Z-nq7p` on the relay-trust caveat that applies to this
content too).

**Landscape, three tiers:**

- **Substrate, not siblings** — `AGENTS.md`, Cursor/Claude rule loaders,
  and `agf/rules/` are the delivery mechanism assistants already load;
  AGF is a structured superset (schemas, gates, coordination,
  validators) built on top of that mechanism, not a parallel choice
  against it. The comparison "AGENTS.md vs. AGF" is a category error;
  the real comparison is "rule pack alone vs. rule pack plus AGF."
- **Collaborators** — agent runtimes/IDEs, CI/forge branch protection,
  runtime agent-governance platforms (explicitly a different layer, not
  a competitor), model/ML governance tooling, security scanning,
  identity/signing (Sigstore, signed commits — could harden AGF's
  currently-unbound `from:`/approval story per the gt3x/hv3k learnings).
- **Competitors** — "CODEOWNERS + required CI + good prompts" for
  small/low-AI-volume teams; **forge-native AI review** (GitHub Copilot
  PR review, GitLab Duo) as a distinct row — competes on convenience and
  existing procurement budget, not on a full coordination/gate model,
  and is likely the first thing many teams reach for; runtime governance
  platforms when the actual problem is production tool-call policy, not
  git process; heavier multi-agent orchestration frameworks when the
  buyer wants an orchestration product, not a file library; and simply
  "nothing," when disposable code doesn't justify the process cost.

**Exit-criteria table (detect "AGF has no business here"):**

| Signal | Why it's an exit |
|---|---|
| No durable git product (chat-only, spikes, one-off scripts) | Nothing for the process to attach to |
| Problem is runtime behavior (tool calls, sandboxing, kill switches) | Different layer — AGT-class/mesh/gateway territory |
| Problem is the model itself (training, eval, bias, release gates) | ML governance, not process governance |
| Zero human governor capacity | AGF cannot supply authority — Track 3 confirmed this directly |
| Branch protection + CODEOWNERS + "no AI on main" already meets the org's risk appetite | Simpler sibling already does the job |
| Buyer wants a root of trust — tamper-proof, identity-bound approval as the product | Current AGF is scaffolding; this is the overclaim failure mode |
| Org already standardized on a different in-repo protocol | Second vocabulary is pure cost |
| AI writes little of the code | Process overhead without the failure mode it targets |

**The sharpest point, tying back to `t8mz`:** the "root of trust" exit
row is not just a positioning judgment call — it's currently
undetectable by AGF itself. Without a self-reported enforcement-profile
label (schema-shape-only / CI-blocking-configured / identity-bound),
there's no mechanism for AGF to cleanly signal "you're in this exit
category, go elsewhere" to a prospect who needs real tamper-evidence.
The same missing label that risks overclaiming to a casual reader is
the label that would let the docs correctly decline the wrong customer.
One fix (the maturity-vocabulary extension `t8mz` proposes) serves both
the honesty gap and this business-development disambiguation.

**Recommended eventual home:** a "Who should not adopt" section with
equal visual/structural weight to "Should you adopt?", using this exit
table directly, once (a) the observation hold lifts and (b) this
content has had an actual accuracy/citation review — none of the
competitor claims above have been fact-checked the way the AGF-specific
technical findings elsewhere in this plan were.
