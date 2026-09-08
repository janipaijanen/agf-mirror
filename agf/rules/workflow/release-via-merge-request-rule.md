---
provenance: verified-agf-v2
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Releases: Via Merge Request Only
#
# Framework principle: Immutability Constraint, Quality Gates
# See: framework.md — Principle 3 (The Immutability Constraint)
# Companion rule: immutable-tags-rule.md

**Scope:** All version releases and git tag creation.

## Rule

A version tag MUST only be created on the main branch **after** a merge request has been reviewed and merged. Direct commits to main followed by tagging are forbidden for release versions.

## Release workflow

1. Ensure all feature commits are merged to main first.
2. Create a `release-X.Y.Z` branch **from main** — at this point, main
   and the release branch are identical.
3. On the release branch (not main), commit the release-only changes:
   - `VERSION` bump
   - `CHANGELOG.md` entry for the new version
   - Any other release metadata
4. Open a merge request from `release-X.Y.Z` → main. The MR diff
   contains **only** the version bump and changelog — making review
   trivial and the MR non-empty.
5. Review the MR: verify version consistency, CI passes, distribution
   archives build correctly.
6. Merge the MR.
7. Tag the merge commit on main with the version (e.g. `v0.15.0`).
8. Push the tag immediately — CI creates the release with archives.

**Steps 7–8 require no additional confirmation.** The merged MR is the
authorization. An agent or contributor MUST tag and push in a single action
immediately after merge — not stop to ask permission.

**Critical:** Never commit the version bump to main before creating the
release branch. If you do, the MR will have no diff and cannot be
reviewed.

## Why

Tags are immutable (see immutable-tags-rule.md). A broken tag cannot be fixed — only superseded by a new version. The merge request is the quality gate that prevents tagging broken commits. Without it:

- CI config errors produce broken releases (empty version strings, missing assets).
- Version references across files may be inconsistent.
- Distribution archives may contain wrong content.

The cost of a merge request is minutes. The cost of a broken immutable tag is a wasted version number and a confusing release history.

## Forbidden operations

- Tagging main directly after pushing commits without an MR review.
- Tagging a non-main branch for release.
- Creating tags from CI pipelines without a preceding MR merge.

## What the MR MUST verify

- [ ] `VERSION` file matches the intended release version.
- [ ] `CHANGELOG.md` contains an entry for this version (not backfilled later).
- [ ] For adopter-relevant changes, the release changelog entry contains
      `Adopter docs: updated`, `Adopter docs: not applicable`, or
      `Adopter docs: waived <coordination-id>`.
- [ ] All version references are consistent (agent definitions, documentation).
- [ ] `agov validate adopter-docs` passes.
- [ ] `agov dist validate` and `agov dist build` pass (if applicable).
- [ ] CI pipeline configuration is correct for the release.
- [ ] SPDX headers and license are correct across all files.
- [ ] Standard-path namespace-absence evidence is attached for the intended
      tag: all four namespaces report `ABSENT`, with project identity, version,
      source/target commits, checkpoint time, and the canonical MR pipeline ID.

## Pre-tag namespace-absence obligation (standard path)

Before a standard-path release is merged or tagged, the Git tag, GitLab
Release, generic-package version, and exact publication OCI tag MUST be
unoccupied. A pre-existing object or ambiguous result MUST stop the release.
This obligation does not replace the stricter exception checklists below.

Use one canonical check, `agov release namespace-check`, from the clean,
committed release checkout. Derive project identity from the configured release
target (same-project CI context in CI) and verify its canonical path; derive
the package name from the validated release manifest; derive the OCI reference
from `agov-oci-image.sh image-ref` and require `verify-ref` to agree. Never
substitute a cached registry repository ID for the project ID or hand-type the
OCI tag. Resolve the registry repository ID fresh after full discovery.

The check MUST establish positive authenticated access for each namespace
before interpreting a candidate-object 404 as absence. Enumerate collections
completely and fail closed on missing/contradictory pagination, inaccessible
collections, malformed responses, timeouts, HTTP errors, identity mismatch,
or multiple exact OCI repository matches. Generic packages require exact
name/version/type matching after complete pagination of every status
(`default`, `hidden`, `processing`, `error`, `pending_destruction`,
`deprecated`); the API's fuzzy name filter is not an exact-match proof.
Empty OCI discovery proves absence only after successful authenticated listing.

