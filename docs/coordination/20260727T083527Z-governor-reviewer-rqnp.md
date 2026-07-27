---
artifacts_owned: []
from: governor-reviewer
id: 20260727T083527Z-governor-reviewer-rqnp
immutable: true
kind: decision-record
model: claude-sonnet-5
references-to:
- 20260727T083143Z-governor-reviewer-strh.md
requires-ack: false
timestamp: 2026-07-27T08:35:27Z
to: planner-architect
topic: mdbook-theming-proposal
---

# Human governor confirms: Ayu theme, mdBook migration proposal otherwise unchanged

Human governor has reviewed the theming portion of
`20260727T083143Z-governor-reviewer-strh` and confirmed: **Ayu is the
approved base theme.** No objection raised to the proposed
`additional-css` refinements (wider content column, larger
font-size/line-height) or the three proposed preprocessors
(`mdbook-toc`, `mdbook-admonish`, `mdbook-linkcheck`).

This decides the theming question. The **sequencing question remains
open**: MR !699 (issue #70's operations/continuity runbook) has since
landed fully implemented and tested against MkDocs (pipeline green,
26 doc-hook tests, strict MkDocs build passed) -- it is ready for human
Gate 2 review as-is. Whether to merge !699 on MkDocs now and migrate
the toolchain afterward, or hold it and migrate first, is still an
open decision for the human governor; this record does not resolve
that question, only the theme choice.

No Gate 1 plan has been written for the mdBook migration itself. That
remains blocked on the sequencing decision per
`20260727T083143Z-governor-reviewer-strh`.
