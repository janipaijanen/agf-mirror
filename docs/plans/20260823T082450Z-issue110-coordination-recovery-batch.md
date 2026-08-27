---
id: issue110-coordination-recovery-batch
status: pending-gate1
topic: issue-110-coordination-recovery-batch
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Coder-tester (Codex, model gpt-5.4) performed the discovery and
      published the coordination-recovery proposal this plan implements
      (`20260822T202031Z-coder-tester-rbqm.md`, MR !1780), including the
      exact candidate set, fail-closed boundaries, and required
      recomputation-before-construction instruction. Requesting Codex's
      independent contribution-auditor review of this plan document next,
      per `6srq` — Codex authored the underlying discovery evidence but
      not this plan text, mirroring how `hr8k` reviewed
      `20260816T114725Z-open-mr-hygiene-disposition.md` (MR !1617) as
      contribution-auditor despite also having performed coder-tester
      review on the same document, in a different jurisdiction.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      This plan touches only MR state (close, with a standardized note)
      and coordination-record classification/recovery. It adds no code,
      no schema change, no credential, and no security-surface or
      network-authority change. No security-relevant design exists to
      consult on — same reasoning `20260816T114725Z-open-mr-hygiene-
      disposition.md` used for the same not-applicable disposition on
      the directly preceding issue #110 slice.
---

# Plan: issue #110 coordination-recovery and closure batch (round 2)

## Decision requested

Authorize one coordination-only recovery merge that copies 61 already-
written, already-validated coordination records — currently reachable
only from 51 open, stale merge request branches — onto `main` verbatim,
followed by closing those 51 source MRs with a standardized note
pointing at the recovery merge. No merge request's *content* is altered;
no coordination record's *text* is modified; no branch is deleted.

This is the second slice of issue #110. The first slice
(`20260816T114725Z-open-mr-hygiene-disposition.md`, MR !1617) is
Gate 1 approved and closed one MR (!921) as a deliberately narrow first
step. Coder-tester's handoff `rbqm` (MR !1780, referenced above)
proposes the next slice: rather than closing MRs one at a time, recover
every MR whose *entire* content is coordination-only and already fully
represented by files this exact batch will place on `main`, in a single
recovery train, then close all of them together.

## Why recovery, not silent closure

