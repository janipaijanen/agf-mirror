---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260824T145228Z-contribution-auditor-krpk
timestamp: 2026-08-24T14:52:28Z
model: codex-gpt-5
session: codex-subagent-mr1847-round12-fresh-audit-20260824
immutable: true
references-to:
  - 20260824T143412Z-planner-architect-hhg3.md
  - 20260824T140020Z-contribution-auditor-mkps.md
  - 20260823T191334Z-governor-reviewer-am2p.md
topic: mr1847-c75b931d-round12-complete-audit
requires-ack: true
artifacts_owned: []
---

# MR !1847 round-12 exact-head audit: COMPLETE / BLOCK

Audience: planner-architect and governor-reviewer.

Scope: MR !1847 at exact source head
`c75b931dc33e4404156b3edbca72fdb0bbd01b35`; its complete 1,833-line,
one-file Gate 1 plan contribution and all twelve commits; the exact round-11
delta; live MR description, notes/discussions, approvals, merge state and
target history; issue #115 and its notes; handoff `hhg3`; required prior audit
`mkps`; current projection rule, schema, Rust validator, schema/distribution
loading and repository-root behavior; repo-local wrapper; GitLab workflow and
job rules; all 51 open MRs targeting `main` and their changed paths; and exact
pipeline `2785792711` with both complete job traces.

Coverage: COMPLETE

Recommendation: **BLOCK** before Gate 1

Blocking findings: 2 open

Advisory findings: 1 open

Open findings with accepted risk: 0

Authority: advisory contribution-auditor recommendation only. This record
cannot approve or reject Gate 1; governor-reviewer retains the binding gate
decision.

## Independence disclosure

This audit ran in the separately dispatched Codex task
`/root/mr1847_round12_fresh_audit`. It did not author or materially co-author
MR !1847, issue #115, the round-11 correction, or any earlier audit. No
different model family was available, so this uses the governed same-family
fresh-session fallback and does not claim cross-model independence. The
complete exact-head contribution and live repository/GitLab state were formed
into an independent judgment before reconciling `mkps`. A fresh sparse
worktree rooted at current `origin/main` was used. Exact-head CI already built
and validated this plan, so no duplicative local build was performed.

## Finding `CA-NNP-R12-1` — `stage: planned` can carry no owner path and vacuously pass Gate 2

Check: the strict Gate 2 command must prove a concrete planned owner now
exists

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: critical

Enforcement: blocking

Basis: issue #115's Gate 2 acceptance constraint,
`documentation-rule-projection-rule.md`, `gate-self-test-rule.md`, and the
contribution-auditor rule-soundness / boundary-integrity dimensions

Finding state: open

Risk disposition: none

The new `normativeOwnerEntry` only composes today's `surfaceEntry` with the
optional `stage` enum. Today's `surfaceEntry` requires only `disposition` and
`reason`; neither `path` nor `paths` is required, and `paths` has no
`minItems`. The planned validator logic likewise iterates only values that are
present. Consequently each of these is schema-valid under the proposed design:

```yaml
normative_owner:
  disposition: updated
  reason: net-new owner
  stage: planned
```

```yaml
normative_owner:
  disposition: updated
  reason: net-new owner
  stage: planned
  paths: []
```

In both cases `--allow-planned` has nothing to relax, the unconditional
staleness check has nothing to inspect, and the direct no-flag Gate 2 command
has no missing path to reject. It therefore passes without proving any owner
exists, contradicting the plan's claim that the strict invocation makes an
unresolved planned owner fail every time. None of the specified tests covers
missing or empty owner-path declarations.

Concrete failure scenario: a future Gate 1 manifest uses either shape above;
the plan and implementation proceed, and the exact same manifest is presented
at Gate 2. Both relaxed tree-wide validation and the supposedly strict direct
command pass while no normative-owner target was ever named or created.

Assumed intent: `stage: planned` denotes one or more exact absent paths that
the same Gate 1 plan commits to create before Gate 2.

Resolution condition: make `stage: planned` conditionally require at least one
non-empty `path` or non-empty `paths` member in the schema and validator
contract; decide whether both keys together are allowed; add negative tests
for a missing path and `paths: []`; and demonstrate that the direct no-flag
Gate 2 command rejects both shapes. Keep pathless `not applicable` dispositions
available where they are legitimate and not marked `planned`.

## Finding `CA-NNP-R12-2` — `--repo-root` does not contain declared paths to that repository

Check: the promised complete fixture/repository identity boundary

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `documentation-rule-projection-rule.md`'s “real repository file”
contract, `gate-self-test-rule.md`, `verifiable-claims-rule.md`, and the
contribution-auditor plausibility / rule-soundness dimensions

Finding state: open

Risk disposition: none

Round 10/11 correctly threads repository, schema and distribution roots as
explicit parameters, including through `validate_file_against`. But the
existing path resolver remains `repo_root.join(relative).is_file()`. In Rust,
joining an absolute path discards the base, and a relative path containing
`..` can traverse above it. No schema constraint rejects either shape and no
canonical containment check exists. Therefore `--repo-root <fixture>` does not
make path resolution reflect only the fixture: a manifest can name an
absolute caller-machine file or `../` path and pass because that outside file
exists. Symlink traversal is also left undecided because `is_file()` follows
the link target.

Concrete failure scenario: the isolated fixture uses
`normative_owner.stage: existing` and an absolute path to a file in the caller
checkout. `projection-check-all --repo-root <fixture>` accepts it even though
the path is absent from the fixture, directly falsifying the plan's repeated
claim that manifest discovery and path resolution operate against the fixture
tree “never the caller checkout.” The same escape applies to
`changed_pages_or_runbooks` and every surface `path`/`paths` entry. Current
repository manifests were scanned and use relative in-repository paths, so
this is not a current-corpus compatibility problem; adopter-owned manifests
remain unknown.

