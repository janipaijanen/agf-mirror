---
id: 20260909T195116Z-post-v211-queue-closeout-work-order
status: approved
gate1_approved_by: 20260910T052328Z-planner-architect-771h
gate1_approved_at: 2026-09-10T05:23:28Z
topic: post-v211-queue-closeout-work-order
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: >-
      Consultation requested at this Gate 1, not yet on record. This work
      order closes forge MRs/issues per an evidence ledger, creates durable
      `reconstruct` carriers, and edits two existing portfolio ledger files.
      InfoSec must confirm at Gate 1: no coordination / security / gate
      evidence is lost when a source MR closes (the content-hash archive is
      the control); `reconstruct` carriers do not surface protected or
      credential content; and the `#129` / `!2459` / `!2460` / `!2462`
      semantic-data exclusion is honoured for analysis, reconstruction,
      carrier work, and closure. Execution batches that touch anything
      beyond forge state + ledger rows get their own exact-head screen.
  - role: contribution-auditor
    status: waived
    reason: >-
      Consultation requested at this Gate 1, not yet on record.
      Contribution-auditor must confirm: the vocab and evidence bar match
      the approved queue-convergence plan §3b; `close-superseded` never
      closes before the successor is demonstrably on `main`; `reconstruct`
      never closes before a durable visible carrier exists; the batch
      consent-agenda does not become bulk closure without per-item
      evidence; and each batch record is a faithful, auditable list. Every
      execution batch also gets a non-author contribution review at its
      exact state.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Post-v2.11 queue closeout work order (Gate 1)

## Authority and provenance

- Governor direction (2026-09-09): "plan draft … mechanism to close vast
  majority of open MRs and Issues"; `st83`; and **`j8bq`** — which
  supersedes the parent scope plan's "verified B2–B8 terminal disposition
  before the metadata MR" requirement, makes release-first binding, and
  **authorizes planner-architect to draft this separate post-v2.11
  queue-closeout work order** (no Gate 1 approval granted by that record).
- Split from the withdrawn `docs/plans/20260909T185609Z-…` per review
  `nj1j` (V211 advisory: the release-publication child plan and this
  closeout work order have different authority, rollback, and readiness
  boundaries). The release-publication child plan is a separate artifact.
- Substrate ledgers already exist and are current:
  `docs/portfolio/post-v248-issue-disposition.md` (B3; Claude review
  `pgq8` CLEAR) and `docs/portfolio/post-v210-mr-disposition.md` (MR
  ledger; B4–B7 cohorts feed it).
- This work order authorizes **no** MR close, issue close, tag, release,
  or metadata merge. Every execution batch returns for its own binding
  governor decision. Execution begins only **after v2.11 is published and
  verified** (`j8bq`).

## Scope

Bring the open-MR and open-issue queue to a small intentional set by
executing evidence-backed terminal dispositions. Live forge state at
drafting (2026-09-09): **64 open issues, 58 open MRs.** Each execution
batch re-reads and **freezes its own baseline** immediately before acting;
the numbers here are indicative only.

## One vocabulary, two ledgers

Both ledgers use the queue-convergence plan §3b terminal vocabulary:
`ship-in <release>`, `merge-now`, `reconstruct`, `close-superseded`,
`close-invalid`, `park-radar`, `awaiting-governor`. Each row cites named
evidence; "age is not evidence"; "absence of a row is not a disposition".

Current issue-ledger split (B3, live open only, corrected per `nj1j`
V211-P5): **8 `close-superseded` + 20 `reconstruct` + 26 `park-radar` +
5 `ship-in v2.11.0` + 4 `awaiting-governor` + 1 `ship-in` (unnumbered
candidate — resolve per `pgq8` A1) = 64.**

## Per-disposition execution rules

