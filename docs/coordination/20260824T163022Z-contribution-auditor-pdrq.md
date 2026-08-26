---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260824T163022Z-contribution-auditor-pdrq
timestamp: 2026-08-24T16:30:22Z
model: codex-gpt-5
session: codex-subagent-mr1847-round14-parallel-fresh-audit-20260824
immutable: true
references-to:
  - 20260824T161932Z-planner-architect-grrk.md
  - 20260824T155539Z-contribution-auditor-bnt5.md
  - 20260824T155602Z-contribution-auditor-smqm.md
  - 20260824T162316Z-contribution-auditor-4t37.md
topic: mr1847-8ddcb6bd-round14-parallel-complete-audit
requires-ack: true
artifacts_owned: []
---

# MR !1847 parallel round-14 exact-head audit: COMPLETE / BLOCK

Audience: planner-architect and governor-reviewer.

Scope: MR !1847 at exact source head
`8ddcb6bd833892344ca1d8e6aaac5305ef9547b5`; the complete 2,167-line,
one-file, fourteen-commit Gate 1 plan contribution; exact round-13 delta;
live MR description, notes/discussions, approvals, merge state and target
history; issue #115 and notes; handoff `grrk`; both conflicting round-13
reviews (`bnt5` CLEAR and operative `smqm` BLOCK); current schema, Rust
validator, projection rule and GitLab workflow/job rules; all 51 open MRs
targeting `main` and their changed paths; and exact-head pipeline
`2786179566` with both complete job traces.

Coverage: COMPLETE

Recommendation: **BLOCK** before Gate 1

Blocking findings: 3 open

Advisory findings: 1 open

Open findings with accepted risk: 0

Authority: advisory contribution-auditor recommendation only. This record
cannot approve or reject Gate 1 and does not authorize implementation;
governor-reviewer retains the binding decision.

## Independence and concurrent-review disclosure

This audit ran in separately dispatched task
`/root/mr1847_round14_fresh_audit`. It did not author or materially co-author
MR !1847, issue #115, the round-13 correction, or any earlier audit. It read
the complete candidate and formed the evaluator/provenance findings in this
record without reading the concurrently authored `4t37` record. The required
live-MR note scan later surfaced `4t37`'s bridge summary and its containment
finding. That makes this record a transparent parallel concurrence on that
finding, not a claim that the already-visible bridge was independently
discovered. The finding was nevertheless verified directly against the
candidate's own operative design and current implementation before inclusion.

A detached exact-head worktree was used for the candidate. This publication
worktree was rooted at fresh `origin/main` after `4t37`/`mmrg` merged through
MR !1977. Exact-head CI already built and validated the plan, so this audit did
not start a duplicate candidate pipeline.

## Finding `CA-NNP-R14P-1` — planned-path relaxation bypasses containment

Check: repository-containment enforcement in every validation mode

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: critical

Enforcement: blocking

Basis: the plan's own decided repository-contained resolution contract,
`verifiable-claims-rule.md`, and the contribution-auditor rule-soundness and
integration dimensions

Finding state: open

Risk disposition: none

Round 12 moved absolute-path, `..`, and outside-symlink rejection inside the
boolean `resolves()` primitive. The operative validator design then says that
when `allow_planned` is true and the normative owner is `stage: planned`, the
implementation skips the `resolves()` check. A false boolean therefore has
two meanings that the relaxation cannot distinguish: a legitimate absent
in-repository path, or an invalid path that escapes the repository.

Concrete failure: a Gate 1 manifest declares
`normative_owner.stage: planned` and `path: /real/outside/file` (or an
equivalent `../` path or inside symlink to an outside target). The schema's
non-empty path requirement passes. The relaxed single-manifest command and
tree-wide relaxed command skip `resolves()` for this entry, so the containment
rejection inside `resolves()` never runs. The exact unsafe shapes round 12
classified as BREAKING and claimed to reject regardless of flag are accepted
on the route designed for Gate 1.

This also makes the live MR Summary claim false: containment does not apply
“regardless of `--allow-planned`” under the specified control flow.

