---
id: v250-final-release-publication-maintenance-line
status: pending-gate1
topic: v250-final-release-publication
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      A fresh exact-head review is requested with this planner-owned plan.
      Earlier review 20260809T150825Z-contribution-auditor-dqsd.md blocked
      MR !1325 on candidate identity, incomplete inventory and evidence,
      moving-base risk, namespace checks, and unauthorized authorship. This
      replacement preserves every accepted finding and is not an edit of the
      coder-tester-authored plan.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Governor security assessment
      20260809T171844Z-governor-reviewer-sq5j.md established that the active
      infosec-sentinel charter is information-leak screening, not release-ref
      supply-chain control. That assessment returned CLEAR for this topology
      and its required level-40 branch/tag protections, exact merge invocation,
      and retained-ref API evidence are incorporated here.
  - role: build-engineer
    status: consulted
    reason: >-
      20260809T170723Z-build-engineer-pste.md verified that the current MR and
      tag pipelines support an older maintenance target without CI or release-
      script changes. It also identified the accepted one-time over-inclusive
      first-push preview pipeline; this plan records rather than hides that
      non-safety cost.
---

# Plan: publish `v2.5.0` from the immutable RC2 candidate

## Decision requested

Authorize a metadata-only stable promotion of the exact published
`v2.5.0-rc.2` functional candidate through the protected maintenance-line
workflow now implemented on `main` by MR !1345.

This replacement is planner-architect-authored. It supersedes, rather than
edits, coder-tester-authored MR !1325 and plan
`docs/plans/20260809T144857Z-v250-final-release-publication.md`. It implements
the topology selected by Governor decision
`20260809T161825Z-governor-reviewer-2mmn.md` while preserving
`20260809T143855Z-governor-reviewer-qpkg.md` and
`20260809T153150Z-governor-reviewer-kprg.md`: stable must contain RC2's exact
functional candidate unchanged. The RC3/current-main alternative remains
rejected.

Gate 1 for this plan authorizes only the bounded release preparation,
validation, protected-ref setup, release MR, publication, and metadata-forward
sync described below. It does not itself approve either implementation MR,
create a tag, or authorize bypassing either exact-head Gate 2 decision.

## Preconditions and authority chain

The following must all remain true before any branch or protection setting is
created:

1. `qpkg`'s observation-window decision remains binding for `v2.5.0` final,
   and no candidate-specific defect has been found that triggers RC3.
2. Maintenance topology decision `2mmn` remains binding.
3. The prerequisite rule plan is approved and published as
   `docs/plans/20260809T163051Z-v250-maintenance-release-line-rule.md`.
4. Its implementation MR !1345 is on `main` as merge commit
   `1af67604a383c6bb3d1d07df8c11f60d3c64a2c2`, backed by binding Gate 2
   record `20260809T193605Z-governor-reviewer-ns86.md` and CLEAR review
   `20260809T192446Z-contribution-auditor-5sg3.md`.
5. This replacement plan has its own COMPLETE/CLEAR contribution-auditor
   review, binding Governor Gate 1 decision, approved front matter, and merged
   plan artifact before release execution begins.

Any failure of these preconditions stops the release. A merge event, green CI,
or an advisory CLEAR is not a substitute for binding Governor authority.

## Frozen candidate identity

All release work begins from these immutable RC2 anchors, never from current
`main`:

- candidate tag: `v2.5.0-rc.2`;
- annotated tag object:
  `2bdcbb76716ba84ffa78e792b380b7befdd9340e`;
- peeled candidate commit:
  `3874a5b92b006fc867a4cedf359cfc17c6dfbae9`;
- candidate tree:
  `2bd9b7be772859410b202dcb1fe296a4acc07d4b`;
- protected maintenance target: `release/2.5`;
- protected release source: `release-2.5.0`;
- intended stable tag: `v2.5.0`.

Before the first mutation, derive these values again from the canonical remote
and stop on any mismatch:

```bash
git fetch --force --tags origin
test "$(git rev-parse v2.5.0-rc.2)" = \
  2bdcbb76716ba84ffa78e792b380b7befdd9340e
test "$(git rev-parse 'v2.5.0-rc.2^{}')" = \
  3874a5b92b006fc867a4cedf359cfc17c6dfbae9
test "$(git show -s --format=%T 'v2.5.0-rc.2^{}')" = \
  2bd9b7be772859410b202dcb1fe296a4acc07d4b
git merge-base --is-ancestor 3874a5b92b006fc867a4cedf359cfc17c6dfbae9 \
  origin/main
```