| Disposition | Execution |
|---|---|
| `close-superseded` | Close only after the successor / preservation is **demonstrably on `main`**; the closure note names it by path / MR / commit. Batch-eligible. |
| `reconstruct` | A **durable, visible carrier must exist first** (`nj1j` V211-P6): a tracked file (branch + a `reconstruct/*` plan-or-note artifact) containing the accepted unique scope, the owner, the gate / acceptance criteria, the unique content itself, backlinks to the source MR/issue and this work order, and passing `agov` validation where applicable. An empty branch or deletable stub does **not** qualify. Shared carriers (e.g. `reconstruct/identity-accountability-gate-enforcement`) are created once and backlinked by each contributing item. Only then close the source. |
| `close-invalid` | Close with the specific evidence that falsifies the premise. Batch-eligible. |
| `merge-now` / "Refresh candidate" that survives review | **Never batch.** Own exact-head review + current gate + Gate 2. |
| `park-radar` | Keep the issue open; add label `radar` + the `review-by` date + trigger from the ledger row. (MRs rarely `park-radar`; if one does, close with a `radar/` record.) |
| `awaiting-governor` | Keep open; label `awaiting-governor`; the ledger row states the exact governor-only question. |
| `ship-in <release>` | Keep open; closes on verified publication of that release (informational changelog reference only — `Closes #N` in CHANGELOG does **not** auto-close on GitLab, per `nj1j` V211-P4). |
| `Archive → close` (MR ledger, coordination/review/session-closeout carriers) | Preserve unique immutable records in a content-hash-manifested `docs/coordination-archive/<date>-<cohort>/` archive (the proven B4 pattern — MR !2581 / `8bcaef31`), **merge + validate the archive first**, then close each source MR with the canonical link. Batch-eligible once the archive is on `main`. |

## Batch mechanism (`nj1j` V211-P7)

Closing forge state is not a merge-request change, so it does not need one
MR per close. Per class cohort:

1. **Batch authorization record** — planner-architect submits a `handoff`
   / proposal listing, for every item: the exact IID, its forge state at
   read time, the disposition, the evidence, the exact closure-note text,
   and a digest (SHA-256) over `{IID, state, disposition, note}`. This is a
   proposal, not a decision.
2. **Non-author contribution review** of that batch record at its exact
   state.
3. **Governor decision record** — the governor issues a `decision-record`
   approving the batch as a consent agenda. Any single item may be
   **pulled** for individual handling and leaves the batch. Only the
   governor's record authorizes closure.
4. **Pre-execution re-read** — immediately before acting, re-read each
   item's live forge state. **Any item whose state, head, or discussion
   changed since the digest is invalidated** and dropped from this batch
   (re-proposed later). `reconstruct` carriers and `Archive → close`
   archives must be on `main` at this point.
5. **Execute** — for each surviving item, idempotently: post the closure
   note (skip if already present), then close the MR/issue via the forge
   API (skip if already closed). **Stop on the first unexpected API
   failure**; do not continue the batch.
6. **Completion record** — list every IID actually closed, every item
   pulled, every item invalidated at step 4, every item skipped at step 5,
   the archive/carrier SHAs, and the stop point if step 5 halted.
7. **Ledger reconciliation MR** — a normal MR marking the executed rows
   `closed <date> (batch <record-id>)` in
   `post-v248-issue-disposition.md` / `post-v210-mr-disposition.md`. This
   ledger edit is the **only tracked-file change**; it takes ordinary
   **Gate 2**, separate from the governor's batch authorization.

## Partial failure and recovery (`nj1j` V211-P7)

- A batch that stops at step 5 is **partially complete**: the completion
  record is authoritative for what closed. The remainder is re-proposed as
  a fresh batch after the failure cause is understood.
- A wrongly-closed item is **reopened** and re-dispositioned via a new
  batch — never argued in place.
- The ledger reconciliation MR only marks rows that the completion record
  says actually closed. Ledger and forge are reconciled by the completion
  record, which pins IIDs + SHAs.

## Cohort plan

| Cohort | Source | Notes |
|---|---|---|
| C-ISS-SUP | 8 `close-superseded` issues | successor-on-`main` check per row |
| C-ISS-REC | 20 `reconstruct` issues | carriers first; shared carriers batched |
| C-ISS-INV | any `close-invalid` issues found on re-read | |
| C-ISS-PARK | 26 `park-radar` issues | label + date only; no close |
| C-MR-ARCH | coordination/review/session-closeout carrier MRs (MR ledger) | one dated archive per cohort |
| C-MR-SUP | `close-superseded` MRs | successor-on-`main` check |
| C-MR-STALE-G1 | the ~12 stale Gate 1 plan MRs (`qqg4` B8-A1) | each `close-superseded` / `close-invalid` / `merge-now` via own Gate 1 |
| C-MR-CLASS | remaining B4–B7 class cohorts (learning, radar/sources, docs/governance, Rust/tooling) | disposition first, then execute |

