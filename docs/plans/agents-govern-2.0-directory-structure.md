---
id: agents-govern-2.0-directory-structure
status: pending-gate1
gate1_approved_by: ""
gate1_approved_at: ""
topic: v2.0-directory-structure
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: no security boundaries, auth, or PII changes; structural reorganization only
  - role: contribution-auditor
    status: not-applicable
    reason: governor-reviewer Gate 1 covers structural scope; no ownership ambiguity
---

# Plan: agents-govern-2.0 — directory structure

**Date:** 2026-06-12T21:35:00Z
**Status:** PENDING GATE 1 APPROVAL
**Authorized by:** ADR `docs/adr/20260506-agents-govern-2-scope.md`
**Pending scope amendment:** Governor feedback after seq 91 requests a GitLab
Pages site containing both public marketing/evaluation content and progressive
end-user HTML documentation, plus academically durable PDF documentation. This
extends the original ADR's "no marketing rewrite" boundary and therefore
requires explicit Gate 1 approval as part of this amended plan.
Governor feedback after seq 93 requests a pedagogical planner-writer to plan
the HTML end-user contents with the rest of the agents.
Governor feedback after seq 94 requires early end-user documentation for
downstream project impact and how to exit/decommission framework adoption.
Governor feedback after seq 95 asks whether existing figures can be adopted or
regenerated in CI for v2.0 documentation, and whether v2.0 should define its
own color palette.
Governor feedback after seq 98 asks for an explicit HTML/PDF source-toolchain
choice: pure Markdown-to-HTML, MkDocs, Pandoc, or reStructuredText. Governor
feedback after seq 99 asks whether the PDF path should emit TeX from Markdown
and compile it in a pinned release-specific LaTeX container.
**Triggered by:** Adopter feedback (seq 86/87) — root noise, incoherent docs,
positioning unclear, no decommission guide. Constitutes the N≥1 additional
layout-blockage record that supports the v2.0 path (ADR threshold was N≥3
for *in-place* v1.x restructure; v2.0 side-by-side was already authorized).

---

## Intent

Build `agents-govern-2.0/` at repo root, side-by-side with `agents-govern/`
(v1.x canonical, untouched). v2.0 is Provisional per the ADR.

Goals (ADR-scoped):
1. **First-arrival clarity** — reduce root noise from 18 to ~9 files;
   clear reading-order gradient; "is this for you?" positioning.
2. **Better IA** — move governance-meta docs out of root into `docs/governance/`;
   rename confusable files; add missing artifacts (decommission guide).
3. **Broken-link repair** — fix the 8 broken README links identified in the ADR.
4. **Public site and durable documentation** — add source content and CI
   outputs for a GitLab Pages HTML site with a marketing/evaluation front door
   and progressive end-user documentation, plus a rigorous, academically
   survivable PDF documentation track.
5. **Archive/source parity** — v2.0 dist includes all shipped artifacts.

Out of scope:
- Persona-targeted content for unsupported audiences (PM, RDI, SVP) unless
  separately approved with evidence
- New agents
- YAML-LD / linked-data work
- Modifying any v1.x file

Scope amendment requested for Gate 1:
- Public-facing marketing/evaluation content for v2.0, focused on what the
  framework is, why it matters, credible use cases, adoption path, and evidence.
- Progressive end-user HTML documentation on the same GitLab Pages site,
  moving from first concepts to adoption, operation, advanced configuration,
  and future/evolution material.
- A pedagogical planner-writer lane for the end-user HTML docs. Recommended
  assignment: temporarily activate the existing `documentation-writer` role
  for v2.0 docs pedagogy and content sequencing, without granting it authority
  over source code, governance decisions, or final approval.
- Early end-user documentation for downstream impact and exit/decommission:
  what changes in an adopting project, what obligations or ongoing costs are
  introduced, which files and CI jobs are touched, and how to remove the
  framework cleanly if adoption stops.
- CI-regenerated figures and a v2.0 visual system: reusable diagrams for the
  public site, progressive docs, and PDF; generated from source where possible;
  governed by a small colorblind-safe, print-aware palette.
