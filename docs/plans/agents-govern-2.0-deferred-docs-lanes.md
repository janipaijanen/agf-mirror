---
id: agents-govern-2.0-deferred-docs-lanes
status: pending-gate1
gate1_approved_by: ""
gate1_approved_at: ""
topic: agents-govern-2.0-deferred-documentation-lanes
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: no secrets, credentials, auth, or sensitive operational data; docs build and presentation assets only
  - role: contribution-auditor
    status: consulted
    reason: plan explicitly separates marketing claims, academic claims, and generated figures so later reviews can audit evidence and avoid overclaiming
  - role: fact-checker
    status: consulted
    reason: active for factual/docs claim review; Gate 2 must include claim sweep for marketing and paper claims
  - role: academic-peer-reviewer
    status: consulted
    reason: active for academic/manual review; Gate 2 must include paper/PDF rigor, citation, and survivability review
---

# Plan: agents-govern-2.0 deferred documentation lanes

**Date:** 2026-06-12T21:05:00Z
**Status:** PENDING GATE 1 APPROVAL
**Branch:** `plan/v2-deferred-docs-lanes`
**Triggered by:** Deferred scope in seq 107 and seq 110; governor request to
continue the deferred work as Claude nears token exhaustion.

---

## Intent

Prepare the three documentation/presentation lanes deferred from the approved
`agents-govern-2.0/` structural work:

1. Design system, visual palette, and CI-generated figures.
2. Academic/manual PDF pipeline under `docs/paper/`.
3. Marketing/evaluation polish for the GitLab Pages site.

This plan does **not** implement the lanes. It creates a gated path so the work
can continue on separate implementation branches after the v2.0 structural
merge is complete.

## Preconditions

Implementation must wait until:

- `feat/agents-govern-2.0-structure` is merged to `master`;
- the merge completion entry is present in `docs/coordination/`;
- the implementation branch starts from that post-merge `master`.

Reason: the target files live under `agents-govern-2.0/`, which is still the
subject of the current structural Gate 2 flow.

## Scope

## Additional activated reviewers

The governor activated `fact-checker` and `academic-peer-reviewer` in seq 117
for the deferred documentation lanes. They are now listed in the
`advisory_consultation` front matter and remain required Gate 2 review
participants for their relevant slices:

- `fact-checker`: claim sweep for marketing/evaluation copy, end-user docs, and
  paper/manual factual claims.
- `academic-peer-reviewer`: paper/PDF rigor, citation, definitions, and
  academic-survivability review before merge.

### Slice A: design system, palette, and figures

Create a small, explicit visual system for v2.0 documentation:

- `agents-govern-2.0/docs/visual/palette.md`
- `agents-govern-2.0/docs/visual/figures.md`
- `agents-govern-2.0/docs/figures/src/`
- generated figure artifacts emitted by CI, not hand-edited binaries

Minimum figure set:

- governance loop: plan, gate, implement, review, merge, learn;
- Gate 1 / Gate 2 pipeline;
- coordination channel and handoff resolution;
- evidence lifecycle: incident -> learning -> rule/tool/doc change;
- adoption impact and exit flow.

Palette requirements:

- colorblind-safe;
- legible in print/PDF;
- usable on light and dark backgrounds;
- restrained enough for academic PDF and operational docs;
- not a one-hue theme.

CI requirements:

- explicitly install Graphviz;
- generate SVG for HTML and PDF-friendly formats where appropriate;
- fail if figure source is invalid;
- keep generated outputs reproducible and reviewable.

### Slice B: academic/manual PDF pipeline

Add `agents-govern-2.0/docs/paper/` as the source for a release-specific PDF
manual or academic-style technical report.

Expected source files:

- `agents-govern-2.0/docs/paper/agents-govern-framework.md`
- `agents-govern-2.0/docs/paper/references.bib`
- `agents-govern-2.0/docs/paper/metadata.yaml`
- `agents-govern-2.0/docs/paper/README.md`

Expected CI shape:

1. Pandoc converts Markdown sources to a TeX intermediate.
2. `kjarosh/latex:2025.1-medium` compiles the TeX with `latexmk`.
3. CI exposes PDF and TeX as review artifacts.

Content requirements:

- title page with version and provenance;
- abstract;
- definitions;
- current framework method and constraints;
- evidence and learning model;
- downstream impact and exit summary;
- limits, non-claims, and future work;
- references.

The PDF must distinguish canonical practice, provisional v2.0 content, and
future hypotheses.

### Slice C: marketing and evaluation polish

Polish the GitLab Pages front door without turning it into hype.

Target pages:

- `agents-govern-2.0/docs/site/index.md`
- `agents-govern-2.0/docs/site/value.md`
- `agents-govern-2.0/docs/site/use-cases.md`
- `agents-govern-2.0/docs/site/docs/start/should-you-adopt.md`

Requirements:

- make the first page answer "what is this, who is it for, why should I care?";
- keep non-use-cases visible;
- keep downstream impact and exit/decommission linked before advanced docs;
- label v2.0 as provisional and v1.x as canonical until promotion;
- avoid vendor-negative claims and unsupported superlatives;
- cite or remove factual/evidence claims.

## Explicitly out of scope

- changing canonical `agents-govern/` v1.x behavior;
- merging v2.0 structural work;
- releasing or tagging;
- adding new agents;
- replacing MkDocs as the HTML site builder;
- using reStructuredText/Sphinx unless a later venue requires it;
- hand-editing generated figure/PDF artifacts without source.

## Implementation strategy

Use three implementation branches after this plan is approved:

1. `feat/v2-visual-system-figures`
2. `feat/v2-paper-pdf-pipeline`
3. `feat/v2-marketing-polish`

Recommended order:

1. Visual system and figures first, because both site and PDF consume them.
2. Paper PDF pipeline second, using the visual assets.
3. Marketing polish third, after the visual identity and documentation evidence
   language are stable.

Each branch gets its own Gate 2 handoff and can be approved independently.

## Verification

Minimum verification per slice:

| Slice | Required checks |
|---|---|
| Visual system / figures | Graphviz installed in CI; figure generation command exits 0; generated artifacts exist; at least one SVG renders in built site or PDF artifact |
| Paper PDF | Pandoc step exits 0; TeX intermediate exists; `kjarosh/latex:2025.1-medium` build exits 0; PDF artifact exists |
| Marketing polish | `mkdocs build --strict` exits 0; claim sweep identifies cited/removed evidence claims; downstream-impact and exit links remain before advanced topics |

Cross-slice checks:

- no `agents-govern/` v1.x files touched;
- no untracked generated artifacts committed unless explicitly listed as
  source-of-truth outputs;
- coordination entry filed for each Gate 2 handoff;
- all claims about current capability use canonical/provisional/hypothesis
  labels.

## Risks

| Risk | Mitigation |
|---|---|
| Visual work expands into branding rabbit-hole | Keep palette and five figures as minimum viable visual system |
| PDF pipeline consumes too much time | Build skeleton pipeline first; content depth can iterate later |
| Marketing copy overclaims maturity | Require claim sweep and verifiable-claims discipline at Gate 2 |
| CI becomes heavy | Separate jobs and artifacts; avoid running PDF job unless docs/paper or figure inputs change |
| Branch collision with v2 structural merge | Implementation waits for structural merge completion and starts from post-merge master |

## Gate 1 ask

Approve this as the planning boundary for the deferred v2.0 documentation lanes.
Approval authorizes implementation of the three separate branches above, in the
recommended order, after the v2.0 structural merge completion lands.
