---
id: 20261029T220500Z-v213-release-publication
status: pending-gate1
topic: v213-release-publication
candidate_release: v2.13.0
advisory_consultation:
  - role: contribution-auditor
    status: pending
    reason: Independent review requested before Gate 1.
  - role: infosec-sentinel
    status: pending
    reason: Independent publication and residual-risk review requested before Gate 1.
affected_paths:
  - docs/plans/20261029T220500Z-v213-release-publication.md
  - AGENTS.md
  - CHANGELOG.md
  - agf/ADOPTING.md
  - agf/CHANGELOG.md
  - agf/VERSION
  - agf/agents/MANIFEST.yaml
  - agf/agents/academic-peer-reviewer.yaml
  - agf/agents/academic-visual-reviewer.yaml
  - agf/agents/build-engineer.yaml
  - agf/agents/capability-developer.yaml
  - agf/agents/coder-tester.yaml
  - agf/agents/contribution-auditor.yaml
  - agf/agents/deployment-engineer.yaml
  - agf/agents/documentation-estate-steward.yaml
  - agf/agents/documentation-writer.yaml
  - agf/agents/fact-checker.yaml
  - agf/agents/governor-reviewer.yaml
  - agf/agents/improvement-propeller.yaml
  - agf/agents/independent-implementation-reviewer.yaml
  - agf/agents/infosec-sentinel.yaml
  - agf/agents/planner-architect.yaml
  - agf/agents/web-ux-reviewer.yaml
  - agf/docs/governance/governance.md
  - agf/docs/governance/manifest.md
  - agf/docs/site/roadmap.md
  - agf/learnings/foundry.yaml
  - agf/templates/AGENTS.md
  - agf/templates/learning-record.yaml
  - docs/roadmap.md
  - tooling/rust/Cargo.lock
  - tooling/rust/Cargo.toml
depends_on:
  - 20261026T151646Z-next-release-scope-intake
  - 20261027T230355Z-v212-release-freeze-corrections
  - 20260913T085123Z-work-item-steward-gate1
  - 20260913T093405Z-row5-s3-priority-ordering-and-challenge-response
  - 20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion
  - 20260915T075705Z-issue130-pending-advisory-consultation-status
  - 20260916T002012Z-roadmap-refresh-post-v2101-issue130
  - 20261021T220000Z-retire-windows-support
  - 20261022T205113Z-instruction-context-reduction
  - 20261023T045214Z-remove-elixir-references
  - 20261023T080431Z-instruction-context-followup
  - 20261023T195516Z-recall-smoke-criterion-and-weak-paraphrases
  - 20261024T070435Z-timer-reality-polling-alignment
  - 20261025T183939Z-v211-changelog-and-scope-reconciliation
  - 20261025T192020Z-supersedes-adopter-docs
  - 20261029T163752Z-v213-release-scope
  - 20261029T204751Z-v213-release-freeze-preparation
  - 20261027T235807Z-v212-release-publication
  - 20261027T045944Z-record-only-publication-termination
  - 20260920T083259Z-distribution-scan-profiling-slice-b
  - 20261029T192050Z-dist-profile-roi-amendment
  - 20261028T194451Z-ci-reproducibility-pins
  - 20261029T192922Z-release-publisher-api-amendment
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Publish AGF v2.13.0 from the verified content freeze

## Outcome, authority and frozen baseline

Publish v2.13.0 through the standard current-main release workflow. The verified
freeze is `f02eb6b2e9b43887d92150a9b5bb00deecce4045` (!3378), following the
risk-record merge `03aad424f126e8b44325e8a7c16a1b7146f78bcc` (!3379).
Both merges and their actual parents/trees were verified under pre-published
`20261029T215632Z-governor-reviewer-jp1q` in shared commit
`aef24d0a7c091c9d6f37823dbfe35af506a4f08b`. Its audit-only carrier !3380 is now merged and verified at
`bd341ab11e0b789ff8bebc277f33b9cbf9983336`; no feature bytes changed. No maintenance or reconstruction exception applies.

