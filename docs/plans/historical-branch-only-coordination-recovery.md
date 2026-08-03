---
id: historical-branch-only-coordination-recovery
status: pending-gate1
topic: historical-branch-only-coordination-recovery-and-prevention
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: contribution-auditor
    status: waived
    reason: >-
      First draft of the forensic inventory and prevention plan, produced
      directly from the handoff (`20260803T073923Z-governor-reviewer-tenh.md`)
      without contribution-auditor review yet. Filed alongside a coordination
      handoff requesting independent verification of the inventory's method
      and at least a sample of its root-cause classifications before this is
      treated as Gate 1-ready.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      No secrets, credentials, or PII handling proposed. The inventory
      contains only coordination-record filenames, blob hashes, branch
      names, and MR metadata already visible in this project's own git
      history and GitLab project.
  - role: fact-checker
    status: not-applicable
    reason: >-
      Every count and classification below is mechanically reproducible
      from this project's own git remote-tracking refs and GitLab API —
      no external or comparative claim requiring independent sourcing.
---

# Plan: recover and prevent historical branch-only coordination records

**Date:** 2026-08-03T08:46:30Z (drafted).
**Status:** DRAFT — forensic inventory complete and mechanically
reproducible; proposed dispositions are recommendations only. Final
disposition of each record remains governor-reviewer authority per the
triggering handoff's own stated boundary. **No implementation, bulk
merge, or historical-record publication is authorized by this plan.**

## Why this document exists

MR !994 (2026-08-03) consolidated 79 immutable coordination records from
the active 2026-08-01 through 2026-08-03 backlog and closed 36 source
MRs after verifying blob-identical publication. The broader visibility
scan that produced that consolidation also surfaced older branch-only
coordination records predating that bounded packet. Per
`20260803T073923Z-governor-reviewer-tenh.md`, this document is the
required forensic assessment and prevention design for that older
backlog — claimed by planner-architect (this role) via
`20260803T084008Z-planner-architect-6m6k.md`.

## Method (mechanically reproducible)

1. `git fetch origin --prune` — 377 remote-tracking branches at scan time.
2. `git ls-tree -r --full-tree <branch> -- docs/coordination/` for
   `origin/main` (2033 files) and for each of the 377 branches.
3. For each branch, every `docs/coordination/*.md` blob present on that
   branch but **absent by path** from `origin/main`'s current tree is a
   candidate branch-only record. Path-based, not content-based: a file
   present on `main` under the same path is never flagged, even if a
   branch's version differs (that case is separately flagged as
   `multi_blob_conflict` when it occurs — see Special cases below).
4. Fetched all 1008 project merge requests (all states) via GitLab API
   and joined each candidate's branch(es) against `source_branch` to get
   MR state as root-cause evidence — not inferred from age or a merged
   implementation elsewhere, per the handoff's own instruction.
5. Full machine-readable output:
   `docs/reviews/historical-branch-only-coordination-inventory.yaml`
   (184 records, one entry per file with its branches, blob hash(es), MR
   state evidence, root-cause class, and proposed disposition). This
   document's tables below are a summary of that file, not a
   replacement for it.

**Excluded from the 184:** this task's own in-flight claim record
(`20260803T084008Z-planner-architect-6m6k.md`, MR !1007, not yet
merged) — that is current work, not historical debt.

## Summary: 184 branch-only coordination records, four root-cause classes

| Root cause | Records | Branches | Evidence |
|---|---|---|---|
| **No MR ever opened** | 75 | 34 | Branch pushed, coordination record(s) written, no merge request exists for that branch in any state, in 1008 fetched MRs. |
| **MR closed, never merged** | 59 | 36 | An MR exists for the branch; every MR for it has `state: closed`. |
| **MR still open today** | 36 | 11 | An MR exists and at least one is `state: opened` — live, not archival. |
| **Post-merge branch reuse** | 14 | 4 | The branch's MR **merged**, but the coordination file's commit is not part of that merge — the branch was pushed to again *after* its MR merged, and those later commits, including the coordination record, were never re-submitted. |

### Post-merge branch reuse — verified mechanism, not inference

For all 4 branches in this class
(`docs/site-adoption-rust-tooling` MR !42, `docs/topbar-version` MR !71,
`feat/rust-coord-stale` MR !45, `feat/v240-workstream-d-class5-labels`
MR !442), the merge commit **is** an ancestor of current `main` (the MR
genuinely merged), but the coordination file does not exist in the merge
commit's tree — confirmed with `git show <merge-commit>:<path>` failing
"path does not exist," and the file's actual introducing commit
confirmed to postdate the merge on that branch's own history. This is
the same pattern the handoff's evidence packet (MR !1002, publication-race
section) already flagged for a different reason (duplicate CI on one
SHA) — reusing an already-merged branch ref is a recurring habit in this
project's history, not a one-off.

## Special cases

**Legacy sequence-number collisions (4 records, `duplicate-rename-required`).**
The pre-timestamp-ID coordination scheme (`NNNNN-role.md`) is not
collision-safe across concurrently-worked branches. Four sequence slots
have **a different, already-published record on `main` under the same
filename**:

