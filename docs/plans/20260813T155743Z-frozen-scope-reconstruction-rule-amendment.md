---
id: frozen-scope-reconstruction-rule-amendment
status: pending-gate1
topic: frozen-scope-reconstruction-rule-amendment
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Named as required reviewer by governor decision
      `20260813T154648Z-governor-reviewer-2r8r.md`, ordering item 2:
      "Contribution-auditor reviews the amendment plan." Requesting that
      review as part of this plan's publication; not yet received at
      drafting time.
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Named as required consultation by the same governor decision, item
      2: "infosec-sentinel is consulted on protected refs, approval
      integrity, namespace checks, and forward synchronization" — exactly
      the four areas the rule amendment below governs. Requesting that
      consultation as part of this plan's publication; not yet received
      at drafting time.
---

# Plan: add a "Frozen-scope reconstruction" exception to release-via-merge-request-rule.md

## Decision requested

Add a new, narrowly-eligible exception section to
`agf/rules/workflow/release-via-merge-request-rule.md` (and its
`.claude/rules/` projection), authorizing the release-construction
mechanism proposed in MR !1492's plan and conditionally approved by
governor decision `2r8r`: a release whose content is a governed,
enumerated subset of what `main` currently contains, built as an
independent lineage from the last immutable stable tag forward, never
merged back into `main` as a content-changing merge (only as a
metadata-only forward-sync, specified below).

This is governance infrastructure on `main` — the amendment itself, not
the v2.6.0 release. Per `2r8r`: "The rule amendment is governance
infrastructure on `main`; it is not part of the v2.6.0 artifact
contents." Ordering, per `2r8r`'s "Next ownership and ordering":
this plan (item 1, planner-architect) → contribution-auditor review +
infosec-sentinel consultation (item 2) → this plan's own Gate 1/Gate 2 →
only then does MR !1492 return for a fresh exact-head Gate 1 decision
(item 4).

## Why the existing exceptions don't cover this

`release-via-merge-request-rule.md` currently has two paths:

1. **Standard workflow** — release branch from `main`'s current tip, MR
   back to `main`, tag the merge commit. Assumes the release's content
   *is* whatever `main` currently holds. Does not fit here: `main` holds
   groups unrelated to the frozen v2.6.0 scope (`8hgm`), and per
   governor decision `2r8r` a one-off waiver is explicitly not
   authorized — a rule amendment is required instead.
2. **Maintenance-line exception** — re-publish an *already-tagged,
   immutable* release-candidate as stable after `main` moved on. Does
   not fit here: no `v2.6.0-rc` tag was ever cut; there is no candidate
   to promote, only a set of individually-approved commits scattered
   across `main`'s history that need to be reconstructed together for
   the first time.

Both existing paths assume the release's content already exists as one
contiguous, already-approved artifact (a `main` tip or a tagged RC). This
amendment covers the third case: content that is real and approved, but
scattered, requiring reconstruction rather than promotion.

## Proposed rule text

Insert as a new top-level section in
`agf/rules/workflow/release-via-merge-request-rule.md`, positioned after
the existing "Maintenance-line exception" section (same file, same
`## `-level heading depth), before "## Exceptions":

---

