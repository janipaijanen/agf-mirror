---
id: historical-branch-only-coordination-recovery
status: pending-gate1
topic: historical-branch-only-coordination-recovery-and-prevention
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Round 1 review (`20260803T093704Z-contribution-auditor-1ajh.md`) at
      exact head `365d192e`: BLOCK, six blocking findings (B-HR-1 through
      B-HR-6) plus two advisories, all addressed in the first revision.
      Round 2 review (`20260803T114233Z-contribution-auditor-hpaq.md`) at
      exact head `1c5223f3`: BLOCK, five further blocking findings plus
      one advisory, confirming B-HR-4 and B-HR-5 clear but finding the
      first revision's fixes for B-HR-1/B-HR-2/B-HR-3/B-HR-6 incomplete.
      B-HR-EVID-1: `acknowledged-elsewhere` classifications named no
      actual citing record. B-HR-ROOT-2: MR state was still labelled
      "root cause" rather than observed state. B-HR-OBL-2: any citation
      was treated as acknowledgement, including for handoffs, which have
      their own take/complete lifecycle. B-HR-VAL-1: the plan said "29
      records fail validation" when the true total is 67 (29 was only
      the disposition-bucket subset) — independently confirmed by this
      role before `hpaq` arrived. B-HR-COST-2: "wasted" CI compute and
      "invisible obligations" overstated what the evidence proves.
      A-HR-MR-1 (advisory): MR !1010's description was still the default
      template. All five findings and the advisory addressed in this
      second revision — see the renamed/added sections below. A bounded
      renewed review is requested before this is treated as Gate 1-ready.
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
      from this project's own git remote-tracking refs, local branches,
      and GitLab API — no external or comparative claim requiring
      independent sourcing.
---

# Plan: recover and prevent historical branch-only coordination records

**Date:** 2026-08-03T08:46:30Z (drafted); revised 2026-08-03T11:45:00Z
per contribution-auditor's round 1 review (`1ajh`); revised again
2026-08-03T12:05:00Z per contribution-auditor's round 2 review
(`hpaq`).
**Status:** REVISED DRAFT (round 2) — round 1's six findings (B-HR-1
through B-HR-6) and round 2's five findings (B-HR-EVID-1, B-HR-ROOT-2,
B-HR-OBL-2, B-HR-VAL-1, B-HR-COST-2) plus both advisories are addressed
below; a bounded renewed review is requested before this is treated as
Gate 1-ready. Proposed dispositions remain recommendations only. Final
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

## Method (mechanically reproducible, revised per B-HR-1/B-HR-2/B-HR-5)

1. `git fetch origin --prune` — 383 remote-tracking branches at this
   scan's time (up from 377 at the first draft; more MRs opened between
   drafts). **Per B-HR-5:** additionally enumerated every **local**
   branch with no remote-tracking counterpart
   (`git branch` minus `git branch -r`) — **318 local-only branches**,
   scanned identically and labelled as a separate, non-publication
   recovery source (local presence is not shared visibility).
2. `git ls-tree -r --full-tree <ref> -- docs/coordination/` for
   `origin/main` (2048 files at this scan) and for each of the 383
   remote + 318 local-only refs.
3. **Per B-HR-4, the inventory unit is corrected to (path, blob) — a
   distinct message — not path alone.** A legacy path with several
   distinct branch-only blob contents (e.g.
   `00044-planner-architect.md`, 3 blobs) is now counted and evidenced
   as multiple records sharing one legacy filename, not collapsed into
   one. `docs/coordination/README.md` is excluded from the record set
   entirely: it is a directory index, not an append-only message
   record, and its cross-branch drift is ordinary documentation
   editing, not a lost obligation.
4. A (path, blob) pair is excluded from the candidate set only when it
   **exactly** matches `main`'s current blob at that path — i.e.,
   already published verbatim.
5. **Per B-HR-1, for every remaining candidate:** extracted the full
   frontmatter evidence envelope directly from blob content — `id`,
   `kind`, `from`, `to`, `timestamp`, `requires-ack`, `immutable`,
   `topic`, `references-to`, `model` — not path/branch/MR-state alone.
