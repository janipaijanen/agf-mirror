---
id: 20260911T052932Z-ci-first-push-publication-guard
status: pending-gate1
topic: ci-first-push-publication-guard
advisory_consultation:
  - role: contribution-auditor
    status: waived
    reason: >
      Not synchronously consulted before drafting, under the governor's
      explicit CI-priority sequencing (`p7tr`: no new RUW branch work
      starts ahead of this fix; advance a bounded Gate 1 plan). Waived
      rather than blocked because the change is narrow and additive by
      construction: it adds a publication *procedure* and refines one
      learning-record recommendation field, touches no `.gitlab-ci.yml`
      logic, and does not narrow `ci-pipeline-deduplication`'s approved
      scope (Scope section above states this explicitly — that plan's
      rule is relied on, not modified). Contribution-auditor drove that
      sibling plan (2026-07-22) and remains free to review this one at
      or before Gate 2; any finding narrows or blocks implementation,
      not this Gate 1 filing.
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
  cost table for six same-head pairs (!2684-!2689), corroborating the same
  defect with harder numbers (174.7s-207.0s wasted per pair).
- **Codex's `atkm` note** (`docs/coordination/20260911T051355Z-build-engineer-atkm.md`,
  same branch): confirmed via the GitLab API that all six `g5mj` push
  pipelines were *created* before their MRs existed (8.3s-13.3s before MR
  registration in each case) — this is GitLab's documented first-push
  limitation, not a double-push or a guard failure. `atkm` proposed the
  prevention-first mechanism this plan implements, and explicitly advised
  refining `dupe`'s cancellation-first recommendation rather than filing a
  competing learning.
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
cancellation cannot reliably beat these pipelines to completion: the
`g5mj` sample's push pipelines ran to full `success` in single-digit to
low-double-digit seconds, often faster than a human or agent poll cycle
notices them.

**Conclusion: prevent the redundant pipeline from being created, rather
than trying to catch and cancel it afterward.** Reactive cancellation
remains the correct *fallback* for the paths this plan does not change
(see Scope), not the primary mechanism for the new-branch-plus-immediate-MR
path.

## Objective

For the specific sequence "push a brand-new branch, then immediately open
its MR" — the shape that produced all 17 confirmed duplicates — ensure
exactly one CI pipeline instance is created for the branch's initial SHA:
the canonical `merge_request_event`-sourced pipeline. Achieve this by
skipping pipeline creation on the first push itself, not by suppressing or
cancelling a pipeline after GitLab has already created it.

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

3. **Verify exactly one canonical pipeline exists** for the intended SHA:
   query the project's pipelines for that ref/SHA and confirm exactly one
   result, sourced from `merge_request_event`. This is the visible
   evidence step — it is what makes "the fix worked" a checked fact
   instead of an assumption.

4. **Fail visibly if MR creation fails, or if step 3's check does not find
   exactly one canonical pipeline.** A push with `ci.skip` that is not
   followed by a successful MR registration leaves the branch with *zero*
   pipelines — a worse silent state than a duplicate, because it looks
   like "nothing to check" rather than "wasted a runner-minute." The
   recovery path on failure is: retry MR creation immediately; if that
   also fails, push again *without* `ci.skip` (an ordinary push-sourced
   pipeline; not ideal, but visible and safe) and record the failure.
   Never leave a `ci.skip`-published branch unregistered and unpiped
   without an explicit, recorded follow-up.

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
  verification procedure as the primary publication path for a brand-new
  branch that will get an MR immediately, in AGENTS.md's polling/
  publication guidance (the "known first-push limitation" paragraph) and
  in `docs/reference/agent-cold-start-habits.md` if that file documents
  the same sequence.
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
| Normal path: `ci.skip` push, immediate MR create succeeds | Exactly one pipeline at the SHA, `merge_request_event`-sourced |
| MR creation fails after `ci.skip` push | Visible failure recorded; recovery path followed (retry create, or re-push without `ci.skip`); branch never left silently un-piped |
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
  Foundry:          Additive procedural guidance; no existing setup breaks.
                     Agents not yet using ci.skip continue to get the
                     existing (imperfect) cancel-after-the-fact coverage.
  Direct adopters:  ADDITIVE — a new recommended publication habit for
                     projects with the same GitLab first-push limitation.
                     Nothing previously permitted is forbidden.
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
