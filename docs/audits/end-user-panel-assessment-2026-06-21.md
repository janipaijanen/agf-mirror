---
title: End-user documentation panel assessment
date: 2026-06-21
branch: docs/end-user-panel-assessment
status: draft-gate2
review_scope: end-user framework documents and public documentation endpoints
panel:
  - human-language-and-clarity
  - factual-claims
  - visual-and-information-architecture
---

# End-user documentation panel assessment

This audit reviews the public and adopter-facing documentation after merge of
MR !66 (`docs/downstream-impact-embedded-legend`). It is an assessment and
change proposal, not an implementation patch for the findings below.

## Scope

Repository entry points reviewed:

- `README.md`
- `AGENTS.md`
- `agf/ADOPTING.md`
- `agf/docs/site/content-plan.md`
- `agf/docs/site/index.md`
- `agf/docs/site/value.md`
- `agf/docs/site/use-cases.md`
- `agf/docs/site/docs/index.md`
- `agf/docs/site/docs/start/should-you-adopt.md`
- `agf/docs/site/docs/start/first-governed-change.md`
- `agf/docs/site/docs/adopt/install-and-layouts.md`
- `agf/docs/site/docs/adopt/downstream-impact.md`
- `agf/docs/site/docs/adopt/exit-and-decommission.md`
- `agf/docs/site/docs/concepts/roles-and-gates.md`
- `agf/docs/site/docs/concepts/evidence-and-learnings.md`
- `agf/docs/site/docs/operate/day-to-day.md`
- `agf/docs/site/docs/operate/review-and-release.md`
- `agf/docs/site/docs/advanced/contracts-and-validators.md`
- `agf/docs/site/docs/advanced/customizing-the-framework.md`
- `agf/docs/site/reference/environment-variables.md`
- `agf/docs/site/reference/release-artifacts.md`
- `agf/docs/site/reference/release-info.md`
- `agf/docs/site/reference/palette.md`
- `agf/docs/site/reference/figures.md`
- `agf/docs/site/reference/tooling-conformance.md`
- `agf/docs/site/future/evolution.md`

Public endpoints checked against the deployed GitLab Pages URL all redirected
to GitLab sign-in and returned HTTP 403 from this environment:

- `/`
- `/value/`
- `/use-cases/`
- `/docs/`
- `/docs/start/should-you-adopt/`
- `/docs/adopt/install-and-layouts/`
- `/docs/adopt/downstream-impact/`
- `/docs/adopt/exit-and-decommission/`
- `/docs/start/first-governed-change/`
- `/docs/concepts/roles-and-gates/`
- `/docs/concepts/evidence-and-learnings/`
- `/docs/operate/day-to-day/`
- `/docs/operate/review-and-release/`
- `/docs/advanced/contracts-and-validators/`
- `/docs/advanced/customizing-the-framework/`
- `/reference/environment-variables/`
- `/reference/release-artifacts/`
- `/reference/palette/`
- `/reference/figures/`
- `/reference/tooling-conformance/`
- `/reference/release-info/`
- `/future/evolution/`

Local strict MkDocs build succeeded and produced corresponding local pages.

## Panel Method

Three read-only panelists reviewed the same branch with different lenses:

- Human language, clarity, and end-user phasing.
- Factual claims, versioning, release/tooling statements, and terminology.
- Visual presentation, information architecture, and endpoint-by-endpoint
  navigation.

The integrated findings below merge overlapping findings and remove verified
false positives. For example, `agf/docs/site/reference/release-info.md` exists
on current `origin/master`, so earlier concerns that the release-info page was
absent are not carried as a defect.

## Executive Summary

The documentation has the right raw material: it is candid about cost, includes
exit guidance early, and explains the governance loop without pretending tooling
is the governor. The main problem is that the public reader does not yet get one
unambiguous path. Several pages still preserve bootstrap-era or v1-era details
that conflict with the current `agf/` product boundary, readable coordination
IDs, and Rust-only release-tooling direction.

Recommended priority order:

1. Fix factual contradictions that can misconfigure adopters or agents.
2. Collapse public onboarding into one phased spine:
   evaluate -> decide -> adopt -> first governed change -> operate -> advanced.
3. Demote or bridge legacy-style adopter material instead of letting it compete
   with the newer site path.
4. Create page-specific visual variants only where full reference figures become
   too wide or tall for instructional pages.

## Priority Findings

### P0: Role Authority Is Contradictory

Files:

- `AGENTS.md`
- `agf/docs/site/reference/environment-variables.md`
- `agf/docs/site/docs/adopt/install-and-layouts.md`

Problem:

`AGENTS.md` still says `agents-govern/agents/MANIFEST.yaml` is authoritative.
The active project orientation and current product boundary say
`agf/agents/MANIFEST.yaml` is the AGF-NG role authority. The environment
variable reference also says Foundry points at bootstrap until AGF v2 has its
own authority, which is now stale or at least misleading.

Risk:

Agents and humans can wire validation or role status checks to the bootstrap
reference instead of the current product authority.

