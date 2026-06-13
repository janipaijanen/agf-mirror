# Plan: Pedagogical Restructure of v2.0 Public Documentation
#
# Timestamp: 2026-06-13T11:26:08Z
# Status: pending-gate1
# References: seq 142 (planner-architect), seq 143 (Codex pedagogical review)

## Intent

The v2.0 public documentation site has the right content but an uneven
learning progression. Validator command names appear before they are taught.
Page order sends readers to "first governed change" before "install". Figures
and tables lack first-reference prose introductions. This plan restructures
the learning path to follow the progressive-disclosure model described in
`content-plan.md`'s pedagogical principles section.

## Problem statement (from seq 143)

- `contracts-and-validators.md` introduces `agov-validate-agents`,
  `agov-validate-contributions`, `agov-validate-infosec`, and `agov-pre-tag-check`
  in a table before any prose teaches what a validator command is.
- `content-plan.md` lists first-governed-change (#6) before install (#7),
  conflicting with the prerequisite column that says "Install done".
- Several pages ask readers to operate artifacts before showing concrete shapes.
- Figures lack first-reference prose on multiple pages.

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

T1 and T2 are small enough for a fast-track commit on `feat/v2-visual-system-figures`
if the governor prefers to keep them in Slice A. Otherwise T1–T7 form a new slice.

## Rollback

All changes are documentation-only. Rollback = revert the commits. No data
or schema is at risk.

## Questions for Gate 1

1. Should T1 (validator intro) be applied to `feat/v2-visual-system-figures`
   immediately (small enough to be part of Slice A's Gate 2 cycle), or should
   it wait for a new slice?
2. Should T7 (page split) be in scope, or is the targeted intro paragraph
   sufficient for now?
3. Are value.md, use-cases.md, and docs/index.md within scope of this
   restructure, or are they a separate content-creation slice?
