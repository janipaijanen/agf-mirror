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
      settling the question with evidence rather than assertion). A
      renewed coder-tester technical review
      (`20260813T164610Z-coder-tester-qrmm.md`) confirmed all four fixes
      COMPLETE/CLEAR against the revised head. Coder-tester then prepared
      a decision-ready evidence packet
      (`20260813T180855Z-coder-tester-qrr8.md`) for the governor's own
      required non-self judgment, explicitly declining to claim it
      satisfied the role itself. The governor's own non-self
      contribution-auditor review
      (`20260813T182631Z-contribution-auditor-1tq2.md`) landed
      COMPLETE/CLEAR at exact head `80c0d78b`: zero blocking findings, one
      advisory (`A-FSR-CA-AID-1`, disposed — a single successful bounded
      necessity-check run must not be read as proof of reliability; stop
      and return to the governor rather than inferring it), and an
      explicit human precedent judgment accepting the reusable exception
      for Gate 1 consideration on the conjunctive-eligibility, per-use
      binding-freeze, fail-closed-to-governor terms this plan states.
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Named as required consultation by the same governor decision, item
      2: "infosec-sentinel is consulted on protected refs, approval
      integrity, namespace checks, and forward synchronization." A
      coder-tester infosec-review-aid
      (`20260813T165120Z-coder-tester-mdth.md`, explicitly not an
      infosec-sentinel verdict) queried the live GitLab API and found
      three blocking platform-control findings (B-FSR-SEC-1 overclaimed
      deletion-proof guarantee, B-FSR-SEC-2 direct-push bypass of the
      reviewed merge path, B-FSR-SEC-3 unclosed namespace time-of-check
      race) and one advisory (A-FSR-SEC-1 platform-identity vs.
      governance-authority boundary). All four independently re-verified
      against the live project (Owner access 50 confirmed; `release/2.5`
      protection confirmed at push level 40, unprotect level 40) and
      addressed in one revision. A round-3 coder-tester technical review
      (`20260813T172451Z-coder-tester-1jnk.md`) then found that the
      B-FSR-SEC-1/2 fix introduced a genuine internal contradiction — the
      required source-branch construction-to-frozen transition would
      itself trip the fail-closed "any protection drift invalidates"
      rule as originally worded — plus an advisory (plan-local
      self-reference in reusable rule text). Both confirmed by re-reading
      the contradiction directly and fixed with a two-baseline scheme
      (target: single constant baseline; source: construction baseline
      until one authorized transition, frozen baseline after). A
      renewed coder-tester technical review
      (`20260813T175059Z-coder-tester-24rj.md`) confirmed the fix
      COMPLETE/CLEAR. A further coder-tester infosec-review-aid at the
      final exact head (`20260813T184351Z-coder-tester-qpj4.md`) found
      one new blocking finding, `B-FSR-SEC-FH-1`: the
      metadata-forward-synchronization section bound the reconstruction
      release merge to reviewed source/target SHAs but did not bind the
      later forward-sync-to-`main` merge the same way, leaving it able to
      authorize a different merge context (a stale sync-source head or an
      advanced `main` tip) without refreshed proof. Fixed in this
      revision by adding explicit sync-source/sync-target SHA recording,
      an any-advance-invalidates rule mirroring the existing tree-delta
      fail-closed condition, a pre-merge exact-target-SHA re-read plus
      exact-sync-source-SHA API merge requirement, and a post-merge
      allowed-diff verification step. A round-2 coder-tester technical
      review of that fix (`20260813T185934Z-coder-tester-stqp.md`) found
      one further blocking finding, `B-FSR-SEC-FH-R2-1`: recording the
      sync-source head at branch creation (before the required
      changelog/version edits) meant those very edits would trip the
      just-added drift rule, making the branch unable to reach Gate 2
      while satisfying both. Fixed by separating the two recording
      times: the sync-target head is recorded once at branch creation
      (before any edits); the sync-source head is recorded once after
      the changelog/version-field proof and pipeline exist — the
      required edits between those two points are not themselves drift,
      only changes after each respective recording are. A round-six
      exact-head technical review
      (`20260813T191518Z-coder-tester-6pet.md`) confirmed the fix
      COMPLETE/CLEAR and flagged that the human contribution-auditor
      CLEAR and infosec-sentinel's own consultation both needed
      refreshing at the new head; both refresh requests were filed
      (`20260813T193341Z-coder-tester-dmjq.md`,
      `20260813T193344Z-coder-tester-gd2t.md`). A fresh-context Codex
      security review commissioned by the governor
      (`20260813T200130Z-coder-tester-8hn8.md`, explicitly not
      infosec-sentinel's own verdict) then found two further blocking
      findings and one advisory at that head: `B-FSR-FRESH-1` — the
      forward-sync merge's "exact-SHA merge" language could be read as
      claiming atomic target binding, when GitLab's merge API only pins
      the source branch's HEAD; fixed by stating the non-atomicity
      plainly, citing the already-governed absence of merge-train/
      merged-results support on this project's GitLab tier
      (`coordination-publication-rule.md`), and making the post-merge
      check (actual merge-commit parent vs. recorded sync-target head)
      the defined detection-and-remediation control, not prevention.
      `B-FSR-FRESH-2` — the frozen-out negative proof was a curated
      2-item named list rather than a structural check; fixed by
      switching to attribution-based verification (every file in the
      reconstructed tree's diff-from-base must be attributable to a
      named allowlisted commit's own diff) and independently
      reproducing the v2.6.0 application's actual file list in a scratch
      worktree to confirm it: exactly 10 files, all attributable, no
      role-YAML/manifest/doc/rule-path leakage. `A-FSR-FRESH-1`
      (advisory) — added a protected-tag-rule object re-read at the
      pre-tag and post-publication checkpoints, alongside the existing
      namespace-absence re-reads. A round-seven exact-head technical
      review (`20260813T201833Z-coder-tester-adjp.md`) then found the
      `B-FSR-FRESH-1` fix was a mitigation, not a resolution:
      `B-FSR-R7-1` — detect-and-remediate-after-landing is not the same
      property as `2r8r`'s literal fail-closed-on-target-drift
      requirement, and this plan does not have standing authority to
      narrow that requirement on its own; **left explicitly open, routed
      to the governor as a named decision point (acceptance criterion
      19), not resolved by further planner text.** `B-FSR-R7-2` — path
      attribution alone does not prove a manually resolved conflict's
      *content* is free of unauthorized additions; fixed by adding a
      third required proof (content faithfulness for every manually
      resolved conflict, independently re-verified line-by-line for both
      `CHANGELOG.md` conflicts in this application). A round-eight
      exact-head technical review
      (`20260814T035143Z-coder-tester-qk8s.md`) confirmed `B-FSR-R7-1`
      remains correctly open pending the governor and found the
      round-seven `B-FSR-R7-2` fix itself incomplete: `B-FSR-R8-1` —
      content faithfulness was scoped only to conflict-marker regions, but
      a paused cherry-pick lets an operator stage arbitrary edits in a
      cleanly-applied file or a non-conflicting region of a conflicted
      file, which would pass both path attribution and the deterministic-
      replay hash undetected. Fixed by replacing marker-scoped review with
      a whole-result requirement: a documented, mechanical
      conflict-resolution algorithm, applied by an independent party
      working only from the stable base, the named commit hashes, and the
      algorithm itself — never touching the author's already-resolved
      tree — with the binding constraint that only cherry-pick-flagged
      conflicts may be touched at all, and only within their marker
      regions. Re-derived independently twice in fresh scratch worktrees
      with identical resulting hashes both times; those hashes now
      supersede every earlier round's informally-produced values. The
      governor then decided `B-FSR-R7-1`
      (`20260814T040118Z-governor-reviewer-apkm.md`): detect-and-
      remediate is accepted as a narrow, scoped exception to `2r8r` for
      this post-release metadata-forward-sync merge only, conditioned on
      named mandatory boundaries plus a new platform-reassessment
      condition for any future reuse — reflected in this revision
      (acceptance criterion 19 now satisfied). The named
      infosec-sentinel's own consultation (distinct from these
      coder-tester-assisted and fresh-context passes) remains the last
      outstanding item before governor Gate 1.
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
>    record, before any cherry-pick lands, protected as: `push_access_
>    levels: No one` (level 0 — matching how `main` itself is actually
>    configured on this platform, not the weaker "level 40 push and
>    merge" pattern), `merge_access_levels: Maintainers` (level 40),
>    `allow_force_push: false`. **State the platform guarantee
>    accurately, not optimistically:** branch protection prevents
>    Git-level force-push and deletion, but does **not** make the ref
>    undeletable to a Maintainer/Owner acting through the platform UI or
>    API, who can unprotect then delete. This is a residual risk the
>    protection setting does not close; it is closed instead by treating
>    unexpected unprotection, deletion, or protection-setting drift as a
>    fail-closed integrity incident (see below), and by reading and
>    recording the full protection object from the platform API at three
>    points: immediately after creation, immediately before the release
>    MR merges, and immediately after tag publication. **The target's
>    baseline is single and constant**: its protection object recorded at
>    all three points must be identical. Any difference at any point
>    invalidates the exception — the target never has an authorized
>    reason to change.
> 2. Create a reconstruction source branch (e.g. `release-vX.Y.Z`) at the
>    same exact base commit. **Construction and freeze are separate
>    protection states for the *source* branch specifically — the target
>    has no such transition (step 1's single constant baseline is
>    unaffected).** During construction (cherry-picks and the metadata
>    change set being assembled), the source branch may carry the minimum
>    push access needed to do that work (e.g. level 40); record this as
>    the source's **construction baseline** at creation. Before the
>    reconstruction release MR's exact-head Gate 2 evidence is presented,
>    the source branch **must** be reprotected to `push_access_levels: No
>    one`, `merge_access_levels: Maintainers`, `allow_force_push: false`
>    — the same posture as the target — and this single, one-time
>    transition is recorded as the source's **frozen baseline**,
>    immediately after reprotection, alongside both branches' resolved
>    head SHAs, as Gate 2 evidence. From that point forward the source's
>    fail-closed comparison is against its **frozen baseline**, not its
>    construction baseline — the recorded transition is the one
>    authorized difference; any other change, before or after it, is not.
>    A level-40 "push and merge" setting held all the way to Gate 2 grants
>    direct-push authority indistinguishable from a reviewed merge, which
>    defeats the point of the reviewable MR in step 3 below; this
>    two-baseline scheme is what actually closes that gap without
>    contradicting the drift check. Cherry-pick the reconstruction
>    allowlist onto the *source* branch,
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
>    instead has three required, independently checkable parts —
>    **reproducibility, path attribution, and content faithfulness are
>    three distinct properties; none of the three substitutes for
>    another** (round-seven review `adjp`'s `B-FSR-R7-2` on a two-part
>    version of this proof: path attribution alone does not show that a
>    *manually resolved* file's content is free of unauthorized
>    additions, and reproducing a recorded hash proves the replay is
>    deterministic, not that the originally recorded resolution was
>    itself faithful):
>    - **Deterministic replay proof:** re-running the exact ordered
>      cherry-pick sequence and recorded conflict-resolution hashes from
>      step 2, from the exact stable base, must produce a tree whose Git
>      tree hash (or a binary base-to-head patch hash, either is
>      acceptable but the governing plan must state which) equals the
>      value the governing plan recorded at authoring time. A mismatch
>      is a fail-closed condition (see below), not a discretionary call.
>      This proves the replay is *reproducible*, not that the recorded
>      result is *faithful* — see the next check.
>    - **Path attribution (necessary, not sufficient):** compute `git
>      diff --stat` of the resulting tree against the stable base, and
>      independently compute `git diff --stat` (or full diff) of each
>      allowlisted commit against its own immediate parent. Every file
>      path in the resulting tree's diff-from-base must appear in at
>      least one allowlisted commit's own diff. Any file path present in
>      the resulting diff that is not attributable to a named allowlisted
>      commit fails this check — but a file passing this check has only
>      been shown to be an *expected file*, not *expected content*: a
>      manually resolved conflict can stage arbitrary text inside an
>      otherwise-expected file without tripping this check alone.
>    - **Whole-result content faithfulness, independently re-derived —
>      not scoped to conflict-marker regions (round-eight review `qk8s`'s
>      `B-FSR-R8-1` on a marker-scoped version of this check: a paused
>      cherry-pick lets an operator stage arbitrary edits anywhere in the
>      index — a cleanly-applied file, or a non-conflicting region of a
>      conflicted file — and both would pass path attribution *and* the
>      deterministic-replay hash, because that hash only proves the
>      *recorded* result is reproducible, never that the recording itself
>      was innocent of manual staging).** Marker-scoped review of only
>      the conflicted regions cannot close this gap, because the risk is
>      *not* confined to conflicted regions. The proof instead requires
>      an **independent party who never reads or trusts the author's
>      already-resolved tree at all**: starting only from (a) the exact
>      stable base, (b) the named allowlisted commits' own hashes, and
>      (c) a written, mechanical conflict-resolution algorithm recorded
>      in the governing plan (not a copy of the author's resolved text —
>      an algorithm specific enough that a second independent party could
>      follow it without consulting the first), the independent party
>      re-runs the entire cherry-pick sequence from scratch in a fresh
>      worktree under one binding constraint on the procedure itself:
>      **the only files touched are the ones the cherry-pick tool itself
>      flags as conflicted, and within each such file the only edits made
>      are strictly inside the tool's own conflict-marker-delimited
>      regions, replaced per the documented algorithm — every other byte
>      in every file, conflicted or not, is committed exactly as the
>      cherry-pick's own mechanical application (including its own
>      auto-merge of non-conflicting hunks) produced it, with no
>      supplementary `git add` of anything beyond what conflict
>      resolution requires.** This constraint is what closes `B-FSR-R8-1`:
>      it removes the opportunity to stage unrelated edits at all, rather
>      than trying to detect them after the fact. The independent party
>      then compares the resulting tree/patch hash to the recorded value.
>      **Because this reproduction never touches the author's staged
>      index or resolved files at any point, a match is proof that the
>      entire result — cleanly-applied files and conflict-resolved files
>      alike, marker regions and non-conflicting regions alike — is
>      fully and solely derived from the base, the named commits' own
>      diffs, and the documented algorithm, with no channel for
>      unaccounted staged content anywhere in the tree.** A mismatch is a
>      fail-closed condition exactly like the deterministic-replay proof
>      above (the two checks share evidence but are not one check: the
>      deterministic-replay proof establishes the *recorded* value is
>      reproducible from the *recorded* procedure; this proof establishes
>      that a party with no access to the author's index reaches the
>      *same* value from only base + commits + algorithm, which the
>      recorded-procedure check alone cannot show).
>    All three are exact-head Gate 2 evidence, not advisory.
> 6. Exact-head Gate 2 remains binding governor authority, applying the
>    same standard the maintenance-line exception applies: the named SHA
>    with the reviewed pipeline, nothing else. **Platform credentials
>    execute a governance decision; they do not originate one.** Holding
>    Owner or Maintainer access on the platform proves who could act, not
>    which AGF-governed role or human decided the release-truth diff was
>    correct — the two are different claims, and only the binding
>    governor coordination record makes the second one. Before merge,
>    that record must name: the exact source-branch head SHA, the exact
>    target-branch head SHA, the expected tree/patch hash from the
>    deterministic-replay proof, and the canonical project identifier.
>    The merge itself must be performed via the platform's merge API with
>    the `sha` parameter set to the exact reviewed source head (never a
>    UI click against "whatever the branch currently points to"),
>    `squash=false`, and source-branch retention requested explicitly.
>    Read the merge response and both refs immediately afterward,
>    confirming the resulting merge commit, and that both branches still
>    exist at their intended SHAs — required Gate 2 evidence, matching
>    the maintenance-line exception's own retention-proof requirement.
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
> **The eligibility checklist's namespace-absence check is a
> point-in-time result, not a reservation.** Time elapses between that
> early check and the actual tag push — enough for another actor or
> process to occupy the same names. A second, independent, authenticated
> namespace check (Git tag, GitLab Release, generic package across every
> relevant status — default, hidden, processing, error,
> pending_destruction, deprecated — paginated fully, and exact OCI
> repository tag with the repository ID resolved fresh, not cached) is
> required **immediately before the tag is pushed**, bound to the exact
> merge commit and tree the release MR produced. Any non-absence result
> or API ambiguity at this second check stops the release; it is not
> treated as equivalent to the earlier result. After publication, re-read
> each namespace and bind its resulting immutable identity/digest to the
> tag pipeline as evidence.
>
> **Protected-tag-rule object, not just namespace absence.** The
> eligibility checklist's protected-tag-rule check (naming the exact
> intended tag) is also a point-in-time result. Re-read the exact
> protected-tag-rule object via the platform API immediately before the
> tag push (same checkpoint as the namespace recheck above) and record
> it as evidence; a rule that no longer matches the exact intended tag,
> or that has been removed or widened, stops the release under the
> fail-closed section below. Re-read it once more after publication and
> bind the confirmed rule state to the tag-pipeline evidence, alongside
> the namespace identities.
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
>    a fresh branch created exactly at `main`'s then-current tip. Record
>    that tip SHA as the **sync-target head** in the forward-sync MR
>    description — this is the binding reference point for the target
>    side of every check below. Do **not** record a source head yet: the
>    branch has no edits at this point, so there is nothing to bind as
>    the reviewed source.
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
>    a mechanical, Gate-2-checkable condition, not advisory. Once this
>    proof and its pipeline are produced, record the branch's resulting
>    exact head as the **sync-source head** in the forward-sync MR
>    description — this is the reviewed content presented for Gate 2, and
>    the binding reference point for the source side of every check
>    below.
> 5. **Sync-source/sync-target drift invalidates the proof.** The two
>    recording times are distinct and each guards a different boundary:
>    **target drift** is any advance of `main`'s tip after the step-1
>    recording; **source drift** is any advance of the sync branch's head
>    after the step-4 recording (i.e. any change made after the diff
>    proof and pipeline were produced, not the changelog/version edits
>    step 4 itself records). Either drift invalidates the corresponding
>    proof and requires a refreshed branch, diff, proof, pipeline, and a
>    fresh exact-head Gate 2 review before proceeding — exactly as a
>    stable-base or protected-target drift invalidates the reconstruction
>    release merge under the fail-closed section below.
> 6. **Exact-SHA source merge with a pre-merge target read — not an
>    atomic target guarantee.** Immediately before merging the
>    forward-sync MR, re-read `main`'s current tip via the platform API
>    and require it to equal the recorded sync-target head exactly; a
>    mismatch triggers step 5's target-drift refresh, not a
>    proceed-anyway judgment call. Merge only through the platform API's
>    exact-SHA merge parameter, set to the recorded sync-source head — no
>    branch-name merge, no squash, no rebase. **State plainly: GitLab's
>    merge API pins the source branch's HEAD (`sha` must match the
>    source), but has no equivalent parameter for the target branch's
>    HEAD.** The pre-merge read and the merge call are therefore two
>    separate operations, not one atomic one — a narrow race window
>    exists in which another commit could land on `main` between the
>    read and the merge accepting. This project's GitLab plan tier does
>    not expose merge-train or merged-results pipelines that would close
>    this window (already investigated and recorded in
>    `coordination-publication-rule.md`'s CI-implementation-shape
>    section); do not re-investigate this per-application. The residual
>    risk is bounded, not eliminated, and step 7 below is the mandatory
>    detection-and-remediation control for it — never described as
>    prevention.
> 7. **Post-merge verification is the enforcement point for step 6's
>    race window, not an optional double-check.** Immediately after
>    merge, record the resulting merge commit SHA and its first-parent
>    (target-side) parent SHA. If that parent SHA does not exactly equal
>    the recorded sync-target head, the race in step 6 occurred: `main`
>    advanced between the pre-merge read and the merge accepting, and
>    the merge commit's actual diff was not the diff Gate 2 reviewed.
>    Independently of that check, verify the merge commit's diff against
>    its actual first-parent is limited to exactly the changelog and
>    version-field edits from steps 2-3 — proving no code, test, schema,
>    or other file change slipped in. **Remediation on either check
>    failing:** do not touch the tag or either reconstruction branch (the
>    published release is unaffected — this step is metadata-only, per
>    step 8); open a corrective forward-sync MR from a fresh branch at
>    `main`'s new actual tip, following steps 1-7 again in full, and
>    treat the defective merge commit as superseded evidence, not as a
>    silent partial state.
>
>    **Resolved by binding governor decision (round-seven review
>    `adjp`'s `B-FSR-R7-1`, decided by
>    `20260814T040118Z-governor-reviewer-apkm.md`).** Governor decision
>    `2r8r` requires fail-closed behavior on target drift generally; a
>    planner-architect plan does not have standing authority to narrow
>    that requirement on its own, so this exact question was routed to
>    the governor rather than settled by plan text (see the prior
>    revision's coordination record `spkh`). The governor has now
>    explicitly decided: detect-and-remediate (steps 6-7 above) is
>    accepted as a **narrow, scoped exception** to `2r8r`'s literal
>    fail-closed requirement, **only** for this post-release
>    metadata-forward-sync merge — never described as equivalent to
>    atomic compare-and-swap, and never treated as a precedent for
>    ordinary feature, governance, release-construction, or any other
>    forward-sync merge. The exception holds only while all of `apkm`'s
>    mandatory boundaries remain true, each already reflected above:
>    the sync changes only the governed metadata surfaces (step 3);
>    immediately-before-merge target re-read and exact-source-SHA-only
>    merge (step 6); immediately-after-merge parent-SHA and diff checks
>    (step 7); a mismatch is a proof-invalid merge requiring a fresh
>    corrective MR through the full process, tag and reconstruction refs
>    untouched (step 7's remediation); and human observation of merge
>    activity is optional additional care, never the security property
>    itself (never claimed as such anywhere in this section). **One
>    additional condition `apkm` adds, not previously stated here:** if
>    GitLab later supplies a usable target-bound merge parameter, merge
>    train, or equivalent serialization control, that stronger mechanism
>    must be evaluated before this exception is reused for any future
>    application of the frozen-scope-reconstruction rule — do not assume
>    the platform constraint checked at this application's authoring
>    time still holds at a later one.
> 8. If this synchronization fails or is delayed, the published release
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
> failure, an allowlist commit whose own Gate 2 approval cannot be
> independently re-confirmed at merge time, the target branch's
> protection object differing from its own single constant baseline at
> any of the three checkpoints, the source branch's protection object
> differing from its construction baseline before the one authorized
> construction-to-frozen transition or from its frozen baseline at any
> point after that transition, the source branch reaching Gate 2
> evidence without that transition having occurred, the pre-tag-push
> namespace check finding any non-absence or API ambiguity, the pre-tag
> or post-publication protected-tag-rule re-read finding the rule
> missing, widened, or no longer matching the exact intended tag, or —
> for the metadata-forward-synchronization step — target drift (`main`'s
> tip advancing after the sync-target head is recorded) or source drift
> (the sync branch's head advancing after the sync-source head is
> recorded, post-proof) that is not resolved by that step's required
> refresh before merging, or the post-merge check finding the forward-
> sync merge commit's actual first parent does not match the recorded
> sync-target head (per that step's own remediation, not a
> proceed-anyway judgment call).

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
- **Frozen-out negative proof (structural, per the rule text's
  attribution method — corrected from a curated named list by fresh-
  context review finding `B-FSR-FRESH-2`).** Independently computed in
  a scratch worktree from the exact stable base
  `f1932a1a486b772ca4353c7fa71c2279132fe0dc`:
  - `3456c1fa`'s own diff touches exactly 5 files: `.gitlab-ci.yml`,
    `CHANGELOG.md`, `agf/CHANGELOG.md`,
    `agf/docs/site/reference/release-artifacts.md`,
    `docs/plans/20260805T084100Z-drop-aarch64-linux-required-target.md`.
  - `bcb0fadd`'s first-parent diff touches exactly 7 files: `AGENTS.md`,
    `CHANGELOG.md`, `agf/CHANGELOG.md`,
    `docs/reference/coord-visibility.md`,
    `tooling/rust/crates/agov/src/main.rs`,
    `tooling/rust/crates/agov/tests/coord_visibility.rs`,
    `tooling/rust/crates/agov/tests/coord_write.rs`.
  - The reconstructed tree's `git diff --stat` against the stable base
    (after both cherry-picks, both `CHANGELOG.md`/`agf/CHANGELOG.md`
    conflicts resolved by merging, never discarding, either side) is
    exactly the 10-file union of the two lists above — `.gitlab-ci.yml`,
    `AGENTS.md`, `CHANGELOG.md`, `agf/CHANGELOG.md`,
    `agf/docs/site/reference/release-artifacts.md`, the aarch64-drop plan
    file, `docs/reference/coord-visibility.md`, and the three Rust source/
    test files. Every path is attributable; nothing else appears. This
    also empirically confirms the specific groups `B-FSR-FRESH-2` named
    as unverified — role YAML (`agf/agents/*.yaml`), `MANIFEST.yaml`,
    `agf/VERSION`, documentation/projection/skill paths, and rule-file
    paths beyond the two commits' own diffs — are genuinely absent from
    this reconstruction: by construction, a cherry-pick of a commit's own
    diff cannot introduce a file that diff never touched. This includes
    the maintenance-line release exception content in
    `agf/rules/workflow/release-via-merge-request-rule.md` beyond what
    already existed at the `v2.5.0` stable tag (this new frozen-scope-
    reconstruction section itself is **not** part of v2.6.0's content —
    it lands on `main` only, per this plan's own scope) and the
    `web-ux-reviewer`/`independent-implementation-reviewer` manifest
    entries the earlier curated list named — both are correctly absent
    as a consequence of the structural proof, not because they were
    separately grepped for.
- **Documented mechanical conflict-resolution algorithm (this
  application; the rule text's required "written, mechanical
  conflict-resolution algorithm").** For every `CHANGELOG.md`/
  `agf/CHANGELOG.md` conflict in this ordered replay: take the raw
  content between `<<<<<<< HEAD` and `=======` (`ours`) and between
  `=======` and `>>>>>>>` (`theirs`); replace the entire marker-
  delimited block, markers included, with `theirs` immediately followed
  by `ours`, verbatim, no line added/removed/altered/reordered within
  either side; every byte of the file outside the marker-delimited
  block is left exactly as the cherry-pick's own auto-merge produced
  it, with no other file staged beyond what the cherry-pick itself
  flags as touched.
- **Whole-result content-faithfulness proof, independently re-derived
  twice (round-seven finding `B-FSR-R7-2`, strengthened to whole-result
  scope by round-eight finding `B-FSR-R8-1` after a marker-scoped
  version was shown insufficient — see the coordination note
  accompanying this revision).** Applying the algorithm above from
  scratch in a clean worktree, never consulting any prior round's
  already-resolved files:
  - `3456c1fa`'s cherry-pick conflicts on both changelog files (same
    two files both prior rounds identified); `bcb0fadd`'s cherry-pick
    on top of the resolved result **auto-merges cleanly with zero
    further conflicts** — the entire final tree beyond the two
    algorithmically resolved regions is therefore the cherry-pick
    tool's own unmodified mechanical output, with no manual editing
    anywhere else in the sequence.
  - For each of the two conflicts, the region of the file outside the
    marker-delimited block was diffed byte-for-byte against the
    pre-resolution (raw conflict-marker) file and found identical
    before and after — confirming no non-conflicting-region edit
    occurred, closing `B-FSR-R8-1`'s specific gap.
  - Reproduced independently twice, in two separate fresh worktrees,
    with identical resulting hashes both times (below) — deterministic,
    not a one-off.
- **Deterministic replay proof.** Per the rule text's "either is
  acceptable but the plan must state which": **this plan designates the
  binary base-to-head patch hash as authoritative** (the Git tree hash
  is recorded too, as a cross-check, but the patch hash is what Gate 2
  evidence must reproduce). Computed by running the documented algorithm
  above, fresh, in two independent scratch worktrees during this
  revision — **these values supersede every earlier round's recorded
  hashes**, which used an informal, undocumented resolution rather than
  the now-specified algorithm and are retained nowhere as authoritative:
  - After `git cherry-pick 3456c1fa` + the documented algorithm:
    `CHANGELOG.md` SHA-256 `8cf206e92fbd866a48f8e347523da99b5dbe403bfac28eac527249e866a2d086`,
    `agf/CHANGELOG.md` SHA-256 `2e3e097fcfc0b77401f6c0b03c30c5c3922bc136827543d09a7ebf307715aff8`
    (recorded for auditability; not independently authoritative).
  - After `git cherry-pick -m 1 bcb0fadd` on top (auto-merged cleanly,
    no further manual resolution):
    `CHANGELOG.md` SHA-256 `8cf206e92fbd866a48f8e347523da99b5dbe403bfac28eac527249e866a2d086`
    unchanged by this step for the root file's own conflict region (the
    coord-visibility entries land via clean auto-merge, not a second
    conflict) — the file's overall content still changes (auto-merged
    additions), only its previously-resolved conflict region does not
    require re-resolution.
  - **Authoritative:** final base-to-head binary patch
    (`git diff --binary f1932a1a486b772ca4353c7fa71c2279132fe0dc <final>`)
    SHA-256 `c7db9317f4e4bec130a916e97e54ab33f1f864632041b8b3c40fcb9e0de6147a`.
    Cross-check: final tree hash `fbe16ab183373b461bc641dd192bfe386cf07d1e`.
    Identical on both independent reproductions.
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
10. Both reconstruction branches' protection objects are read fresh from
    the platform API at all three checkpoints (creation, pre-merge,
    post-tag): the target's three readings are identical to each other;
    the source's readings match its construction baseline until the one
    authorized reprotection, then match its frozen baseline thereafter —
    per the two-baseline scheme, not the single-baseline check the
    target uses (`mdth`'s B-FSR-SEC-1, corrected for internal
    consistency by `1jnk`'s B-FSR-SEC-R3-1).
11. The source branch is proven reprotected to `push_access_levels: No
    one` (its frozen baseline) before the reconstruction release MR's
    exact-head Gate 2 evidence is presented, and that MR's merge is
    proven to have used the exact-SHA API call, not a UI click (`mdth`'s
    B-FSR-SEC-2).
12. A second, independent namespace-absence check runs immediately
    before the tag push, bound to the exact merge commit/tree, covering
    every relevant package status (`mdth`'s B-FSR-SEC-3).
13. The governor coordination record authorizing merge names the exact
    source/target SHAs, expected hash, and project identifier before the
    exact-SHA merge call executes (`mdth`'s A-FSR-SEC-1).
14. The rule text contains no plan-local self-reference ("this plan's
    Gate 2") — all timing language names the governed artifact directly
    (`1jnk`'s A-FSR-SEC-R3-1), and the target/source checkpoint
    baselines are two distinct, individually-consistent state machines,
    not one shared equality check the required source transition would
    itself violate (`1jnk`'s B-FSR-SEC-R3-1).
15. The forward-sync MR records its exact sync-target head (`main`'s tip
    at branch creation) before any edits, and its exact sync-source head
    (the branch's head after the changelog/version-field proof and
    pipeline) only once that proof exists — never the same recording
    step, since the required edits between them are not themselves drift
    (`qpj4`'s B-FSR-SEC-FH-1, corrected for the two-boundary distinction
    by `stqp`'s B-FSR-SEC-FH-R2-1). It is merged only via the platform
    API's exact-SHA parameter against a freshly re-read `main` tip
    matching the recorded sync-target head, and any drift in either head
    after its own recording point triggers a refresh (fresh branch,
    diff, proof, pipeline, and exact-head Gate 2 review) rather than a
    proceed-anyway judgment call.
16. The forward-sync merge's inherent non-atomicity (GitLab's merge API
    binds only the source branch's HEAD; no target-branch-HEAD parameter
    exists) is stated plainly, not described as achieving atomic target
    binding, and the post-merge check verifies the actual merge commit's
    first parent against the recorded sync-target head — not merely the
    file-level diff — with a defined remediation (fresh corrective
    forward-sync MR, tag and reconstruction branches untouched) on
    mismatch (`8hn8`'s `B-FSR-FRESH-1`, technical mitigation only —
    criterion 19 below is the actual closure condition).
17. The frozen-out negative proof has three independently checkable
    parts — deterministic replay, path attribution, and whole-result
    content faithfulness — not path attribution alone, and the content-
    faithfulness proof covers the **entire** resulting tree via an
    independent, from-scratch re-derivation using a documented
    mechanical algorithm (never scoped to only conflict-marker regions,
    which a paused cherry-pick can be staged around) (`8hn8`'s
    `B-FSR-FRESH-2`, strengthened by round-seven `adjp`'s `B-FSR-R7-2`,
    strengthened again to whole-result scope by round-eight `qk8s`'s
    `B-FSR-R8-1` after a marker-scoped version was shown insufficient).
18. The exact protected-tag-rule object (not only namespace absence) is
    re-read immediately before the tag push and once more after
    publication, bound to the tag-pipeline evidence (`8hn8`'s
    `A-FSR-FRESH-1`).
19. **Satisfied.** An explicit binding governor decision
    (`20260814T040118Z-governor-reviewer-apkm.md`) accepts
    detect-and-remediate-after-landing (criterion 16) as a narrow,
    scoped exception to `2r8r`'s literal fail-closed-on-target-drift
    requirement, for this post-release metadata-forward-sync merge
    only, conditioned on the mandatory boundaries the plan text now
    cites explicitly, including the platform-reassessment condition for
    any future reuse of this exception (round-seven review `adjp`'s
    `B-FSR-R7-1`, resolved).

## Stop conditions

Stop and return to the governor if: the necessity check's result is
ambiguous or contradicts the recorded evidence; any cherry-pick conflict
cannot be resolved without discarding either side's content; the
deterministic-replay patch/tree hash, or the frozen-out-set proof, fails
at the actual release MR's exact head even though it passed in this
plan's scratch-worktree verification; the merge-result byte-identity
check fails for any allowlisted merge commit; the forward-sync's
governed version-surface list cannot be updated together (no partial
sync); contribution-auditor/infosec-sentinel raise a finding this plan
does not already address; **or the forward-sync merge is about to be
executed and any of `apkm`'s mandatory boundaries (acceptance criterion
19) does not hold at that exact moment** — the governor's decision is
scoped to those boundaries holding, not an unconditional clearance, and
a boundary failing at execution time is itself a stop condition, not a
discretionary call.