- Documentation source/toolchain decision: Markdown is the canonical prose
  source format; MkDocs builds GitLab Pages HTML; Pandoc builds the academic
  PDF pipeline from Markdown paper sources via a committed/reviewable TeX
  intermediate; `kjarosh/latex:2025.1-medium` compiles the release PDF;
  reStructuredText is out of scope unless a future venue explicitly requires it.
- Academic-style PDF documentation that can be cited, archived, and read
  offline: stable title page, version/provenance, abstract, table of contents,
  definitions, method, claims/evidence limits, references, appendix material,
  and clear distinction between current capability and future hypotheses.

---

## Role orchestration for docs planning

The GitLab Pages end-user documentation should be planned by a pedagogical
planner-writer before page drafting begins.

Recommended role assignment:

| Role | Status in current MANIFEST | Assignment for this work |
|---|---|---|
| planner-architect | active | Owns site IA, page inventory, dependency order, and Gate 1 plan coherence |
| documentation-writer | skipped | Recommended temporary activation for pedagogical sequencing, audience fit, page objectives, examples, tone, and end-user docs outline |
| coder-tester | active | Implements files, build configuration, CI jobs, and validates generated artifacts after Gate 1 approval |
| fact-checker | suspended | Optional advisory review of factual/existence claims before Gate 2 if governor activates or delegates the check |
| academic-peer-reviewer | skipped | Optional advisory review of `docs/paper/` rigor before Gate 2 if governor activates or delegates the check |
| governor-reviewer | active | Decides whether temporary role activation and the amended scope are approved |

`documentation-writer` does not approve its own content. It produces a
pedagogical content plan and later drafts/revises documentation under the
approved Gate 1 scope; governor-reviewer remains the approving role.

Required planning output before writing full pages:
- `docs/site/content-plan.md` in the v2.0 tree, covering audiences, learning
  objectives, prerequisites, page sequence, examples, glossary needs, status
  labels, and review checkpoints.

Existing figure inventory:
- No committed SVG/PNG/PDF figure assets are present in the current
  `agents-govern/` checkout.
- Reusable source exists in `agents-govern/docs/agent-communication-0.26.0.md`
  as a generated Mermaid communication graph.
- Reusable generator exists in `agents-govern/src/agents_govern/graph.py`,
  supporting Graphviz SVG/PNG/PDF output and Mermaid output via
  `agov-validate-agents --graph`.
- Graphviz `dot` is not available in the current local shell. CI must
  explicitly install Graphviz for the `figures-build` job so SVG/PNG/PDF
  figure generation is guaranteed in review artifacts. `.dot`/Mermaid
  fallbacks are acceptable only for local/offline development, not CI.

Documentation toolchain decision:
- Use **Markdown as canonical source** for end-user docs, marketing/evaluation
  pages, figure source pages, and academic PDF/manual source.
- Use **MkDocs + MkDocs Material** for GitLab Pages HTML. This matches the
  existing `agents-govern/pyproject.toml` docs dependency group and the
  framework's release history of strict `mkdocs build` checks.
- Use **Pandoc** for academic PDF generation from `docs/paper/*.md`, with
  `docs/paper/references.bib` as citation input when citations are needed.
- PDF builds should be two-stage in CI:
  1. Pandoc converts Markdown sources into a release-specific TeX intermediate.
  2. `kjarosh/latex:2025.1-medium` compiles the TeX with `latexmk`.
- The TeX intermediate is a review/debug artifact for each release. It may be
  committed if the release process wants exact source traceability, or emitted
  as a CI artifact if generated outputs are preferred; either way, Gate 2 must
  inspect or reference it.
- Do **not** use raw one-off Markdown-to-HTML scripts for the main site; they
  would recreate navigation, link checking, theme, search, and extension
  concerns already handled by MkDocs.
- Do **not** adopt reStructuredText/Sphinx for v2.0 unless a later academic
  venue or standards process requires it. The current repo has no
  reStructuredText footprint, and introducing a second prose source format
  would increase contributor and synchronization cost.

## File mapping

### Root — keep (~9 files, platform-convention justified)

