---
id: v270-release-publication
status: pending-gate1
topic: v270-release-publication
advisory_consultation:
  - role: coder-tester
    status: consulted
    reason: >-
      Three independent technical reviews completed by coder-tester
      (model codex-gpt-5 — a different model and session from
      planner-architect), each at that round's exact plan head:
      `20260827T162158Z-coder-tester-rshs.md` (BLOCK, 5 findings, → r2),
      `20260827T170730Z-coder-tester-rn1g.md` (BLOCK, 2+1, → r3), and
      `20260827T180107Z-coder-tester-12ak.md` (BLOCK, 2+3, → this r4).
      Every finding across all three rounds is accepted and resolved,
      none contested (see "Review disposition" below). A fresh bounded
      coder-tester review of THIS r4 head is requested before binding
      Gate 1 — the r4 prose is again planner-architect self-authored.
  - role: contribution-auditor
    status: waived
    reason: >-
      No separate contribution-auditor pass is proposed. The three
      independent coder-tester reviews above (`rshs`, `rn1g`, `12ak`;
      codex-gpt-5) are offered as the substitute independent check for
      this release-construction plan, on
      the same basis the governor accepted for v2.6.0's release plan
      (`kpkk`) and its rule amendment (`33br`): a genuinely independent
      LLM/session re-derivation of the tree-delta, path-attribution,
      byte-identity, and namespace-preflight evidence. The governor
      ratifies or rejects this substitution as part of the Gate 1
      decision; this field records the request, not a granted waiver.
  - role: infosec-sentinel
    status: waived
    reason: >-
      Waiver requested for this exact plan head, not yet granted. Same
      underlying fact established in `33br` and `kpkk`: the configured
      `infosec-sentinel` CI actor's authority is limited to its mechanical
      checks (secret scanning, dependency CVEs, SPDX) and cannot author
      the semantic protected-ref / namespace / merge-race review this
      plan's construction section requires. The governor decides this
      plan-specific semantic-consultation waiver at Gate 1. Configured
      mechanical checks remain mandatory throughout regardless.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: publish AGF-NG v2.7.0-rc.1 via frozen-scope reconstruction

## Scope of this Gate 1 request (revision r4)

**This plan and the Gate 1 decision it requests authorize the release
CANDIDATE only:** construction of `v2.7.0-rc.1`, its publication, its
publication-verification, and the 48-hour verified hold.

**Stable `v2.7.0` promotion is NOT authorized by this plan.** Per the
maintenance-line exception, stable promotion needs (a) a binding governor
record naming the exact accepted-candidate tag object, peeled commit, and
tree — facts that do not exist until an RC is immutably published and
passes its hold — and (b) its own separate Gate 1-approved
release-publication plan. Both are produced *after* an accepted RC
completes the hold. The "Non-authorizing forward preview: stable
promotion" section at the end of this document is orientation for the
governor only; it confers no authority and is not part of the Gate 1
decision requested here.

Requested per handoff `20260827T123825Z-coder-tester-n6ar.md`, downstream
of the governor's binding topology-and-candidate-policy decision
`20260827T124803Z-governor-reviewer-rpam.md` ("yes to both": frozen-scope
reconstruction from `v2.6.1`; RC-first-then-48h-hold candidate policy),
carrying exactly the frozen scope binding decision `8hgm` names: issues
**#86, #87, #88, #90**.

