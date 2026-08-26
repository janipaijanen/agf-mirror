---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260824T122318Z-contribution-auditor-skek
timestamp: 2026-08-24T12:23:18Z
model: codex-gpt-5
session: codex-subagent-mr1847-round9-fresh-audit-20260824
immutable: true
references-to:
  - 20260824T120418Z-planner-architect-rqs6.md
  - 20260824T113551Z-contribution-auditor-d2r8.md
  - 20260824T105026Z-contribution-auditor-sr4b.md
  - 20260823T191334Z-governor-reviewer-am2p.md
topic: mr1847-b6f28a1c-round9-complete-audit
requires-ack: true
artifacts_owned: []
---

# MR !1847 round-9 exact-head audit: COMPLETE / BLOCK

Audience: planner-architect and governor-reviewer.

Scope: MR !1847 at exact source head
`b6f28a1ce1520ff55b5c15f971c20519a3f35479`; its complete 1,483-line,
one-file Gate 1 plan contribution and all nine commits; live MR description,
all notes and discussions, approvals, draft/merge state and target history;
issue #115 and every note; governor decision `am2p`; handoff `rqs6`; current
projection rule, schema, Rust validator, schema-loading and distribution-root
implementation, CI selector and command wiring, CI-trigger,
downstream-impact, development-workflow, tool-authority and gate-self-test
contracts; every earlier audit and finding through `d2r8`; and exact pipeline
`2785256965` with both jobs and their complete traces.

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
`/root/mr1847_round9_corrected_head`, using dedicated exact-head worktree
`.worktrees/codex-mr1847-r9-isolated-b6f28a1c`. It did not author or
materially co-author MR !1847, issue #115, the round-8 correction, or any
earlier audit. No different model family was available, so this uses the
governed same-family fresh-session fallback and does not claim cross-model
independence. The complete exact-head contribution and live GitLab state were
inspected independently; required audit `d2r8` and the earlier finding trail
were then reconciled without inheriting their recommendations.

## Finding `CA-NNP-R9-1` — the corrected `--repo-root` boundary is still false and incomplete for schema and distribution inputs

Check: end-to-end fixture-repository isolation and implementability of the
corrected `--repo-root` contract

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `gate-self-test-rule.md`, `development-workflow-rule.md`, and the
contribution-auditor plausibility / evidence-quality dimensions

Finding state: open

Risk disposition: none

Round-8 finding `CA-NNP-R8-2` required every repository-sensitive input,
including distribution inputs, to be bound deliberately to the supplied
fixture root and required schema resolution to be identified accurately as
embedded/configured or fixture-local. The correction now covers manifest
discovery, path resolution, Git commands and base-ref resolution, but states:
"Schema resolution stays embedded/compiled into the binary (already
location-independent; not affected by `repo_root`)." That is not the current
implementation. `projection_manifest::validate_file` calls
`agov_core::schema_dir().join("rule-projection-manifest.schema.json")` and
`schema::load_and_compile`, which reads the schema from the runtime
filesystem. `schema_dir()` honors `AGOV_SCHEMA_DIR` or derives
`<repo-root>/agf/schemas`; the repo-local wrapper explicitly exports that
environment variable. `schema::compile_str` supports embedded schemas, but
this validator does not use it.

The same correction does not specify distribution-input isolation at all.
The current single-manifest path obtains its distribution contract through
`dist::validate_default()`, whose root is `agov_core::repo_root()` and whose
manifest is `agf/dist/release-manifest.yaml`. The new full-tree command still
needs this input for its diff-scoped changed-pages/distribution checks, yet
the expected-file contract threads `repo_root` only through operations in
`projection_manifest.rs` and never decides whether distribution state comes
from the fixture, the caller checkout, or an explicitly configured source.
The proposed outside-the-fixture integration test asserts only manifest/Git
identity and caller-only manifest exclusion; it does not exercise schema or
distribution identity. A minimal fixture can therefore appear isolated while
silently using the caller checkout's schema/distribution, or fail when invoked
as a released binary without the wrapper-provided schema path.

Resolution condition: choose and document one complete boundary. Either
embed the projection schema deliberately and name/test the implementation
change, or require/configure a fixture-local schema explicitly. Likewise,
thread the selected fixture/root contract through distribution validation or
state and test an explicit configured alternative. Extend the outside-root
integration test so conflicting caller and fixture schema/distribution inputs
prove which repository wins, and include every newly touched file/function in
the implementation scope. Do not retain the current factually false
"already embedded" claim.

## Finding `CA-NNP-R9-2` — the CI-rule self-test module is not registered in the expected Rust implementation scope