| v1.x file | v2.0 location | Change |
|---|---|---|
| README.md | README.md | Rewritten: 3-para pitch + "is this for you?" + links |
| ADOPTING.md | ADOPTING.md | Carried over; broken links repaired |
| framework.md | framework.md | Carried over unchanged |
| framework.yaml | framework.yaml | Version updated to 2.0.0 |
| CHANGELOG.md | CHANGELOG.md | Fresh; v2.0 entries only |
| LICENSE | LICENSE | Carried over unchanged |
| CODE_OF_CONDUCT.md | CODE_OF_CONDUCT.md | Carried over unchanged |
| CONTRIBUTING.md | CONTRIBUTING.md | Carried over; paths updated |
| SECURITY.md | SECURITY.md | Carried over unchanged |
| VERSION | VERSION | 2.0.0 |

### Move to docs/governance/ (currently at root or docs/ root)

| v1.x file | v2.0 location | Rename? |
|---|---|---|
| GOVERNANCE.md | docs/governance/governance.md | lowercase only |
| LEARNINGS.md | docs/governance/learning-system.md | Yes — describes the system |
| LESSONS-LEARNED.md | docs/governance/retrospective.md | Yes — it's a retrospective |
| MANIFEST.md | docs/governance/manifest.md | lowercase only |
| DISCLAIMER.md | docs/governance/disclaimer.md | lowercase only |
| docs/ANONYMIZATION-NOTICE.md | docs/governance/anonymization-notice.md | move + lowercase |

### docs/adopt/ — existing + new

| File | Status | Notes |
|---|---|---|
| docs/adopt/compliance-profile.md | carry over | unchanged |
| docs/adopt/decommission.md | **NEW** | How to remove adoption cleanly |

### docs/reference/ — new subdirectory

| File | Status | Notes |
|---|---|---|
| docs/reference/quick-reference.md | **NEW** | One-page role/pipeline/checklist summary |

### docs/site/ — GitLab Pages site source (new)

Source pages for the GitLab CI generated HTML site. This is both the public
marketing/evaluation surface and the progressive end-user documentation for
v2.0. The first pages should sell the value clearly and credibly; the docs
path should then guide users from first concepts through adoption, daily
operation, advanced configuration, and future/evolution material without
overstating Provisional claims.

| File | Status | Notes |
|---|---|---|
| docs/site/index.md | **NEW** | Public landing page: what AGF is, why now, proof/evidence, canonical-vs-Provisional status |
| docs/site/value.md | **NEW** | Concise value proposition and problem framing |
| docs/site/use-cases.md | **NEW** | Credible use cases and non-use cases |
| docs/site/content-plan.md | **NEW** | Pedagogical plan for end-user HTML docs: audiences, objectives, progression, examples, review checkpoints |
| docs/site/docs/index.md | **NEW** | End-user documentation index with beginner-to-advanced reading path |
| docs/site/docs/start/should-you-adopt.md | **NEW** | Lightweight decision page; who should use it now, who should wait |
| docs/site/docs/start/first-governed-change.md | **NEW** | Beginner walkthrough of one small change through Gate 1/Gate 2 |
| docs/site/docs/adopt/install-and-layouts.md | **NEW** | Practical adoption setup and Layout A/B choices |
| docs/site/docs/adopt/downstream-impact.md | **NEW** | Required early page: project impact, touched files, CI/process changes, obligations, costs, and risks |
| docs/site/docs/adopt/exit-and-decommission.md | **NEW** | Required early page: how to stop using the framework cleanly and verify removal |
| docs/site/docs/concepts/roles-and-gates.md | **NEW** | Core mental model: agents, authority, gates, coordination |
| docs/site/docs/concepts/evidence-and-learnings.md | **NEW** | Why learnings exist; adopter-owned vs upstream-owned records |
| docs/site/docs/operate/day-to-day.md | **NEW** | Practical operating loop for active projects |
| docs/site/docs/operate/review-and-release.md | **NEW** | Governor review, release, upgrade, and audit-trail expectations |
| docs/site/docs/advanced/contracts-and-validators.md | **NEW** | Schemas, invariants, binding contracts, validator integration |
| docs/site/docs/advanced/customizing-the-framework.md | **NEW** | How far adopters may tune roles/rules without breaking semantics |
| docs/site/future/evolution.md | **NEW** | Clearly marked Provisional hypotheses: where v2 could evolve next |

### docs/site/assets/ — visual assets and design tokens (new)

