<!-- Radar carrier. Retained verbatim from MR !1718 (source branch docs/learning-dd-operational-debt-backlog, head a5e690e2), file docs/plans/20260817T210510Z-operational-debt-phase0-measurement.md. Its front matter records an approved Phase-0 plan; only Snapshot 1 was taken, Snapshot 2 never was, and nothing here is a live approval.
     Non-authoritative: parked on the radar, grants no Gate 1, rule, template, validator or implementation authority. -->
---
id: operational-debt-phase0-measurement
status: approved
topic: operational-debt-phase0-measurement
gate1_approved_by: 20260817T224145Z-governor-reviewer-qt32
gate1_approved_at: 2026-08-17T22:41:45Z
advisory_consultation:
  - role: coder-tester
    status: consulted
    reason: >-
      Codex/coder-tester reviewed the original proposal (37mj: BLOCK on
      2 findings, joint consensus for a Phase 0 baseline first) and the
      first plan draft (6keg: BLOCK on 3 findings — ODP0-R1 collapsed
      Phase 0 and Phase 2, ODP0-R2 dropped drift classification,
      ODP0-R3 substituted an unagreed 10+10 sample for the required
      two-snapshot baseline). This revision applies all three
      corrections; re-review requested at this head.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Codex/contribution-auditor independently audited this plan across
      three rounds. Round 2 (20260817T213354Z-contribution-auditor-h4bb):
      CLEAR on the OD-1/OD-2 and ODP0-R1/R2/R3 corrections; BLOCK on two
      further findings (ODP0-R4: branch taxonomy dropped the orphaned/
      explicitly-retained distinction; ODP0-R5: this consultation entry
      itself was left `not-applicable` despite h4bb supplying the
      required review). Round 3 (20260817T214556Z-contribution-auditor-
      1m7p): ODP0-R5 confirmed resolved; BLOCK on one further finding
      (ODP0-R6: branch-state precedence placed `closed/merged-MR` ahead
      of `explicitly-retained`, making retention unreachable for the
      common case of a closed/merged branch with a later retention
      decision), plus an advisory on the Snapshot-1 arrivals/dispositions
      denominator. Round 4 (20260817T220129Z-contribution-auditor-pk4h):
      ODP0-R6 and the Snapshot-1 advisory confirmed resolved; BLOCK on one
      further finding (ODP0-R7: Snapshot-2 arrivals/dispositions/
      transitions cannot be computed from net aggregate-count deltas, since
      equal arrivals and dispositions produce the same count as no
      activity), plus two advisories (duplicate "Branch states" heading;
      the MR description was stale, still describing the `6keg`-round
      state). ODP0-R4 through R7 and every advisory are all applied at this
      head. Current authority (verified against origin/main's agf/agents/
      MANIFEST.yaml): contribution-auditor is played by human (Jani) +
      Claude (at plan review) + Codex, per binding governor decisions
      20260816T124942Z-governor-reviewer-pehh (path B2) and
      20260816T131724Z-governor-reviewer-6srq (cross-LLM/session-
      independence policy, superseding 20260816T130941Z-governor-
      reviewer-p4hk). Codex audited this plan (authored by Claude, not
      Codex) under that policy.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      This plan requests a read-only classification exercise over
      existing tool output only (agov coord status/pending, the
      existing agov.validate.assistant_drift validator, git/GitLab
      queries) — no new write path, credential, permission, or schema
      change. Re-evaluate if a later Phase 2 tooling plan or any
      enforcement-mechanism plan introduces write/deletion authority.
---

# Plan: operational-debt Phase 0 — measurement and triage baseline (DD R8/F7)

- Timestamp: 2026-08-17T21:05:10Z (OS clock, `date -u`); revised per
  `6keg` at 2026-08-17T21:35:00Z; revised again per round-2 audit
  `h4bb` at 2026-08-17T21:40:42Z; revised again per round-3 audit
  `1m7p` at 2026-08-17T21:54:14Z; revised again per round-4 audit
  `pk4h` at 2026-08-17T22:13:00Z (OS clock, `date -u`).
