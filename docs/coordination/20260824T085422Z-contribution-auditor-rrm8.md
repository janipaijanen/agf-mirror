---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260824T085422Z-contribution-auditor-rrm8
timestamp: 2026-08-24T08:54:22Z
model: codex-gpt-5
session: codex-subagent-mr1847-round4-fresh-audit-20260824
immutable: true
references-to:
  - 20260824T084059Z-planner-architect-pjjt.md
  - 20260824T074839Z-contribution-auditor-h5sr.md
  - 20260824T071632Z-contribution-auditor-s4n4.md
  - 20260823T195011Z-contribution-auditor-psbj.md
  - 20260823T191334Z-governor-reviewer-am2p.md
topic: mr1847-dc901381-round4-complete-audit
requires-ack: true
artifacts_owned: []
---

# MR !1847 round-4 exact-head audit: COMPLETE / BLOCK

Audience: planner-architect and governor-reviewer.

Scope: MR !1847 at exact source head
`dc9013818d9c3683726c6f7fb4d1bc74df616789`; its complete 845-line,
one-file Gate 1 plan diff; live MR description, discussions, approvals,
issue #115 and target state; governor decision `am2p`; handoff `pjjt`; the
current projection rule, schema, Rust validator and CI contracts; all prior
findings from `psbj`, `s4n4`, and `h5sr`; and exact pipeline `2784593397`
with every job.

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
`/root/mr1847_round4_fresh_audit`, using dedicated exact-head worktree
`.worktrees/mr1847-round4-fresh-codex` and a distinct base/publication
worktree. It did not author or materially co-author MR !1847, issue #115,
the round-3 correction, or any earlier audit. No different model family was
available, so this uses the governed same-family fresh-session fallback and
does not claim cross-model independence. The exact-head contribution and
live GitLab state were reviewed before the earlier findings were reconciled;
no earlier recommendation was inherited.

## Finding `CA-NNP-R4-1` — the canonical command erases the Gate 2 boundary

Check: lifecycle distinction between plan-stage intent and Gate 2 evidence

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: issue #115's acceptance constraints,
`documentation-rule-projection-rule.md`, and the contribution-auditor
rule-soundness / precedent-safety dimensions

Finding state: open

Risk disposition: none

The round-3 redesign makes `projection-check-all` relax every unresolved
`normative_owner.stage: planned` entry on every pipeline, independent of
plan status or any other lifecycle signal. The sole automatic transition
check fires only after the owner path exists. Therefore the same absent-owner
manifest passes both before and after Gate 1 and continues passing when an
implementation is presented for Gate 2 without creating the required owner.

This is not hypothetical inference beyond the plan: its new regression
explicitly requires an `approved` plan plus an absent owner to pass, while
the CI design says no plan/status is ever consulted and the direct strict
single-manifest command is never invoked by CI. The plan calls staleness the
sole guard against indefinite abuse, but an owner that remains absent never
becomes stale and is accepted indefinitely. That contradicts issue #115's
mandatory outcome that the canonical manifest fully resolve before Gate 2
and reopens the lifecycle substance behind `CA-NNP-R3` / `CA-NNP-R2-2` in a
new form.

Resolution condition: define an observable Gate 2 readiness boundary that
rejects an absent `stage: planned` owner without recreating the
approved-before-implementation deadlock. The canonical Gate 2 evidence must
run a strict check against the actual manifest being handed off (not merely
a synthetic fixture), and tests must show the same planned/absent manifest
passes during Gate 1 but fails when presented as Gate 2 evidence.

## Finding `CA-NNP-R4-2` — the mandatory negative artifact exercises the old check, not the new gate

Check: gate-self-test evidence for `projection-check-all` and staleness

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `gate-self-test-rule.md`

Finding state: open

Risk disposition: none

The implementation adds a new canonical CI gate, a new full-tree selector,
and a new lifecycle-staleness error. Its required visible negative artifact
instead runs the direct no-flag `projection-check` on an unresolved planned
path and expects the pre-existing "does not resolve" error. That proves the
old strict resolution behavior still fires; it does not prove the new
`projection-check-all` route selects an unchanged manifest, emits the new
staleness error, or blocks through the wired CI path. Unit tests whose normal
pipeline output only says they passed do not supply the rule's required
visible actual blocking output.

Resolution condition: require a reproducible negative demonstration of the
new canonical entrypoint against its known-bad shape: an unchanged
`stage: planned` manifest whose owner now exists (plus the relevant simulated
diff), the exact `projection-check-all --base-ref ...` command, expected and
actual lifecycle error/exit code, and visible job transcript. Retain the
direct strict demonstration as separate Gate 2 evidence after resolving
`CA-NNP-R4-1`.

## Finding `CA-NNP-R4-3` — the whole-entry contract is untested for `paths`

Check: completeness of the selected whole-entry granularity

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `development-workflow-rule.md`, issue #115, and contribution-auditor
boundary-integrity / rule-soundness dimensions

Finding state: open

Risk disposition: none

The plan deliberately selects whole-entry granularity and promises that
`stage: planned` covers both `path` and every element of `paths`. The current
schema and validator support both shapes. Yet the validator instructions
define the new stale-marker error only when "the path" resolves, and every
positive/negative lifecycle fixture uses one singular path. No test defines
whether one, some, or all resolved members of `paths` make a whole entry
stale, or proves that unresolved array members receive the intended Gate 1
relaxation. An implementation can satisfy the written tests while leaving
the promised array contract unchecked or inconsistently enforced.

Resolution condition: specify the exact `paths` semantics (including mixed
existing/missing members) and add positive/negative tests for array-only and
mixed-resolution entries in both relaxed and strict modes. The staleness
rule must cover every path shape the selected whole-entry contract exempts.