| File | Status | Notes |
|---|---|---|
| docs/site/assets/design-system.md | **NEW** | v2.0 visual principles, color tokens, diagram rules, accessibility/print requirements |
| docs/site/assets/styles/palette.css | **NEW** | CSS custom properties for GitLab Pages HTML theme |
| docs/site/assets/styles/palette.json | **NEW** | Machine-readable color tokens for figure/PDF generation |
| docs/site/assets/figures/README.md | **NEW** | Figure inventory, source files, generated outputs, regeneration command |
| docs/site/assets/figures/agent-communication.md | generated/adapted | v2.0 communication graph source/output page |
| docs/site/assets/figures/governance-loop.md | **NEW** | Source for governance feedback loop figure |
| docs/site/assets/figures/gate-pipeline.md | **NEW** | Source for Gate 1/Gate 2 pipeline figure |
| docs/site/assets/figures/adoption-impact-map.md | **NEW** | Source for downstream impact map |
| docs/site/assets/figures/exit-flow.md | **NEW** | Source for exit/decommission flow |
| docs/site/assets/figures/docs-learning-path.md | **NEW** | Source for progressive docs reading path |
| docs/site/assets/figures/evidence-lifecycle.md | **NEW** | Source for learnings/evidence lifecycle figure |

### docs/paper/ — academic PDF source (new)

Source for a durable, citable PDF artifact. This is not a brochure; it is the
framework's rigorous long-form documentation with enough structure to survive
academic, standards, or architecture-review scrutiny.

| File | Status | Notes |
|---|---|---|
| docs/paper/agents-govern-framework.md | **NEW** | Main long-form manual/paper source |
| docs/paper/references.bib | **NEW** | Bibliography or citation source for cited works and project evidence |
| docs/paper/appendix-evidence.md | **NEW** | Evidence limits, case summaries, validation status, and known gaps |
| docs/paper/appendix-contracts.md | **NEW** | Contract, schema, and validator reference material suitable for PDF |
| docs/paper/README.md | **NEW** | Maintainer notes for PDF build and citation/provenance rules |

### docs/build/ — docs build configuration (new)

| File | Status | Notes |
|---|---|---|
| docs/build/mkdocs.yml | **NEW** | GitLab Pages HTML site build config; includes marketing pages and end-user docs |
| docs/build/pdf.yml | **NEW** | Pandoc-to-TeX + pinned `kjarosh/latex:2025.1-medium` PDF build config |
| docs/build/figures.yml | **NEW** | Figure regeneration config: Mermaid/Graphviz inputs, output formats, palette source |
| docs/build/README.md | **NEW** | Maintainer notes for local docs build and CI parity |

### .gitlab/ — generated documentation jobs (new/update)

| File | Status | Notes |
|---|---|---|
| .gitlab-ci.yml | update/carry over | Add `figures-build`, `site-html-build`, `pages`, and `paper-pdf-build` jobs for v2.0 artifacts; `figures-build` explicitly installs Graphviz |
| .gitlab/ci/docs.yml | **NEW** | Optional include file if the pipeline is split by concern |

### docs/contracts/ — reorganize from docs/ root

| v1.x file | v2.0 location |
|---|---|
| docs/binding-contract.md | docs/contracts/binding-contract.md |
| docs/contract-stability.md | docs/contracts/contract-stability.md |
| docs/integration-contract.md | docs/contracts/integration-contract.md |

### Carried over unchanged (same relative paths)

- `agents/` — all 12 YAMLs + MANIFEST.yaml
- `rules-template/` — all rules
- `schemas/` — JSON schemas + invariants
- `scripts/` — generate-schemas.py
- `skills/` — all 7 skills
- `project-specific-examples/` — all stacks
- `learnings/` — foundry.yaml + _template.yaml + .adopter-mapping.example
- `docs/adr/` — all ADRs
- `docs/agent-communication-*.md` — generated graphs
- `pyproject.toml` + `src/agents_govern/` — tooling
- `.pre-commit-config.yaml`
- `.gitlab/` — except docs CI additions listed above

---

## New files (content to write)

### docs/adopt/decommission.md

