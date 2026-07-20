---
id: multi-agent-coordination-experiment
status: pending-gate1
topic: multi-agent-coordination-experiment
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: experiment protocol and sample-repo scaffolding only; no production secrets, auth, or security-boundary code
  - role: contribution-auditor
    status: consulted
    reason: this plan proposes a new evidence-generation methodology whose output will update a public framework claim's evidence status; contribution-auditor review of the protocol's soundness (pre-registration rigor, anonymization of any sample-repo content, avoidance of cherry-picked framing) is warranted before execution begins
---

# Plan: Documented multi-agent coordination experiment (issue #38)

**Date:** 2026-07-20T07:55:03Z
**Status:** PENDING GATE 1 APPROVAL

## Intent

Design and pre-register one controlled experiment comparing the same coding
task executed by N parallel AI agents, with AGF coordination (Condition A)
versus without it / ad-hoc coordination (Condition B). Measure conflicts,
rework, gate outcomes, wall-clock time, and token cost. Publish the protocol
*before* running it, publish raw artifacts and results regardless of
outcome, and update the framework's claim wording to match measured
evidence status — including a null or negative result.

## Why this needs a real plan, not a fast-track note

Unlike the other four items in this cluster, this is a methodology-design
task: it requires defining a representative task, two experimental
conditions, a metrics protocol, and a sample repository — before any
"implementation" in the usual code-change sense exists. It also touches a
public, marketed claim (evidence status moves from Theoretical toward
Empirically informed only if the experiment is designed and executed
soundly), so the design itself needs Gate 1 scrutiny, not just the eventual
write-up.

## Current claim under test

`agf/docs/site/index.md`, "Decisions you can reconstruct" (lines 143-148):

> "Because the channel is just files in a repository, it works whether one
> assistant fills all roles across sessions or several assistants
> collaborate in parallel."

Per `agf/docs/site/reference/maturity-vocabulary.md`'s Evidence Status
ladder (Empirically validated / Empirically informed / Theoretical), this
claim currently has no evidence-status label attached to it at all — it
reads as a bare assertion. The parallel-collaboration half of the claim is,
by the framework's own admission on `agf/docs/site/index.md` ("Evidence,
honestly stated"), unexercised: the framework's operational history to date
is predominantly single-assistant-across-sessions, not concurrently
multi-agent. This experiment is the first attempt to generate direct
evidence for the parallel half specifically.

## Scope