**Named mechanical blocking authority:** `release:namespace-check` MUST run
on the release-metadata MR pipeline when `agf/VERSION` changes and MAY block
merge for any failed identity/access/absence check described above. It MUST
not allow failure. It binds the detached MR source SHA, live target SHA,
candidate version, and canonical pipeline ID; it does not claim knowledge of
a future merge commit. The job consumes the existing `rust:tooling:preview`
artifact, uses the default non-DinD runner, and performs no second Rust build.
CI uses only the same-project ephemeral `CI_JOB_TOKEN` in a `JOB-TOKEN` header;
no PAT/project-token addition, query-string credential, tracing, or credential
artifact is permitted. Tool success is evidence, never a governor gate decision.

**Both operator runs are mandatory:** run the same check at reversible
preflight (`--phase preflight`) and again immediately before the tag push
(`--phase pre-push`). The final run MUST bind the resulting release-target
merge commit and stop publication on any failure. A prior CI result cannot
replace this fresh check. Operator credentials must already be authorized to
read all four namespaces; do not print them or place them in command arguments.
The check does not reserve namespaces or eliminate the check-to-push race;
publication still follows protected-tag controls and the authorized release
sequence. Any retained post-tag verification checks consistency/non-overwrite,
never Git-tag absence.

Per `gate-self-test-rule`, changes to this gate require negative evidence for
an occupied object in each namespace, a later-page/non-default-status package,
wrong project/path, inaccessible 404, malformed/incomplete responses, and the
operator pre-push stop. Also prove authorized absence and a fuzzy-only package
match behave correctly. Keep the reproducible transcript in the implementation
MR's `## Negative test` section; no real release objects are created for tests.

## Maintenance-line exception (optional, narrowly eligible)

The standard workflow above is the default and is not weakened by this
section. It exists for exactly one case the standard workflow cannot
represent safely: promoting a previously published, immutable
release-candidate commit to a stable version **after** the default
branch has accumulated unrelated later work, without pulling that
later work into the promoted release.

### Eligibility (all conditions required, evidenced before any branch is created)

- [ ] A binding governor record names the exact candidate tag object,
      peeled commit, tree, intended stable version, and why the
      standard current-main path is excluded.
- [ ] The candidate commit already has its own Gate 1, Gate 2,
      tag-pipeline, and post-publication evidence. A branch-only or
      unreviewed commit is ineligible.
- [ ] The new release changes only version, changelog, and other
      enumerated release-truth metadata. Any functional, schema,
      migration, test, dependency, CI, or artifact-layout change makes
      the candidate ineligible — use the standard workflow (a new RC
      or current-main release) instead.
- [ ] A separate Gate 1-approved release-publication plan freezes the
      exact base, branch names, metadata allowlist, validation matrix,
      immutable-namespace checks, and forward-sync procedure.
- [ ] The Git tag, GitLab Release, generic-package, and OCI namespaces
      for the new version are independently confirmed absent through
      their authoritative APIs.

If any condition becomes false, stop. Do not fall back to a temporary
rewind of the default branch, a custom tree-overwriting merge, or
artifact relabeling.

### Protected target and source

1. Create a maintenance target named `release/X.Y` at the exact
   eligible candidate commit. Protect it — reject force-push and
   deletion, restrict both push and merge to GitLab access level 40
   (Maintainer) — **before** any release content is created on it.
   Re-read the exact protection rule from the platform API as
   evidence; do not assume a wildcard pattern matched.
2. Create `release-X.Y.Z` at exactly the same candidate commit, with
   equivalent level-40 protection. Confirm both refs resolve to the
   frozen SHA before the first release-metadata edit.
3. Open a non-squash, non-rebase merge request from `release-X.Y.Z` to
   `release/X.Y`. Any change to source, target, merge base, or
   candidate head invalidates prior review.
4. The MR diff is restricted to the release plan's exact metadata
   allowlist.
5. A protected-tag rule matching the exact intended stable tag (or a
   governed `vX.Y.Z*` wildcard convention) must exist, with tag
   creation restricted to access level 40, before the tag is created.
