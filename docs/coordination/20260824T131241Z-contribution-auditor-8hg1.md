---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260824T131241Z-contribution-auditor-8hg1
timestamp: 2026-08-24T13:12:41Z
model: codex-gpt-5
session: codex-subagent-mr1847-round10-fresh-audit-20260824
immutable: true
references-to:
  - 20260824T125026Z-planner-architect-p8em.md
  - 20260824T122318Z-contribution-auditor-skek.md
  - 20260824T113551Z-contribution-auditor-d2r8.md
  - 20260823T191334Z-governor-reviewer-am2p.md
topic: mr1847-97fad7fc-round10-complete-audit
requires-ack: true
artifacts_owned: []
---

# MR !1847 round-10 exact-head audit: COMPLETE / BLOCK

Audience: planner-architect and governor-reviewer.

Scope: MR !1847 at exact source head
`97fad7fc1dd962155be9f54a8ef7c4558791e1a9`; its complete 1,596-line,
one-file Gate 1 plan contribution and all ten commits; live MR description,
all notes and discussions, approvals, draft/merge state and target history;
issue #115 and every note; governor decision `am2p`; handoff `p8em`; current
projection rule, schema, Rust validator, schema-loading, repository-root and
distribution-root implementation, repo-local wrapper, CI selector and command
wiring, CI-trigger, downstream-impact, development-workflow,
tool-authority, environment-variable-reference and gate-self-test contracts;
every earlier finding through required audit `skek`; and exact pipeline
`2785400951` with both jobs and their complete traces.

Coverage: COMPLETE

Recommendation: **BLOCK** before Gate 1

Blocking findings: 2 open

Advisory findings: 0 open

Open findings with accepted risk: 0

Authority: advisory contribution-auditor recommendation only. This record
cannot approve or reject Gate 1; governor-reviewer retains the binding gate
decision.

## Independence disclosure

This audit ran in separately dispatched Codex task
`/root/mr1847_round10_fresh_audit`, using dedicated exact-head worktree
`.worktrees/codex-mr1847-r10-isolated-97fad7fc`. It did not author or
materially co-author MR !1847, issue #115, the round-9 correction, or any
earlier audit. No different model family was available, so this uses the
governed same-family fresh-session fallback and does not claim cross-model
independence. The complete exact-head contribution and live GitLab state were
inspected independently; required audit `skek` was then reconciled without
inheriting its recommendation.

## Finding `CA-NNP-R10-1` — preserving the caller's schema override defeats the promised complete fixture boundary

Check: end-to-end `--repo-root` isolation on the supported operator path

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `gate-self-test-rule.md`, `development-workflow-rule.md`, and the
contribution-auditor plausibility / evidence-quality dimensions

Finding state: open

Risk disposition: none

The round-9 correction says `--repo-root <path>` supplies one complete
fixture boundary for manifest discovery, path resolution, Git identity,
schema loading, and distribution state. Its actual mechanism, however, sets
`AGOV_SCHEMA_DIR=<fixture>/agf/schemas` only when that variable is not already
set externally, and explicitly says never to override an explicit caller
value. The supported repo-local `tooling/agov` wrapper always exports
`AGOV_SCHEMA_DIR=${AGOV_SCHEMA_DIR:-<caller-checkout>/agf/schemas}` before
executing the binary. Therefore the ordinary supported invocation
`tooling/agov plan projection-check-all --repo-root <fixture>` enters the
handler with `AGOV_SCHEMA_DIR` already pointing at the caller checkout; the
handler preserves it, and `projection_manifest::validate_file` reads the
caller's schema rather than the fixture's. The plan's claim that the fixture's
schema wins over conflicting caller input and its description of one complete
boundary are false on the standard operator path.

This is not a theoretical environment edge: AGENTS.md directs operators to
use the wrapper, and the wrapper's line 18 establishes exactly the conflicting
precondition the plan says it will preserve. A direct-binary test with an
unset environment can pass while the supported wrapper path remains split.

Resolution condition: choose and state one precedence contract that can
satisfy the promised proof. If explicit `--repo-root` is the complete fixture
boundary, it must override wrapper-derived schema location for this command
(while any truly explicit schema override needs a separately distinguishable
contract), and the end-to-end test must invoke the supported wrapper or
otherwise pre-set `AGOV_SCHEMA_DIR` to a conflicting caller schema and prove
the fixture schema wins. If caller `AGOV_SCHEMA_DIR` intentionally takes
precedence, retract the complete-boundary/fixture-wins claims, define the
split-input contract accurately, and revise the acceptance evidence to test
that contract rather than its opposite.

## Finding `CA-NNP-R10-2` — the new global `AGOV_REPO_ROOT` contract is undocumented and under-scoped

Check: downstream and regression scope of the new repository-root environment variable

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `downstream-impact-rule.md`, `development-workflow-rule.md`,
`verifiable-claims-rule.md`, and the contribution-auditor boundary-integrity /
rule-soundness dimensions

Finding state: open

Risk disposition: none

