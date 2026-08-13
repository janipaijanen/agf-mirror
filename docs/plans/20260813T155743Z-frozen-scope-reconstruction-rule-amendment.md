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
      "Contribution-auditor reviews the amendment plan." A coder-tester
      technical pre-review (`20260813T161358Z-coder-tester-kpt5.md`, not
      contribution-auditor's own review — explicitly self-disclosed as
      such) landed on the first exact head and returned COMPLETE/BLOCK:
      three blocking findings (B-FSR-1 merge-result-vs-approved-head
      identity, B-FSR-2 non-deterministic "union of diffs" proof, B-FSR-3
      incomplete forward-sync version-surface coverage) and one advisory
      (A-FSR-1 ancestry-vs-evidence-only ambiguity). All four addressed
      in this revision, each independently re-verified (B-FSR-1's
      SHA-256 claim recomputed and confirmed; B-FSR-2's replay redone
      fresh with concrete hashes; B-FSR-3's surface list now reuses the
      release plan's own enumeration verbatim; A-FSR-1 resolved via an
      actual `--no-ff` merge test that found real content conflict,
      settling the question with evidence rather than assertion). The
      named contribution-auditor's own review (distinct from this
      coder-tester technical pass) remains outstanding.
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Named as required consultation by the same governor decision, item
      2: "infosec-sentinel is consulted on protected refs, approval
      integrity, namespace checks, and forward synchronization" — exactly
      the four areas the rule amendment below governs. Requested; not yet
      received.
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
> - [ ] Every commit proposed for the reconstruction allowlist either
>       (a) *is* the exact SHA with binding Gate 2 approval, or (b) is a
>       merge commit whose first-parent binary patch is proven
>       byte-identical (SHA-256 of `git diff --binary <merge-base>
>       <approved-head>` equals SHA-256 of `git diff --binary
>       <merge-commit^1> <merge-commit>`) to the exact Gate-2-approved
>       head's own patch against its own reviewed merge base. Case (b)
>       exists because Gate 2 approvals are issued against a feature
>       branch's pre-merge head, while the mainline history only offers
>       the merge commit — the byte-identity proof is what makes citing
>       the merge commit truthful rather than approximate. Evidenced by
>       reference and by the recomputed hash, not re-litigated here.
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
>    Cherry-pick the reconstruction allowlist onto the *source* branch,
>    as **deterministic ordered replay**: the governing plan states the
>    exact ordered list of `git cherry-pick` invocations (commit SHA,
>    parent selection for merge commits, no squash), and that exact
>    sequence — not an unordered "apply these commits somehow" — is what
>    Gate 2 evidence reproduces. Any cherry-pick conflict is resolved
>    file-by-file, and the governing plan must record, for each
>    conflicted file, the **exact resolved content's hash** (e.g. SHA-256)
>    as a content-bound expected value — not a prose description of the
>    resolution policy. A resolution that discards either side's content
>    is ineligible and must be redone. Then apply the release-only
>    metadata change set (version, changelog, manifest, and other
>    enumerated release-truth surfaces per the governing plan) as the
>    final commit(s).
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
>    Gate 2.** "The diff looks like the union of the inputs" is not a
>    defined, reproducible Git operation and is explicitly **not**
>    sufficient — patch union is non-compositional the moment two
>    allowlisted commits touch the same insertion point (this
>    exception's own first application does exactly that). The proof
>    instead has two required, independently checkable parts:
>    - **Deterministic replay proof:** re-running the exact ordered
>      cherry-pick sequence and recorded conflict-resolution hashes from
>      step 2, from the exact stable base, must produce a tree whose Git
>      tree hash (or a binary base-to-head patch hash, either is
>      acceptable but the governing plan must state which) equals the
>      value the governing plan recorded at authoring time. A mismatch
>      is a fail-closed condition (see below), not a discretionary call.
>    - **Frozen-out-set negative check:** grep or otherwise search the
>      resulting tree for every named item in the frozen-out set (file
>      paths, manifest entries, changelog section markers) and prove
>      zero matches.
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
> reconciles `main`'s own version and changelog record with the fact that
> a subset of its `Unreleased` content has now shipped under an earlier
> tag — it does **not** pull anything out of `main`'s code, tests, or any
> non-version-metadata file, and it does not revert or rewrite any commit
> already on `main`.
>
> **This connection is evidence-only, not Git-ancestry-preserving.** A
> real `--no-ff` merge of the tagged release commit into `main` was
> tested empirically during this exception's first drafting and produces
> genuine content conflicts (not merely positional ones) wherever `main`
> has independently continued editing the same prose the reconstruction
> allowlist also touches — `main`'s own text is typically the more
> current, correct version at that point, and resolving such a conflict
> requires editorial judgment a mechanical merge cannot make safely. The
> forward-sync MR is therefore evidence connecting the release to `main`
> for a human/reviewer to follow, not a claim that the tag is a Git
> ancestor of `main`. To keep the weaker relationship reconstructable,
> the forward-sync MR description must explicitly name: the immutable
> tag, its exact commit and tree SHAs, the release MR, and both
> reconstruction branch names.
>
> 1. After successful tag publication, open a separate MR to `main` from
>    a fresh branch based on `main`'s then-current tip.
> 2. Split `main`'s own `CHANGELOG.md` and `agf/CHANGELOG.md`
>    `## [Unreleased]` sections so that exactly the entries corresponding
>    to the frozen release's allowlist (matched by issue/commit
>    reference, not by guesswork) move into a new
>    `## [<released version>] - <release date>` section, content-identical
>    to what the tag itself published, positioned in correct
>    chronological order among `main`'s existing version headings.
>    Everything else remains in `## [Unreleased]`, unchanged.
> 3. **Every governed current-version metadata surface updates to the
>    released version — the same surface list the governing release
>    plan's own release-only change set names, no narrower.** This
>    exception does not maintain a separate, independently-enumerated
>    surface list that can drift from the release plan's own; it reuses
>    that plan's list verbatim, restricted here to version-field edits
>    only (no code, test, or content changes). `agf/VERSION`'s new value
>    is a **last-released-version marker**: it does not imply `main`'s
>    own next tag will be that version, since `main`'s own next release
>    may be a later number encompassing the remaining `Unreleased`
>    content. State this explicitly in the forward-sync MR description so
>    a future reader does not mistake the marker for a scope commitment.
>    A split version state across governed surfaces (some updated, some
>    not) is never acceptable as a resting state — if the full surface
>    list cannot be updated together, stop and return to the governor
>    rather than normalizing partial sync.
> 4. Prove the forward-sync MR's diff touches only the changelog files
>    from step 2 and the version fields (not content) of the surfaces
>    from step 3 — no code, test, schema, or other file changes. This is
>    a mechanical, Gate-2-checkable condition, not advisory.
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
- **Reconstruction allowlist (ordered, deterministic replay):**
  1. `git cherry-pick 3456c1fa` — **conditional**, pending the bounded
     necessity check below (governor decision `2r8r`, item 1). Not
     consumed until the check runs and its result is recorded.
  2. `git cherry-pick -m 1 bcb0fadd` — the #76/#84/#85/#89 merge's
     first-parent diff. **Merge-result byte-identity proof (per the
     eligibility checklist's case (b)):** binding Gate 2 record `7qmr`
     approves MR !1167 at exact head `efaef6a27e6234181b85bd2af19acc11c709ec52`,
     not at merge commit `bcb0fadd`. Independently computed:
     `git diff --binary $(git merge-base efaef6a27e6234181b85bd2af19acc11c709ec52 d5ad15ccbca02feb6fa406ad60ee7c4dd1b9e719) efaef6a27e6234181b85bd2af19acc11c709ec52 | sha256sum`
     and
     `git diff --binary d5ad15ccbca02feb6fa406ad60ee7c4dd1b9e719 bcb0fadd9704b782ba02e9672943eae386e8e7e1 | sha256sum`
     both equal
     `baf3afa2e988ef6281236330447e79ec2726477d4f9491d58456d3367abebeae`.
     Byte-identical; case (b) is satisfied.
- **Frozen-out set (must be absent from the resulting tree):**
  `agf/agents/web-ux-reviewer.yaml` activation in
  `agf/agents/MANIFEST.yaml`; `independent-implementation-reviewer`
  activation/suspension entries in the same manifest; the maintenance-
  line release exception content in
  `agf/rules/workflow/release-via-merge-request-rule.md` beyond what
  already existed at the `v2.5.0` stable tag (i.e., this new frozen-
  scope-reconstruction section itself is **not** part of v2.6.0's
  content — it lands on `main` only, per this plan's own scope).
- **Deterministic replay proof.** Per the rule text's "either is
  acceptable but the plan must state which": **this plan designates the
  binary base-to-head patch hash as authoritative** (the Git tree hash
  is recorded too, as a cross-check, but the patch hash is what Gate 2
  evidence must reproduce). Re-verified fresh in a scratch worktree
  during this revision (not reused from earlier rounds' informal runs,
  which used a flawed conflict-resolution method — see the coordination
  ack accompanying this revision):
  - After `git cherry-pick 3456c1fa` + mechanical resolution:
    `CHANGELOG.md` SHA-256 `c3793351f2e592d10e1015b89acb98d5c958482863b52ba56e91a81f2002e5a9`,
    `agf/CHANGELOG.md` SHA-256 `f03a33dcb68de194f8799f4f0bdb75f2d83b08257f8e0db1516671dbf2fa57e7`
    (recorded for auditability; not independently authoritative).
  - After `git cherry-pick -m 1 bcb0fadd` + mechanical resolution on top:
    `CHANGELOG.md` SHA-256 `c1a6c3567c037380c83c6aade38e5353a5877e971f4736c2b7b4870e11d55fd1`,
    `agf/CHANGELOG.md` SHA-256 `90fb43be9ba761237d0dd372f65558525d5fc9586e53fe556371eb3864633cd6`.
  - **Authoritative:** final base-to-head binary patch
    (`git diff --binary f1932a1a486b772ca4353c7fa71c2279132fe0dc <final>`)
    SHA-256 `a9ae691f02a87ea42c07caa246133dbe42f3bd351ac9712342e15e63aa1c1ba2`.
    Cross-check: final tree hash `ca26fb93bc7aa49f2d926949eacec41c2d5b6074`.
  Both recorded before the release-only metadata change set is applied
  on top — that step's own diff is separately, mechanically verifiable
  against the governed version-surface list (forward-sync section
  above), not hash-pinned here since the release date/version strings
  are not fixed at plan-authoring time.
- **Functional verification on the reconstructed tree (pre-metadata-
  change-set):** `cargo check` exits 0; `cargo test --package agov
  --test coord_visibility` — 20 passed, 0 failed; frozen-out-set grep
  (`web-ux-reviewer|independent-implementation-reviewer` against
  `agf/agents/MANIFEST.yaml`) — 0 matches. Re-run required at the actual
  release MR's exact head — this scratch-worktree run is evidence the
  construction is sound, not a substitute for exact-head Gate 2
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
6. The `bcb0fadd`/`efaef6a2` merge-result byte-identity proof is
   independently re-verified at the actual release MR's exact head, not
   only cited from this plan (`kpt5`'s B-FSR-1).
7. The deterministic-replay authoritative hash (this plan: the binary
   base-to-head patch hash) is independently recomputed at the actual
   release MR's exact head and matches this plan's recorded value, or
   the mismatch is resolved before Gate 2 (`kpt5`'s B-FSR-2).
8. The forward-sync MR's diff is proven to cover exactly the governed
   version-surface list (reused verbatim from the release plan's own
   change set) plus the two changelog files, with no partial-surface
   resting state (`kpt5`'s B-FSR-3).
9. The forward-sync MR description explicitly names the immutable tag,
   its commit/tree SHAs, the release MR, and both reconstruction branch
   names, consistent with the evidence-only (non-ancestry) linkage this
   plan declares (`kpt5`'s A-FSR-1).

## Stop conditions

Stop and return to the governor if: the necessity check's result is
ambiguous or contradicts the recorded evidence; any cherry-pick conflict
cannot be resolved without discarding either side's content; the
deterministic-replay patch/tree hash, or the frozen-out-set proof, fails
at the actual release MR's exact head even though it passed in this
plan's scratch-worktree verification; the merge-result byte-identity
check fails for any allowlisted merge commit; the forward-sync's
governed version-surface list cannot be updated together (no partial
sync); or contribution-auditor/infosec-sentinel raise a finding this
plan does not already address.