Covers:
1. Files to delete (agent YAMLs, rules-template copy, skills copy, learnings codename file)
2. Hooks to remove (.pre-commit-config.yaml, .claude/settings.json changes)
3. CI changes to revert (validator jobs, path-scoped triggers)
4. Git history note (commit the removal as a single atomic commit with a clear message)
5. Verification: run `agov-validate-agents` before removal to confirm current state;
   confirm no remaining `agov-*` references in CI config after removal

### docs/reference/quick-reference.md

One-page summary covering:
- Agent roles (Phase 1 names, responsibilities, 1 sentence each)
- Pipeline steps (Gate 1 / Gate 2 / fast-track)
- Coordination channel kinds (9 kinds, 1 line each)
- Key rules checklist (5 most-referenced rules)

### docs/site/* GitLab Pages HTML content

Creates a coherent public site with two connected lanes:

1. **Marketing/evaluation lane** — what AGF is, why it matters, evidence,
   adoption status, credible use cases, and explicit non-use cases.
2. **Progressive end-user documentation lane** — beginner-to-advanced docs for
   actual users.

The end-user documentation path is:

1. **Docs index** — where to start, which path to follow, and what is
   available now versus Provisional.
2. **Start** — whether to adopt, first governed change, install/layout choices.
3. **Adopt safely** — downstream project impact and how to exit/decommission.
4. **Concepts** — roles, gates, evidence, coordination, learnings.
5. **Operate** — day-to-day work, review, release, upgrade.
6. **Advanced** — schemas, validators, binding contracts,
   customization boundaries.
7. **Evolution** — Provisional hypotheses for where the framework may evolve,
   explicitly labelled as non-binding and not yet canonical.

`docs/site/content-plan.md` is the control artifact for this lane. It should
answer, before full page drafting:
- Who is the page for?
- What should the reader be able to do or understand after reading it?
- What prerequisites does the page assume?
- Which examples or diagrams does the page need?
- Which claims need fact-checking?
- Which pages are beginner, intermediate, advanced, or future-facing?
- Where downstream-impact and exit/decommission guidance appears in the first
  adoption path, before advanced topics.

HTML source/build decision:
- Source format: Markdown (`.md`).
- HTML builder: MkDocs in strict mode.
- Site output: GitLab Pages artifact.
- Theme: MkDocs Material or a minimal MkDocs theme layer using the v2.0 palette
  tokens; avoid bespoke static-site scripts.
- Navigation: explicit MkDocs navigation so marketing pages and progressive
  docs are visibly separate but connected.

### docs/site/docs/adopt/downstream-impact.md

Required early end-user page. Covers:
- Files/directories the framework adds or changes in Layout A and Layout B.
- CI, hooks, validation, and review-gate changes.
- Human/agent role obligations introduced by adoption.
- Ongoing maintenance costs: learning records, upgrades, review overhead,
  coordination hygiene, and version pinning.
- Boundaries: what the framework does not control, enforce, or guarantee.
- Risk tradeoffs and signs that adoption is too heavy for the project.

### docs/site/docs/adopt/exit-and-decommission.md

Required early end-user page. Cross-links `docs/adopt/decommission.md` and
translates it into a practical user flow:
- Decide whether to downgrade compliance, pause usage, or remove adoption.
- Remove copied agent definitions, rules, skills, hooks, CI jobs, coordination
  conventions, and local learning files where appropriate.
- Preserve audit history without pretending the project remains governed.
- Verify removal with search checks for `agov`, `agents-govern`, copied rule
  paths, CI includes, and assistant command/skill references.
- Record the exit decision in project history.

### docs/site/assets/* visual system and generated figures

v2.0 should define its own palette, but as documentation infrastructure rather
than decoration. The palette must:
- Use named semantic tokens (`--agf-role-production`, `--agf-role-governance`,
  `--agf-risk-warning`, `--agf-evidence`, etc.) rather than hardcoded ad hoc
  hex values in pages or diagrams.
- Be colorblind-safe and distinguishable in grayscale/print.
- Work in HTML, SVG, PNG, and PDF outputs.
- Preserve meaning without color: diagrams must use labels, line styles,
  shapes, or icons in addition to color.
- Be modest enough for academic PDF use and distinctive enough for the public
  GitLab Pages site.

