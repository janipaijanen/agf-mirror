---
id: 20260909T185609Z-v211-release-and-queue-closeout
status: pending-gate1
topic: v211-release-publication-and-queue-closeout-mechanism
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: >-
      Consultation requested at this Gate 1, not yet on record. Part A is
      release routing + version/changelog metadata over already-gated
      content; Part B closes forge MRs/issues per an evidence ledger and
      creates `reconstruct` carrier branches. InfoSec must confirm at Gate 1
      that: the release mechanism keeps the mandatory `release:namespace-check`
      and protected-tag checkpoints; no coordination/security/gate evidence
      is lost when a source MR is closed (the B4-style content-hash archive
      is the control); and no credential, corpus, or authorization surface
      is touched by a closeout. The execution batches get their own
      exact-head InfoSec screen where a batch touches anything beyond forge
      state + ledger rows.
  - role: contribution-auditor
    status: waived
    reason: >-
      Consultation requested at this Gate 1, not yet on record.
      Contribution-auditor must confirm: the closeout vocab and evidence
      bar match the approved queue-convergence plan §3b; `close-superseded`
      never erases unique work (archive-first / carrier-first); the batch
      consent-agenda mechanism does not become bulk closure without
      evidence; and the CHANGELOG's closed-issue list is faithful to what
      actually shipped. Each execution batch also gets a non-author
      contribution review at its exact state.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.11 release publication + post-release queue closeout mechanism (Gate 1)

## Authority and provenance

- Governor direction (2026-09-09): "plan draft release content and mechanism
  to close vast majority of open MRs and Issues"; and `st83`: "After RUW
  v2.11 is published and verified, the active priority is to dispose of the
  remaining open merge requests to terminal states using their ledgers,
  current gates, exact-head evidence, and ownership … not authority for
  indiscriminate bulk closure or for bypassing required gates."
