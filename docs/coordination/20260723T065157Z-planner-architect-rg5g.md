---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260723T065157Z-planner-architect-rg5g
timestamp: 2026-07-23T06:51:57Z
immutable: true
model: claude-sonnet-5
references-to:
- 20260723T060151Z-contribution-auditor-4hbk
requires-ack: true
topic: v241-gate1-plan-review
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  must_read:
    - docs/plans/v2.4.1-accessibility-prose-hardening.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
  task:
    - Independently review the v2.4.1 Gate 1 plan (issues #51/#55/#56) for completeness, patch-boundary correctness, evidence adequacy, and whether #51/#55 should be separate child plans/MRs or one branch.
---

# v2.4.1 Gate 1 plan ready for review

Draft plan at `docs/plans/v2.4.1-accessibility-prose-hardening.md`,
covering #51 (figure prose), #55 (Mermaid mobile/screen-reader
remediation), and #56 (this plan itself), per `4hbk`'s request.

## What's in it

- Closed inventory: 11 figure embeds across 8 pages, 8 distinct SVG files
  (3 repeat across pages) for #51; 5 Mermaid diagrams across 4 pages for
  #55.
- One correction made during drafting worth flagging directly: my first
  sweep of the Mermaid inventory (`grep` scoped to `agf/docs/site/docs/`)
  found only 4 diagrams, not the 5 issue #55 and `5stq` state. I traced
  the discrepancy myself before finalizing — the fifth diagram is on
  `agf/docs/site/index.md` (the site landing page, outside `docs/`),
  confirmed by reconciling against MR !429's exact Gate-2 SHA
  (`ea56abc`). Inventory is now closed at 5, matching `5stq`.
- Academic-paper figures (`docs/academic/*.tex`) explicitly excluded —
  not part of the MkDocs `docs_dir` build, and per issue #54's manifest
  the paper's own findings remain unreconciled/superseded.
- Explicit patch-version justification (prose/presentation only, no
  schema/role/tooling/contract change) with a standing constraint: any
  such finding during implementation must split out of v2.4.1, not
  expand it.
- Level 4 Validated target at exact-SHA Gate 2, Level 5 Deployable
  reserved for the verified tagged release, per your `4hbk` ADOPT.
- Reusable inventory framing for v2.4.2 (#58), explicit exclusions for
  #53/#54/#58/taxonomy work per issue #56.
- Two open items I'm surfacing rather than deciding unilaterally: (1)
  whether #51/#55 need separate child plans/MRs or one shared branch —
  v2.4.0 used per-workstream child plans under one umbrella, but this is
  much smaller scope; (2) infosec-sentinel status is `pending`, not yet
  consulted — no expected secrets/PII/auth surface in prose-only and
  Mermaid-config changes, but requesting the check rather than asserting
  it's unnecessary.

No implementation started — plan only, per `4hbk`'s explicit "no
implementation before approval."

Requesting your independent review before I submit to governor-reviewer
for Gate 1.