6. **Per B-HR-2 (evidence beyond MR state — observed state, not causal
   root cause; see B-HR-ROOT-2 correction in item 8 below):**
   - **Collision classification** — compared frontmatter field-sets
     against `main`'s content at the same exact path, and, for legacy
     `NNNNN-role.md` paths, against `main`'s content under the same
     5-digit sequence number regardless of role suffix. See "Collision
     classification" below — this replaced the original draft's single
     `multi_blob_conflict` flag with three distinct, evidenced classes.
   - **Originating commit** — for every (branch, blob) pair, walked
     `git log`/`git ls-tree` backward on that branch to find the
     earliest commit introducing this exact blob at this exact path.
   - **Bundling evidence** — checked whether the introducing commit
     also touched non-coordination files (`git show --name-only`),
     distinguishing a dedicated coordination commit from one bundled
     into implementation work.
   - **MR-state evidence** — cross-referenced every candidate's
     carrying branch(es) against all **1029** project merge requests
     (fetched via paginated GitLab API, `state=all`; up from 1008 at
     first draft).
   - **Validation evidence** — ran each record's actual extracted
     content through `tooling/rust/target/release/agov coord validate`
     for a real pass/fail result, not an inferred one.
7. **Per B-HR-3/B-HR-EVID-1/B-HR-OBL-2 (obligation state and governed
   disposition, not a root-cause-class default, with named evidence):**
   scanned every current `main` coordination file's `references-to`
   field for a citation of each candidate's `id` or filename, and
   **recorded the actual citing record(s)** — path, id, kind, timestamp
   — as a `citing_main_records` field, not a boolean (B-HR-EVID-1). A
   citation is treated as **visibility, not proof of resolution**
   (B-HR-OBL-2): for `kind: handoff` records specifically, only a
   citing record of `kind: completion` counts as resolution
   (`resolved-by-completion`); any other citation is `cited-elsewhere`
   (visibility only). `requires-ack: true` records are never
   short-circuited by a mere citation or by the source MR still being
   open — they are flagged `unresolved-live` (or
   `unresolved-live-but-visible` if also cited) ahead of both.
   `proposed_disposition` is derived per-record from the full ordered
   combination — see "Obligation state and disposition methodology"
   below.
8. **Per B-HR-ROOT-2:** the field this document's round-1 revision
   called `root_cause_class` is renamed `observed_state_class` — it
   describes observed MR/repository state (does an MR exist, is it
   open/closed/merged, does the path collide), not causation. A
   separate `causal_root_cause` field is added and is honestly
   `undetermined` for every record: this scan gathered MR state,
   validation results, bundling evidence, and collision classification,
   but no close-reason, discussion-thread, or bridge-note evidence —
   observed state is not the same as knowing *why* a branch never got
   an MR or *why* an MR closed without merging, and this document no
   longer implies otherwise.
9. **Per B-HR-6/B-HR-COST-2:** built a bounded operational-cost
   assessment from real, queryable evidence (GitLab pipeline counts),
   labelled to match exactly what the evidence proves — "correlated,"
   not "wasted"; "unadjudicated," not "decided" — with categories that
   have no recoverable evidence explicitly labelled
   unknown/not-recoverable rather than estimated. See "Operational cost
   assessment" below.
10. Full machine-readable output, regenerated under this corrected
   method:
   `docs/reviews/historical-branch-only-coordination-inventory.yaml`
   (**200 distinct (path, blob) records** across **198 distinct
   paths**, one entry per record with its full evidence envelope,
   collision classification, originating commit(s), bundling evidence,
   MR-state evidence, validation result, obligation state, and
   per-record proposed disposition). This document's tables below are a
   summary of that file, not a replacement for it.

**Excluded from the 200:** this task's own in-flight claim and review
records (`20260803T084008Z-planner-architect-6m6k.md`,
`20260803T084905Z-planner-architect-r6sr.md`, and this revision's own
in-progress branch) — that is current work under active coordination
tracking, not historical debt.

## Summary: 200 records across 198 paths, five observed-state classes

**Per B-HR-ROOT-2, these are observed MR/repository states, not root
causes.** This scan can establish *that* a branch has no MR, or *that*
an MR closed without merging — it cannot establish *why* without
discussion-thread, close-reason, or bridge-note evidence this pass did
not collect. `causal_root_cause` is recorded as `undetermined` on every
record for exactly this reason.