Assumed intent: all resolving projection paths are repository-relative and
must identify files contained by the selected repository root.

Resolution condition: define and implement a fail-closed repository-contained
path contract (reject absolute paths and parent traversal; explicitly decide
symlink containment), apply it consistently to changed files and every surface
path, and add `--repo-root` negative tests for absolute and `..` escapes plus
the decided symlink case. Because those path shapes currently pass when they
resolve, add them to the BREAKING compatibility scan, migration guidance and
CHANGELOG scope, or narrow any claim only if a governor-reviewed alternative
preserves the real-repository-file contract.

## Advisory `CA-NNP-R12-A1` — the local CI evaluator still overstates platform proof

Check: evidence strength for “GitLab actually creates the job”

Check status: reviewed

Outcome: needs-review

Requirement strength: recommended

Impact: minor

Enforcement: advisory

Basis: `verifiable-claims-rule.md` and the contribution-auditor
evidence-quality dimension

Finding state: open

Risk disposition: none

Round 11 now models both workflow admission and job selection, resolving the
specific missing-stage defect in `CA-NNP-R11-2`. However, the evidence remains
a new self-authored, dependency-free approximation of GitLab's `rules:if` and
YAML evaluation semantics. The plan does not define its supported expression
grammar, first-match behavior, unsupported-expression failure mode, or a
differential check against GitLab CI Lint/live platform output, yet repeatedly
says this proves “GitLab actually creates the job.” A model can agree with its
own expected answers while diverging from the platform.

Resolution condition: either narrow the claim to structural/local-model
evidence and document a fail-closed supported subset, or add authoritative
GitLab CI Lint/live-pipeline conformance evidence for the contexts claimed.
This does not reopen the specific workflow-admission omission from round 11.

## Prior-finding reconciliation

- `CA-NNP-R11-1` is resolved in its named scope: `schema_path` is now required
  through `validate_file_against`, and the new two-commit/non-empty-diff test
  exercises that call path under a conflicting caller `AGOV_SCHEMA_DIR`.
- `CA-NNP-R11-2` is resolved in its named scope: the design now explicitly
  evaluates workflow admission before job selection and distinguishes the
  two non-default push cases. `CA-NNP-R12-A1` concerns how strongly the local
  model may be represented as GitLab-platform proof, not the missing stage.
- `CA-NNP-R10-1..2` remain resolved: no new environment variable or shared
  root override is introduced, and schema/distribution inputs are explicitly
  rooted. `CA-NNP-R12-2` is a separate surviving path-containment boundary in
  the existing resolver.
- `CA-NNP-R9-2` remains resolved: `agov-validators/src/lib.rs` and exact module
  registration are named, with test-enumeration evidence required.
- Earlier findings remain resolved within their named scope: the mechanism is
  normative-owner-only; plan-status correlation is eliminated; the direct
  strict command is named as the future-user Gate 2 gate; mixed-`paths`
  staleness is fail-closed on any resolving member; both previously known
  BREAKING cases and the CHANGELOG obligation are recorded; the governance
  job is made unconditional on admitted MR/push/tag pipelines; and the plan no
  longer claims its own implementation carries a real candidate manifest.

## Exact-head, GitLab and CI evidence

- MR !1847 is open, non-draft, conflict-free and mergeable into `main`, with
  source head exactly `c75b931dc33e4404156b3edbca72fdb0bbd01b35` and diff
  base `a56da0977701674250cbb410475b41d42164ef2b`. The complete twelve-commit
  contribution adds only the 1,833-line plan; `git diff --check` is clean.
  Fresh `origin/main` at audit start is
  `e1ae69ec40767951a3cb2034cf2bc21ddfb616fe`.
- The live MR has 87 discussions/notes and zero unresolved resolvable notes.
  The approvals endpoint says `approved: true` only because zero approvals are
  required; `approved_by` is empty and `user_has_approved: false`. This is not
  governor Gate 1 approval. Issue #115 remains open with `bug` and `plan`
  labels. Handoff `hhg3` is visible on `origin/main` and names this exact head.
- All 51 open MRs targeting `main` were scanned with their live head SHA,
  `updated_at` and changed paths. No other MR touches this plan. MR !1965
  touches future implementation path `CHANGELOG.md`; it is not a plan-review
  collision, but implementation must refresh/reconcile that path after Gate 1.
- Canonical exact-head pipeline `2785792711` is the sole pipeline for this SHA
  and passed. Job `16070469547` (`agov:governance:validate`, 209.508686s)
  checked out exact commit `c75b931d`, built `agov 2.6.1` from that head, and
  validated the plan with blocking severity; its complete trace is 56 lines.
  Job `16070469548` (`commercial-corpus:validate`, 78.552405s) passed with its
  declared advisory corpus findings; its complete trace is 58 lines. Both are
  non-allow-failure. No redundant same-SHA paid pipeline existed to cancel.
- Duplicate suppression at cold start and again before authoring found no
  concurrent exact-head audit publication for `c75b931d`; only the expected
  planner handoff referenced this SHA.

## Final recommendation

**COMPLETE / BLOCK.** The round-11 schema-path and workflow-admission defects
are fixed, but the lifecycle still admits a pathless/empty `planned` marker
that makes the strict Gate 2 proof vacuous, and the promised fixture-root
boundary does not contain projection paths to that root. Correct both on one
new exact head, refresh the live carrier and migration classification, validate,
and request another fresh COMPLETE audit. Do not start implementation or route
Gate 1 while these blockers remain open.