Candidate figures to regenerate in CI:
1. **Agent communication graph** — adopt from the existing `agov-validate-agents
   --graph` generator; v2 should generate SVG/PNG/PDF where Graphviz is
   available and save Mermaid or `.dot` source for traceability.
2. **Governance feedback loop** — shows plan, implement, review, learn,
   update framework.
3. **Gate pipeline** — shows Gate 1, implementation, validation, Gate 2, merge.
4. **Adoption impact map** — shows downstream files, CI/hooks, roles, and
   obligations introduced by adoption.
5. **Exit/decommission flow** — shows downgrade, pause, remove, verify, record.
6. **Progressive docs learning path** — shows beginner-to-advanced HTML docs.
7. **Evidence lifecycle** — shows learning records becoming proposals, plans,
   reviews, releases, and future framework changes.

Every figure must have text before the diagram explaining what it shows and
what the reader should take away. Generated figures should include source files
and generated outputs in CI artifacts so PDF/site rendering can be inspected.

### docs/paper/* academic PDF content

Creates a durable PDF artifact with:
1. Title page, version, commit/provenance, license, and citation suggestion.
2. Abstract and scope statement.
3. Definitions and conceptual model.
4. Framework method: roles, gates, coordination, evidence, contracts.
5. Operational guidance and adoption model.
6. Validation status, evidence base, limitations, and threats to validity.
7. References and appendices.

The public site may be persuasive. The PDF must be precise, citable, and
careful about claims.

PDF source/build decision:
- Source format: Markdown (`docs/paper/*.md`).
- PDF pipeline: Pandoc emits TeX from Markdown, then
  `kjarosh/latex:2025.1-medium` runs `latexmk` to produce the PDF.
- Bibliography support: `references.bib`.
- Output: CI artifact suitable for archival review.
- Review artifacts: generated `.tex`, generated `.pdf`, build log, and any
  generated figures included in the PDF.
- Relationship to HTML: PDF may reuse definitions and figures, but it is not
  generated by scraping the MkDocs site. It has its own academic structure:
  title page, abstract, method, limitations, references, and appendices.

---

## Affected files summary

| Location | Action | Count |
|---|---|---|
| `agents-govern-2.0/` root | New directory | — |
| Root files carried over | Copy + minor edits | 9 |
| Root files moved to docs/governance/ | Copy + rename | 6 |
| docs/ root files moved to docs/contracts/ | Copy | 3 |
| docs/adopt/ | Carry over + 1 new | 2 |
| docs/reference/ | 1 new | 1 |
| docs/site/ | New GitLab Pages site source | 17 |
| docs/site/assets/ | New visual system + figure sources | 11 |
| docs/paper/ | New academic PDF source | 5 |
| docs/build/ | New docs/figures build config | 4 |
| .gitlab / CI | Update/add docs build jobs | 1-2 |
| All other dirs | Copy unchanged | ~8 dirs |

**v1.x is untouched throughout.**

---

## Implementation order

1. Create `agents-govern-2.0/` skeleton (dirs only)
2. Copy unchanged directories
3. Copy + lightly edit root files (broken link repair in README, ADOPTING)
4. Copy + rename governance-meta files to `docs/governance/`
5. Write `docs/adopt/decommission.md`
6. Write `docs/reference/quick-reference.md`
7. Write `docs/site/content-plan.md` as the pedagogical plan for the end-user
   HTML docs, led by `documentation-writer` if governor-reviewer temporarily
   activates or delegates that role
8. Write `docs/site/` GitLab Pages site pages: marketing/evaluation front door
   plus progressive end-user HTML docs from first concepts through advanced
   operation
9. Write `docs/site/docs/adopt/downstream-impact.md` and
   `docs/site/docs/adopt/exit-and-decommission.md` before advanced docs pages
10. Define `docs/site/assets/design-system.md` and palette tokens before
    drawing final figures
11. Add figure source files and CI regeneration config for communication graph,
    governance loop, gate pipeline, adoption impact, exit flow, docs learning
    path, and evidence lifecycle
