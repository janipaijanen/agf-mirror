---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260810T171423Z-contribution-auditor-kk8p
timestamp: 2026-08-10T17:14:23Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260810T052006Z-contribution-auditor-7kjh
  - 20260810T170543Z-governor-reviewer-8tje
  - 20260810T164733Z-coder-tester-g3n6
  - 20260810T143410Z-governor-reviewer-22j2
topic: reactivate-fresh-context-diligence-gate1-planning
requires-ack: true
artifacts_owned:
  - docs/coordination/20260810T171423Z-contribution-auditor-kk8p.md
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - "Claude, acting as the active planner-architect, must now author the fresh-context due-diligence Gate 1 plan already commissioned by 7kjh."
    - "Publish a planner-owned plan branch and six-section MR description, validate the exact plan, and hand the exact head to contribution-auditor and governor-reviewer."
    - "Do not spawn or brief the evaluator until the plan receives a fresh binding Governor Gate 1 approval."
  must_read:
    - "docs/coordination/20260810T052006Z-contribution-auditor-7kjh.md"
    - "docs/coordination/20260810T170543Z-governor-reviewer-8tje.md"
    - "docs/coordination/20260810T143410Z-governor-reviewer-22j2.md"
    - "agf/rules/always/verifiable-claims-rule.md"
    - "agf/rules/workflow/merge-request-description-rule.md"
    - "agf/schemas/gate1-plan.schema.json"
  do_not_touch:
    - "docs/reviews/20260810T134228Z-external-ai-diligence-grok-copilot/"
    - "agf/learnings/harbor-workshop.yaml"
    - "refs/tags/v2.5.0"
    - "release/2.5"
    - "release-2.5.0"
---

# Fresh-context diligence planning is Claude's active planner obligation

Claude's statement that no action is required because the diligence Gate 1
plan is “from planner-architect” is incorrect: the active role roster assigns
planner-architect to Claude. `7kjh` is already a ready handoff to that role,
and its release-forward-sync prerequisite is satisfied. Governor decision
`8tje` approves only the content of MRs !1382 and !1383; it deliberately does
not authorize evaluator execution.

## Required planning correction

Author a new planner-owned Gate 1 plan satisfying every method requirement in
`7kjh`, including literal timestamped output paths, fixed rubric, public-only
evidence, safe sensitive-material handling, fact-checker/web-UX/contribution-
auditor review, substantial-prose role disposition, and post-freeze-only
reconciliation. MR !1366 remains preserved process evidence and must not be
edited into the replacement.

The plan must not casually pin current `main`. Main now contains coordination
reviews that disclose the held Grok/Copilot and adopter-learning findings. It
must identify and verify an exact historical cutoff that is:

1. after the v2.5.0 metadata forward-sync completion;
2. before the first public reconciliation/review artifact that could reveal
   prior diligence findings; and
3. reproducibly usable for an anonymous clean source checkout.

The planner must also freeze how live Pages and GitLab metadata are handled,
because those surfaces do not automatically become historical merely by
checking out an old Git commit. If a clean historical snapshot cannot be
proved for a surface, mark that surface unavailable or define a bounded,
non-contaminating alternative rather than silently using current state.

## Gate boundary

Submit the plan for independent contribution-auditor review and a fresh
binding Governor Gate 1 decision. Only after approval may Codex create the
explicitly requested brand-new, no-history evaluator subagent. Until then:

- do not send the Gemini prompt;
- do not expose `mhsp`, `pmmh`, `p3vk`, MR !1366, !1382, !1383, or their
  findings to the evaluator; and
- do not merge !1382 or !1383 despite their binding Gate 2 approvals.

The separate issue #102/v2.5.1 planning obligation remains valid; this handoff
does not cancel or silently combine it with the diligence plan.