| Observed state | Records | Evidence |
|---|---|---|
| **No MR ever opened** | 72 | Branch pushed, coordination record(s) written, no merge request exists for that branch in any state, in 1029 fetched MRs. |
| **MR closed, never merged** | 59 | An MR exists for the branch; every MR for it has `state: closed`. |
| **MR still open today** | 40 | An MR exists and at least one is `state: opened` — live, not archival. |
| **Path collision** | 15 | The record's path collides with `main`'s current content, either at the exact same filename or the same legacy sequence number under a different role suffix — see "Collision classification" below. Takes priority over MR-state classification since the collision itself is the more consequential fact. |
| **Post-merge branch reuse** | 14 | The branch's MR **merged**, but the coordination file's commit is not part of that merge — the branch was pushed to again *after* its MR merged, and those later commits, including the coordination record, were never re-submitted. |

Observed-state-class counts do not sum one-to-one with disposition
counts (below) — per B-HR-3/B-HR-OBL-2, disposition is derived from the
full evidence combination, not the observed-state class alone. Of the
40 `mr-open` records, 18 have `requires-ack: true` (with or without an
additional citation) and are pulled into `preserve-via-consolidation`
ahead of `defer`, since an unresolved live obligation is more
consequential than the source MR simply still being open; the
remaining 22 get `defer`.

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

## Collision classification (replaces the original single `multi_blob_conflict` flag, per B-HR-2/B-HR-4)

15 records collide with `main`'s current content. Comparing frontmatter
field-sets (not just noting a conflict exists) splits this into three
evidenced classes:

**3 records — `same-message-edited-draft` (discard, no rename needed).**
The branch-only blob and `main`'s published blob share the identical
`id` and every frontmatter field except `timestamp`; the body text
differs by minor wording only. This is a stale pre-merge draft of the
*same* message, refined before its actual merge — not a naming
collision. Confirmed for all 3
(`20260623T200805Z-coder-tester-r8g8.md`,
`20260723T202047Z-planner-architect-6get.md`,
`20260726T122234Z-build-engineer-mk8p.md`) by direct diff. Disposition:
`main`'s published version is authoritative; the branch draft is
superseded and needs no separate preservation.

**6 records — `different-message-exact-path-collision`.** The
branch-only blob and `main`'s content at the *same exact filename*
differ in `kind`, `to`, `topic`, `references-to`, or `requires-ack` —
two genuinely different messages landed at the same path. Two of these
are **timestamp-format IDs with the identical id string**
(`20260722T102341Z-coder-tester-ehbh.md`,
`20260722T102346Z-coder-tester-j8qs.md`) yet different `kind`/`to` —
meaning the 4-character random suffix collided in practice, not just in
theory. This is new evidence beyond the first draft, which only found
collisions in the legacy numeric scheme. The other four
(`00148-coder-tester.md`, `00275-planner-architect.md`,
`00276-coder-tester.md`, `00325-governor-reviewer.md`) are legacy-format
exact-path collisions.

**6 records — `different-message-numeric-collision`.** Legacy
`NNNNN-role.md` paths where `main` holds a **different, already-published
record** under the same 5-digit sequence number but a different role
suffix:

| Legacy number | On `main` (published) | Branch-only (different message) |
|---|---|---|
| `00044` | `00044-coder-tester.md` | `00044-planner-architect.md` (3 distinct blobs across `plan/T3b-route-restore`, `plan/downstream-impact-rule`, `feat/T3-spec-alignment`/`plan/T3-spec-alignment`) |
| `00057` | `00057-contribution-auditor.md` | `00057-coder-tester.md` |
| `00307` | `00307-planner-architect.md` | `00307-coder-tester.md` |
| `00312` | `00312-planner-architect.md` | `00312-coder-tester.md` |