6. Exact-head Gate 2 remains binding governor authority. Merge only
   the named SHA with the reviewed pipeline. **Retention of both
   maintenance refs is not satisfied by decision text alone** — the
   merge invocation's exact command (including its branch-retention
   option) and an immediate post-merge API read proving both refs
   still exist at the intended SHAs are required Gate 2 evidence.

### Tag and publication

After the maintenance-target merge, prove its tree differs from the
frozen candidate only by the approved metadata fields. Recheck all
four immutable namespaces and both ref identities. Then create one
annotated tag on the exact maintenance-target merge commit and push it
once — never move, delete, or recreate it (see immutable-tags-rule.md).
The tag pipeline binds every archive, checksum, provenance record,
package, and OCI digest to that tag commit; anonymous post-publication
checks operate on the downloaded bytes.

### Metadata-forward synchronization

Only after successful publication: freeze the latest default-branch
base, merge the tagged maintenance commit into a fresh sync branch
with `--no-ff` (squash and rebase are forbidden — they erase the
ancestry that proves the tagged commit is reachable), resolving so
every nonmetadata path stays byte-for-byte identical to the frozen
base. Open a separate MR to the default branch; prove the net diff
from the frozen base contains only the forward-metadata allowlist. Any
target advance before merge invalidates that proof and requires a
refreshed merge, pipeline, and review. If this sync fails, the
published release remains valid — correct the sync forward without
touching the tag or maintenance line.

## Frozen-scope reconstruction exception (optional, narrowly eligible)

Canonical owner: `agf/rules/workflow/release-via-merge-request-rule.md`.
The standard workflow and the maintenance-line exception above are
unweakened by this section. It exists for exactly one case neither
covers: a governor-frozen release scope (an enumerated, named set of
already Gate 2-approved commits) that must ship as its own immutable
artifact while `main` has since accumulated other, unrelated,
individually-approved work that must **not** enter that artifact, and
no immutable release-candidate tag already exists to promote.

### Eligibility (all conditions required, evidenced before any branch is created)

- [ ] A binding governor record names the frozen release scope by
      tracker identifier (issues/MRs), the exact immutable stable tag
      to build from, and states why neither the standard workflow nor
      the maintenance-line exception applies.
- [ ] Every commit proposed for the reconstruction allowlist either
      (a) *is* the exact SHA with binding Gate 2 approval, or (b) is a
      merge commit whose first-parent binary patch is proven
      byte-identical (SHA-256 of `git diff --binary <merge-base>
      <approved-head>` equals SHA-256 of `git diff --binary
      <merge-commit^1> <merge-commit>`) to the exact Gate-2-approved
      head's own patch against its own reviewed merge base. Case (b)
      exists because Gate 2 approvals are issued against a feature
      branch's pre-merge head, while the mainline history only offers
      the merge commit — the byte-identity proof is what makes citing
      the merge commit truthful rather than approximate. Evidenced by
      reference and by the recomputed hash, not re-litigated here.
- [ ] A separate Gate 1-approved release-publication plan enumerates:
      the exact stable base tag/commit, the exact ordered list of
      commit SHAs to cherry-pick (the "reconstruction allowlist"), the
      exact release-only metadata change set on top, and the exact
      named content that must be **absent** from the resulting tree
      (the "frozen-out set").
- [ ] Any commit on the allowlist admitted under a *conditional*
      prerequisite-necessity exception (per the governing decision)
      has that condition's check specified and executed, with its
      result recorded, before the commit is treated as consumed. A
      conditional exception that is never checked is not eligible.
- [ ] The Git tag, GitLab Release, generic-package, and OCI namespaces
      for the new version are independently confirmed absent.

### Protected target and source

