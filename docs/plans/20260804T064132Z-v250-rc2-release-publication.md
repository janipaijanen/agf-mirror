---
id: v250-rc2-release-publication
status: pending-gate1
topic: v250-rc2-release-publication
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Codex recommended cutting a bounded rc.2
      (`20260804T002719Z-contribution-auditor-rnp5.md`, to governor-reviewer)
      and commissioned this plan
      (`20260804T002720Z-contribution-auditor-73kj.md`). A separate
      handoff (`20260804T003638Z-contribution-auditor-qbgs.md`) proposed a
      two-lane release-bake boundary and asked for an independent
      challenge before this plan incorporates it; that challenge is filed
      as `20260804T012713Z-planner-architect-jqs5.md` and is referenced,
      not restated, in "Relationship to the CI-latency plans" below.
      Requesting fresh Gate 1 review of this plan itself.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Release routing, version, and changelog metadata only, over content
      (the NASA-attribution acknowledgements page) already independently
      fact-checked at its own Gate 2
      (`20260803T234841Z-fact-checker-sppt.md`) and Gate 1-approved
      (`20260803T233430Z-governor-reviewer-ts77.md`). No new credential,
      dependency, network-trust-boundary, or authorization-semantic change
      is introduced by this plan. Matches
      `v250-rc1-release-publication.md`'s own disposition for the same
      reason.
---

# Plan: publish `v2.5.0-rc.2`

## Decision requested

