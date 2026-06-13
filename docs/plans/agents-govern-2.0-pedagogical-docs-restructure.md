---
id: agents-govern-2.0-pedagogical-docs-restructure
status: pending-gate1
topic: pedagogical-docs-restructure
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: documentation-only restructuring; no secrets, auth, or security boundary changes
  - role: contribution-auditor
    status: consulted
    reason: seq 143 pedagogical review and seq 155 planner-architect argument reviewed scope and documentation quality risks
---

# Plan: Pedagogical Restructure of v2.0 Public Documentation
#
# Timestamp: 2026-06-13T11:26:08Z
# Status: pending-gate1
# References: seq 142 (planner-architect), seq 143 (Codex pedagogical review),
# seq 155 (planner-architect customer-success argument)

## Intent

The v2.0 public documentation site has the right content but an uneven
learning progression. Validator command names appear before they are taught.
Page order sends readers to "first governed change" before "install". Figures
and tables lack first-reference prose introductions. Seq 155 adds a
customer-success concern: first-time evaluators need enough synthesis on a
single page to build confidence without being forced through a "next, next,
next" chain. This plan restructures the learning path to follow the
progressive-disclosure model described in `content-plan.md`'s pedagogical
principles section while strengthening synthesis pages.

## Problem statement (from seq 143)

- `contracts-and-validators.md` introduces `agov-validate-agents`,
  `agov-validate-contributions`, `agov-validate-infosec`, and `agov-pre-tag-check`
  in a table before any prose teaches what a validator command is.
