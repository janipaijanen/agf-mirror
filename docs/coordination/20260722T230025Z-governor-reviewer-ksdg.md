---
kind: decision-record
from: governor-reviewer
to: all
id: 20260722T230025Z-governor-reviewer-ksdg
timestamp: 2026-07-22T23:00:25Z
model: human-jani
immutable: true
references-to:
  - 20260722T223540Z-contribution-auditor-r1gq.md
  - 20260722T223540Z-governor-reviewer-mdnp.md
topic: mr417-gate2-approved-with-postrelease-accessibility-followup
requires-ack: false
artifacts_owned: []
---

# Decision: MR !417 Gate 2 approved; residual accessibility work moves post-release

The governor approves MR !417 at exact SHA
`ca1ebcbd8b3f3fc3212dc6100738fd2040f66b83` for Gate 2.

The replacement handoff `20260722T225500Z-coder-tester-pn84` on MR !427,
independent poll review, and exact-SHA pipeline `2698581448` establish that all
three findings in `r1gq` are corrected:

1. actor type and evidence durability are no longer conflated;
2. the five incorrect responsibility mappings use the neutral/shared
   treatment; and
3. current `main` and the academic-TeX scope decision are integrated.

Pipeline `2698581448` succeeded with four of four jobs, no retry, and no
duplicate push pipeline. The committed SVGs regenerate exactly from their DOT
sources; PDF extracted content matches; contrast and negative self-tests pass;
and the figures are legible at full resolution.

## Explicitly accepted limitations and post-release obligations

The following findings remain real. The governor accepts them for this release
and directs that they be fixed later rather than holding v2.4.0:

1. **Non-colour role-category cues.** `agent-participation-by-phase` and
   `role-communication-topology` still distinguish human-primary, AI-primary,
   mixed advisory, and tooling/CI roles primarily by colour. Add a direct
   label, pattern, shape, border, or other non-colour cue. Review the reused
   public role-axis precedent at the same time so Class 1 and Class 3 do not
   retain inconsistent accessibility semantics.
2. **Meaningful screen-reader alternatives.** The standalone academic SVGs
   currently contain Graphviz identifier-like `<title>` values rather than
   descriptive alternatives, and the PDFs are not tagged. Add a maintained,
   regeneration-safe descriptive text path for each academic figure. Do not
   hand-edit generated files or reopen the low-priority TeX pagination work.
3. **Topology layout.** Compact the tall, low-density legend column in
   `role-communication-topology` when the accessibility follow-up is planned.
4. **Narrow-view claim.** Future evidence should say that the standalone
   PDF/SVG is reader-zoomable and inspected at full resolution, not that a
   shrink-to-fit presentation defect is impossible.

These are accepted limitations, not completed requirements and not evidence
that the current artifacts are fully accessible. Planner-architect should
place them in governed post-release planning; coder-tester may implement them
only after the appropriate Gate 1 approval.

## Release direction

MR !417 may merge at the approved exact SHA. Continue Workstream D Classes 4
and 5 as release blockers. Do not spend v2.4.0 release time on LaTeX tooling,
paper pagination, or the deferred post-release accessibility improvements
above.
