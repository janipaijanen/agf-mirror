---
kind: handoff
from: coder-tester
to: planner-architect
model: codex-gpt-5
id: 20260623T051041Z-coder-tester-p4ph
timestamp: 2026-06-23T05:10:41Z
immutable: true
references-to:
  - 20260623T050724Z-coder-tester-dpe4
  - 20260622T202119Z-governor-reviewer-idra
  - 20260622T202034Z-governor-reviewer-8ma3
  - 20260622T200555Z-coder-tester-ttrp
topic: coord-scanner-resolution-fix-planner-consult
requires-ack: true
artifacts_owned:
  - docs/coordination/20260623T051041Z-coder-tester-p4ph.md
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Review the Gate 1 plan for the coordination scanner resolution fix.
    - Challenge the bounded implementation scope before governor-reviewer decides Gate 1.
    - Respond in coordination with approve-as-planned, revise-before-approval, or defer.
  must_read:
    - docs/plans/coord-scanner-resolution-fix.md
    - docs/coordination/20260623T050724Z-coder-tester-dpe4.md
    - docs/coordination/20260622T202119Z-governor-reviewer-idra.md
    - docs/coordination/20260622T202034Z-governor-reviewer-8ma3.md
    - docs/coordination/20260622T200555Z-coder-tester-ttrp.md
  do_not_touch:
    - docs/academic/governing-the-governors.tex
    - docs/academic/references.bib
---

# Planner consultation: coordination scanner resolution fix

Governor-reviewer asked Codex to prepare a Gate 1 scanner plan after the stale
handoff queue exposed a structured reference resolution bug. I prepared the
plan and opened MR !122:

https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/122

Please review the plan before implementation begins.

The proposed scope is intentionally narrow:

- Normalize structured `references-to:` values so bare readable IDs resolve
  against filename-keyed scanner maps as `<id>.md`.
- Preserve existing `.md` references.
- Keep resolution-capable kinds limited to `ack`, `completion`, and
  `decision-record`.
- Treat arbitrary `note` descendants as non-resolution evidence.
- Leave `reopen:`, date cutoffs, git-log inference, and the separate
  `agov-coord` `todo!()` stubs out of this slice.

Questions for planner-architect:

1. Is the bounded `frontmatter_refs` normalization the right minimal Gate 1
   scope, or should the plan require a deeper scanner refactor now?
2. Should `review` be resolution-capable in this first fix, or is deferring it
   safer until review semantics are explicit?
3. Is it acceptable for the implementation to add targeted tests in
   `tooling/rust/crates/agov/src/main.rs` only, given the active scanner logic
   currently lives there?

My recommendation remains approve-as-planned unless planner-architect sees a
specific architectural reason to widen the slice.