The three approved slices are bounded record-only publication termination
(!3354, ee2a768a), opt-in distribution profiling (!3372, e68d162e), and CI
pins/create-only release publication (!3358, c7ea3cfb). The freeze adds the
reviewed factual urgency-status correction and reconciled changelogs. The full
six-dimension estate report is `docs/reviews/documentation-estate/20261029-v213-release-content-freeze.md`.
All 41 projection manifests passed; combined-source tests passed 852 with four
disclosed ignores, and both archives passed 28 shipped tests. These are freeze
evidence, not substitutes for validating the release metadata and its CI.

The report reconciles 48 first-parent merges through 6042bc8b. Later 4b8b384b
and 03aad424 carry only audit/decision records; f02eb6b2 is the reviewed freeze
itself. Before implementation, enumerate every further merge/path from this
exact freeze through the final construction base. Only this publication plan,
its review/approval trail and the exhausted jp1q carrier may intervene. Any other
content requires renewed inspection and explicit re-freeze before proceeding.
Require the final construction base's relevant pipeline to pass.

Gate 1 requests authority to create/read back one exact protected-tag rule and
to implement the concrete metadata/changelog edits below on `release-2.13.0`.
It grants no merge or tag authority. A separate exact-head governor Gate 2
precedes the metadata merge; that merged MR supplies tag/push authorization under
the release rule. The user's governor delegation ss2k lasts through
2026-10-30T18:00Z and covers these decisions; no new delegation or role activation
is requested. Root owns publication work; existing separate Codex reviewers
provide contribution/security reviews while Claude is unavailable. No new
sessions or timers. Preserve all existing claims and root-checkout changes.

The dependencies acknowledge overlapping release-truth paths and preserve their
landed semantics; they neither close predecessor plans nor transfer claims.
Recheck changed-plan collisions before approval and at the final base.

## Exact metadata inventory and allowed edits

Independent read-only audit of frozen f02eb6b2 reconciles all 138 occurrences of
`2.12.0` across 64 files: A 36/27 files, B 9/3 files, C 93/35 files. The mixed
governance manifest occurs in A and B. Re-run `git grep -n -F '2.12.0' <base>`
at the final construction base and classify added plan/review history separately.
Unmatched cells or new A/B paths stop for amendment. No blanket replacement.

A cells change to 2.13.0 in the metadata MR. Line numbers refer to frozen f02eb6b2:

| File | Exact current-version lines | Count |
| --- | --- | --- |
| AGENTS.md | 4 | 1 |
| agf/ADOPTING.md | 333 | 1 |
| agf/VERSION | 1 | 1 |
| agf/agents/MANIFEST.yaml | 11 | 1 |
| agf/agents/academic-peer-reviewer.yaml | 17 | 1 |
| agf/agents/academic-visual-reviewer.yaml | 17 | 1 |
| agf/agents/build-engineer.yaml | 36 | 1 |
| agf/agents/capability-developer.yaml | 36 | 1 |
| agf/agents/coder-tester.yaml | 15 | 1 |
| agf/agents/contribution-auditor.yaml | 29 | 1 |
| agf/agents/deployment-engineer.yaml | 15 | 1 |
| agf/agents/documentation-estate-steward.yaml | 42 | 1 |
| agf/agents/documentation-writer.yaml | 15 | 1 |
| agf/agents/fact-checker.yaml | 28 | 1 |
| agf/agents/governor-reviewer.yaml | 15 | 1 |
| agf/agents/improvement-propeller.yaml | 8 | 1 |
| agf/agents/independent-implementation-reviewer.yaml | 34 | 1 |
| agf/agents/infosec-sentinel.yaml | 26 | 1 |
| agf/agents/planner-architect.yaml | 15 | 1 |
| agf/agents/web-ux-reviewer.yaml | 33 | 1 |
| agf/docs/governance/governance.md | 5 | 1 |
| agf/docs/governance/manifest.md | 11, 38, 76 | 3 |
| agf/templates/AGENTS.md | 3 | 1 |
| agf/templates/learning-record.yaml | 8 | 1 |
| agf/learnings/foundry.yaml | 38 | 1 |
| tooling/rust/Cargo.toml | 14, 20, 21, 22 | 4 |
| tooling/rust/Cargo.lock | 13, 36, 48, 61, 72 | 5 |