> ## Frozen-scope reconstruction exception (optional, narrowly eligible)
>
> Canonical owner: `agf/rules/workflow/release-via-merge-request-rule.md`.
> The standard workflow and the maintenance-line exception above are
> unweakened by this section. It exists for exactly one case neither
> covers: a governor-frozen release scope (an enumerated, named set of
> already Gate 2-approved commits) that must ship as its own immutable
> artifact while `main` has since accumulated other, unrelated,
> individually-approved work that must **not** enter that artifact, and
> no immutable release-candidate tag already exists to promote.
>
> ### Eligibility (all conditions required, evidenced before any branch is created)
>
> - [ ] A binding governor record names the frozen release scope by
>       tracker identifier (issues/MRs), the exact immutable stable tag
>       to build from, and states why neither the standard workflow nor
>       the maintenance-line exception applies.
> - [ ] Every commit proposed for the reconstruction allowlist already
>       has its own binding Gate 2 approval at the exact SHA being
>       cherry-picked, evidenced by reference, not re-litigated here.
> - [ ] A separate Gate 1-approved release-publication plan enumerates:
>       the exact stable base tag/commit, the exact ordered list of
>       commit SHAs to cherry-pick (the "reconstruction allowlist"), the
>       exact release-only metadata change set on top, and the exact
>       named content that must be **absent** from the resulting tree
>       (the "frozen-out set").
> - [ ] Any commit on the allowlist admitted under a *conditional*
>       prerequisite-necessity exception (per the governing decision)
>       has that condition's check specified and executed, with its
>       result recorded, before the commit is treated as consumed. A
>       conditional exception that is never checked is not eligible.
> - [ ] The Git tag, GitLab Release, generic-package, and OCI namespaces
>       for the new version are independently confirmed absent.
>
> ### Protected target and source
>
> 1. Create a reconstruction target branch (e.g. `reconstruct/vX.Y`) at
>    the exact stable base tag/commit named by the eligible governor
>    record. Protect it — reject force-push and deletion, restrict push
>    and merge to access level 40 (Maintainer) — before any cherry-pick
>    lands. Confirm the protection rule from the platform API as
>    evidence.
> 2. Create a reconstruction source branch (e.g. `release-vX.Y.Z`) at the
>    same exact base commit, with equivalent level-40 protection.
>    Cherry-pick the reconstruction allowlist onto the *source* branch in
>    the plan's specified order, one commit per cherry-pick (no squash),
>    then apply the release-only metadata change set (version, changelog,
>    manifest, and other enumerated release-truth surfaces per the
>    governing plan) as the final commit(s). Any merge conflict must be
>    resolved by union (keep both sides' content); a resolution that
>    drops either side's content is ineligible and must be redone. Record
>    the exact resolution for each conflicted file in the release MR.
> 3. Open a non-squash, non-rebase merge request from the reconstruction
>    source branch to the reconstruction target branch. This MR's diff
>    against the target (== the stable base) is exactly the allowlist's
>    cumulative diff plus the release-only metadata change set — this
>    *is* the reviewable release-truth diff Gate 2 evaluates. Retain both
>    branches after merge and after tagging.
> 4. A protected-tag rule matching the exact intended release tag must
>    exist, tag creation restricted to access level 40, before the tag is
>    created.
> 5. **Mandatory tree-delta proof, evidenced in the release MR before
>    Gate 2:** the MR diff described in step 3 must equal, file-for-file
>    and line-for-line, the union of the allowlisted commits' own diffs
>    plus the release-only metadata change set — no more, no less. A
>    companion negative check must grep or otherwise search the resulting
>    tree for every named item in the frozen-out set (file paths,
>    manifest entries, changelog section markers) and prove zero matches.
>    Both proofs are exact-head Gate 2 evidence, not advisory.
> 6. Exact-head Gate 2 remains binding governor authority, applying the
>    same standard the maintenance-line exception applies: the named SHA
>    with the reviewed pipeline, nothing else. The merge invocation's
>    exact command (including branch-retention behavior) and an immediate
>    post-merge API read proving both refs still exist at the intended
>    SHAs are required Gate 2 evidence, matching the maintenance-line
>    exception's own retention-proof requirement.
>
> ### Tag and publication
>
> Ordinary exact-head Gate 2, tag-pipeline, immutable-namespace, and
> anonymous downloaded-artifact verification — unweakened from the
> standard workflow's own requirements
> (`immutable-tags-rule.md`; every archive, checksum, provenance record,
> package, and OCI digest binds to the tag commit exactly as it does for
> any other release).
>
> ### Metadata-forward synchronization (content-preserving, main untouched)
>
> `main` already contains the frozen release's content (it merged there
> independently, earlier) plus the frozen-out set. This section
> reconciles `main`'s own changelog record with the fact that a subset of
> its `Unreleased` content has now shipped under an earlier tag — it does
> **not** pull anything out of `main`'s code, tests, or any non-changelog
> file, and it does not revert or rewrite any commit already on `main`.
>
> 1. After successful tag publication, open a separate MR to `main` from
>    a fresh branch based on `main`'s then-current tip.
> 2. The only permitted change: split `main`'s own `CHANGELOG.md` and
>    `agf/CHANGELOG.md` `## [Unreleased]` sections so that exactly the
>    entries corresponding to the frozen release's allowlist (matched by
>    issue/commit reference, not by guesswork) move into a new
>    `## [<released version>] - <release date>` section, content-identical
>    to what the tag itself published, positioned in correct
>    chronological order among `main`'s existing version headings.
>    Everything else remains in `## [Unreleased]`, unchanged.
> 3. `agf/VERSION` on `main` updates to the released version **only as a
>    last-released-version marker** — this does not imply `main`'s own
>    next tag will be that version; `main`'s own next release may be a
>    later number encompassing the remaining `Unreleased` content. State
>    this explicitly in the forward-sync MR description so a future
>    reader does not mistake the marker for a scope commitment.
> 4. Prove the forward-sync MR's diff touches only `CHANGELOG.md`,
>    `agf/CHANGELOG.md`, and `agf/VERSION` — no code, test, schema, or
>    other file changes. This is a mechanical, Gate-2-checkable
>    condition, not advisory.
> 5. If this synchronization fails or is delayed, the published release
>    remains valid; correct the sync forward without touching the tag or
>    either reconstruction branch.
>
> ### Fail-closed behavior
>
> Any of the following invalidates the exception and requires returning
> to the governor rather than proceeding under assumed authority:
> drift in the stable base tag identity, drift in the protected target
> branch's resolved SHA, drift in the reconstruction source branch's
> head between review and merge, a tree-delta or frozen-out-set proof
> failure, or an allowlist commit whose own Gate 2 approval cannot be
> independently re-confirmed at merge time.

