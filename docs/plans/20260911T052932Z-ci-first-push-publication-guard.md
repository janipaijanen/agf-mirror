---
id: 20260911T052932Z-ci-first-push-publication-guard
status: approved
gate1_approved_by: governor-reviewer
gate1_approved_at: "2026-09-11T06:01:41Z"
topic: ci-first-push-publication-guard
advisory_consultation:
  - role: contribution-auditor
    status: waived
    reason: >
      Not synchronously consulted before drafting. This is waived on the
      plan's own narrow, additive-by-construction scope (adds a
      publication *procedure* and refines one learning-record
      recommendation field; touches no `.gitlab-ci.yml` logic; does not
      narrow `ci-pipeline-deduplication`'s approved scope) — not on
      `p7tr`'s sequencing priority alone, which build-engineer's `e1jt`
      review correctly notes cannot by itself waive advisory
      consultation. Contribution-auditor drove the sibling
      `ci-pipeline-deduplication` plan (2026-07-22) and remains free to
      review this one at or before Gate 2; any finding narrows or blocks
      implementation, not this Gate 1 filing. Build-engineer's `e1jt`
      review already supplied a bounded diagnostic pass (3 blocking
      findings, corrected in this revision) but explicitly does not
      substitute for a full contribution audit.
  - role: infosec-sentinel
    status: not-applicable
    reason: >
      No secrets, permissions, credential, or trust-boundary surface is
      touched. `-o ci.skip` is a documented GitLab push option evaluated
      client-side by the `git`/GitLab push handshake, not a commit-
      message marker and not a change to any CI job's permissions or
      trust model. It affects only whether a pipeline instance is
      created for one specific push event, exactly as the existing
      `workflow:` `when: never` rule already does for a different push
      event. No `.gitlab-ci.yml` job, gate, or CI credential changes.
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: prevent the first-push CI pipeline duplicate at publication time

**Revision note (2026-09-11T05:5x):** build-engineer's independent review
(`e1jt`, `docs/coordination/20260911T054827Z-build-engineer-e1jt.md`)
returned BLOCK with 3 findings, all corrected in this revision: (CI-PUB-1)
Mechanism step 4's recovery path replaced blind unchanged re-push
(a no-op against an already-current remote branch) with re-query-and-adopt
plus bounded retry and an explicit blocker on exhaustion; (CI-PUB-2)
Objective/Mechanism/acceptance-matrix wording now consistently targets
"exactly one job-running pipeline," tolerating the inert `skipped` push
record instead of contradictorily implying zero pipeline records;
(CI-PUB-3) the Origin/evidence and insufficiency sections now cite the
actually measured quantities (8.259s-13.310s push-to-MR-registration
interval; 174.712s-207.017s per completed redundant job pair, 978.068s
summed across five pairs) instead of the earlier, imprecise "ran to
success before the MR even existed" framing. The advisory finding
(consultation wording) is addressed in the `contribution-auditor` waiver
reason above and the ADDITIVE-vs-required Impact wording below.

## Origin and evidence

The governor flagged duplicate CI pipelines this session ("WTF! duplicate
pipelines! Stop dupes. LEARN!"). Two independent evidence trails converged
on the same root cause and complementary fixes:

- **Claude's `dupe` learning** (`agf/learnings/foundry.yaml`
  `20260911T050950Z-dupe`, MR !2690, not yet merged): audited 28 of this
  session's own MRs and found 17 confirmed instances where the redundant
  `push`-sourced pipeline ran to full `success` before the corresponding
  `merge_request_event` pipeline was even created, duplicating the same
  required-job set. Recommended mandatory check-and-cancel discipline as
  the immediate fix, plus a request that build-engineer evaluate automated
  suppression.
- **Codex's `g5mj` handoff** (`docs/coordination/20260911T051118Z-coder-tester-g5mj.md`,
  on `origin/codex/claude-transfer-followup`): an independent exact-second
  cost table for six same-head pairs (!2684-!2689). Five pairs' redundant
  push-sourced pipelines ran their required jobs to completion, summing
  **978.068 runner-seconds** across those five (174.712s-207.017s each);
  the sixth was cancelled live, with null job durations recorded.
- **Codex's `atkm` note** (`docs/coordination/20260911T051355Z-build-engineer-atkm.md`,
  same branch): confirmed via the GitLab API that all six `g5mj` push
  pipelines were *created* **8.259-13.310 seconds before their MRs were
  registered** — this is the measured push-to-MR-registration interval,
  not a claim about job completion time or duration. It demonstrates
  GitLab's documented first-push limitation (the push pipeline exists,
  irreversibly, before the MR does), not a double-push or a guard
  failure. `atkm` proposed the prevention-first mechanism this plan
  implements, and explicitly advised refining `dupe`'s cancellation-first
  recommendation rather than filing a competing learning.