The release MR base is always the peeled RC2 commit. A different source,
target, merge base, candidate tree, release-MR head, or target head invalidates
all earlier review evidence.

## Fail-closed namespace and protection preflight

Run the following before creating either branch, and repeat the four immutable
publication checks immediately before tagging:

1. Git tag: `git ls-remote --tags origin 'refs/tags/v2.5.0'` returns empty.
2. GitLab Release: `glab api projects/:id/releases/v2.5.0` returns HTTP 404.
3. Generic package: `glab api
   'projects/:id/packages?package_type=generic&package_name=agf&package_version=v2.5.0'`
   returns an empty array.
4. OCI: resolve the canonical `agov` registry repository and require
   `glab api registry/repositories/<repository-id>/tags/2.5.0` to return
   HTTP 404.
5. Repository branches `release/2.5` and `release-2.5.0` and exact protected-
   branch entries for both are absent. If a ref or protection entry already
   exists, do not reuse or delete it; return to the Governor with its exact
   SHA and policy.

The plan audit observed all four publication namespaces and both branch names
absent on 2026-08-09, and observed zero protected-tag rules. Those observations
are not durable reservations; execution must re-query authoritative APIs.

After the absence checks:

1. Create `release/2.5` at exactly
   `3874a5b92b006fc867a4cedf359cfc17c6dfbae9`.
2. Immediately protect the exact branch name with push and merge restricted to
   GitLab access level 40 (Maintainer), force-push disabled, and deletion
   denied. Re-read the exact protection entry through the API.
3. Create `release-2.5.0` at that same commit and apply the same exact-name,
   level-40, no-force-push/no-deletion policy before any content edit.
4. Create an exact protected-tag rule for `v2.5.0` with creation restricted to
   access level 40. Re-read it through the API; do not assume a wildcard.
5. Re-read both branches and prove their initial SHAs equal the peeled RC2
   commit. Record all API responses without exposing credentials.

Access level 30, a wildcard whose concrete match is not proven, a missing tag
rule, or any interval in which release content exists on an unprotected ref is
a STOP condition.

## Exact RC2 release-metadata boundary

The stable release commit may change exactly these 23 RC2-tree paths and no
others:

1. `CHANGELOG.md`
2. `agf/CHANGELOG.md`
3. `agf/VERSION`
4. `agf/ADOPTING.md`
5. `agf/agents/MANIFEST.yaml`
6. `agf/agents/academic-peer-reviewer.yaml`
7. `agf/agents/academic-visual-reviewer.yaml`
8. `agf/agents/build-engineer.yaml`
9. `agf/agents/capability-developer.yaml`
10. `agf/agents/coder-tester.yaml`
11. `agf/agents/contribution-auditor.yaml`
12. `agf/agents/deployment-engineer.yaml`
13. `agf/agents/documentation-writer.yaml`
14. `agf/agents/fact-checker.yaml`
15. `agf/agents/governor-reviewer.yaml`
16. `agf/agents/infosec-sentinel.yaml`
17. `agf/agents/planner-architect.yaml`
18. `agf/docs/governance/manifest.md`
19. `agf/docs/site/roadmap.md`
20. `agf/templates/AGENTS.md`
21. `agf/templates/learning-record.yaml`
22. `tooling/rust/Cargo.toml`
23. `tooling/rust/Cargo.lock`

This is the complete current-version surface found in the frozen RC2 tree by:

```bash
git grep -l '2\.5\.0-rc\.2' \
  3874a5b92b006fc867a4cedf359cfc17c6dfbae9 -- \
  CHANGELOG.md agf tooling/rust
```

Historical evidence strings outside this release-truth set remain unchanged.
The two post-RC2 roles, `independent-implementation-reviewer` and
`web-ux-reviewer`, do not exist in the candidate and must not be added. No
post-RC2 rule, documentation, schema, Rust behavior, test, dependency, CI, or
artifact-layout change may enter the release branch.

The metadata edits are:

- replace current-version `2.5.0-rc.2` with `2.5.0` in the named version
  surfaces;
- update the Cargo workspace version, its three internal path-dependency pins,
  and exactly the five lockfile packages `agov`, `agov-core`, `agov-coord`,
  `agov-mcp`, and `agov-validators`;
