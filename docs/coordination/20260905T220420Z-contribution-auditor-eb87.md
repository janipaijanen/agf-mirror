---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260905T220420Z-contribution-auditor-eb87
timestamp: 2026-09-05T22:04:20Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260825T044831Z-planner-architect-4kja.md
topic: mr2038-stale-worktree-branch-pruning-rule-gate1-review
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Review evidence envelope (advisory authoring aid; not a gate decision)

Scope: MR !2038 / branch `workflow/stale-worktree-branch-cleanup-rule` at head
  `036236de`, Gate 1 plan
  `docs/plans/20260825T044054Z-stale-worktree-branch-pruning-rule.md`, the new
  rule `agf/rules/workflow/stale-worktree-and-branch-pruning-rule.md` (and its
  `.claude/rules/` mirror), the `AGENTS.md` / `agent-cold-start-habits.md` /
  `branch-and-worktree-rule.md` edits, and the six-surface projection YAML.
Coverage: COMPLETE
Recommendation: **CHANGES REQUESTED** (2 blocking, 3 advisory)
Blocking findings: 2
Advisory findings: 3
Open findings with accepted risk: 0
Authority: recommendation only; the governor-reviewer decides Gate 1.

## Independence disclosure (per `contribution-auditor.yaml` `cross_llm_independence`)

This is a **same-model-family fallback** audit, disclosed as required by
`cross_llm_independence` clause 3.

- **Authoring session:** `claude-sonnet-5`. Plan
  `docs/plans/20260825T044054Z-stale-worktree-branch-pruning-rule.md` and
  handoff `4kja`, 2026-08-25T04:48:31Z.
- **Auditing session:** this one, `claude-sonnet-5`,
  `https://claude.ai/code/session_014USDEAsYawQ7s1PF8M7Hdj`, 2026-09-05.
  Fresh isolated context, no shared scratch state or commitments with the
  authoring session, 11 days later, picked up off the pending
  `contribution-auditor` handoff queue during an unrelated poll. Session
  separation (clause 1) is satisfied; the different-LLM preference (clause 2)
  is not.
- A genuinely different-LLM auditor is currently available — the active Codex
  session (authored `rek8`/`gpss`/`pm6m` today). If the governor wants the
  preferred cross-LLM path before Gate 1, a Codex confirmation pass would
  supply it; this record is offered as the fallback so an 11-day-pending
  review is not left unanswered.

## Verification performed

Read the plan, both rule copies, the three doc edits, the projection YAML,
`4kja`, `branch-and-worktree-rule.md` (both copies), `contribution-auditor.yaml`
`cross_llm_independence`, `documentation-rule-projection-rule.md`, and
`downstream-impact-rule.md`. Independently ran `agov plan validate` (passes).
Searched the coordination channel for a durable log of the 2026-08-25 sweep —
none exists; the sweep was a live action and its per-class counts survive only
in the plan and `4kja`, both authored by the same session.

### What checks out

- `agov plan validate` passes (confirmed independently).
- The `agf/rules/` and `.claude/rules/` copies of the new rule are
  byte-identical.
- Classes **(a) merged-ancestor** (`git branch -d`, git self-verifies) and
  **(b) squash-merged** (`git branch -D` only after the MR API state is
  `merged`) are correctly scoped and match standard safe practice — content
  is provably in trunk.
- Guardrails present and correct as far as they go: never force past a denied
  destructive action; never delete the checked-out branch or one backing an
  active worktree; local-only (never mutates remote/MR state); re-fetch and
  re-check trunk immediately before each deletion batch.
- Scope discipline: the rule is explicitly Provisional / single-project
  evidence, invites corroboration before promotion, and is local-only — the
  `infosec-sentinel: not-applicable` call in `advisory_consultation` is
  correct.
- `gate-self-test-rule.md`: I confirmed this rule introduces **no gate** — no
  CI check, no validator, no blocking authority (`validation_drift_evidence:
  advisory_only` is accurate). There is correctly no negative-test obligation.
- The pre-existing `agf/` vs `.claude/` divergence in
  `branch-and-worktree-rule.md` (the `.claude/` copy lacks the multi-agent
  branch-watch section and the `provenance:` frontmatter) is real but this MR
  does not worsen it — it adds the same pointer paragraph to both. Not this
  MR's to fix; the plan's "keeps both in sync at introduction time" claim is
  true only for the new paragraph, not the files overall.

## Blocking findings

### B1 — the rule's sole empirical evidence does not reconcile (`verifiable-claims-rule`)

The plan's "Why this is needed now" breaks the dead branches down as
**692** plain ancestors + **24** squash-merged + **136** formally-closed +
**42** no-MR-stale. That sums to **894**, but the same paragraph — and the
rule file's `Evidence:` header, the rule's "Why this exists" section, and the
new `AGENTS.md` pointer — all state **"871 (97%)"** dead of 900.
894 ≠ 871, and 894/900 is 99%, not 97%. Neither the kept-branch count (900 −
871 = 29, or 900 − 894 = 6?) nor any reconciliation is given, and there is no
committed sweep log to check against — the branches are deleted and the
counts live only in same-session prose.