Regenerate Cargo.lock through Cargo; never hand-edit it. Require exactly the
five workspace version changes, with no external dependency/lock drift.

B cells remain byte-identical in the metadata MR: governance manifest line18;
site roadmap lines15,17,23,27; root roadmap lines10,12,17,25. After verified public
bytes, a separate status-sync MR updates these nine literal-bearing cells plus the
following bounded companion fields in the same three files. The literal count
is not the complete semantic status-sync edit boundary:

| File | Companion fields at frozen baseline |
| --- | --- |
| agf/docs/governance/manifest.md | Line19 tagged date, agreeing with actual tag evidence |
| agf/docs/site/roadmap.md | Line16 Observed at; lines23-26 current-release paragraph publication date, actual tag-pipeline job count and verification evidence |
| docs/roadmap.md | Line11 Observed at; lines17-21 current-release paragraph publication date, actual tag-pipeline job count and completion-record citation |

Use v2.13.0, the actual tag object/peeled commit, UTC observation timestamp,
publication date, actual pipeline/job evidence, new completion record and
Release URL. Do not retain the old 23-job count or gm8j verification as proof
for v2.13. Preserve historical release sections and unrelated roadmap rows. Gate 1 includes this later bounded work;
independent exact-head Gate 2 remains required before its merge.

C stays byte-identical: coordination 53 occurrences/19 files, plans 26/7,
reviews 6/4, historical changelog sections 2/2, and six occurrences in three
other files: archived coordination b7jd line96; rule-trigger recall corpus
line403; release_create.rs read-only job-token probe lines416,422,448,458. New audit records
may cite historical versions; they never authorize replacing existing C cells.
`harbor-workshop.yaml` remains the historical 2.5.0 submission. CLAUDE.md pointers,
README, derived release manifest and old v2.11 shipment provenance need no edits.

## Changelog release-truth edits

In both changelogs add an empty Unreleased section and turn the frozen existing
Unreleased body into the v2.13.0 section, dated at the actual cut. Preserve all
v2.12-and-older bytes and the frozen feature scope. In the root introduction,
change only the cross-reference from Unreleased to 2.13.0. Retain the qualified
profiling baseline and publisher's pre-publication Level 4/test/read-only evidence;
do not claim production POST success before it occurs.

In the adopter record-only entry, replace only the pending-shipment sentence
with: "Capability: Level 5 Deployable in the v2.13.0 distribution when using
compatible artifacts." Keep Empirically informed and the no-broad-adopter-proof
qualification. This is the release artifact's scoped delivery statement, not a
premature live latest-release status claim. Retain all Adopter docs lines and
the already-shipped v2.11 urgency correction. Final exact wording is reviewed
as part of the metadata MR Gate 2; no additional feature or compatibility claim.

## Preconditions, namespace and tag protection

Before version edits, reobserve all freeze prerequisites and dependency results,
check active claims/open MRs, resolve review obligations, and confirm default
runner plus current Podman OCI runner capacity for one tag pipeline. Preserve
ja28's residual-risk scope and report; it accepts mutable APK/tool/CA drift and
availability only for this reviewed publisher and this v2.13 cut until
2026-10-30T18:00Z. New known vulnerability, material publisher change, expired
authority or unfinished cut at expiry stops for a fresh binding decision.
No known-vulnerability waiver, full-job reproducibility or production POST proof
is inferred. The risk record is not an exhausted publication-only permission.

Require an exact `v2.13.0` protected-tag rule with Maintainers-only creation
(access40). Gate 1 permits creating only that exact rule if absent, while the
tag is demonstrably absent; read back and retain its full policy object. Any
pre-existing nonmatching policy shape stops; do not edit/delete it. Keep this
object identical at pre-push and post-publication verification. Enumerate the
complete paginated protected-tag policy collection and evaluate every exact or
wildcard match for v2.13.0. Effective creation access must remain Maintainers-only;
a broader user/group/deploy-key grant, ambiguous policy or incomplete listing
stops. Retain the matching policy set and recheck it before push and after
publication. No authority to edit/remove any other protection rule.