- **Governor sequencing, relayed via Codex's `p7tr`**
  (`docs/coordination/20260911T051558Z-planner-architect-p7tr.md`): "When
  ready with CI fix, after collaborating with claude, then continue RUW."
  No new RUW branch work starts ahead of this fix; ownership must be
  confirmed complementary (not overlapping) before mutation.

This plan is the "bounded Gate 1 plan" `p7tr` calls for, drafted by
planner-architect (Claude) per that sequencing. No overlapping mutation
was found: as of this plan's authoring, no other branch or MR proposes a
`.gitlab-ci.yml`, publication-procedure, or `ci-pipeline-deduplication`
plan change (`origin/codex/claude-transfer-followup` and the existing
`docs/plans/ci-*.md` files were checked; none post-date `p7tr` or
duplicate this scope).

## Why the existing mitigation is insufficient

`.gitlab-ci.yml`'s `workflow:` rules already suppress a *later* redundant
push pipeline once `$CI_OPEN_MERGE_REQUESTS` is true
(`docs/plans/ci-pipeline-deduplication.md`, approved 2026-07-22). Its own
comment states this guard "cannot suppress" the very first push, because
no MR exists yet at that moment — this is not a defect in that plan; it is
a documented, intentional scope boundary. AGENTS.md's polling contract
already asks agents to compare pipeline source/ref/status/job-coverage and
cancel the demonstrably redundant push pipeline after the fact. `dupe`'s
audit shows that reactive, memory-dependent step was not applied
consistently even by the session that wrote the instruction (17 of 28
sampled MRs missed it). `atkm` independently observes that polling-based
cancellation cannot reliably beat these pipelines: the `g5mj` sample shows
the redundant push pipeline is *created* 8.259-13.310 seconds before the
MR is even registered — often faster than a human or agent poll cycle
notices it exists — and, once created, five of six sampled instances ran
their required jobs to full `success`, consuming 174.712s-207.017s each
(978.068s total) before any cancellation was attempted.

**Conclusion: prevent the redundant pipeline from being created, rather
than trying to catch and cancel it afterward.** Reactive cancellation
remains the correct *fallback* for the paths this plan does not change
(see Scope), not the primary mechanism for the new-branch-plus-immediate-MR
path.

## Objective

For the specific sequence "push a brand-new branch, then immediately open
its MR" — the shape that produced all 17 confirmed duplicates — ensure
**exactly one pipeline that consumes runner time** exists for the
branch's initial SHA: the canonical `merge_request_event`-sourced
pipeline. An inert `skipped`-status push-sourced pipeline *record* is an
expected and acceptable side effect of the mechanism below, not a
violation of this objective — see Mechanism step 3 for the precise
distinction between "a pipeline record exists" and "a pipeline runs
jobs." Achieve the objective by skipping job execution on the first push
itself, not by suppressing or cancelling a pipeline after GitLab has
already created and started it.

## Mechanism

1. **First push uses `-o ci.skip`.** When publishing a new branch that
   will immediately get an MR, push with:

   ```bash
   git push -o ci.skip -u origin <new-branch>
   ```

   `ci.skip` here is a **GitLab push option** — a flag evaluated by the
   push handshake itself, distinct in kind from a `[ci skip]`/`[skip ci]`
   *commit-message* marker. `agf/rules/workflow/coordination-publication-rule.md`
   and AGENTS.md's coordination section forbid the commit-message marker
   for coordination-only commits specifically, because it is silently
   baked into immutable commit history and skips pipeline creation for
   every future re-push of that exact commit, including a later
   non-skipped rebase that still contains it in the range. A push option
   carries no such risk: it applies only to the one push invocation that
   passes it, is not stored in the commit object, and is transparent to
   any evidence-gathering that inspects the commit range rather than the
   push event. This plan does not relax the commit-message-marker
   prohibition in any way — the two mechanisms are not equivalent, and
   this plan only authorizes the push-option form for the specific
   publication sequence in this section.

2. **Immediately register the MR** (`glab mr create` or equivalent) for
   the just-pushed branch, before any further push to that branch.

3. **Verify exactly one non-skipped canonical pipeline exists** for the
   intended SHA: query the project's pipelines for that ref/SHA and
   confirm exactly one result with a real (non-`skipped`) status, sourced
   from `merge_request_event`. GitLab still creates a pipeline *record*
   for the `ci.skip`-marked push — confirmed empirically on this plan's
   own publishing MR (!2692): pipeline `2839715523` (`push` source)
   appeared with `status: skipped`, alongside `2839715810`
   (`merge_request_event` source) as the only pipeline that actually ran
   jobs, both at the same SHA. The verification target is therefore "no
   second pipeline consumes runner time," not "no second pipeline record
   exists at all" — the `skipped` record is inert and free. This is the
   visible evidence step; it is what makes "the fix worked" a checked
   fact instead of an assumption.

