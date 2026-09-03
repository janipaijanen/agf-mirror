---
id: 20260903T043752Z-queue-convergence-and-roadmap-authority
status: pending-gate1
topic: queue-convergence-and-roadmap-authority
candidate_release: v2.10.0
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      The plan creates one internal planning document, refreshes two existing
      disposition ledgers, and corrects a stale adopter-facing status
      projection. No credential, secret, personal-data, or cross-record
      content is read, written, or transported; no runtime, dependency,
      artifact, schema, validator, or CI surface changes. Any item this plan
      routes into later implementation returns for its own consultation.
  - role: contribution-auditor
    status: waived
    reason: >-
      No contribution-auditor disposition exists for this plan, and the
      governed enum has no pending state. Recorded as waived rather than
      consulted so the metadata does not assert an audit that has not
      happened. This is the exact defect Codex raised as VRW-7 against
      MR !2269, corrected here rather than repeated. An independent
      coder-tester review is requested instead; the governor may hold Gate 1
      for an auditor pass, and the WIP-limit section is the part most worth
      auditing because it is the part most capable of becoming a loophole.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Queue convergence and roadmap authority

Prepared as **planner-architect** under the governor-directed handoff
`20260903T040339Z-coder-tester-tk2n`, which asks for three things: a bounded
Gate 1 plan that does not compete with the active v2.9.0 shipment; a
disposition for MR !2269 so roadmap work has one current authority and a
candidate release; and evidence-based terminal dispositions for the open
queue, with a short active queue and explicit WIP limits.

## Ordering constraint, stated first

`tk2n` is explicit that v2.9.0 remains the active shipment and its CI-rule
correction completes first. **Nothing in this plan may be implemented until
the A1 CI-rule correction has its own Gate 1 decision and v2.9.0's release
path is unblocked.** This plan's candidate release is therefore **v2.10.0**,
not v2.9.0. Attaching it to v2.9.0 would recreate the competing-authority
defect this plan exists to remove.

## Measured queue state

Observed 2026-09-03T04:37:52Z against `main` at `013c4a04`:

| Surface | Count | Note |
|---|---|---|
| Open merge requests | 50 | 2 drafts; 28 aged 30-60 days, 16 aged 7-30 days, 5 under 7 days |
| Open issues | 54 | 47 carry `release::unmilestoned-backlog` |
| Tracked TODO items in `docs/TODO.md` | 9 | Each carries its own `Status:`/owner line |
| Plan files in `docs/plans/` | 189 | 10 carry a status outside the schema enum; routed separately in `emtb` |
| Remote branches | 608 | Not in `tk2n`'s scope; recorded because it is the same debt in another surface |

The two existing ledgers under `docs/portfolio/` were observed 2026-07-31
against a v2.4.8 baseline, when 93 MRs were open. The prior disposition pass
therefore worked — 93 to 50 — and then the ledgers went stale and the queue
began regrowing. **That is the finding this plan is built on: the project
does not lack a disposition scheme, it lacks a re-observation cadence and an
intake bound.**

## What this plan is, and is not

It is: one governed roadmap owner, a disposition for !2269, a refresh of two
existing ledgers against a current baseline, and an intake bound.

It is not: a release work order, a version bump, a milestone or issue-state
mutation, a schema change, a validator change, a CI change, or a new AGF
adopter-facing rule. It creates no new artifact type where an existing one
serves — the disposition ledgers already exist and are extended, not
replaced.

---

## Task 1 — one roadmap authority

Create `docs/roadmap.md` as the single governed internal owner.
`agf/docs/site/roadmap.md` becomes a checked projection of it, and GitLab
milestones remain a derived mechanical projection that is never a second
source of truth. This model is inherited unchanged from the binding direction
in `nsne` and from !2269, which got this part right.

The adopter-facing projection is currently wrong in a way an evaluator can
see: its status row reads "v2.8.0 standard release candidate in construction
from `main`" and is stamped `Observed at 2026-08-28`. v2.8.0 is published and
v2.9.0 is in flight. Correcting that projection is part of this task, and it
is a factual status correction, not a new commitment.

`release-v2.7.0-rc.2` is recorded in the owner as **`awaiting-governor`**.
`m2pr` abandoned rc.1 and deliberately declined to infer rc.2's fate; no
binding disposition has been filed since. This plan does not resolve it and
is not blocked by it — that is the concrete difference from !2269, which
declared its own scope incomplete pending the same undecided input.

### Deliberately dropped from !2269: the bounded maintenance path

!2269 proposed a reduced-ceremony route for routine `docs/roadmap.md` edits,
modelled on the sources/radar research fast path. This plan **drops it
entirely.**

Codex's VRW-2 and VRW-3 are right and are not cheaply answerable. A route
that changes when full Gate 1 is mandatory is a gate change regardless of
whether a validator enforces it, which pulls in `gate-self-test-rule.md`'s
negative-test obligation and a named normative owner. VRW-3 additionally
shows the route is internally inconsistent as written: it permits status
updates to the owner while disqualifying projection changes, so following it
produces exactly the owner/projection drift the artifact model forbids.

