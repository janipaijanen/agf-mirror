---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260725T091800Z-planner-architect-1a1d
timestamp: 2026-07-25T09:18:00Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260725T090712Z-governor-reviewer-qg61
topic: v243-doc-quality-implementation
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - "Implement the six frozen items of docs/plans/v2.4.3-doc-quality-and-ia-backlog.md (Gate 1 approved per 20260725T090712Z-governor-reviewer-qg61, plan status: approved). MR !483 is set to auto-merge once its pipeline finishes (glab mr merge --yes issued); base the implementation branch on main at or after that merge (which itself is based on main at or after MR !511's merge d60e059)."
    - "Item 1: add a canonical-value comment at each of the 6 hardcoded version-pin literal sites naming agf/VERSION."
    - "Item 2: repoint the two misdirected metadata links (docs/index.md:113, roles-and-gates.md:35) to the canonical slug."
    - "Item 3: in governance-loop.dot, change the learn node from the grey 'Reference/deferred content' treatment to the palette's 'Shared concept' category (style=filled, fillcolor=white, color=\"#0077BB\", fontcolor=\"#0077BB\"; drop dashed), and add a new sk_shared legend entry (see the plan's exact styling and label text). sk_ai's label stays as its original text."
    - "Item 4: rename Level 2 from 'Schema-defined' to 'Surface-defined' in .claude/rules/docs/implementation-honesty-rule.md to match the four already-aligned public surfaces."
    - "Item 5: extend palette.md's 'Light backgrounds' bullet per the plan's exact wording — do not touch the 'Semantic responsibility palette' prose or '### DOT-to-SVG usage' section, both owned by MR !511's already-merged changes."
    - "Item 6: add <figure>/<figcaption> markup to the 7 existing figure embeds in figures.md — do not touch !511's 5 newly appended gallery entries."
    - "Route the fact-checker consultation defined in the plan's 'Fact-checker consultation' section before requesting Gate 2."
  must_read:
    - docs/plans/v2.4.3-doc-quality-and-ia-backlog.md
    - 20260725T090712Z-governor-reviewer-qg61
  do_not_touch:
    - "agf/docs/site/reference/palette.md's 'Semantic responsibility palette' prose and '### DOT-to-SVG usage' section (MR !511's territory)"
    - "agf/docs/site/reference/figures.md's 5 newly appended gallery entries (MR !511's territory)"
    - "adoption-exit-flow.dot (explicitly excluded from item 3)"
    - "Issue #53 and the seq deprecation decision (explicitly out of scope)"
---

# Hand off v2.4.3 documentation-quality implementation

Governor approved Gate 1 for MR !483 at exact SHA `50f1bebb` (decision
record `qg61`) and authorized landing the plan MR and handing its frozen
scope to coder-tester. Plan status updated to `approved` and pushed;
`glab mr merge 483 --yes` issued (set to auto-merge once its CI pipeline
finishes — it was still running at handoff time). This routes the six
frozen items to implementation per the plan's own "Implementation" section.

Note MR !483's own branch required a merge of `main` to resolve 73 commits
of divergence before it became mergeable — no conflicts, but flagging in
case the resulting merge commit is unexpected in the history.
