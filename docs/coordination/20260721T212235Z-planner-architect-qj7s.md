---
artifacts_owned: []
from: planner-architect
id: 20260721T212235Z-planner-architect-qj7s
immutable: true
kind: review
model: claude-sonnet-5
references-to:
- 20260721T211851Z-planner-architect-hb5j.md
- 20260721T205000Z-governor-reviewer-nqjs.md
- 20260721T205437Z-contribution-auditor-hhkg.md
- 20260721T210529Z-governor-reviewer-bg88.md
- 20260721T210958Z-contribution-auditor-1a7k.md
requires-ack: true
timestamp: 2026-07-21T21:22:35Z
to: governor-reviewer
topic: post-release-public-experience-audit-claude-consolidated
---

# Consolidated independent marketing/UX audit (Claude side)

Three independent read-only reviewers ran per `hb5j`: (A) adopter
conversion/positioning + technical credibility/evidence, (B) information
architecture/evaluator journey + repository landing + release/download
journey + operational artifacts, (C) visual language/figure semantics +
accessibility. Each formed its own judgment first, then compared against
Codex's `hhkg`/`1a7k`. This record consolidates all three; full per-reviewer
detail is in this session's transcript and can be republished on request.

**Self-correction already applied:** reviewer B caught that this session's
own Workstream B docs edit (`release-artifacts.md`) asserted "As of v2.4.0,
every provenance record..." as already-shipped fact, while `agf/VERSION` is
still `2.3.1` and no v2.4.0 tag exists — an implementation-honesty violation
introduced by Claude, in Claude's own open MR !351. Fixed immediately in
commit `1b13a9f` on that MR (reframed around the schema's own
`schema_version` field instead of the unreleased product version).

## BLOCK — factual/trust defects

1. **Self-approval described as a technical impossibility.**
   `index.md:161` ("structural, not aspirational") and similar language
   contradicts AGF's own `systematic-code-review-rule.md`, which classifies
   BLOCK self-checks as *advisory*, not structurally enforced. (Reviewer A;
   matches Codex `hhkg` BLOCK 4.)
2. **"Data never leaves your infrastructure" is false against AGF's own
   rule.** `downstream-impact.md:106` contradicts the shipped
   `external-anonymization-rule.md` boundary table, which classifies
   assistant-provider API calls as *external*. Compounded by `index.md:
   178-179`'s unbounded "without revealing your project" claim. (Reviewer A;
   matches Codex BLOCK 3.)
3. **Level 5 conflated with production/adoption evidence.** `index.md:
   128,132-145` glosses Level 5 as "in production use" and marks all 14
   listed commands Level 5 with no discrimination — defeats the maturity
   model's purpose. (Reviewer A; matches Codex BLOCK 5.)
4. **Version-truth surface is internally contradictory.**
   `release-artifacts.md` (pre-fix) asserted a v2.4.0 contract as shipped
   while `agf/VERSION`, `CHANGELOG.md`, and `release-info.md` all say
   `2.3.1` with an empty `[Unreleased]` section — the same class of defect
   the 2.3.0→2.3.1 recovery existed to fix. (Reviewer B; **already
   corrected**, see above. Not in Codex's reports — Codex's BLOCK 1 flagged
   the *opposite* direction, stale `2.3.1` pins, which two reviewers found
   overstated; see disagreements below.)
5. **Legends name shapes they never draw.** `gate-pipeline.dot` and 5 other
   committed SVGs (governance-loop, coordination-channel,
   operating-responsibility-map, evidence-lifecycle, learning-loop) have
   legend rows reading "(oval shape)"/"(diamond shape)" next to a plain
   rectangular swatch — violates the project's own
   `agf/docs/figures/figures.md:53-55`. (Reviewer C; matches Codex `1a7k`
   BLOCK 1.)
6. **White text on amber/green/grey fails WCAG AA.** `palette.md:35-38`
   explicitly warns against this; 11pt labels use `fontcolor=white` on
   amber/green/grey across governance-loop, evidence-lifecycle,
   operating-responsibility-map, docs-audience-map, gate-pipeline,
   coordination-channel, adoption-exit-flow. Fails the project's own
   `.claude/rules/ui/accessibility-rule.md:16`. (Reviewer C; matches Codex
   BLOCK 3.)
7. **One color, incompatible meanings across figures.** Amber =
   pending/changes-requested *and* incident/observation *and*
   human-only-authority, depending on figure. (Reviewer C; matches Codex
   BLOCK 2.)
8. **Figure terminology contradicts the canonical page.**
   `adoption-exit-flow.dot` shows Freeze/Replace/Decommission;
   `exit-and-decommission.md` defines Pause/Downgrade/Remove as different
   *concepts*, not synonyms. (Reviewer C; matches Codex BLOCK 4; confirmed,
   not refuted.)

## WARN — conversion / IA / accessibility

- Hardcoded OCI version pins (`2.3.1`) in 5+ copy-paste blocks with no
  release-time validator (Reviewer A; downgraded from Codex's BLOCK —
  correct today, fragile, not "already stale").
- Homepage delays value behind two mechanism visuals and a 14-row CLI table
  (Reviewer A + B; matches Codex WARN 6).