12. Write `docs/paper/` academic PDF source and appendices
13. Add `docs/build/` docs build configuration
14. Add GitLab CI jobs for figure regeneration, GitLab Pages HTML site, and
    academic PDF artifacts; HTML uses MkDocs strict mode; PDF uses Pandoc to
    emit TeX and `kjarosh/latex:2025.1-medium` to compile with `latexmk`; and
    `figures-build` must explicitly install Graphviz and fail CI if SVG/PNG/PDF
    figure rendering fails
15. Write new README.md (3-para pitch + positioning)
16. Update internal cross-links in all moved/renamed files
17. Update `dist.py` for v2.0 archive generation while preserving v1.x behavior
18. Validate: `agov-validate-agents` passes from v2.0 root
19. Validate docs: CI figure generation succeeds with Graphviz-rendered
    SVG/PNG/PDF outputs; GitLab Pages HTML site build succeeds in strict mode;
    Pandoc emits the release TeX intermediate; `kjarosh/latex:2025.1-medium`
    compiles the PDF artifact successfully
20. Spot-check generated outputs: public site has clear value proposition and
    Provisional status; progressive end-user docs are navigable from beginner
    to advanced topics; downstream-impact and exit/decommission pages are
    reachable before advanced topics; figures are legible in HTML and PDF;
    PDF has provenance, table of contents, citations, limitations, and
    appendices.

---

## Downstream impact

**Classification:** ADDITIVE (Provisional)

- **Foundry:** New directory alongside v1.x. v1.x untouched. No existing
  pipeline or tooling broken.
- **Direct adopters:** New Provisional option. v1.x remains canonical.
  No migration required. No existing contract removed.
- **Documentation consumers:** New GitLab Pages site and academic PDF make the
  framework easier to evaluate and use from first concepts through advanced
  operation. Both must clearly mark v2.0 as Provisional.
- **Downstream:** None until v2.0 promotes to canonical.
- **Migration:** Not required. v2.0 is opt-in.

---

## Risks

- Cross-link rot: files moved/renamed need all internal links updated.
  Mitigation: step 8 (link audit) is explicit in the implementation order.
- `dist.py` must be pointed at v2.0 root for the v2.0 archive build;
  v1.x dist target must be preserved. Mitigation: separate dist invocation
  per version, not a shared build.
- v2.0 Provisional banner must be visible at README entry point so adopters
  mid-evaluation know v1.x is still canonical. Mitigation: first line of
  v2.0 README.
- Docs drift between repository Markdown, GitLab Pages site, end-user docs,
  and PDF outputs could recreate the current first-arrival confusion in a new
  format. Mitigation: keep `docs/site/` and `docs/paper/` cross-linked, share
  definitions where practical, and gate CI on both artifacts.
- The `docs/site/future/evolution.md` page could read as a commitment rather
  than hypothesis. Mitigation: label the page Provisional/non-binding and
  separate "available now" from "possible evolution."
- Marketing content could overclaim beyond the evidence base. Mitigation:
  include non-use cases, Provisional status, and links to evidence/limits.
- Academic PDF content could become too lightweight to survive serious review.
  Mitigation: require provenance, definitions, claim boundaries, evidence
  limits, references, and appendices before Gate 2.
- End-user docs could become a pile of pages rather than a learning path.
  Mitigation: require `docs/site/content-plan.md` before full page drafting,
  with explicit learning objectives, prerequisites, progression, examples,
  and fact-check points.
- Adoption docs could sell the framework without explaining downstream burden
  or reversibility. Mitigation: require early downstream-impact and
  exit/decommission pages in the first adoption path, before advanced topics.
- Figures could drift from current agents, gates, or docs structure.
  Mitigation: regenerate data-derived figures in CI, keep source files beside
  outputs, and require figure source review at Gate 2.
- A custom palette could reduce accessibility or academic seriousness if it is
  treated as branding decoration. Mitigation: define semantic palette tokens,
  colorblind/grayscale requirements, and non-color encodings before final
  figures are drawn.
- Multiple documentation source formats could create drift and contributor
  confusion. Mitigation: keep Markdown as canonical source, use MkDocs for
  HTML and Pandoc for PDF, and avoid reStructuredText/Sphinx unless a future
  external venue requires it.

---

## Rollback

`agents-govern-2.0/` is a new directory. Rollback = `rm -rf agents-govern-2.0/`.
No v1.x file is modified, so rollback has zero blast radius on existing adopters.