- Parent scope: `docs/plans/20260909T021841Z-v211-release-scope.md`
  (Gate 1 approved `sd23`, amended for the #129 withdrawal `st83`). That
  plan **requires** a child v2.11 release-publication plan with its own
  Gate 1 — **Part A is that child plan.**
- Ledgers already in place: `docs/portfolio/post-v248-issue-disposition.md`
  (B3, 64 open issues each dispositioned; Claude review `pgq8` CLEAR) and
  `docs/portfolio/post-v210-mr-disposition.md` (MR ledger; B4-B7 cohorts
  feed it).
- This is a **draft work-order + mechanism plan.** It authorizes no version
  bump, tag, release, MR close, or issue close. Every execution batch
  returns for its own governor decision.

---

## Part A — v2.11 release publication

### A1. Release-cut preconditions (the gate before any release branch opens)

The v2.11 release-metadata MR may open only when **all** of these hold. This
restates the parent scope plan's "Release-cut precondition" section against
current state (2026-09-09):

| IN workstream | State today | Blocks cut until |
|---|---|---|
| Queue convergence B1/B2/B3/B8 | B1 merged; B2 merged; B3 merged; B8 merged | done |
| Queue convergence B4–B7 (MR cohorts) | ledger exists; cohorts not complete | every cohort dispositioned + its Gate 2, per Part B3 |
| Improvement Propeller Stage 4 (#131) | activated (MR !2597); heartbeat armed | **first genuine unattended Monday wake + separate Level-5 record** |
| `qbka` Slice A / Slice B | no Gate 1 plan yet | Slice A schema+validator+transition merged; Slice B migration merged |
| Coordination lifecycle semantics — #121, #122, !2027, CP-A | no work-order Gate 1 yet (`mbg6` task 2) | work order Gate 1 + each slice's Gate 2; #122 after `qbka` Slice A |
| Coordination discovery/collision — #123, CP-C | no Gate 1 plan yet | Gate 1 + Gate 2 |
| Coordination publication efficiency — CP-B | CP-B-1 Gate 1 approved (`st83`), not implemented; CP-B-2 no plan | CP-B-1 impl merged + evidenced **and** CP-B-2 merged + evidenced |
| Pre-cut governor dispositions (`release-preflight-learning-gate2`, `capability-developer-startup-sync`) | MRs merged; owed a governor decision/supersession record | that record filed |

**Implication:** v2.11 is weeks out. This plan is drafted now so the
mechanism is ready; A2–A4 execute only after A1 is fully green.

### A2. Release content

**Version surfaces** (bump `2.10.0 → 2.11.0`, `semver-no-carry-rule`):
`VERSION`; every `agf/agents/*.yaml` `framework_version`; `README.md`
major.minor citation; `agf/agents/improvement-propeller.yaml` and any other
definition carrying `2.10.0`. The exact surface list is the parent scope
plan's metadata-MR-contents bullet, frozen at cut time.

**`CHANGELOG.md` + `agf/CHANGELOG.md`** entry, per the `release` skill:
- one-line release theme (RUW — autonomous AGF development: coordination
  protocol semantics, publication efficiency, queue convergence, Propeller
  Level 5);
- test count from the reversible preflight;
- `Closes #N` for every issue whose B3 disposition is `ship-in v2.11.0`
  (currently #121, #122, #123, #124, #131) and any `close-superseded` issue
  whose canonical artifact ships in v2.11;
- changes grouped BREAKING / features / fixes, with the `downstream-impact`
  classification per merged workstream;
- evidence citations (learning IDs, coordination decision records).
- The changelog MUST NOT describe behaviour not yet on `main` (CP-B-2, etc.
  only if merged by cut time).

### A3. Release mechanism (standard path)

Per `release-via-merge-request-rule` + the `release` skill, **standard
path** (not maintenance-line, not frozen-scope reconstruction — no
eligibility condition for those applies):

1. Reversible preflight from a clean checkout: full Rust test suite + each
   built framework archive's shipped tests; record counts.
2. `release:namespace-check` MR job + the operator preflight
   (`agov release namespace-check --phase preflight`) — exit 0 / `ABSENT`
   for all four namespaces (Git tag, GitLab Release, generic package, OCI).
3. `release/v2.11.0` branch from `main`; commit only the A2 metadata.
4. MR → `main`; diff is metadata-only; exact-head pipeline green.
5. Governor Gate 2 on the metadata MR.
6. After merge: `git checkout main && git pull`; the mandatory pre-push
   `agov release namespace-check --phase pre-push` (HEAD == live target);
   re-read the exact `v2.11.0` protected-tag rule; then `git tag v2.11.0 &&
   git push origin v2.11.0` in one action.
7. Tag pipeline builds/publishes archives + OCI; anonymous post-publication
   verification on the downloaded bytes.

### A4. What Part A does NOT authorize

Steps A3.6–A3.7 (tag creation + push) require a **separate binding governor
decision after the metadata MR merges**, per the parent scope plan. This
plan supplies the procedure; it does not pre-authorize the tag.

---

## Part B — post-release queue closeout mechanism

### B1. One vocab, two ledgers

Both ledgers use the queue-convergence plan §3b terminal vocabulary:
`ship-in <release>`, `merge-now`, `reconstruct`, `close-superseded`,
`close-invalid`, `park-radar`, `awaiting-governor`. Each row cites named
evidence. "Age is not evidence"; "absence of a row is not a disposition".

### B2. Issue closeout (64 open, all dispositioned in B3)

Current B3 tallies: 24 `close-superseded`, 20 `reconstruct`, 26
`park-radar`, 5 `ship-in v2.11.0`, 4 `awaiting-governor`, 1 `ship-in`
(unnumbered candidate — see `pgq8` A1).

Per-disposition execution:

| Disposition | Action |
|---|---|
| `close-superseded` | Close the issue with a note naming the canonical artifact from its ledger row. Batch-eligible. |
| `reconstruct` | **Create the named carrier branch first** (empty branch or a stub `reconstruct/*` note capturing what is unique), then close the issue with a note linking the carrier. Shared carriers (e.g. `reconstruct/identity-accountability-gate-enforcement`) are created once and referenced by each contributing issue. |
| `park-radar` | Add label `radar` and the `review-by` date; **keep open** as a labelled watch item, OR (governor's choice per batch) close with the ledger row as the durable record and a `radar/` tracking file. Default: keep open + label. |
| `awaiting-governor` | Keep open; label `awaiting-governor`; the ledger row states the exact governor-only question. |
| `ship-in v2.11.0` | Keep open; closes via `Closes #N` in the v2.11 CHANGELOG (Part A2) when the release merges. |
| `ship-in <unnumbered>` | Resolve per `pgq8` A1 — either a numbered target or reclassify `awaiting-governor`. |

### B3. MR closeout (54 open; ledger `post-v210-mr-disposition.md`)

1. **Complete B4–B7 cohorts.** Every currently-open MR gets a ledger row
   with an evidence-backed disposition, in cohorts by class (learning
   records, radar/sources, plan MRs, docs/governance, coordination
   carriers, Rust/tooling). Fatigue control: cohorts are like-with-like; an
   unexaminable row is recorded `unexamined`, never given a weak
   disposition.
2. **Per-disposition execution:**
   - `Archive → close` (coordination/review/session-closeout carriers):
     preserve unique immutable records in a content-hash-manifested
     `docs/coordination-archive/<date>-<cohort>/` archive (the B4 pattern,
     already proven — MR !2581 / `8bcaef31`), merge + validate the archive,
     then close each source MR with the canonical link. Batch-eligible once
     the archive is merged.
   - `close-superseded` / supersession candidate: close only after the
     successor/preservation is demonstrably on `main`; the closing note
     names it. Batch-eligible.
   - `close-invalid`: close with the specific evidence that falsifies the
     premise.
   - `merge-now` / `Refresh candidate` that survives review: goes through
     its **own** exact-head review + current gate + Gate 2 — never
     batch-closed, never batch-merged.
   - `reconstruct`: carrier branch first, then close.
   - `park-radar`: rare for MRs; if used, close with a `radar/` record.
3. **Stale Gate 1 plan MRs** (the ~12 flagged in `qqg4` B8-A1, mostly
   2026-06/07) are their own cohort: each is `close-superseded` (a later
   plan controls), `close-invalid` (premise gone), or `merge-now` via its
   own Gate 1 — resolving this cohort is what brings the B8 intake bound
   under its ≤3 limit.

### B4. Batch mechanism (how "the vast majority" close without MR sprawl)

Closing forge state is not itself a merge-request change, so it does not
need one MR per close. Instead, per cohort:

1. **Disposition-batch coordination record** (`kind: note` or `decision-record`
   request), listing every MR/issue in the cohort with: IID, disposition,
   the evidence, and the exact closure-note text to post.
2. **Non-author contribution review** of that batch record at its exact
   state (CLEAR or findings).
3. **Governor approves the batch** as a consent-agenda list (any single
   item may be pulled for individual handling; pulled items leave the
   batch).
4. **Execute:** post each closure note, then close each MR/issue via the
   forge API. `reconstruct` carriers and `Archive → close` archives are
   created/merged *before* the batch executes.
5. **Completion record** listing the exact IIDs closed, the archive/carrier
   SHAs, and any pulled items.

Ledger files (`post-v248-issue-disposition.md`,
`post-v210-mr-disposition.md`) are updated in a normal MR to mark rows
`closed <date>` with the batch record ID — that ledger edit is the only
tracked-file change; the closes themselves are forge operations.

### B5. Safety rails (per `st83`: "not indiscriminate bulk closure")

- No issue/MR closes without a ledger row **and** an evidence citation
  **and** a posted closure note.
- `reconstruct` never closes before its carrier exists; `close-superseded`
  never before the successor is on `main`; `Archive → close` never before
  the archive is merged + validated.
- `merge-now` items never batch — own gate.
- Every close is individually reversible (reopen) and the batch record is
  the audit trail; a wrongly-closed item is reopened and re-dispositioned,
  not argued.
- **Exclusions kept open regardless:** anything `awaiting-governor`;
  `ship-in` items until their release ships; MRs !2409/!2410/!2452 (B4
  exclusion — importing their unresolved `requires-ack` would regress the
  queue); any MR the governor names.
- Semantic/linked-data-scoped items (#129 and its cluster) close only per a
  separate governor decision — `st83` standing direction unchanged.

---

## Sequencing

Part A **before** Part B (per `st83`: "After RUW v2.11 is published and
verified …"). B4–B7 cohort *dispositioning* (B3.1) may proceed in parallel
with A1 preconditions since it is analysis, not execution; the *execution*
batches (B4) wait for the v2.11 tag to be published and verified. The one
exception is the stale-Gate-1-plan-MR cohort (B3.3): dispositioning it
early is what unblocks the B8 intake bound and lets new v2.11 planning
proceed under the ≤3 limit — its execution may run pre-release with its own
governor batch approval.

## Decomposition (each slice its own Gate 1 evidence + Gate 2 / batch approval)

| Slice | Content | Owner |
|---|---|---|
| **RC-A** | v2.11 release-publication procedure detail: frozen metadata surface list, CHANGELOG draft, preflight/namespace/protected-tag checklist. This plan is the skeleton; RC-A fills the exact values at cut time. | planner-architect → governor Gate 1; executes only after A1 green |
| **RC-B1** | Stale Gate 1 plan-MR cohort disposition + batch close (pre-release, unblocks B8 bound) | planner-architect dispositions → non-author review → governor batch approval |
| **RC-B2** | Remaining B4–B7 cohorts disposition (learning, radar/sources, docs/governance, coordination carriers, Rust/tooling) | planner-architect / coder-tester per class → non-author review |
| **RC-B3** | Issue closeout batches (post-release): carriers created, then `close-superseded` + `reconstruct` batches | planner-architect → non-author review → governor batch approval |
| **RC-B4** | MR closeout batches (post-release): archives merged, then `Archive → close` + `close-superseded` batches | coder-tester (archives) + planner-architect (batch records) → review → governor |

## Downstream impact

Classification: **ADDITIVE**. Part A is release metadata (no contract
change). Part B closes forge state and edits two portfolio ledger files
that already exist for this purpose; it adds no rule, schema, validator,
CI, or adopter-facing change. The batch mechanism is a working procedure,
not a gate.

```
Impact:
  Foundry:          Yes — the open-MR/issue queue is brought to a small,
                    intentional set; the B8 intake bound becomes meaningful.
  Direct adopters:  None. No shipped rule, schema, tool, or doc changes.
  Downstream:       None.
  Migration:        None.
  Evidence:         st83; queue-convergence plan j4rg §3b/§3d; B3 issue
                    ledger (pgq8 CLEAR); post-v210 MR ledger; B4 archive
                    pattern (MR !2581 / 8bcaef31); qqg4 B8-A1 (stale
                    Gate 1 plan-MR count).
```

## Risks and controls

- **A weak disposition closes something unique.** Control: evidence bar
  per §3b; `reconstruct`/archive-first; non-author review of every batch;
  reopen is cheap.
- **The release cuts before an IN precondition is truly done.** Control:
  A1 is a hard checklist; the Propeller Level-5 and `qbka` conditions are
  non-merge events explicitly enumerated.
- **Batch approval becomes rubber-stamping at scale.** Control: cohorts
  are class-scoped and review-sized; any item is pullable; the completion
  record's exact IID list is auditable; the governor sets batch size.
- **Closing MRs races with new work.** Control: RC-B execution waits for
  the v2.11 tag; the intake bound (B8) limits new opens meanwhile.
- **Ledger and forge drift.** Control: each batch re-reads forge state
  immediately before executing; the completion record pins IIDs + SHAs.

## Validation before Gate 1

`agov plan validate` on this file; coordination visibility/collision scan;
`git diff --check`. (No projection manifest: this plan introduces no
normative rule text — it is a work order over existing ledgers and the
existing release rule.)

## Decision requested at Gate 1

Approve (a) Part A as the required child v2.11 release-publication plan
(procedure only; tag/publish stays a separate post-merge governor
decision), (b) the A1 precondition checklist as the binding gate before a
release branch opens, (c) Part B's one-vocab / two-ledger / per-disposition
execution rules, (d) the B4 batch consent-agenda mechanism and B5 safety
rails, (e) the Part A → Part B sequencing with the two named pre-release
exceptions (B4–B7 dispositioning; the stale-Gate-1-plan-MR cohort), and
(f) the RC-A…RC-B4 decomposition. Each slice and each execution batch
returns for its own binding decision.