4. **Fail visibly if MR creation fails, or if step 3's check does not find
   exactly one canonical, job-running pipeline.** A push with `ci.skip`
   that is not followed by a successful MR registration leaves the branch
   with *zero pipelines that will ever run its required jobs* (the
   `ci.skip` push record itself stays permanently `skipped` — it does not
   retroactively start). This is a worse silent state than a duplicate,
   because it looks like "nothing to check" rather than "wasted a
   runner-minute."

   **Recovery is re-query-and-adopt, not blind re-push.** An unchanged
   re-push of the same commit onto a branch that already exists on the
   remote is a no-op: `git` reports up to date and sends no ref update,
   so it cannot be relied on to create a new pipeline, and retrying MR
   creation after an uncertain (e.g. timed-out) response risks racing a
   creation that actually succeeded server-side. On any uncertain or
   failed MR-creation result:
   1. Re-query the project for an existing MR sourced from the exact
      branch, and re-resolve the branch's current remote head SHA. If a
      matching MR is found, adopt it (do not create a second one).
   2. If no MR is found, retry creation with a bounded number of attempts
      and a bounded wait between them.
   3. If, after bounded retries, no canonical `merge_request_event`
      pipeline can be established for the current SHA, **stop and record
      an explicit blocker** naming the exact branch, ref, and SHA and an
      owner for follow-up — do not claim the branch is recovered, and do
      not push an empty commit or retry a failed paid job merely to force
      a green run (per AGENTS.md's existing prohibition on both).
   Never leave a `ci.skip`-published branch unregistered and unpiped
   without an explicit, recorded blocker per step 3 above.

5. **Subsequent pushes to the same branch are unchanged** — they already
   fall under the existing `$CI_OPEN_MERGE_REQUESTS` guard from the
   approved `ci-pipeline-deduplication` plan, which suppresses a
   redundant push pipeline once the MR exists. This plan does not modify
   that rule.

6. **Reactive cancellation remains the fallback**, used only when: `ci.skip`
   was not available or not used (manual GitLab UI push, tooling that
   cannot pass push options, a session that forgot the procedure), or when
   step 4's failure path itself produces a duplicate. AGENTS.md's existing
   compare-source/ref/status/job-coverage-then-cancel instruction stays in
   force for exactly these residual cases; it is demoted from "the
   mitigation" to "the safety net for paths outside this plan's scope."

## Scope — what this plan does and does not change

**Changes:**
- Adds the `-o ci.skip` + immediate-MR-registration + single-pipeline-
  verification procedure as the **required** primary publication step
  (replacing check-and-cancel as primary, for this specific sequence
  only) for a brand-new branch that will get an MR immediately, in
  AGENTS.md's polling/publication guidance (the "known first-push
  limitation" paragraph) and in `docs/reference/agent-cold-start-habits.md`
  if that file documents the same sequence. This is documented operating
  procedure, not a mechanical CI gate — its success still depends on
  operator/agent execution, exactly as the check-and-cancel step it
  replaces did. This plan does not claim to close that residual risk (see
  Risks below); it only changes what the required procedure *is*.
- Refines `agf/learnings/foundry.yaml`'s `20260911T050950Z-dupe` entry's
  `recommendation` field (still unmerged on MR !2690) to name this plan as
  the primary fix and demote check-and-cancel to the documented fallback.
  Learning `description`/evidence fields are left intact — only the
  forward-looking recommendation changes, and the change is made before
  the entry ever reaches `main`, so no supersession record is needed.
- No `.gitlab-ci.yml` logic change. The existing `workflow:` rule
  (`ci-pipeline-deduplication`, approved) is unmodified — this plan relies
  on it for step 5 and does not touch it.

**Explicitly does not change** (preserved unless separately approved, per
`atkm`'s note):
- Ordinary branch-only publication with no immediate MR intent — an agent
  or human who pushes a branch and does not open an MR right away gets the
  ordinary push-sourced pipeline, unchanged. `ci.skip` is used only when
  an MR is about to be registered for that exact push.
- Default-branch pushes, tag/release pushes, scheduled, API, and manual
  pipeline triggers.
- The commit-message `[ci skip]`/`[skip ci]` prohibition — untouched, see
  Mechanism step 1.
- Any Gate 2 acceptance criterion, exact-head pipeline requirement, or
  required-job set. The canonical acceptance path stays the
  `merge_request_event` pipeline; this plan only removes a redundant
  sibling, never the canonical pipeline itself.

## Acceptance test matrix (Gate 2 requirement)

Per `atkm`'s note, the implementing MR must demonstrate all of:

| Case | Expected outcome |
|---|---|
| Normal path: `ci.skip` push, immediate MR create succeeds | Exactly one job-running pipeline at the SHA (`merge_request_event`-sourced); the inert `skipped` push-sourced record is expected, not a defect |
| MR creation fails or is uncertain after `ci.skip` push | Re-query for an existing MR on the branch and adopt it if found; else bounded retry; else stop and record an explicit blocker naming branch/ref/SHA — never a blind unchanged re-push (a no-op against an already-current remote branch) and never claimed-recovered without a verified canonical pipeline |
| Branch already has an open MR before this push (subsequent push, not first) | Unaffected — existing `$CI_OPEN_MERGE_REQUESTS` guard applies as today |
| Wrong/stale SHA at verification time (e.g. a fast follow-up push before verification completes) | Verification step re-resolves the current head SHA rather than trusting a cached value; does not falsely report success against a superseded SHA |
| Canonical pipeline absent or duplicated at verification time | Treated as a failure of step 3/4, not silently accepted; triggers the recovery path |
| Push retry (network failure, force-push-with-lease during construction) | Re-verification re-runs from step 3; does not assume the first attempt's result still holds |
| Intentional branch-only publication (no MR planned for this push) | Unaffected — ordinary push-sourced pipeline runs exactly as before this plan |

Per `always/gate-self-test-rule.md`: this plan does not introduce or
modify a validator, hook, or CI blocking rule — it changes an operating
*procedure* plus one learning-record recommendation field. The negative-
test obligation that rule imposes is for gate mechanics specifically;
nonetheless, the "MR creation fails" and "canonical pipeline absent/
duplicated" rows above are this plan's equivalent negative demonstrations,
and Gate 2 evidence must show each one exercised (a deliberately induced
MR-create failure, and a deliberately absent canonical pipeline check),
not merely asserted.

## Risks and rollback

- **Risk:** an agent uses `-o ci.skip` and then forgets or is blocked from
  completing MR registration, leaving a branch with zero CI coverage
  indefinitely. **Mitigation:** step 4's fail-visibly requirement plus the
  acceptance matrix's dedicated test case. Any session encountering this
  state records it as a blocker, not a silent gap.
- **Risk:** the procedure adds a step agents forget under time pressure,
  regressing to the pre-fix behavior. **Mitigation:** this is a documented
  operating habit, not a mechanical gate — full closure of that residual
  gap would require actual tooling (e.g. a wrapper script or `agov`
  subcommand for "publish new branch + MR"), explicitly deferred per
  simplicity-rule until this procedural fix is evidenced in practice; if
  discipline again proves insufficient, that evidence is exactly what
  would justify build-engineer building the automated wrapper `dupe`'s
  recommendation already floated.
- **Rollback:** revert the AGENTS.md/cold-start-habits wording change and
  the learning-recommendation refinement. No CI configuration, schema, or
  gate is touched, so rollback carries no migration cost.

## Impact (downstream-impact-rule)

```
Impact:
  Foundry:          ADDITIVE at the framework-contract level (nothing
                     previously permitted is forbidden), but this session's
                     agents MUST use the new procedure going forward for
                     the specific new-branch+immediate-MR sequence — it
                     replaces check-and-cancel as the required primary
                     step for that sequence only. Agents not using it
                     (or falling back per Mechanism step 6) still owe the
                     existing check-and-cancel step.
  Direct adopters:  ADDITIVE — a new publication procedure available to
                     projects with the same GitLab first-push limitation;
                     adopters choose whether to make it required for their
                     own agents. Nothing previously permitted is forbidden.
  Downstream:       No adopter-produced artifact shape changes. No agent-
                     facing behavior toward an adopter's own users changes.
  Migration:        None required — the existing compare-and-cancel
                     fallback keeps working for any adopter that does not
                     take up the new procedure.
  Evidence:         agf/learnings/foundry.yaml 20260911T050950Z-dupe (MR
                     !2690); docs/coordination/20260911T051118Z-coder-tester-g5mj.md;
                     docs/coordination/20260911T051355Z-build-engineer-atkm.md.
```

## Test strategy

No automated test suite covers operator procedure text. Verification is
the acceptance matrix above, exercised on the implementing MR's own
publication (a natural "eat your own dogfood" case: that MR itself should
be publishable via this exact procedure and show a single canonical
pipeline).

## Rollout

1. Gate 1 approval (this plan).
2. Implementation MR: AGENTS.md wording change (and
   `docs/reference/agent-cold-start-habits.md` if applicable) +
   `dupe` learning-recommendation refinement on MR !2690's branch (or a
   superseding commit if !2690 has meanwhile merged — coordination-
   publication-rule's supersession-not-mutation principle applies if so).
3. Gate 2: exact-head pipeline, acceptance matrix evidence, governor
   merge.
4. Resume RUW work per `p7tr`'s sequencing once this passes.
