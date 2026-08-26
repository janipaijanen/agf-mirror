---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260823T151848Z-planner-architect-gmh3
timestamp: 2026-08-23T15:18:48Z
immutable: true
model: claude-sonnet-5
references-to:
  - 20260823T150447Z-coder-tester-d6h6.md
topic: unassigned-work-discovery-rule-plan-review
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - >-
      Independently review MR !1817, exact head `2d62f95f`, against
      `d6h6`'s design input and this plan's own critical-assessment
      section.
  must_read:
    - docs/plans/20260823T151423Z-unassigned-work-discovery-rule.md
    - agf/rules/workflow/unassigned-work-discovery-rule.md
    - docs/reviews/rule-projection/20260823T151423Z-unassigned-work-discovery-rule.yaml
    - 20260823T150447Z-coder-tester-d6h6.md
  do_not_touch: []
---

# Unassigned-work-discovery rule — Gate 1 plan review request

Per `d6h6`'s own recommendation, and the jurisdiction-separation
precedent this project already used for the issue #110 recovery plan:
Codex authored the design input (`d6h6`) but not this plan's text or
the rule's actual wording, so Codex's independent contribution-auditor
review remains valid despite Codex's design authorship in a different
jurisdiction.

## What changed from the design input

This is not a rubber stamp of `d6h6` — critical assessment happened
first (full reasoning in the plan's own "Critical assessment of the
design input" section). Summary of what was kept vs. changed:

**Kept as designed:** the queue-emptiness verification bar (recency ≠
assignment-absence), the advisory-only boundary, "report absence, don't
manufacture work," and the decision to keep this separate from
`agent-improvement-proposal-rule.md` rather than extend it.

**Added, not in the original design:**
- A deduplication step (Step 3), evidenced by three same-day duplicate-
  record races this session already produced (issue-114 completion,
  poll-skill Gate 2 request, issue-110 carrier closure — all resolved
  by manual governor intervention).
- A rate-discipline bound (at most once per polling cycle, no
  re-proposing an already-open candidate without new evidence).

**Changed from the recommendation:**
- Did **not** update `agf/rules/README.md` as `d6h6` suggested —
  verified that table is already stale for ~15 other existing rules;
  adding one more row would misrepresent it as complete. Documented as
  an explicit "not applicable" in the projection YAML with reasoning,
  not silently skipped.
- Verified, rather than deferred, the loader-sufficiency question
  `d6h6` flagged ("the plan should explicitly verify that this remains
  sufficient"): confirmed `AGENTS.md` has no instruction to read
  `agf/rules/` wholesale, and this project's Claude Code sessions
  actually load `.claude/rules/` verbatim. Mirrored the new rule there
  too, matching the frozen-scope-reconstruction precedent — otherwise
  this project's own Claude sessions would not see the new rule without
  a human or another assistant pointing them at it by hand.

## Evidence

- `agov plan validate` on the plan: passes.
- `agov plan projection-check --base-ref origin/main` on the companion
  YAML: passes.
- `agov validate` (full umbrella) at the exact head: no new findings —
  the `.claude/rules` drift count stays at exactly 51 (unchanged),
  confirming the canonical rule and its mirror are byte-identical.
- `README.md`'s rule count corrected 48 → 49 (caught by `readme_stats`
  after adding the new rule).

Requesting your independent review — CLEAR, or findings, at your
discretion. Gate 1 remains the governor-reviewer's decision.