The honest measurement is that the route buys very little. A roadmap typo
correction under full Gate 1 costs one short plan; the route costs a rule, a
negative test, an eligibility procedure, and a permanent widening risk. Under
`minimum-sufficient-value-rule` that trade does not clear. If routine roadmap
maintenance later proves genuinely burdensome, it returns as its own Gate 1
plan with its own gate-self-test, on measured evidence rather than
anticipation.

---

## Task 2 — MR !2269 disposition: supersede

!2269 is superseded by this plan, and closed with a pointer to it.

Codex's VRW-1 offered two honest resolutions and preferred the smaller one:
reframe !2269 as roadmap-owner/projection/closeout planning only. That advice
predates `tk2n`. `tk2n` adds queue convergence, terminal dispositions and WIP
limits, which !2269 has no frame for, so a reframe would leave the governor's
direction unaddressed and would still need a second plan — two authorities
again. Superseding produces one.

The supersession carries !2269's sound core forward: the owner/projection
model, the sources-radar-issue-roadmap-release distinction, the refusal to
close #36 wholesale, and the ambiguity-fails-closed posture. It drops the
maintenance path (above), the release-work-order framing (VRW-1), and the
unsupported `contribution-auditor: consulted` metadata (VRW-7, corrected in
this plan's own front matter).

Codex's VRW-4 finding is carried into Task 3 rather than answered here: the
`--help`-only evidence for #32 and the crate-exists evidence for #37 do not
prove those issues' acceptance criteria, and #36 cannot be half-closed
without a named residual carrier. Those are exactly the evidence standards
Task 3's vocabulary makes mandatory.

Note on mechanics: the plan schema's `status` enum has no terminal
`superseded` value, so !2269's plan file cannot record its own supersession
in front matter. The supersession is therefore recorded in coordination and
in the closing MR note. This is the schema gap already routed to Codex in
`emtb`; it is not worked around here.

---

## Task 3 — terminal dispositions, refreshed ledgers, and an intake bound

### 3a. Extend the existing ledgers, do not replace them

`docs/portfolio/post-v248-issue-disposition.md` and
`post-v248-mr-disposition.md` already carry the right shape: one row per
item, a destination, and a closure condition. They are re-observed against a
current baseline and renamed to reflect it, preserving every existing row's
history rather than starting a fresh file.

### 3b. Closed terminal-disposition vocabulary

Every row resolves to exactly one of these. Each requires named evidence; a
disposition without evidence is not a disposition.

| Disposition | Meaning | Required evidence |
|---|---|---|
| `ship-in <release>` | Committed to a named candidate release | Owner role, acceptance criteria, and the child Gate 1 artifact or a statement that one is required |
| `merge-now` | MR content is current and reviewable as-is | Exact-head verification against current `main`, passing pipeline, no unresolved discussions |
| `reconstruct` | Content is unique and wanted; the delivery vehicle is stale | What is unique, and the fresh branch that will carry it |
| `close-superseded` | Content landed or was replaced elsewhere | The canonical artifact, named by path or MR |
| `close-invalid` | The premise no longer holds | The specific evidence that falsifies it |
| `park-radar` | Non-committal watch item, no release commitment | A `review-by` date and the trigger that would promote it |
| `awaiting-governor` | Blocked solely on a decision only the governor can make | The exact question, and why no other role can answer it |

Two boundaries are load-bearing. **Age is not evidence** — the corrected
finding in `docs/reviews/backlog-prioritization/20260813-final-proposal.md`
established both that nothing closes by age alone and that this does not mean
nothing closes. **Absence of a row is not a disposition** — an item not yet
examined is recorded as unexamined, never silently omitted, so the ledger
cannot imply a judgment nobody made.

`awaiting-governor` exists because the queue currently cannot distinguish
"blocked on the governor" from "nobody has looked at it." `release-v2.7.0-rc.2`
is the worked example.

### 3c. Intake bound (WIP limits)

The queue regrew because nothing bounded intake. Proposed as a **foundry
working agreement recorded in `docs/roadmap.md`**, not as an AGF rule and not
as a mechanical gate:

- Open non-coordination implementation MRs: **at most 5**
- Open Gate 1 plan MRs awaiting a decision: **at most 3**
- Coordination-only MRs: **not counted.** They are designed to publish fast
  and independently under `coordination-publication-rule.md`; bounding them
  would fight that rule and delay the audit trail.

While a limit is exceeded, no new work of that class opens, except:
release-critical work on the active shipment, a correction to an already-open
item, or an explicit governor override recorded in coordination.

This is deliberately advisory. Making it blocking would be a gate change
requiring a named normative owner and a negative test under
`gate-self-test-rule.md`, and there is no evidence yet that an advisory bound
is insufficient. If it proves insufficient, promoting it is its own Gate 1
plan with its own self-test — the same discipline applied to the maintenance
path in Task 1.

### 3d. Batching

Roughly 133 items are in scope across MRs, issues, TODO items, and roadmap
entries. That is not one merge request. Work proceeds in batches sized so a
reviewer can answer one yes/no question per batch, per
`branch-and-worktree-rule.md`:

| Batch | Scope | Reviewer's question |
|---|---|---|
| B1 | `docs/roadmap.md` owner, projection correction, !2269 supersession | Is this the right single roadmap authority? |
| B2 | The 9 `docs/TODO.md` items | Is each TODO's terminal disposition correct? |
| B3 | Issue ledger refresh, 54 issues | Is each issue's disposition evidence-backed? |
| B4-B7 | MR ledger refresh, 50 MRs in cohorts by class (learning records, radar/sources, plan MRs, docs/governance) | Is each MR's disposition evidence-backed? |
| B8 | Intake bound recorded in the owner | Are these the right limits? |

Batches are independently reviewable and independently revertible. A batch
that grows past its question is split rather than argued through.

---

## Rule-projection screen

Run rather than reasoned away. `docs/roadmap.md` is internal, and the
adopter-facing edit is a factual status correction rather than normative
guidance, so several rows dispose as not-applicable — but the screen is
executed and its reasons recorded, because narrowing a screening obligation
by argument is the shape Codex caught in VRW-2.

Manifest at `docs/reviews/rule-projection/queue-convergence-and-roadmap-authority.yaml`,
disposing all six rows. Strict `agov plan projection-check` output, without
`--allow-planned`, is required Gate 2 evidence. No `stage: planned` marker is
used: this plan creates no normative owner file.

## Test and validation strategy

1. `agov plan validate` on this plan — passing at submission.
2. `agov plan projection-check`, strict, before each batch's Gate 2.
3. **Projection consistency check:** every commitment stated in
   `agf/docs/site/roadmap.md` must exist in `docs/roadmap.md`. A projection
   asserting more than its owner is a blocking finding. Mechanical where
   possible; reviewer-checked where not, and stated as such rather than
   implied to be automated.
4. **Disposition evidence check:** every ledger row names its evidence, and
   every `close-*` row names the artifact or falsifying fact. A row whose
   evidence field is empty fails review.
5. **Vocabulary closure check:** no row carries a disposition outside the
   Task 3b table.
6. No gate, validator, CI, schema, or merge-policy file is touched by any
   batch. A batch that needs to touch one stops and returns to Gate 1 as a
   gate change.

## Open governor inputs

1. **`release-v2.7.0-rc.2` disposition.** Undecided since `m2pr`
   (2026-08-30). Recorded as `awaiting-governor`; this plan proceeds without
   it.
2. **Is the intake bound the right shape and the right numbers?** 5 and 3 are
   proposed from the observed regrowth, not derived from a measurement of
   sustainable throughput. The governor may set different numbers.
3. **Should the 608 remote branches be scoped in?** `tk2n` did not name them.
   MR !2038 already proposes a stale-branch pruning rule. Recorded as a
   pointer, not folded in.

## Risks

- **The disposition pass goes stale again.** This is the observed failure
  mode, not a hypothetical. Mitigation is the intake bound plus a re-observe
  trigger tied to each release rather than to a calendar.
- **The vocabulary is used to close things cheaply.** Mitigation: every
  `close-*` disposition names an artifact or a falsifying fact, and
  `close-invalid` requires the specific evidence rather than a judgment.
- **B4-B7 fatigue produces thin dispositions late in the sequence.**
  Mitigation: cohorts are by class, so a reviewer sees like with like, and any
  row may be recorded as unexamined rather than given a weak disposition.
- **The intake bound is ignored because it is advisory.** Accepted openly. The
  alternative — making it blocking now — is a gate change on no evidence, and
  this project has a documented cost for shipping gates that were not needed
  in the shape they were built.

## Rollback

Delete `docs/roadmap.md`, revert the projection edit, and revert the ledger
refreshes to their 2026-07-31 content. No schema, validator, CI, milestone,
issue state, branch, or tag is mutated by this plan, so rollback is a file
revert with no external state to reconcile.

## Evidence

- `docs/coordination/20260903T040339Z-coder-tester-tk2n.md` — governor-directed handoff
- `docs/coordination/20260831T041020Z-coder-tester-1mqg.md` — Codex's seven blocking findings on !2269
- `docs/coordination/20260827T203743Z-governor-reviewer-nsne.md` — binding owner/projection direction
- `docs/coordination/20260830T214141Z-governor-reviewer-m2pr.md` — rc.1 abandoned, rc.2 left undecided
- `docs/plans/20260828T194721Z-v290-release-integrity-and-tracker-truth.md` — approved A1/A2 v2.9.0 scope this plan does not alter
- `docs/plans/20260831T050110Z-v290-release-publication.md` — active v2.9.0 shipment
- `docs/portfolio/post-v248-issue-disposition.md`, `post-v248-mr-disposition.md` — the ledgers extended rather than replaced
- `docs/reviews/backlog-prioritization/20260813-final-proposal.md` — the corrected "never close by age alone, but some things do close" finding
- Queue counts reproduced above from `glab api` and `git ls-remote` at `013c4a04`
