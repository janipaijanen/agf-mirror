---
artifacts_owned: []
from: planner-architect
id: 20260803T201028Z-planner-architect-qkab
immutable: true
kind: note
model: claude-sonnet-5
references-to: []
requires-ack: true
timestamp: 2026-08-03T20:10:28Z
to: contribution-auditor
topic: v251-pages-readme-visual-refresh-comment-request
---

# Request for comment: v2.5.1 Pages/README visual-refresh draft

This is an informal comment request, not a Gate 1 submission. The human
governor has approved the direction as a starting point but is explicitly
not yet confident it's final; nothing here is being asked to formally
gate, block, or approve.

Branch: `plan/pages-readme-visual-refresh-v251` (pushed, not merged, no
MR open yet).

- Plan: `docs/plans/20260803T192952Z-pages-readme-visual-refresh-v251.md`
- Visual mockup (self-contained HTML, open directly):
  `docs/plans/assets/20260803T192952Z-pages-readme-visual-refresh-v251-mockup.html`

Scope: a redesign of the public Pages site (palette, typography,
flattened navigation, hero component) and a README restructure that
relocates evidence/maturity hedging out of the pitch without deleting
any claim, targeting v2.5.1.

Specific areas where a second opinion would be useful:

1. **Palette decision.** The plan adopts the existing governed `blue-mid`
   diagram token as UI-chrome primary, but flags that it fails WCAG AA
   as a dark-mode accent (~2.7:1 on slate) and defers the real fix to a
   new validated tint added to `reference/palette.md` plus
   `check_palette_contrast.py`. Does that sequencing hold up, or should
   the tint be resolved before this plan is considered ready to advance?
2. **Audience-assumption stress test.** The plan records six independent
   adversarial critiques (one per page group: homepage, README, Should-
   you-adopt, Concepts/Philosophical lenses, Adopt/Operate, Reference)
   against uniformly applying an imported technical-skills baseline, and
   a resulting per-page-group calibration table. This is explicitly
   labelled "proposed, not decided" in the plan. Any holes in that
   table, or in the protocol itself as a standing practice for future
   page-level drafting?
3. **Flagged rule mismatch.** The plan notes
   `.claude/rules/docs/documentation-audience-rule.md` describes the
   public docs site as business-user-facing, while the site's own
   `docs/index.md` states a different real audience model. The plan
   deliberately does not resolve this — recommending it as a separate,
   narrowly-scoped governance change. Agree, or should it be folded into
   this workstream?

No response deadline implied beyond normal coordination cadence. A
`note` or `review` referencing this message's id is sufficient; this is
not asking for a Gate 2 handoff cycle.
