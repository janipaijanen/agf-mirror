---
id: 20260916T172653Z-known-duty-self-check-discovery
status: pending-gate1
topic: known-duty-self-check-discovery
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      This plan scopes a discovery-only exercise: it classifies eleven
      existing learning records and specifies a candidate mechanical
      self-check design. It authorizes no code, no new credential, secret,
      or network access, and no write authority beyond the plan file and
      its own Gate 1 request. Any later implementation plan carries its
      own infosec consultation.
  - role: contribution-auditor
    status: pending
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Known-duty self-check: narrowed Gate 1 discovery plan

## Provenance

Governor decision `20260916T170239Z-governor-reviewer-qnjr` authorizes
planner-architect (Claude) to draft and submit this narrowed discovery plan
for Gate 1, per contribution-auditor review
`20260916T152152Z-contribution-auditor-g41r`'s recommended disposition on
proposal `20260916T150215Z-planner-architect-vh4t`. `qnjr` is explicit that it
authorizes plan drafting and submission only — not the plan's approval, any
implementation, a new command, blocking authority, role changes, or
deployment.

`vh4t` proposed a mechanical self-check at the coordination layer after
observing eleven `agf/learnings/foundry.yaml` records sharing one shape: an
agent already had the knowledge, draft, or authorization needed to complete
an obligation, and did not act on it until an external prompt forced the
action. `g41r` returned BLOCK on two findings in `vh4t`'s premise and
mechanism, and ADVISORY on a third, then recommended narrowing rather than
rejecting outright. This plan implements that narrowed disposition, addressing
each `g41r` finding directly:

- **G41R-F1** (the universal "rule text already existed" premise is false for
  at least one record): resolved by the per-record classification below,
  which replaces the single root-cause claim with a cluster classification.
- **G41R-F2** (the cited `-o ci.skip` structural fix remains agent-applied,
  not automatic enforcement, and has a live adoption gap): resolved by
  treating "automatic invocation point" as a first-class open design
  question in this plan, not an assumed solved precedent.
- **G41R-A1** (split the candidate mechanism into deterministic, typed-graph,
  and semantic slices): resolved by the signal taxonomy below.

## What this plan is not

This plan does not authorize:

- implementation of any tool, command, or CI job;
- any change to blocking authority for any check (per
  `tool-authority-boundary-rule.md`, that requires a named rule change, which
  is itself a future, separately gated decision);
- any change to agent roles, maturity, or activation state.

Its only output is a Gate 1-ready design for a follow-up implementation plan,
plus the false-positive/missed-obligation evidence-collection protocol that
must run and report back **before** any blocking-authority proposal is even
drafted.

## Part 1 — Classification of the eleven cited records

`g41r`'s resolution condition for F1 requires replacing the single
"rule text already existed" premise with a classification that separates
incidents by cause. Four causes are distinguished below: (A) rule text was
adequate and available at incident time but was not applied — an
execution/adoption gap; (B) rule text or standing authority was missing or
incomplete at incident time, and was subsequently added by a governor
decision or new rule; (C) no step in the existing process owned a specific
recurring follow-through duty; (D) another evidenced cause not covered by A–C.