1. Create a reconstruction target branch (e.g. `reconstruct/vX.Y`) at
   the exact stable base tag/commit named by the eligible governor
   record, before any cherry-pick lands, protected as: `push_access_
   levels: No one` (level 0 — matching how `main` itself is actually
   configured on this platform, not the weaker "level 40 push and
   merge" pattern), `merge_access_levels: Maintainers` (level 40),
   `allow_force_push: false`. **State the platform guarantee
   accurately, not optimistically:** branch protection prevents
   Git-level force-push and deletion, but does **not** make the ref
   undeletable to a Maintainer/Owner acting through the platform UI or
   API, who can unprotect then delete. This is a residual risk the
   protection setting does not close; it is closed instead by treating
   unexpected unprotection, deletion, or protection-setting drift as a
   fail-closed integrity incident (see below), and by reading and
   recording the full protection object from the platform API at three
   points: immediately after creation, immediately before the release
   MR merges, and immediately after tag publication. **The target's
   baseline is single and constant**: its protection object recorded at
   all three points must be identical. Any difference at any point
   invalidates the exception — the target never has an authorized
   reason to change.
2. Create a reconstruction source branch (e.g. `release-vX.Y.Z`) at the
   same exact base commit. **Construction and freeze are separate
   protection states for the *source* branch specifically — the target
   has no such transition (step 1's single constant baseline is
   unaffected).** During construction (cherry-picks and the metadata
   change set being assembled), the source branch may carry the minimum
   push access needed to do that work (e.g. level 40); record this as
   the source's **construction baseline** at creation. Before the
   reconstruction release MR's exact-head Gate 2 evidence is presented,
   the source branch **must** be reprotected to `push_access_levels: No
   one`, `merge_access_levels: Maintainers`, `allow_force_push: false`
   — the same posture as the target — and this single, one-time
   transition is recorded as the source's **frozen baseline**,
   immediately after reprotection, alongside both branches' resolved
   head SHAs, as Gate 2 evidence. From that point forward the source's
   fail-closed comparison is against its **frozen baseline**, not its
   construction baseline — the recorded transition is the one
   authorized difference; any other change, before or after it, is not.
   A level-40 "push and merge" setting held all the way to Gate 2 grants
   direct-push authority indistinguishable from a reviewed merge, which
   defeats the point of the reviewable MR in step 3 below; this
   two-baseline scheme is what actually closes that gap without
   contradicting the drift check. Cherry-pick the reconstruction
   allowlist onto the *source* branch, as **deterministic ordered
   replay**: the governing plan states the exact ordered list of `git
   cherry-pick` invocations (commit SHA, parent selection for merge
   commits, no squash), and that exact sequence — not an unordered
   "apply these commits somehow" — is what Gate 2 evidence reproduces.
   Any cherry-pick conflict is resolved file-by-file, and the governing
   plan must record, for each conflicted file, the **exact resolved
   content's hash** (e.g. SHA-256) as a content-bound expected value —
   not a prose description of the resolution policy. A resolution that
   discards either side's content is ineligible and must be redone.
   Then apply the release-only metadata change set (version, changelog,
   manifest, and other enumerated release-truth surfaces per the
   governing plan) as the final commit(s).
3. Open a non-squash, non-rebase merge request from the reconstruction
   source branch to the reconstruction target branch. This MR's diff
   against the target (== the stable base) is exactly the allowlist's
   cumulative diff plus the release-only metadata change set — this
   *is* the reviewable release-truth diff Gate 2 evaluates. Retain both
   branches after merge and after tagging.
4. A protected-tag rule matching the exact intended release tag must
   exist, tag creation restricted to access level 40, before the tag is
   created.
5. **Mandatory tree-delta proof, evidenced in the release MR before
   Gate 2.** "The diff looks like the union of the inputs" is not a
   defined, reproducible Git operation and is explicitly **not**
   sufficient — patch union is non-compositional the moment two
   allowlisted commits touch the same insertion point. The proof
   instead has three required, independently checkable parts —
   **reproducibility, path attribution, and content faithfulness are
   three distinct properties; none of the three substitutes for
   another**:
   - **Deterministic replay proof:** re-running the exact ordered
     cherry-pick sequence and recorded conflict-resolution hashes from
     step 2, from the exact stable base, must produce a tree whose Git
     tree hash (or a binary base-to-head patch hash, either is
     acceptable but the governing plan must state which) equals the
     value the governing plan recorded at authoring time. A mismatch
     is a fail-closed condition (see below), not a discretionary call.
     This proves the replay is *reproducible*, not that the recorded
     result is *faithful* — see the next check.
   - **Path attribution (necessary, not sufficient):** compute `git
     diff --stat` of the resulting tree against the stable base, and
     independently compute `git diff --stat` (or full diff) of each
     allowlisted commit against its own immediate parent. Every file
     path in the resulting tree's diff-from-base must appear in at
     least one allowlisted commit's own diff. Any file path present in
     the resulting diff that is not attributable to a named allowlisted
     commit fails this check — but a file passing this check has only
     been shown to be an *expected file*, not *expected content*: a
     manually resolved conflict can stage arbitrary text inside an
     otherwise-expected file without tripping this check alone.
   - **Whole-result content faithfulness, independently re-derived —
     not scoped to conflict-marker regions:** a paused cherry-pick lets
     an operator stage arbitrary edits anywhere in the index — a
     cleanly-applied file, or a non-conflicting region of a conflicted
     file — and both would pass path attribution *and* the
     deterministic-replay hash, because that hash only proves the
     *recorded* result is reproducible, never that the recording itself
     was innocent of manual staging. Marker-scoped review of only the
     conflicted regions cannot close this gap, because the risk is
     *not* confined to conflicted regions. The proof instead requires
     an **independent party who never reads or trusts the author's
     already-resolved tree at all**: starting only from (a) the exact
     stable base, (b) the named allowlisted commits' own hashes, and
     (c) a written, mechanical conflict-resolution algorithm recorded
     in the governing plan (not a copy of the author's resolved text —
     an algorithm specific enough that a second independent party could
     follow it without consulting the first), the independent party
     re-runs the entire cherry-pick sequence from scratch in a fresh
     worktree under one binding constraint on the procedure itself:
     **the only files touched are the ones the cherry-pick tool itself
     flags as conflicted, and within each such file the only edits made
     are strictly inside the tool's own conflict-marker-delimited
     regions, replaced per the documented algorithm — every other byte
     in every file, conflicted or not, is committed exactly as the
     cherry-pick's own mechanical application (including its own
     auto-merge of non-conflicting hunks) produced it, with no
     supplementary `git add` of anything beyond what conflict
     resolution requires.** This constraint removes the opportunity to
     stage unrelated edits at all, rather than trying to detect them
     after the fact. The independent party then compares the resulting
     tree/patch hash to the recorded value. **Because this reproduction
     never touches the author's staged index or resolved files at any
     point, a match is proof that the entire result — cleanly-applied
     files and conflict-resolved files alike, marker regions and
     non-conflicting regions alike — is fully and solely derived from
     the base, the named commits' own diffs, and the documented
     algorithm, with no channel for unaccounted staged content anywhere
     in the tree.** A mismatch is a fail-closed condition exactly like
     the deterministic-replay proof above (the two checks share
     evidence but are not one check: the deterministic-replay proof
     establishes the *recorded* value is reproducible from the
     *recorded* procedure; this proof establishes that a party with no
     access to the author's index reaches the *same* value from only
     base + commits + algorithm, which the recorded-procedure check
     alone cannot show).
   All three are exact-head Gate 2 evidence, not advisory.
