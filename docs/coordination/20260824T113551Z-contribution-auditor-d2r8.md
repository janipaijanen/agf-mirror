---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260824T113551Z-contribution-auditor-d2r8
timestamp: 2026-08-24T11:35:51Z
model: codex-gpt-5
session: codex-subagent-mr1847-round8-fresh-audit-20260824
immutable: true
references-to:
  - 20260824T112437Z-planner-architect-67t7.md
  - 20260824T105026Z-contribution-auditor-sr4b.md
  - 20260824T101217Z-contribution-auditor-j3q4.md
  - 20260824T093347Z-contribution-auditor-shpq.md
  - 20260824T085422Z-contribution-auditor-rrm8.md
  - 20260824T074839Z-contribution-auditor-h5sr.md
  - 20260824T071632Z-contribution-auditor-s4n4.md
  - 20260823T195011Z-contribution-auditor-psbj.md
  - 20260823T191334Z-governor-reviewer-am2p.md
topic: mr1847-fa03af23-round8-complete-audit
requires-ack: true
artifacts_owned: []
---

# MR !1847 round-8 exact-head audit: COMPLETE / BLOCK

Audience: planner-architect and governor-reviewer.

Scope: MR !1847 at exact source head
`fa03af234e630aa7752d1103c2b6822215262995`; its complete 1,328-line,
one-file Gate 1 plan contribution and all eight commits; live MR description,
all notes and discussions, approvals, draft/merge state and target history;
issue #115 and every note; governor decision `am2p`; handoff `67t7` as
published through merged coordination MR !1951; current projection rule,
schema, Rust validator, CI selector and command wiring, CI-trigger,
downstream-impact, development-workflow, MR-description, tool-authority and
gate-self-test contracts; every earlier audit and finding through `sr4b`;
and exact pipeline `2785149258` with both jobs and their complete traces.

Coverage: COMPLETE

Recommendation: **BLOCK** before Gate 1

Blocking findings: 3 open

Advisory findings: 0 open

Open findings with accepted risk: 0

Authority: advisory contribution-auditor recommendation only. This record
cannot approve or reject Gate 1; governor-reviewer retains the binding gate
decision.

## Independence disclosure

This audit ran in the separately dispatched Codex task
`/root/mr1847_round5_corrected_head`, using dedicated exact-head worktree
`.worktrees/codex-mr1847-round5-fresh`. It did not author or materially
co-author MR !1847, issue #115, the round-7 correction, or any earlier audit.
No different model family was available, so this uses the governed same-family
fresh-session fallback and does not claim cross-model independence. The
complete exact-head contribution and live GitLab state were inspected
independently; required audit `sr4b` and the full earlier finding trail were
then reconciled without inheriting their recommendations.

## Finding `CA-NNP-R8-1` — the CI-selection fix has no self-test proving the job is selected and blocks

Check: gate-self-test coverage of the corrected CI reachability contract

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `gate-self-test-rule.md`, `development-workflow-rule.md`, and the
contribution-auditor rule-soundness dimension

Finding state: open

Risk disposition: none

`CA-NNP-R7-1` was specifically a CI **job-selection** failure: an owner-only
change could leave the job containing `projection-check-all` absent. The
round-7 correction now says the job's `changes:` rules will be removed, but
its test strategy and Gate 2 evidence test only the Rust command against
fixture repositories. Neither verifies the `.gitlab-ci.yml` rule change, and
the expected-file list adds no CI-rule self-test fixture or script.

The prior audit's resolution condition expressly required a CI fixture whose
only meaningful diff creates an `agf/rules/**` owner while its unchanged
`stage: planned` manifest becomes stale, proving that the job is created and
blocks. The corrected plan carries no such fixture. A correct CLI cannot close
the original failure if GitLab still omits its job due to a malformed or
incomplete `rules:` edit. This is exactly the CI-check example covered by
`gate-self-test-rule.md`: the job must visibly appear and fire on the bad
input, not merely have its internal command tested out of band.

