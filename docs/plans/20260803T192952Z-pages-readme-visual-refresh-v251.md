---
id: pages-readme-visual-refresh-v251
status: pending-gate1
topic: pages-readme-visual-refresh-v251
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: Reviewed the exploratory draft on 2026-08-03
      (docs/coordination/20260803T201537Z-contribution-auditor-3kpe.md)
      and the bounded revision on 2026-08-05
      (docs/coordination/20260805T095052Z-contribution-auditor-nr3r.md).
      All findings from both reviews are addressed in this revision and
      independently confirmed CLEAR at exact head 52a96687 in
      docs/coordination/20260805T132830Z-contribution-auditor-rrsm.md.
  - role: infosec-sentinel
    status: not-applicable
    reason: Presentation-layer documentation/CSS/navigation change only;
      no credentials, access control, schema, or security-boundary
      change.
---

# Pages + README visual refresh — active planning, implementation held

Timestamp: 20260803T192952Z (OS-generated, `date -u +%Y%m%dT%H%M%SZ`)
Branch: `plan/pages-readme-visual-refresh-v251`
Status: ACTIVE PLANNING / IMPLEMENTATION HELD — plan + visual mockup only.
Governor decision `20260805T090722Z-governor-reviewer-83m8.md` narrows
the observation hold only for bounded refinement of this plan and mockup.
MR !1151 is approved and merged, so its semantic prerequisite is satisfied.
Production Pages/README edits, implementation, and every release action
remain unauthorized pending their separate governed decisions.
Temporary planning ownership belongs to Codex until the earliest expiry
condition in `83m8`.

## 2026-08-05 bounded planning activation

This revision turns the exploratory material into one reviewable plan while
preserving the sequencing decision in
`20260805T081045Z-planner-architect-nt3e.md`:

1. MR !1151 was the semantic prerequisite. It received authoritative
   contribution-auditor CLEAR, binding governor Gate 1 approval, and merged;
   the prerequisite is satisfied and this plan may now enter Gate 1.
2. This plan fixes scope and acceptance criteria; it does not implement the
   site or root README.
3. Implementation is split into independently reviewable slices under one
   Gate 1 decision: README; Pages theme/hero; and Pages navigation. Each slice
   receives its own implementation commit and Gate 2 disposition.
4. Historical drafts below remain audit evidence. Where they conflict with the
   decisions or bounded scope above, the decisions in this revision control.

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

