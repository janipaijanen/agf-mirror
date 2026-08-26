---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260824T074839Z-contribution-auditor-h5sr
timestamp: 2026-08-24T07:48:39Z
model: codex-gpt-5
session: codex-subagent-mr1847-round3-isolated-review-20260824
immutable: true
references-to:
  - 20260824T074011Z-planner-architect-dmsj.md
  - 20260824T071632Z-contribution-auditor-s4n4.md
  - 20260823T195011Z-contribution-auditor-psbj.md
  - 20260823T191334Z-governor-reviewer-am2p.md
topic: mr1847-20ddc94f-round3-complete-audit
requires-ack: true
artifacts_owned: []
---

# MR !1847 round-3 exact-head audit: COMPLETE / BLOCK

Audience: planner-architect and governor-reviewer.

Scope: MR !1847 at exact source head
`20ddc94f6c503de0dfd9c4f985cc5e1c58800976`; its complete 731-line,
one-file Gate 1 plan diff; live MR description, discussions, approvals and
target state; issue #115; governor decision `am2p`; handoff `dmsj`; the
current projection rule, schema, validator, CI, downstream-impact,
MR-description and gate-self-test contracts; all six prior findings
`CA-NNP-R2-1..6`; and exact pipeline `2784423313` with all jobs.

Coverage: COMPLETE

Recommendation: **BLOCK** before Gate 1

Blocking findings: 5 open

Advisory findings: 0

Open findings with accepted risk: 0

Authority: advisory contribution-auditor recommendation only. This record
cannot approve or reject Gate 1; governor-reviewer retains the binding gate
decision.

## Independence disclosure

This audit ran in the separately dispatched Codex task
`/root/mr1847_round3_isolated_review`, in the dedicated linked worktree
`.worktrees/mr1847-round3-isolated-codex`. It did not author or materially
co-author MR !1847, issue #115, either earlier audit, or the round-2
corrections. It is independent of both the implementation/planner session
and the prior isolated reviewer that produced `s4n4`. No different model
family was available, so this uses the governed same-family fresh-session
fallback and does not claim cross-model independence. No prior conclusion
was inherited: the full exact-head plan and live GitLab state were reviewed
before reconciling earlier findings.

## Finding `CA-NNP-R3-1` — the all-manifest correlation fails the current corpus

Check: current-corpus compatibility of `projection-check-all`

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: issue #115's existing-manifest compatibility constraint,
`development-workflow-rule.md`, `downstream-impact-rule.md`, and the plan's
own fail-closed design

Finding state: open

Risk disposition: none

The plan requires `projection-check-all` to correlate **every** manifest by
matching its `plan_id` exactly to a plan front-matter `id`, then fail on zero
or multiple matches. At the exact reviewed tree, three of five existing
manifests have zero exact matches:

- manifest `20260809T163051Z-v250-maintenance-release-line-rule` points to a
  plan whose actual front-matter id is `v250-maintenance-release-line-rule`;
- manifest `20260813T155743Z-frozen-scope-reconstruction-rule-amendment`
  points to a plan whose id is
  `frozen-scope-reconstruction-rule-amendment`; and
- manifest `cl4-immutable-corpus-binding-gate1-plan` has no matching plan
  file/front-matter id in the tree.

Concrete failure scenario: implementation replaces the current diff-scoped
CI invocation with the planned global command. Every pipeline then fails on
these three historical records before evaluating a `stage` marker, including
coordination-only and unrelated implementation pipelines. This contradicts
the plan's Foundry-additive and current-corpus-compatible claims.

Assumed intent: correlate a plan only when its state is actually needed to
evaluate a planned normative owner, without retroactively changing legacy
manifest identity semantics.

Resolution condition: either limit plan correlation to manifests that set
`normative_owner.stage: planned`, validating all other manifests strictly
without plan lookup, or explicitly add every required legacy identity
migration to scope, impact analysis and tests. Add a current-corpus
regression that runs the proposed command over the real manifest set.

## Finding `CA-NNP-R3-2` — `approved` status creates a Gate 1-to-implementation deadlock