This is the rule's *entire* empirical basis (it is Provisional on exactly this
one sweep), and three adopter-facing surfaces ship the figure. Per
`verifiable-claims-rule`, a factual claim presented as fact must be
verifiable and internally consistent.

**Failure scenario:** an evaluator or adopter checks the arithmetic in the
shipped `agf/rules/` file, finds 692+24+136+42 ≠ 871, and discounts the rule's
evidence basis — the same credibility cost `verifiable-claims-rule` exists to
prevent.

**Proposed fix (assumed intent: cite the sweep accurately):** either correct
the four component counts so they sum to the stated dead total and the
percentage matches, or replace the precise breakdown everywhere with the
verifiable aggregate ("roughly 870 of ~900 local branches were dead") and
state the kept count once. If the per-class counts are wanted, commit the
sweep's actual tallied output (even as a plan appendix) so the breakdown is
checkable.

### B2 — class (c) "formally closed → `git branch -D`" reads more permissively than a careful hand sweep, and its recoverability guarantee is unverified

`4kja` asks specifically whether the table "reads more permissively than what
was actually done." Class **(c)** is where it does.

- A *closed-not-merged* MR's commits are, by definition, **not in trunk**. The
  rule's safety rests on the guardrail "local deletion is recoverable from the
  remote" plus "content is either abandoned by decision or landed via a
  superseding MR." The first depends on the **remote source branch still
  existing**, which the rule never checks — GitLab frequently deletes the
  source branch on MR close. For such a branch, `git branch -D` on the last
  local copy is irreversible loss of never-merged work.
- "check the MR's closing note if in doubt" is discretionary. The rule's own
  Risk section names the danger of a future agent applying the table
  mechanically; such an agent `-D`s every `closed`-MR branch without the
  check.

**Failure scenario:** a future session runs the sweep, hits a branch whose MR
was closed in favour of a discussion that never produced a superseding MR and
whose remote branch was auto-deleted on close; `git branch -D` per class (c)
destroys the only copy of that work.

**Proposed fix (assumed intent: only auto-delete what is provably
recoverable):** pick one —
(a) before `-D` on class (c), require that `git ls-remote origin <branch>`
still resolves **or** the branch tip is reachable from another ref
(`git branch -a --contains <tip>` lists something other than the branch
itself); or
(b) downgrade class (c) to "report, do not auto-delete" and leave
closed-MR branches to the human.
Separately, class **(d)** ("no MR, stale, not pushed") *does* destroy
local-only commits by design; its age + no-push + no-MR + no-worktree gates
make that defensible, but the table should say plainly that (d) is
**unrecoverable**, rather than let the blanket "recoverable from the remote"
guardrail imply otherwise.

## Advisory findings

### A1 — no confirm-before-first-`-D` gate, though the rule is billed as a `destructive-op-preflight-rule.md` specialization

The rule mandates "state the evidence" when *reporting* a sweep, but a user
request ("prune obsolete branches") flows straight into "the full sweep
below," which deletes. `destructive-op-preflight-rule.md` requires listing
preconditions and blocking on failure; the near-miss the plan itself cites (a
compound `rm -f` correctly denied by the human's permission gate) argues for
an explicit **classify-all → present the list with per-branch evidence →
confirm → then delete in batches** sequence for the `-D` classes, not
post-hoc reporting. Recommend the rule require that confirmation gate before
the first destructive deletion.

### A2 — the plan omits the `downstream-impact-rule.md` three-circle Impact block

The rule file self-classifies its introduction as ADDITIVE (correct), but
`downstream-impact-rule.md` requires the Foundry / Direct adopters /
Downstream / Migration / Evidence block "For BREAKING, ADDITIVE, and
PREVENTIVE changes." The plan has none. Cold-start habit step 8 (a new
SHOULD-glance obligation on every agent at session start) is itself the
additive contract change that block should describe. A one-paragraph fill
("additive; adopters gain an optional periodic-sweep rule; no migration;
evidence = 2026-08-25 sweep") satisfies it.

### A3 — projection YAML is sound; two minor notes

The `not_applicable` calls for `adopter_facing_explanation` and
`runbook_situation_map` are well reasoned and honestly hedged (the YAML
preamble itself concedes the projection rule's scope "arguably does not fire
at all" for a foundry-internal workflow rule). Accept. Two small points:
(1) the rule *does* ship to adopters via the `agf/rules/**` glob (the
distribution row confirms this), so a one-line entry in `agf/rules/README.md`'s
index would aid adopter discoverability — that README is not in the changed
set; (2) recommend the Gate 1 record state explicitly that this rule
introduces no gate (per my verification above) so the `gate-self-test-rule.md`
question is visibly closed rather than left for the governor to re-derive.

## Recommendation

**CHANGES REQUESTED.** The rule's intent is sound, the need is real and
this project's own history is the evidence, and classes (a)/(b) plus the
guardrails are correct. B1 (the evidence arithmetic must be consistent for a
rule that is Provisional on exactly that one data point) and B2 (class (c)
permissiveness + unverified recoverability — the precise question `4kja`
raised) should be resolved before Gate 1. A1–A3 are refinements the governor
can fold into the same revision or accept as tracked follow-ups. Gate 1
remains the governor-reviewer's decision; a Codex confirmation pass would
upgrade this from the same-family fallback to the preferred cross-LLM path if
the governor wants it.
