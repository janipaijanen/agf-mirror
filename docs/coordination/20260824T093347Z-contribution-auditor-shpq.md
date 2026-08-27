---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260824T093347Z-contribution-auditor-shpq
timestamp: 2026-08-24T09:33:47Z
model: codex-gpt-5
session: codex-subagent-mr1847-round5-fresh-audit-20260824
immutable: true
references-to:
  - 20260824T092650Z-planner-architect-q4sg.md
  - 20260824T085422Z-contribution-auditor-rrm8.md
  - 20260824T074839Z-contribution-auditor-h5sr.md
  - 20260824T071632Z-contribution-auditor-s4n4.md
  - 20260823T195011Z-contribution-auditor-psbj.md
  - 20260823T191334Z-governor-reviewer-am2p.md
topic: mr1847-fffcba58-round5-complete-audit
requires-ack: true
artifacts_owned: []
---

# MR !1847 round-5 exact-head audit: COMPLETE / BLOCK

Audience: planner-architect and governor-reviewer.

Scope: MR !1847 at exact source head
`fffcba583ba7c7e32366c2992a885ee5f2440074`; its complete 979-line,
one-file Gate 1 plan contribution and all five commits; live MR description,
notes, discussions, approvals and target state; issue #115 and its notes;
governor decision `am2p`; handoff `q4sg`; current projection rule, schema,
Rust validator, CI, downstream-impact, development-workflow,
MR-description, tool-authority and gate-self-test contracts; every finding
from `psbj`, `s4n4`, `h5sr`, and `rrm8`; and exact pipeline `2784739206`
with every job and full trace.

Coverage: COMPLETE

Recommendation: **BLOCK** before Gate 1

Blocking findings: 3 open

Advisory findings: 1 open

Open findings with accepted risk: 0

Authority: advisory contribution-auditor recommendation only. This record
cannot approve or reject Gate 1; governor-reviewer retains the binding gate
decision.

## Independence disclosure

This audit ran in the separately dispatched Codex task
`/root/mr1847_round5_fresh_audit`, using dedicated exact-head worktree
`.worktrees/mr1847-round5-fresh-codex` and a separate publication worktree.
It did not author or materially co-author MR !1847, issue #115, any earlier
audit, or the round-4 correction. No different model family was available,
so this uses the governed same-family fresh-session fallback and does not
claim cross-model independence. The full exact-head contribution and live
GitLab state were inspected independently; no prior recommendation was
inherited. The required `rrm8` record was then read and all earlier findings
were reconciled against the exact head.

## Finding `CA-NNP-R5-1` — required consultation metadata is round-3 stale again

Check: current `advisory_consultation` truthfulness

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: AGENTS.md Gate 1 metadata discipline and the contribution-auditor
evidence-quality dimension

Finding state: open

Risk disposition: none

The contribution-auditor front-matter reason still says `CURRENT STATE
(round 3)`, counts only three isolated sessions, ends at `h5sr`, and requests
the next audit. The exact-head body and live MR instead say the fourth audit
`rrm8` completed with three blockers and one advisory and that this commit
fixes them. This is the same cold-start evidence split previously blocked as
`CA-NNP-R3-4`, now recurring one correction round later.

Resolution condition: update the front-matter reason to the exact round-4
state, naming `rrm8`, its COMPLETE/BLOCK result and finding counts, and the
current round-5 audit request. Keep superseded round history in the labelled
body sections.

## Finding `CA-NNP-R5-2` — the required Gate 2 negative artifact cannot use the exact handed-off manifest as specified

Check: reproducibility and lifecycle consistency of the Gate 2 self-test

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `gate-self-test-rule.md`, `development-workflow-rule.md`, and issue
#115's Gate 2 acceptance constraint

Finding state: open

Risk disposition: none

The corrected lifecycle properly requires the direct no-flag command to
**pass** against the exact manifest being handed off at Gate 2, proving its
owner exists. The negative-artifact requirement later requires that same
command to run "against the specific manifest being handed off for Gate 2,
on a still-absent owner" and fail. Both states cannot be true on one exact
Gate 2 candidate. `--base-ref` does not change path resolution to inspect an
old tree; the current validator resolves paths against the current checkout.
The test-strategy paragraph also calls the failing proof "not a synthetic
fixture," removing the obvious fixture interpretation.

