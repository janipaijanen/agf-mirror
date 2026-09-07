---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260907T045556Z-contribution-auditor-n7p1
timestamp: 2026-09-07T04:55:56Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260825T044831Z-planner-architect-4kja.md
topic: stale-worktree-branch-pruning-rule-gate1-contribution-audit
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 contribution audit — MR !2038 (stale worktree/branch pruning rule)

Fulfils the `4kja` handoff (addressed to contribution-auditor; a
contribution-auditor session satisfies it per the role-vs-session
addressing note, issue #121). Reviewed MR !2038 at branch head
`036236de40` against `origin/main` `f26adecd`.

Artifacts read: the Gate 1 plan
`docs/plans/20260825T044054Z-stale-worktree-branch-pruning-rule.md`, the
new rule `agf/rules/workflow/stale-worktree-and-branch-pruning-rule.md`
(and its `.claude/rules/` copy), the projection manifest
`docs/reviews/rule-projection/20260825T044054Z-stale-worktree-branch-pruning-rule.yaml`,
the `branch-and-worktree-rule.md` / `AGENTS.md` /
`agent-cold-start-habits.md` edits, and the `4kja` request itself.

## Verdict: CLEAR with findings

The rule's classification table faithfully reflects the hand-verified
method described in `4kja` and the plan, and is appropriately conservative
— it does **not** read as a blanket "always safe to auto-delete" claim.
F1 (plan evidence arithmetic) should be fixed before Gate 1 approval; F2
and F3 are advisory hardening the governor may fold in or defer given the
rule's Provisional status; F4 and F5 are rebase / drift notes.

## Handoff question 1 — does the table match what was actually done?

Yes, substantially. Every class is gated on a **live** check, not a cached
assumption:

- **Merged ancestor** → `git branch -d` (git verifies) — conservative.
- **Squash-merged** → `-D` only after the MR state is confirmed `merged`
  via API.
- **Formally closed** → `-D` when the MR state is `closed`.
- **No MR, stale by age** → `-D` only when *all* of: no MR ever, last
  commit older than the configured threshold, not pushed to remote, no
  active worktree — and the report must name that specific evidence.
- **Everything else** (open MR, active worktree, recent unpushed commits,
  any uncommitted content) → never auto-delete.

The guardrails section encodes this session's two near-misses directly:
never force past a denied destructive action, never delete the
checked-out branch or one backing an active worktree, local-only /
remote-recoverable scope, re-fetch and re-check trunk immediately before
each deletion batch, and "state the evidence, not just the verdict." The
new rule's `agf/` and `.claude/` copies are byte-identical at
introduction.

## Findings

### F1 — plan evidence sums to 894, not the stated 871 (should-fix before Gate 1)

The plan's "Why this is needed now (evidence)" section breaks the dead
branches down as *692 plain ancestors + 24 squash-merged + 136 formally
closed + 42 no-MR-stale* = **894**, but the plan headline, the rule's
evidence header comment, and `4kja` all state **871** dead of 900 (≈97%).
894/900 is ≈99%. The component figures and the total disagree by 23.
Per `verifiable-claims-rule`, reconcile the breakdown with the total (or
drop/hedge the breakdown) before the rule's evidence header ships. This is
an evidence-accuracy fix only; it does not affect the rule's design.

### F2 — "squash-merged" class ignores post-merge commits (advisory)

`Squash-merged → git branch -D` keys solely on MR state `merged`. A branch
can be a non-ancestor with a `merged` MR yet still carry commits pushed
*after* the merge (someone kept working on it). Recommend the class also
require the local branch tip to equal the MR's recorded source `sha`
(no commits after the merge) before `-D`, mirroring the "Everything else"
row's unpushed-recent-commits carve-out. Assumed intent: "squash-merged"
should mean "fully represented in trunk," not merely "an MR by this name
was merged once."

### F3 — "formally closed" guardrail is softer than the others (advisory)

`Formally closed → git branch -D` is qualified only by "check the MR's
closing note if in doubt." A `closed`-not-merged MR often means the work
was rejected; its branch may hold the only copy of commits someone meant
to revisit. The rule's "always recoverable from the hosting platform"
mitigation holds only while the remote branch ref still exists, and many
forges delete the source ref on close. Recommend: for `closed`-not-merged,
require either a named superseding MR or an explicit "abandoned by
decision" note before `-D`; otherwise fall through to "Everything else"
(report and leave). Assumed intent: don't let "an MR existed and is not
open" stand in for "this content is safe to lose."

### F4 — branch is 2 days stale and predates the !2463 merge (rebase check, non-blocking)

MR !2038's branch (last updated 2026-09-05) predates today's merge of
MR !2463 (universal polling), which modified three of the eight files
!2038 touches: `AGENTS.md`, `agf/rules/workflow/branch-and-worktree-rule.md`,
and `docs/reference/agent-cold-start-habits.md`. GitLab reports
`detailed_merge_status: mergeable` and the !2038 edits are
end-of-section / end-of-file appends (no textual conflict detected), but
the governor should confirm the merged result reads coherently — e.g.
`agent-cold-start-habits.md` would gain !2038's step 8 alongside !2463's
additions to the same file.

### F5 — pre-existing agf/ vs .claude/ drift on branch-and-worktree-rule.md (out of scope, note only)

The `agf/rules/` and `.claude/rules/` copies of `branch-and-worktree-rule.md`
already diverge on this branch (the `.claude/` copy lags: no frontmatter,
missing the multi-agent branch-watch subsection), and !2463 widened that
gap. The plan explicitly scopes this out ("ongoing sync is an existing
known gap tracked separately, not solved by this plan"), which is a
reasonable call — flagged only for visibility, not as a !2038 defect.

## Handoff question 2 — are the guardrails sufficient for future sessions?

Yes, with one suggested addition. The four guardrails plus "re-fetch trunk
before each batch" and "state the evidence" cover the two near-misses
`4kja` cites (trunk advancing mid-sweep twice; a compound `rm -f`
correctly denied by the human's permission gate). Consider adding an
explicit **bulk-deletion confirmation threshold**: when a single sweep
would delete more than N branches (the motivating sweep removed 871 at
once), require a human confirmation or a dry-run summary first, so a
future agent misclassifying an entire class cannot do bulk damage before a
human notices. Advisory, not blocking.

## Handoff question 3 — is the projection evidence sound?

Yes. The `not_applicable` calls for `adopter_facing_explanation` and
`runbook_situation_map` are correct: the change touches no `agf/docs/site/**`
Pages content, and `docs/reference/agent-cold-start-habits.md` is an
explicitly foundry-internal habits page ("do not copy this foundry
checklist downstream"). `distribution: confirmed` via the existing
`agf/rules/**` release-manifest glob is right. `validation_drift_evidence:
advisory_only` is correct — this is advisory workflow guidance, not a
mechanically enforced gate, so `always/gate-self-test-rule.md`'s
negative-test requirement does **not** fire (verified: the MR adds no CI
check, no validator, and no `agov` subcommand). `agov plan validate` and
strict `agov plan projection-check --base-ref origin/main` both pass at the
branch head.

## advisory_consultation dispositions

- `infosec-sentinel: not-applicable` — agree. Local git state only; no
  credential, auth, or cross-record leak surface; the rule's own scope note
  is explicit that it never mutates remote/hosted state.
- `contribution-auditor: waived` at plan-filing → **now satisfied** by this
  review. The plan's `advisory_consultation` block should be updated to
  `status: consulted` citing this record (`n7p1`) in the same review cycle
  as Gate 1 approval, so the correction does not have to be made after the
  fact.

## downstream-impact classification

ADDITIVE — a new advisory rule plus loader pointers; no existing contract
removed, no schema/manifest/CI change. The plan's "Migration / schema
impact: None" is correct. No migration note required.