Use the existing `agov release namespace-check` from a clean committed release
checkout at both `--phase preflight` and `--phase pre-push`. Derive project
83321848/path janipaijanen/agents-govern-framework from configured origin,
package name from the validated manifest, OCI reference from the governed
image-ref helper and verify-ref agreement. Use an already-authorized credential
through the environment only; never print/store it or put it in argv. Require
positive authenticated access and complete pagination for Git tag, GitLab
Release, exact generic-package name/version in every status, and exact OCI tag.
All four must be ABSENT. Ambiguous/inaccessible/malformed results stop; no
manual 404 reinterpretation, cached registry repository ID or reserved-namespace
claim. CI's mandatory non-allow-failure release:namespace-check binds the source,
live target and pipeline; it does not replace either operator check.

## Validate, publish metadata, merge and tag

1. Create `release-2.13.0` from the reobserved construction base on main in a
   clean existing worktree. Edit only A cells and the two changelogs. No CI,
   source, dependency, profile-default, schema, risk-report or B/C edits.
2. Validate exact draft bytes: release-mode workspace/all-features tests,
   configured clippy/deny and all agov validators, adopter docs/assistant loader,
   distribution validate/build, both archives' shipped tests/self-sufficiency,
   artifact self-report, strict site/hooks/link checks, release paths and licenses.
   Record disclosed intentional ignores; never convert them to passing tests.
3. Seed a fresh temporary index from HEAD, stage only the allowed draft paths,
   and record write-tree; delete the temporary index. Commit exactly those
   validated bytes. Require a clean worktree, HEAD tree equality and an exact
   base-to-head allowlist diff. Reconcile A=36 changes, B=9 unchanged and C
   unchanged before publishing; derived Cargo.lock is included in those counts.
4. Run operator namespace preflight. First-push with `git push -o ci.skip`,
   immediately open the metadata MR to main, verify one skipped push pipeline
   and one canonical job-running MR pipeline. Derive expected mandatory,
   optional/manual and tag-only job matrices from current CI and the actual
   diff; require every expected non-manual job green at the exact reviewed head.
5. Publish binding exact-head governor Gate 2 after independent review of
   metadata, changelogs, tested tree, namespace and artifacts. Integrate any
   audit-only target advance before final acceptance; rerun affected validation
   and exact-head CI. Any other advance requires renewed freeze review.
6. Merge the exact approved head without squash. Prove live main's merge has
   first parent equal to the reviewed target, second parent equal to approved
   head, and tree equal to both the approved head and tested metadata tree.
   Verify no unapproved intervening source or target content.
7. Re-read protected-tag object and effective matching policy set, authority/risk
   validity and runner capacity.
   Run namespace pre-push on that exact proven merge. Immediately on success,
   create one annotated v2.13.0 tag on the merge and push once in a single
   action. Do not stop for extra confirmation. Never move/delete/recreate it.
   Protection and exact tag identity checks constrain races but are not atomic
   with the publisher's POST; retain this operational limit.

## Public verification, status sync and failure handling

Require every expected tag-pipeline package, release, OCI and verification job
to pass. Run `agov dist post-publication-check` using fresh anonymous HTTPS
actual downloads: hashes/sidecars, provenance/source commit, executable self
report, package identity and OCI reference/digest chain must agree. Verify the
annotated tag object/tagger/message and peeled merge, and re-read the unchanged
Maintainers-only protection object and matching policy set. Retain no
credentials/download residue.

Publish a completion record with those identities, archive names/checksums,
Release/package/OCI identities, tag pipeline/jobs, protection object and anonymous
verification result. Only then implement B-and-companion-only status synchronization under
its own exact-head Gate 2. Pages publication and release announcement follow
verified status-sync merge; no release-complete claim from tag creation alone.

Before tag push, correct/review the reversible metadata branch or abandon it.
Any missing job, base/tree/namespace/policy mismatch, expired authority or
security ambiguity stops. After push, any failure stops completion/status sync;
reconcile uncertain POST by read-only inspection, never blindly retry a paid job
or POST. Preserve the immutable tag and correct forward through an explicitly
authorized v2.13.1 plan; this plan does not authorize that future release.