| Filename | On `main` (published) | Branch-only (different content) |
|---|---|---|
| `00044-*.md` | `00044-coder-tester.md` | `00044-planner-architect.md` (3 further distinct blobs across `plan/T3b-route-restore`, `plan/downstream-impact-rule`) |
| `00057-*.md` | `00057-contribution-auditor.md` | `00057-coder-tester.md` |
| `00307-*.md` | `00307-planner-architect.md` | `00307-coder-tester.md` |
| `00312-*.md` | `00312-planner-architect.md` | `00312-coder-tester.md` |

These cannot be preserved under their original filename — publishing
them unchanged would silently overwrite or shadow an already-published
record at the same path. They need a governor-approved renaming
convention (e.g. re-timestamp under the current ID scheme, with a note
that the content predates it) before any preservation, not a default
consolidation pass. This is concrete, first-party evidence for exactly
the risk the timestamp+random-suffix ID scheme was introduced to avoid.

**Legacy sequence gaps with zero trace on any of 377 branches (4 slots,
not part of the 184-record inventory).** `main`'s legacy sequence
(00001–00361) has 51 gaps; 47 of them are accounted for by branch-only
records above. Four are not: **00063, 00161, 00163, 00252** — no file at
that path exists on `main` or on any of the 377 fetched remote branches.
This scan cannot determine whether these numbers were reserved and never
written, written only to a since-deleted-and-pruned branch, or exist
only in an unpushed local clone. **Disposition: flag to the governor to
check any local-only clones or GitLab's server-side reflog/event history
before treating as permanently unrecoverable** — this plan cannot verify
that possibility itself.

## Proposed dispositions (recommendations, not decisions)

Per-record disposition is recorded in the YAML inventory; grouped here
by class since the reasoning is identical within a class:

- **137 records → `preserve-via-consolidation`** (the 75 no-MR-ever + 59
  MR-closed + 14 post-merge-reuse = 148 records in those three classes,
  minus the 4 seq-collision entries and the 7 `master`-only entries,
  both of which get their own handling below and above respectively:
  148 − 4 − 7 = 137). Rationale: a closed MR or an unmerged branch does not
  retroactively make the coordination record's *existence* untrue — the
  record documents that an action, claim, or review happened at a point
  in time, which remains true regardless of whether the associated code
  change ultimately merged. This mirrors MR !994's own precedent exactly:
  preserve unique records unchanged via one fresh-main,
  content-hash-manifested consolidation MR, validate the range, merge,
  **then** close source branches — never delete first.
- **36 records → `defer`** (the MR-still-open class). These are live,
  not archival. Bulk-consolidating a coordination record whose MR might
  still merge normally — or might still get closed, at which point it
  joins the closed-MR class — would pre-empt an in-flight decision. Per
  the v2.5.0 Statement of Work's own portfolio-disposition rule ("No
  blanket retrigger-and-sequential-merge... prefer one fresh-main
  archive for *completed* historical records"), these stay out of any
  bulk pass and get resolved individually as their MRs resolve.
- **7 records → `governor-decision`** (all 7 files found only on
  `origin/master`, the legacy pre-rename default branch, not a feature
  branch in the normal sense). Whether `master` itself is a preservable
  historical artifact or an intentionally-superseded rename byproduct is
  a project-identity question, not a mechanical one.
- **4 records → `duplicate-rename-required`** (the sequence-collision
  cases above).

**No disposition in this plan is final.** Per the handoff's explicit
boundary and `coordination-sequence-guard-rule.md`, only the
governor-reviewer issues the binding disposition for each record or
class.

## What this forensic pass deliberately does NOT do

- Does not merge, close, relabel, or delete any branch or MR.
- Does not publish any historical coordination record.
- Does not treat a merged implementation, a bridge note, or a closed MR's
  age as proof that a record's obligation is resolved — per the
  handoff's own instruction, disposition requires evidence, not
  inference.
- Does not resolve the 4 untraceable legacy-sequence gaps — flagged to
  the governor, not silently written off.

## Prevention design

Addressing each requirement named in `docs/TODO.md`'s "Historical
Branch-Only Coordination Recovery and Prevention" section and the
`tenh` handoff, without granting tooling binding gate authority it does
not have:

1. **Mechanically checkable close/merge invariant.** An MR that
   introduces a coordination record cannot be treated as disposed
   (mergeable, closeable-as-resolved) until the identical blob is
   verified present on the target branch, or an explicit governor
   disposition record names the replacement/exclusion. Implementation
   candidate: extend `agov coord publication-check` (already used for
   the `coordination:`-prefix fast path) with a `--verify-published
   <sha>` mode that fails if any `docs/coordination/*.md` blob
   introduced by the range is absent from the target tip after merge —
   run as a required post-merge check, not just pre-merge.