- README exit-effort claim omits the "repository-local install" qualifier
  the site version carries (Reviewer A; matches Codex WARN 7).
- Competing evaluator entry points; the actual fit-decision page is three
  nav levels deep behind a prominent "Evaluate" link that leads to reviewer
  *process* documentation instead (Reviewer B; matches Codex WARN 9).
- Three different curated "what to read next" route tables across
  index.md/docs/index.md/evaluate.md (Reviewer B; matches Codex WARN 11).
- README duplicates site content with volatile counts instead of routing
  (Reviewer B; matches Codex WARN 10).
- **Download journey has no macOS/darwin target and no verify-your-download
  step** for a downloader (only build/CI-facing instructions exist).
  (Reviewer B; **not in Codex's reports**.)
- **Misdirected metadata link**: `docs/index.md:108` "Quick reference"
  links to the legacy `agent-governance` GitLab slug instead of the
  canonical `agents-govern-framework`. (Reviewer B; **not in Codex's
  reports**.)
- Grey is overloaded: active-Learn-step color, decommissioned-state color,
  and advisory-status color simultaneously; also an off-palette `#CCCCCC`
  value in `docs-audience-map.dot`. (Reviewer C; **not in Codex's `1a7k`**.)
- Legend edge samples are typed dashes, not real rendered edges (Reviewer
  C; matches Codex `1a7k`).

## INFO

- Maturity Level 2 is named "Surface-defined" in public docs but
  "Schema-defined" in the project's own `implementation-honesty-rule.md`
  (Reviewer A; not in Codex).
- foundry-specific telemetry filenames appear in adopter-facing reference
  docs instead of generic examples (Reviewer A; matches Codex INFO 16).
- CLI help (`agov --help`) has no onboarding pointer for a first-time
  runner (Reviewer B).
- No root `CONTRIBUTING.md` (contributor guidance lives elsewhere); no
  local repo-topics/description artifact (Reviewer B).
- No `<figure>/<figcaption>` semantics; several alt texts are just the
  figure title (Reviewer C; matches Codex `1a7k`).
- Figures hardcode `bgcolor=white`, in tension with `palette.md`'s own
  explicit light-background-only assumption — flagging the tension rather
  than picking a side (Reviewer C).

## Where Claude's reviewers disagree with or add to Codex

- **Codex BLOCK 1 (stale version pins) is overstated by two reviewers
  independently.** The `2.3.1` literals match the current `agf/VERSION`
  today; the real defect runs the *other* direction (see BLOCK 4 above,
  now fixed in !351). Recommend re-ranking Codex's BLOCK 1 to WARN
  (release-fragility, not present incorrectness).
- **Codex BLOCK 2 ("cost statements don't form one intelligible model") is
  partly a false positive.** Reviewer A found no "afternoon installation"
  claim in the audited files; the cited "twenty-minute"/"forty-minute"
  figures are dossier *reading-time* estimates, a different axis from
  install time, and the actual install/governor-attention numbers are
  consistently hedged. Downgrade or narrow this item.
- **Substantive challenge to `1a7k`'s proposed visual-semantics
  specification: reviewer C recommends against it as scoped.** The
  four-namespace token DSL, licensed icon catalogue, five-second-
  comprehension tests, and 4-width×zoom×dark×print screenshot matrix are
  design-system-scale investment for eight Graphviz flowcharts, and collide
  with the project's own `simplicity-rule`. Two namespaces (actor vs.
  workflow-state) are sufficient; a third "evidence-maturity-as-color"
  namespace is self-defeating since maturity is deliberately textual per
  `implementation-honesty-rule` (binary/color-only signaling is exactly
  what that rule forbids). The proposed AI "processor/spark" icon is not
  actually a conventional/familiar symbol by Codex's own stated criterion,
  unlike diamond=decision and oval=terminal (ISO 5807), which are already
  correctly used in the DOT sources — only the *legends* are broken.
  Acceptance-test #7 ("retire figures without a unique reader question")
  and "retire the docs audience map" are remediation decisions, not audit
  findings, and should not be pre-authorized by an assessment pass.
- Two reviewers found real defects **not present in either Codex report**:
  the version-truth contradiction in `release-artifacts.md` (BLOCK 4,
  self-corrected), the macOS/verify gap in the download journey, the
  wrong-slug metadata link, the grey-color overload, and the
  Level-2-naming drift between public docs and the project's own rule.

## Recommendation

Findings converge strongly with Codex's on trust/factual defects (BLOCK 1,
2, 3, 4, 5 above) and several IA/conversion items — treat those as
well-corroborated. Recommend the future Gate 1 plan use a **narrower** first
phase for the visual-semantics work (two namespaces, palette-doc extension
+ legend/contrast/terminology fixes to the 8 existing figures) rather than
`1a7k`'s full specification, revisiting a broader system only if a
concrete need appears. All other sequencing in `hhkg`'s "recommended
post-release order" looks reasonable as a starting point.

No content changes were made beyond the one self-correction on !351
(pre-existing, in-flight release documentation, not the audited public
corpus itself). All remediation remains post-v2.4.0 per `nqjs`/`bg88`.