| Record | Title (short) | Cause | Basis |
|---|---|---|---|
| `20260907T073424Z-rl9k` | Busy peer misread as unreachable | B — missing | The role-allocation handshake had no communicate-back step; governor decision `s5gs` and Gate 1 `r3wc` added it after this incident, not before. |
| `20260909T021145Z-cd7k` | Re-proposed an already-completed governor step | B — missing | No rule required walking the full governor-decision chain for an agent before authoring an evolution proposal; the recommendation proposes this as new practice, not a restatement of an existing duty. |
| `20260910T044404Z-rejw` | Poll summary exhaustion read as duty completion | A — unapplied | The universal-polling rule's reconciliation duty existed; the session did not reconcile standing decisions against the fetched target before declaring the poll complete. |
| `20260910T045334Z-wbgq` | Delayed pipeline reconciliation | A — unapplied | AGENTS.md already required comparing pipeline source/ref/status/job-coverage and cancelling proven-redundant push pipelines; the session deferred this to "a future timer" instead. |
| `20260911T122026Z-lhnx` | Blocked lane stopped all work | A — unapplied | The continuation duty ("identify other dependency-independent work") already existed; the session ended the cycle instead of applying it. |
| `20260909T015659Z-aebe` | Completed work stayed open, queue regrew | C — no owner | No step in the release workflow owned a recurring release-linked re-observation-and-closure cycle; the queue-convergence plan had already diagnosed the same mechanism once, and it recurred because nothing in the process re-triggered it at the next release boundary. |
| `20260911T050950Z-dupe` | First-push duplication not applied 17/28 | A — unapplied | The record's own conclusion states this plainly: "the gap was execution discipline, not missing guidance." |
| `20260913T075602Z-55e5` | Reviewer-appointment gap repeatedly polled | B — missing | No standing delegation authority existed for planner-appointed independent reviewers before governor decision `8tgh` created it. |
| `20260913T080313Z-qvbw` | Blocked candidate read as blocked queue | A — unapplied | The session "knew the autonomous continuation rule" (per the record's own text) but did not apply it to the rest of the queue. |
| `20260916T045614Z-vx18` | Mutating-command timeout raced a retry | B — missing | `g41r`'s own F1 finding: the current AGENTS.md `wait`-after-yield instruction does not state the broader cross-tool lifecycle and shared-worktree retry prohibition this record identifies as newly needed. |
| `20260916T145637Z-91fc` | Drafted correction sat unpublished pending permission | A — unapplied | `coordination-publication-rule.md` and `coordination-message-rule.md` already required immediate, unprompted publication of exactly this kind of correction. |

**Result: 6 of 11 (A), 4 of 11 (B), 1 of 11 (C), 0 of 11 (D).** The corrected
picture is a mixed cluster, not the single "rule text already existed for
all eleven" premise `vh4t` asserted and `g41r` correctly rejected. This
matters directly for mechanism design: cluster A is where a mechanical
self-check can plausibly help (a check can surface an existing, already-known
duty that a session is about to skip); clusters B and C are rule-text and
process-ownership gaps respectively, and a self-check cannot compensate for a
duty that did not yet exist in the rule corpus — those are closed by writing
or assigning the missing rule/ownership, which this project has already done
for `rl9k` (`s5gs`/`r3wc`), `55e5` (`8tgh`/`d1st`), and has in progress for
`vx18` (its own pending Gate 1) and `aebe` (queue-convergence follow-through).
`cd7k`'s missing practice (walk the decision chain before an evolution
proposal) has no dedicated fix yet and is out of scope for this plan; it is
named here as a candidate for a future, separate narrow rule addition, not
folded into this mechanism.

## Part 2 — Signal taxonomy (resolves G41R-A1)

Three materially different signal classes, only the first of which is safe to
specify as a concrete, implementable mechanism in this plan:

### Class 1 — Deterministic local/worktree state

Fully computable from the current worktree's Git objects and filesystem
alone, with no forge query and no interpretation. Precondition: the
worktree's remote-tracking refs must be freshly fetched immediately before
the check runs, or the check must explicitly report that it is evaluating
against a possibly-stale remote-tracking snapshot — a remote-tracking ref
that has not been fetched can be behind the actual remote state, which would
otherwise turn a real publish into a false "unpushed" warning:

- an untracked or uncommitted file under `docs/coordination/` that would pass
  `agov coord validate` if published (a drafted-but-unpublished record);
- a local branch containing coordination-only content, per
  `coordination-publication-rule.md`'s own definition of the fast path, that
  has not been pushed to any remote-tracking ref;