No product implementation, schema, validator semantic, command semantic,
new dependency, or new CI behavior change is in scope — all four frozen
issues (#86, #87, #88, #90) are already Level 4 Validated on `main`.

## Review disposition (r2, r3, then r4)

`rshs` raised five blocking findings against the prior head
(`6cf64345b795574628694174f83c7b10392bfc3c`). All five accepted; none
contested. Resolution in revision r2:

- **B-V270-PUB-R1-1** (RC and stable cannot share one Gate 1) — resolved
  by "Scope of this Gate 1 request" above: this plan is RC-only; the
  stable section is explicitly non-authorizing.
- **B-V270-PUB-R1-2** (frozen-out set not enumerated) — resolved by
  "Frozen-out set (enumerated, snapshot-bound)" below and the committed
  artifact `docs/plans/v270-frozen-out-inventory.txt`.
- **B-V270-PUB-R1-3** (construction sentence targets the protected
  target) — resolved by "Construction: one ordered procedure" below,
  which puts every cherry-pick and metadata commit on the source branch
  only and never on `reconstruct/v2.7`.
- **B-V270-PUB-R1-4** (metadata boundary not exact) — addressed in r2,
  then corrected in r3 (see below): `2.7.0-rc.1` frozen as the Cargo
  value; the exact `Cargo.lock` / `Cargo.toml` occurrences enumerated.
- **B-V270-PUB-R1-5** (frontmatter contradicts the requested role) —
  resolved by the rewritten `advisory_consultation` frontmatter above:
  `coder-tester` is `consulted` (the review happened); the
  contribution-auditor and infosec dispositions record *requests* for
  the governor to decide at Gate 1, not granted waivers.

`rn1g` reviewed r2 (head `d433ca0f83d6d5216d837cd3ccc9ba0e825aea7f`) and
recommended BLOCK with two more blocking findings + one advisory: R1-1,
R1-3, R1-5 confirmed resolved; R1-2 and R1-4 needed correction.
Resolution in revision r3:

- **B-V270-PUB-R2-1** (the r2 Gate 2 invariant is internally impossible —
  it required the final base→merge diff to equal exactly the 21
  functional paths while also requiring release-metadata commits that
  necessarily touch paths inside Section B) — resolved by the
  **two-checkpoint model** in "Frozen-out set" and "Mandatory tree-delta
  proof" below. Checkpoint 1 proves the pre-metadata functional tree is
  exactly 21 paths / tree `caf3de54…`. Checkpoint 2 proves the final RC
  diff is exactly the **43-path union** (21 functional + 26
  release-metadata, 4 shared), with shared- and metadata-path changes
  each bounded to the exact intended edit. The frozen-out path inventory
  (Section B) is regenerated by subtracting the **complete** 43-path
  final allowlist: **615 paths**, `sha256
  58a2757b74b680f08911000426e6b4df2c8c18f5f21ffda8ed7e6625f1808158`.
  Section C of the artifact lists the 43-path union.
- **B-V270-PUB-R2-2** (r2's metadata audit used the wrong tree — the
  `f780ac0e` base instead of the post-cherry-pick tree `caf3de54…` — and
  missed live pins introduced by the allowlist commits) — resolved by
  re-auditing every version surface against tree `caf3de54…`. Corrections:
  root **`AGENTS.md` IS a version-bump target** (the `<!-- AGF: 2.6.1 -->`
  assistant-loader marker was added by `acaec947`; it did not exist at
  the base); **`tooling/rust/Cargo.toml` has four `2.6.1` occurrences**,
  not one (workspace-package version + three path-dependency version
  constraints). See "Release-only change set (candidate) — exact" below.
- **A-V270-PUB-R2-1** (advisory: "both frozen issues" reads as a narrower
  scope) — fixed: "all four frozen issues (#86, #87, #88, #90)".

`12ak` reviewed r3 (head `040cb2af686c0336cc12f08008f71c7e9240f5ed`) and
recommended BLOCK with two more blocking findings + three advisory,
while confirming the two-checkpoint proof and the 26/43-path sets are
structurally correct. Resolution in revision r4:

- **B-V270-PUB-R3-1** (the changelog "move `## [Unreleased]` entries"
  instruction has no source content — neither `CHANGELOG.md` nor
  `agf/CHANGELOG.md` has an `## [Unreleased]` heading or any #86/#87/#88/
  #90 reference on tree `caf3de54…`, and neither cherry-pick touches
  either file) — resolved: item 12 below now **authorizes creation** of
  the new `## [2.7.0-rc.1]` section and enumerates the exact
  issue→bullet mapping for both changelogs.
- **B-V270-PUB-R3-2** (`agf/docs/governance/manifest.md` has **five**
  live `2.6.1` occurrences on tree `caf3de54…`, not three — r3 omitted
  the two publication-status sentences describing v2.6.1 as unpublished
  and v2.5.0 as latest, and left the `to_version` example conditional so
  there is no single deterministic expected diff) — resolved: item 10
  below specifies **one exact manifest outcome**, no discretion.
- **A-V270-PUB-R3-1** (Section B cardinality shorthand; Section C hash
  not published) — fixed: the intersection is stated (27 of the 43
  Section C paths are among the 642 changed paths, so `642 − 27 = 615`),
  and Section C's `sha256`
  `8a865ac36f332c0d83a0fc91c3763a0069af6e8f6c99a68962b85f85796e33e5` is
  now published in the artifact and cited below.
- **A-V270-PUB-R3-2** (frontmatter / scope heading still said "r2") —
  fixed: frontmatter cites all three reviews (`rshs`, `rn1g`, `12ak`);
  scope and disposition headings updated to r4.
- **A-V270-PUB-R3-3** (Checkpoint 1 called the "pre-metadata parent",
  but with multiple metadata commits it is an ancestor) — resolved:
  Construction step 3c records the **exact Checkpoint-1 SHA**, and every
  Gate 2 comparison is against that recorded SHA, not "the parent".

## What is actually implemented, verified against current main

- **#86, #88**: MR !806 implemented `agov-mcp` transport protocol
  correctness. Merged at `960124e1beed7eb7ae381a20c9a176a09b492b5e` after
  exact-head Gate 2 (`20260827T080547Z-governor-reviewer-e5pk.md`).
  Closed (`3h6j`).
- **#87, #90**: MR !2138 implemented assistant-discovery/archive-loader
  setup. Merged at `acaec9472274cebcc32f0d9ade5c92864fd14ccd` after
  exact-head Gate 2 (`20260827T101500Z-governor-reviewer-g2m8.md`).
  Closed (`3h6j`).
- The one deferred item — a fresh Claude Code session confirming
  `.claude/CLAUDE.md`/imported `AGENTS.md` under `/context` Memory files —
  remains partially verified (`20260827T104200Z-planner-architect-lvck.md`:
  file-level wiring confirmed; live `/context` surfacing not confirmable
  from an already-running session). Accepted-and-monitored, not a release
  blocker; the RC's step-7 fresh-clone check is where it finally closes.

**Maturity note (per `implementation-honesty-rule.md`):** #86-#90 are
Level 4 Validated as of this baseline. They reach Level 5 Deployable only
after this plan's own publication-verification steps complete against the
actually-published `v2.7.0-rc.1` tag (and, later, stable `v2.7.0`).

## Eligibility (per the frozen-scope reconstruction exception)

- [x] **Binding governor record naming the frozen scope, exact stable
      base, and why neither the standard nor maintenance-line path
      applies for candidate construction.** Satisfied by `rpam` + `8hgm`:
      frozen issues #86/#87/#88/#90; stable base `v2.6.1` (tag object
      `7e7ad93121be16c3e44a7c12ccab1104efd126ff`, peeled
      `f780ac0ea37061e0ad3ee6fb1f00c6958b996fc0`, tree
      `1c6d2eac56f620ff6b17a6d50dd904427329e1c5`); standard path excluded
      because current `main` is 965 commits / 642 changed paths past
      `v2.6.1` (snapshot `468a16a473e80e0900ff17bf09c15f31009702df`),
      containing substantial unrelated work; maintenance-line path
      excluded for *candidate* construction because no immutable `v2.7.0`
      release candidate exists yet to promote. Independently re-verified:
      `git merge-base --is-ancestor f780ac0e 468a16a4` passes, and
      `agf/VERSION` at `f780ac0e` reads `2.6.1`.
