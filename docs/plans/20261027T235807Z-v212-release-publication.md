---
id: 20261027T235807Z-v212-release-publication
status: approved
gate1_approved_by: 20261028T001516Z-governor-reviewer-kman
gate1_approved_at: "2026-10-28T00:15:16Z"
topic: v212-release-publication-child-plan
candidate_release: v2.12.0
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Corrected-plan review `20261028T000412Z-infosec-sentinel-s7g1` is CLEAR
      with complete coverage and no findings at content SHA-256
      34676d9dbf9c8f67e261aaea1372a9532fbd9fdc26835f53f03b5d2f984d63ad.
      It confirms the namespace, exact protected-tag, credential, immutable-tag,
      anonymous-byte, post-publication identity, and fail-forward controls.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Corrected-plan review `20261028T000402Z-contribution-auditor-ajra` is CLEAR
      with complete coverage and no findings at content SHA-256
      34676d9dbf9c8f67e261aaea1372a9532fbd9fdc26835f53f03b5d2f984d63ad.
      It confirms all fourteen overlaps are acknowledged, frozen provenance and
      historical references are preserved, final-base and tree identity are
      fail-closed, and publication remains separated from status synchronization.
affected_paths:
  - docs/plans/20261027T235807Z-v212-release-publication.md
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
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: publish AGF v2.12.0

## Outcome and authority boundary

Publish v2.12.0 from reviewed current `main` through the standard release
workflow in `agf/rules/workflow/release-via-merge-request-rule.md`.

The release-content freeze is complete at `main`
`35eedd854c9a604a0f7d811e95b1eabbb9c0c655`: frozen implementation MR !3320
merged there after exact-head Gate 2 decision
`20261027T234948Z-governor-reviewer-rmps`. The frozen content is the retained
skill-command correction, tooling-coexistence guidance, homepage modernization,
and bounded v2.11 shipment-status corrections recorded in
`docs/reviews/documentation-estate/20261027-v212-release-content-freeze.md`.

This plan requests Gate 1 for the publication procedure only. Gate 1 approval
authorizes one preparatory external mutation: create and read back only the
exact `v2.12.0` Maintainers-only protected-tag rule specified below. It does
not authorize a version edit, release branch, metadata MR merge, tag, package,
Release, OCI publication, Pages deployment, announcement, or post-publication
status edit. Those actions occur only at the named gates below.

## Preconditions

Before the first release-metadata edit:

1. `main` still contains merge `35eedd854c9a604a0f7d811e95b1eabbb9c0c655`
   and every frozen item, review, and Gate 2 record. After this plan and its
   review trail merge, record the exact release-construction base, enumerate
   every intervening commit and changed path since `35eedd854...`, and prove
   they add only governed publication-plan/review artifacts. Any other `main`
   advance stops for renewed inspection and explicit re-freeze.
2. The exact release-construction base has a green pipeline and no stranded
   release-scope review, acknowledgement, or decision record.
3. The documentation-estate freeze report remains complete and every finding
   remains dispositioned.
4. Runner and DinD capacity are sufficient for the release MR and one
   irreversible tag pipeline.
5. An exact protected-tag rule for `v2.12.0` exists with creation restricted to
   Maintainers (access level 40), and its API object is read back while the tag
   is absent. Gate 1 authorizes creating only this exact rule if absent. A
   different or pre-existing nonmatching policy shape stops the release.

The thirteen additional `depends_on` plans overlap release-truth paths. Their
landed results are preserved, not reimplemented: work-item stewardship,
priority/challenge behavior, plan-collision enforcement, pending consultation,
roadmap truth, Windows and Elixir retirement, instruction-context reductions,
recall corpus, timer wording, v2.11 changelog reconciliation, and supersession
docs. If any predecessor reopens or its landed result differs at the final base,
stop and renew the collision/scope review.

This is the standard current-main path. No maintenance-line or frozen-scope
reconstruction exception applies.

## 1. Re-derive the exact metadata allowlist

At the frozen base, run both audits and retain their complete output:

```sh
git grep -n -F '2.11.0' -- \
  ':!docs/coordination' ':!docs/plans' ':!docs/reviews' \
  ':!CHANGELOG.md' ':!agf/CHANGELOG.md' ':!agf/learnings'
git grep -n -F '2.11.0' -- agf/learnings/foundry.yaml
```

Classify every occurrence, with per-file, per-category, and total counts that
reconcile before and after editing:

- **A: current-version cells.** Change only authored cells whose truth becomes
  v2.12.0: `agf/VERSION`, assistant-loader markers, adopter/example version
  cells, shipped agent definitions and manifest, current governance version
  cells, the live foundry adoption cell, and Rust workspace/path-dependency
  versions. Regenerate `Cargo.lock`; never hand-edit it.
- **B: live publication-status cells.** Keep claims that v2.11.0 is the latest
  published release unchanged in the release-metadata MR. Enumerate them now so
  they cannot be accidentally bumped. After anonymous v2.12.0 verification,
  update only those enumerated live cells in a separate status-sync MR.
- **C: historical references.** Keep shipped-feature provenance, old release
  baselines and links, issue history, portfolio history, tests, and examples
  whose meaning is specifically v2.11.0 byte-identical.

The two audits are the completeness authority. The affected-path list above is
a collision boundary, not permission to edit every named file: the reconciled
A table is the exact release-MR cell allowlist, and the reconciled B table is
the later status-sync cell allowlist. A new path or unmatched occurrence stops
the release for plan amendment or renewed review.

## 2. Freeze both changelogs

Convert `agf/CHANGELOG.md` `[Unreleased]` into `2.12.0` dated at the cut and
retain exactly the three frozen adopter-facing groups:

1. evidence-calibrated third-party tooling coexistence guidance;
2. homepage routes and reviewed responsive/accessibility presentation; and
3. skill-command and v2.11 shipment-status documentation corrections.

Every adopter-facing row keeps its `Adopter docs:` statement and calibrated
maturity/evidence wording. Add a concise repository-level `CHANGELOG.md`
v2.12.0 entry that points to the canonical adopter changelog and describes the
release as documentation-only. This is standard release-truth metadata, not a
new repository-operations/content claim and not a reversal of the freeze
report's finding that no repository-operation row was needed. Do not invent a
commit count, compatibility, integration, performance, or adoption claim. The
final wording is a separate human confirmation gate before the release-metadata
commit.

## 3. Build and prove the release-metadata commit

After explicit human confirmation to begin the version bump:

1. Create governed branch **`release-2.12.0`** from the still-frozen `main`.
2. Apply only the reconciled A cells and the two changelog edits to the dirty
   working tree. Do not update B publication-status claims before publication.
3. Validate the exact dirty bytes: full Rust workspace test/clippy/deny as
   configured by current CI; `agov validate`; `agov validate adopter-docs`;
   `agov validate assistant_loader`; `agov dist validate`; `agov dist build`;
   each built framework archive's shipped tests; strict site build; release-path
   validation; SPDX/license checks; and release artifact self-report checks.
4. Use a fresh temporary Git index seeded with `git read-tree HEAD`; stage the
   draft there and record `git write-tree`. Delete the temporary directory.
5. Commit exactly those tested bytes. Require a clean worktree,
   `HEAD^{tree}` equal to the recorded draft tree, and a base-to-head diff
   confined to the reconciled allowlist.
6. From that clean unpublished commit, run `agov release namespace-check
   --phase preflight` with project identity, target `main`, package name, and OCI
   reference derived by the governed tooling. Require authenticated, fully
   paginated `ABSENT` results for Git tag, GitLab Release, generic package in all
   statuses, and exact OCI tag; ambiguity or access failure stops.
7. First-push `release-2.12.0` with the governed `ci.skip` push option, open the
   MR to `main` immediately, and verify the skipped push pipeline plus exactly
   one running MR pipeline.

At cut time, derive the complete mandatory non-manual MR job matrix from the
current `.gitlab-ci.yml` rules for the actual version/changelog diff. Record
expected, observed, optional/manual, and tag-only jobs; every expected job must
be green at the exact MR head. Gate 2 independently reviews the allowlist,
changelogs, tested-tree identity, preflight, job matrix, archives, and diff.

## 4. Merge, recheck, tag, and publish

After binding exact-head Gate 2 and metadata-MR merge:

1. Prove live `main` equals the reviewed release merge commit; its first parent
   equals the reviewed target/release-construction base; its second parent
   equals the Gate-2-approved release head under the project's merge strategy;
   and its tree equals both the approved release-head tree and recorded clean
   metadata tree. Any drift stops.
2. Re-read the exact `v2.12.0` Maintainers-only protected-tag rule and require
   it to match the preflight object. Confirm tag-pipeline runner/DinD capacity.
3. Re-run `agov release namespace-check --phase pre-push` and require the same
   four authenticated `ABSENT` results.
4. Immediately after that namespace check, with no intervening read or action,
   create one annotated `v2.12.0` tag on the proven merge commit and push it
   once. Never move, delete, or recreate it. The merged metadata MR is the
   release-rule authorization; the operator records the irreversible action.

## 5. Verify public bytes and synchronize status

Release completion requires all of:

- the tag pipeline's required release, package, OCI, and verification jobs
  succeed;
- `agov dist post-publication-check` downloads through anonymous HTTPS into a
  fresh temporary directory and verifies archive hashes, checksum sidecars,
  provenance, source commit, executable self-report, package identity, and OCI
  digest/reference chain;
- the annotated tag object, tagger/message, and peeled commit match the reviewed
  merge;
- the exact `v2.12.0` protected-tag rule is re-read after publication and still
  matches the pre-push Maintainers-only object; and
- temporary downloads and credentials are absent from retained evidence.

The completion record names the tag object and peeled commit, GitLab Release
identity, archive names and SHA-256 values, provenance identities, generic
package identity, OCI digest, protected-tag-rule object, tag pipeline and job
IDs, and anonymous verification result. A missing or widened protected-tag rule,
or any namespace/identity mismatch, stops completion and status synchronization
and requires forward correction without mutating the tag.

Only after that record is published, create a separate branch/MR updating the
enumerated B cells in `agf/docs/governance/manifest.md`,
`agf/docs/site/roadmap.md`, and `docs/roadmap.md` to state v2.12.0 is the latest
published and independently verified stable release, with its exact tag object,
peeled commit, date, and Release link. Normal Gate 2 applies. Pages deployment
and any announcement follow the verified status-sync merge, not the tag alone.

## Failure and rollback

Before tag push, abandon the release branch/MR or correct it through a new
reviewed head; no release object exists. Any base drift, non-absent namespace,
policy mismatch, credential ambiguity, missing expected job, tree mismatch, or
pre-push archive mismatch stops publication. After tag push, any anonymous
archive, identity, policy, or verification mismatch stops release completion,
status synchronization, Pages deployment, and announcement. Immutability still
applies: keep the tag and correct forward as v2.12.1; never delete or retarget
v2.12.0.

## Gate 1 decision requested

Approve the standard current-main procedure, the derived A/B/C allowlist and
count reconciliation, the `release-2.12.0` branch, explicit version-bump and
changelog confirmation gates, dirty-draft/tree-identity proof, current-rule job
matrix, four-namespace checks, creation/readback of only the exact
Maintainers-only protected-tag rule, anonymous byte verification, and separate
post-publication status-sync MR. No implementation or irreversible release
action is requested by this draft.