- add a new dated `[2.5.0]` section above `[2.5.0-rc.2]` in both changelogs;
  preserve the complete RC2 and RC1 sections rather than retitling them;
- state that this is a metadata-only stable promotion with zero functional,
  schema, documentation-content, migration, test, dependency, CI, or artifact-
  layout change from RC2;
- include exactly `Adopter docs: not applicable — exact RC2 functional
  candidate; metadata-only promotion`;
- carry forward `rq2n`'s disposition that `sjqm` findings 2–4 remain unresolved
  post-2.5.0 hardening candidates, not accepted risks and not defects introduced
  by this promotion;
- record actual final test counts and artifact identities, never copied assumed
  counts.

## Release MR construction and review

Commit the metadata change on `release-2.5.0` and open a non-draft,
non-squash, non-rebase MR targeting `release/2.5`. The MR description must use
all six governed headings, name both refs and the exact source head, list all 23
paths, include the five-field downstream-impact statement, record `Adopter docs
parity: not applicable`, and disclose the one-time first-push pipeline cost
identified by `pste`.

Classification:

- Change type: PREVENTIVE release metadata.
- Foundry impact: publishes the already validated RC2 candidate as stable.
- Direct adopter impact: version identity changes from RC2 to stable; functional
  bytes and behavior are unchanged.
- Downstream impact: no migration; consumers selecting stable receive the exact
  RC2 functional candidate.
- Migration: none.

Require an exact-head contribution-auditor review and a separate binding
Governor Gate 2 decision for the release MR. Any source-head or target-head
change after review requires refreshed validation, pipeline, review, and Gate 2.

## Validation and evidence before release Gate 2

Run from a clean worktree at the release MR's exact head and record exact
commands, versions, counts, artifact names, hashes, and exit results.

### Identity and delta

```bash
test "$(git merge-base HEAD origin/release/2.5)" = \
  3874a5b92b006fc867a4cedf359cfc17c6dfbae9
git diff --name-only origin/release/2.5...HEAD
git diff --check origin/release/2.5...HEAD
```

The changed-path output must equal the 23-path allowlist exactly. Compare the
candidate and release head trees and prove every non-allowlisted blob is
identical. A fresh current-version scan must find no overlooked live
`2.5.0-rc.2` surface and no post-RC2 role or content.

### Rust and governance validation

```bash
cargo fmt --manifest-path tooling/rust/Cargo.toml --all -- --check
cargo test --manifest-path tooling/rust/Cargo.toml --workspace
cargo clippy --manifest-path tooling/rust/Cargo.toml --workspace \
  --all-targets -- -D warnings
cargo build --release --locked --manifest-path tooling/rust/Cargo.toml \
  -p agov -p agov-mcp
./tooling/agov validate
./tooling/agov validate agents
./tooling/agov validate adopter-docs
./tooling/agov dist validate
./tooling/agov dist build
```

Record actual test pass/fail/ignored counts; RC2's 340/0/3 result is precedent,
not an expected-result substitute. Cargo metadata must show the workspace,
three internal pins, and five lock packages at `2.5.0`. Both built binaries
must self-report `2.5.0`, the exact release-MR head, and clean source.

### Archive, docs, provenance, and parity

- Build fresh `agf-2.5.0.tar.gz` and `.zip`; verify `agf/VERSION`, all 12 RC2
  roles, and the learning template report `2.5.0`; verify acknowledgements and
  its nav/cross-link remain present; verify live `MANIFEST.yaml` remains
  excluded and `MANIFEST.example.yaml` remains included.
- Run `tooling/rust/scripts/check-release-self-sufficiency.sh selftest` and
  `check <tar> <zip>`.
- Run `tooling/rust/scripts/check-live-tooling-surfaces.sh check` and `selftest`.
- Run `tooling/rust/scripts/check-release-paths.sh selftest`,
  `tooling/rust/scripts/release-assets.sh selftest`, and all release-path,
  provenance, artifact-exclusion, SPDX, license, and dependency-denial checks
  represented by the RC2 tag pipeline.
- Run `mkdocs build -f agf/docs/build/mkdocs.yml --strict` and the repository's
  documentation hook/link tests.
- Run the tag/version guard with `CI_COMMIT_TAG=v2.5.0`; run a mismatched-tag
  negative control and require rejection.
