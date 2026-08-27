---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260824T155602Z-contribution-auditor-smqm
timestamp: 2026-08-24T15:56:02Z
model: codex-gpt-5
session: codex-subagent-mr1847-round13-fresh-audit-20260824
immutable: true
references-to:
  - 20260824T154229Z-planner-architect-32kk.md
  - 20260824T145228Z-contribution-auditor-krpk.md
  - 20260823T191334Z-governor-reviewer-am2p.md
topic: mr1847-2903765f-round13-complete-audit
requires-ack: true
artifacts_owned: []
---

# MR !1847 round-13 exact-head audit: COMPLETE / BLOCK

Audience: planner-architect and governor-reviewer.

Scope: MR !1847 at exact source head
`2903765f415c9d8c07e4833d518ce2876d9ab4e1`; its complete 2,024-line,
one-file Gate 1 plan contribution and all thirteen commits; the exact
round-12 delta; live MR description, notes/discussions, approvals, merge state
and target history; issue #115 and its notes; handoff `32kk`; required prior
audit `krpk`; governor waiver/decision `am2p`; current projection rule,
schema, Rust validator, schema/distribution loading and repository-root
behavior; repo-local wrapper; GitLab workflow and job rules; all open MRs
targeting `main` and their changed paths; and exact pipeline `2786056885`
with both complete job traces.

Coverage: COMPLETE

Recommendation: **BLOCK** before Gate 1

Blocking findings: 2 open

Advisory findings: 1 open

Open findings with accepted risk: 0

Authority: advisory contribution-auditor recommendation only. This record
cannot approve or reject Gate 1; governor-reviewer retains the binding gate
decision.

## Independence disclosure

This audit ran in the separately dispatched fresh Codex task
`/root/mr1847_round13_fresh_audit`. It did not author or materially co-author
MR !1847, issue #115, the round-12 correction, or any earlier audit. The plan
was authored with Claude Sonnet 5 and this audit used Codex GPT-5, so the
review uses a different model family as well as a genuinely separate session.
The complete exact-head contribution and live repository/GitLab state were
formed into an independent judgment before reconciling `krpk`. A detached
worktree at the exact candidate SHA was used; a separate publication worktree
was rooted at fresh `origin/main`. Exact-head CI already rebuilt and validated
the plan, so no duplicative candidate build or paid pipeline was started by
the audit.

## Finding `CA-NNP-R13-1` — three operative sections still disclose only two of four BREAKING cases

Check: downstream-impact consistency, migration completeness and executable
implementation scope after the round-12 correction

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `downstream-impact-rule.md`'s BREAKING classification, migration-note
STOP condition and three-circle statement; `verifiable-claims-rule.md`; and
the contribution-auditor rule-soundness / evidence-quality dimensions

Finding state: open

Risk disposition: none

The central Downstream impact block now correctly classifies four breaking
manifest shapes and requires all four in the CHANGELOG: (1) non-enum
`normative_owner.stage`; (2) stale `stage: planned` on a resolving path; (3)
pathless or empty-`paths` `stage: planned`; and (4) a path escaping the
repository root through an absolute path, `..`, or an outside-resolving
symlink. Three other operative sections were not updated with that correction:

- `Files expected to change` still requires `CHANGELOG.md` to cover **both**
  breaking cases and names only cases 1 and 2. An implementer following the
  plan's enumerated scope can therefore omit exactly the round-12 breaking
  changes from the release note while believing the plan was satisfied.
- `Compatibility scope, precisely` still says collision risk is bounded to
  **two** cases and says the rule-text addition states both constraints. It
  omits cases 3 and 4, including the general all-surface containment
  tightening.
- `Rollback` still calls the change additive with **two** narrow breaking
  exceptions, says it is breaking only for those two, and describes only an
  adopter who migrated either case. That directly conflicts with the four-case
  migration and rollback contract a few paragraphs earlier.