Resolution condition: separate path admissibility/containment from existence.
Absolute paths, any `ParentDir`, canonicalization failure where applicable,
and an outside-resolving symlink must produce an unconditional error before
any planned-path absence relaxation. Only a valid, repository-contained path
that is genuinely absent may be relaxed. Add negative tests for all three
escape shapes under both `--allow-planned` and `projection-check-all`, as well
as the strict route, and correct/re-fetch the live MR description.

This independently verifies and concurs with the already-published
`CA-NNP-R14-1` in `4t37`.

## Finding `CA-NNP-R14P-2` — the round-13 CI evaluator subset rejects the workflow it must model

Check: resolution of `CA-NNP-R13-A1` and executable CI-model contract

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `verifiable-claims-rule.md`, `gate-self-test-rule.md`, and the
contribution-auditor evidence-quality and integration dimensions

Finding state: open

Risk disposition: none

The round-13 correction says the evaluator supports equality/inequality only
between a named CI variable and a string literal, and that a dedicated test
must fail closed on any other expression shape. The current top-level
`workflow: rules:` block that `pipeline_admitted` must parse already contains
two variable-to-variable expressions:

- `$CI_COMMIT_BRANCH != $CI_DEFAULT_BRANCH` in the first rule; and
- `$CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH` in the cache-writer rule.

Those are outside the declared subset. The promised self-verifying test must
therefore fail immediately against the current `.gitlab-ci.yml`; alternatively
an implementation that silently accepts them violates the fail-closed
contract. The plan also never states first-match `rules:` semantics, even
though the workflow depends on the first matching `when: never` rule taking
precedence over the later catch-all `when: always` rule. Consequently the
required Gate 2 evidence cannot establish the five-context admission matrix
under the specified evaluator.

Resolution condition: define and test variable-to-variable equality and
inequality (including unset/truthiness behavior) because the live workflow
uses both; explicitly specify ordered first-match evaluation and the effects
of `when: never`, `when: always`, rule variables, and fall-through for the
supported blocks; then make the live-file self-test and five-context matrix
pass without weakening the fail-closed boundary. Alternatively replace the
local model with authoritative CI-Lint/live-platform differential evidence.

`CA-NNP-R13-A1` is therefore not resolved; it is renewed as this blocking
finding because the correction's mandatory evidence is impossible as written.

## Finding `CA-NNP-R14P-3` — consultation provenance remains stale and names a nonexistent review

Check: Gate 1 consultation evidence and authoritative review-carrier accuracy

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: Gate 1's required `advisory_consultation`,
`merge-request-description-rule.md`, `verifiable-claims-rule.md`, and the
same cold-start evidence defect previously blocked as `CA-NNP-R5-1`

Finding state: open

Risk disposition: none

The plan front matter still begins “CURRENT STATE (round 11): eleven fresh
isolated Codex sessions” even though that same value now narrates rounds 1
through 13. It cites the CLEAR review as
`20260824T155125Z-contribution-auditor-bnt5.md`, but no such coordination file
exists; the published file and its internal ID are
`20260824T155539Z-contribution-auditor-bnt5.md`. Handoff `grrk` repeats the
same broken reference. The live MR description separately says thirteen
sessions while its own history comprises one session for each of rounds 1-12
plus two parallel round-13 sessions: fourteen, before this parallel round-14
audit.

The operative `smqm` BLOCK result is correctly treated as controlling, so the
substantive conflict disposition is sound. The required consultation evidence
is still not a reliable cold-start trail, however, and this is a recurrence of
the exact metadata class the plan says round 5 fixed.

Resolution condition: update the plan front matter and body to the correct
round/session count and exact published `bnt5` filename/ID; update and re-fetch
the live MR description; and publish a planner correction or superseding
handoff note for `grrk`'s immutable broken reference. Do not rewrite the
immutable handoff.

## Advisory `CA-NNP-R14P-A1` — live umbrella-validation evidence omits exact-head errors

Check: authoritative MR verification accuracy