- From real published baselines `v2.4.5`, `v2.4.8`, and `v2.4.11`, overlay the
  new framework archive in fresh scratch adopters. For each, prove a divergent
  adopter `agf/agents/MANIFEST.yaml` is byte-identical, the RC2 functional
  content remains, and stable `VERSION` is visible.
- Install the packaged tooling archive and compare CLI `agov coord pending`
  with MCP `agov_coord_pending`; compare CLI `agov validate learnings` with MCP
  `agov_validate` for the same positive and malformed fixtures. Require
  equivalent accept/reject results and output shape.
- Require one successful exact-head MR pipeline. If GitLab creates duplicate
  push/MR pipelines for the SHA, preserve the canonical acceptance pipeline,
  cancel the redundant one, and disclose consumed runner time.

### Gate-self-test evidence

The release MR description must quote the controlling rule clause and show
expected and actual visible outcomes for one positive path and at least these
negative paths:

1. source or target starts anywhere other than the frozen RC2 commit — STOP;
2. a 24th path or any functional blob changes — STOP and route to RC3;
3. any immutable namespace is occupied — STOP before merge/tag;
4. either branch has access level 30, force-push, or deletion permitted — STOP;
5. no exact protected-tag rule exists immediately before tag creation — STOP;
6. source or target head changes after review — invalidate review and Gate 2;
7. either retained branch is missing or at the wrong SHA after merge — STOP,
   do not tag or publish, and escalate to Governor;
8. tag/version mismatch — guard rejects before archives are written.

## Exact merge, tag, and publication sequence

After binding release Gate 2, recheck exact heads, pipeline, discussions,
protections, and namespace absence. Merge through the GitLab API with the
reviewed SHA, squash disabled, and source-branch removal explicitly false:

```text
PUT /projects/:id/merge_requests/<release-mr-iid>/merge
sha=<approved-exact-head>
squash=false
should_remove_source_branch=false
merge_when_pipeline_succeeds=false
```

Immediately re-read the MR, `release/2.5`, `release-2.5.0`, and both protection
entries. Prove the target points to the returned merge commit, the source still
points to the approved source head, both refs remain protected, and neither was
deleted or moved unexpectedly. Then prove the merge tree differs from RC2 only
through the approved metadata paths.

Recheck all four immutable namespaces and the exact protected-tag rule. If all
remain clear, create one annotated `v2.5.0` tag on the exact maintenance-target
merge commit and push it once. Never move, delete, recreate, or retarget it.

Require the immutable tag pipeline to pass all framework/tooling archive,
checksum, provenance, OCI, generic-package, and ordinary GitLab Release jobs.
Run:

```bash
agov dist post-publication-check \
  --releases-url \
  https://gitlab.com/api/v4/projects/83321848/releases/v2.5.0 \
  --download-dir <fresh-empty-directory>
```

From a fresh clone and fresh extraction of the real published tar and zip,
repeat identity, contents, self-sufficiency, packaged binary, CLI/MCP parity,
checksum/provenance, OCI digest, and ordinary-release checks. Publication is
not complete until anonymous downloads and the four namespaces agree with the
tag pipeline's immutable commit and digests.

## Metadata-forward synchronization to `main`

Only after successful publication, fetch current `main` and freeze its exact
SHA as `forward_base`. Create a fresh branch from that SHA and merge the tagged
maintenance commit with `git merge --no-ff v2.5.0`; squash and rebase are
forbidden. Resolve so every nonmetadata path is byte-for-byte identical to
`forward_base` while the tagged commit remains an ancestor.

At this plan's audit anchor (`origin/main` merge `1af67604...`), the exact
forward-metadata allowlist is:

- `CHANGELOG.md`, `agf/CHANGELOG.md`, `agf/VERSION`, `agf/ADOPTING.md`;
- `agf/agents/MANIFEST.yaml` and the 14 current role YAMLs, including the
  suspended `independent-implementation-reviewer` and `web-ux-reviewer`;
- `agf/docs/build/hooks/check_commissioning_links.py`;
- `agf/docs/governance/manifest.md`;
- `agf/docs/site/reference/release-artifacts.md`;
- `agf/docs/site/roadmap.md`;
- `agf/templates/AGENTS.md`, `agf/templates/learning-record.yaml`;
- `tooling/rust/Cargo.toml`, `tooling/rust/Cargo.lock`.