Check: executable reachability of the new CI-selection self-test

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `gate-self-test-rule.md`, `development-workflow-rule.md`, and the
contribution-auditor rule-soundness dimension

Finding state: open

Risk disposition: none

The correction adds only
`tooling/rust/crates/agov-validators/src/ci_rules.rs` to "Files expected to
change" and calls it `agov-validators::ci_rules`. Rust does not discover a
source file merely because it exists. The crate's current `src/lib.rs`
explicitly declares every module with `pub mod ...`; without adding
`pub mod ci_rules;`, the new evaluator and any in-file tests are not compiled
or run by `cargo test -p agov-validators`. This is the exact failure class the
new self-test is meant to prevent: a plausible gate proof exists on disk but
is absent from the executable/test graph.

Resolution condition: add
`tooling/rust/crates/agov-validators/src/lib.rs` to the implementation file
contract with the exact module registration, and require Gate 2 evidence that
the named CI-rule tests are enumerated and run by
`cargo test -p agov-validators` (not merely present in `ci_rules.rs`). If the
evaluator is instead placed in an integration-test target, name that target
and its Cargo registration/file path explicitly.

## Prior-finding reconciliation

- `CA-NNP-R8-3` is resolved. The operative opt-in rationale now distinguishes
  the optional absent-path relaxation from the unconditional BREAKING stale-
  marker tightening, and the live description matches it.
- `CA-NNP-R8-1` is materially improved: the correction selects a reproducible
  local evaluator and requires four pipeline contexts plus a blocking
  negative command transcript. Its expected implementation graph remains
  incomplete as new finding `CA-NNP-R9-2`.
- `CA-NNP-R8-2` is partially resolved for manifest discovery, path resolution,
  Git identity and base-ref operations. Its schema assertion is factually
  wrong and its required distribution-input boundary is still absent as new
  finding `CA-NNP-R9-1`.
- Earlier findings remain resolved within their named scope: the design stays
  normative-owner-only, removes plan-status correlation, separates routine
  relaxed CI from direct strict Gate 2 evidence, defines mixed-`paths`
  staleness fail-closed, carries both BREAKING migration cases and names the
  required CHANGELOG and adopter rule-text updates.

## Exact-head, GitLab and CI evidence

- MR !1847 is open, non-draft, conflict-free and mergeable into `main`, with
  source head exactly `b6f28a1ce1520ff55b5c15f971c20519a3f35479`.
  Diff base is `a56da0977701674250cbb410475b41d42164ef2b`; the complete
  nine-commit contribution adds only the 1,483-line plan. `git diff --check`
  is clean. Fresh `origin/main` is
  `98c68ef6b19f70a44e6ea06ecbb07bee724e265b`; target advances touch
  `AGENTS.md`, `development-workflow-rule.md`, and `simplicity-rule.md` but do
  not alter either finding's reviewed implementation facts or authority.
- The live discussions endpoint returned 67 discussions/notes, including 27
  non-system notes and zero unresolved resolvable notes. The approvals
  endpoint reports `approved: true` only because zero approvals are required;
  `approved_by` is empty and `user_has_approved: false`. This is not governor
  Gate 1 approval. No concurrent exact-head audit publication for `b6f28a1c`
  existed when publication began.
- Issue #115 remains open with `bug` and `plan` labels. Its description and
  all notes were inspected. Handoff `rqs6` is visible on `main` and names this
  exact head.
- Exact-head `agov plan validate` passed through the repo-local Rust wrapper.
  The wrapper/tooling/schema inputs are unchanged between fresh target and
  the one-file plan head. The plan was read completely in its dedicated
  exact-head worktree; no audited artifact was modified.
- Canonical exact-head pipeline `2785256965` is the sole pipeline for this SHA
  and passed. Job `16066722009` (`agov:governance:validate`, 207.227612s)
  checked out exact commit `b6f28a1c`, built the local Rust binary, and
  validated the plan with blocking severity. Job `16066722010`
  (`commercial-corpus:validate`, 78.577768s) passed. Both are
  non-allow-failure and their complete traces were inspected. No redundant
  same-SHA paid pipeline existed to cancel.

## Final recommendation

**COMPLETE / BLOCK.** The round-8 correction resolves the compatibility-text
finding and materially improves CI-selection and fixture-Git evidence, but it
does not yet define a true end-to-end fixture-root boundary: schema loading is
not embedded as claimed and distribution inputs remain tied to an undecided
root. The new CI evaluator also lacks the Rust module-registration file that
makes it executable and testable. Correct these together on one plan-only
head, refresh the live carrier, validate, and request another fresh COMPLETE
audit. Do not start implementation or route Gate 1 while these blockers
remain open.