6. Exact-head Gate 2 remains binding governor authority, applying the
   same standard the maintenance-line exception applies: the named SHA
   with the reviewed pipeline, nothing else. **Platform credentials
   execute a governance decision; they do not originate one.** Holding
   Owner or Maintainer access on the platform proves who could act, not
   which AGF-governed role or human decided the release-truth diff was
   correct — the two are different claims, and only the binding
   governor coordination record makes the second one. Before merge,
   that record must name: the exact source-branch head SHA, the exact
   target-branch head SHA, the exact reviewed source HEAD's Git tree
   OID — recorded only after all release-only metadata edits are
   frozen, and distinct from the pre-metadata deterministic-replay
   patch/tree hashes recorded earlier (a Git tree OID and a SHA-256
   patch hash are different object types and are never asserted equal
   to each other; the deterministic-replay hashes are computed before
   the metadata change set while this tree OID is computed after it) —
   and the canonical project identifier.

   **Immediately before merge, re-read both the target and source
   branch heads via the platform API and require each to equal the
   exact SHAs the binding governor record names** — a mismatch on
   either stops here, before any merge call, and returns to the
   governor for a fresh protected target and complete proof (the
   target's dedicated, `push_access_levels: No one` protection reduces
   accidental drift but does not supply target compare-and-swap —
   nothing mechanically prevents a separately authorized merge from
   advancing this same target between review and this merge, however
   unlikely in practice). The merge itself must be performed via the
   platform's merge API with the `sha` parameter set to the exact
   reviewed source head (never a UI click against "whatever the branch
   currently points to"), `squash=false`, and source-branch retention
   requested explicitly.

   **Immediately after merge, verify — not merely that both branches
   exist at *some* SHA, but that the resulting merge commit's exact
   structure matches the reviewed context:** its first parent equals
   the reviewed target SHA (proving the merge landed on the reviewed
   target, not a drifted one), its second parent equals the reviewed
   source SHA (proving the exact reviewed content merged, not a
   substituted branch state), and its resulting tree OID equals that
   exact reviewed source HEAD's Git tree OID named in the governor
   record above. **This is fail-closed at the publication boundary:**
   no tag, release, package, OCI artifact, or other namespace may be
   created from a mismatched merge commit; mark the target proof-
   invalid and abandoned, and return to the governor for a fresh
   protected target rather than proceeding under assumed authority.
   **This is narrower than literal prevention of target drift: at
   check time, a proof-invalid merge has already landed on the
   protected reconstruction target** — dedicated-target containment
   makes the impact far lower than a proof-invalid merge to `main`,
   but it remains post-landing detection relative to that target, not
   literal prevention. **Resolved by binding governor decision
   `20260814T074427Z-governor-reviewer-33br.md`**, separate from and
   narrower in scope than the metadata-forward-synchronization
   exception below: fail-closed-before-publication with isolated-target
   detect-and-remediate satisfies this section's fail-closed-on-
   target-drift requirement, for this release-construction merge only,
   conditioned on every boundary this section already states holding
   at execution time, plus a platform-reassessment condition — any
   future application of this exception must re-evaluate whether
   GitLab then supplies a stronger target-bound or serialized merge
   mechanism before reusing this residual. All of the above is
   required Gate 2 evidence, matching the maintenance-line exception's
   own retention-proof requirement.

### Tag and publication

Ordinary exact-head Gate 2, tag-pipeline, immutable-namespace, and
anonymous downloaded-artifact verification — unweakened from the
standard workflow's own requirements (`immutable-tags-rule.md`; every
archive, checksum, provenance record, package, and OCI digest binds to
the tag commit exactly as it does for any other release).