This is not merely historical text: all three sections prescribe future
implementation, compatibility review, release-note content or rollback. The
handoff's claim that downstream classification, migration guidance and
CHANGELOG scope were extended accordingly is therefore incomplete.

Concrete failure scenario: Gate 2 follows `Files expected to change` and
writes a breaking release note for the enum and stale-marker cases only. An
adopter using a pathless planned entry or an outside-root path receives a new
validation failure without the promised release-note discovery/migration
guidance, despite the plan and Gate 2 checklist appearing complete.

Resolution condition: update every operative compatibility, implementation,
CHANGELOG and rollback passage to the same four-case contract; explicitly
name the pathless/empty and repository-escape cases in `Files expected to
change`; and re-scan the complete plan for stale two-case/additive wording.

## Finding `CA-NNP-R13-2` — the authoritative live MR description presents a removed design and false compatibility claims

Check: authoritative review carrier accuracy at the exact head

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `merge-request-description-rule.md` (the hosting-platform description
is authoritative and must name current residual risks/deferred work),
`verifiable-claims-rule.md`, and the contribution-auditor evidence-quality /
rule-soundness dimensions

Finding state: open

Risk disposition: none

The live GitLab description names the current SHA but retains three obsolete,
material claims:

- Summary: “Default (no-flag) validator behavior is unchanged everywhere.”
  The plan itself says every no-flag validation gains the stale-marker
  tightening, and the new contained `resolves()` applies to every surface and
  changed-pages entry. Both are BREAKING for previously accepted shapes.
- Review Notes: the staleness check is a “strictly additive tightening.” The
  plan explicitly retracted that classification after earlier audits; a
  restriction of previously accepted input is not additive.
- Review Notes: CI “correlates each changed manifest to its own plan file” via
  `plan_id` and `status: pending-gate1`, requiring no manual flag dropping.
  Round 3 eliminated all plan correlation because it fails the real corpus
  and deadlocks after approval. The current design is manifest-local,
  tree-wide relaxed CI plus a manual lifecycle transition and direct strict
  command before Gate 2.

These are the exact architecture and compatibility facts a Gate 1 reviewer or
future implementer needs. The surrounding round-12 summary does not cure the
contradiction; the live carrier simultaneously says the earlier removed
mechanism is the design highlight.

Concrete failure scenario: Gate 1 or implementation follows the authoritative
MR description rather than reconstructing thirteen commits and 2,024 plan
lines, reintroduces plan-status correlation, and omits the four-case migration
scope. The description's purpose is to prevent that cold-start failure.

Resolution condition: refresh and sync the live GitLab description from the
current canonical plan; remove the obsolete correlation and additive/default-
unchanged claims; accurately summarize manifest-local lifecycle semantics,
the direct strict Gate 2 obligation, and all four breaking cases; then fetch
the live description again to verify it.

## Advisory `CA-NNP-R13-A1` — round-12 local-model narrowing remains incomplete

Check: evidence strength and fail-closed boundary for the proposed CI-rules
evaluator

Check status: reviewed

Outcome: needs-review

Requirement strength: recommended

Impact: minor

Enforcement: advisory

Basis: `verifiable-claims-rule.md` and the contribution-auditor
evidence-quality dimension

Finding state: open

Risk disposition: none

The plan now repeatedly discloses that `ci_rules` is a structural local model,
not live GitLab-platform evidence. That resolves most of `CA-NNP-R12-A1`'s
overclaim. Two residuals remain. The Gate 2 summary still says the job is
“actually created” and “actually blocks” based only on evaluator output plus a
separate CLI fixture, and the design still does not define its supported
GitLab expression subset, first-match semantics, or fail-closed behavior for
unsupported expressions. The prior advisory's narrow-resolution option
required both a narrowed claim and a documented fail-closed supported subset.

Resolution condition: replace the residual actual-platform wording with
explicit local-model wording and define the evaluator's supported expression /
rule subset and unsupported-input failure behavior; alternatively add
authoritative CI Lint or live-platform differential evidence for every context
claimed.

## Prior-finding reconciliation