`docs/evidence/` (new top-level evidence directory, consistent with this
repo's existing convention that "project working evidence lives under root
`docs/`" — see `agf/docs/site/docs/concepts/evidence-and-learnings.md`) plus
a disposable sample repository. Framework claim wording in
`agf/docs/site/index.md` and, if warranted by result,
`agf/docs/site/reference/maturity-vocabulary.md` usage examples or
`agf/docs/site/future/evolution.md`.

## Out of scope (per issue #38)

- Statistical generality: one experiment can move the claim's evidence
  status from Theoretical to Empirically informed at most (per the
  five-level maturity model referenced in the issue and this repo's own
  Evidence Status ladder) — never to Empirically validated, which requires
  multiple independent projects with consistent results.
- Cross-vendor comparisons (this experiment is agent-count and
  coordination-protocol variation, not a comparison across different AI
  vendors/models).

## Experiment design

### Representative task

A medium-size feature, sized so that natural decomposition into 2-3
independent-ish work units is plausible but conflicts are also plausible if
coordination fails. Candidate: "add CSV export to an existing list view,"
mirroring the exact example already used illustratively in this repo's own
`agent-proactive-analysis-rule.md`. Decompose into three units with a
realistic shared-file collision risk:
1. Export route/handler (new file, low collision risk)
2. A shared list/query helper the export route also needs (collision risk:
   both the export unit and an existing-feature unit may want to touch it)
3. Tests + a shared fixture file (collision risk: multiple agents adding
   fixtures to the same file)

The shared-file collision points are deliberately designed in, not left to
chance — otherwise a "no conflicts observed" result is uninformative (it
could mean coordination worked, or it could mean the task never created an
opportunity for conflict).

### Sample repository

A small, disposable, purpose-built repo (not this repo, not a real
adopter's repo) — minimal web app skeleton sized to make the task above
meaningful. Built fresh for this experiment; committed to `docs/evidence/`
as a fixture or referenced as a separate throwaway repo linked from the
protocol. Must be small enough that both conditions can be re-run
inexpensively if the first run is inconclusive.

### Conditions

- **Condition A (AGF):** N=3 parallel agents in separate worktrees, each
  assigned an AGF role (e.g., two planner-architect/coder-tester agents
  splitting units 1 and 2, one on unit 3), using the coordination channel,
  Gate 1/Gate 2, and this repo's rule set as-is.
- **Condition B (ad hoc):** The same 3 agents, same task decomposition,
  same underlying model(s), but no coordination channel, no gates, no rule
  set — agents work from the bare task description with whatever ad-hoc
  communication the human orchestrator provides (e.g., a shared chat
  thread, or none).

Both conditions use the same starting commit of the sample repo, the same
task decomposition, and (as closely as practically achievable) the same
underlying model/version, to isolate the coordination-mechanism variable.

### Metrics (recorded for both conditions)

| Metric | Definition |
|---|---|
| Merge conflicts | Count of git merge conflicts encountered when integrating the parallel units |
| Duplicated work | Count of functionally-overlapping changes independently produced by 2+ agents for the same requirement |
| Discarded work | Count of complete units/commits that were thrown away rather than merged |
| Gate violations | Count of instances where a change bypassed or violated a rule/gate that was supposed to apply (Condition A only, by definition; recorded as N/A with justification for Condition B) |
| Wall-clock time | From task assignment to final merge, per condition |
| Token cost | Aggregate token usage across all agents, per condition, if obtainable from the runtime |

### Acceptance criteria (per issue #38)

- Experiment protocol (this plan, or a dedicated protocol document derived
  from it) committed to `docs/evidence/` **before** execution begins
  (pre-registration).
- Raw coordination artifacts (coordination channel files, git logs, commit
  diffs, timing logs) from both conditions published in
  `docs/evidence/multi-agent-coordination-experiment/`.
- Results published, including the metrics table above, regardless of
  outcome.
- `agf/docs/site/index.md`'s claim wording updated to carry an explicit
  Evidence Status label per the maturity vocabulary, reflecting the
  measured outcome — including if the outcome is negative or null (e.g.,
  "AGF coordination showed no measurable reduction in conflicts for this
  task size" is a publishable, acceptable result under this plan; the
  acceptance criterion is honesty of the update, not a particular result).

## Affected files (initial scaffolding only — this plan does not implement the experiment itself)

| File | Action |
|---|---|
| `docs/evidence/multi-agent-coordination-experiment/protocol.md` | Create — the pre-registered protocol: task spec, conditions, metrics, sample-repo reference, analysis plan, written and committed before any run |
| `docs/evidence/multi-agent-coordination-experiment/sample-repo/` or a linked external throwaway repo reference | Create — the fixture repo used by both conditions |
| `docs/evidence/multi-agent-coordination-experiment/results.md` | Create, after execution — raw metrics, artifacts index, honest discussion of limitations (single task, single trial, small N) |
| `agf/docs/site/index.md` | Update the parallel-collaboration claim with an Evidence Status label once results land |
| `CHANGELOG.md` | Add an entry once results are published (evidence changes are user-visible per the downstream-impact-rule's ADDITIVE category) |

This Gate 1 approval covers the protocol design and scaffolding above. A
follow-up Gate 2 covers the actual experiment execution and results
write-up, since execution outcomes cannot be known at plan time.

## Migration needed

No.

## Test strategy

This is a methodology artifact, not code, so "test strategy" means
protocol soundness checks rather than automated tests:

- Pre-registration check: protocol committed and reviewed (Gate 1) before
  any experimental run starts, so metrics can't be redefined post-hoc to
  fit a preferred outcome.
- Reproducibility check: another reviewer, given only the protocol
  document and the sample repo, should be able to re-run both conditions
  and produce comparably-shaped raw artifacts (not necessarily identical
  results, given LLM non-determinism, but the same artifact categories).
- Anonymization check (contribution-auditor scope): the sample repo and
  any published artifacts must not leak real project data — it is a
  purpose-built fixture, so this should be trivially satisfiable, but
  confirm no copy-pasted real-repo content leaked in during construction.
- Claim-update check (verifiable-claims-rule, implementation-honesty-rule):
  the post-experiment claim wording must carry hedging language
  proportional to N=1 trial, single task, single model family — "one
  controlled experiment suggests..." not "AGF reduces conflicts."

## Risks

| Risk | Mitigation |
|---|---|
| Task not representative enough to generalize | Documented explicitly as a limitation in `results.md`; acceptance criteria only require honest reporting, not generality |
| Non-determinism makes single-trial results noisy | Report as a limitation; consider (optionally, not required for acceptance) a second trial if resources allow, documented as N=2 rather than silently averaged |
| Negative/null result perceived as undermining a marketed differentiator | Explicitly pre-committed to in the acceptance criteria — the plan requires updating claim wording "including if the outcome is negative or null"; this is a feature of the plan, not a risk to work around |
| Sample repo scope creep (turns into a real mini-product) | Keep the sample repo minimal — only as much as the task decomposition requires; simplicity-rule applies here too |

## Rollback

If the experiment is abandoned before execution, no rollback is needed
beyond leaving the protocol document in place as a documented but
unexecuted design (itself a Level 1 Planned artifact per the maturity
vocabulary — honest either way). If executed and the write-up is later
found methodologically unsound, retract via a follow-up learning record and
revert the claim-wording update, per this repo's usual evidence-correction
path rather than deleting the raw artifacts (which stay as a documented
negative learning).