**The eligibility checklist's namespace-absence check is a
point-in-time result, not a reservation.** Time elapses between that
early check and the actual tag push — enough for another actor or
process to occupy the same names. A second, independent, authenticated
namespace check (Git tag, GitLab Release, generic package across every
relevant status — default, hidden, processing, error,
pending_destruction, deprecated — paginated fully, and exact OCI
repository tag with the repository ID resolved fresh, not cached) is
required **immediately before the tag is pushed**, bound to the exact
merge commit and tree the release MR produced. Any non-absence result
or API ambiguity at this second check stops the release; it is not
treated as equivalent to the earlier result. After publication, re-read
each namespace and bind its resulting immutable identity/digest to the
tag pipeline as evidence.

**Protected-tag-rule object, not just namespace absence.** The
eligibility checklist's protected-tag-rule check (naming the exact
intended tag) is also a point-in-time result. Re-read the exact
protected-tag-rule object via the platform API immediately before the
tag push (same checkpoint as the namespace recheck above) and record
it as evidence; a rule that no longer matches the exact intended tag,
or that has been removed or widened, stops the release under the
fail-closed section below. Re-read it once more after publication and
bind the confirmed rule state to the tag-pipeline evidence, alongside
the namespace identities.

### Metadata-forward synchronization (content-preserving, main untouched)

`main` already contains the frozen release's content (it merged there
independently, earlier) plus the frozen-out set. This section
reconciles `main`'s own version and changelog record with the fact that
a subset of its `Unreleased` content has now shipped under an earlier
tag — it does **not** pull anything out of `main`'s code, tests, or any
non-version-metadata file, and it does not revert or rewrite any commit
already on `main`.