2. **Visible publication failures, not silent ones.** The evidence in
   this scan (36 branches with an MR that closed without merging; 34
   with no MR at all) shows failure is currently silent by default — a
   bridge note or chat summary can claim "published" with nothing
   mechanically checking it. Candidate: a required field or check
   correlating a coordination record's `id` with an actual `main`-branch
   commit before any agent or role treats the record as durably
   published in session-visible tooling output.
3. **Extend `agov` remote-visibility scanning.** The scan this plan ran
   by hand (`git fetch --prune` + `git ls-tree` per branch + GitLab MR
   join) should become a supported `agov coord scan-branches` (or
   equivalent) command: deterministic, continues past legacy
   sequence-collision diagnostics instead of erroring out (this scan
   found 4 real collisions and had to reason around them manually),
   and reports newly-introduced branch-only records separately from
   already-acknowledged historical debt (this document, once approved,
   becomes that acknowledged baseline).
4. **CI/repository check for MR-carried-but-target-absent coordination
   records**, with the negative-test coverage `always/gate-self-test-rule.md`
   requires before any such gate ships: a coordination-only MR (should
   pass once merged), an implementation MR carrying a handoff (should
   pass), a superseded record (should pass with the superseding record
   named), a byte-identical consolidation (should pass), **the exact
   sequence-collision shape found in this scan** (should fail closed,
   not silently pick one), a deleted source branch post-merge (should
   not be treated as evidence of resolution by itself), and a skipped
   pipeline (should fail closed per the existing
   `allow_merge_on_skipped_pipeline: false` project setting).
5. **Bridge notes must name the record, exact source commit, branch/MR,
   and intended publication path.** This scan's own reconstruction
   relied entirely on git/GitLab evidence, not session memory or bridge
   notes, precisely because many of the 184 records had none — the
   requirement is that future bridge notes make this reconstruction
   unnecessary, not merely possible.
6. **One owner and cursor for recurring remote visibility polling.**
   Candidate: fold into the existing coder-tester poll-and-coordinate
   cycle (`agf/docs/site/docs/operate/poll-and-coordinate.md`) as an
   additional step run on a longer cadence (e.g. weekly, not every poll
   cycle — the full 377-branch scan took real wall-clock time and API
   budget), with a bounded escalation age (e.g. 14 days branch-only
   before an agent must file a note, not silently let it accumulate).
7. **MR-close and worktree/branch-cleanup guidance checks for
   unpublished coordination before declaring work complete.**
   `workflow/branch-and-worktree-rule.md`'s existing cleanup section
   (`git worktree remove`, `git branch -d`) should gain an explicit
   pre-check: no coordination record introduced on this branch may be
   absent from the target branch at cleanup time. This is a process
   amendment to that rule, not a new rule — flagged here, not
   implemented here.

## Acceptance criteria

1. Governor-reviewer reviews the inventory
   (`docs/reviews/historical-branch-only-coordination-inventory.yaml`)
   and either accepts, amends, or requests re-scoping of the proposed
   per-class dispositions.
2. contribution-auditor independently re-verifies the method (a fresh
   `git fetch --prune` + spot-check of at least the 4 post-merge-reuse
   and 4 seq-collision special cases, since those carry the most
   consequential dispositions).
3. A follow-on implementation plan (separate Gate 1) is written for
   whichever `preserve-via-consolidation` records the governor approves,
   following MR !994's exact precedent: one fresh-main,
   content-hash-manifested archive MR, validated, merged, **then**
   source branches closed as merged-by-content — never deleted first.
4. A separate follow-on implementation plan (or plans) covers the
   prevention-design items above, each with its own Gate 1 review,
   negative-test evidence per `gate-self-test-rule.md`, and no binding
   gate authority granted to tooling beyond what is explicitly approved.

## Stop conditions

- Do not bulk-merge, publish, or delete any branch referenced in this
  plan or its inventory before governor-reviewer disposition.
- Do not treat the `defer` (open-MR) class as resolved by this plan —
  each resolves individually with its own MR.
- Do not treat the 4 untraceable legacy-sequence gaps as closed without
  the governor confirming no local-only or server-side copy exists.

## Related documents

- `docs/coordination/20260803T073923Z-governor-reviewer-tenh.md` — the
  triggering handoff.
- `docs/coordination/20260803T084008Z-planner-architect-6m6k.md`
  (MR !1007) — this role's claim of the handoff.
- `docs/reviews/historical-branch-only-coordination-inventory.yaml` —
  the full 184-record machine-readable inventory this plan summarizes.
- `docs/TODO.md`'s "Historical Branch-Only Coordination Recovery and
  Prevention" section (MR !1002) — the scope this plan responds to.
- MR !994 and its consolidation precedent — the preserve-then-close
  pattern this plan's proposed dispositions follow.
- `workflow/coordination-publication-rule.md`,
  `always/coordination-message-rule.md`,
  `always/coordination-sequence-guard-rule.md`,
  `workflow/branch-and-worktree-rule.md`,
  `always/gate-self-test-rule.md` — governing rules for the prevention
  design above.