- [x] **Every allowlist commit is a Gate-2-approved exact SHA, or a merge
      commit with a proven byte-identical first-parent patch (case b).**
      Both allowlist entries are case-(b) merge commits. Recomputed
      independently in `rshs` (codex-gpt-5), matching planner-architect's
      own recomputation:
      - `960124e1` (MR !806, #86/#88): parents `c3a94a51` / `98c491d0`;
        `98c491d0` is the exact Gate-2-approved source head (`e5pk`).
        `git merge-base c3a94a51 98c491d0` = `08f8640e`.
        `sha256(git diff --binary 08f8640e 98c491d0)` =
        `a3e31d770bfce5d127cf980bbff0f30392a7b8d6b52dd1df7a85c7e123fd1bd6`
        = `sha256(git diff --binary 960124e1^1 960124e1)`. Byte-identical.
      - `acaec947` (MR !2138, #87/#90): parents `a741fab7` / `26cd74e0`;
        `26cd74e0` is the exact Gate-2-approved source head (`g2m8`).
        `git merge-base a741fab7 26cd74e0` = `c9462dab`.
        `sha256(git diff --binary c9462dab 26cd74e0)` =
        `a58caedf95ec101562fa6cadaad7cca89e0ec32293b2b44cb5764ef7c014529d`
        = `sha256(git diff --binary acaec947^1 acaec947)`. Byte-identical.
- [x] **No conditional prerequisite is claimed for either allowlist
      commit.** Both are unconditional exact-head Gate 2 approvals with no
      aarch64-style prerequisite (unlike v2.6.0's `3456c1fa`). Nothing to
      execute or record under this checklist item.
- [x] **Git tag, GitLab Release, generic-package, and OCI namespaces for
      `v2.7.0-rc.1` independently confirmed absent** — point-in-time
      result from `d7pg` (2026-08-27T12:54:06Z): all four namespaces
      absent, across all generic-package statuses, against freshly
      resolved OCI repository ID `11946773`. **Point-in-time, not a
      reservation** — the mandatory immediately-pre-tag re-check is in
      "Merge, tag, and publication sequence (candidate)" below.

## Frozen-out set (enumerated, snapshot-bound, two-checkpoint)

Per the frozen-scope reconstruction exception, the exact content that
must be **absent** from the reconstructed tree is enumerated, not
replaced with a structural-exclusion argument. `rn1g` (B-V270-PUB-R2-1)
showed the r2 single-invariant form was internally impossible — the
release-metadata step necessarily touches paths that r2 had put in the
"must be absent" set. r3 uses two checkpoints and a complete final
allowlist.

- **Comparison snapshot (frozen):** `origin/main` at
  `468a16a473e80e0900ff17bf09c15f31009702df`, captured 2026-08-27T16:52Z.
  Binding boundary for the frozen-out set. `main` will advance before
  Gate 2; any later advance is *additionally* frozen out and only adds to
  this set. The Checkpoint-2 equality below asserts an exact 43-path set,
  so a later `main` tip cannot introduce a frozen-out path into the
  reconstructed tree without breaking that equality.
- **Excluded commits — Section A of `docs/plans/v270-frozen-out-inventory.txt`:**
  963 commit SHAs = `git rev-list f780ac0e..468a16a4` minus the two
  allowlist merge commits, `LC_ALL=C` sorted.
  `sha256` = `689e9dd27f4d47fac1e61465f2a28791f84ad67478207665dd45bcaff2258b26`.
  (Metadata commits live only on the release source branch, never on
  `main`, so they do not affect Section A.)
- **Final RC allowlist — Section C of the artifact:** the **43** distinct
  paths = the 21 functional reconstruction paths ∪ the 26-path
  release-metadata allowlist ("Release-only change set (candidate) —
  exact" below). 4 paths are shared (in both sets): `AGENTS.md`,
  `agf/ADOPTING.md`, `agf/agents/MANIFEST.yaml`, `agf/templates/AGENTS.md`.
  `sha256` (`LC_ALL=C` sorted) =
  `8a865ac36f332c0d83a0fc91c3763a0069af6e8f6c99a68962b85f85796e33e5`.
- **Excluded paths — Section B of the artifact:** **615** paths. Of the
  43 Section C paths, exactly **27** occur in the 642 base→snapshot
  changed paths (21 functional + 10 metadata, minus the 4 shared counted
  once); the other 16 Section C paths are metadata paths absent from the
  642-path set. So Section B = `git diff --name-only f780ac0e 468a16a4`
  (`LC_ALL=C sort -u`, 642 paths) minus those 27 = **642 − 27 = 615**
  (not `642 − 43`).
  `sha256` = `58a2757b74b680f08911000426e6b4df2c8c18f5f21ffda8ed7e6625f1808158`.
- **Named examples, for orientation only** (the full gate is the artifact,
  not this list): MR !1965's `GIT_DEPTH` fix, MR !2041's
  review-convergence workflow, MR !2119's necessary-scope-expansion rule,
  MR !2122's Pattern 6 reconciliation, MR !2125's net-new-projection-path
  lifecycle (issue #115), MR !2145 (an earlier head of this plan), and
  this plan file / this inventory artifact themselves.

**Gate 2 absence proof (exact, mandatory) — two checkpoints:**

*Checkpoint 1 — pre-metadata functional tree.* After the two
cherry-picks and **before** any metadata commit:
`git diff --name-only f780ac0e <pre-metadata HEAD>` == exactly the 21
functional paths; `HEAD^{tree}` == `caf3de542359e572f43e18be5ef4f92fca236164`;
`sha256(git diff --binary f780ac0e <pre-metadata HEAD>)` ==
`98ab30cdfa029725df4c987f0f735faac051a119f9d06ebb3c9d802763dd3ed7`.

*Checkpoint 2 — final RC tree.* After the metadata commit(s), on the
`reconstruct/v2.7` merge commit:
`git diff --name-only f780ac0e <merge commit>` == exactly the 43-path
Section C set. For each of the 4 shared paths,
`git diff <pre-metadata HEAD> <merge commit> -- <path>` is **only** the
`2.6.1` → `2.7.0-rc.1` substitution (plus, for the narrative surfaces,
only the intended section/heading insert). For each metadata-only path,
the diff is **only** the intended release-metadata edit. `git log
--format=%H f780ac0e..<merge commit>` contains only the two cherry-pick
commits, the metadata commit(s), and the merge commit — no Section A SHA
reachable.

The Checkpoint-2 43-path equality structurally proves none of the 615
Section B paths appear in the reconstructed tree; combined with
Checkpoint 1's tree hash and the bounded per-path edit proofs, the
frozen-out set is proven absent. Any path outside Section C in the
Checkpoint-2 diff, any Section B path present, any Section A commit
reachable, or any shared/metadata-path diff exceeding the intended edit
is a STOP condition (return to governor).

Gate 2 regenerates Sections A, B, and C from `f780ac0e` and the recorded
snapshot and byte-compares against the artifact; any hash mismatch is a
STOP condition.

## Verified construction: cherry-pick onto the stable base

Reproduced independently five times (planner-architect's own run,
`mqjk`, `rshs`, `rn1g`, `12ak`), each in an isolated detached worktree
at the peeled `v2.6.1` commit `f780ac0e`:

1. `git cherry-pick -m 1 960124e1beed7eb7ae381a20c9a176a09b492b5e`:
   auto-merges cleanly, **zero conflicts** (one auto-merge note on
   `tooling/rust/crates/agov-core/src/roi.rs`, no manual resolution).
2. `git cherry-pick -m 1 acaec9472274cebcc32f0d9ade5c92864fd14ccd` on
   top: auto-merges cleanly, **zero conflicts** (auto-merge notes on
   `AGENTS.md`, `agf/learnings/foundry.yaml`, `agf/templates/AGENTS.md`;
   no manual resolution).
3. Resulting tree: `caf3de542359e572f43e18be5ef4f92fca236164` — identical
   across all five independent executions.
4. Base-to-head binary patch:
   `sha256(git diff --binary f780ac0e HEAD)` =
   `98ab30cdfa029725df4c987f0f735faac051a119f9d06ebb3c9d802763dd3ed7`.

Because both cherry-picks apply with **zero conflicts**, the
"whole-result content faithfulness" proof required by the rule is
structurally simpler than v2.6.0's case: there is no manually-resolved
region for an independent party to re-derive. The entire reconstructed
tree beyond the base is the cherry-pick tool's own unmodified mechanical
output, confirmed by tree-hash reproducibility across five independent
executions.

## Construction: one ordered procedure

Exactly this order. `reconstruct/v2.7` is the protected target and is
**never constructed on**; every cherry-pick and metadata commit lands on
the source branch `release-v2.7.0-rc.1` only.

1. Create target `reconstruct/v2.7` at exactly `f780ac0e`. Immediately
   protect it: `push_access_levels: No one` (level 0),
   `merge_access_levels: Maintainers` (level 40),
   `allow_force_push: false`, no deletion. Read the full protection
   object via the platform API and record it (checkpoint 1 of 3).
2. Create source `release-v2.7.0-rc.1` at exactly `f780ac0e` (the same
   base commit). Record its construction-baseline protection (level 40
   push access is acceptable during construction) and its resolved head
   SHA.
3. On `release-v2.7.0-rc.1` only:
   a. `git cherry-pick -m 1 960124e1beed7eb7ae381a20c9a176a09b492b5e`
   b. `git cherry-pick -m 1 acaec9472274cebcc32f0d9ade5c92864fd14ccd`
   (this exact order; reversing it is untested and not authorized)
   c. **record the exact Checkpoint-1 commit SHA here** — the
      `release-v2.7.0-rc.1` head immediately after step 3b and before any
      metadata edit — and its evidence (21-path diff, tree
      `caf3de542359e572f43e18be5ef4f92fca236164`, patch hash
      `98ab30cd…`). Every Gate 2 Checkpoint-1 comparison is against this
      recorded SHA, never against "the merge commit's parent" (with more
      than one metadata commit the Checkpoint-1 commit is an ancestor,
      not the direct parent).
   d. one or more release-only metadata commit(s) per the 26-path
      "Release-only change set (candidate) — exact" allowlist below.
      (A single squashed metadata commit is acceptable and makes the
      Checkpoint-1 commit the direct parent; multiple are allowed as
      long as the recorded Checkpoint-1 SHA is used for every proof.)
4. Freeze the source: reprotect `release-v2.7.0-rc.1` to the same
   `No one` / `Maintainers` / no-force-push posture as the target. Record
   this one-time construction→frozen transition as the source's frozen
   baseline, alongside both branches' resolved head SHAs. This is
   required Gate 2 evidence.
5. Open the release MR: non-squash, non-rebase, from
   `release-v2.7.0-rc.1` to `reconstruct/v2.7`. Its diff against the
   target is exactly the 43-path Section C set (the two cherry-picks'
   cumulative 21-path diff plus the 26-path metadata allowlist, 4
   shared). Retain both branches after merge and after tagging.
6. Create the `v2.7.0-rc.1` protected-tag rule (exact match, creation
   restricted to level 40) before the tag itself is created.

## Mandatory tree-delta proof — Checkpoint 1 (pre-metadata, three parts)

Computed on the reconstructed tree **before** the metadata change set
(i.e. on the two-cherry-pick result), then re-verified at Gate 2 against
the **recorded Checkpoint-1 SHA** (Construction step 3c) — not "the
merge commit's parent", which is only the direct parent when there is
exactly one metadata commit. Checkpoint 2 (the final RC tree, = 43-path
Section C set with bounded per-path edits) is specified in "Frozen-out
set" above and re-verified at the same Gate 2.

1. **Deterministic replay proof.** Authoritative value: base-to-head
   binary patch hash
   `98ab30cdfa029725df4c987f0f735faac051a119f9d06ebb3c9d802763dd3ed7`
   (cross-check tree `caf3de542359e572f43e18be5ef4f92fca236164`).
   Reproduced independently five times (planner-architect, `mqjk`,
   `rshs`, `rn1g`, `12ak`).
2. **Path attribution — exactly 21 files.** `960124e1`'s own diff touches
   5: `agf/docs/site/docs/adopt/mcp-server.md`,
   `docs/plans/agov-mcp-transport-protocol-correctness.md`,
   `tooling/rust/crates/agov-core/src/roi.rs`,
   `tooling/rust/crates/agov-mcp/src/main.rs`,
   `tooling/rust/crates/agov-mcp/tests/stdio.rs`. `acaec947`'s own diff
   touches 17: `AGENTS.md`, `agf/ADOPTING.md`,
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
   union to exactly **21 files**. The reconstructed tree's
   `git diff --stat` against `f780ac0e` lists exactly these 21 — no
   unattributed path.
3. **Whole-result content faithfulness.** Not applicable in the
   "manually resolved conflict region" sense — there were none. The
   entire result is the cherry-pick tool's own mechanical output,
   reproducible byte-for-byte (tree hash `caf3de54...`) across three
   independent executions from the same base and same two named commits.

## Protected target and source (evidence checkpoints)

- **Target `reconstruct/v2.7`:** single constant protection baseline
  (`No one` / `Maintainers` / no-force-push / no-deletion). Protection
  object read and recorded at three checkpoints — immediately after
  creation, immediately before the release MR merges, immediately after
  RC tag publication — all three byte-identical. Any difference
  invalidates the exception.
- **Source `release-v2.7.0-rc.1`:** two baselines — the construction
  baseline (recorded at creation, level 40 push acceptable) and the
  frozen baseline (recorded at the one-time reprotection in Construction
  step 4). After the transition, the source's fail-closed comparison is
  against its frozen baseline; the recorded transition is the only
  authorized difference.
- **Release MR:** any change to source head, target head, merge base, or
  either allowlist commit's Gate 2 approval invalidates prior review.
- **Immediately-pre-merge:** re-read both branch heads via the platform
  API; each must equal the exact SHAs the binding governor Gate 2 record
  names. Merge via the platform merge API with `sha` = the exact
  reviewed source head, `squash=false`, source-branch retention
  requested.
- **Immediately-post-merge:** verify the merge commit's first parent =
  reviewed target SHA, second parent = reviewed source SHA, resulting
  tree OID = the reviewed source HEAD's tree OID named in the governor
  record. Mismatch = fail-closed before any tag/release/package/OCI
  action; mark the target proof-invalid and return to the governor.

## Release-only change set (candidate) — exact

**This plan file and `docs/plans/v270-frozen-out-inventory.txt` are out
of the reconstructed tree** — neither existed at the `v2.6.1` cut point,
and neither is in the 43-path Section C allowlist. Adding either to
`release-v2.7.0-rc.1` would create an unattributed 44th/45th path and
break Checkpoint 2. Their Gate 1 metadata lives on this MR and `main`,
exactly like every other plan file — never on the reconstruction
branches.

Apply the following on `release-v2.7.0-rc.1`, after both cherry-picks.
**Every occurrence below was audited against the post-cherry-pick tree
`caf3de542359e572f43e18be5ef4f92fca236164` (`git grep -nF "2.6.1"` on
that tree), not against the `f780ac0e` base** — `rn1g` (B-V270-PUB-R2-2)
showed the base audit misses markers the allowlist commits introduced.
Line numbers are on tree `caf3de54…` and are indicative; the edit is
keyed to the string, not the line. This is the **26-path
release-metadata allowlist** (Section C minus the 17 functional-only
paths). 4 of the 26 are shared with the functional set and are marked
**[shared]**.

**Mechanical version-string edits (`2.6.1` → `2.7.0-rc.1`):**

1. **`agf/VERSION`** — whole file (1 occ).
2. **`AGENTS.md` [shared]** — the assistant-loader marker
   `<!-- AGF: 2.6.1 — … -->` at line 1 (1 occ). Introduced by
   `acaec947`; absent at the base. Leaving it stale while `agf/VERSION`
   bumps violates the assistant-loader equality contract
   (`assistant_loader.rs`). The rest of `AGENTS.md`'s `acaec947` content
   is carried unchanged; this is the only edit to it.
3. **`agf/templates/AGENTS.md` [shared]** — the `<!-- AGF: 2.6.1 — … -->`
   marker at line 3 (1 occ).
4. **`agf/ADOPTING.md` [shared]** — `framework_version: "2.6.1"` in the
   adopter example block (line ~333, 1 occ). Matches prior-release
   practice of keeping the example current with `agf/VERSION`.
5. **`agf/agents/MANIFEST.yaml` [shared]** — the header comment
   `# Framework version: 2.6.1  |  Layout: B` (line ~11, 1 occ).
6. **`agf/templates/learning-record.yaml`** — `framework_version:
   "2.6.1"` (line ~8, 1 occ).
7. **The fourteen `agf/agents/*.yaml` role files** — `academic-peer-
   reviewer`, `academic-visual-reviewer`, `build-engineer`,
   `capability-developer`, `coder-tester`, `contribution-auditor`,
   `deployment-engineer`, `documentation-writer`, `fact-checker`,
   `governor-reviewer`, `independent-implementation-reviewer`,
   `infosec-sentinel`, `planner-architect`, `web-ux-reviewer` — the
   `framework_version:` field only (1 occ each). Do **not** touch each
   file's own role-definition `version:` field. All fourteen exist on
   the reconstructed tree.
8. **`tooling/rust/Cargo.toml`** — **four** occurrences (`rn1g`
   B-V270-PUB-R2-2): `[workspace.package] version = "2.6.1"` (line ~14)
   **and** the three path-dependency version constraints
   `agov-core = { path = "crates/agov-core", version = "2.6.1" }`,
   `agov-coord = { … version = "2.6.1" }`,
   `agov-validators = { … version = "2.6.1" }` (lines ~20-22). All four
   → `2.7.0-rc.1`.
9. **`tooling/rust/Cargo.lock`** — the five workspace-crate `[[package]]`
   entries whose `version = "2.6.1"`: `agov`, `agov-coord`, `agov-core`,
   `agov-mcp`, `agov-validators` (lines ~13/35/47/60/70). Each →
   `2.7.0-rc.1`. **These five are the only `2.6.1` package versions in
   `Cargo.lock`.** Note line ~1179 carries `version = "2.7.0"` for an
   unrelated third-party crate — **do not touch it** (a blind
   search-replace of `2.7.0` would corrupt the lock). Confirm with
   `cargo metadata` / a lock-consistency check in validation; no
   dependency-graph change is intended or permitted.

**Structured version cells — `agf/docs/governance/manifest.md` (one exact
outcome, no discretion; `rn1g` B-R3-2 / `12ak` B-R3-2). Five live `2.6.1`
occurrences on tree `caf3de54…`:**

10. Apply exactly these five edits and nothing else in this file:
    - `| Version | 2.6.1 |` (line ~11) → `| Version | 2.7.0-rc.1 |`.
    - `` `VERSION` | Current version string (`2.6.1`) | Reference `` (line
      ~40) → `` Current version string (`2.7.0-rc.1`) ``.
    - The publication-status paragraph (lines ~17-23), currently
      "AGF v2.6.1 is being prepared as an independently packaged release,
      correcting v2.6.0's incomplete publication (its tag and OCI image
      exist, but its GitLab Release and generic package were never
      created). Until v2.6.1's publication is verified, adopters should
      install the latest fully published release (`v2.5.0`) from GitLab
      rather than treating this release-candidate tree as published." →
      replace verbatim with: "AGF v2.7.0-rc.1 is a release candidate
      under a 48-hour verified hold. Until stable `v2.7.0` is published,
      adopters should install the latest fully published stable release
      (`v2.6.1`) from GitLab rather than treating this release-candidate
      tree as published." (This both removes the two now-stale `2.6.1`
      occurrences and states the candidate-accurate status.)
    - `to_version: "2.6.1"` inside the `upgrade_history` example block
      (line ~78) → `to_version: "2.7.0-rc.1"`. **Bound, not conditional:**
      prior RC practice (`v2.5.0-rc.2` set every such example to the
      artifact version) is followed; the example tracks the artifact
      version. No implementation-time discretion.

**Narrative edits (not mechanical string substitution):**

11. **`agf/docs/site/roadmap.md`** — identify `v2.7.0-rc.1` as the
    candidate under verification, following the exact pattern `v2.6.1`'s
    own roadmap entry used for its predecessor candidate. Preserve all
    historical release evidence unchanged. (This file is also changed on
    `main` since the base; the edit is to the reconstructed-tree /
    `v2.6.1` version, never to `main`'s.)
12. **`CHANGELOG.md`** and **`agf/CHANGELOG.md`** — **create** (not move)
    a new section `## [2.7.0-rc.1] - <release date>` immediately above the
    existing `## [2.6.1]` section. `rn1g` / `12ak` (B-R3-1) confirmed:
    on tree `caf3de54…` neither changelog has a `## [Unreleased]` heading
    or any #86/#87/#88/#90 reference, and neither cherry-pick touches
    either changelog — so there is nothing to "move"; the section is
    authored fresh. Both files get the **same bullets** (root
    `CHANGELOG.md` repo-level phrasing, `agf/CHANGELOG.md` adopter-facing
    phrasing, matching how the `## [2.6.1]` sections already mirror each
    other). Exact issue→bullet mapping — no other bullets:

    Header line: **Release candidate.** Frozen-scope reconstruction from
    `v2.6.1` carrying exactly issues #86, #87, #88, #90, per
    `docs/plans/20260827T160105Z-v270-release-publication.md`.

    ### Fixed
    - **agov-mcp JSON-RPC envelope validation (#88, MR !806).** The
      `agov-mcp` stdio server previously accepted requests with a missing
      or wrong `jsonrpc` version and still dispatched the method; it now
      validates the JSON-RPC 2.0 envelope and rejects a malformed request
      with the correct error response instead of executing it.

    ### Added
    - **agov-mcp coordination-subcommand coverage for cold start (#86,
      MR !806).** The MCP server now exposes the `agov coord` subcommands
      needed for cold-start and visibility that were previously missing
      (7 of the 10), so an assistant driving AGF entirely through
      `agov-mcp` can complete a cold start and visibility scan without
      shelling out to the CLI.
    - **`.claude/CLAUDE.md` loader in this repository (#87, MR !2138).**
      Adds the Claude Code loader file so Claude Code sessions pick up
      `AGENTS.md`'s operational guidance; `agov`'s assistant-loader
      validation and the release self-sufficiency check are extended to
      cover it. Adopter guidance:
      `agf/docs/site/docs/adopt/assistant-loader-configuration.md`.
    - **Ready-to-copy Claude `.mcp.json` template in tooling archives
      (#90, MR !2138).** Packages `examples/claude/.mcp.json` in the
      `agov` tooling archives and documents it in the adopter
      assistant-loader / MCP-server guides, so adopters can copy a
      working MCP client configuration rather than hand-writing one.

    Use the release branch's actual test/build counts wherever the
    prior sections quote them. Preserve every existing historical
    section (`## [2.6.1]` and earlier) byte-for-byte. Both files are also
    changed on `main` since the base; the edit is to the
    reconstructed-tree version, never to `main`'s.

**Explicitly NOT edited for version** (verified `git grep -cF "2.6.1"` =
0 on tree `caf3de54…`, or the `2.6.1` present is not a current-version
marker):

- `agf/agents/MANIFEST.example.yaml`, `agf/templates/CLAUDE.md`,
  `.claude/CLAUDE.md`, `agf/project-specific-examples/python-flask/CLAUDE.md`,
  `examples/claude/.mcp.json` — no version marker; carried through with
  their `acaec947` content unchanged.
- `tooling/rust/crates/agov/src/dist.rs` (`write(…, "2.6.1\n")`,
  `"<!-- AGF: 2.6.1 -->\n"` at lines ~932/936) and
  `tooling/rust/crates/agov-validators/src/assistant_loader.rs`
  (`<!-- AGF: 2.1.6 -->` / `2.2.0` fixtures) — **test fixture strings**,
  not release surfaces. Bumping them would break their tests. In the
  21-path functional delta, carried unchanged.
- `agf/learnings/foundry.yaml` (`framework_version: "2.0.0-provisional"`)
  and `agf/learnings/harbor-workshop.yaml` (`"2.5.0"`) — historical
  per-record values; learning records keep their as-authored
  `framework_version`. `foundry.yaml` is in the functional delta
  (`acaec947` appended a record); carried unchanged.
- `agents-govern/**` — the bootstrap reference distribution (all
  `1.1.5x`); never touched by an AGF release.
- historical `docs/coordination/**` and `docs/plans/**` prose mentioning
  `v2.6.1` — identical to the base; not touched.

No product implementation, command semantic, schema, validator semantic,
new dependency, or new CI behavior change is in scope beyond what
#86-#90 already merged.

## Validation before Gate 2 (candidate)

Exact-head evidence for, at minimum: **Checkpoint 1** (pre-metadata
tree == 21 paths / tree `caf3de54…` / patch hash `98ab30cd…`) **and
Checkpoint 2** (final RC diff == the 43-path Section C set, each shared /
metadata path's edit bounded to the intended change) from "Frozen-out
set" and "Mandatory tree-delta proof" above; full version-surface audit
against the change set (every listed surface reads `2.7.0-rc.1`,
including all four `Cargo.toml` occurrences and the `AGENTS.md` /
`agf/templates/AGENTS.md` `<!-- AGF: -->` markers; the
explicitly-not-edited files unchanged); the **assistant-loader equality
check** (`agov validate` / `assistant_loader.rs` — `AGENTS.md`'s marker
must match `agf/VERSION`); `Cargo.lock` consistency / no dependency-graph
change (and the unrelated `2.7.0` third-party entry untouched);
`cargo test --manifest-path tooling/rust/Cargo.toml` (workspace); strict
Clippy, warnings denied; `agov validate` (including adopter-docs); `agov
dist validate` and clean archive builds; built `agov` / `agov-mcp`
self-reporting `2.7.0-rc.1`; tag/version guard evidence with
`CI_COMMIT_TAG=v2.7.0-rc.1`; strict `mkdocs build`; a GitLab API check
that the release MR's `closes_issues` response is empty (issues #86-#90
already closed per `3h6j` — this MR must not re-trigger closure);
release-path, provenance, artifact-exclusion, SPDX/license, and `git diff
--check` checks; Section A/B/C regeneration and byte-compare against the
artifact; the protection-object evidence at all recorded checkpoints; a
green exact-head MR pipeline; and the immediately-pre-merge re-read of
both branch heads.

## Merge, tag, and publication sequence (candidate)

1. Merge only the exact Gate-2-approved release MR head into
   `reconstruct/v2.7`, per the pre/post-merge parent/tree-OID checks in
   "Protected target and source" above. Retain `release-v2.7.0-rc.1`.
2. **Second, independent, immediately-pre-tag namespace check** for
   `v2.7.0-rc.1` across Git tag, GitLab Release, generic package (all
   statuses — default, hidden, processing, error, pending_destruction,
   deprecated — fully paginated), and OCI tag (repository ID resolved
   fresh, not cached). Any non-absence or API ambiguity stops the
   release. Re-read the `v2.7.0-rc.1` protected-tag-rule object at the
   same checkpoint.
3. Create one annotated `v2.7.0-rc.1` tag on the exact `reconstruct/v2.7`
   merge commit; push immediately. Never move, delete, or recreate it.
4. Require the tag pipeline to pass: framework and Rust tooling archives,
   provenance / checksums, OCI publication, GitLab Release creation.
5. Run `agov dist post-publication-check` against the anonymous GitLab
   Release API and a fresh download; verify all downloaded bytes,
   provenance, and executable self-reporting `2.7.0-rc.1`.
6. Verify the GitLab Release, generic package `agf/v2.7.0-rc.1`, and OCI
   tag `agov:2.7.0-rc.1` correspond to the immutable tag pipeline.
   Re-read the protected-tag-rule object once more post-publication.
   Record all four resulting immutable identities/digests as tag-pipeline
   evidence.
7. From a fresh clone at the exact immutable `v2.7.0-rc.1` tag: confirm
   the public documentation site builds strictly; confirm #86's/#88's
   `agov-mcp` transport behavior and #87's/#90's assistant-discovery /
   archive-loader behavior match their documented contracts; and — the
   one item this plan can uniquely verify from a genuinely fresh session
   — a **new** Claude Code session opened against this exact tagged
   checkout runs `/context` and confirms `.claude/CLAUDE.md` and imported
   `AGENTS.md` content appear under Memory files, closing `7bkm`'s
   deferred live-surface check (`lvck` could not, from a running
   session).

## 48-hour verified hold

Begin only after every check in steps 4-7 passes. During the hold: no
further mutation of `reconstruct/v2.7`, `release-v2.7.0-rc.1`, or the
`v2.7.0-rc.1` tag. If a relevant defect surfaces, govern the correction
on a fresh `release-v2.7.0-rc.2` branch from the same `reconstruct/v2.7`
target, publish immutable `v2.7.0-rc.2`, and restart the 48-hour hold —
never move, delete, or replace the `-rc.1` tag or artifacts. Record the
hold's start and end timestamps in a coordination completion record; the
hold is calendar time, not a countdown the governor actively watches.

**On hold completion:** planner-architect produces (a) a binding-decision
request naming the exact accepted-candidate tag object, peeled commit,
and tree, and (b) a separate Gate 1 release-publication plan for the
stable `v2.7.0` maintenance-line promotion. Neither exists yet; neither
is authorized by this plan.

## Staffing and ownership

Candidate construction (branches, cherry-picks, metadata edits,
validation evidence): coder-tester, per this session's established
division of Rust / release-mechanics labor. Gate 1 and Gate 2 decisions:
governor-reviewer (human). Post-hold stable promotion: re-claimed fresh
after the hold passes under its own plan — the candidate's claim does not
silently extend to promotion.

## Acceptance criteria (candidate only)

1. The candidate release MR is release-only, targets `reconstruct/v2.7`,
   and receives exact-head Gate 2 approval.
2. All version surfaces and built binaries agree on `2.7.0-rc.1`,
   including all four `tooling/rust/Cargo.toml` occurrences and the
   `AGENTS.md` / `agf/templates/AGENTS.md` `<!-- AGF: -->` markers; the
   explicitly-not-edited files (incl. the `dist.rs` /
   `assistant_loader.rs` test fixtures and the unrelated `Cargo.lock`
   `2.7.0` third-party entry) are unchanged; the assistant-loader
   equality check passes.
3. The immutable `v2.7.0-rc.1` tag points to the exact `reconstruct/v2.7`
   merge commit, with post-merge parent / tree-OID proof matching the
   reviewed target / source / metadata-frozen source.
4. The RC tag pipeline and anonymous downloaded-asset validation pass.
5. Published `agov-mcp` (#86/#88) and assistant-discovery / archive-loader
   (#87/#90) behavior on the tagged RC matches documented contracts,
   including a genuinely fresh Claude Code session's `/context` check.
6. The frozen-out set (Section A: 963 commits, `689e9dd2…`; Section B:
   615 paths, `58a2757b…`; Section C: 43 paths, `8a865ac3…`; in
   `docs/plans/v270-frozen-out-inventory.txt`) is proven absent from the
   tagged RC's tree by the exact Checkpoint-2 43-path (Section C)
   equality plus Checkpoint-1's tree hash and the bounded per-path edit
   proofs — not by enumeration alone and not by a structural argument
   alone.
7. Issues #86-#90 remain closed throughout — no double-close, no
   premature reopening.
8. The 48-hour hold completes with no unresolved defect (or a clean
   `-rc.2` cycle is run).

## Stop conditions

Return to the governor if: release preparation requires any product,
schema, or CI-behavior change beyond what #86-#90 merged; a dependency /
checksum / `Cargo.lock` graph change (the five workspace-crate version
bumps are metadata, not a graph change); weakening any existing check;
changing an existing tag; **Checkpoint 1 not equal to exactly the 21
functional paths / tree `caf3de54…`**; **Checkpoint 2 not equal to
exactly the 43-path Section C set, or any shared / metadata path's edit
exceeding the intended change**; any Section A commit reachable from the
reconstructed merge commit; any Section B path present in the
Checkpoint-2 diff; a Section A/B/C regeneration hash mismatch; deleting a
source or target branch; any stable-base, protected-target, or
protected-source drift at any checkpoint; a path-attribution proof
failure; the assistant-loader equality check failing; a pre-tag or
post-publication namespace / protected-tag-rule re-read finding any
non-absence, mismatch, or API ambiguity; the release-merge post-merge
parent / tree checks finding a mismatch; or the 48-hour hold surfacing a
defect not cleanly correctable via a fresh `-rc.2` without touching the
accepted `-rc.1` artifacts.

---

## Non-authorizing forward preview: stable promotion (NOT part of this Gate 1)

*Orientation for the governor only. Confers no authority. A separate
Gate 1-approved plan and a separate binding exact-candidate decision
record are required before any of this begins, produced after an RC
passes its 48-hour hold. Included so the governor can see the full
trajectory when deciding the candidate step.*

Expected shape, following `v2.5.0`'s promotion from `v2.5.0-rc.2`
(`docs/plans/20260809T194404Z-v250-final-release-publication.md`):

- Begin from the accepted RC's immutable anchors only; re-derive tag
  object, peeled commit, and tree from the canonical remote; stop on any
  mismatch.
- Fail-closed namespace + protection preflight for `v2.7.0` specifically
  (all four namespaces; plus absence of `release/2.7` and `release-2.7.0`
  branches). Create `release/2.7` at the exact accepted-RC commit,
  level-40 push/merge, no force-push, no deletion; API re-read. Create
  `release-2.7.0` at the same commit, same protection. Create the exact
  `v2.7.0` protected-tag rule (level 40).
- Exact metadata boundary: `agf/VERSION` `2.7.0-rc.1` → `2.7.0`; both
  changelogs' RC heading → `## [2.7.0] - <release date>` (fold in any
  hold findings, or leave otherwise unchanged if the hold produced none);
  the same version-surface list from the candidate change set, each →
  plain `2.7.0`. Zero functional / schema / doc-content / migration /
  test / dependency / CI / artifact-layout change from the accepted RC.
- Release MR: non-squash, non-rebase, `release-2.7.0` → `release/2.7`;
  exact-head Gate 2; immediately-pre-merge re-read of both heads.
- Tag + publish `v2.7.0` on the exact `release/2.7` merge commit after a
  fresh immediately-pre-tag namespace + protected-tag-rule re-check; tag
  pipeline; post-publication verification; re-confirm GitLab Release,
  generic package `agf/v2.7.0`, OCI tag `agov:2.7.0`.
- Metadata-forward synchronization to `main` per the merged rule's
  standard procedure (content-preserving; `main`'s code / tests /
  non-version-metadata untouched): split `main`'s `## [Unreleased]` so
  the matching entries move into `## [2.7.0] - <release date>`; update
  the version-surface list to `2.7.0` as a last-released-version marker
  (not a claim about `main`'s next release number); prove the sync MR
  diff metadata-only; exact-SHA merge with pre/post sync-target drift
  checks per `apkm`'s narrow accepted detect-and-remediate exception.
- Publish + merge a validated completion record marking `v2.7.0` as the
  new latest published stable release before any further release treats
  it as baseline.