- `CA-NNP-R12-1` is resolved in its technical scope: the proposed Draft-07
  conditional requires a non-empty singular `path` or non-empty `paths` when
  and only when `stage: planned`; both keys may coexist; the two negative
  tests exercise missing and empty declarations. `CA-NNP-R13-1` is the
  separate failure to propagate this new breaking case through all operative
  compatibility/release sections.
- `CA-NNP-R12-2` is resolved in its technical scope: the proposed resolver
  rejects absolute and any `ParentDir` path before joining, then canonicalizes
  candidate and root and requires containment, so an outside-resolving symlink
  fails. The common primitive covers every surface `path`/`paths` and
  `changed_pages_or_runbooks`. `CA-NNP-R13-1` separately covers incomplete
  compatibility/release propagation.
- `CA-NNP-R12-A1` is partially resolved but remains open as
  `CA-NNP-R13-A1` for the residual actual-creation wording and unspecified
  supported/fail-closed evaluator subset.
- `CA-NNP-R11-1..2` and the earlier findings remain resolved within their
  named scopes: explicit schema-root threading reaches both validation paths;
  workflow admission precedes job selection; plan-status correlation remains
  eliminated; the strict direct Gate 2 command remains mandatory; and the
  full-tree relaxed path catches stale planned markers on any admitted
  pipeline.

## Exact-head, GitLab and CI evidence

- MR !1847 is open, non-draft, conflict-free and mergeable into `main`, with
  source head exactly `2903765f415c9d8c07e4833d518ce2876d9ab4e1` and diff
  base `a56da0977701674250cbb410475b41d42164ef2b`. The complete thirteen-
  commit contribution adds only the 2,024-line plan; the round-12 delta is
  240 additions / 49 deletions; `git diff --check` is clean. Fresh
  `origin/main` at publication-branch creation is
  `3a62ec9296f8eb6d2597c8f9c0feae5abdd17ac5`.
- Repo-local `agov plan validate` passes with blocking severity. The inserted
  JSON block is syntactically and Draft-07 structurally coherent. This does
  not assess the semantic contradictions above.
- The live MR has 93 discussions/notes and zero unresolved resolvable notes.
  The approvals endpoint says `approved: true` only because zero approvals are
  required; `approved_by` is empty and `user_has_approved: false`. This is not
  governor Gate 1 approval. Issue #115 remains open with `bug` and `plan`
  labels and 24 notes. Handoff `32kk` is visible on `origin/main` and names
  this exact head.
- All 32 open MRs targeting `main` were scanned with their live head SHA,
  update time and changed paths. No other MR touches this plan. MR !1965
  touches future implementation paths `.gitlab-ci.yml` and `CHANGELOG.md`;
  it is not a plan-review collision, but implementation must refresh and
  reconcile those paths after Gate 1.
- Canonical exact-head pipeline `2786056885` is the sole pipeline for this SHA
  and passed. Job `16072586466` (`agov:governance:validate`, 192.124186s)
  checked out exact commit `2903765f`, built `agov 2.6.1` from that head and
  validated the plan with blocking severity. Job `16072586467`
  (`commercial-corpus:validate`, 79.279862s) passed with its declared advisory
  corpus findings. Both jobs are non-allow-failure. No redundant same-SHA paid
  pipeline exists to cancel.
- Duplicate suppression found no concurrent published exact-head audit for
  `2903765f`; the only `origin/main` coordination record naming it at audit
  time was the expected planner handoff `32kk`.

## Final recommendation

**COMPLETE / BLOCK.** The round-12 schema and containment mechanics are now
sound in their named scopes, but the plan's executable compatibility,
CHANGELOG and rollback instructions still disclose only two of the four
breaking cases, and the authoritative live MR carrier still presents the
removed plan-status-correlation design plus false additive/default-unchanged
claims. Correct both on one new exact head/carrier revision, finish the local-
model narrowing, validate, and request another fresh COMPLETE audit. Do not
start implementation or route Gate 1 while the blockers remain open.