---

## Application to v2.6.0 (this amendment's first and current intended use)

- **Frozen release scope:** issues #76, #84, #85, #89 (`8hgm`).
- **Stable base tag:** `v2.5.0`, peeled commit
  `f1932a1a486b772ca4353c7fa71c2279132fe0dc`.
- **Reconstruction allowlist (ordered):**
  1. `3456c1fa` — **conditional**, pending the bounded necessity check
     below (governor decision `2r8r`, item 1). Not consumed until the
     check runs and its result is recorded.
  2. `bcb0fadd` — the #76/#84/#85/#89 merge, applied with `-m 1` (its
     first-parent diff only).
- **Frozen-out set (must be absent from the resulting tree):**
  `agf/agents/web-ux-reviewer.yaml` activation in
  `agf/agents/MANIFEST.yaml`; `independent-implementation-reviewer`
  activation/suspension entries in the same manifest; the maintenance-
  line release exception content in
  `agf/rules/workflow/release-via-merge-request-rule.md` beyond what
  already existed at the `v2.5.0` stable tag (i.e., this new frozen-
  scope-reconstruction section itself is **not** part of v2.6.0's
  content — it lands on `main` only, per this plan's own scope).
- **Tree-delta proof:** independently re-verified in a scratch worktree
  during round-3 review (`mm4k`): both cherry-picks apply with exactly
  two mechanical, union-resolvable conflicts (both `CHANGELOG.md` files),
  `cargo check` exits 0, and the full `coord_visibility` test suite (20
  tests) passes against the reconstructed tree. Re-run required at the
  actual release MR's exact head — the scratch-worktree run is evidence
  the construction is sound, not a substitute for exact-head Gate 2
  evidence.

## Bounded without-`3456c1fa` necessity check (governor decision `2r8r`, item 1)

**What:** Run the tag-simulating tooling-release CI job
(`.gitlab-ci.yml`'s Rust tooling archive build, the job containing the
`for target in x86_64-unknown-linux-musl ...` loop at line ~1063) against
the reconstruction target branch **before** cherry-picking `3456c1fa`,
with `aarch64-unknown-linux-musl` temporarily reinstated in the target
loop (i.e., simulating the pre-`3456c1fa` state), as a manual pipeline
run on a throwaway, non-protected branch — never on a protected ref, never
tag-triggered.

**How:** Trigger the job manually against the reconstruction target
branch's pre-`3456c1fa` state with the target list temporarily including
`aarch64-unknown-linux-musl`. Do not merge this throwaway state anywhere.

**Result recording:**
- If the job fails with disk exhaustion (`no space left on device` or
  equivalent, matching `3456c1fa`'s own original evidence) on the shared
  `saas-linux-small-amd64` runner class: this **conclusively satisfies**
  the necessity condition per `2r8r`. Record the pipeline URL, exact
  failure log excerpt, and runner class in a coordination completion
  record. `3456c1fa` becomes effective and is cherry-picked.
- If the job succeeds reliably: **stop.** Do not cherry-pick `3456c1fa`.
  Return to the governor with the result before proceeding — per `2r8r`,
  automatic inclusion is not authorized on a success result.
- Any other outcome (timeout unrelated to disk, runner unavailability,
  ambiguous failure): treat as inconclusive, do not consume the
  exception, return to the governor.

This check runs once, its result is durable evidence for this exact
release; it does not need to be repeated for future releases unless the
governor's decision is revisited.

## Documentation projection (per `documentation-rule-projection-rule.md`)

This plan changes normative guidance in a governed rule
(`release-via-merge-request-rule.md`). Six-row screen:

| Surface | Disposition |
|---|---|
| Normative owner | Updated — `agf/rules/workflow/release-via-merge-request-rule.md` gains the new section above. |
| Adopter-facing explanation | Updated — `agf/docs/site/docs/operate/review-and-release.md` gains a subsection analogous to its existing maintenance-line-exception coverage, per `documentation-audience-rule.md` (business-user/admin audience: describe when this path exists and what evidence it produces, not the full rule mechanics). |
| Runbook/situation map | Updated — `agf/skills/release/SKILL.md`'s existing Step 0 fail-closed selection procedure (added for the maintenance-line exception) gains this exception as a third named path with its own eligibility gate. |
| AI landing/loaders | Updated — `.claude/rules/workflow/release-via-merge-request-rule.md` (foundry's own active local projection of the canonical rule) mirrors the same new section, carrying an explicit "Canonical owner" pointer line, matching exactly how the maintenance-line exception's own projection record (`20260809T163051Z-v250-maintenance-release-line-rule.yaml`) handled this row. `AGENTS.md` / `agf/templates/AGENTS.md` not applicable — already point generically to `agf/rules/workflow/` without enumerating individual rules. `.claude/CLAUDE.md` not applicable — does not exist in this repository. |
| Validation/drift evidence | Not mechanically decidable — same reasoning as the maintenance-line exception's own precedent record: whether the eligibility fence is honored at release time is evidence-based governor/reviewer judgment at each future release, not something a validator decides. No new mechanical validator introduced or claimed. |
| Distribution | Confirmed — `agf/rules/**`, `.claude/rules/**` (foundry-local, correctly not part of the adopter distribution manifest, matching the maintenance-line precedent), `agf/docs/site/**`, and `agf/skills/**` globs in `agf/dist/release-manifest.yaml` cover all touched adopter-facing paths. |

Full structured record:
`docs/reviews/rule-projection/20260813T155743Z-frozen-scope-reconstruction-rule-amendment.yaml`,
distribution claims independently verified against
`agf/dist/release-manifest.yaml` before writing (not assumed from the
maintenance-line precedent alone).

## Acceptance criteria

1. The new rule section is added to both `agf/rules/workflow/release-
   via-merge-request-rule.md` and its `.claude/rules/` projection,
   identical content.
2. `agf/docs/site/docs/operate/review-and-release.md` and
   `agf/skills/release/SKILL.md` are updated per the projection screen.
3. The rule-projection YAML record exists and validates.
4. This plan receives its own exact-head Gate 1 and Gate 2 approval
   before MR !1492 returns for a fresh Gate 1 decision.
5. The v2.6.0 application section above is the first use of the new
   exception and must itself pass every eligibility condition the new
   rule text states — this plan does not grant itself a waiver from its
   own rule.

## Stop conditions

Stop and return to the governor if: the necessity check's result is
ambiguous or contradicts the recorded evidence; any cherry-pick conflict
cannot be resolved by clean union (i.e., requires discarding either
side's content); the tree-delta or frozen-out-set proof fails at the
actual release MR's exact head even though it passed in this plan's
scratch-worktree verification; or contribution-auditor/infosec-sentinel
raise a finding this plan does not already address.