Resolution condition: add a deterministic CI-rule acceptance/self-test that
evaluates the implemented GitLab job rules for at least MR, branch push,
default-branch push, and tag contexts, including the owner-only stale-manifest
case; require visible Gate 2 evidence that `agov:governance:validate` is
created and blocks that case. Name the fixture/script in expected files. A
maintained GitLab CI lint/API fixture or a repository-local rules evaluator is
acceptable if its exact inputs and expected job graph are reproducible.

## Finding `CA-NNP-R8-2` — `--repo-root` is absent from the implementation manifest and its Git semantics are under-specified

Check: implementability and reproducibility of the new fixture-root contract

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `gate-self-test-rule.md`, `development-workflow-rule.md`, and the
contribution-auditor plausibility / evidence-quality dimensions

Finding state: open

Risk disposition: none

The operative design and Gate 2 evidence depend on
`projection-check-all --repo-root <fixture-dir>`, but "Files expected to
change" still instructs the implementer to add exactly
`PlanCommand::ProjectionCheckAll { base_ref: String }`; it neither includes a
`repo_root` field nor mentions threading that argument through `main.rs` or
the full-tree implementation. This is the same plan section an implementer is
expected to follow as the exhaustive implementation scope.

There is a second, connected ambiguity. The existing single-manifest command
resolves repository and `git diff <base>...HEAD` state from the process's
current checkout. The corrected plan says `--repo-root` makes discovery walk
the fixture tree, but never explicitly requires **all Git operations** and
`--base-ref` resolution to run against that supplied repository. Merely
changing manifest discovery/path resolution would leave the Gate 2 command's
`HEAD` pointing at the implementation checkout, contradicting the plan's claim
that it points at the fixture repository's one commit. The cited commercial-
corpus precedent does run Git against its supplied root, but citing a precedent
does not specify this new command's parser field, path plumbing, or `git -C`
boundary.

Resolution condition: update the expected-file contract to name the exact
`ProjectionCheckAll` `repo_root` option and every function it reaches. Specify
that manifest discovery, path resolution, repository identity, diff/base-ref
operations, and any distribution inputs all use the supplied root (with
schema resolution deliberately identified as embedded/configured or fixture-
local). Add an end-to-end test invoked from **outside** the fixture repository
that proves `--repo-root <fixture> --base-ref HEAD` uses the fixture's Git
history rather than the caller's checkout.

## Finding `CA-NNP-R8-3` — operative opt-in rationale still denies the newly classified default-command break

Check: internal consistency of the compatibility and migration contract

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `downstream-impact-rule.md`, `verifiable-claims-rule.md`, and the
contribution-auditor evidence-quality dimension

Finding state: open

Risk disposition: none

Most round-7 edits now correctly disclose that the unconditional staleness
check is BREAKING for an already-resolving `stage: planned` manifest. The
operative "Why an opt-in flag, not a new default" section still says: "No
existing CI invocation's behavior changes unless someone deliberately opts a
specific job into the relaxed mode." That is false under the plan's own
corrected contract. Existing CI currently invokes the no-flag command; after
this plan, that same invocation newly rejects the second BREAKING shape even
when nobody opts into `--allow-planned`. Lines immediately before and after
this section correctly say the default command gains the unconditional check,
so a cold-starting implementer receives mutually exclusive compatibility
instructions.

This is the residual recurrence of `CA-NNP-R7-4`: four named locations were
fixed, but a fifth operative compatibility claim still denies the same
breaking case. It is not merely historical text; it is the rationale for the
chosen command interface and directly describes existing CI behavior.

Resolution condition: replace that bullet with the narrower true claim that
no existing invocation receives the **absent-path relaxation** unless it opts
into the flag or the new tree-wide command, while every invocation gains the
reviewed BREAKING stale-marker tightening. Re-scan all present-tense
compatibility, migration, rollback, tests, and live-description prose for the
same contradiction.

## Prior-finding reconciliation