`awaiting-governor`, `ship-in`, `merge-now`, and the `#129` /
`!2459` / `!2460` / `!2462` semantic-data exclusion set are **never**
in a closeout batch absent a fresh governor decision (`j8bq`).

## Sequencing

Execution of every cohort waits for **v2.11 published and verified**
(`j8bq`). Cohort *dispositioning* (writing ledger rows for
not-yet-dispositioned MRs — C-MR-CLASS) is analysis and may proceed in
parallel; it opens and closes nothing.

## Decomposition (each its own gate / batch decision)

| Slice | Content | Gate |
|---|---|---|
| QC-1 | Complete C-MR-CLASS dispositioning (ledger rows for all remaining open MRs) | non-author review of the ledger MR → Gate 2 |
| QC-2 | Carrier creation for all `reconstruct` issues/MRs (durable visible carriers) | non-author review → Gate 2 per carrier batch |
| QC-3 | Archive creation for C-MR-ARCH cohorts (content-hash manifests) | non-author review → Gate 2 per archive |
| QC-4 | Execution batches (C-ISS-SUP, C-ISS-REC, C-MR-ARCH, C-MR-SUP, C-MR-STALE-G1) | batch authorization record → non-author review → **governor decision record** → execute → completion record → ledger reconciliation MR (Gate 2) |

## Downstream impact

Classification: **ADDITIVE** — closes forge state and edits two existing
portfolio ledger files that exist for this purpose. No rule, schema,
validator, CI, or adopter-facing change; the batch mechanism is a working
procedure, not a gate.

```
Impact:
  Foundry:          Yes — the open queue is brought to a small intentional
                    set; the B8 advisory intake bound becomes meaningful.
  Direct adopters:  None.
  Downstream:       None.
  Migration:        None.
  Evidence:         j8bq; st83; queue-convergence plan j4rg §3b/§3d; B3
                    issue ledger (pgq8 CLEAR); post-v210 MR ledger; B4
                    archive pattern (MR !2581 / 8bcaef31); nj1j
                    V211-P5/P6/P7; qqg4 B8-A1.
```

## Risks and controls

- **A weak disposition closes something unique.** Control: §3b evidence
  bar; carrier-first / archive-first; non-author review of every batch
  record; reopen is cheap and defined.
- **Batch approval becomes rubber-stamping at scale.** Control: class-
  scoped review-sized cohorts; any item pullable; the completion record's
  exact IID list is auditable; the governor sets batch size.
- **Live-state race between proposal and execution.** Control: the step-4
  digest re-read invalidates any changed item.
- **Ledger drifts from forge.** Control: the completion record pins IIDs +
  SHAs; the reconciliation MR only marks confirmed-closed rows.
- **Semantic-data exclusion violated via carrier/analysis work.** Control:
  `#129` + `!2459` / `!2460` / `!2462` are hard-excluded from every slice
  (`j8bq`); InfoSec confirms at Gate 1.

## Validation before Gate 1

`agov plan validate` on this file; coordination visibility/collision scan;
`git diff --check`. No projection manifest — no normative rule text; a work
order over existing ledgers and the existing release/queue-convergence
rules.

## Decision requested at Gate 1

Approve (a) the one-vocab / two-ledger model, (b) the per-disposition
execution rules including the durable-carrier requirement for
`reconstruct`, (c) the batch mechanism (planner proposal → non-author
review → governor decision record → digest-verified execution → completion
record → separate Gate 2 ledger MR) and its partial-failure/reopen
recovery, (d) the cohort plan and the exclusion set, (e) the
"execution after v2.11 published; dispositioning may run in parallel"
sequencing, and (f) the QC-1…QC-4 decomposition. Every slice and every
execution batch returns for its own binding decision.
