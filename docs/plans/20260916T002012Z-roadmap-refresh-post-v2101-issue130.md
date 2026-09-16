---
id: 20260916T002012Z-roadmap-refresh-post-v2101-issue130
status: approved
gate1_approved_by: governor-reviewer
gate1_approved_at: 2026-09-16T04:23:34Z
topic: roadmap-refresh-post-v2101-issue130
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Status-only documentation update to already-published roadmap
      state; no credential, secret, network, or new write authority.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Self-performed consultation published as
      20260916T002157Z-contribution-auditor-jhg1.md, before Gate 1
      request, per this project's established practice. Found and
      fixed two real gaps in the same draft (a false claim of overlap
      between unrelated workstreams; a miscategorized non-issue in the
      staleness count) -- see that record for detail.
affected_paths:
  - docs/roadmap.md
  - agf/docs/site/roadmap.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Roadmap refresh: post-v2.10.1, post-issue-#130

## Provenance

`docs/roadmap.md`'s own "Update discipline" section requires every
change to name its authorizing decision, refresh the observation
timestamp/baseline, and update the public projection in the same
batch, following the normal Gate 1/Gate 2 workflow -- "no special
roadmap fast path exists." This plan is that workflow.

The roadmap's current observation baseline is `main` at `90bb57a2`,
timestamped 2026-09-12T18:50:00Z. A survey this session (2026-09-16)
found it stale on at least six points, enumerated below with the
authorizing decision/completion record for each.

## Problem: five stale statements (plus one non-issue, addressed for completeness)

1. **v2.10.1 status.** The roadmap's `Status` field and "Released
   baseline" section both say v2.10.1 is "the pending reviewed
   release." It is not pending -- `glab release list` confirms
   `v2.10.1` published (tag pipeline, GitLab Release), corroborated by
   `20260912T210138Z-planner-architect-asdh` (merged-and-tagged),
   `20260912T215750Z-coder-tester-4jss` (public artifact verification),
   and `20260912T220356Z-planner-architect-grqd` (independently
   verified). `CHANGELOG.md` already carries the `## [2.10.1] -
   2026-09-12` entry.
2. **Row 6 S4 (`agov plan collision-check` blocking promotion) is
   missing entirely**, and the roadmap's Issue #123 row still says
   "Needs own Gate 1 plan. Add collision-detection tooling only after
   the current manual collision pass" -- exactly the tooling Row 6 S3
   (advisory) and Row 6 S4 (blocking) already shipped and merged
   (`20260915T050307Z-governor-reviewer-g7j1`, Gate 2 approval). Only
   #123's third original bullet -- a post-implementation
   `affected_paths`-vs-actual-diff drift check -- remains out of scope,
   explicitly deferred by the approved Row 6 S4 plan, not silently
   dropped.
3. **Row 5 S2 (self-announcing supersession) is missing entirely.** It
   shipped and merged (`20260914T115129Z-governor-reviewer-4bm1`, Gate
   2 approval) and is the direct answer to Issue #122 ("supersession is
   convention-only... does not announce itself"), which the roadmap
   still lists under "Issues #121, #122, MR !2027 | Needs own Gate 1
   workstream" as if unstarted.
4. **`qbka` plan-lifecycle work (Slice A + Slice B) is missing
   entirely.** Both merged (`20260913T160721Z-governor-reviewer-5jjg`
   for Slice A; Slice B cleared after an F1 evidence correction, see
   `41d27775`/`c246a322` in `git log`), yet the roadmap's row still says
   "`qbka` plan lifecycle | Needs its own Gate 1 plan | A separate
   planner owns the schema, validator, transition semantics..." as
   though no such plan exists.
5. **CPB2 (publication-hold urgency) and Row 5 S3 (priority ordering)
   are missing entirely** from the "Active post-v2.10 sequence" table,
   though both merged and closed
   (`20260913T085749Z-coder-tester-b1nb`; `20260913T121333Z-planner-architect-gkqh`).
6. **Issue #130 is absent from the table -- correctly, not a staleness
   defect.** It did not exist at the roadmap's baseline; it was filed,
   Gate 1 approved (`20260915T173154Z-governor-reviewer-nnpb`),
   independently audited across four rounds, implemented, Gate 2
   approved (`20260915T191933Z-governor-reviewer-gsq7`), merged, and
   the GitLab issue closed -- entirely within the gap since the
   roadmap's baseline. The table tracks open/active work; #130 was
   never open *and* stale simultaneously, so no row was ever missing.
   Listed here only so the survey record is complete, not because it
   requires a correction.

## Design

Update `docs/roadmap.md` and its public projection
`agf/docs/site/roadmap.md` together, in the same commit, per the
roadmap's own update discipline:

1. **`Status` field and "Released baseline" section:** replace "v2.10.1
   is the pending reviewed release" with "v2.10.1 is the latest
   published stable release (tag pipeline, GitLab Release, and
   anonymous artifact verification complete)"; v2.10.0 is superseded as
   latest, not removed from history.
2. **Observation baseline:** refresh to `main` at the exact SHA
   resolved at Gate 2 time for this plan's own merge, timestamped at
   that merge.
3. **"Active post-v2.10 sequence" table:**
   - Add explicit `done` rows for CPB2 and Row 5 S3, citing their
     completion records. Neither currently has any row -- correcting
     an earlier draft's mistaken claim that they were "implicitly
     covered" by the unrelated "Queue convergence" workstream; they are
     distinct, separately-planned workstreams (`git log --grep="cpb2\|
     row5-s3"` confirms no overlap) and were simply never added.
   - Issue #123 row: change disposition from "Needs own Gate 1 plan" to
     "Plan-vs-plan collision detection shipped and blocking (Row 6 S3 +
     S4, `g7j1`); the post-implementation `affected_paths`-vs-actual-diff
     drift check (the plan's original third bullet) remains open and
     unclaimed."
   - Issues #121, #122 row: split into two. #122: "Resolved by Row 5 S2
     self-announcing supersession (`4bm1`)." #121: "Row 5 S1 shipped
     session-level addressing (`for_session`, merged
     `20260911T035333Z-planner-architect-nh1j`); unresolved whether this
     fully satisfies #121's scope or leaves the execution-lane/claim
     work its own comments describe (`hqps`, 2026-09-05) as a separate
     slice -- flagged for verification, not asserted closed here.
     Closing #121 on GitLab is out of this plan's scope; it requires
     that verification first."
   - `qbka` plan lifecycle row: change to "done" citing Slice A + Slice
     B completion, removing the "needs its own Gate 1 plan" framing.
   - Issue #130: no row needed. The "Active post-v2.10 sequence" table
     tracks open/active work, not a changelog of everything ever
     closed, and #130 is fully closed (GitLab issue closed, no open
     MRs remain). Its absence from the table is therefore correct, not
     a gap -- unlike the six items above, which were absent from the
     table *while still being live, undiscoverable commitments*.
4. **Issue #131/#132 rows:** left as-is unless this plan's own survey
   finds new evidence (none found this session beyond what the roadmap
   already states); their disposition genuinely has not changed since
   baseline.
5. Sync `agf/docs/site/roadmap.md`'s `Status`/`Observed
   at`/`Observation baseline` fields and "Current released baseline"
   section to match, per the roadmap's own rule that the public
   projection updates in the same batch.

## Non-goals

- No new roadmap commitment, priority decision, or scope change --
  this plan only corrects the record to match already-governed,
  already-evidenced completions and closures.
- No disposition of Issue #121 beyond flagging it for separate
  verification -- closing it (if warranted) is a separate, smaller
  follow-up once that verification happens.
- No re-litigation of the advisory intake bound (five/three open-MR
  ceiling) or the collision/ownership boundaries sections -- unaffected
  by this refresh.

## Test strategy

This is a documentation-accuracy change with no schema, validator, or
gate-mechanics surface -- `gate-self-test-rule.md` does not apply
(no gate introduced or modified). Verification is evidence-citation
accuracy, not automated test coverage:

1. Every changed table row/field cites an exact coordination-record id
   or `git log` commit hash as its authorizing evidence, per the
   roadmap's own update-discipline rule 1.
2. `agov coord validate` on any newly-referenced coordination file
   (none created by this plan; all citations are to existing records).
3. Manual diff review at Gate 2 confirms `docs/roadmap.md` and
   `agf/docs/site/roadmap.md` were updated in the same commit and say
   the same thing about shared fields (Status, Observed at, Observation
   baseline), per marketing-tone-rule's message-integrity principle.

## Rule projection

Not applicable. This plan changes only observational/status prose in
two governed-roadmap documents; it introduces, changes, or removes no
MUST/SHOULD/blocking-type normative guidance, so
`documentation-rule-projection-rule.md`'s six-row screen is not
triggered. (Contrast with issue #130's plan, which changed a
schema's actual blocking behavior and required the full screen.)

## Rollback

Trivial -- revert the commit. No schema, code, or gate state changes;
reverting restores the prior (stale) roadmap text with no migration or
data implications.

## Decision requested at Gate 1

Approve the six corrections above (v2.10.1 status; Row 6 S3+S4/#123;
Row 5 S2/#122; `qbka` Slice A+B; CPB2/Row 5 S3 visibility; #130
disposition), the #121 flag-for-verification-not-closure disposition,
and the "not applicable" rule-projection disposition.