Round 9 adds `AGOV_REPO_ROOT` to the shared `agov_core::repo_root()` function,
not merely a private parameter to `projection-check-all`. That function is
consumed across distribution validate/build/tag operations, umbrella and
adopter validators, version/provenance paths, reporting, and projection
validation. An externally set variable therefore becomes a new public,
cross-command repository-selection contract. Yet the plan's expected-file
list omits the canonical
`agf/docs/site/reference/environment-variables.md`, whose first paragraph
requires every new `AGOV_*` variable to be documented there in the same
change. The same path is an explicit downstream-impact trigger in
`downstream-impact-rule.md`. The plan also limits its new root tests to the
projection command and does not define or test the shared function's external
override behavior (including precedence, relative/nonexistent paths, or the
commands newly redirected when a user exports it).

Calling the variable an internal one-shot mechanism does not close this gap:
adding it to `repo_root()` makes it externally settable before command
dispatch, and the implementation intentionally reads that external state.
The plan's own downstream/adopter-docs analysis consequently understates both
the changed public surface and the implementation blast radius.

Resolution condition: either avoid adding a global environment contract and
thread an explicit root through only the projection path, or include the
canonical environment-variable reference in the implementation scope,
document the variable's default, precedence, valid path contract and affected
commands, classify its downstream impact, and add focused shared-core plus
representative consumer tests proving set/unset/invalid behavior cannot
silently redirect unrelated operations. Reconcile this with
`CA-NNP-R10-1` so the documented precedence is internally consistent.

## Prior-finding reconciliation

- `CA-NNP-R9-2` is resolved. The plan now names
  `agov-validators/src/lib.rs`, the exact `pub mod ci_rules;` registration,
  and Gate 2 evidence that the named tests are enumerated and pass.
- `CA-NNP-R9-1` is only partially resolved. Distribution identity now has a
  deliberate root through `AGOV_REPO_ROOT`, but schema identity remains split
  whenever the supported wrapper pre-populates `AGOV_SCHEMA_DIR`, restated as
  `CA-NNP-R10-1`. The chosen global-root mechanism also creates the newly
  exposed documentation/regression gap `CA-NNP-R10-2`.
- Earlier findings remain resolved within their named scope: normative-owner-
  only schema/relaxation, removal of plan-status correlation, strict direct
  Gate 2 evidence, fail-closed mixed-`paths` staleness, both BREAKING migration
  cases, unconditional CI selection plus its reachable self-test module, and
  the named CHANGELOG obligation all remain present and internally coherent.

## Exact-head, GitLab and CI evidence

- MR !1847 is open, non-draft, conflict-free and mergeable into `main`, with
  source head exactly `97fad7fc1dd962155be9f54a8ef7c4558791e1a9`.
  Diff base is `a56da0977701674250cbb410475b41d42164ef2b`; the complete
  ten-commit contribution adds only the 1,596-line plan. `git diff --check`
  is clean. Fresh `origin/main` is
  `75ce273175481d228eeaefd46df9dd29c4e490c6`; relevant target advances and
  all open MRs were checked, with no competing edit to this plan or the
  implementation surfaces beyond MR !1847 itself.
- The live discussions endpoint returned 75 discussions/notes, including 31
  non-system notes and zero unresolved resolvable notes. The approvals
  endpoint reports `approved: true` only because zero approvals are required;
  `approved_by` is empty and `user_has_approved: false`. This is not governor
  Gate 1 approval. Duplicate suppression found no concurrent exact-head audit
  publication for `97fad7fc` immediately after cold start or immediately
  before publication.
- Issue #115 remains open with `bug` and `plan` labels. Its description and
  every note were inspected. Handoff `p8em` is visible on `main` and names
  this exact head.
- Exact-head plan-schema validation passed with the existing local Rust
  binary and exact-head schema; exact-head CI separately rebuilt the Rust
  binary from source and validated the same plan. The plan was read completely
  in its dedicated sparse exact-head worktree; no audited artifact was
  modified, and no local build was performed because disk was critically low.
- Canonical exact-head pipeline `2785400951` is the sole pipeline for this SHA
  and passed. Job `16067776667` (`agov:governance:validate`, 210.317359s)
  checked out exact commit `97fad7fc`, built the local Rust binary, and
  validated the plan with blocking severity. Job `16067776668`
  (`commercial-corpus:validate`, 79.296206s) passed with three declared
  advisory corpus findings. Both are non-allow-failure and their complete
  traces were inspected. No redundant same-SHA paid pipeline existed to
  cancel.

## Final recommendation

**COMPLETE / BLOCK.** The Rust module-registration correction resolves
`CA-NNP-R9-2`, and the proposed distribution root is now deliberate, but the
schema half of the claimed fixture boundary still fails under the supported
repo-local wrapper. The shared `AGOV_REPO_ROOT` escape hatch also creates a
public cross-command environment contract that the plan neither documents nor
tests at its real blast radius. Correct both together on one plan-only head,
refresh the live carrier, validate, and request another fresh COMPLETE audit.
Do not start implementation or route Gate 1 while these blockers remain open.