**This connection is evidence-only, not Git-ancestry-preserving.** A
real `--no-ff` merge of the tagged release commit into `main` produces
genuine content conflicts (not merely positional ones) wherever `main`
has independently continued editing the same prose the reconstruction
allowlist also touches — `main`'s own text is typically the more
current, correct version at that point, and resolving such a conflict
requires editorial judgment a mechanical merge cannot make safely. The
forward-sync MR is therefore evidence connecting the release to `main`
for a human/reviewer to follow, not a claim that the tag is a Git
ancestor of `main`. To keep the weaker relationship reconstructable,
the forward-sync MR description must explicitly name: the immutable
tag, its exact commit and tree SHAs, the release MR, and both
reconstruction branch names.

1. After successful tag publication, open a separate MR to `main` from
   a fresh branch created exactly at `main`'s then-current tip. Record
   that tip SHA as the **sync-target head** in the forward-sync MR
   description — this is the binding reference point for the target
   side of every check below. Do **not** record a source head yet: the
   branch has no edits at this point, so there is nothing to bind as
   the reviewed source.
2. Split `main`'s own `CHANGELOG.md` and `agf/CHANGELOG.md`
   `## [Unreleased]` sections so that exactly the entries corresponding
   to the frozen release's allowlist (matched by issue/commit
   reference, not by guesswork) move into a new
   `## [<released version>] - <release date>` section, content-identical
   to what the tag itself published, positioned in correct
   chronological order among `main`'s existing version headings.
   Everything else remains in `## [Unreleased]`, unchanged.
3. **Every governed current-version metadata surface updates to the
   released version — the same surface list the governing release
   plan's own release-only change set names, no narrower.** This
   exception does not maintain a separate, independently-enumerated
   surface list that can drift from the release plan's own; it reuses
   that plan's list verbatim, restricted here to version-field edits
   only (no code, test, or content changes). `agf/VERSION`'s new value
   is a **last-released-version marker**: it does not imply `main`'s
   own next tag will be that version, since `main`'s own next release
   may be a later number encompassing the remaining `Unreleased`
   content. State this explicitly in the forward-sync MR description so
   a future reader does not mistake the marker for a scope commitment.
   A split version state across governed surfaces (some updated, some
   not) is never acceptable as a resting state — if the full surface
   list cannot be updated together, stop and return to the governor
   rather than normalizing partial sync.
4. Prove the forward-sync MR's diff touches only the changelog files
   from step 2 and the version fields (not content) of the surfaces
   from step 3 — no code, test, schema, or other file changes. This is
   a mechanical, Gate-2-checkable condition, not advisory. Once this
   proof and its pipeline are produced, record the branch's resulting
   exact head as the **sync-source head** in the forward-sync MR
   description — this is the reviewed content presented for Gate 2, and
   the binding reference point for the source side of every check
   below.
5. **Sync-source/sync-target drift invalidates the proof.** The two
   recording times are distinct and each guards a different boundary:
   **target drift** is any advance of `main`'s tip after the step-1
   recording; **source drift** is any advance of the sync branch's head
   after the step-4 recording (i.e. any change made after the diff
   proof and pipeline were produced, not the changelog/version edits
   step 4 itself records). Either drift invalidates the corresponding
   proof and requires a refreshed branch, diff, proof, pipeline, and a
   fresh exact-head Gate 2 review before proceeding — exactly as a
   stable-base or protected-target drift invalidates the reconstruction
   release merge under the fail-closed section below.
6. **Exact-SHA source merge with a pre-merge target read — not an
   atomic target guarantee.** Immediately before merging the
   forward-sync MR, re-read `main`'s current tip via the platform API
   and require it to equal the recorded sync-target head exactly; a
   mismatch triggers step 5's target-drift refresh, not a
   proceed-anyway judgment call. Merge only through the platform API's
   exact-SHA merge parameter, set to the recorded sync-source head — no
   branch-name merge, no squash, no rebase. **State plainly: GitLab's
   merge API pins the source branch's HEAD (`sha` must match the
   source), but has no equivalent parameter for the target branch's
   HEAD.** The pre-merge read and the merge call are therefore two
   separate operations, not one atomic one — a narrow race window
   exists in which another commit could land on `main` between the
   read and the merge accepting. Where the platform tier in use does
   not expose a merge-train, merged-results pipeline, or equivalent
   serialization control that would close this window, the residual
   risk is bounded, not eliminated, and step 7 below is the mandatory
   detection-and-remediation control for it — never described as
   prevention.
