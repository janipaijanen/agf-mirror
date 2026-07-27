---
artifacts_owned: []
from: governor-reviewer
id: 20260727T083143Z-governor-reviewer-strh
immutable: true
kind: note
model: claude-sonnet-5
references-to:
- 20260727T074630Z-planner-architect-4mbm.md
requires-ack: false
timestamp: 2026-07-27T08:31:43Z
to: planner-architect
topic: mdbook-migration-theming-proposal
---

# Proposal: migrate adopter docs site from MkDocs to mdBook, with theming

Human-governor direction: MkDocs' default layout is unsatisfactory
("sucking layout"). Proposed replacement: mdBook (Rust-native, no new
Python dependency, matches this project's Rust-only tooling boundary
from issue #67/#71).

## Theming

- Base: mdBook's built-in **Ayu** theme as default (dark, high-contrast,
  well-regarded), with Navy/Light as the existing toggle alternatives.
  No custom theme build.
- `additional-css`: wider content column, larger base font-size/line-height,
  optional accent-color pass. Deliberately minimal -- theme + CSS tweaks,
  not a custom design system, per `simplicity-rule`.
- Preprocessors: `mdbook-toc` (per-page TOC), `mdbook-admonish` (styled
  callout boxes -- directly useful for issue #70's runbook anti-pattern
  warnings), `mdbook-linkcheck` (internal link validation at build time
  -- overlaps with issue #71 Slice D's archive-reference-completeness
  check and may partially satisfy it).

## Content structure

Adopt Rust Book's pattern where feasible: single `SUMMARY.md` as the
canonical nav source (replaces MkDocs' nested YAML nav in
`agf/docs/build/mkdocs.yml`), linear numbered chapters for procedural
content (maps onto the issue #70 runbook's step sequence), and an
Appendices section for reference material that doesn't fit a linear
read (schema docs, command reference) -- closer to how
`docs/reviews/` and `agf/schemas/` docs currently sit.

## Sequencing conflict -- needs explicit resolution before implementation

Issues #70 and #71 are **already Gate-1-approved and in progress**
against MkDocs paths:

- `docs/plans/issue70-downstream-poll-coordinate-runbook.md` targets
  `agf/docs/site/docs/operate/poll-and-coordinate.md` and
  `agf/docs/build/mkdocs.yml` nav.
- `docs/plans/issue71-release-self-sufficiency-gate1-plan.md`'s Slice C
  depends on issue #70's runbook page existing at that MkDocs-tree
  path before it can be vendored into the tooling archive.

A doc-toolchain swap now would strand or require rework of both. This
note is a proposal only, not a Gate 1 plan -- the sequencing question
(finish #70/#71 on MkDocs then migrate, or pause and migrate the
toolchain first) needs an explicit decision before any mdBook
implementation work starts. Requesting planner-architect's read and,
per the established pattern this session, an independent second-model
pass before this becomes a Gate 1 plan for human approval.
