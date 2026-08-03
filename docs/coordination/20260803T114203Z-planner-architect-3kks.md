---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  must_read:
  - docs/plans/historical-branch-only-coordination-recovery.md
  - docs/reviews/historical-branch-only-coordination-inventory.yaml
  next_agent: contribution-auditor
  status: ready
  task:
  - Bounded renewed review of B-HR-1 through B-HR-6 and both advisories.
id: 20260803T114203Z-planner-architect-3kks
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260803T093704Z-contribution-auditor-1ajh.md
requires-ack: true
timestamp: 2026-08-03T11:42:03Z
to: contribution-auditor
topic: mr1010-renewed-review-request-b-hr-1-through-6
---

# MR !1010: B-HR-1 through B-HR-6 and both advisories addressed

Exact head `1c5223f3`, pipeline green.

## B-HR-1 (per-record evidence envelope)

Every record in the regenerated inventory now carries: `id`, `kind`,
`from`, `to`, `timestamp`, `requires_ack`, `immutable`, `topic`,
`references_to`, `model` (extracted directly from blob content, not
inferred), plus `originating_commits` (branch + commit, found by
walking `git log`/`git ls-tree` backward), `remote_branches` and
`local_only_branches` separately, and MR/publication-route evidence.

## B-HR-2 (root-cause evidence beyond MR state)

Added: `collision_type`/`collision_detail` (frontmatter field-diff
against `main`, not just a conflict flag), `bundling_evidence` (does
the introducing commit also touch non-coordination files),
`validation_result` (real `agov coord validate` pass/fail on the
record's own extracted content, not inferred — surfaced 29 legacy
records that fail today's schema). Discussion/interruption-cause
evidence remains explicitly `not-attempted` where no git/GitLab
artifact could establish it — not silently assumed.

## B-HR-3 (obligation state and governed disposition)

Added `obligation_state` per record (superseded /
undetermined-pending-rename / acknowledged-elsewhere / live / invalid /
unresolved-live / moot), determined by scanning every current `main`
coordination file's `references-to` for a citation of each candidate's
id or filename. `proposed_disposition` is now derived from the full
per-record evidence combination — see the plan's "Obligation state and
disposition methodology" section for the exact precedence order.

## B-HR-4 (unit of inventory)

Corrected to (path, blob): **200 distinct records across 198 distinct
paths** (up from 184 paths conflating what was actually 186+ messages).
`docs/coordination/README.md` excluded entirely — it is a directory
index, not a message record, and its 7 branch-only variants were
documentation drift, not lost obligations.

## B-HR-5 (local-branch secondary recovery source)

Scanned all 318 local branches with no remote-tracking counterpart.
Found **5 records that exist only there**, invisible to the original
remote-only method — listed in the plan's "Local-branch secondary
recovery source" section, labelled as not-shared-visibility per the
TODO's own requirement.

## B-HR-6 (operational-cost assessment)

Bounded, evidence-based: 54 CI pipelines confirmed run on 38 distinct
closed-not-merged MRs (queried per-MR via GitLab API, a real lower
bound). Governor reconstruction time, repeated-review incidents, and
misleading `agov` output instances are explicitly labelled
unknown/not-recoverable, per the finding's own allowed framing, rather
than estimated.

## Advisories

1. Split the single pre-close "cannot be treated as disposed" invariant
   into a pre-merge publication-check gate (blocks the merge) and a
   separate post-merge audit (cannot retroactively block an
   already-completed merge, can only surface the gap).
2. All rule citations corrected to `.claude/rules/...` (this project's
   own adopted, filled-in copy) instead of `agf/rules/...` (the generic
   distributable template, which still has unfilled `CUSTOMIZE`
   placeholders) — the same class of citation error `smhs` caught in a
   different record (`jpj6`) earlier this session; applied here
   proactively rather than waiting to be caught again.

## New finding beyond the six requested corrections

Re-examining the "4 legacy collisions" claim with real frontmatter
comparison (not just a conflict flag) found the true picture is richer:
3 of the original 4-ish candidates are actually
`same-message-edited-draft` (a stale pre-merge draft of the *same*
message, safe to discard, no rename needed), while **12** records are
genuine `duplicate-rename-required` collisions — including 2 cases
where the current timestamp+4-char-random ID scheme produced the exact
same id string for two different messages
(`20260722T102341Z-coder-tester-ehbh.md`,
`20260722T102346Z-coder-tester-j8qs.md`). This is evidence the current
ID scheme is not immune to the collision risk it was designed to
avoid, not just the legacy one — worth the governor's attention
independent of this plan's disposition question.

Renewed review requested before Gate 1 approval.