7. **Post-merge verification is the enforcement point for step 6's
   race window, not an optional double-check.** Immediately after
   merge, record the resulting merge commit SHA and its first-parent
   (target-side) parent SHA. If that parent SHA does not exactly equal
   the recorded sync-target head, the race in step 6 occurred: `main`
   advanced between the pre-merge read and the merge accepting, and
   the merge commit's actual diff was not the diff Gate 2 reviewed.
   Independently of that check, verify the merge commit's diff against
   its actual first-parent is limited to exactly the changelog and
   version-field edits from steps 2-3 — proving no code, test, schema,
   or other file change slipped in. **Remediation on either check
   failing:** do not touch the tag or either reconstruction branch (the
   published release is unaffected — this step is metadata-only, per
   step 8); open a corrective forward-sync MR from a fresh branch at
   `main`'s new actual tip, following steps 1-7 again in full, and
   treat the defective merge commit as superseded evidence, not as a
   silent partial state.

   **Resolved by binding governor decision
   `20260814T040118Z-governor-reviewer-apkm.md`.** Detect-and-remediate
   (steps 6-7 above) is accepted as a **narrow, scoped exception** to
   this section's own fail-closed requirement, **only** for this
   post-release metadata-forward-sync merge — never equivalent to
   atomic compare-and-swap, and never a precedent for ordinary feature,
   governance, release-construction, or any other forward-sync merge.
   The exception holds only while all of the following remain true: the
   sync changes only the governed metadata surfaces (step 3);
   immediately-before-merge target re-read and exact-source-SHA-only
   merge (step 6); immediately-after-merge parent-SHA and diff checks
   (step 7); a mismatch is a proof-invalid merge requiring a fresh
   corrective MR through the full process, tag and reconstruction refs
   untouched (step 7's remediation); and human observation of merge
   activity is optional additional care, never the security property
   itself. **One additional condition:** if GitLab later supplies a
   usable target-bound merge parameter, merge train, or equivalent
   serialization control, that stronger mechanism must be evaluated
   before this exception is reused for any future application of this
   rule — do not assume the platform constraint checked at this
   application's authoring time still holds at a later one.
8. If this synchronization fails or is delayed, the published release
   remains valid; correct the sync forward without touching the tag or
   either reconstruction branch.

### Fail-closed behavior

Any of the following invalidates the exception and requires returning
to the governor rather than proceeding under assumed authority: drift
in the stable base tag identity; drift in the protected target
branch's resolved SHA (enforced by the release merge's mandatory
pre-merge target/source re-read and post-merge first-parent/second-
parent/tree-OID verification; a mismatch here is fail-closed before
the tag — though, per governor decision
`20260814T074427Z-governor-reviewer-33br.md`, this is accepted
post-landing detection relative to the isolated protected target, not
literal prevention of target drift); drift in the reconstruction
source branch's head between review and merge; a tree-delta or
frozen-out-set proof failure; an allowlist commit whose own Gate 2
approval cannot be independently re-confirmed at merge time; the
target branch's protection object differing from its own single
constant baseline at any of the three checkpoints; the source branch's
protection object differing from its construction baseline before the
one authorized construction-to-frozen transition or from its frozen
baseline at any point after that transition; the source branch
reaching Gate 2 evidence without that transition having occurred; the
pre-tag-push namespace check finding any non-absence or API ambiguity;
the pre-tag or post-publication protected-tag-rule re-read finding the
rule missing, widened, or no longer matching the exact intended tag;
or — for the metadata-forward-synchronization step — target drift
(`main`'s tip advancing after the sync-target head is recorded) or
source drift (the sync branch's head advancing after the sync-source
head is recorded, post-proof) that is not resolved by that step's
required refresh before merging, or the post-merge check finding the
forward-sync merge commit's actual first parent does not match the
recorded sync-target head (per that step's own remediation, not a
proceed-anyway judgment call).

## Exceptions

Lightweight tags for non-release purposes (e.g. `deploy-staging-20260405`) are exempt from the MR requirement but still subject to the immutability rule.