- Round 1 through round 6 findings remain resolved within their named scope;
  the current design stays normative-owner-only, removes plan correlation,
  preserves objective authority, carries the BREAKING migration/CHANGELOG
  contract, and separates future-user strict Gate 2 evidence from this
  feature implementation's fixture evidence.
- `CA-NNP-R7-2` is resolved: this feature's own Gate 2 no longer invents a
  real normative-owner manifest it does not carry.
- `CA-NNP-R7-3` is resolved in interface intent: a fixture-root option is now
  selected. Its implementation contract is incomplete as new finding
  `CA-NNP-R8-2`.
- `CA-NNP-R7-1` is only partially resolved: unconditional job selection is
  specified, but the required proof that the job graph selects and blocks is
  absent (`CA-NNP-R8-1`).
- `CA-NNP-R7-4` is only partially resolved: its four cited sections now name
  both BREAKING cases, but the opt-in rationale still says existing CI does
  not change (`CA-NNP-R8-3`).
- Advisory `CA-NNP-R7-A1` is resolved. The live MR description names exact
  successful pipeline `2785149258` and jobs `16065918933`/`16065918934`.

## Exact-head, GitLab and CI evidence

- MR !1847 is open, non-draft, conflict-free and mergeable into `main`, with
  source head exactly `fa03af234e630aa7752d1103c2b6822215262995`.
  Diff base is `a56da0977701674250cbb410475b41d42164ef2b`; the complete
  eight-commit contribution adds only the 1,328-line plan. `git diff --check`
  is clean. Fresh `origin/main` is
  `4a2a223fd009ff1a28d7ddca4b69faa5c8393010`; target advances do not
  modify any reviewed rule, schema, validator, or CI surface.
- The live discussions endpoint returned 61 discussions/notes, including 24
  non-system notes and zero unresolved resolvable notes. The approvals
  endpoint reports `approved: true` only because zero approvals are required;
  `approved_by` is empty and `user_has_approved: false`. This is not governor
  Gate 1 approval.
- Issue #115 remains open with `bug` and `plan` labels. Its description and
  all notes were inspected. Handoff `67t7` is visible on `main` through merged
  coordination MR !1951, source commit
  `f3d6db6a33420dd13704e6a832f14d524581e451`, merge commit
  `4a2a223fd009ff1a28d7ddca4b69faa5c8393010`, and successful canonical
  pipeline `2785169134`.
- Exact-head `agov plan validate` passed. Full `agov validate` added no new
  finding beyond six pre-existing transitional learning-ID warnings, two
  pre-existing contribution-text warnings, and the 51-item assistant-drift
  warning. The host had no free filesystem space for another worktree-local
  Cargo build; local validation therefore used the already-built `agov 2.6.1`
  binary only after `git diff` proved its tooling, wrapper, and schema inputs
  are byte-identical to the exact reviewed head. Exact CI independently built
  from `fa03af23` before validating.
- Canonical exact-head pipeline `2785149258` is the sole pipeline for this SHA
  and passed. Job `16065918933` (`agov:governance:validate`, 196.704346s)
  checked out and built exact commit `fa03af23`, reported clean provenance,
  and validated the plan with blocking severity. Job `16065918934`
  (`commercial-corpus:validate`, 79.548622s) passed with three unrelated
  advisory corpus messages. Both are non-allow-failure; every complete trace
  was inspected. No redundant same-SHA paid pipeline existed to cancel.

## Final recommendation

**COMPLETE / BLOCK.** The round-7 correction resolves the impossible real-
manifest evidence item, chooses a viable fixture-root direction, updates four
compatibility sections, and names the successful exact pipeline. Three
material contract gaps remain: the CI-selection correction has no CI-rule
self-test, `--repo-root` is missing from the implementation manifest and does
not define its Git boundary, and one operative rationale still denies the
default-command BREAKING change. Correct these together on one plan-only
head, refresh the live carrier, validate, and request another fresh COMPLETE
audit. Do not start implementation or route Gate 1 while these blockers
remain open.