- `content-plan.md` lists first-governed-change (#6) before install (#7),
  conflicting with the prerequisite column that says "Install done".
- Several pages ask readers to operate artifacts before showing concrete shapes.
- Figures lack first-reference prose on multiple pages.
- `should-you-adopt.md` is not self-contained on cold read: it uses terms
  such as governor and gates before briefly defining the decision being made.
- The landing-page routing table uses audience/persona-ish phrasing where
  reader intent would be more precise.

## Proposed learning path (from seq 143)

The seq 143 review proposes moving from the current ad-hoc ordering to an
explicit seven-phase progression:

1. Decision — what it is, whether to adopt, evidence
2. Downstream impact and exit — cost before action
3. Install — prerequisites, layout choice, file inventory
4. First governed change — concrete end-to-end walkthrough
5. Mental model — roles, gates, coordination channel concepts
6. Daily operation — day-to-day loop, review/release
7. Contracts and advanced customization — validators, schema, CI wiring

The content-plan.md already defines this progression in principle (pages 1–16)
but the current page order does not match.

## Affected files

### Immediate targeted fix (small, could go on `feat/v2-visual-system-figures`)
- `agents-govern-2.0/docs/site/docs/advanced/contracts-and-validators.md`
  — add validator command introduction paragraph before the "What is under
  contract" table

### Navigation and ordering (structural)
- `agents-govern-2.0/docs/build/mkdocs.yml`
  — reorder nav so install (page 7) precedes first-governed-change (page 6)
  per content-plan.md prerequisites

### First-reference discipline (across multiple pages)
- `agents-govern-2.0/docs/site/docs/advanced/contracts-and-validators.md`
  — validator intro paragraph (see targeted fix above)
- `agents-govern-2.0/docs/site/docs/adopt/install-and-layouts.md`
  — validator setup section appears before commands are named; needs a
  forward-reference or validator intro first
- `agents-govern-2.0/docs/site/docs/start/first-governed-change.md`
  — needs concrete filenames and miniature examples before asking readers
  to file plans and coordination entries
- `agents-govern-2.0/docs/site/content-plan.md`
  — correct page order conflict (#6 before #7)

### Figure placement (first-reference prose missing)
Per seq 143's missing teaching artifacts list, the following pages should
embed their associated figures with introductory prose:
- `docs/concepts/roles-and-gates.md` — gate-pipeline + coordination-channel
- `docs/concepts/evidence-and-learnings.md` — evidence-lifecycle (done in seq 144)
- `docs/operate/day-to-day.md` — governance-loop + responsibility-map (partially done)
- `docs/adopt/downstream-impact.md` — adoption-exit-flow
- `docs/adopt/exit-and-decommission.md` — adoption-exit-flow

### Possible page splits (larger scope — governor to decide)
- `contracts-and-validators.md` → split into:
  1. A brief "validators" introduction page in `docs/start/` or `docs/adopt/`
  2. The current page renamed to `advanced/validator-reference.md`

### Possible missing pages (larger scope — governor to decide)
Content-plan.md references pages that may not yet exist as full drafts:
- `value.md` (page 2 — evaluator value proposition)
- `use-cases.md` (page 3 — use cases and non-use cases)
- `docs/index.md` (page 4 — docs learning path index)

### Customer-success / evaluator synthesis additions (from seq 155)
- `agents-govern-2.0/docs/site/docs/start/should-you-adopt.md`
  — add a short standalone lead section ("What you are deciding") so the
  page explains what AGF is and what trade-off adoption makes before the
  adopt/wait/skip checklist.
- `agents-govern-2.0/docs/site/index.md`
  — update the routing table from audience/persona-ish phrasing to reader
  intents while keeping the main nav stage labels.
- `agents-govern-2.0/docs/site/index.md`
  `agents-govern-2.0/docs/site/value.md`
  `agents-govern-2.0/docs/site/docs/start/should-you-adopt.md`
  — apply "expert term, plain job" definitions on first use for the main
  synthesis-page terms: governor, coordination channel, Gate 1, Gate 2,
  learning record, fast-track waiver.

## Migration needed

No database migration. MkDocs nav changes are additive or ordering changes.
Page renames require updating any links that reference the old paths.
Link checking is enforced by `mkdocs build --strict`, which will catch
broken internal links.

## Test strategy

1. `mkdocs build --strict` must pass after every commit.
2. Graphviz `dot -Tsvg` regeneration for any modified DOT sources.
3. Visual inspection of built site navigation and figure placement.
4. Grep for bare command names (`agov-validate-`, `agov-pre-tag-check`) that
   appear in a table before any surrounding paragraph defines what they are.

## Risks

- Page splits create new URLs; external links to old URLs break if the
  old page is removed. Mitigation: keep a redirect or stub at the old URL,
  or mark as acceptable since the site is not yet publicly distributed.
- Reordering nav changes the canonical reading path that other docs may
  reference by page number. Mitigation: all cross-references use relative
  paths, not page numbers.

## Proposed decomposition

| Task | Size | Depends on |
|---|---|---|
| T1: Validator intro paragraph in contracts-and-validators.md | 5 min | none |
| T2: Fix content-plan.md page order (#6/#7 swap in table) | 5 min | none |
| T3: Add first-reference prose + figure embeds to roles-and-gates.md | 20 min | Slice A merged |
| T4: Add first-reference prose + figure embeds to downstream-impact.md, exit-and-decommission.md | 20 min | Slice A merged |
| T5: Strengthen first-governed-change.md with concrete filenames | 30 min | Slice A merged |
| T6: Validate missing pages (value.md, use-cases.md, docs/index.md) — stub or draft | 45 min | governor decides scope |
| T7: Page split of contracts-and-validators.md (if governor approves) | 45 min | T1, T3 |
| T8: Expand should-you-adopt.md with cold-read synthesis lead | 20 min | none |
| T9: Update landing routing table to reader-intent phrasing | 10 min | none |
| T10: Add first-use "expert term, plain job" definitions to synthesis pages | 25 min | T8 |

Slice A has already merged, so T1 and T2 should be handled in this new docs
slice rather than as follow-ups on `feat/v2-visual-system-figures`.

## Rollback

All changes are documentation-only. Rollback = revert the commits. No data
or schema is at risk.

## Questions for Gate 1

1. Approve T1 and T2 as part of this new docs slice now that Slice A is merged?
2. Should T7 (page split) be in scope, or is the targeted intro paragraph
   sufficient for now?
3. Are value.md, use-cases.md, and docs/index.md within scope of this
   restructure, or are they a separate content-creation slice?
4. Approve absorbing seq 155's T8-T10 evaluator-experience additions into
   this Gate 1 plan rather than creating a separate customer-success slice?