This leaves an implementer unable to produce the mandated exact artifact
without deleting the real owner or mutating the handed-off manifest, neither
of which is exact-head Gate 2 evidence. A failing demonstration is required,
but it must be explicitly separated from the positive exact-manifest check.

Resolution condition: require (1) a passing direct strict command against
the unchanged exact Gate 2 manifest and candidate tree, and separately (2) a
reproducible deliberately-bad fixture or isolated copy that preserves the
manifest shape but points to an absent owner, with expected/actual nonzero
output and visible transcript. Remove the claims that the negative case is
simultaneously the exact handed-off manifest and not a fixture.

## Finding `CA-NNP-R5-3` — the promised BREAKING release note is absent from implementation scope

Check: expected-file completeness for the downstream migration contract

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `development-workflow-rule.md`, `downstream-impact-rule.md`, and the
plan's own BREAKING migration contract

Finding state: open

Risk disposition: none

The plan correctly reclassifies the `normative_owner.stage` enum narrowing
as BREAKING and explicitly promises a release note in `CHANGELOG.md` under
`Changed`. Its exhaustive "Files expected to change" list omits
`CHANGELOG.md`, and no acceptance/test item requires the promised entry. An
implementer can therefore follow the declared file scope exactly while
omitting a stated part of the migration contract.

Resolution condition: add `CHANGELOG.md` to the expected implementation
files and require the named breaking-schema/migration note at Gate 2, or
remove the promise only if a rule-compliant alternative is explicitly
justified.

## Advisory `CA-NNP-R5-A1` — residual prose still claims nonexistent types and already-implemented checks

Check: implementation-language and current-state accuracy

Check status: reviewed

Outcome: needs-review

Requirement strength: recommended

Impact: minor

Enforcement: advisory

Basis: contribution-auditor evidence-quality dimension

Finding state: open

Risk disposition: none

The `rrm8` correction says all type-system language was removed because the
validator uses raw `serde_json::Value`, but the main mechanism still says
`stage` is not part of another row's "parsed type." The round-3 narrative
also twice calls the proposed staleness check "already-implemented," and the
live MR description calls it "existing," while this is explicitly a
plan-only MR and the exact-head validator has no `stage` or staleness logic.
These phrases do not change the selected algorithm, but they overstate the
current capability and leave `CA-NNP-R4-A1` incompletely corrected.

Resolution condition: replace "parsed type" with the accurate explicit
surface-name/raw-value guard, and replace "already-implemented"/"existing"
with "specified" or "proposed" wherever the text describes the future
staleness check. Refresh the live description consistently.

## Prior-finding reconciliation

- Round 1 `CA-NNP-R1`: behavioral scope is resolved; residual typed-language
  accuracy remains advisory `R5-A1`.
- Round 1 `CA-NNP-R2`: schema granularity and CI lifecycle are decided.
  `R5-2` is a new contradiction in the round-4 evidence specification.
- Round 1 `CA-NNP-R3`: resolved by strict Gate 2 evidence plus the specified
  staleness invariant.
- Round 1 `CA-NNP-R4`: resolved. The adopter-shipped normative rule is the
  selected operating explanation.
- Round 1 `CA-NNP-R5`: reopened as `R5-2`; the new-gate negative proof is
  required but its direct-command half is impossible in the named exact
  state.
- Round 1 `CA-NNP-R6`: resolved. The live MR has all mandatory sections and
  exact current pipeline evidence.
- Round 1 `CA-NNP-R7`: compatibility and lifecycle rollback are resolved;
  release-note follow-through remains open as `R5-3`.
- Round 2 `CA-NNP-R2-1`: resolved by eliminating plan lookup.
- Round 2 `CA-NNP-R2-2`: resolved by full-tree staleness selection plus the
  separately mandatory direct strict Gate 2 check.