Every one of these 51 MRs represents real coordination work — reviews,
handoffs, decision-records, corrections — written and pushed but never
merged, mostly because the coordination-only fast path (`coordination-
publication-rule.md`) processes records individually and a large backlog
accumulated faster than it drained. Closing an MR without first landing
its content on `main` would silently delete that record from the
project's audit trail — the exact failure mode `coordination-publication-
rule.md`'s append-only discipline exists to prevent. This plan's
sequencing (recover first, verify on `main`, close only after) makes
closure a bookkeeping act on top of a preserved trail, not a substitute
for one.

## Exact candidate set (independently reverified 2026-08-23 against
## `origin/main` at `ef80edbde12d11c82057164ebbc8bd4f3f97bc8a`)

`rbqm` cited these predicates against an earlier `main` head
(`118981ed`). Per its own required recomputation-before-construction
instruction, I independently recomputed every predicate from scratch
against current `origin/main` before writing this plan — not by trusting
the cited numbers. Method: for each candidate MR, fetch its head, compute
the merge-base against current `origin/main`, diff merge-base...head,
and require every changed path to be an **addition** matching
`docs/coordination/*.md`; then confirm the resulting blob is **absent**
from current `main`; then check every collected file's `id:` frontmatter
field for uniqueness against every other collected file and against
every one of the 2,712 coordination records already on `main`.

Result: identical to `rbqm`'s claim.

- 51 candidate MRs, all state `opened`:
  `!1032, !1083, !1543, !1545, !1549, !1550, !1551, !1552, !1553, !1554,
  !1555, !1556, !1557, !1558, !1559, !1560, !1561, !1562, !1620, !1623,
  !1660, !1683, !1740, !1741, !1742, !1743, !1744, !1746, !1747, !1748,
  !1749, !1751, !1752, !1753, !1754, !1755, !1756, !1757, !1758, !1759,
  !1760, !1761, !1762, !1763, !1764, !1765, !1766, !1767, !1768, !1769,
  !1770`
- 61 distinct files, every one a pure addition under `docs/coordination/`
  matching `*.md`, absent from current `main`.
- Zero path collisions among the 61 files.
- Zero `id:` collisions among the 61 files.
- Zero `id:` collisions between the 61 files and any of the 2,712
  coordination records already present on `main`.
- All 61 files pass `agov coord validate` (schema-level check) as
  collected, individually.

No MR outside this exact set is in scope. No content-bearing or plan MR
is in scope. No branch outside these 51 source branches is touched.

## Procedure

### Step 1 — Construct the recovery branch (coder-tester)

1. Fetch `origin/main` fresh. Recompute every predicate in the prior
   section from scratch — MR state, diff shape, path/ID uniqueness,
   `agov coord validate` — against the freshly fetched head, not against
   this plan's cited numbers. Any MR that fails a predicate now (changed,
   closed, path/ID collision, validation failure) is **dropped from the
   batch**, not repaired or normalized. Record which MRs were dropped, if
   any, and why.
2. Create one new branch from current `origin/main`, named for this
   recovery batch.
3. Copy the confirmed-surviving files into `docs/coordination/` on that
   branch, byte-for-byte, using each source branch's exact blob — no
   reformatting, no frontmatter edits, no content changes of any kind.
4. Run `agov coord validate` on the aggregate. Commit as one pure-addition
   commit, coordination-only branch discipline
   (`feedback-coordination-record-own-branch` convention: no other file
   type in this branch, no merge commits).

### Step 2 — Publish and validate (coder-tester)

1. Push with `-o merge_request.create`, `coordination:` tip-commit title
   prefix so `agov coord publication-check` applies the coordination
   fast-path check to the full commit range, not just the tip diff.
2. Confirm the exact-head pipeline is green, including
   `agov:governance:validate`'s `coord validate` and
   `coord publication-check` steps.
3. Do not create a pipeline per source MR. MR closure itself needs no CI
   run — the recovery merge's own pipeline is the only required run for
   this batch.

### Step 3 — Independent review (contribution-auditor, then Gate 1/Gate 2)

1. Contribution-auditor independently reviews this plan document.
2. On CLEAR, request the binding Gate 1 decision from governor-reviewer
   for this exact plan head.
3. After Gate 1, coder-tester constructs and publishes the recovery
   branch per Steps 1–2, referencing the Gate 1 decision.
4. Request Gate 2 for the recovery branch's exact head plus the
   conditional close operation described in Step 4 — one Gate 2 decision
   covering both, per `rbqm`'s "minimal resource use" framing, since the
   close operation is mechanical and fully determined by the recovery
   merge's own verified content.

### Step 4 — Close the source MRs (coder-tester, after Gate 2 and after the recovery merge is verified on `main`)

For each of the 51 MRs whose file(s) are confirmed present, byte-for-byte,
on `main` after the recovery merge:

1. Post a standardized closing note on the MR, naming the recovery merge
   commit/MR and, for that MR's specific file(s), confirming their exact
   presence on `main` by path.
2. Close the MR via the GitLab API. Do not delete the source branch.
3. Do **not** represent MR closure as resolving the coordination message
   itself — a closed carrier MR is a bookkeeping act; whether the
   message's own obligation (handoff, review, decision) is resolved is a
   separate question answered by `agov coord pending` / `agov handoff
   next`, unaffected by this batch.

If, at Step 4 time, any of the 51 MRs no longer matches its Step-1-
verified state (source branch force-pushed, MR manually closed already,
etc.), skip that one MR, record why, and do not force the closure.

## What this plan does NOT authorize

- Editing, reformatting, or otherwise modifying the text of any of the 61
  recovered coordination records.
- Deleting any of the 51 source branches.
- Closing any MR outside the exact 51-MR candidate set, including any
  MR that becomes coordination-only-and-stale after this plan's Gate 1
  approval but was not part of the reverified set at construction time —
  that is a separate future slice, not an extension of this one.
- Any judgment about whether a message's underlying obligation is
  resolved — only that its carrier MR is closed with the record now
  durably on `main`.
- Touching the remaining ~46 open MRs that are not coordination-only
  (issue #110's own scan already found every open MR carries at least
  one content delta from `main` — see `20260816T114725Z-open-mr-hygiene-
  disposition.md`'s "Why this plan is deliberately narrow" section for
  that finding; this plan's 51-MR set is exactly the coordination-only
  subset, not the full backlog).

## Test / evidence strategy

- `agov coord validate` on the full 61-file aggregate before commit, and
  again as part of the exact-head CI pipeline after push (structural
  schema check).
- `agov coord publication-check` on the full commit range (append-only,
  coordination-only-range enforcement per `coordination-publication-
  rule.md`).
- Manual post-merge spot check: for a sample of the 51 source MRs,
  confirm the exact file path now resolves on `main` with identical
  content (`git show origin/main:<path>` byte-equal to the source
  branch's blob) before that MR's closing note is posted.

## Rollback

The recovery merge only adds files; if a defect is found after merge
(e.g., a byte-level corruption during copy), the fix is a new, separate
coordination-only commit correcting or superseding the affected record —
never an edit of the landed file (append-only discipline, same as
`immutable-tags-rule.md`'s sibling principle for tags). Source MRs are
not closed until their corresponding file is confirmed correct on `main`,
so an uncaught defect blocks only that one MR's closure, not the whole
batch.