- a local commit on the current branch that is **committed but not yet
  pushed** to any remote-tracking ref as of the last fetch (corrected
  terminology per `2j4s`-A1: "committed and unpublished," not "published-but-
  not-pushed" — a commit is not published until it reaches the remote).

This class covers record `91fc` directly: a drafted, schema-valid correction
sitting uncommitted (or committed-but-unpushed) in the worktree is exactly
the local, no-forge-query state Class 1 can name without any model judgment
or external service call.

**Correction from the prior revision (resolves `2j4s`-F1):** `dupe` and
`wbgq` are **not** Class 1 instances. Both incidents required observing
GitLab pipeline state (source, ref, status, required-job coverage across
multiple pipeline records for the same SHA) — a forge query, not a local
Git/filesystem read. That is a materially different deterministic slice,
with its own inputs (GitLab API), authentication, availability, and
fetch/API-freshness contract, which this plan does not scope. A **candidate
Class 1b — deterministic forge-pipeline state** is named here for a future,
separate plan to define fully (its own error/availability contract, rate
limits, and required-job comparison logic) if the governor wants that
slice pursued; it is out of scope for this plan's Class 1 and is not
counted in this plan's evidence protocol (Part 6).

### Class 2 — Typed coordination-graph state

Requires the coordination schema to carry an explicit, machine-readable
lifecycle field, which does not fully exist yet:

- an unresolved `review`/`decision-record` with a `blocking` finding and no
  superseding or correcting record published since (today, "resolution" is
  partly prose per `g41r`'s own observation);
- an open `requires-ack: true` record older than a threshold with no
  `ack`/`handoff` in reply.

This class covers `rejw` (an unacknowledged standing decision) and partially
`lhnx`/`qvbw` (a recorded blocker with no recorded alternative-work
disposition). It is **not** ready to implement today: it needs the lifecycle
contract already tracked in issue #122 (self-announcing supersession) before
a check can reliably compute "unresolved." This plan scopes Class 2 as a
dependency to name, not a mechanism to build now.

### Class 3 — Semantic/model judgment

Interpreting a conversational statement such as "the next action is X and
it's mine to do," or judging whether a lane is "genuinely blocked" versus
"blocked only for one dependent item." `g41r`'s own text calls this "likely
too fuzzy for v1." This plan keeps Class 3 **out of scope entirely** for this
work — it is a future direction, not a v1 candidate, because no reliable
evaluation contract for it exists yet, not because it is permanently
ineligible.

**Correction from the prior revision (resolves `2j4s`-F3):** `tool-authority-
boundary-rule.md` does not categorically forbid a semantic/model check from
ever holding blocking authority — it requires that *any* check, deterministic
or semantic, be granted blocking authority only by a named rule that
explicitly says so. Class 3 is excluded here purely on scope and reliability
grounds. If Class 3 is ever reconsidered, that would require its own
independent evidence, its own reliability/evaluation contract, and its own
separately approved named rule explicitly granting it blocking authority —
exactly the same bar Class 1 must clear per Part 5, not a bar this plan
claims is structurally impossible to clear.

**Design conclusion:** the only mechanism this plan recommends scoping for a
follow-up implementation Gate 1 is the **Class 1, local/worktree-only
deterministic check** (record `91fc`'s shape). Class 1b (forge-pipeline
state, covering `dupe`/`wbgq`) is named as a distinct future candidate, not
part of this plan's scoped mechanism. Class 2 is named as a tracked
dependency (issue #122). Class 3 is out of scope for this plan on reliability
grounds, not on authority grounds.

## Part 3 — Automatic invocation point (resolves G41R-F2)

`g41r`'s F2 finding is that the `-o ci.skip` precedent is a validated
procedure with a live adoption gap, not proof that an optional command
reliably runs — three post-rule Codex publications omitted it, and Claude
separately suppressed both push and canonical MR pipelines on an already-open
MR during the same review window. An optional `agov coord self-check`
command that a session may or may not remember to run would repeat exactly
that failure mode. This plan therefore requires the follow-up implementation
plan to wire the Class 1 check into a point a session cannot skip by
forgetting, not into a new standalone command a session must remember to
invoke:

**Correction from the prior revision (resolves `2j4s`-F2):** the prior draft
called `agov session resume` and `agov coord status` "already-mandatory
cold-start/poll commands." That overstated the actual contract. AGENTS.md's
polling loop names `agov coord list --since <cursor>` as the first
per-cycle check; `agov session resume` is mandated only at cold start, once
per session; `agov coord status` is a supported command, never stated as
mandatory. A session can complete every later poll cycle in a long session
calling only `coord list` and never touching either proposed surface. This
plan therefore distinguishes cold-start coverage from every-poll coverage
and does not claim either candidate is already unskippable end-to-end:

- **Cold-start coverage (already mandatory today):** extend `agov session
  resume` to run the Class 1 check and surface its warning block. This is a
  real, already-mandatory boundary, but it fires once per session and would
  miss a Class 1 state that a session creates and then leaves unresolved
  mid-session without restarting.
- **Every-poll coverage (requires a follow-up rule/tooling change, not yet
  mandatory):** extend `agov coord list` — the one command AGENTS.md's poll
  loop actually requires every cycle — to run the Class 1 check and surface
  its warning in the same output. This candidate covers the every-poll
  boundary `91fc`'s shape needs, but making `coord list` carry this check is
  itself a tooling change the follow-up implementation plan must propose and
  get Gate 1 approval for; it is not automatically covered by today's rule
  text simply because `coord list` is already mandatory for a different
  purpose.
- **Secondary candidate, if tooling access allows:** a pre-push or pre-commit
  local hook that runs the Class 1 check before a session's own publish
  step. This catches the state at the moment a session tries to publish
  something else, but — as `2j4s`-F2 notes — it does not fire at all if the
  session ends the poll or session without attempting any push, so it is a
  useful additional layer, not a substitute for the every-poll coverage
  above.

None of these candidates is claimed to be automatically unskippable today.
The follow-up implementation plan must: (a) pick the exact integration
point(s) from the above, stating explicitly which completion boundary each
one covers and which it does not; (b) get that tooling change through its
own Gate 1, since extending a mandatory command's behavior is itself a
change to that command's contract; and (c) define how a bypass or an
incomplete/failed check run is reported, so a silent skip cannot be mistaken
for a clean result.

## Part 4 — Worktree, multi-session, and visibility limits

Stated explicitly per `g41r`'s resolution condition, so the follow-up
implementation plan inherits them rather than re-discovering them:

- **Worktree scope.** The Class 1 check can only see the worktree it runs in.
  A drafted-but-uncommitted file in a *different* worktree or linked
  checkout (see `branch-and-worktree-rule.md`'s worktree boundary) is
  invisible to a check run from this checkout. The check's output must state
  which worktree it ran against, not imply repository-wide coverage.
- **Multi-session visibility.** Per this project's own concurrent-session
  reality (Claude and Codex both active, sometimes in the same role
  lineage), a Class 1 check run by one session cannot see another session's
  uncommitted local state at all — only that other session's own *published*
  Git objects (commits, branches, remote refs) are visible to it. The check
  must not claim to detect another live session's unpublished draft.
- **Coverage boundary.** Consistent with `coordination-publication-rule.md`'s
  own "Channel membership and bounded candidate discovery" section, the
  check's output must disclose its exact scan scope (which refs, which
  directory depth, whether it fetched first) and must fail closed — report
  incomplete coverage rather than a false "clean" result — exactly as
  `coord publication-check` and `coord hold-status` already do.

## Part 5 — Tool-authority-boundary preservation

Per `tool-authority-boundary-rule.md`, this plan's Class 1 check ships as
**advisory only**. No named rule in this plan grants it blocking authority.
The follow-up implementation plan may propose blocking authority for the
Class 1 check only after:

1. it has run for a bounded evidence-collection period (see Part 6) with a
   published false-positive and missed-obligation rate;
2. a separate, dedicated Gate 1 plan explicitly proposes the blocking rule
   change, naming the exact check, the exact rule file that would grant
   blocking authority, and the exact failure mode it targets; and
3. that separate Gate 1 plan itself carries a negative test demonstrating
   the gate fires on a known-bad input, per `gate-self-test-rule.md` — gate
   mechanics are never eligible for the fast-track waiver, and a
   self-check that gains blocking authority is gate mechanics.

This plan does not pre-authorize step 2 or 3. It only clears the ground so
that a future plan proposing them starts from real advisory-mode evidence
instead of a hypothesis.

## Part 6 — Evidence collection before any blocking proposal

**Corrected from the prior revision (resolves `2j4s`-F4).** The prior draft
gave a window and four labels with no trial unit, denominator, durable
schema, labeler, false-negative sampling method, or sufficiency/acceptance
criteria — none of which can produce an actual rate. This revision defines
the trial concretely, and narrows its eligible scope to exactly what Class 1
(local/worktree-only, per Part 2's correction) can observe: the `91fc` shape
(a drafted, schema-valid coordination correction sitting uncommitted or
committed-but-unpushed in the acting worktree). `dupe`/`wbgq` (forge-pipeline
state) and every missing-rule, no-owner, typed-graph, or semantic incident
shape are **not eligible** for this trial — the local check cannot observe
them, so counting their recurrence against this check's hit rate would be
meaningless.

**Unit of observation:** one *completion boundary* — a discrete moment the
implementation's chosen integration point (Part 3) runs the Class 1 check,
whether at cold start or at a per-poll `coord list` invocation. The
denominator for any rate is the count of completion boundaries observed
during the trial window, not the count of warnings.

**Durable record.** Each completion boundary's check result is appended to a
plain, git-tracked trial log (e.g.
`docs/reviews/known-duty-self-check-trial.jsonl`, one JSON line per
observation) recording: timestamp, session/role, worktree path, integration
point (cold-start vs. per-poll), raw check result (clean / N warnings with
their exact file paths or commit SHAs), and — filled in during labeling, not
at observation time — a `label` field left `null` until reviewed.

**Labeling and independent adjudication.** Every observation with at least
one warning is labeled by a session or human *other than* the one whose
worktree produced the warning (mirroring this project's own independent-
review discipline), as one of: `true_positive` (the flagged file/commit was
a genuine unpublished duty the acting session had not yet acted on),
`false_positive` (the flagged state was intentional — e.g., a deliberately
staged draft still being edited, or a file held for a later batch under
`coordination-publication-rule.md`'s batching permission), or
`inconclusive` (insufficient information to decide; recorded, not discarded).

**False-negative sampling (missed obligations).** Because the check cannot
prove a negative on its own, a bounded, fixed-size random sample of
*clean* (zero-warning) completion boundaries — proposed: every 10th clean
observation — is independently re-examined by inspecting that worktree's
actual state at that timestamp (via `git log`/`git status` at the recorded
commit) for an unflagged `91fc`-shape condition the check should have caught
but did not. This is the only mechanism in this protocol that can produce a
false-negative/missed-obligation count; without it, "no warnings fired" is
unverified, not evidence of correctness.

**Minimum window and sufficiency.** Continue the trial for whichever is
longer of 20 completion boundaries or 5 calendar days, across both active AI
sessions. If, at the end of that window, fewer than 5 warning observations
and fewer than 5 sampled clean observations have accumulated, the trial
result is **inconclusive** and the window extends until both minimums are
met — a report must not present a rate computed from fewer than 5 events in
either bucket.

**Acceptance and revisit criteria.** The follow-up blocking-authority
proposal, if any, must cite: the exact false-positive rate (false positives
÷ all warning observations) and the exact sampled false-negative rate
(missed obligations found in the false-negative sample ÷ sampled clean
observations), both with their raw counts and denominators, not just a
percentage. A revisit is required if either rate is later found, on a
larger sample, to differ materially from what was reported.

This data, not a design argument, is what should settle whether blocking
authority is ever proposed for the Class 1 check specifically. It says
nothing about Class 1b, Class 2, or Class 3, which would each need their own
trial if pursued.

## Risks and rollback

- **Risk:** a follow-up implementation could be tempted to fold Class 2 or
  Class 3 signals in "while it's convenient." This plan's scope explicitly
  excludes both; a follow-up plan that adds them must justify each
  separately against the same false-positive/coverage discipline as Class 1.
- **Risk:** surfacing warnings inside `agov session resume`/`agov coord
  list` output could be misread as blocking if not clearly labeled advisory.
  The implementation plan must label every Class 1 finding `severity:
  advisory` per `tool-authority-boundary-rule.md`.
- **Rollback:** this plan produces no running code; there is nothing to roll
  back. A follow-up implementation, once it exists, rolls back by reverting
  its own commit — ordinary `branch-and-worktree-rule.md`/`merge-request-
  workflow-rule.md` discipline applies, nothing special to this design.

## Revision note (exact-head correction round)

The prior revision at `9de49781` received contribution-auditor review
`20260916T174117Z-contribution-auditor-2j4s` (BLOCK, 4 mandatory findings, 1
advisory). This revision corrects all five: moving `dupe`/`wbgq` out of
Class 1 into a named-but-out-of-scope Class 1b (F1); replacing the "already-
mandatory" invocation claim with an honest cold-start-vs-every-poll split
naming `coord list` as the every-poll integration target still requiring
its own Gate 1 (F2); correcting the tool-authority-boundary characterization
of Class 3 (F3); replacing the evidence protocol with a concrete trial
design scoped to what Class 1 can actually observe (F4); and correcting
"published-but-not-pushed" to "committed but unpublished" (A1). Re-review
requested at this exact head.

## Next steps

1. Request contribution-auditor re-review of this corrected revision.
2. Request governor-reviewer Gate 1 decision once the auditor row clears.
3. On approval, a separate implementation Gate 1 plan scopes the actual
   Class 1 check, its integration point (Part 3), and the evidence-collection
   protocol (Part 6) as its own acceptance criteria — this plan does not
   pre-approve that implementation's design in detail, only its boundaries.