Combined with the exact-path-collision class, this is **12 records**
needing `duplicate-rename-required` (not the first draft's 4) —
distinct messages that cannot be preserved under their original
filename without silently overwriting or shadowing an already-published
record. They need a governor-approved renaming convention (e.g.
re-timestamp under the current ID scheme, with a note that the content
predates it) before any preservation, not a default consolidation pass.
This is concrete, first-party evidence for exactly the risk the
timestamp+random-suffix ID scheme was introduced to avoid — and, per the
`ehbh`/`j8qs` finding above, evidence that the current scheme is not
immune to it either.

**Legacy sequence gaps with zero trace on any scanned branch (4 slots,
not part of the 200-record inventory).** `main`'s legacy sequence
(00001–00361) has 51 gaps; 47 of them are accounted for by branch-only
records above. Four are not: **00063, 00161, 00163, 00252** — no file at
that path exists on `main` or on any of the 383 remote + 318 local-only
branches scanned (up from "377 remote only" at first draft — the local
scan did not surface these either). **Disposition: flag to the
governor to check GitLab's server-side reflog/event history or any
clone this scan cannot reach before treating as permanently
unrecoverable** — this plan cannot verify that possibility itself.

## Local-branch secondary recovery source (B-HR-5)

318 local branches have no remote-tracking counterpart. Scanning them
identically to the 383 remote branches surfaced **5 records found
exclusively via the local scan** — invisible to the original
remote-only method entirely:

| Path | Local-only branch | Note |
|---|---|---|
| `01KV6J4XAZQC1TXHPT0T8ZF3A0-governor-reviewer.md` | `backup/claude-d40caca` | ULID-format legacy id, not on any remote branch at all |
| `20260722T102341Z-coder-tester-ehbh.md` | `coord/v240-ad-implementation-claims` | One of the two same-id-different-message collisions above |
| `20260722T102346Z-coder-tester-j8qs.md` | `coord/v240-ad-implementation-claims` | The other same-id-different-message collision |
| `20260726T122234Z-build-engineer-mk8p.md` | `learning/tool-sync-drift-and-skill-fix` | The stale pre-merge draft example above |
| `00275-planner-architect.md` | `plan/release-pages-artifacts-policy` | Legacy exact-path collision |

Per the TODO's own requirement, local presence is explicitly **not**
shared visibility — none of these 5 were ever pushed to a shared remote,
so they carry no obligation another agent could have been expected to
see. They are reported here as an evidence source, not as an
obligation-bearing backlog.

## Obligation state and disposition methodology (B-HR-3, revised per B-HR-EVID-1/B-HR-OBL-2/B-HR-VAL-1)

Per record, obligation state and disposition are derived in this order.
**Collision classification takes priority** (a colliding record cannot
be preserved as-is regardless of anything else); **then proven
resolution**; **then `requires-ack: true`** (never short-circuited by a
mere citation or an open MR — per B-HR-OBL-2, visibility is not the
same as resolution); **then MR state; then validation; then citation
visibility; then moot**:

1. `same-message-edited-draft` → obligation **superseded** → disposition
   `discard-superseded-by-main`.
2. `different-message-*-collision` → obligation
   **unadjudicated-filename-conflict** → disposition
   `duplicate-rename-required`.
3. `kind: handoff` **and** cited by a `kind: completion` record in
   `citing_main_records` → obligation **resolved-by-completion** →
   disposition `preserve-via-consolidation` (historical, resolution
   evidenced — the specific completing record is named in
   `citing_main_records`).
4. `requires-ack: true` and not already resolved above → obligation
   **unresolved-live** (or **unresolved-live-but-visible** if also
   cited by some other record — a citation alone does not prove
   acknowledgement or completion, per B-HR-OBL-2) → disposition
   `preserve-via-consolidation`, flagged as the highest-priority
   preservation candidate regardless of citation or MR state.
5. Observed state `mr-open` (and not already covered above) →
   obligation **live** → disposition `defer`.
6. `agov coord validate` fails on the extracted content (and not
   already covered above) → obligation **invalid** → disposition
   `invalid-with-validation-evidence`, with the actual validator error
   recorded.
7. Cited by `id` or filename in any current `main` record's
   `references-to` (and not already covered above) → obligation
   **cited-elsewhere** → disposition `preserve-via-consolidation`,
   explicitly labelled *visibility only* — the citing record(s) are
   named in `citing_main_records`, but a citation is evidence of
   downstream visibility, not proof the obligation was formally
   acknowledged or resolved.
8. Otherwise → obligation **moot** → disposition
   `preserve-via-consolidation` (historical record, no citation, no
   live obligation detected).

**B-HR-EVID-1 fix:** every `cited-elsewhere` and `resolved-by-completion`
record's `citing_main_records` field in the YAML names the actual
citing record(s) — path, id, kind, timestamp — not an assertion.

Resulting distribution: **134 preserve-via-consolidation** (86
`unresolved-live`/`unresolved-live-but-visible` — highest priority; 16
`cited-elsewhere`, visibility only; 2 `resolved-by-completion`; 30
`moot`), **22 defer**, **29 invalid-with-validation-evidence**, **12
duplicate-rename-required**, **3 discard-superseded-by-main**. (Round 1
reported 124/32/29/12/3 — the `requires-ack`-before-citation and
`requires-ack`-before-mr-open reordering per B-HR-OBL-2 moved 10 records
from `defer`/`cited-elsewhere` into the higher-priority
`unresolved-live` bucket.)

**B-HR-VAL-1 fix — validation and disposition are distinct, cross-checked
numbers, never conflated again:** **67 of 200 records fail
`agov coord validate` today** (the true total — round 1's "29" was only
the disposition-bucket subset that reached step 6 above without being
claimed by a higher-priority rule first). Of those 67: 29 are disposed
`invalid-with-validation-evidence`; 27 are disposed
`preserve-via-consolidation` (mostly `unresolved-live`, since a live
unacknowledged obligation still needs preservation even if it also
needs reformatting); 10 are disposed `duplicate-rename-required`; 1 is
disposed `discard-superseded-by-main` (moot, since it's being discarded
regardless). Each record's `validation_disposition_note` field states
this explicitly per record. Legacy-format records (pre-dating the `id:`
field requirement, using `seq:` instead) genuinely fail the current
schema's `Required property id was not present` check — this is real
evidence, not inference. Beyond the 29 already disposed `invalid-with-validation-evidence`, **37
more records** (27 `preserve-via-consolidation` + 10
`duplicate-rename-required`) also fail validation and would need
reformatting before actual publication — the discarded record's
validation status doesn't matter since it isn't being preserved. A
disposition label alone does not surface this; `validation_disposition_note`
is how a reader finds these per record.

## Operational cost assessment (B-HR-6, revised per B-HR-COST-2)

Bounded to what is mechanically countable; categories with no
recoverable evidence are labelled unknown/not-recoverable rather than
estimated, per the finding's own required framing. **Labels below match
exactly what the evidence proves, corrected per B-HR-COST-2 — no
category asserts more than that.**

- **CI pipelines correlated with closed-not-merged MR branches carrying
  branch-only coordination content: 54**, across 38 distinct closed MRs
  (queried via `GET /merge_requests/:iid/pipelines` for each).
  **Correction (B-HR-COST-2): this is correlated pipeline exposure, not
  proven waste.** These pipelines belonged to MRs that in most cases
  also carried substantive code changes; they may have tested those
  changes legitimately, informed the decision to close the MR, or both.
  No job-level evidence isolates a cost specifically attributable to
  the hidden coordination content — round 1's "wasted CI compute"
  framing overstated what this number proves.
- **Obligation breakdown within the 134 `preserve-via-consolidation`
  records** (replaces round 1's single "124 invisible obligations"
  figure, which conflated three different evidence strengths):
  - 86 `unresolved-live`/`unresolved-live-but-visible` — a genuine live
    obligation regardless of citation.
  - 16 `cited-elsewhere` — visible via a later main record's citation,
    but not proven acknowledged or resolved.
  - 2 `resolved-by-completion` — an actual completing record found.
  - 30 `moot` — no citation, no `requires-ack`, historical record only.
  This is a count of records by evidence strength, not a time or cost
  estimate; converting it to person-hours or review cycles would
  require session-log or human-recollection evidence this pass has no
  access to.
- **Unadjudicated filename conflicts: 12** — the `duplicate-rename-required`
  count. **Correction (B-HR-COST-2, renamed from "decisions made
  without an expected visible record"):** a filename collision proves a
  conflict existed and which content ended up on `main` by merge order
  — it does not prove anyone made an active adjudicating decision. No
  decision record was found for any of these 12; that absence is
  evidence of a gap, not evidence that a decision was made and simply
  left unrecorded.
- **Governor reconstruction time: unknown/not-recoverable.** No
  session-log or time-tracking artifact exists to measure how much time
  was spent reconstructing context that a branch-only record would have
  supplied directly.
- **Repeated review incidents: unknown/not-recoverable.** Would require
  correlating specific Gate 1/Gate 2 review rounds against missing
  prior context; no such correlation is recoverable from git/GitLab
  alone.
- **Misleading `agov` output instances: unknown/not-recoverable.** Would
  require historical `agov coord pending`/`agov coord list` output
  captured at the time, which was not retained.

## Proposed dispositions (recommendations, not decisions)

Per-record disposition is recorded in the YAML inventory; grouped here
by class since the reasoning is documented in "Obligation state and
disposition methodology" above:

- **134 records → `preserve-via-consolidation`.** Rationale: a closed
  MR, an unmerged branch, or an unresolved obligation does not
  retroactively make the coordination record's *existence* untrue — the
  record documents that an action, claim, or review happened at a point
  in time, which remains true regardless of whether the associated code
  change ultimately merged. This mirrors MR !994's own precedent
  exactly: preserve unique records unchanged via one fresh-main,
  content-hash-manifested consolidation MR, validate the range, merge,
  **then** close source branches — never delete first. **27 of these
  134 additionally require reformatting to pass current schema
  validation before preservation** (see "Obligation state and
  disposition methodology" — corrected from round 1's "29 of 124,"
  which conflated a different bucket entirely, per B-HR-VAL-1).
- **22 records → `defer`** (the still-live-and-uncited `mr-open`
  subset, after `requires-ack` records are pulled into
  `preserve-via-consolidation` first per B-HR-OBL-2). These are live,
  not archival. Bulk-consolidating a coordination record whose MR might
  still merge normally — or might still get closed, at which point it
  joins the closed-MR class — would pre-empt an in-flight decision. Per
  the v2.5.0 Statement of Work's own portfolio-disposition rule ("No
  blanket retrigger-and-sequential-merge ... prefer one fresh-main
  archive for *completed* historical records"), these stay out of any
  bulk pass and get resolved individually as their MRs resolve.
- **12 records → `duplicate-rename-required`** (the collision cases
  above: 6 exact-path, 6 numeric; 10 of these 12 also fail validation
  and would need reformatting in addition to renaming).
- **3 records → `discard-superseded-by-main`** (same-message
  pre-merge-edited drafts; `main`'s published version is authoritative).
- **29 records → `invalid-with-validation-evidence`** (the subset of
  the 67 total validation failures that weren't already claimed by a
  collision, resolution, `requires-ack`, or open-MR rule first — see
  "Obligation state and disposition methodology" for the full 67-record
  breakdown; this bucket and the 134 preserve-via-consolidation total
  are mutually exclusive, not overlapping, per B-HR-VAL-1).

**No disposition in this plan is final.** Per the handoff's explicit
boundary and `.claude/rules/always/coordination-sequence-guard-rule.md`,
only the governor-reviewer issues the binding disposition for each
record or class.

## What this forensic pass deliberately does NOT do

- Does not merge, close, relabel, or delete any branch or MR.
- Does not publish any historical coordination record.
- Does not treat a merged implementation, a bridge note, or a closed
  MR's age as proof that a record's obligation is resolved — disposition
  requires evidence (citation, validation, collision classification),
  not inference.
- Does not resolve the 4 untraceable legacy-sequence gaps — flagged to
  the governor, not silently written off.
- Does not attempt to reconstruct governor time cost, repeated-review
  incidents, or misleading tooling-output instances — explicitly
  labelled unknown/not-recoverable rather than guessed at.

## Prevention design

Addressing each requirement named in `docs/TODO.md`'s "Historical
Branch-Only Coordination Recovery and Prevention" section and the
`tenh` handoff, without granting tooling binding gate authority it does
not have. Rule citations below use the project's own adopted copy
(`.claude/rules/`), not the generic distributable template
(`agf/rules/`) — per contribution-auditor advisory 2, an implementation
that cites the template path would find unfilled `CUSTOMIZE`
placeholders instead of this project's actual governing text.

1. **Two separate mechanically checkable invariants, not one conflated
   check (contribution-auditor advisory 1).** The first draft's single
   "cannot be treated as disposed until the blob is verified present"
   framing conflated a pre-merge gate with a post-merge audit — a
   post-merge check cannot retroactively block an already-completed
   merge, so the two need different enforcement points:
   - **Pre-close/pre-merge invariant:** before an MR carrying a
     coordination record is merged, `agov coord publication-check`
     (already used for the `coordination:`-prefix fast path) verifies
     the record's blob will land on the target tip. This blocks the
     merge itself.
   - **Post-merge audit:** a separate, periodic check (see item 3, 6
     below) scans for coordination blobs that were on a since-merged
     branch's history but did not survive into the merge commit's tree
     (the exact `post-merge-branch-reuse` shape this scan found 14
     instances of) — or that a source branch was pushed to *after* its
     MR merged. This cannot block the merge (it already happened); it
     can only surface the gap for governor disposition, the same way
     this plan's own forensic scan did.
2. **Visible publication failures, not silent ones.** The evidence in
   this scan (72 branches with no MR at all; 59 with an MR that closed
   without merging) shows failure is currently silent by default — a
   bridge note or chat summary can claim "published" with nothing
   mechanically checking it. Candidate: a required field or check
   correlating a coordination record's `id` with an actual `main`-branch
   commit before any agent or role treats the record as durably
   published in session-visible tooling output.
3. **Extend `agov` remote-visibility scanning.** The scan this plan ran
   by hand (`git fetch --prune` + `git ls-tree` per remote AND
   local-only branch + GitLab MR join + frontmatter/collision/validation
   evidence extraction) should become a supported
   `agov coord scan-branches` (or equivalent) command: deterministic,
   scans local-only branches as a labelled secondary source by default
   (per B-HR-5, not an afterthought), continues past legacy
   sequence-collision diagnostics instead of erroring out (this scan
   found 12 real collisions across two distinct mechanisms and had to
   reason around them manually), and reports newly-introduced
   branch-only records separately from already-acknowledged historical
   debt (this document, once approved, becomes that acknowledged
   baseline).
4. **CI/repository check for MR-carried-but-target-absent coordination
   records**, with the negative-test coverage
   `.claude/rules/always/gate-self-test-rule.md` requires before any
   such gate ships: a coordination-only MR (should pass once merged), an
   implementation MR carrying a handoff (should pass), a superseded
   record (should pass with the superseding record named), a
   byte-identical consolidation (should pass), **the exact
   same-message-edited-draft shape found in this scan** (should pass —
   the published version supersedes the draft, this is not a failure),
   **the exact different-message collision shapes found in this scan,
   both exact-path and numeric** (should fail closed, not silently pick
   one), a deleted source branch post-merge (should not be treated as
   evidence of resolution by itself), and a skipped pipeline (should
   fail closed per the existing `allow_merge_on_skipped_pipeline: false`
   project setting).
5. **Bridge notes must name the record, exact source commit, branch/MR,
   and intended publication path.** This scan's own reconstruction
   relied entirely on git/GitLab evidence, not session memory or bridge
   notes, precisely because most of the 200 records had none — the
   requirement is that future bridge notes make this reconstruction
   unnecessary, not merely possible.
6. **One owner and cursor for recurring remote AND local visibility
   polling.** Candidate: fold into the existing coder-tester
   poll-and-coordinate cycle
   (`agf/docs/site/docs/operate/poll-and-coordinate.md`) as an
   additional step run on a longer cadence (e.g. weekly, not every poll
   cycle — the full 383-remote + 318-local-branch scan took real
   wall-clock time and API budget), with a bounded escalation age (e.g.
   14 days branch-only before an agent must file a note, not silently
   let it accumulate). Per B-HR-5, this owner's scan must include local
   branches, not just remote-tracking refs.
7. **MR-close and worktree/branch-cleanup guidance checks for
   unpublished coordination before declaring work complete.**
   `.claude/rules/workflow/branch-and-worktree-rule.md`'s existing
   cleanup section (`git worktree remove`, `git branch -d`) should gain
   an explicit pre-check: no coordination record introduced on this
   branch may be absent from the target branch at cleanup time. This is
   a process amendment to that rule, not a new rule — flagged here, not
   implemented here.

## Acceptance criteria

1. Governor-reviewer reviews the inventory
   (`docs/reviews/historical-branch-only-coordination-inventory.yaml`)
   and either accepts, amends, or requests re-scoping of the proposed
   per-record dispositions.
2. contribution-auditor independently re-verifies this revision's
   corrected method (a fresh `git fetch --prune` + local-branch
   enumeration, and a spot-check of at least the 3
   same-message-edited-draft, 6 exact-path-collision, and 6
   numeric-collision records, since those carry the most consequential
   and most-revised dispositions).
3. A follow-on implementation plan (separate Gate 1) is written for
   whichever `preserve-via-consolidation` records the governor approves,
   following MR !994's exact precedent: one fresh-main,
   content-hash-manifested archive MR, validated, merged, **then**
   source branches closed as merged-by-content — never deleted first.
   That plan must account for all 66 records requiring reformatting to
   pass current schema validation before preservation or rename (29
   `invalid-with-validation-evidence` + 27 `preserve-via-consolidation`
   + 10 `duplicate-rename-required` that also fail validation; see
   `validation_disposition_note` per record).
4. A separate follow-on implementation plan (or plans) covers the
   prevention-design items above, each with its own Gate 1 review,
   negative-test evidence per
   `.claude/rules/always/gate-self-test-rule.md`, and no binding gate
   authority granted to tooling beyond what is explicitly approved.

## Stop conditions

- Do not bulk-merge, publish, or delete any branch referenced in this
  plan or its inventory before governor-reviewer disposition.
- Do not treat the `defer` (open-MR) class as resolved by this plan —
  each resolves individually with its own MR.
- Do not treat the 4 untraceable legacy-sequence gaps as closed without
  the governor confirming no local-only or server-side copy exists.
- Do not preserve any of the 12 `duplicate-rename-required` records
  under their original filename.
- Do not preserve or rename any of the 66 records with
  `validation_disposition_note` indicating a validation failure
  (29 `invalid-with-validation-evidence` + 27 `preserve-via-consolidation`
  + 10 `duplicate-rename-required`) without first reformatting to pass
  current schema validation.
- Do not treat a `cited-elsewhere` classification as proof of
  acknowledgement or resolution — it is visibility evidence only, per
  B-HR-OBL-2.
- Do not treat any record's `causal_root_cause` as anything other than
  `undetermined` — this plan did not collect discussion, close-reason,
  or bridge-note evidence, only observed MR/repository state.

## Related documents

- `docs/coordination/20260803T073923Z-governor-reviewer-tenh.md` — the
  triggering handoff.
- `docs/coordination/20260803T084008Z-planner-architect-6m6k.md`
  (MR !1007) — this role's claim of the handoff.
- `docs/coordination/20260803T093704Z-contribution-auditor-1ajh.md` —
  the round 1 review the first revision responded to.
- `docs/coordination/20260803T114233Z-contribution-auditor-hpaq.md` —
  the round 2 review this revision responds to.
- `docs/reviews/historical-branch-only-coordination-inventory.yaml` —
  the full 200-record machine-readable inventory this plan summarizes.
- `docs/TODO.md`'s "Historical Branch-Only Coordination Recovery and
  Prevention" section (MR !1002) — the scope this plan responds to.
- MR !994 and its consolidation precedent — the preserve-then-close
  pattern this plan's proposed dispositions follow.
- `.claude/rules/workflow/coordination-publication-rule.md`,
  `.claude/rules/always/coordination-message-rule.md`,
  `.claude/rules/always/coordination-sequence-guard-rule.md`,
  `.claude/rules/workflow/branch-and-worktree-rule.md`,
  `.claude/rules/always/gate-self-test-rule.md` — governing rules for
  the prevention design above (paths corrected per advisory 2; these
  are this project's own adopted copies, not the generic
  `agf/rules/` distributable template).
