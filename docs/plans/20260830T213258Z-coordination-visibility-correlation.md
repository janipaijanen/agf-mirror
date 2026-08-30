---
id: 20260830T213258Z-coordination-visibility-correlation
status: pending-gate1
topic: coordination-visibility-correlation-in-queue-surfaces
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      No credential, secret, personal-data, or cross-record content is read or
      emitted. The change reads coordination front matter already tracked in
      the repository and already readable by every agent, and adds no network
      access, no new file writes, and no new external input.
  - role: contribution-auditor
    status: consulted
    reason: >-
      The motivating incident and its evidence are recorded in
      docs/coordination/20260830T210228Z-coder-tester-1tm5.md and the stranded
      record 20260828T052452Z-governor-reviewer-dmtn.md. Routed for audit of
      whether the proposed severity and scope match the evidence.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Coordination queue surfaces must not answer confidently from a partial view

## The defect

`agov handoff next` reported `ksq6` as a blocked governor-reviewer obligation
on 2026-08-30. It had been resolved on 2026-08-28 by governor decision-record
`dmtn`, which sat unmerged on `origin/coordination/resolve-ksq6` (MR !2172).
The tooling read `main`, `dmtn` was not on `main`, so the queue reported a
state the repository itself contradicted.

An agent — this one — then repeated that stale answer to the governor three
times as "your oldest open decision," with increasing confidence, because a
deterministic tool had said so.

**The defect is not that the record was late.** Late publication is an
untidiness. The defect is that a query surface returned a *definite,
unqualified* answer derived from a view it could not know was complete, and
had no way to signal that.

## Why discipline cannot fix this

The obvious remedy — "agents should also run `agov coord visibility`" — fails
for the reason `always/gate-self-test-rule.md` already documents: process
discipline is at-will and breaks under speed. It also fails on evidence:

- `agov coord visibility` **already existed** and **already reported `dmtn`**.
- It still did not prevent the error, because it *lists* branch-local records
  without *correlating* them to the pending obligation. A human or agent must
  read both outputs and make the link. I made that link only after the
  governor had already been misinformed three times.

A second failure the same night (`1tm5`) has the identical shape: a control
that existed as an expectation rather than a mechanism.

## A second instance, found while filing this plan

The defect has two distinct sub-cases. Both were observed in this repository
within one hour on 2026-08-30, and both are fixed by the same mechanism.

**Sub-case A — resolving evidence is off the base branch.** The `ksq6` /
`dmtn` incident above. Requires cross-branch resolution.

**Sub-case B — resolving evidence is *on* the base branch and still ignored.**
`agov handoff next governor-reviewer` currently reports:

> Issue a fresh binding Gate 2 disposition for MR !2181 …