Proposed changes:

- Update `AGENTS.md` so only `agents-govern/learnings/foundry.yaml` remains an
  allowed bootstrap exception unless explicitly directed.
- State that `agf/agents/MANIFEST.yaml` is the current AGF-NG role authority.
- In environment docs, separate implementation defaults from governance
  authority. If a tool still defaults to a bootstrap path, document it as a
  tooling gap or override requirement, not as the authoritative policy.

### P0: Public Coordination Guidance Teaches Legacy Sequence Filenames

Files:

- `AGENTS.md`
- `agf/ADOPTING.md`
- `agf/docs/site/docs/start/first-governed-change.md`
- `agf/docs/site/docs/concepts/roles-and-gates.md`
- `agf/docs/site/docs/advanced/contracts-and-validators.md`

Problem:

Several docs still teach `docs/coordination/<seq>-<sender>.md`,
`NNNNN-sender.md`, or CI globs limited to
`docs/coordination/[0-9][0-9][0-9][0-9][0-9]-*.md`. Current coordination
policy prefers readable IDs generated with:

```bash
agov coord brief --id-format readable --from <role>
```

Risk:

New adopters will learn the old high-collision sequence model and CI may skip
current-format readable coordination files.

Proposed changes:

- Make readable IDs the public default.
- Keep sequence filenames documented only as legacy-valid.
- Change CI examples to validate `docs/coordination/*.md`, or explicitly
  explain why a narrower historical glob is being used.
- Update first-governed-change examples to generate IDs with `agov coord brief`
  rather than hard-coded `00001-*` files.

### P0: `agf/ADOPTING.md` Competes With the New Site Path

Files:

- `agf/ADOPTING.md`
- `README.md`
- `agf/docs/site/docs/index.md`

Problem:

`agf/ADOPTING.md` is a long legacy-style practice manual while the newer site
docs present a phased learning path. The two overlap but differ in sequencing,
tone, and current tooling assumptions. Readers can plausibly pick either as the
main adoption path.

Risk:

The user journey forks before installation. The older path carries stale
details such as old release URLs, version examples, sequence filenames, and
missing path promises.

Proposed changes:

- Convert `agf/ADOPTING.md` into a short bridge:
  "For v2.0 adoption, follow the public documentation path: evaluate fit and
  cost, install AGF, complete one governed change, then move into day-to-day
  operation and advanced customization."
- Move any still-current checklist material into the site pages or advanced
  references.
- Put a clear warning at the top if the file remains historical/reference-only.

### P0: Stale Release and Version Examples

Files:

- `agf/ADOPTING.md`
- `agf/docs/site/docs/concepts/roles-and-gates.md`
- `agf/docs/site/reference/release-artifacts.md`

Problem:

`agf/ADOPTING.md` links to
`https://gitlab.com/janipaijanen/agent-governance/-/releases/permalink/latest`,
but the current project remote is
`https://gitlab.com/janipaijanen/agents-govern-framework.git`. It also uses
`framework_version: "0.4.0"` while `agf/VERSION` is `2.0.0`, and says to check
the `framework.md` header even though the version file is the clear source.

Risk:

An adopter can download from the wrong project or record the wrong adopted
framework version.

Proposed changes:

- Replace old release URLs with the current project releases URL.
- Make `agf/VERSION` the single source for framework content version.
- Use `framework_version: "2.0.0"` in current examples.
- Where exact release artifacts are referenced, align wording with the current
  Rust-only tooling release direction.

### P1: Evaluation and Adoption Navigation Are Phase-Confused

Files:

- `README.md`
- `agf/docs/build/mkdocs.yml`
- `agf/docs/site/docs/index.md`
- `agf/docs/site/docs/start/should-you-adopt.md`
- `agf/docs/site/docs/adopt/downstream-impact.md`
- `agf/docs/site/docs/adopt/exit-and-decommission.md`

Problem:

Downstream impact and exit are explicitly "read before adopting," but the nav
places them under `Adopt`, after install. Some page paths send decided users
straight to install without requiring cost and exit review first.

Proposed changes:

- Split public nav into:
  - `Evaluate`: Value, Use cases, Should you adopt?, Downstream impact, Exit.
  - `Adopt`: Install, First governed change.
  - `Operate`: Day-to-day, Review and release.
  - `Advanced`: Contracts, Customizing, References.
- Align `README.md`, docs overview, and page-level "Next" links to that spine.

### P1: Install Page Carries Too Much Concept Load

File:

- `agf/docs/site/docs/adopt/install-and-layouts.md`

Problem:

The install page covers layout, ownership, assistant rule copies, role
commissioning, validator authority, manifests, CI semantics, evidence paths,
and verification. This is all useful, but it is too much before the reader gets
a first successful trial.

Proposed changes:

- Split into "Minimal trial install" and "Production hardening."
- Keep layout, rule copy, and minimal validation in the first path.
- Move commissioning matrix, manifest-driven tooling, and CI blocking semantics
  to operator or advanced pages.

