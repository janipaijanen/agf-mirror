---
id: v270-release-publication
status: pending-gate1
topic: v270-release-publication
advisory_consultation:
  - role: contribution-auditor
    status: waived
    reason: >-
      Not yet consulted. This plan's release-construction claims (tree-delta
      proof, path attribution, namespace preflight) are independently
      re-derived by planner-architect from first principles in this
      revision — not merely cited from `mqjk`/`d7pg` — but that
      re-derivation is disclosed self-review (same underlying model as
      planner-architect), not independent verification. Requesting a
      genuinely independent contribution-auditor review (Codex, or an
      isolated fresh-session fallback per governor decision `6srq` if no
      independent LLM is available) before this plan proceeds to binding
      Gate 1, given the release's blast radius (an immutable public tag).
  - role: infosec-sentinel
    status: waived
    reason: >-
      Same underlying fact established in `33br` (v2.6.0's rule-amendment
      Gate 1) and `kpkk` (v2.6.0's own release plan): the configured
      `infosec-sentinel` CI actor's authority is limited to its configured
      mechanical checks (secret scanning, dependency CVEs, SPDX) and cannot
      author the semantic protected-ref/namespace/merge-race review this
      plan's construction section requires. Requesting the same
      plan-specific semantic-consultation waiver disposition from the
      governor for this exact plan head. Configured mechanical checks
      remain mandatory throughout.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: publish AGF-NG v2.7.0 via frozen-scope reconstruction + maintenance-line stable promotion

## Decision requested

Authorize construction and publication of `v2.7.0-rc.1`, a bounded
verified hold, and — if that hold passes — promotion to stable `v2.7.0`,
carrying exactly the frozen scope binding decision `8hgm` names: issues
**#86, #87, #88, #90**. Requested per handoff
`20260827T123825Z-coder-tester-n6ar.md`, itself downstream of the
governor's binding topology-and-candidate-policy decision
`20260827T124803Z-governor-reviewer-rpam.md` ("yes to both": frozen-scope
reconstruction from `v2.6.1`; RC-first-then-48h-hold candidate policy).

This plan authorizes only release metadata, the two governed
constructions below (candidate, then stable promotion), validation, and
publication verification. No implementation, schema, or command-semantic
change is in scope — both frozen issues are already Level 4 Validated on
`main`.

## What is actually implemented, verified against current main

- **#86, #88**: MR !806 implemented `agov-mcp` transport protocol
  correctness. Merged at `960124e1beed7eb7ae381a20c9a176a09b492b5e` after
  exact-head Gate 2 (`20260827T080547Z-governor-reviewer-e5pk.md`).
  Closed (`3h6j`).
- **#87, #90**: MR !2138 implemented assistant-discovery/archive-loader
  setup. Merged at `acaec9472274cebcc32f0d9ade5c92864fd14ccd` after
  exact-head Gate 2 (`20260827T101500Z-governor-reviewer-g2m8.md`).
  Closed (already, per `3h6j`).
- The one deferred item — a fresh Claude Code session confirming
  `.claude/CLAUDE.md`/imported `AGENTS.md` under `/context` Memory files —
  remains partially verified (`20260827T104200Z-planner-architect-lvck.md`:
  file-level wiring confirmed; live `/context` surfacing not confirmable
  from an already-running session). This is accepted-and-monitored, not a
  release blocker: the file-level check is the only part actually
  reproducible pre-release, and the deferred part is orthogonal to what
  the release process itself changes.

**Maturity note (per `implementation-honesty-rule.md`):** #86-#90 are
Level 4 Validated as of this baseline. They reach Level 5 Deployable only
after this plan's own publication-verification steps complete against
the actually-published tag.

## Eligibility (per the frozen-scope reconstruction exception)

- [x] **Binding governor record naming the frozen scope, exact stable
      tag, and why neither the standard nor maintenance-line path
      applies for candidate construction.** Satisfied by `rpam`: frozen
      issues #86/#87/#88/#90; stable base `v2.6.1`
      (`7e7ad93121be16c3e44a7c12ccab1104efd126ff`, peeled
      `f780ac0ea37061e0ad3ee6fb1f00c6958b996fc0`); standard path excluded
      because current `main` (`efd811dd` at decision time) is 955 commits
      / 635 paths past `v2.6.1`, containing substantial unrelated work;
      maintenance-line path excluded for *candidate* construction because
      no immutable `v2.7.0` release candidate exists yet to promote.
      Independently re-verified in this revision: `git merge-base
      --is-ancestor` confirms `f780ac0e` is on `main`'s ancestry, and
      `agf/VERSION` at that commit reads `2.6.1`.
- [x] **Every allowlist commit is a Gate-2-approved exact SHA, or a merge
      commit with a proven byte-identical first-parent patch (case b).**
      Both allowlist entries are case-(b) merge commits. Independently
      recomputed in this revision (not merely cited from `mqjk`):
      - `960124e1` (MR !806, issues #86/#88): parents
        `c3a94a51.../98c491d0...`; `98c491d0` is the exact Gate-2-approved
        source head (`e5pk`). `git merge-base c3a94a51 98c491d0` =
        `08f8640e`. `git diff --binary 08f8640e 98c491d0 | sha256sum` =
        `a3e31d770bfce5d127cf980bbff0f30392a7b8d6b52dd1df7a85c7e123fd1bd6`.
        `git diff --binary 960124e1^1 960124e1 | sha256sum` = the same
        value. **Byte-identical, independently confirmed.**
      - `acaec947` (MR !2138, issues #87/#90): parents
        `a741fab7.../26cd74e0...`; `26cd74e0` is the exact Gate-2-approved
        source head (`g2m8`). `git merge-base a741fab7 26cd74e0` =
        `c9462dab`. `git diff --binary c9462dab 26cd74e0 | sha256sum` =
        `a58caedf95ec101562fa6cadaad7cca89e0ec32293b2b44cb5764ef7c014529d`.
        `git diff --binary acaec947^1 acaec947 | sha256sum` = the same
        value. **Byte-identical, independently confirmed.**
- [x] **No conditional prerequisite is claimed for either allowlist
      commit** — unlike v2.6.0's `3456c1fa` necessity check, both
      allowlist entries here are unconditional exact-head Gate 2
      approvals with no aarch64-style prerequisite. Nothing to execute or
      record under this checklist item.
- [x] **Git tag, GitLab Release, generic-package, and OCI namespaces for
      `v2.7.0-rc.1` and `v2.7.0` independently confirmed absent.**
      Point-in-time result from `d7pg` (2026-08-27T12:54:06Z): all four
      namespaces absent for both names, across all generic-package
      statuses, against the freshly-resolved `agov` OCI repository ID
      `11946773`. **This is a point-in-time result, not a reservation**
      — the second, immediately-pre-tag re-check is a mandatory step in
      "Merge, tag, and publication sequence" below, run separately for
      the RC tag and again for the stable tag.

## Frozen-out scope

Current `main` carries 955 commits / 635 changed paths beyond `v2.6.1`
that are **not** part of this release — the volume is too large to
itemize by commit (unlike v2.6.0's four named groups); scope discipline
instead comes from construction, not enumeration: the reconstructed tree
contains *only* what the two named allowlist commits' own diffs touch
(proven below), so anything else on `main` is structurally excluded by
the cherry-pick-onto-tag method, not by a maintained exclusion list.

Named examples of frozen-out content, for orientation only (not an
exhaustive gate): MR !1965's `GIT_DEPTH` fix, MR !2041's review-convergence
workflow, MR !2119's necessary-scope-expansion rule, MR !2122's Pattern 6
reconciliation, MR !2125's net-new-projection-path lifecycle (issue #115),
and this plan file itself.

## Verified construction: cherry-pick onto the stable tag

Independently reproduced in this revision, in an isolated detached
worktree at the peeled `v2.6.1` commit (`f780ac0e`), **not** taken on
`mqjk`'s word alone:

1. `git cherry-pick -m 1 960124e1beed7eb7ae381a20c9a176a09b492b5e`:
   **auto-merges cleanly, zero conflicts** (one auto-merge note on
   `tooling/rust/crates/agov-core/src/roi.rs`, no manual resolution).
2. `git cherry-pick -m 1 acaec9472274cebcc32f0d9ade5c92864fd14ccd` on top:
   **auto-merges cleanly, zero conflicts** (auto-merge notes on
   `AGENTS.md`, `agf/learnings/foundry.yaml`, `agf/templates/AGENTS.md`,
   no manual resolution).
3. Resulting tree: `caf3de542359e572f43e18be5ef4f92fca236164` — **exact
   match** to `mqjk`'s independently-computed value, now confirmed a
   third time (mqjk's own reproduction, then this plan's fresh
   reproduction).
4. Base-to-head binary patch, freshly computed:
   `git diff --binary f780ac0e HEAD | sha256sum` =
   `98ab30cdfa029725df4c987f0f735faac051a119f9d06ebb3c9d802763dd3ed7`.

Because both cherry-picks applied with **zero conflicts**, the "whole-
result content faithfulness" proof required by the merged rule is
structurally simpler than v2.6.0's case: there is no manually-resolved
region for an independent party to re-derive. The entire reconstructed
tree beyond the base is the cherry-pick tool's own unmodified mechanical
output — confirmed by the reproducibility of the tree hash across two
independent executions (`mqjk`'s and this plan's).

**Construction:** branch `reconstruct/v2.7` from `f780ac0e` (the
`v2.6.1` stable tag), cherry-pick `960124e1` then `acaec947` (in that
exact order — reversing order is untested and not authorized), then
apply the release-only change set (below) on the *source* branch. `main`
is never touched by this construction.

## Mandatory tree-delta proof (three independently checkable parts)

1. **Deterministic replay proof.** Authoritative value: binary patch hash
   `98ab30cdfa029725df4c987f0f735faac051a119f9d06ebb3c9d802763dd3ed7`
   (cross-check tree `caf3de542359e572f43e18be5ef4f92fca236164`), both
   pre-metadata-change-set, reproduced independently twice (see above).
2. **Path attribution.** `960124e1`'s own diff touches exactly 5 files:
   `agf/docs/site/docs/adopt/mcp-server.md`,
   `docs/plans/agov-mcp-transport-protocol-correctness.md`,
   `tooling/rust/crates/agov-core/src/roi.rs`,
   `tooling/rust/crates/agov-mcp/src/main.rs`,
   `tooling/rust/crates/agov-mcp/tests/stdio.rs`. `acaec947`'s own diff
   touches exactly 17 files: `AGENTS.md`, `agf/ADOPTING.md`,
   `agf/agents/MANIFEST.example.yaml`, `agf/agents/MANIFEST.yaml`,
   `agf/docs/site/docs/adopt/assistant-loader-configuration.md`,
   `agf/docs/site/docs/adopt/mcp-server.md`, `agf/learnings/foundry.yaml`,
   `agf/project-specific-examples/python-flask/CLAUDE.md`,
   `agf/templates/AGENTS.md`, `agf/templates/CLAUDE.md`,
   `.claude/CLAUDE.md`,
   `docs/reviews/rule-projection/assistant-discovery-archive-setup.yaml`,
   `examples/claude/.mcp.json`, `.gitlab-ci.yml`,
   `tooling/rust/crates/agov/src/dist.rs`,
   `tooling/rust/crates/agov-validators/src/assistant_loader.rs`,
   `tooling/rust/scripts/check-release-self-sufficiency.sh`. The one
   shared path (`agf/docs/site/docs/adopt/mcp-server.md`) collapses the
   union to exactly **21 files**. Independently confirmed: the
   reconstructed tree's `git diff --stat` against `f780ac0e` lists
   exactly these 21 paths — no unattributed file.
3. **Whole-result content faithfulness.** Not applicable in the "manually
   resolved conflict region" sense — there were none. The entire result
   is the cherry-pick tool's own mechanical output, confirmed
   reproducible byte-for-byte (tree hash `caf3de54...`) across two
   independent executions from the same base and the same two named
   commits.

All three figures are re-verified against the actual release MR's own
exact head at Gate 2, not assumed from this plan alone.

## Protected target and source

- **Target:** `reconstruct/v2.7`, created at exactly `f780ac0e`, before
  any cherry-pick lands. Protection: `push_access_levels: No one`
  (level 0), `merge_access_levels: Maintainers` (level 40),
  `allow_force_push: false`. Full protection object read and recorded
  immediately after creation, immediately before the release MR merges,
  and immediately after RC tag publication — all three identical; any
  difference invalidates the exception.
- **Source:** `release-v2.7.0-rc.1`, created at the same exact base.
  Construction baseline (level 40 push access) recorded at creation.
  Before Gate 2 evidence is presented, reprotected to the same
  `No one`/`Maintainers`/no-force-push posture as the target — this
  one-time transition recorded as the frozen baseline, alongside both
  branches' resolved head SHAs.
- **Release MR:** non-squash, non-rebase, from `release-v2.7.0-rc.1` to
  `reconstruct/v2.7`. Diff against the target is exactly the two
  commits' cumulative diff plus the release-only change set. Both
  branches retained after merge and after tagging.
- **Protected-tag rule:** matching `v2.7.0-rc.1` exactly, creation
  restricted to access level 40, created before the tag itself.

## Release-only change set (candidate)

**This plan file is out of the reconstructed tree** — it did not exist
at the `v2.6.1` cut point. Adding it to `release-v2.7.0-rc.1` would
create an unattributed 22nd path, violating the tree-delta proof. Gate 1
metadata for this plan is recorded on this MR and `main`, exactly like
every other plan file — never on the reconstruction branches.

Apply the following on `release-v2.7.0-rc.1`, after both cherry-picks:

1. Set `agf/VERSION` to `2.7.0-rc.1`.
2. Both changelogs' `## [Unreleased]` section (from the two cherry-picked
   commits — issues #86/#88/#87/#90 entries only) becomes a new
   `## [2.7.0-rc.1] - <release date>` section, inserted above the
   existing `## [2.6.1]` section (reverse-chronological, matching every
   prior release). Use the release branch's actual test/build counts.
3. Update `tooling/rust/Cargo.toml` and `Cargo.lock` package version(s)
   from `2.6.1` to `2.7.0-rc.1` (or the workspace's established
   pre-release version convention — verify against the exact convention
   `v2.5.0-rc.2`/`v2.5.0` used, since this is the first RC cycle this
   project has run since then).
4. Update current framework-version surfaces, independently confirmed in
   this revision by grepping the `v2.6.1` tree for live "2.6.1" version
   citations (excluding CHANGELOG/foundry.yaml/roadmap.md/governance-
   manifest, handled narratively in step 5): `agf/ADOPTING.md`,
   `agf/agents/MANIFEST.yaml`, all fourteen `agf/agents/*.yaml` role
   files present on the reconstructed tree (`academic-peer-reviewer`,
   `academic-visual-reviewer`, `build-engineer`, `capability-developer`,
   `coder-tester`, `contribution-auditor`, `deployment-engineer`,
   `documentation-writer`, `fact-checker`, `governor-reviewer`,
   `independent-implementation-reviewer`, `infosec-sentinel`,
   `planner-architect`, `web-ux-reviewer` — all fourteen exist on this
   tree, unlike v2.6.0's two frozen-out role files), `agf/templates/
   AGENTS.md`, `agf/templates/learning-record.yaml`. Note:
   `agf/agents/MANIFEST.example.yaml` and `AGENTS.md` are already touched
   by `acaec947`'s own content — bump their version citations in place,
   do not duplicate the file edit.
5. Update `agf/docs/site/roadmap.md` and `agf/docs/governance/
   manifest.md` to identify `v2.7.0-rc.1` as the candidate under
   verification — narrative edits, not mechanical, per the exact pattern
   `v2.6.1`'s own roadmap entry used for its own predecessor candidate.
   Preserve all historical release evidence unchanged.

No product implementation, command semantic, schema, validator semantic,
new dependency, or new CI behavior change is in scope beyond what
#86-#90 already merged.

## Validation before Gate 2 (candidate)

At minimum, exact-head evidence for: full version-surface audit (every
current release-truth surface reads `2.7.0-rc.1`); Cargo metadata
agreement; `cargo test --manifest-path tooling/rust/Cargo.toml`
(workspace); strict Clippy with warnings denied; `agov validate`
(including adopter-docs); `agov dist validate` and clean archive builds;
built `agov`/`agov-mcp` self-reporting `2.7.0-rc.1`; tag/version guard
evidence with `CI_COMMIT_TAG=v2.7.0-rc.1`; strict `mkdocs build`; a
GitLab API check confirming the release MR's `closes_issues` response is
empty (issues #86-#90 are already closed, independently, per `3h6j` —
this release MR must not re-trigger closure); release-path, provenance,
artifact-exclusion, SPDX/license, and `git diff --check` checks; a green
exact-head MR pipeline; the protection-object and tree-delta evidence
above; and an immediately-pre-merge re-read of both branch heads via the
platform API, matching the exact reviewed SHAs.

## Merge, tag, and publication sequence (candidate)

1. Merge only the exact Gate-2-approved release MR head into
   `reconstruct/v2.7`. Retain `release-v2.7.0-rc.1`.
2. **Second, independent, immediately-pre-tag namespace check** for
   `v2.7.0-rc.1` across Git tag, GitLab Release, generic package (all
   statuses, paginated), and OCI tag (repository ID resolved fresh). Any
   non-absence stops the release. Re-read the protected-tag-rule object
   at the same checkpoint.
3. Create one annotated `v2.7.0-rc.1` tag on the exact
   `reconstruct/v2.7` merge commit; push immediately. Never move,
   delete, or recreate it.
4. Require the tag pipeline to pass: framework and Rust tooling
   archives, provenance/checksums, OCI publication, GitLab Release
   creation.
5. Run `agov dist post-publication-check` against the anonymous GitLab
   Release API and a fresh download; verify all downloaded bytes,
   provenance, and executable self-reporting `2.7.0-rc.1`.
6. Verify the GitLab Release, generic package `agf/v2.7.0-rc.1`, and OCI
   tag `agov:2.7.0-rc.1` correspond to the immutable tag pipeline.
   Re-read the protected-tag-rule object once more post-publication.
7. From a fresh clone at the exact immutable `v2.7.0-rc.1` tag: confirm
   the public documentation site builds strictly; confirm `#86`'s and
   `#88`'s `agov-mcp` transport behavior and `#87`'s/`#90`'s
   assistant-discovery/archive-loader behavior all match their
   documented contracts; and — the one item this plan can uniquely
   verify from a genuinely fresh session — a **new** Claude Code session
   opened against this exact tagged checkout runs `/context` and
   confirms `.claude/CLAUDE.md` and imported `AGENTS.md` content appear
   under Memory files, finally closing `7bkm`'s deferred live-surface
   check with a real fresh-session result (not this plan-authoring
   session, which cannot demonstrate it per `lvck`).

## 48-hour verified hold

Begin only after all of the above checks in step 4-7 pass. During the
hold: no further mutation of `reconstruct/v2.7`, `release-v2.7.0-rc.1`,
or the `v2.7.0-rc.1` tag. If a relevant defect surfaces, govern the
correction on a fresh `release-v2.7.0-rc.2` branch from the same
`reconstruct/v2.7` target, publish immutable `v2.7.0-rc.2`, and restart
the 48-hour hold — never move, delete, or replace the `-rc.1` tag or
artifacts. Record the hold's start timestamp and end timestamp in a
coordination completion record; the hold is calendar time, not a
countdown the governor must actively watch.

## Stable promotion (maintenance-line exception, after the hold passes)

Following the exact pattern `v2.5.0`'s promotion from `v2.5.0-rc.2` used
(`docs/plans/20260809T194404Z-v250-final-release-publication.md`):

1. **Frozen candidate identity.** All stable-promotion work begins from
   the accepted RC's immutable anchors (`v2.7.0-rc.1` or whichever `-rcN`
   passed its hold), never from `reconstruct/v2.7`'s possibly-advanced
   tip. Re-derive tag object, peeled commit, and tree from the canonical
   remote before the first mutation; stop on any mismatch.
2. **Fail-closed namespace and protection preflight for `v2.7.0`
   specifically** (not `-rc.1`): repeat all four namespace absence
   checks, plus absence of `release/2.7` and `release-2.7.0` branches
   and their protection entries. Create `release/2.7` at the exact
   accepted-RC commit, protect at level 40 push/merge, no force-push, no
   deletion; re-read via the API. Create `release-2.7.0` at the same
   commit with the same protection. Create the exact `v2.7.0`
   protected-tag rule, restricted to level 40.
3. **Exact metadata boundary.** The stable release commit may change
   only the same governed surfaces the candidate step already touched:
   `agf/VERSION` (from `2.7.0-rc.1` to `2.7.0`), both changelogs (the RC
   section's heading becomes `## [2.7.0] - <release date>`, with any
   post-RC hold findings folded in if the hold produced none — in which
   case the section is otherwise unchanged), and the same version-surface
   list from step 4 above, each updated to plain `2.7.0`. **Zero
   functional, schema, documentation-content, migration, test,
   dependency, CI, or artifact-layout change from the accepted RC** —
   this is the same "metadata-only promotion" standard `v2.5.0` itself
   used.
4. **Release MR:** non-squash, non-rebase, `release-2.7.0` →
   `release/2.7`. Diff against target is exactly the metadata boundary
   above. Exact-head Gate 2 required; immediately-pre-merge re-read of
   both branch heads.
5. **Tag and publish `v2.7.0`** on the exact `release/2.7` merge commit,
   after a fresh, immediately-pre-tag namespace and protected-tag-rule
   re-check (same four namespaces, same fail-closed discipline as the
   candidate's own tag step). Require the tag pipeline, run
   post-publication verification, and re-confirm the GitLab Release,
   generic package `agf/v2.7.0`, and OCI tag `agov:2.7.0`.
6. **Metadata-forward synchronization to `main`**, per the merged rule's
   standard procedure (content-preserving; `main`'s own code, tests, and
   non-version-metadata content untouched): split `main`'s own
   `## [Unreleased]` changelog sections so the matching entries move into
   a `## [2.7.0] - <release date>` section; update the same version-
   surface list to `2.7.0` as a last-released-version marker (not a
   claim about `main`'s own next release number, since `main` has
   substantial `Unreleased` content beyond this frozen scope); prove the
   sync MR's diff is metadata-only; exact-SHA merge with pre/post
   sync-target drift checks, per `apkm`'s narrow accepted detect-and-
   remediate exception for this specific step.
7. Publish and merge a validated completion coordination record marking
   `v2.7.0` as the new latest published stable release, before any
   further release treats it as baseline.

## Staffing and ownership

Candidate construction (branches, cherry-picks, metadata edits,
validation evidence): coder-tester, per this project's established
division of Rust/release-mechanics labor this session. Gate 1 and Gate 2
decisions: governor-reviewer (human), consistent with every decision
cited above. Post-hold stable promotion: same split, re-claimed fresh
after the hold passes — do not assume the candidate's claim silently
extends to promotion.

## Acceptance criteria

1. The candidate release MR is release-only, targets `reconstruct/v2.7`,
   and receives exact-head Gate 2 approval.
2. All version surfaces and built binaries agree on `2.7.0-rc.1` (then
   `2.7.0` after promotion).
3. The immutable RC tag points to the exact `reconstruct/v2.7` merge
   commit, with post-merge parent/tree-OID proof matching the reviewed
   target/source/metadata-frozen source.
4. The RC tag pipeline and anonymous downloaded-asset validation pass.
5. Published `agov-mcp` (#86/#88) and assistant-discovery/archive-loader
   (#87/#90) behavior on the tagged RC matches documented contracts,
   including a genuinely fresh Claude Code session's `/context` check.
6. The 48-hour hold completes with no unresolved defect.
7. Stable `v2.7.0` promotion is a metadata-only diff from the accepted
   RC, published via the maintenance-line exception, with its own
   independent namespace/protection/tag evidence.
8. The frozen-out volume (955+ commits since `v2.6.1`) does not appear
   in either the RC or stable tagged artifact's public delta from
   `v2.6.1`, proven by the tree-delta/path-attribution evidence above,
   not by enumeration.
9. Issues #86-#90 remain closed throughout (already closed pre-release,
   per `3h6j`) — no double-close, no premature reopening.

## Stop conditions

Stop and return to the governor if: release preparation requires any
product, schema, or CI-behavior change beyond what #86-#90 already
merged; a dependency/checksum change; weakening any existing check;
changing an existing tag; including any frozen-out content; deleting a
source or target branch; any stable-base, protected-target, or
protected-source drift at any checkpoint; a tree-delta or path-
attribution proof failure; a pre-tag or post-publication namespace/
protected-tag-rule re-read finding any non-absence or mismatch; the
release-merge or forward-sync post-merge parent/tree checks finding a
mismatch not resolved by their own documented remediation; or the
48-hour hold surfacing a defect that is not cleanly correctable via a
fresh `-rc.2` without touching the accepted `-rc.1` artifacts.