- Author: planner-architect (Claude)
- Supersedes in scope: the original recommendation in learning
  `20260817T202241Z-ookv` (four-mechanism policy bundle), corrected in
  the same entry to point here, per joint consensus with coder-tester
  (`37mj`), then corrected again per `6keg`'s three findings, then
  again per `h4bb`'s two round-2 findings (branch taxonomy, stale
  consultation status) on this plan's second draft.

## Decision requested

Authorize a **manual/scripted measurement exercise using only existing
read-only surfaces** — `agov coord status`, `agov coord pending`,
`agov validate` (specifically `agov.validate.assistant_drift`), and
`git`/GitLab API queries for branch state. No new `agov` tooling, no
enforcement, no branch deletion, no MR closure, no policy threshold.
Output is a snapshot artifact (a coordination record or plain report
file), captured at **two separate timestamped revisions**, classifying
every pending coordination obligation, every repository branch, and
every assistant-rule drift finding into the mutually-exclusive states
below.

Building new `agov` report tooling (`37mj`'s Phase 2) is explicitly
**deferred** to a later, separate Gate 1 plan, requested only if this
Phase 0 baseline shows a recurring computed report would remove more
review effort than a periodic manual snapshot costs. This plan does
not request, and does not implement, any new tooling.

## Why this plan doesn't ask for the four-mechanism bundle, or for new tooling

The original proposal (`ookv`/`7hqj`) asked to define a closure SLO,
bounded branch lifetime, drift-closure cadence, and dashboard directly
from the DD's raw counts. `37mj` found those counts unvalidated
(`agov coord status` reports demonstrably completed/superseded work as
pending; "547 branches" mixes states with no age/disposition
classification) and required a measurement baseline first, not
enforcement.

This plan's **first draft** then over-corrected the other direction:
it requested building the Phase 2 report *command* while calling
itself Phase 0, and labeling Phase 2 "deferred" in the same breath
(`6keg` finding ODP0-R1). This revision fixes that by scoping Phase 0
to what already exists — no new command, no new code, just a
disciplined manual query pass — so the baseline itself doesn't require
the tooling decision it's supposed to inform.

## Design (per `37mj`'s Phase 0 proposal, corrected per `6keg`)

### Obligation states (mutually exclusive)

- **live** — actionable now by its stated owner.
- **human-gated** — waiting on a human decision/gate; clock paused,
  with an explicit reason recorded.
- **completed-but-unacknowledged** — the underlying work is done
  (verifiable against `main` or a merged MR) but the coordination
  record itself is still open/pending in `agov coord status`.
- **superseded** — a later record explicitly supersedes this one.
- **stale-projection** — the tooling's own state no longer reflects
  reality (e.g., referenced branch/MR no longer exists or changed
  state without an update record).
- **historical** — preserved audit trail, not a live obligation by
  design (e.g., historical plan MRs kept open per an explicit governor
  boundary decision, same pattern as MR !1617).

### Branch states (mutually exclusive) — restored per `h4bb` ODP0-R4, precedence corrected per `1m7p` ODP0-R6

Assignment rule, applied in this fixed priority order so a branch with
multiple historical MRs (e.g., closed-then-reopened, or referenced by
more than one MR over time) still gets exactly one state — the first
matching rule wins. `explicitly-retained` is checked **before**
`closed/merged-MR` (moved up from position 4 to position 3 per `1m7p`
ODP0-R6): a branch whose most recent MR is closed or merged is the
*common* case where retention matters, not an edge case, so putting
`closed/merged-MR` first made retention evidence unreachable for
exactly the branches it exists to protect.

1. **protected** — matches a protected-branch pattern. Checked first;
   overrides everything else regardless of MR history.
2. **active-MR** — has at least one currently open MR.
3. **explicitly-retained** — no open MR, but a coordination record or
   governor decision names this exact branch as intentionally kept
   (e.g., a historical-record boundary decision like `ahh2`'s MR !1617
   treatment, or an evidence-preservation note per issue #110's
   "backlog-preserved" classification — this subsumes the first
   draft's "evidence-preserving" state under a clearer name). Checked
   ahead of `closed/merged-MR` so a retention decision always wins over
   the branch's own closed/merged MR history.
4. **closed/merged-MR** — has at least one MR, none currently open, no
   explicit-retention record found, and the most recent MR's outcome is
   merged or closed-with-disposition.
5. **orphaned** — no open MR, no matching MR history, and no
   explicit-retention record found. This is the state a later
   age-based review trigger (per `37mj`'s Phase 1 proposal) would
   actually need to act on — the population `evidence-preserving`/
   `other` alone couldn't isolate.
6. **other** — a branch that matches none of the above cleanly (e.g.,
   conflicting or ambiguous MR history); surfaced explicitly rather
   than silently mis-bucketed into `orphaned`, since `orphaned` is
   meant to be a confident negative ("checked, nothing found"), not a
   catch-all.

**Worked example (per `1m7p` ODP0-R6):** consider a hypothetical branch
whose sole MR merged, and which a later governor decision then names as
intentionally kept as audit evidence (the same pattern as `ahh2`'s MR
!1617 boundary decision). Under the old (pre-`1m7p`) order, rule 3
(`closed/merged-MR`) matched first and the branch was permanently
classified `closed/merged-MR`, so the retention decision could never be
reached — state 4 (`explicitly-retained`) was unreachable for exactly
this common case. Under the corrected order: rule 1 (protected) — no;
rule 2 (active-MR) — no, the MR is merged, not open; rule 3
(explicitly-retained) — yes, the governor decision names this exact
branch as kept, so classification stops here at `explicitly-retained`
and rule 4 (`closed/merged-MR`) is never reached. For a branch whose
merged/closed MR carries no such retention record, rule 3 does not
match and classification correctly falls through to rule 4,
`closed/merged-MR`.

### Drift states (mutually exclusive) — restored per `6keg` ODP0-R2

Sourced from the existing `agov.validate.assistant_drift` validator
output (currently reports 51 findings: 44 differ, 5 missing, 2 extra
against the copied `.claude/rules` tree), classified as:

- **actionable difference** — a real, unintentional drift from the
  canonical AGF rule tree that should be corrected.
- **expected project override** — an intentional, documented
  project-specific deviation (not a defect).
- **bootstrap-only** — drift confined to the frozen `agents-govern/`
  reference distribution, not project-authored content (same pattern
  as the existing `ISEC-CROSSRECORD-001` sentinel acknowledgement).
- **false positive** — the validator's comparison itself is wrong for
  this case (e.g., a legitimately renamed or restructured file it
  doesn't yet recognize as equivalent).

### Output shape

Per item: **stable identity key** (per `pk4h` ODP0-R7 — see below) + state
+ owner + blocker (if any) + source link (coordination record path,
MR/branch identifier, or drift-finding path) + age bucket. Per `37mj` and
`6keg`: **at least two snapshots**, each capturing counts, oldest age,
arrivals since the prior snapshot, dispositions since the prior snapshot,
and unresolved exceptions, for all three inventories (obligations,
branches, drift) — not a single point-in-time count. Trend storage beyond
two snapshots is deferred until these two demonstrate the mechanism is
worth persisting.

**Snapshot 1 denominator (per `1m7p` advisory):** Snapshot 1 has no
prior snapshot to diff against, so its "arrivals since the prior
snapshot" and "dispositions since the prior snapshot" fields are
recorded as **not applicable** — Snapshot 1 reports absolute counts and
ages only. "Arrivals"/"dispositions" become meaningful starting at
Snapshot 2.

**Item-level identity, not aggregate-count deltas (per `pk4h` ODP0-R7):**
computing Snapshot 2's arrivals/dispositions/transitions as the numeric
difference between Snapshot 1's and Snapshot 2's counts is invalid — five
arrivals and five dispositions produce the same net count as no activity
at all, so a count-only diff cannot distinguish a stable queue from a
churning one, undermining the measurement this plan exists to produce.
Every item in both snapshots MUST therefore carry a **stable identity
key**, and arrivals/dispositions/transitions are computed by **joining
Snapshot 1's and Snapshot 2's item sets on that key**, not by subtracting
totals:

- **Obligations** — identity key: the coordination record's own `id:`
  field (e.g. `20260817T210510Z-planner-architect-xxxx`), which is
  immutable once published (`immutable-tags-rule.md`'s sibling
  principle for coordination records — see
  `coordination-publication-rule.md`'s append-only requirement).
- **Branches** — identity key: the exact Git ref name (e.g.
  `docs/kungfu-sources-radar`). Ref names are stable identifiers for a
  given branch's lifetime; a rename is treated as a disposition of the
  old key plus an arrival of the new key, not a silent identity change.
- **Drift findings** — identity key: the compared file's path relative
  to the tree root (e.g. `.claude/rules/workflow/tdd-rule.md`) plus the
  validator's own finding type (`differ` / `missing` / `extra`), since
  `agov.validate.assistant_drift` reports at that granularity.

An **arrival** is a key present in Snapshot 2 but absent from Snapshot 1.
A **disposition** is a key present in Snapshot 1 but absent from
Snapshot 2 (its final Snapshot-1 state is recorded as the disposition
outcome). A **transition** is a key present in both snapshots whose state
differs between them. Aggregate counts for Snapshot 2's report are then
*derived* from this item-level join (count of arrivals, count of
dispositions, count of transitions by from-state/to-state pair) — never
computed as a standalone `count(snapshot2) - count(snapshot1)` shortcut,
which is exactly the invalid method `pk4h` flagged.

### Mechanism — existing surfaces only, no new tooling

Both snapshots are produced by running the existing commands above and
manually classifying their output against the state definitions —
scripted with a shell one-liner or a short throwaway script is fine;
committing new `agov` subcommand code is not part of this plan. Each
snapshot is preserved as its own timestamped artifact (coordination
record or committed report file) so the classification method and
raw output are auditable, not just the summary counts.

## Ownership (per `37mj`, subject to governor approval)

- **planner-architect** — classifies planning/coordination obligations
  into the states above.
- **coder-tester** — verifies technical/MR state and drift
  classification.
- **build-engineer** — not needed for Phase 0 (no tooling/CI
  integration in this plan's scope); owns Phase 2 if a later plan
  requests computed-report tooling.
- **governor-reviewer** — decides exceptions, any future destructive
  branch action, and any future enforcement-mechanism policy
  threshold. No role is made responsible for closing work it lacks
  authority to close.

## Success criteria

`37mj`'s criteria (for the eventual Phase 1/2 enforcement decision, not
this plan):

1. Every reported pending item has a current state, owner, source, and
   blocker.
2. Completed/superseded records no longer appear as live work, while
   remaining available in the audit trail.
3. Age and arrival/disposition rates are reproducible from repository/
   GitLab data at an exact revision.
4. No branch is deleted and no MR is closed automatically.
5. The mechanism demonstrably removes more recurring review effort
   than it adds; otherwise park it under `workflow/simplicity-rule.md`.

**This plan's own Phase 0 completion bar** (corrected per `6keg`
ODP0-R3): two timestamped, exact-revision snapshots across all three
inventories (obligations, branches, drift), each with counts, oldest
age, arrivals, dispositions, and unresolved exceptions since the prior
snapshot, spaced at least 24 hours apart. A 10-record + 10-branch
sample from the second snapshot is used as an *additional* accuracy
check (independently verified by coder-tester against live
GitLab/repository state), not as the completion criterion by itself.
Any later SLO/lifetime/cadence/dashboard decision is gated on this
two-snapshot evidence, not on this plan's approval alone.

## Files affected

- No code, schema, or migration change. No new `agov` command.
- Two snapshot artifacts (coordination records or committed report
  files under e.g. `docs/reviews/operational-debt-phase0/`), produced
  after this plan's Gate 1 approval — not part of this plan's own diff.
- `agf/learnings/foundry.yaml` — correction to entry
  `20260817T202241Z-ookv` (same MR as this plan), pointing here.

## Test and validation plan

1. `agov plan validate` on this file: must pass.
2. Snapshot 1 and Snapshot 2 (≥24h apart): each independently
   reproducible from `agov coord status`, `agov coord pending`,
   `agov validate`, and `git`/GitLab API state at its own exact
   revision.
3. Snapshot 2's arrivals/dispositions/transitions are computed by
   joining Snapshot 1's and Snapshot 2's item-level identity keys (per
   `pk4h` ODP0-R7), not by subtracting aggregate counts; the join
   itself is part of what the 10+10 accuracy spot-check verifies (next
   item).
4. 10+10 accuracy spot-check on snapshot 2, independently verified by
   coder-tester, including confirming the item-level join for at least
   one arrival and one disposition per inventory (not just the
   snapshot's absolute counts).
5. Confirm the exercise is read-only throughout: no `git push` beyond
   committing the snapshot artifacts themselves, no GitLab mutation API
   calls, no branch/MR state change.

## Downstream impact (per `downstream-impact-rule.md`)

Impact:
  Foundry:          Produces two read-only measurement snapshots; no
                     behavior change to any existing gate, validator,
                     or coordination mechanism, and no new tooling.
  Direct adopters:  None. Foundry-internal.
  Downstream:       None.
  Migration:        None required.
  Evidence:          MR !1691 (DD dossier, not yet merged); `37mj` and
                     `6keg` (joint consensus and corrections); this
                     plan.

## Risks

- **Risk: Phase 0 itself becomes another queue item that never
  closes.** Mitigated by the narrow, bounded two-snapshot completion
  bar above rather than open-ended scope.
- **Risk: classification heuristics are wrong in ways that hide real
  live obligations.** Mitigated by requiring independent coder-tester
  verification on the accuracy spot-check, and by the "other"/
  "false positive" buckets surfacing unclassifiable items explicitly.
- **Risk: two snapshots 24h apart isn't enough to establish a real
  rate.** Accepted for Phase 0 — this plan does not claim two snapshots
  prove a rate, only that they're the minimum needed to compute one
  arrival/disposition delta at all. A stronger baseline (more
  snapshots) is a Phase 1/2 decision, not blocked by this plan.

## Rollback

Read-only measurement exercise; nothing to roll back. No data, branch,
tooling, or MR state is ever mutated by this plan's scope.

## Stop conditions

Stop and return to the governor if: the exercise drifts from read-only
into any write/deletion/closure action, or into writing new `agov`
tooling code (either requires a separate Gate 1 decision); or if the
accuracy spot-check finds classification too unreliable to trust
(redesign the states/heuristics before proceeding).

## Requesting

- Coder-tester: already consulted (`37mj`, `6keg`); requesting
  re-check of this current head against ODP0-R4 through R7 and the
  Snapshot-1 denominator advisory.
- Contribution-auditor (Codex, under current `6srq` authority, or
  human): audit already complete through round 4 (`h4bb`, `1m7p`,
  `pk4h`); requesting confirmation the current head resolves ODP0-R7
  and both round-4 advisories, or correction if not.
- Governor-reviewer: Gate 1 decision on this narrow scope only.

`agov plan validate`: to be run before Gate 1 submission. `git diff
--check`: clean (new file only).