### P1: Tooling Surface Over-Promises `agf coord visibility`

Files:

- `agf/docs/site/docs/advanced/contracts-and-validators.md`
- `agf/docs/site/reference/tooling-conformance.md`

Problem:

The contracts page says hidden coordination obligations are checked by
`agf coord visibility`, while conformance says this command is not wrapped yet
and exists as `mix agov.coord.visibility`.

Proposed changes:

- Change the contracts table to `mix agov.coord.visibility` or "not wrapped by
  `agov` yet."
- Keep the distinction between shipped commands and contract targets visible.

### P1: Broken or Missing Path Promises in `agf/ADOPTING.md`

File:

- `agf/ADOPTING.md`

Examples:

- "Copy this repo's `GOVERNANCE.md`" but no root `GOVERNANCE.md` exists.
- ADR copy commands use `docs/adr/_template.md` while current files are under
  `agf/docs/adr/`.
- `profiles/research-lab.md` is linked but absent.

Proposed changes:

- Replace commands with existing paths.
- Remove promises for absent templates/profiles.
- If those artifacts are intended, add them in a dedicated docs-assets slice.

### P2: Figure Inventory and Page Embeds Drift From the Content Plan

Files:

- `agf/docs/site/content-plan.md`
- `agf/docs/site/index.md`
- `agf/docs/site/docs/index.md`
- `agf/docs/site/docs/adopt/downstream-impact.md`
- `agf/docs/site/reference/figures.md`

Problem:

The content plan mentions planned figures such as `docs-learning-path`,
`adoption-impact-map`, and `agent-communication`, while the shipped site uses a
different set or no figure on some promised pages.

Proposed changes:

- Either update `content-plan.md` to match the shipped figure set, or add the
  missing figure sources in a dedicated visual-docs slice.
- Add a real learning-path figure or compact table to the docs index if the
  ASCII path remains hard to scan.

### P2: Some Reference Figures Are Too Large For Instructional Pages

Files:

- `agf/docs/site/reference/figures/*.svg`
- Pages embedding `gate-pipeline`, `learning-loop`, `evidence-lifecycle`, and
  `adoption-exit-flow`.

Problem:

The figures now have embedded legends, but several are wide or tall enough that
MkDocs default rendering may shrink labels or require awkward scrolling on
mobile/narrow screens.

Proposed changes:

- Keep full diagrams in `Reference > Figures`.
- Create simplified page-specific variants for instructional pages:
  - compact Gate 1/Gate 2 strip for first-run pages;
  - compact learning loop for concepts pages;
  - split adoption and exit panels for downstream impact.

### P2: Operator Pages Need One-Screen Action Checklists

Files:

- `agf/docs/site/docs/operate/day-to-day.md`
- `agf/docs/site/docs/operate/review-and-release.md`

Problem:

The operator docs explain the rhythm, but repeat users need a compact checklist.

Proposed change:

Add a short checklist after the opening workflow section:

```text
Daily checklist:
read new coordination entries;
claim non-trivial work;
decide fast-track vs full plan;
verify branch;
implement only approved scope;
file Gate 2 handoff;
record completion or learning.
```

### P3: Tone Is Mostly Good, But A Few Phrases Are Too Flippant

Files:

- `agf/docs/site/docs/start/should-you-adopt.md`
- `agf/docs/site/docs/adopt/downstream-impact.md`
- `agf/docs/site/docs/operate/review-and-release.md`

Examples:

- "Gates without a governor are theatre."
- "not vibes-based."

Proposed replacements:

- "Gates without an accountable governor do not provide meaningful control."
- "A Gate 2 review is evidence-based."

## Proposed Implementation Slices

1. **Authority and coordination correctness**
   - Fix role authority references.
   - Update readable coordination ID guidance.
   - Fix CI globs for readable coordination files.
   - Correct `agf coord visibility` over-promise.

2. **Release/version/path cleanup**
   - Fix old release URLs.
   - Update version examples to `agf/VERSION`.
   - Remove or repair missing `GOVERNANCE.md`, ADR, and profile path promises.

3. **Public phasing spine**
   - Rework nav into Evaluate / Adopt / Operate / Advanced.
   - Align README and page-level next links.
   - Convert or demote `agf/ADOPTING.md`.

4. **Install and first-run simplification**
   - Split minimal trial install from production hardening.
   - Update first governed change to use readable coordination IDs.

5. **Visual endpoint refinement**
   - Reconcile content plan and shipped figures.
   - Add simplified page-specific figures where full reference diagrams are too
     large.

## Verification Performed

- `uv run --project agents-govern --group docs mkdocs build -f agf/docs/build/mkdocs.yml --strict` passed.
- Live endpoint checks to `https://agent-govern-framework-f5b79e.gitlab.io/...`
  returned HTTP 403 with final URL `https://gitlab.com/users/sign_in`.
- Local generated pages were used for endpoint-equivalent inspection.
- Panelists performed read-only inspection and did not edit files.