Authorize the release-only steps that publish the current `main` delta
since `v2.5.0-rc.1` as `v2.5.0-rc.2`, per
`docs/plans/v250-statement-of-work.md`'s own rc.2 trigger rule ("any
functional, schema, documentation, migration, test, or artifact-layout
correction after rc.1 creates a new RC") and
`v250-rc1-release-publication.md`'s own rc.2/rc.3 trigger text
("`v2.5.0` final ships the same functional candidate ... unless a
correction forces `rc.2`").

This is not discretionary. One documentation surface shipped in the
framework archive changed since `v2.5.0-rc.1` tagged — see "Scope: the
audited `rc.1..main` delta" below — which is exactly the SoW's own rc.2
trigger condition, independently re-derived here rather than assumed
from `rnp5`'s recommendation.

Current baseline: `origin/main` at the commit resolved when this plan's
own Gate 1 metadata is updated (re-run the delta audit below at that
exact head — this revision's own audit is anchored to
`c15186d5d9fbef69c1444ff0eeb52aa0b6d806b4`, per `B-RC2-AUDIT-ANCHOR-8`;
that anchor, not any commit/file count, is the reproducible fact — the
counts below apply only at that exact SHA and must be recomputed at
whatever head Gate 1 actually reviews). Latest published release:
`v2.5.0-rc.1`, tag `623e782b22b5e17e277c8a50ba6d79bedacfe3e6`, tagged
2026-08-03T19:13:28Z. `agf/VERSION` currently reads `2.5.0-rc.1`. No
`v2.5.0-rc.2` tag exists (`git ls-remote --tags origin 'v2.5.0-rc.2'`,
empty), no GitLab Release exists (Releases API, 404), no generic
package `agf/v2.5.0-rc.2` exists (Packages API, empty), and no OCI tag
`agov:2.5.0-rc.2` exists (OCI tags-list API, absent) — corrects
`B-RC2-PREFLIGHT-7`: `git tag -l` and `glab release list` cannot prove
the generic-package or OCI claims; all four namespaces must be checked
via their own APIs, and rechecked immediately before release execution
since availability can change.

This plan authorizes only: release metadata, a version-only bump across
every current version surface, upgrade/overlay validation, the release
MR, one immutable annotated tag after the release MR merges, and
publication verification. No product implementation, new command
semantic, schema field, or validator behavior is authorized here —
`agf/docs/site/acknowledgements.md` and its two companion changes were
already merged, Gate 1-approved
(`20260803T233430Z-governor-reviewer-ts77.md`), and Gate 2-approved
(**corrects `B-RC2-AUTHORITY-4`**: the binding Gate 2 authority is
governor decision `20260804T000744Z-governor-reviewer-k2bk.md` at exact
head `f062883e7e84b2cef47734c4a87fa69b74e19f8d`, which orders the
merge; fact-checker's `20260803T234841Z-fact-checker-sppt.md` CLEAR
verdict is the independent evidence `k2bk` relies on, not itself a
Gate 2 approval — advisory roles do not hold binding gate authority) in
a separately governed implementation MR (!1105). This plan is what
carries that content to Level 5 Deployable, via the publication path
itself, per `implementation-honesty-rule.md`'s five-level model.

## Strongest counterargument, and why RC2 wins anyway

**Adds `B-RC2-COUNTERARGUMENT-9`**, per `73kj`'s requirement to state
the strongest direct-stable alternative and why candidate identity
prevails — the prior revision asserted RC2 is "not discretionary" without
presenting the alternative it was rejecting.

**The alternative:** the NASA-attribution delta is documentation-only,
already independently fact-checked (`sppt`), and its own implementation
MR (!1105) already built and validated the distribution as part of its
own Gate 2 pipeline. A stable-publication plan could explicitly amend
the SoW's candidate-identity rule for this one case and promote `main`
directly to `v2.5.0` final, skipping `rc.2` — repeating the same
validation this content already passed once, without a second release
ceremony.

**Rejected.** Doing so would make the published `v2.5.0-rc.1` tag stop
identifying the actual final candidate — a reader comparing the RC1 tag
to the eventual stable tag would find undocumented content that never
went through its own RC checkpoint. It contradicts two already-approved
plans (`v250-statement-of-work.md`'s rc.2 trigger rule and
`v250-rc1-release-publication.md`'s own rc.2/rc.3 trigger text), both of
which state the rule in advance rather than after the fact this time
happens to be inconvenient. The savings are real but small — one release
ceremony — against a real cost: an amended-after-the-fact candidate-
identity rule invites the same shortcut for the next documentation
change, and the next, until "RC checkpoint" stops meaning anything
verifiable. Candidate identity prevails.

## Scope: the audited `rc.1..main` delta

**Corrects `B-RC2-AUDIT-ANCHOR-8`** — the prior revision's counts (49
commits, 52 coordination files) were already stale by the time
`contribution-auditor` reviewed it (independently reproduced as 50/50 at
that point), and are stale again now. This revision anchors the audit to
an exact SHA rather than presenting a moving count as fixed: at
`origin/main` commit `c15186d5d9fbef69c1444ff0eeb52aa0b6d806b4`,
`git log v2.5.0-rc.1..c15186d5 --no-merges` shows 60 commits, and
`git diff --name-only v2.5.0-rc.1 c15186d5` shows 62 changed files.
Cross-checked file-by-file against `agf/dist/release-manifest.yaml`'s
actual `include`/`exclude`/`forbidden` globs — the only authoritative
answer to "does this ship in the archive" — and against the shipped-path
filter specifically (`git diff --name-only v2.5.0-rc.1 c15186d5 |
grep '^agf/'`), which returns exactly the same four `agf/` paths as the
original draft found: no new shipped surface has appeared between
drafting and this correction. **This anchor is not the final audit** —
re-run both commands at whatever exact head Gate 1 actually approves,
per the Gate 1 metadata sequencing below.

| Path | Ships in framework archive? | Disposition |
|---|---|---|
| `agf/docs/build/mkdocs.yml` | Yes — matches `agf/docs/build/**` | Nav entry for the new acknowledgements page. Part of MR !1105. |
| `agf/docs/site/acknowledgements.md` | Yes — matches `agf/docs/site/**` | New page. Part of MR !1105. |
| `agf/docs/site/docs/concepts/evidence-and-learnings.md` | Yes — matches `agf/docs/site/**` | Cross-link addition. Part of MR !1105. |
| `agf/learnings/foundry.yaml` (multiple commits) | **No** — `agf/learnings/**` is not in the `include` glob at all, per `v250-rc1-release-publication.md`'s own confirmed precedent for this exact path | No release-content impact. |
| `docs/coordination/*.md` (52 files) | No — repo-root `docs/`, not `agf/` | No release-content impact. This is the ordinary coordination-channel activity of drafting, reviewing, and fixing MR !1090 and this plan's own commissioning thread — not release content. |
| `docs/plans/nasa-llis-influence-attribution.md` | No — repo-root `docs/plans/`, not in any `include` glob | No release-content impact (the plan document itself; its authorized *implementation* is the three `agf/` paths above). |
| `docs/sources.md` | No — repo-root `docs/`, not `agf/` | No release-content impact. |

**One governed change set across three shipped paths changed** (corrects
`A-RC2-WORDING-2` — "exactly one shipped surface" read as one file, not
one governed unit): MR !1105's acknowledgements-page addition. No
scope-reconciliation decision is needed (unlike
`v250-rc1-release-publication.md`'s three-surface amendment) — there is
exactly one candidate change set, it is unambiguous, and it is already
Gate 1-approved (`20260803T233430Z-governor-reviewer-ts77.md`) and Gate
2-approved (`k2bk`, evidenced by `sppt`'s CLEAR fact-check — see the
correction above). This plan does not reopen that approval; it only
carries the already-approved content to publication.

**Explicitly excluded from this candidate, per `73kj`'s commissioning
instruction and independently confirmed correct:**

- **MR !1090 (`gate1_approved_by` structural approver-field hygiene,
  Path A).** A nonblocking validator diagnostic, unrelated to any
  contract #79/#80/RC1 already shipped, that changes tooling behavior
  after the RC1 checkpoint. Including it would widen this candidate for
  an unrelated governance enhancement. Deferred to `v2.5.1` per
  `20260804T002719Z-contribution-auditor-rnp5.md` and
  `20260804T012713Z-planner-architect-jqs5.md` (§ answers to `phpp`,
  point 2) — both independently reach the same conclusion.
- **CI bootstrap/validation-latency optimization**
  (`20260803T233746Z-build-engineer-h2dg.md`) and the wrapper-fingerprint
  rebuild-loop defect surfaced in issue #53's live poll evidence. Neither
  is required by any shipped feature in this candidate.
- **The three already-Gate-1-approved CI-latency plans**
  (`ci-pipeline-deduplication.md`, `first-push-ci-changes-fallback.md`,
  `interruptible-superseded-pipelines.md`). See "Relationship to the
  CI-latency plans" below — excluded from this candidate's *content*
  regardless of their merge timing, since none of them touch a shipped
  path.
- **Visual-refresh, radar, and swatch-audit work**
  (`docs/plans/20260803T224324Z-...` is a plan file, not shipped; the
  on-hold `pages-readme-visual-refresh-v251` and `swatch-post250-palette`
  items are explicitly deferred in their own plan files already).

## SemVer classification (independently verified)

The one in-scope surface adds a new documentation page and two small
cross-link/nav additions. No schema, command, validator, or
functional-code change:

- `agf/docs/site/acknowledgements.md`: new page. No existing page's
  content is removed or redefined.
- `agf/docs/build/mkdocs.yml`: one new nav entry. Existing nav entries
  unchanged.
- `agf/docs/site/docs/concepts/evidence-and-learnings.md`: one new
  cross-reference section added. Existing content unchanged.

**Additive minor, documentation-only. `v2.5.0-rc.2` is the correct
target** — a corrective/additive documentation change after `rc.1`,
matching the SoW's own rc.2 definition exactly, not a new feature
requiring its own SemVer justification beyond what `rc.1`'s own
classification already established for the release as a whole.

## Gate 1 metadata sequencing

Following `v250-rc1-release-publication.md`'s corrected sequencing
(itself a fix for MR !1010's stale-metadata failure mode): when the
governor approves Gate 1, this plan's own frontmatter (`status`,
`gate1_approved_by`, `gate1_approved_at`) is updated **on this plan's
own MR**, validated, and merged to `main` as part of that same review
cycle — before any release branch is cut. The release branch is cut
from `main` only after the approved plan (with correct metadata already
on `main`) exists.

**Process failure and correction, recorded here rather than silently
fixed.** This plan's first revision (MR !1123) was merged to `main`
still carrying `status: pending-gate1` — two minutes after
`contribution-auditor`'s BLOCK review (`3r3a`) posted, and before any
Gate 1 approval existed. This directly violated the sequencing this
section itself specifies. The merge was planner-architect's own error
(batch-merging this MR alongside genuinely safe coordination-only fast-
path MRs, without checking that this one still required Gate 1 first)
— not a tooling defect, and not something the reviewer or governor
caused. **Merging a plan is not the same as approving it**: no release
branch, tag, or version file was touched as a result, and no fact this
plan asserts changes because the file happens to already be on `main`.
This revision fixes the nine blocking and two advisory findings from
`3r3a` on a fresh branch; it does **not** set `status: approved` or any
`gate1_approved_by`/`gate1_approved_at` field itself — that remains
exclusively the governor's own action, taken only after renewed review
confirms this correction, per the sequencing above.

## Relationship to the CI-latency plans

**Corrects `B-RC2-CI-BOUNDARY-5`.** `jqs5` argued the three CI-latency
plans are safe to merge before *stable* (not just before RC2) because
`.gitlab-ci.yml` isn't a shipped path and every tag-only job is left
unchanged. `contribution-auditor`'s `3r3a` review credits that as "the
strongest credible case" but does not accept it: `.gitlab-ci.yml` lives
inside the immutable source tag and defines the acceptance machinery
that builds, tests, and publishes the release itself — merging it
between RC2 and stable would make the two tags run different
test/pipeline machinery even if the packaged archive bytes were
otherwise identical, which is a real difference `jqs5`'s
"does-it-ship" framing didn't account for.

**This plan adopts the conservative freeze as its default boundary**,
per `3r3a`'s proposed compromise:

- The three CI-latency plans' own implementation MRs do **not** merge to
  `main` between this plan's Gate 1 approval and stable's publication,
  unless a reproduced release blocker specifically requires one of them.
- CI-latency analysis, local patch preparation, and static validation
  (CI Lint, expanded-configuration checks) may continue on isolated
  branches in the meantime.
- No deliberate race/cancellation/noise test from those plans' own Gate 2
  evidence requirements runs concurrently with either RC2's or stable's
  bake window.
- The measured opportunity cost (real, per issue #53's own evidence) is
  accepted as a short release freeze — smaller than the cost of
  ambiguous RC2/stable acceptance-machinery evidence or an unplanned
  RC3.

This is the default this plan operates under; it is not irreversible.
The governor may explicitly override it — but this plan does not leave
an implicit path for an unrecorded CI merge between tags. Any override
must be its own governor decision-record naming which CI-latency plan(s)
it applies to and why, and this plan's own `rc.3` trigger below still
applies if the override changes anything about the shipped candidate.

## Release-only change set

Create the release commit on `release-2.5.0-rc.2` (per
`release-via-merge-request-rule.md`'s `release-X.Y.Z` form), based on
the then-current `main` (with this plan's approved metadata already
merged), with only these release-truth changes:

1. Set `agf/VERSION` to `2.5.0-rc.2`.
2. Add dated `[2.5.0-rc.2]` entries to `CHANGELOG.md` and
   `agf/CHANGELOG.md`. **Neither file currently has an `[Unreleased]`
   section or any NASA-attribution entry** (verified directly — both
   files go straight from their title/preamble to the existing
   `[2.5.0-rc.1]` entry) — this release-only change set drafts the
   entry fresh, it does not convert existing `[Unreleased]` prose. See
   "Proposed changelog substance" below.
3. Update the Rust workspace version: `tooling/rust/Cargo.toml`'s
   `[workspace.package] version` (currently `2.5.0-rc.1`) and its three
   internal path-dependency pins (`agov-core`, `agov-coord`,
   `agov-validators`), and the corresponding `Cargo.lock` entries. No
   Rust source change — this candidate ships no tooling-binary behavior
   change at all (the one shipped surface is documentation-only), so
   this is a pure version-string bump with zero functional Rust diff,
   even smaller in scope than rc.1's own "version bump only" Rust
   change.
4. Update current framework-version surfaces from `2.5.0-rc.1` to
   `2.5.0-rc.2`: all twelve `agf/agents/*.yaml` role definitions
   (confirmed via `grep -l framework_version agf/agents/*.yaml`:
   `academic-peer-reviewer.yaml`, `academic-visual-reviewer.yaml`,
   `build-engineer.yaml`, `capability-developer.yaml`,
   `coder-tester.yaml`, `contribution-auditor.yaml`,
   `deployment-engineer.yaml`, `documentation-writer.yaml`,
   `fact-checker.yaml`, `governor-reviewer.yaml`, `infosec-sentinel.yaml`,
   `planner-architect.yaml`), **`agf/templates/learning-record.yaml`**
   (corrects `B-RC2-VERSION-SURFACE-6` — omitted from the first
   revision; ships under `agf/templates/**`, currently
   `framework_version: "2.5.0-rc.1"`; leave historical `2.5.0-rc.1`
   evidence references inside `agf/learnings/foundry.yaml` unchanged —
   those are history, not a current-version field), the `# Framework
   version:` comment in `agf/agents/MANIFEST.yaml` (this project's own
   roster — not shipped, kept current for dogfooding honesty per rc.1's
   precedent), and current version statements in `agf/ADOPTING.md`,
   `agf/templates/AGENTS.md`, and `agf/docs/governance/manifest.md`. A
   full current-version scan (`grep -r "2.5.0-rc.1" agf/`) is required
   before Gate 2 to catch any further surface neither this plan nor its
   review named.
5. Update `agf/docs/site/roadmap.md`: change "Latest release:
   `v2.4.11`" (stale since rc.1 published — rc.1 is a release candidate,
   not yet the latest *stable*, so this line's correctness depends on
   the page's own convention for how it describes an in-flight RC;
   confirm the actual current wording at implementation time rather
   than assuming it still says `v2.4.11` unchanged) to reflect
   `v2.5.0-rc.2` as the current release-candidate entry, preserving the
   `rc.1` history line rather than overwriting it.
6. Leave the README major/minor citation unchanged unless the actual
   current wording at implementation time warrants a bump per existing
   convention (same deferred-verification approach as rc.1 item 6).

No product implementation, command semantic, schema, validator
semantic, dependency, CI behavior, new documentation policy, rule, or
milestone/tracker change is in scope — rc.1 already resolved #33/#67/#78
disposition; this plan does not reopen that.

## Proposed changelog substance

Both `CHANGELOG.md` and `agf/CHANGELOG.md` need a fresh `[2.5.0-rc.2]`
section (verify at the release-branch head, don't assume this draft's
wording is final):

- State plainly this is an **RC checkpoint, not final** — same pattern
  as the `rc.1` entry — and that `v2.5.0` final ships the same
  functional candidate as this candidate unless a further correction
  forces `rc.3`.
- **SemVer classification: Additive minor, documentation-only.** One new
  page (`agf/docs/site/acknowledgements.md`) crediting NASA's Lessons
  Learned Information System (LLIS) and other named design influences
  behind specific shipped AGF surfaces (the learning-record
  `supersedes`/`status` fields; the `downstream-impact-rule.md`
  classification model). No schema, command, or validator behavior
  changes.
- Per **A-RC-MATURITY-1**'s pattern from `rc.1`: describe the shipped
  content as "merged and Level 4 Validated," reaching Level 5 Deployable
  only once this publication path succeeds through post-publication
  verification below.
- Use the actual test/build counts from the final release branch run,
  not assumed-unchanged counts from `rc.1`.

## Validation before Gate 2

The release MR must provide exact-head evidence for:

- Cargo metadata showing exactly the workspace package at `2.5.0-rc.2`
  and the three internal path requirements at `^2.5.0-rc.2`;
- `cargo test --manifest-path tooling/rust/Cargo.toml --workspace`, full
  pass/fail/ignored counts recorded in changelog and MR evidence (no
  Rust source changed, so counts are expected unchanged from `rc.1`'s
  own run — record the actual counts to confirm, not assume);
- strict workspace Clippy with warnings denied;
- `agov validate`, including adopter-docs validation;
- `agov dist validate` and clean tar/zip builds;
- **archive-content verification specific to this release's own
  change**: a fresh framework archive (`agf-2.5.0-rc.2.tar.gz`/`.zip`)
  extraction confirms `agf/docs/site/acknowledgements.md` is present,
  reachable from the built site nav, and all twelve
  `agf/agents/*.yaml` role definitions plus
  `agf/templates/learning-record.yaml` report `framework_version:
  "2.5.0-rc.2"`;
- all shipped archive self-sufficiency tests for both archive formats;
- **framework archive overlay, from all three governed published
  baselines** (**corrects `B-RC2-MATRIX-1`** — the prior revision
  reduced this to a single `rc.1` baseline; `73kj` requires repeating
  RC1's exact matrix without a weaker substitute, and `rnp5` names
  three-baseline overlays explicitly): `v2.4.5`, `v2.4.8`, and
  `v2.4.11` — the governed baseline set per
  `20260803T183732Z-governor-reviewer-rntt.md` (`v2.4.8` is the binding
  replacement for unpublished `v2.4.6`; do not fabricate a synthetic
  `v2.4.6` archive). For each baseline: a plain `agf-2.5.0-rc.2`
  framework-archive extraction over a scratch adopter project seeded
  from that baseline must leave a divergent adopter
  `agf/agents/MANIFEST.yaml` byte-for-byte unchanged, and must show the
  updated `acknowledgements.md`/nav content. An additional `rc.1`
  overlay may be included as a fourth check but does not replace any of
  the three;
- **platform tooling archive install and CLI/MCP behavior parity**
  (**corrects `B-RC2-PARITY-2`** — the prior revision reduced this to
  binary self-reporting only; `73kj` and `rnp5` require packaged CLI/MCP
  parity, not identity alone): confirm the packaged `agov` and
  `agov-mcp` binaries self-report `2.5.0-rc.2`, the release MR exact
  head, and clean source; **and**, carrying forward `rc.1`'s own
  fixtures updated only for the RC2 version/head, confirm CLI `agov
  coord pending` and MCP tool `agov_coord_pending` return equivalent
  output for the same fixture, and CLI `agov validate learnings
  <fixture-path>` accepts a positive `status`/`supersedes` fixture and
  rejects a malformed one while MCP tool `agov_validate` (`check:
  "learnings"`) returns equivalent accept/reject results for the same
  two fixtures — no new command shipped in this candidate, but the
  existing commands' CLI/MCP parity is re-verified at the new version,
  not merely asserted unchanged;
- `tooling/rust/scripts/check-live-tooling-surfaces.sh check` and
  `selftest`;
- tag/version guard evidence with `CI_COMMIT_TAG=v2.5.0-rc.2`;
- strict `mkdocs build` and link checks over the current site content,
  specifically confirming the new acknowledgements page builds cleanly
  and its nav entry resolves;
- **bounded external documentation-review classification** (**adds
  `B-RC2-EXTERNAL-3`**, per `rnp5`'s minimum-evidence list): classify
  every external (non-project-internal) documentation-review comment
  received on the acknowledgements-page content, as of the release MR's
  exact head, as either a reproducible released-surface blocker or a
  post-2.5.0 enhancement suggestion. As of this plan's drafting, no
  external documentation-review comment has been received for this
  candidate — state that explicitly with the checked cutoff, rather
  than omitting the step; if any arrive before the release branch is
  cut, classify them here before Gate 2. Only a reproducible blocker may
  alter this candidate; an enhancement suggestion routes to 2.5.1 and
  does not create `rc.3`;
- release-path, provenance, artifact-exclusion, SPDX/license, links, and
  `git diff --check` checks; and
- a green exact-head GitLab MR pipeline, confirmed to be the only
  pipeline for that SHA (no duplicate push/MR pipeline pair, per
  `ci-pipeline-deduplication.md`'s already-shipped guard).

The release MR must target `main`, contain only the governed
release-only change set, retain its source branch, and receive
exact-head Gate 2 approval.

## Merge, tag, and publication sequence

1. Merge only the exact Gate 2-approved release MR head and retain its
   source branch — explicit `should_remove_source_branch: false` through
   the merge API, verified immediately after merge, per
   `v250-rc1-release-publication.md`'s corrected branch-retention
   discipline (A-RC-BRANCH-RETENTION-R3).
2. Confirm `origin/main` resolves to the resulting merge commit and that
   no `v2.5.0-rc.2` tag exists locally or remotely.
3. Create one annotated `v2.5.0-rc.2` tag on that exact `main` merge
   commit and push it immediately. Never move, delete, or recreate the
   tag.
4. Require the tag pipeline to pass, including framework archives, Linux
   Rust tooling archives, provenance/checksum validation, OCI
   publication, and GitLab Release creation. Ordinary GitLab Release
   (`upcoming_release: false`), matching the `rc.1`/`v2.2.0-rc.1`
   precedent — no new pre-release CI behavior.
5. Run `agov dist post-publication-check` against the anonymous GitLab
   Release API and a fresh download directory.
6. Verify the GitLab Release, generic package `agf/v2.5.0-rc.2`, and OCI
   tag `agov:2.5.0-rc.2` correspond to the immutable tag pipeline.
7. From a fresh git clone checked out at the exact immutable
   `v2.5.0-rc.2` tag, run both artifact checks from "Validation before
   Gate 2" above against the real published archive bytes, not just the
   release-branch working tree.
8. Publish and merge a validated completion coordination record.

## rc.3 trigger

Any functional, schema, documentation, migration, test, or
artifact-layout correction discovered after `v2.5.0-rc.2` publishes
creates a new `v2.5.0-rc.3` candidate and repeats the full exact-artifact
acceptance matrix above. Do not schedule a ceremonial rc.3. If `rc.2` is
clean through publication verification, `v2.5.0` final ships the same
functional candidate under its own later, separately Gate 1-approved
release-publication plan, limited to version and changelog metadata.

## Acceptance criteria

1. This plan's own Gate 1 metadata is updated and merged to `main`
   before the release branch is cut, per the sequencing above.
2. The release MR uses branch name `release-2.5.0-rc.2`, is
   release-only, and receives exact-head Gate 2 approval.
3. All current version surfaces — including all twelve
   `agf/agents/*.yaml` role definitions and built binary self-reports —
   agree on `2.5.0-rc.2`.
4. The immutable tag points to the exact release-MR merge commit on
   `main`.
5. The tag pipeline and anonymous downloaded-asset validation pass; the
   GitLab Release is ordinary (`upcoming_release: false`).
6. Fresh archive extraction proves `agf/docs/site/acknowledgements.md`
   is present in the framework archive and reachable from the built
   site nav.
7. Framework-archive overlay test passes from the `v2.5.0-rc.1`
   baseline.
8. `v2.5.0` final's own future plan can promote this exact functional
   candidate with only release-metadata changes, or this plan's own
   observation surfaces a correction requiring `rc.3` instead.

## Stop conditions

Stop and return to the governor if release preparation requires: a
product or CI behavior change beyond the version bump; a dependency/
checksum change; any new command semantic; weakening any existing
check; changing an existing tag; using a release branch name other than
`release-2.5.0-rc.2` without an explicit rule amendment; merging MR
!1090, any CI-latency plan's implementation, or any other surface not
named in "Scope" above into this release branch; deleting a source
branch; or discovering a `main` delta surface not accounted for in the
scope table above (in which case: stop, classify it the same way, and
return to the governor rather than silently including or excluding it).
