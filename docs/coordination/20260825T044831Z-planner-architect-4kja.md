---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260825T044831Z-planner-architect-4kja
timestamp: 2026-08-25T04:48:31Z
model: claude-sonnet-5
immutable: true
references-to: []
topic: stale-worktree-branch-pruning-rule
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Review MR !2038 (branch `workflow/stale-worktree-branch-cleanup-rule`,
      Gate 1 plan `docs/plans/20260825T044054Z-stale-worktree-branch-pruning-rule.md`)
    - Verify the new rule's classification table matches what was actually
      verified by hand this session, not a broader "always safe to
      auto-delete" claim
    - File a coordination `review` record with the result (CLEAR or
      findings) so governor-reviewer Gate 1 approval has a contribution-
      auditor consultation to point to
  must_read:
    - docs/plans/20260825T044054Z-stale-worktree-branch-pruning-rule.md
    - agf/rules/workflow/stale-worktree-and-branch-pruning-rule.md
    - docs/reviews/rule-projection/20260825T044054Z-stale-worktree-branch-pruning-rule.yaml
  do_not_touch:
    - .claude/settings.local.json
---

# Review request: stale worktree and branch pruning rule (MR !2038)

Earlier this session I manually swept this repo's local git state on the
human's request: 9 registered `git worktree` entries and 900 local
branches, of which 871 (97%) turned out to be dead — merged into
`origin/main`, landed via squash-merge, formally closed without merging,
or never had an MR opened and hadn't been touched in 6+ weeks. Nothing in
the existing `branch-and-worktree-rule.md` assigns anyone the job of
sweeping what other sessions/agents leave behind — it only documents
cleanup for the branch *you* just merged. The human then asked for a
standing rule so this becomes routine instead of a one-off manual audit,
explicitly scoped to **every AI assistant** working in this repo, not just
Claude.

MR !2038 adds `agf/rules/workflow/stale-worktree-and-branch-pruning-rule.md`
(mirrored to `.claude/rules/workflow/`), wires pointers into `AGENTS.md`
and `docs/reference/agent-cold-start-habits.md`, links it from
`branch-and-worktree-rule.md`'s existing per-branch cleanup section, and
includes the plan file and six-surface projection evidence.

**What I'd like you to check specifically:**

1. Does the classification table (merged-ancestor / squash-merged /
   formally-closed / no-MR-stale-by-age / everything-else) match what this
   session actually verified before deleting each batch, or does the rule
   text read more permissively than what was actually done?
2. Are the safety guardrails (never force past a denied destructive
   action, never delete the checked-out branch or one backing an active
   worktree, local-only scope, re-verify trunk immediately before each
   batch) sufficient, given this rule will be read and acted on by future
   sessions that did not live through this session's near-misses (trunk
   advancing mid-sweep twice; a compound `rm -f` correctly denied by the
   human's permission gate)?
3. Is the six-surface projection evidence file
   (`docs/reviews/rule-projection/20260825T044054Z-stale-worktree-branch-pruning-rule.yaml`)
   reasoning sound, particularly the `not_applicable` calls for
   `adopter_facing_explanation` and `runbook_situation_map` (this change
   touches no `agf/docs/site/**` Pages content)?

`agov plan validate` and `agov plan projection-check` both pass locally.
Plan status is `pending-gate1`; governor-reviewer Gate 1 approval is still
outstanding, in parallel with this review request.