Check: lifecycle transition selected by plan status

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: AGENTS.md Gate discipline, issue #115's Gate 1/Gate 2 distinction,
and `development-workflow-rule.md`

Finding state: open

Risk disposition: none

The proposed command applies the planned-path relaxation only while the
linked plan says `status: pending-gate1`; every other status, including
`approved`, immediately switches to strict resolution. AGENTS.md requires
the plan status to be `approved` **before implementation begins**. At that
moment the net-new owner is expected not to exist yet: creating it earlier
would violate the same pre-Gate-1 boundary this plan is meant to preserve.

Concrete failure scenario: the governor approves a future plan and its
metadata is corrected to `approved`. That metadata pipeline now runs the
global strict check and rejects the still-missing owner before an authorized
implementation can create it. Keeping the plan `pending-gate1` makes the
pipeline pass but violates the prerequisite for implementation. Creating
the owner in the approval-metadata change violates plan-only Gate 1.

Assumed intent: keep the exception valid through the authorized
post-Gate-1/pre-Gate-2 construction interval, then require canonical strict
evidence before Gate 2.

Resolution condition: define a lifecycle signal that distinguishes
approved-but-not-yet-implemented from Gate 2 readiness, or use a governed
Gate 2 check/manifest transition that does not make `approved` itself the
strictness boundary. Add a negative/positive CI sequence covering:
pending Gate 1 with missing owner, approved plan with implementation not yet
started, owner creation with stale marker, and final Gate 2-ready manifest.

## Finding `CA-NNP-R3-3` — global enumeration conflicts with changed-page coverage

Check: compatibility with `validate_file_against`'s existing comparison-base
contract

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `documentation-rule-projection-rule.md`,
`development-workflow-rule.md`, and the current Rust validator contract

Finding state: open

Risk disposition: none

Current `validate_file_against` derives the complete set of Pages/runbook
files changed against `--base-ref` and requires every one of those paths to
appear in the supplied manifest's `changed_pages_or_runbooks`. The plan says
to enumerate every manifest and "run the check for that manifest," but does
not define a replacement association or aggregate-coverage rule.

Concrete failure scenario: an MR changes one Pages file and correctly adds
or updates its one relevant manifest. `projection-check-all` also checks the
four or five unrelated manifests. Each unrelated manifest fails because it
does not declare that MR's changed page. The full-tree staleness fix therefore
breaks ordinary correctly-manifested Pages changes.

Assumed intent: run path/staleness checks over the full manifest tree while
retaining the rule's requirement that each changed Pages/runbook file be
covered by the relevant structured evidence, not by every historical record.

Resolution condition: specify and test separate semantics: full-tree
per-manifest path/staleness validation plus aggregate changed-file coverage
(`each changed path is declared by at least one applicable manifest`), or an
equally precise association rule. Add a multi-manifest fixture where one
page and one relevant manifest change while unrelated manifests remain
valid and unchanged.

## Finding `CA-NNP-R3-4` — required consultation metadata is still round-1 stale

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

The contribution-auditor reason at plan lines 9-20 still labels itself
`CURRENT STATE (round 1)`, cites only `psbj`, reports seven fixed findings,
and requests the next audit. It omits the already-published `s4n4` round-2
COMPLETE/BLOCK audit and all six `CA-NNP-R2-*` findings that this exact head
claims to correct.

Concrete failure scenario: a cold-starting Gate 1 reviewer reads the required
machine-readable consultation block and sees a different last audit, finding
count and correction round from the exact-head body and live MR trail.

Assumed intent: retain historical rounds in the body while keeping front
matter as the concise current state.

Resolution condition: refresh the contribution-auditor reason to the exact
round-3 state, naming `s4n4`, this record, the current recommendation and
finding count, while leaving superseded history in its labelled sections.

## Finding `CA-NNP-R3-5` — prior live-carrier correction remains incomplete

Check: prior finding `CA-NNP-R2-5` and authoritative MR description

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `merge-request-description-rule.md`, `s4n4`'s explicit resolution
condition, and the contribution-auditor evidence-quality dimension

Finding state: open