Before resolving the sync, rerun the current-version scan on `forward_base`.
If it produces any path outside this allowlist, or a named path has acquired a
non-version semantic change that cannot be preserved, stop and amend/re-review
the plan. Do not dynamically widen the allowlist.

The forward result must:

- preserve all current-main nonmetadata blobs exactly;
- preserve current `[Unreleased]` content above the new stable section;
- insert the same `[2.5.0]` stable-promotion entry above the preserved RC2
  history without moving post-RC2 work into that release entry;
- update current version surfaces to the stable baseline while keeping the
  post-RC2 roles and work in `[Unreleased]`;
- make the exact tagged maintenance merge an ancestor of the sync head.

Open a separate MR targeting `main`. Require exact-head pipeline,
contribution-auditor review, and binding Governor Gate 2. Prove:

```bash
git merge-base --is-ancestor v2.5.0 HEAD
git diff --name-only "$forward_base"...HEAD
git diff --check "$forward_base"...HEAD
```

Any `main` target advance before forward-sync merge invalidates the proof;
recreate the merge against the new base and repeat pipeline/review/Gate 2. A
forward-sync failure never changes or invalidates the published tag; correct
forward under a new governed head.

## Staffing and ownership

- planner-architect owns this plan, Gate 1 corrections, and plan metadata;
- coder-tester performs the two approved implementation phases and evidence;
- build-engineer reviews pipeline, release-asset, package, and OCI evidence;
- contribution-auditor performs independent exact-head review of the release
  MR and forward-sync MR;
- governor-reviewer alone approves Gate 1, both Gate 2 decisions, and any
  exception or release stop-condition disposition.

No role may combine its implementation claim with the independent review used
to clear that same head.

## Acceptance criteria

1. This replacement plan is COMPLETE/CLEAR, binding Gate-1-approved, carries
   correct approval metadata, and is merged before either release ref exists.
2. Both protected branches begin at the exact peeled RC2 commit with exact
   access-level-40/no-force/no-delete evidence; the exact protected tag rule is
   present before tag creation.
3. Release MR diff equals the 23-path RC2 metadata allowlist and contains zero
   functional delta from RC2.
4. Both changelogs add a dated stable section above preserved RC2 history,
   state zero functional change, include the exact adopter-doc outcome, and
   carry the unresolved `sjqm` 2–4 deferral accurately.
5. All version, Cargo, built-binary, archive, baseline-overlay, CLI/MCP parity,
   strict docs, self-sufficiency, license, provenance, and exact-head pipeline
   evidence passes with actual results recorded.
6. The release MR receives independent COMPLETE/CLEAR review and binding Gate
   2 at an unchanged head; its API merge retains and proves both protected refs.
7. `v2.5.0` is created once on the exact maintenance merge; tag pipeline and
   anonymous post-publication checks pass; Git tag, GitLab Release, generic
   package, and OCI identities agree.
8. A separate no-squash/no-rebase forward-sync MR preserves every nonmetadata
   current-main blob and `[Unreleased]` entry, makes the tag reachable, passes
   its own exact-head review/pipeline, and receives separate binding Gate 2.
9. A validated completion record publishes final commit, tag object/peeled
   commit, pipeline/job IDs, archive hashes, package/OCI identities, post-
   publication results, retained-ref proof, and forward-sync disposition.

## Stop and recovery conditions

Stop and return to the Governor on any identity mismatch, occupied namespace,
candidate-specific defect, unexpected path, dependency or checksum change,
functional/schema/test/CI/artifact-layout correction, protection weaker than
specified, head drift, duplicate immutable artifact, failed post-publication
identity check, or need to rewrite history.

Never use reset, force-push, branch/tag deletion, tag movement, artifact
relabeling, or a custom tree-overwriting merge. Before tag creation, leave any
created refs protected and report the exact state rather than deleting them.
After tag creation, correct only forward with a new governed plan/head; the tag
and published artifacts remain immutable.

## Non-goals

- No post-RC2 role, Pages/README, commissioning, v2.6, v2.7, roadmap-content,
  CI-latency, distribution-performance, or other current-main feature ships in
  the `v2.5.0` tag.
- No schema, Rust behavior, dependency, CI, release-script, archive-layout, or
  documentation-content correction is authorized.
- No `v2.5.1-rc.1`, `v2.5.1-rc.2`, RC3, deployment, or unrelated milestone
  action is authorized.
