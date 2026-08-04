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
exact head — the count in this draft, 49 commits, is a measurement at
drafting time, not a frozen constant). Latest published release:
`v2.5.0-rc.1`, tag `623e782b22b5e17e277c8a50ba6d79bedacfe3e6`, tagged
2026-08-03T19:13:28Z. `agf/VERSION` currently reads `2.5.0-rc.1`. No
`v2.5.0-rc.2` tag, GitLab Release, or generic package exists yet
(confirmed via `git tag -l` and `glab release list`).

This plan authorizes only: release metadata, a version-only bump across
every current version surface, upgrade/overlay validation, the release
MR, one immutable annotated tag after the release MR merges, and
publication verification. No product implementation, new command
semantic, schema field, or validator behavior is authorized here —
`agf/docs/site/acknowledgements.md` and its two companion changes were
already merged and independently Gate 2-approved
(`20260803T234841Z-fact-checker-sppt.md`, CLEAR) in a separately
governed implementation MR (!1105). This plan is what carries that
content to Level 5 Deployable, via the publication path itself, per
`implementation-honesty-rule.md`'s five-level model.

## Scope: the audited `rc.1..main` delta

Independently audited (`git log v2.5.0-rc.1..origin/main --no-merges`,
49 commits at drafting time; `git diff --name-only v2.5.0-rc.1
origin/main`), cross-checked file-by-file against
`agf/dist/release-manifest.yaml`'s actual `include`/`exclude`/`forbidden`
globs — the only authoritative answer to "does this ship in the
archive":

| Path | Ships in framework archive? | Disposition |
|---|---|---|
| `agf/docs/build/mkdocs.yml` | Yes — matches `agf/docs/build/**` | Nav entry for the new acknowledgements page. Part of MR !1105. |
| `agf/docs/site/acknowledgements.md` | Yes — matches `agf/docs/site/**` | New page. Part of MR !1105. |
| `agf/docs/site/docs/concepts/evidence-and-learnings.md` | Yes — matches `agf/docs/site/**` | Cross-link addition. Part of MR !1105. |
| `agf/learnings/foundry.yaml` (multiple commits) | **No** — `agf/learnings/**` is not in the `include` glob at all, per `v250-rc1-release-publication.md`'s own confirmed precedent for this exact path | No release-content impact. |
| `docs/coordination/*.md` (52 files) | No — repo-root `docs/`, not `agf/` | No release-content impact. This is the ordinary coordination-channel activity of drafting, reviewing, and fixing MR !1090 and this plan's own commissioning thread — not release content. |
| `docs/plans/nasa-llis-influence-attribution.md` | No — repo-root `docs/plans/`, not in any `include` glob | No release-content impact (the plan document itself; its authorized *implementation* is the three `agf/` paths above). |
| `docs/sources.md` | No — repo-root `docs/`, not `agf/` | No release-content impact. |

**Exactly one shipped surface changed: MR !1105's three-file
acknowledgements-page addition.** No scope-reconciliation decision is
needed (unlike `v250-rc1-release-publication.md`'s three-surface
amendment) — there is exactly one candidate, it is unambiguous, and it
is already Gate 1-approved (`20260803T233430Z-governor-reviewer-ts77.md`)
and Gate 2-approved via `sppt`'s CLEAR fact-check. This plan does not
reopen that approval; it only carries the already-approved content to
publication.

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

## Relationship to the CI-latency plans

`73kj` extends this commission with a request (via `qbgs`) to
"incorporate the agreed boundary" for parallel work into this plan.
**No boundary is agreed yet** — `jqs5` filed an independent challenge to
Codex's proposed two-lane partition and that challenge has not yet
received a response. This plan does not treat either position as
settled; it records both, unresolved, so the governor can decide
without this plan silently picking a side:

- **Codex's `qbgs` position:** CI/validator/wrapper/MCP/packaging/
  archive/release-process implementation changes wait until *stable*
  ships, unless required to fix a demonstrated release blocker.
- **This plan's author's position (`jqs5`):** the three CI-latency plans
  touch only `.gitlab-ci.yml`, which is not a shipped path in
  `release-manifest.yaml` at all, and each plan's own risk analysis
  independently verifies every tag-only/release job is left unchanged —
  so they are safe to merge to `main` before *stable*, specifically in
  the window after this rc.2 tags and its observation window closes,
  not concurrently with either rc.2's or stable's own bake.

**What this plan does regardless of which position prevails:** nothing.
This plan's own release-only change set (below) touches no CI-latency
surface. Whichever position the governor adopts, it does not change
this plan's scope, evidence, or sequencing — it only affects when the
three CI-latency plans' own, separate implementation MRs are permitted
to merge. That decision belongs in a governor decision-record referencing
`qbgs`/`jqs5`, not in this plan's own text.

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
   `planner-architect.yaml`), the `# Framework version:` comment in
   `agf/agents/MANIFEST.yaml` (this project's own roster — not shipped,
   kept current for dogfooding honesty per rc.1's precedent), and
   current version statements in `agf/ADOPTING.md`,
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
  `agf/agents/*.yaml` role definitions report `framework_version:
  "2.5.0-rc.2"`;
- all shipped archive self-sufficiency tests for both archive formats;
- **framework archive overlay, from `v2.5.0-rc.1` as the baseline** (the
  immediately prior release — no product/tooling behavior changed since
  rc.1, so a single-baseline overlay test is sufficient here, unlike
  rc.1's own three-baseline requirement, which existed because rc.1
  shipped genuine new adopter-facing behavior that needed testing from
  multiple starting points): a plain `agf-2.5.0-rc.2` framework-archive
  extraction over a scratch adopter project seeded from `v2.5.0-rc.1`
  must leave a divergent adopter `agf/agents/MANIFEST.yaml`
  byte-for-byte unchanged;
- **platform tooling archive install**: confirm the packaged `agov` and
  `agov-mcp` binaries self-report `2.5.0-rc.2`, the release MR exact
  head, and clean source — no new command or behavior to verify, since
  none shipped;
- `tooling/rust/scripts/check-live-tooling-surfaces.sh check` and
  `selftest`;
- tag/version guard evidence with `CI_COMMIT_TAG=v2.5.0-rc.2`;
- strict `mkdocs build` and link checks over the current site content,
  specifically confirming the new acknowledgements page builds cleanly
  and its nav entry resolves;
- release-path, provenance, artifact-exclusion, SPDX/license, links, and
  `git diff --check` checks; and
- a green exact-head GitLab MR pipeline.

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