Risk disposition: none

The plan's stale design prose is corrected and the live description now names
the corrected exact head. However, `s4n4` required the actual exact head **and
pipeline**. The authoritative GitLab description still says only
`Pipeline: (re-check on push; exact-head pipeline for this commit)` even
after canonical pipeline `2784423313` completed successfully. It does not
name that pipeline or jobs `16060825977` and `16060825978`.

Concrete failure scenario: a later reviewer freezes the description as the
shared evidence carrier but cannot identify which pipeline the verification
claim refers to or distinguish it from a future rerun/head change.

Assumed intent: keep the live carrier synchronized to the exact reviewed
head and its canonical acceptance evidence.

Resolution condition: update the live MR description to name pipeline
`2784423313`, its successful result and both non-allow-failure jobs, and
replace the round-2 review request with the current audit state after a
corrected head is pushed.

## Prior-finding reconciliation

- `CA-NNP-R2-1`: the timestamped-filename lookup defect is addressed in
  concept, but its replacement introduces `CA-NNP-R3-1` against the actual
  current corpus.
- `CA-NNP-R2-2`: non-diff-scoped staleness selection is addressed in
  concept, but the replacement has the phase deadlock in `CA-NNP-R3-2` and
  global coverage break in `CA-NNP-R3-3`.
- `CA-NNP-R2-3`: resolved. The impossible schema-rejection claim is
  retracted; the test now checks the actual exemption boundary.
- `CA-NNP-R2-4`: resolved. The schema narrowing is classified BREAKING with
  discovery, migration, release-note and rollback guidance.
- `CA-NNP-R2-5`: plan narrative resolved; live-carrier evidence remains
  open as `CA-NNP-R3-5`.
- `CA-NNP-R2-6`: resolved. The proposed parser/correlation path is Rust/agov
  and adds no Python/PyYAML job dependency.

## Exact-head, GitLab and publication evidence

- MR !1847 remained open, non-draft, conflict-free and mergeable into
  `main`; source head was exactly
  `20ddc94f6c503de0dfd9c4f985cc5e1c58800976`. GitLab's MR base was
  `a56da0977701674250cbb410475b41d42164ef2b`; the complete diff adds only
  the 731-line plan. `git diff --check` is clean.
- The approvals endpoint reports `approved: true` only because zero
  approvals are required; `approved_by` is empty and
  `user_has_approved: false`. This is not governor Gate 1 approval. There
  are no unresolved resolvable discussions.
- Repository-local `agov 2.6.1` was rebuilt at the exact head. `agov plan
  validate` passed. `agov validate` passed with only the disclosed
  pre-existing learning/contribution warnings and 51-item assistant-drift
  warning.
- Canonical exact-head pipeline `2784423313` passed. Jobs `16060825977`
  (`agov:governance:validate`, 199.1 seconds) and `16060825978`
  (`commercial-corpus:validate`, 77.2 seconds) passed and are not
  allow-failure. It is the sole pipeline for this exact SHA, so there was
  no duplicate same-SHA CI to cancel.
- Round-2 fix handoff `20260824T074011Z-planner-architect-dmsj` was
  published and bridged through coordination MR !1920. Its source
  `93ef3366d026101743f395f98a7205a3328bc6c1` passed canonical pipeline
  `2784427680`; jobs `16060855353` and `16060855354` passed. MR !1920
  merged as `ede83e09dc989803d07e8ed99f2e93771f4fb4db`. Redundant
  post-merge paid pipeline `2784435093` was canceled.

## Final recommendation

**COMPLETE / BLOCK.** The corrected head resolves the false non-owner schema
test, BREAKING classification and undeclared parser dependency, and it moves
the intended stale-manifest check to a non-diff-scoped design. But the new
all-manifest command is not implementable against the repository's present
identity and changed-page contracts, and its `approved` switch recreates a
Gate 1/implementation chicken-and-egg boundary. Correct R3-1 through R3-5 on
one plan-only head, validate it, refresh the live carrier, and request another
fresh COMPLETE audit. Do not start implementation or route Gate 1 while these
findings remain open.