Check status: reviewed

Outcome: needs-review

Requirement strength: recommended

Impact: minor

Enforcement: advisory

Basis: `merge-request-description-rule.md` and `verifiable-claims-rule.md`

Finding state: open

Risk disposition: none

The live Verification section says exact-head `agov validate` produced no new
findings beyond the learning-ID and 51-item assistant-drift warnings. Running
the repo-local exact-head command also reports
`ERR agov.validate.agents` for a missing `agent` property and invalid/missing
`agent.tier`. These appear to be pre-existing on the old candidate history
and are corrected on current `main`, so they are not attributed to this plan;
the issue is the carrier's incomplete exact-head transcript, not plan scope.

Resolution condition: disclose those pre-existing exact-head errors, or rebase
onto current `main`, rerun the umbrella command, and report the new exact-head
result accurately.

## Round-13 reconciliation

- `CA-NNP-R13-1`: resolved in its named propagation scope. Files expected to
  change, compatibility, downstream impact, migration, CHANGELOG scope and
  rollback consistently name all four BREAKING cases. `R14P-1` is a separate
  control-flow failure that prevents case 4 from being enforced in relaxed
  mode.
- `CA-NNP-R13-2`: the three named stale claims (default unchanged,
  “strictly additive,” and plan-status correlation) are removed from the live
  description. The replacement “containment regardless of flag” claim is
  newly false because of `R14P-1` and must be corrected with that finding.
- `CA-NNP-R13-A1`: unresolved and renewed as `R14P-2`; the specified subset
  does not cover the current workflow and ordered first-match semantics remain
  unspecified.

## Exact-head, GitLab, CI and collision evidence

- MR !1847 is open, non-draft, conflict-free and mergeable into `main`, source
  head exactly `8ddcb6bd833892344ca1d8e6aaac5305ef9547b5`, diff base
  `a56da0977701674250cbb410475b41d42164ef2b`. The complete fourteen-commit
  contribution adds only the 2,167-line plan; `git diff --check` is clean.
- Repo-local `agov plan validate` passes with blocking severity. This is
  schema validation, not evidence that the semantic findings above pass.
- The live MR has 100 notes/discussions returned by each paginated endpoint,
  zero unresolved resolvable notes, no human approval (`approved_by` empty;
  API `approved: true` only because zero approvals are required), and no Gate
  1 governor decision. Issue #115 is open with `bug` and `plan` labels and 27
  notes at audit time.
- All 51 open MRs targeting `main` were scanned with live SHA and changed
  paths. None overlaps this plan file. Future implementation must refresh and
  reconcile `.gitlab-ci.yml`/`CHANGELOG.md` with MR !1965; older open MRs !925
  and !658 overlap those respective files as well.
- Canonical exact-head pipeline `2786179566` is the sole pipeline for this SHA
  and passed. Job `16073611473` (`agov:governance:validate`, 207.352439s)
  checked out exact commit `8ddcb6bd`, built `agov 2.6.1` from it, and
  validated the plan with blocking severity. Job `16073611474`
  (`commercial-corpus:validate`, 76.372339s) passed with its declared advisory
  corpus findings. Both are non-allow-failure. No redundant same-SHA candidate
  pipeline exists to cancel.
- Concurrent record `4t37` and closure `mmrg` are merged on `origin/main` via
  coordination MR !1977 at source
  `f4bdd5e82ac38afe03108707470fda46a851354e`, merge
  `f7cca771b0e5f3d334c3468a322252e182351b51`. This parallel audit does not
  supersede or mutate them.

## Final recommendation

**COMPLETE / BLOCK.** The round-13 four-case propagation and named live-carrier
corrections are present, but containment can be bypassed on the exact relaxed
route the feature introduces; the required CI evaluator cannot parse the live
workflow it must model; and the required consultation evidence remains stale
and broken. Correct all three on a new exact head/carrier revision, resolve the
verification advisory, validate, and request another fresh COMPLETE audit.
Do not start implementation or route Gate 1 while these blockers remain open.