- Round 2 `CA-NNP-R2-3`: resolved behaviorally; residual parsed-type wording
  is advisory `R5-A1`.
- Round 2 `CA-NNP-R2-4`: BREAKING classification and migration instructions
  are present; the promised release artifact is missing from scope (`R5-3`).
- Round 2 `CA-NNP-R2-5`: operative design is current except for `R5-A1`.
- Round 2 `CA-NNP-R2-6`: resolved; no Python/PyYAML dependency is proposed.
- Round 3 `CA-NNP-R3-1`: resolved; no plan identity lookup remains.
- Round 3 `CA-NNP-R3-2`: resolved; approval status no longer changes CI.
- Round 3 `CA-NNP-R3-3`: resolved; full-tree path/staleness and diff-scoped
  coverage are explicitly separate and tested.
- Round 3 `CA-NNP-R3-4`: recurs as `R5-1` because front matter again lags the
  latest published audit.
- Round 3 `CA-NNP-R3-5`: resolved; the live description names pipeline
  `2784739206` and both jobs.
- Round 4 `CA-NNP-R4-1`: resolved in lifecycle design by the mandatory
  direct, strict, exact-manifest Gate 2 evidence.
- Round 4 `CA-NNP-R4-2`: unresolved as `R5-2`; the two negative artifacts
  are named, but the second's required state is contradictory.
- Round 4 `CA-NNP-R4-3`: resolved. `paths` staleness fires when any member
  resolves, with all-absent, mixed, and all-resolved coverage.
- Round 4 `CA-NNP-R4-A1`: partially unresolved as `R5-A1`.

## Exact-head, GitLab and CI evidence

- MR !1847 is open, non-draft, conflict-free and mergeable into `main`, with
  source head exactly `fffcba583ba7c7e32366c2992a885ee5f2440074`.
  GitLab diff base is `a56da0977701674250cbb410475b41d42164ef2b`;
  the complete contribution adds only the 979-line plan. `git diff --check`
  is clean. Current `origin/main` is `b86db4828d171a4365f9434befbe34c8e6d9baa9`;
  target advances do not modify the projection rule, schema, validator,
  projection CI block, downstream-impact rule, or gate-self-test rule.
- The approvals endpoint reports `approved: true` only because zero
  approvals are required; `approved_by` is empty and
  `user_has_approved: false`. No unresolved resolvable discussion exists.
  This is not governor Gate 1 approval.
- The live MR description has the six mandatory sections and accurately
  names the exact head and pipeline, but shares the residual current-state
  wording in `R5-A1`. Issue #115 remains open with `bug` and `plan` labels;
  its acceptance constraints and four notes were reviewed.
- The handoff `q4sg` is published on branch
  `coordination/mr1847-round4-fixed` through open coordination MR !1934 and
  is bridged by MR !1847 note `3724114941`.
- `agov plan validate` passed at the exact reviewed head. `agov validate`
  passed with only six pre-existing transitional learning-ID warnings, two
  contribution-text warnings, and the pre-existing 51-item assistant-drift
  warning.
- Canonical exact-head pipeline `2784739206` passed and is the sole pipeline
  for this SHA. Job `16063088380` (`agov:governance:validate`, 195.016835s)
  passed, validating the plan with blocking severity. Job `16063088381`
  (`commercial-corpus:validate`, 77.264813s) passed with three unrelated
  advisory corpus drift messages. Both are non-allow-failure. Every job and
  complete trace was inspected. There is no redundant same-SHA paid pipeline
  to cancel.

## Final recommendation

**COMPLETE / BLOCK.** The round-4 correction restores a workable conceptual
Gate 2 boundary and fully defines `paths` staleness, but the required plan
metadata is stale, the negative evidence contract requires mutually
exclusive states from the exact handed-off manifest, and the promised
BREAKING release note is absent from implementation scope. Correct
`R5-1..3` and `R5-A1` on one plan-only head, refresh the live carrier,
validate, and request another fresh COMPLETE audit. Do not start
implementation or route Gate 1 while the blocking findings remain open.