MR !2181 merged on 2026-08-28T11:25:21Z, by `janipaijanen` — the governor
directly, not an agent. Sixteen minutes earlier, governor decision-record
`20260828T110944Z-governor-reviewer-prn4` ("Gate 2 reissued on corrected
terms; v2.8.0 publication authorized") landed on `main`, and it names the
originating handoff `rdm8` in its own `references-to`.

So the resolving decision is on `main`, correctly authored, correctly linked —
and the queue still reports the obligation as outstanding, because it does not
treat a decision-record referencing a handoff as resolving that handoff.

**Sub-case B is the cheaper fix and probably the more frequent failure.** It
needs no cross-branch work at all: it is a graph walk over records already on
the base branch. It is also previously known — resolution-awareness in handoff
tooling was raised earlier in this project's history and not carried through.

Both sub-cases are the same defect stated once: **a queue surface reports an
obligation that evidence available to it already resolves.** Sub-case A is
evidence it cannot see; sub-case B is evidence it can see and does not use.

## Scope of this plan

**In scope — the correlation fix.** `agov handoff next`, `agov coord pending`,
and `agov session resume` must, by default:

1. Resolve branch-local coordination records not present on the base branch,
   using the machinery `agov coord visibility` already has.
2. Correlate them to the obligations being reported, via the existing
   `references-to` graph that `agov coord graph-index` already builds — not by
   listing them separately for a reader to join by hand.

2a. **Resolution-awareness on the base branch (sub-case B).** An obligation
   MUST NOT be reported as outstanding when a later record on the base branch
   references it and carries a resolving kind (`ack`, `completion`, or a
   `decision-record` from an authorized role for that gate). Where resolution
   is ambiguous — a referencing record that neither clearly resolves nor
   clearly continues — report the obligation as `UNCERTAIN` and name the
   referencing record, rather than silently choosing either reading.
3. **Degrade the answer, not footnote it.** When a branch-local record
   references a reported obligation, the obligation is reported as
   `UNRELIABLE`, naming the branch, the record ID, its kind, and its `to`
   field. A caller that reads only the first line must not receive a clean
   answer.
4. Emit a machine-readable equivalent in `--format json` so agent surfaces and
   the MCP tools inherit the same signal rather than reimplementing it.

**Explicitly out of scope, each needing its own Gate 1:**

- **Auto-merge of clean coordination branches.** This is merge policy, and
  therefore gate mechanics under `workflow/simplicity-rule.md` — ineligible
  for any fast path regardless of diff size. It is the strongest available
  prevention and should be planned next, separately.
- **Required `supersedes:` on decision-records.** `dmtn` itself flags that
  `2h4k` and `2h2s` are two binding governor records for one decision. That
  is a schema and lifecycle change.
- **Scheduled stranded-record aging report.** Useful, independent, cheap.
- **Session-end unpushed-coordination check.** Covers the residual case below.

Splitting these is deliberate. Bundling a query-surface fix with a merge-policy
change would drag gate mechanics into a slice that does not otherwise touch a
gate.

## Honest limit

This does not reach certainty, and the residual is specific: a record written
and never committed, or committed and never pushed, is invisible to every
mechanism proposed here, because git cannot see what is not in it. That case
needs the session-end check listed above.

What this plan closes is the *observed* mode — written, pushed, unmerged —
which occurred at least nine times in this repository and was the direct cause
of the incident. Any stronger numeric claim about residual risk would be a
reasoned estimate rather than a measurement, and
`always/verifiable-claims-rule.md` forbids stating it as fact.

## Is this a gate change?

**No, and the boundary matters.** `handoff next` and `coord pending` are query
surfaces: they report state, they do not block anything. Nothing in
`agf/rules/` grants them blocking authority, and
`agf/rules/always/tool-authority-boundary-rule.md` is explicit that tool output
without a named blocking rule is advisory evidence.

Making an advisory report *more* honest does not create blocking authority.
The out-of-scope auto-merge item is where gate mechanics begin, which is why
it is separated.

If the reviewer disagrees and considers this gate mechanics, the correct
consequence is a mandatory negative test under
`always/gate-self-test-rule.md` — which this plan should carry anyway (below),
so the disagreement changes the label, not the work.

## Test strategy

Negative tests are the acceptance criterion, not an afterthought:

1. **The exact `ksq6` shape.** Base branch reports an obligation; an unmerged
   branch carries a record referencing it. The surface MUST report
   `UNRELIABLE`. This is the regression test for the incident.
2. **Correlation, not mere presence.** An unmerged branch carrying a record
   that references *nothing* in the pending set MUST NOT degrade unrelated
   obligations — otherwise every answer becomes unreliable and the signal is
   worthless.
3. **Clean case.** No branch-local records: answer is reported normally, with
   no degradation text.
4. **Unfetchable or absent remote.** MUST fail closed or state the limit
   explicitly; MUST NOT silently report a clean answer, which is the original
   bug in a new costume.
5. **JSON parity.** The degraded state appears in `--format json`, not only in
   human output.
6. **Sub-case B regression.** The exact `!2181` / `prn4` shape: a handoff on
   the base branch, and a later `decision-record` from `governor-reviewer` on
   the same branch naming it in `references-to`. The obligation MUST NOT be
   reported as outstanding.
7. **Resolution ambiguity.** A referencing record that neither resolves nor
   continues MUST produce `UNCERTAIN`, not a silent resolve and not a silent
   outstanding. Guessing in either direction is the defect, not the fix.

Mutation requirement: removing the correlation logic MUST fail test 1 and only
test 1, demonstrating the test binds to the defect rather than to incidental
output.

## Rule-projection screen

Required by `workflow/documentation-rule-projection-rule.md` if this
introduces normative operating guidance in an adopter-facing page or runbook.
Preliminary assessment is that it does — cold-start guidance in `AGENTS.md`
and the session-resume reference tell agents to trust these surfaces, and that
instruction changes. Structured manifest to be authored at
`docs/reviews/rule-projection/coordination-visibility-correlation.yaml` before
Gate 2, with the strict `agov plan projection-check` output as Gate 2
evidence.

## Risks

- **Alarm fatigue.** If degradation fires too broadly, agents will learn to
  ignore it, reproducing the original failure with extra noise. Test 2 exists
  to bound this; the reviewer should attack that boundary specifically.
- **Cost.** Resolving branch-local records on every `handoff next` adds git
  work to the most frequently run command. Needs a measured cost check, not
  an assumption that it is cheap.
- **Correlation false negatives.** A record that resolves an obligation
  without naming it in `references-to` will not correlate. This fix is only as
  good as reference hygiene, and that limit should be stated in the adopter
  documentation rather than glossed.

## Rollback

Revert the command changes. No schema change, no data migration, no artifact
format change, so rollback is a code revert only.

## Evidence

- `docs/coordination/20260830T210228Z-coder-tester-1tm5.md`
- `docs/coordination/20260828T052452Z-governor-reviewer-dmtn.md`
- Nine coordination merge requests published 2026-08-30 after being stranded
  2–5 days (!2039, !2066, !2068, !2172, !2173, !2183, !2185, !2186, !2205).
- `agov coord visibility --base-branch main` reported 232 `bridge: missing`
  occurrences at the time of writing. That figure needs triage before it is
  cited as a count of live obligations; many are expected to be dead branches.