Target release: **unassigned; this is not an active v2.5.1 release
obligation.**
`v2.5.0-rc.1` has since published, been independently verified (three-track
external evaluation, `docs/coordination/20260803T204257Z-planner-architect-sjqm.md`),
and is now under a **binding governor observation hold**
(`docs/coordination/20260803T200227Z-governor-reviewer-5m41.md`, 2026-08-03):
"Do not begin a v2.5.0 final, v2.5.0-rc.2, v2.5.1-rc.1, or v2.5.1-rc.2 plan
or release process yet." Decision `83m8` permits this plan and mockup to
advance, but does not reopen v2.5.1 or any other release target. No version
bump, CHANGELOG entry, tag, release, or deployment is part of this plan (per
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
on `blue-mid` is 4.82:1. Use `blue-mid` for light-theme accents and the
existing governed `blue-light` (`#99CCEE`) for dark-theme accents. Do
**not** ship the mockup's illustrative `#5BB6EC` tint or invent another
token ad hoc. Implementation records `blue-light`'s measured contrast on
the actual Material slate background in `reference/palette.md` and must
clear WCAG AA before use; if it fails, the slice blocks and returns to
planning rather than choosing an ungoverned fallback.

### 2. Typography

Use the mockup's system stacks for the first refresh: platform sans-serif
for display/body and platform monospace for labels, evidence/status values,
and code. Do not vendor Inter or JetBrains Mono and do not add a font CDN in
this workstream. Font binaries, licensing/attribution, subsetting, and cache
policy would create a separate supply-chain review that is not necessary to
establish the visual hierarchy; it may be proposed later.

### 3. Navigation — kind-based structure with five top-level entry points

**Status: this subsection replaces the original flat 7-tab draft below**
(kept in "Agent drafts (verbatim)" for audit only — do not implement it
as written). The flat-tab draft was a real improvement over the current
6-level nested tree but treated all pages as one undifferentiated list.
A later exchange in the same review thread (with a human governor and
an independent external model, Grok, both weighing in) found a sharper
structure and converged on it collaboratively — recorded here as the
resolved decision, not as one party's unilateral call.

**The problem the flat draft missed:** a pure two-lane split ("adopter
path" vs. "evaluate/deep-dive path") was proposed first and is *almost*
right, but applied literally it leaves pages like "Roles and gates" and
"Coordination protocol" homeless — not adopter-only (a new visitor
doesn't need them before day one) and not evaluator-only (a working
adopter needs them by day two, not as theory). The fix does not invent
a third category — this site already states one, in prose, in
`agf/docs/site/docs/index.md`'s "How to read this site" section, and
has simply never built it into navigation:

> Human decision · AI action · Shared concept · Reference

**Resolved structure:** keep two reading contracts (critical adopt path
vs. evaluate/deep-dive path) but assign pages by their existing four-kind
classification, not by a fresh binary. Expose five stable top-level entries:
**Home, Adopt, Evaluate, Reference, Contribute**. `Reference` remains a
separate tab for direct lookup while still belonging to the evaluate/deep-dive
reading contract.

| Kind (existing taxonomy) | Lane | Rule |
|---|---|---|
| Human decision | Adopter path, front | Adopt, approve, accept risk, exit — ordered toward the first governed change |
| AI action | Adopter path, front | Procedure an assistant executes under supervision |
| Shared concept | Adopter path, **early** — not stranded under Evaluate | Needed by day one/two *and* by evaluators; pulling it into the adopt path is the one-word fix for the flat-lane version's biggest gap |
| Reference | Evaluate/reference lane, after the concept is clear | Precision — contracts, provenance, full inventories |
| Evaluate/theory (adjacent to Reference) | Evaluate/deep-dive lane only | Dossier, philosophical lenses, external-review posture, non-binding future direction |

**Per-page inventory** (the 2026-08-05 current `nav:` contains 42 Markdown
entries; this classification is a planning map, not new page metadata and not
a requirement to edit all 42 files):

| Page | Kind | Lane |
|---|---|---|
| Home (`index.md`) | entry / lane selector | both — the front door, links into each lane explicitly |
| Use cases | Human decision | Evaluate, linked from Adopt's decision group |
| Evaluate | Human decision | Evaluate, linked from Adopt's decision group |
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
| AI evaluation dossier | Evaluate/theory | Evaluate/deep-dive lane only |
| Philosophical lenses on AI governance | Evaluate/theory | Evaluate/deep-dive lane only |
| Evolution | Evaluate/theory (non-binding future direction) | Evaluate/deep-dive lane only |
| Contribute | mixed / community | Contribute, footer-weight |
| Influences and acknowledgements | Reference / community | Contribute, footer-weight |

**Implementation rules:**

1. Configure `navigation.tabs` + `navigation.tabs.sticky` and use exactly the
   five top-level entries above.
2. `Adopt` contains decision pages, the Shared-concept pages needed on day
   one/two, setup procedures, and operating procedures, ordered toward the
   first governed change.
3. `Evaluate` contains use cases, the existing evaluate page, roadmap,
   dossier, philosophical lenses, and Evolution. It must not hide Track 3
   findings, non-goals, or evidence status.
4. `Reference` contains lookup/contract pages without changing their paths.
   `Contribute` contains the existing contribute and acknowledgement pages.
5. This slice changes `mkdocs.yml` navigation only. It does **not** merge page
   bodies, rename/move files, add 42 per-page badges, or perform a site-wide
   cross-link retrofit. Those earlier ideas would materially enlarge the
   review surface and require their own content plan. Existing URLs therefore
   remain unchanged.

**Exact navigation target (all 42 current entries, no additions or drops):**

```yaml
nav:
  - Home: index.md
  - Adopt:
      - Decide:
          - Should you adopt?: docs/start/should-you-adopt.md
          - Adopter FAQ: docs/start/adopter-faq.md
          - Downstream impact: docs/adopt/downstream-impact.md
          - Exit and decommission: docs/adopt/exit-and-decommission.md
      - Understand:
          - Documentation overview: docs/index.md
          - Framework map: docs/start/framework-map.md
          - AGF and agent runtimes: docs/concepts/position.md
          - Roles and gates: docs/concepts/roles-and-gates.md
          - Coordination protocol: docs/concepts/coordination-protocol.md
          - Learning loop and evidence: docs/concepts/evidence-and-learnings.md
      - Start:
          - Install the agf directory: docs/adopt/install-and-layouts.md
          - Assistant loader configuration: docs/adopt/assistant-loader-configuration.md
          - Your first governed change: docs/start/first-governed-change.md
          - agov MCP server: docs/adopt/mcp-server.md
      - Operate:
          - Operations runbook: docs/operate/runbook.md
          - Session continuity: docs/operate/session-continuity.md
          - Poll and coordinate: docs/operate/poll-and-coordinate.md
          - Day-to-day operation: docs/operate/day-to-day.md
          - Run AGF in real situations: docs/operate/run-agf-in-real-situations.md
          - Review and release: docs/operate/review-and-release.md
          - Upgrade from v1: docs/adopt/upgrade-from-v1.md
  - Evaluate:
      - Use cases: use-cases.md
      - Evaluate AGF: evaluate.md
      - Roadmap: roadmap.md
      - AI evaluation dossier: ai-evaluation-dossier.md
      - Philosophical lenses: docs/concepts/philosophical-lenses.md
      - Evolution: future/evolution.md
  - Reference:
      - Adopter applicability catalog: docs/adopt/applicability-catalog.md
      - Tooling and token savings: docs/adopt/tooling-and-tokens.md
      - Contracts and validators: docs/advanced/contracts-and-validators.md
      - Customizing the framework: docs/advanced/customizing-the-framework.md
      - Environment variables: reference/environment-variables.md
      - Release artifacts: reference/release-artifacts.md
      - Maturity vocabulary: reference/maturity-vocabulary.md
      - Coordination graph index: reference/coord-graph-index.md
      - Review findings: reference/review-findings.md
      - Tooling conformance: reference/tooling-conformance.md
      - Release information: reference/release-info.md
      - Disclaimer: reference/disclaimer.md
  - Contribute:
      - Contribute: contribute.md
      - Influences and acknowledgements: acknowledgements.md
```

This is **not** two products or two truth levels — one framework, two
reading contracts, four kinds already on record in this site's own
docs, now actually built into navigation instead of asserted in prose
one reader has to already be on `docs/index.md` to see.

MkDocs page URLs are driven by file path, not nav position, so none of
this restructuring breaks any existing external link into the site.

### 4. Hero copy

**Final selection: Variant A** from the marketing draft: *"Plans reviewed
before code. Code reviewed before merge."* — states the mechanism, not
an adjective; a reader can verify it by reading the workflow rather than
trusting a claim. Variants B and C are rejected alternatives retained only
as visible audit history in the draft section; neither is an open choice for
implementation. Any later change to the selected headline returns to planning.

### 5. README restructure

Adopt the technical-writing draft's structure: unchanged epigraph, a
tightened hero paragraph and bullet list with zero inline hedging, and a
new `## Evidence and maturity` section that consolidates every
qualifier currently interleaved in the "Typical cost" paragraph into a
four-row claim/status/basis table: mechanical installation, initial
commissioning, governor attention, and exit duration. No claim is deleted or
weakened — every
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
| AI assistants | 3 lines | Separately labeled route to `AGENTS.md`; the `agov` CLI is the universal supported path, while MCP-capable assistants are recommended to connect agov MCP for structured access, linking to the MCP status/limitations page |
| Map | short table | `agf/` = shipped product; `docs/` = this repo's own evidence/process |
| Deep dive | one line | Link to the published docs site — full narrative lives there, not in README |

The separately labeled AI-assistant route uses this exact relationship:
"Read `AGENTS.md` before acting. The `agov` CLI is the universal supported
path. Recommended for MCP-capable assistants: connect agov MCP for structured
access." The MCP phrase links to the published `docs/adopt/mcp-server.md`
status page so its Level 4 maturity and known path-contract limitation remain
one click away. It is a recommendation for a capable surface, not a requirement
for adoption and not a claim that MCP is Level 5 Deployable.

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

**Resolved planning model (executes `3kpe`'s accepted follow-up):**

| Group | Primary / secondary audience | User task | Prerequisite | Action and verification duty |
|---|---|---|---|---|
| Home | Curious technical evaluator / engineering decision-maker | Decide whether to inspect fit or start the adopt path | Familiarity with pull requests or code review | Human selects a lane; claims link to evidence |
| README | Developer arriving through the forge / evaluator or AI assistant | Identify the project, reject obvious misfit, install or open deeper docs | Git repository literacy | Human verifies pinned install source; assistants are routed to governed instructions rather than given an implicit execution contract |
| Human-decision pages | Governor or engineering lead / assurance stakeholder | Adopt, wait, reject, accept risk, or exit | Can read a technical proposal and make a resourcing/risk decision | Human owns the decision and can locate costs, caveats, and exit path |
| Shared-concept pages | Adopter/operator / evaluator | Build the mental model needed to interpret procedures and reference contracts | Decision/escalation literacy; no assumed AGF vocabulary | Reader can explain the concept and follow links into both action and evidence surfaces |
| AI-action: Adopt | AI assistant under a human governor / developer verifying setup | Install, configure, and complete the first governed change | Git, diffs, YAML, and CI-log literacy for the verifier | AI executes; human gets explicit checkpoints, troubleshooting, and acceptance evidence |
| AI-action: Operate | AI assistant under a human governor / operator | Resume, coordinate, poll, review, and release | Completed adoption path plus relevant concept page | AI executes bounded steps; human verifies exact revisions, CI, and gate decisions |
| Reference | Operator or reviewer seeking a contract / evaluator checking evidence | Command, schema, environment, release, or evidence lookup | Relevant concept/action page named where necessary | Reader verifies an exact value or contract; reference text does not silently become a tutorial |

This matrix is a planning decision for the three bounded slices, grounded in
the single-project audience exercise and therefore **Empirically informed**, not
independent-adopter validation. It does not mandate a page-by-page content
retrofit in this workstream.

## Satisfied prerequisite: the documentation-audience-rule mismatch

**Current prerequisite status (2026-08-05):** MR !1151 contains the narrow
audience-rule correction at exact head `ef8b8947`, with a successful exact-head
pipeline and authoritative contribution-auditor CLEAR in
`20260805T095156Z-contribution-auditor-hje3.md`. The binding governor Gate 1
approval is recorded in `20260805T102412Z-governor-reviewer-jrk1.md`, and the
merge is recorded in `20260805T102901Z-planner-architect-r11e.md`. The
prerequisite is satisfied; this plan may enter Gate 1.

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

- `README.md` — hero/bullet restructure, separately labeled AI-assistant
  route, and a four-row Evidence and maturity section preserving mechanical
  installation, initial commissioning, governor-attention, and exit-duration
  qualifiers.
- `agf/docs/build/mkdocs.yml` — `theme.palette` (custom primary/accent),
  `theme.features` (`navigation.tabs`, `navigation.tabs.sticky` replacing
  `navigation.top`), restructured `nav:` tree.
- `agf/docs/site/stylesheets/agf.css` — hero component styles, type
  scale, spacing scale.
- `agf/docs/site/index.md` — semantic hero markup/copy and the governed-change
  flow; implementation reuses the existing accessible figure or equivalent
  shape-and-label treatment rather than relying on colour alone.
- `agf/docs/site/reference/palette.md` — measured `blue-light` contrast on
  Material's dark background; no new colour token.
- `agf/docs/build/hooks/check_palette_contrast.py` — extend the existing
  checker and its self-tests with the actual Material slate background and
  the governed dark-theme text/accent pairings used by the hero.
- `agf/docs/build/hooks/check_nav_inventory.py` — new deterministic check that
  extracts all `nav:` leaf URLs, rejects duplicates, and compares the set with
  every eligible Markdown page below MkDocs `docs_dir`; the expected result at
  this plan's baseline is exactly the 42 unique URLs listed above.

Explicitly not affected: `agf/docs/site/overrides/main.html`, font binaries,
the 42 existing page bodies, URLs/file paths, release metadata, and version
files.

No migration needed (presentation-layer only, no schema/data changes).

## Test strategy

Run these exact commands from repository root in this order:

```
agov plan validate docs/plans/20260803T192952Z-pages-readme-visual-refresh-v251.md
```

```
agov validate adopter-docs
```

```
python3 agf/docs/build/hooks/check_palette_contrast.py --self-test
```

```
python3 agf/docs/build/hooks/check_palette_contrast.py
```

```
python3 agf/docs/build/hooks/check_nav_inventory.py --config agf/docs/build/mkdocs.yml
```

```
mkdocs build -f agf/docs/build/mkdocs.yml --strict
```

The plan and adopter-doc checks run before implementation and again at each
slice head. Palette and MkDocs checks run on the Pages theme/hero and navigation
slices. The palette checker extension and its self-tests are a required task in
the theme/hero slice, not pre-existing evidence. The nav-inventory command is a
required task and blocking acceptance check in the navigation slice; it must
report 42 unique leaves at this baseline and fail on any missing, extra, or
duplicate URL. The README slice runs the repository's full governed acceptance
pipeline; it does not claim MkDocs coverage for a root-only file.
- Per `accessibility-rule.md`: new hero CTAs and tab nav need visible
  keyboard focus states, accessible names, and must not rely on colour
  alone (the gate-ledger hero visual already carries shape distinctions
  from the existing governed responsibility palette, so this should
  carry over rather than being designed fresh).
- Per `visible-artifact-verification-rule.md` (this is visible output a human
  is meant to look at): before merge, a human opens the rendered site via
  `mkdocs serve` in both light and dark theme at the core viewports 375×844,
  641×844, 900×1000, and 1280×900. Responsive inspection additionally checks
  both sides of every custom-CSS breakpoint at 639/641, 719/721, 819/821, and
  899/901 CSS pixels. The reviewer confirms no clipped copy, hidden action,
  horizontal page scroll, overlap, lost focus indicator, or layout dependence
  on a particular locally installed font. The Artifact mockup is a stand-in
  for early-stage visual review, not a substitute for that final check against
  the real MkDocs Material build.
- The theme/hero implementation gives "Get started" the existing first-
  governed-change destination and "See the docs" the documentation overview
  destination. The standalone mockup's custom header and tab chrome are
  illustrative composition only; production uses MkDocs Material's existing
  responsive header/navigation affordances, including mobile access, rather
  than carrying over `.chrome-links` or its hide-at-640px behavior.

## Risks

- Dark-mode accent contrast remains a blocking check: `blue-light` must be
  measured on the actual Material background before use, never inferred from
  the mockup.
- System font rendering differs by platform. The implementation must preserve
  layout at the acceptance viewports without depending on a particular locally
  installed font; font vendoring remains out of scope.
- Nav flattening is a real information-architecture change; the DX
  draft's "first 60 seconds" journey should be sanity-checked against
  actual adopter feedback if any exists, not treated as validated by
  this drafting pass alone.

## Rollout

Implementation (once separately authorized) uses three ordered slices on
reviewable branches/MRs per `branch-and-worktree-rule.md` and
`merge-request-workflow-rule.md`:

1. root README structure and claim-preserving copy;
2. Pages palette/theme/hero; and
3. Pages navigation only.

Each slice is independently revertible and receives exact-head CI, visible
artifact verification where applicable, and its own Gate 2 disposition. No
implementation commit lands on this plan branch; no slice may tag, version,
release, or deploy.

---

## Agent drafts (verbatim)

### 1. Technical writing — README restructure

**Superseded — see Decisions 5 and 5a above. Do not implement this draft as
written.** It remains visible only as audit history; the adopted structure has
four evidence rows and a separately labeled AI-assistant route.

Restructured hero: unchanged epigraph, tightened "What AGF gives you"
bullets (each cut to one independent clause), new "## Evidence and
maturity" table with three rows (install time, governor attention, exit
duration), one linking sentence from the hero. Rationale: nothing
deleted, only relocated; the causal clause on "Reviewed learning,
separately governed improvement" was shortened but kept because it is
load-bearing for how the improvement process actually works, not
decorative hedging.

### 2. Developer experience / IA — navigation

**Superseded — see Decision 3 above. Do not implement this seven-tab draft or
its proposed page merges/demotions.** It remains visible only as audit history.

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

**Superseded — see Decision 4 above. Do not treat these as open variants.**
Variant A is final; B and C remain visible only as rejected audit history.

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

**Superseded — see Decisions 1–2 and the implementation/acceptance rules
above. Do not implement this draft as written.** In particular, its font and
dark-accent choices contradict the adopted system-font and governed-token
decisions; it remains visible only as audit history.

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