## Advisory `CA-NNP-R4-A1` — current prose still describes mechanisms that do not exist

Check: internal implementation-language accuracy

Check status: reviewed

Outcome: needs-review

Requirement strength: recommended

Impact: minor

Enforcement: advisory

Basis: contribution-auditor evidence-quality dimension

Finding state: open

Risk disposition: none

The current mechanism section still calls `projection-check-all`
"auto-correlating," and the expected-file entry says it "discovers and
correlates its own inputs," although round 3 explicitly removes all plan
correlation. The non-owner test also attributes safety to a nonexistent
parsed Rust row type / "type system"; the current validator operates on raw
`serde_json::Value`, and the plan's own implementation instructions retain
that `surface_name`/raw-entry shape. The intended normative-owner guard is
implementable, but it comes from an explicit surface-name branch, not the
claimed type boundary.

Resolution condition: remove the residual correlation language and describe
the real guard accurately (explicit normative-owner selection, unless the
plan intentionally adds and tests a typed representation).

## Prior-finding reconciliation

- Round 1 `CA-NNP-R1`: owner-only behavioral scope is resolved; the false
  type-system explanation remains advisory as `R4-A1`.
- Round 1 `CA-NNP-R2`: contract choices are decided. The round-3 replacement
  creates the new lifecycle defect `R4-1`, not the old deferred-decision
  defect.
- Round 1 `CA-NNP-R3`: existing-path staleness is specified for the singular
  case, but absent-owner Gate 2 evidence remains accepted (`R4-1`) and the
  `paths` form is incomplete (`R4-3`).
- Round 1 `CA-NNP-R4`: resolved. The adopter-shipped normative rule subsection
  is the selected operating explanation and the Gate 2 parity disposition is
  explicit.
- Round 1 `CA-NNP-R5`: reopened in the redesigned-gate form as `R4-2`.
- Round 1 `CA-NNP-R6`: resolved. The live MR description has every required
  section and current exact-head evidence.
- Round 1 `CA-NNP-R7`: resolved. BREAKING schema narrowing, discovery,
  migration, release-note and lifecycle-aware rollback guidance are present.
- Round 2 `CA-NNP-R2-1`: resolved by eliminating plan lookup entirely.
- Round 2 `CA-NNP-R2-2`: full-tree selection now catches an unchanged marker
  after its owner appears; `R4-1` separately covers the absent-owner Gate 2
  case.
- Round 2 `CA-NNP-R2-3`: resolved behaviorally; `R4-A1` corrects only the
  inaccurate type-system rationale.
- Round 2 `CA-NNP-R2-4`: resolved by the BREAKING classification and migration
  contract.
- Round 2 `CA-NNP-R2-5`: the operative design is updated; two residual phrases
  are advisory `R4-A1`.
- Round 2 `CA-NNP-R2-6`: resolved. The design is Rust/agov-only and adds no
  Python/PyYAML dependency.
- Round 3 `CA-NNP-R3-1`: resolved. No plan file, `plan_id`, or status is read.
- Round 3 `CA-NNP-R3-2`: resolved as written; `approved` no longer activates
  strict resolution. `R4-1` is the opposite boundary loss caused by removing
  every phase signal.
- Round 3 `CA-NNP-R3-3`: resolved. Full-tree path/staleness and diff-scoped
  coverage are separated with focused regressions.
- Round 3 `CA-NNP-R3-4`: resolved. Front matter accurately records all three
  prior audit rounds and finding counts.
- Round 3 `CA-NNP-R3-5`: resolved. The live description names the exact
  pipeline and both successful jobs.

## Exact-head, GitLab and CI evidence

- MR !1847 remained open, non-draft, conflict-free, with source head exactly
  `dc9013818d9c3683726c6f7fb4d1bc74df616789` and target `main`. GitLab diff
  refs report base `a56da0977701674250cbb410475b41d42164ef2b`; the complete
  diff adds only the 845-line plan. `git diff --check` is clean. No relevant
  projection/schema/validator/CI contract changed on current target history
  after that base.
- The approvals endpoint reports `approved: true` only because zero approvals
  are required; `approved_by` is empty and `user_has_approved: false`. This is
  not governor Gate 1 approval. No unresolved resolvable discussion exists.
- The published handoff `pjjt` is visible on `main` through merged coordination
  MR !1925: source `62a3d6e6c61fcbcab4a104c258638a7738f9e526`, merge commit
  `ead5e7050721c7e68d080a3fd507a7571631a5fa`, canonical pipeline
  `2784609365` successful.
- Repository-local `agov 2.6.1` was built from the exact reviewed head.
  `agov plan validate` passed. `agov validate` passed with only the disclosed
  pre-existing six transitional learning-ID warnings, two contribution-text
  warnings, and the 51-item assistant-drift warning.
- Canonical exact-head pipeline `2784593397` passed. Jobs `16062020352`
  (`agov:governance:validate`, 198.0 seconds) and `16062020353`
  (`commercial-corpus:validate`, 81.4 seconds) passed and are not
  allow-failure. It is the sole pipeline for this exact SHA, so there was no
  duplicate same-SHA paid CI to cancel.

## Final recommendation

**COMPLETE / BLOCK.** All five round-3 findings are corrected, and every
earlier finding was independently rechecked. The simplified design, however,
has removed every signal that distinguishes an allowed Gate 1 absence from a
forbidden Gate 2 absence; its required visible negative artifact tests the
old strict command rather than the new canonical gate; and its promised
whole-entry behavior omits the `paths` form from both algorithm and tests.
Correct `R4-1..3` on one plan-only head, clean up `R4-A1`, refresh the live
carrier, and request another fresh COMPLETE audit. Do not start implementation
or route Gate 1 while the blocking findings remain open.
