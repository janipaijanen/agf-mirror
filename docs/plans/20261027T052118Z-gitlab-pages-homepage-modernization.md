---
id: 20261027T052118Z-gitlab-pages-homepage-modernization
status: pending-gate1
topic: release-gated-gitlab-pages-homepage-modernization
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      The proposed implementation changes public homepage prose and existing
      CSS only. It adds no executable code, credential path, network request,
      data collection, external asset, dependency, or deployment mechanism.
  - role: contribution-auditor
    status: pending
    reason: >-
      Independent review must confirm that the two-file visual refresh stays
      subordinate to the canonical documentation, does not add unsupported
      third-party claims, and cannot begin before the release freeze and
      coexistence-content prerequisites are satisfied.
  - role: web-ux-reviewer
    status: pending
    reason: >-
      An independent reviewer must inspect the rendered exact-head candidate
      at desktop and narrow-mobile widths, in light and dark themes, including
      keyboard focus, contrast, overflow, navigation clarity, and reduced-motion
      behavior before Gate 2.
affected_paths:
  - agf/docs/site/index.md
  - agf/docs/site/stylesheets/agf.css
depends_on:
  - 20261026T151646Z-next-release-scope-intake
  - 20261027T052500Z-tooling-coexistence-discoverability
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Modernize the GitLab Pages homepage after release content freezes

## Decision requested

Approve a two-file, presentation-only refresh of the public GitLab Pages
homepage after the next release content is stable. The implementation may edit
only `agf/docs/site/index.md` and `agf/docs/site/stylesheets/agf.css`.

This plan does not freeze release scope, choose a version, approve the
coexistence copy, deploy Pages, or authorize implementation before the entry
conditions below are met.

## Entry conditions

Implementation is blocked until all of these are visible on `main`:

1. a Human Governor release-freeze decision for plan
   `20261026T151646Z-next-release-scope-intake` that names the selected `IN`
   scope, semantic version, and exact base SHA;
2. reconciled release changelogs and no unresolved release-content
   disposition at that base;
3. the approved and merged output of
   `20261027T052500Z-tooling-coexistence-discoverability`, including its
   canonical third-party coexistence page and navigation destination; and
4. Gate 1 approval of this exact plan head after all declared advisory rows
   are resolved.

The implementation branch starts from the frozen base or a later descendant
that changes no frozen content assumption. Any changed release scope,
third-party claim, destination URL, or site architecture returns to planning.

## Smallest complete implementation

The homepage currently has a useful hero and established visual system, but
the long sequence below it repeats concepts before offering clear routes for
the visitor's next decision. The implementation will:

- remove or consolidate repeated homepage prose without deleting a unique
  caveat, evidence-status qualifier, cost statement, or release-status fact;
- present three clear routes—**Decide**, **Adopt**, and **Operate**—using
  existing canonical destination pages;
- add one compact **Fits around your stack** route to the canonical
  third-party-plugin coexistence guide produced by the prerequisite plan;
- use a small responsive card/grid treatment built from the existing AGF
  palette tokens and typography; and
- preserve the existing hero's meaning, release-information link, and honest
  capability/evidence vocabulary unless the frozen content sweep explicitly
  requires a factual update.

No JavaScript, dependency, framework, plugin, external font, new image,
animation, analytics, or new page is permitted. Existing MkDocs and GitLab
Pages build/deploy configuration remains unchanged.

## Legacy overlap screen

The deterministic plans-only collision check reports zero assessable
collisions but incomplete coverage because older site plans predate
`affected_paths`. A manual exact-path search found historical homepage and
stylesheet plans. Their shipped output is the baseline this plan preserves;
it is not parallel ownership. Two still-pending files mention the homepage:

- `external-reviewer-fit-assessment-proposals` records its homepage proposal
  as already done and requests no further edit; and
- `multi-agent-coordination-experiment` reserves a future evidence-label
  correction to the **Decisions you can reconstruct** claim. This plan leaves
  that paragraph and claim unchanged. If pruning would touch it, work stops
  until that plan has a binding disposition.

The exact implementation-head collision check and this manual screen must be
repeated after the freeze base is named. No other session may own either of
the two affected paths in parallel.

## Authority and claim boundary

This plan changes presentation, not governance semantics. The homepage may
summarize and link to the coexistence guide, but it must not duplicate its
worked examples or claim that AGF officially integrates with, endorses, or
validates a named third-party tool. AGF continues to own authority, gates, and
durable records; external coding or output tools remain subordinate to those
controls. The prerequisite content plan owns the exact evidence labels and
named-tool facts.

## Acceptance evidence

Gate 2 requires all of the following at the exact implementation head:

1. The diff contains only the two affected paths and introduces no new asset,
   script, dependency, configuration, or generated file.
2. A strict MkDocs build succeeds using the repository's existing supported
   docs build command, and existing documentation/link checks pass.
3. A before/after content inventory shows every removed homepage paragraph is
   either duplicated elsewhere on the same page or retained behind a direct
   canonical link; unique warnings, qualifications, and measured-versus-
   theoretical labels are not silently lost.
4. The Decide, Adopt, Operate, and Fits around your stack routes resolve in the
   rendered build, use descriptive link text, and remain usable without CSS.
5. An independent `web-ux-reviewer`, distinct from the implementer, records a
   rendered exact-head review at desktop and narrow-mobile widths in both
   light and dark themes. The review covers keyboard-only traversal, visible
   focus, contrast, text zoom/reflow, horizontal overflow, link clarity, and
   reduced-motion preference. Every blocking finding is fixed or explicitly
   returned to the Human Governor.
6. The layout uses semantic headings and links rather than clickable generic
   containers, has no content conveyed only by color, and retains readable
   source order when the grid collapses.
7. `git diff --check` and the required exact-head pipeline pass; the current-
   target merge tree is rechecked immediately before Gate 2.

## Collaboration and review separation

Codex owns this plan and may implement the two-file visual slice only after
Gate 1 and the freeze prerequisites. Claude independently audits the plan and
the implementation's factual alignment with the coexistence content. The
`web-ux-reviewer` must be independent of the implementing session. If Claude
and Codex both author the implementation diff, neither cross-approves it;
Gate 2 returns to the Human Governor.

## Risks and rollback

- **Homepage pruning removes an important qualification.** The content
  inventory and documentation-estate freeze sweep make every deletion
  reviewable.
- **A visual refresh invents a second component system.** Reuse the current
  tokens, spacing, typography, focus treatment, and hero conventions; add only
  the minimum card/grid selectors needed by the four routes.
- **Named-tool facts drift after freeze.** The homepage carries no volatile
  feature list; it links to the canonical, version-pinned coexistence guide.
- **Responsive styling looks correct at one width only.** Independent rendered
  checks cover desktop, narrow mobile, zoom/reflow, both themes, and overflow.

Rollback is one revert of the two homepage files. No data, schema, runtime,
CI, dependency, or release artifact format changes.
