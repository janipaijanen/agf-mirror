---
id: v250-rc1-release-publication
status: pending-gate1
topic: v250-rc1-release-publication
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Round 1 review (`20260803T135943Z-contribution-auditor-gbkp.md` +
      addendum `20260803T140917Z-contribution-auditor-qprt.md`) at exact
      head `9ae18ca4`: BLOCK, seven blocking findings (B-RC-SCOPE-1,
      B-RC-GATE-METADATA-2, B-RC-VERSION-SURFACES-3,
      B-RC-ARTIFACT-MATRIX-4, B-RC-PRERELEASE-5, B-RC-BRANCH-6,
      B-RC-TRACKER-7) plus two advisories (A-RC-MATURITY-1,
      A-RC-ROADMAP-2). All addressed in this revision: the
      scope-reconciliation table now includes the governance-layout-rule
      fix and issue #101's cache-topology command (both previously
      omitted or wrongly dismissed) under one combined scope-amendment
      decision; Gate 1 metadata now updates on this plan's own MR before
      any release branch is cut; all twelve `agf/agents/*.yaml` role
      definitions are named in the version-bump scope; the
      upgrade/overlay validation is split into framework-archive overlay
      (schemas/templates/rules, no binary) versus platform-tooling-archive
      CLI/MCP parity (the actual binaries), each tested separately;
      the GitLab-pre-release authorization is withdrawn in favor of the
      verified ordinary-Release precedent; the release branch name is
      corrected to `release-2.5.0-rc.1` (no `v`); the v2.5.0 milestone's
      open #33/#67/#78 now get an explicit per-issue disposition step
      before the release is called feature-complete; and maturity language
      throughout now distinguishes "merged, Level 4 Validated" from
      Level 5 Deployable. A bounded renewed review is requested before
      this is treated as Gate 1-ready.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Release routing, version, and changelog metadata only, over content
      already individually assessed at its own implementation Gate
      (#79/#80's infosec consultation in `v250-rc1-contract-core.md`; the
      learning-supersession plan's own consultation). No new credential,
      dependency, network-trust-boundary, or authorization-semantic change
      is introduced by this plan.
---

# Plan: publish v2.5.0-rc.1

## Decision requested

Authorize the release-only steps that publish the current `main` delta
since `v2.4.11` as `v2.5.0-rc.1`, per
`docs/plans/v250-statement-of-work.md`'s release train and
`docs/plans/post-v248-release-batching-amendment.md`'s batching scope — with
one explicit scope-reconciliation decision requested below, since the
actual `main` delta is broader than the batching amendment's original
"authority and acknowledgement contract core" framing.

Current baseline: `origin/main` at `ca5a56d68bbce222c77ec6568d0e8ac2a830d478`
(refreshed after
contribution-auditor's round-1 review `gbkp`/`qprt`; the only commits
between the prior `f77d80d2` baseline and this one are this plan's own
review coordination records plus MR !1050, a repo-internal Gate 1 metadata
sync for the already-merged historical-recovery plan — confirmed via
`git log f77d80d2..ca5a56d6 --oneline --no-merges`, no new shipped surface).
Latest published release: `v2.4.11`. `agf/VERSION` currently reads
`2.4.11`. At planning time there is no `v2.5.0-rc.1` tag, GitLab Release, or
generic package (preflight-confirmed by `qprt`).

This plan authorizes only: the scope decision below, release metadata, a
version-only Rust bump, upgrade/overlay validation, the release MR, one
immutable annotated tag after the release MR merges, publication
verification, and issue #79/#80 closure-link confirmation (both issues are
already closed on GitLab; this plan does not reopen or re-close them, it
records the release that ships their implementation). No product
implementation, new command semantic, schema field, or validator behavior
is authorized here — every one of those was merged and Level 4 Validated
(per `implementation-honesty-rule.md`) in a separately Gate 2-approved
implementation MR, listed below. This plan is what carries each to Level 5
Deployable, via the publication path itself.

## Scope reconciliation: the actual `main` delta since v2.4.11

Independently audited (`git log v2.4.11..origin/main --no-merges`, 121
commits at audit time, reclassified by what they touch and cross-checked
against `agf/dist/release-manifest.yaml`'s actual `include`/`exclude`
globs — the only authoritative answer to "does this ship in the archive").

| Surface | Ships in `agf-{version}` archive? | In original batching-amendment scope? | Disposition |
|---|---|---|---|
| **#79** — upgrade-safe adopter role authority | Yes (`agf/agents/**` glob; `MANIFEST.yaml` explicitly excluded, `MANIFEST.example.yaml` ships) | Yes, named explicitly | Ship as planned. |
| **#80** — `requires-ack` obligation semantics | Yes (`agf/schemas/**`; new `agov coord pending` command ships in the Rust tooling archive) | Yes, named explicitly | Ship as planned. |
| **Learning-record `status`/`supersedes`** (`docs/plans/learning-supersession-status-fields.md`, Path B, MR !1025, merged `e6dbc0a3`) | Yes (`agf/schemas/**`, `agf/templates/**`) | **No** | **Governor scope-amendment decision requested — see below.** |
| **`governance-file-layout-rule.md` correction** (both `.claude/rules/` and `agf/rules/` copies, reclassified Additive per Codex MR !916, 2026-08-02) | Yes — `agf/rules/**` is an `include` glob. **This was omitted from the original audit entirely; found by contribution-auditor's review (`gbkp`, B-RC-SCOPE-1), not by me.** | No | **Governor scope-amendment decision requested — see below.** |
| **Issue #101 — CI cache-policy Slice A/B** (`agov ci cache-topology-check` command) | Yes — the compiled command necessarily enters every Rust tooling binary built from current `main`, regardless of the feature's internal purpose. The CI config/script changes that consume it (`.gitlab-ci.yml`, `.gitlab/scripts/`) do not ship — those are `forbidden` paths. **Original audit wrongly concluded "no scope decision needed" because the *purpose* is internal; contribution-auditor correctly points out a compiled command doesn't disappear from the product archive because of its purpose.** | No | **Governor scope-amendment decision requested — see below.** |
| `docs/plans/historical-branch-only-coordination-recovery.md` + its inventory (MR !1010, merged `f77d80d2`) | No — `docs/plans/` and `docs/reviews/` (except the two individually-included review docs already in the manifest) are not in the `include` glob. | No, and this is repository-internal governance process, not a product surface. | No release-content impact. No changelog entry needed — no other internal plan-drafting commit gets one either. |
| MR !1050 (Gate 1 metadata sync for the already-merged historical-recovery plan) | No — plan-file frontmatter edit only, not in any `include` glob. | N/A | No release-content impact. |
| Source registry / radar tracking (`docs/sources.md`, `docs/radar.md`, `docs/plans/radar-governance-signal-gaming.md`, `docs/TODO.md`) | No — repo-root `docs/`, not `agf/`. | No. | No release-content impact (confirmed against the manifest). Currently has zero coverage in either changelog's `[Unreleased]` section; this is a `CHANGELOG.md` (repository-level) completeness question, not a release-artifact question — flagged as a non-blocking observation, not a stop condition, since repo-root `CHANGELOG.md`'s own precedent already omits other purely-internal governance/plan commits (the historical-recovery-plan row above) without complaint. |
| `agf/learnings/foundry.yaml` edits | **No** — `agf/learnings/**` is not in the `include` glob at all. This project's own learning corpus never ships. | N/A | No release-content impact. Confirms there is no latent version of the #78 MANIFEST-collision problem here (unlike `agf/agents/**`, `agf/learnings/**` was never included, so nothing needed excluding). |

### Governor scope-amendment decision requested: three surfaces ship but weren't in the original batching-amendment scope

**Correction:** the first revision of this plan wrongly treated only the
learning-supersession feature as needing a scope decision, and wrongly
concluded issue #101's cache-topology command needed none because its
*purpose* is CI-internal. Contribution-auditor's review (`gbkp`,
B-RC-SCOPE-1) correctly found both errors and a third omission
(`governance-file-layout-rule.md` was never in the table at all, despite
the governor's own handoff `t41t` naming it explicitly and both changelogs
already classifying it Additive). All three ship in the archive today
regardless of intent or purpose — that is the only test that matters for
this table. Presenting one combined scope-amendment decision, per `gbkp`'s
own suggestion that one combined amendment is acceptable:

1. **Learning-record `status`/`supersedes`** — already merged to `main`
   (MR !1025, Gate 1 via `3q28`, Gate 2 via `h75h`/`1gbs`). Additive minor,
   two new optional schema fields, no existing behavior changed.
2. **`governance-file-layout-rule.md` correction** — already merged, both
   changelogs already classify it Additive (reclassified from Editorial in
   Codex's MR !916 review, 2026-08-02): the removed dual-layout block was
   inert, so the replacement single-layout ownership table is new operative
   guidance, not a restatement.
3. **Issue #101's `agov ci cache-topology-check` command** — already
   merged, ships as a new CLI subcommand with zero adopter-facing
   obligation semantic (it inspects this project's own resolved CI
   configuration; an adopter's project has no equivalent config for it to
   check, so the command is inert for them, not broken — but it is present
   in their binary and their `agov --help` output).

**Recommended: Option A, amend the boundary to include all three.**
Justification, applying uniformly: all three are already on `main`,
already independently Gate 2-approved, already have complete
`CHANGELOG.md`/`agf/CHANGELOG.md` coverage written at merge time (except
the changelog-acceptance-text contradiction noted below, now fixed), and
are the same additive-minor SemVer class as #79/#80. Excluding any of them
would require reverting an already-approved merge or branching the release
from an older, non-`main` commit — more costly and less honest than
acknowledging the scope grew by three governed items since the amendment
was written. `post-v248-release-batching-amendment.md`'s own text does not
forbid amendment; it states the batches as the then-current decision, not
as immutable.

**Option B (per surface): hold any of the three for v2.6.0.** Would
require cherry-picking #79/#80 onto a release branch that excludes the
held commits, or reverting the feature(s) from `main` until v2.6.0. More
invasive than the features themselves and risks a broken cherry-pick given
`main`'s linear history since each merge. The governor may choose Option B
for zero, one, two, or all three surfaces independently — they don't have
to be decided as a single unit even though they're presented together.

This plan proceeds on **Option A for all three** pending explicit governor
confirmation at Gate 1 — do not treat silence as approval; if the governor
prefers Option B for any surface, this plan requires a revision (removing
that surface's changelog entry and shipped artifact from this candidate)
before Gate 1 can close.

## SemVer classification (independently verified)

All five in-scope feature surfaces (#79, #80, learning-supersession,
governance-layout-rule, #101's cache-topology command) add optional new
capability with no removed or redefined existing behavior:

- #79: renames a shipped template file, adds one new fail-closed error path.
  No existing command, flag, or schema field changes meaning.
- #80: adds a new command (`agov coord pending`) and one narrow schema
  rejection (`kind: ack` + `requires-ack: true`, previously schema-legal
  but functionally inert — not a behavior anyone relied on).
  `handoff next`/`take`/`complete` and `coord graph-index` are provably
  unchanged (existing tests pass unmodified).
- Learning-supersession: two new optional schema fields (`status`,
  `supersedes`). `superseded_by` is marked `x-deprecated: true` but remains
  schema-legal (Path B, deprecate-don't-remove).
- Governance-layout-rule: a rules-document correction. No schema, command,
  or validator behavior change — Additive because the removed block was
  inert (asserted no concrete guidance), not because it changes runtime
  behavior.
- #101's cache-topology command: one new CLI subcommand
  (`agov ci cache-topology-check`), inert for any adopter project (it
  inspects this project's own CI configuration shape), no schema or
  existing-command change.

**Additive minor. `v2.5.0-rc.1` is the correct target**, matching
`v250-rc1-contract-core.md`'s own classification and the SoW's release-train
shape (RC checkpoint before final).

## Gate 1 metadata sequencing (corrects B-RC-GATE-METADATA-2)

**Correction:** the first revision of this plan deferred its own Gate 1
approval-metadata update to the later release branch (as release-only
change-set step 1). Contribution-auditor correctly identified this as the
exact failure mode MR !1010 just demonstrated: it merged with stale
`pending-gate1` metadata and needed a separate follow-on MR (!1050) to
repair it after the fact. `AGENTS.md`'s Gate discipline requires updating
plan metadata in the same review cycle as approval, not after merge.

**Corrected sequencing:**

1. When the governor approves Gate 1 (including the scope-amendment
   decision above), this plan's own frontmatter (`status`,
   `gate1_approved_by`, `gate1_approved_at`) is updated **on this plan's
   MR (!1051 or its corrected successor)**, validated, and merged to `main`
   as part of that same review cycle — before any release branch is cut.
2. The release branch (`release-2.5.0-rc.1`, see branch-naming correction
   below) is cut from `main` only **after** the approved plan (with correct
   metadata already on `main`) exists. The release-only change set below
   therefore never touches this plan file's own frontmatter — that
   contamination is exactly what B-RC-GATE-METADATA-2 flagged.

## Release-only change set

Create the release commit on `release-2.5.0-rc.1` (governed branch name
per `release-via-merge-request-rule.md`'s `release-X.Y.Z` form — the prior
`release-v2.5.0-rc.1` draft incorrectly added a `v`; this rule takes
precedence over any historical branch that also got this wrong), based on
the then-current `main` (with this plan's approved metadata already
merged, per the sequencing above), with only these release-truth changes:

1. Set `agf/VERSION` to `2.5.0-rc.1` (full prerelease string, matching the
   `v2.2.0-rc.1`/`v2.2.0-rc.2` precedent — `agf/VERSION` carries the
   complete SemVer prerelease identifier, not just the numeric core).
2. Add dated `[2.5.0-rc.1]` entries to `CHANGELOG.md` and
   `agf/CHANGELOG.md`, converting the current `[Unreleased]` content
   (substance already drafted at each feature's merge time — see "Proposed
   changelog substance" below) rather than rewriting it from scratch.
3. Update the Rust workspace version: `tooling/rust/Cargo.toml`'s
   `[workspace.package] version` and its three internal path-dependency
   pins (`agov-core`, `agov-coord`, `agov-validators`) from `2.4.11` to
   `2.5.0-rc.1`, and the corresponding `Cargo.lock` entries. The other two
   crates (`agov`, `agov-mcp`) inherit via `version.workspace = true` and
   need no direct edit. No Rust source changes in this release beyond the
   version bump itself — all functional Rust changes already merged in
   #79/#80/learning-supersession/governance-layout/#101's own implementation
   MRs.
4. **Update current framework-version surfaces (corrects
   B-RC-VERSION-SURFACES-3):**
   - `agf/agents/MANIFEST.yaml` (this project's own roster — not shipped,
     but kept current for dogfooding honesty);
   - **all twelve adopter-facing `agf/agents/*.yaml` role definitions**
     (`build-engineer.yaml`, `capability-developer.yaml`,
     `coder-tester.yaml`, `contribution-auditor.yaml`,
     `deployment-engineer.yaml`, `documentation-writer.yaml`,
     `fact-checker.yaml`, `governor-reviewer.yaml`, `infosec-sentinel.yaml`,
     `planner-architect.yaml`, and any others present at implementation
     time — enumerate via `ls agf/agents/*.yaml`, don't hand-count), each
     currently `framework_version: "2.4.11"`, all shipped via
     `agf/agents/**`. **This was omitted entirely from the first revision**
     — it named only `MANIFEST.yaml`;
   - `agf/ADOPTING.md`, `agf/templates/AGENTS.md`,
     `agf/templates/learning-record.yaml`, and current version
     statements/examples in `agf/docs/governance/manifest.md`.
   - A full current-version scan (grep for `2.4.11` across every `agf/`
     path in the release diff) is required before Gate 2 to catch any
     further surface neither this plan nor the review named.
5. Update `agf/docs/site/roadmap.md`: first correct the already-stale
   "Latest release: `v2.4.10`" line (v2.4.11 has been published and
   verified since; confirmed by direct inspection — this predates and is
   independent of this release) to the actual current published stable
   (`v2.4.11`), **then** add `v2.5.0-rc.1` as the new release-candidate
   entry distinct from that stable baseline. Preserve the batching
   amendment's later commitments (v2.6.0, v2.7.0, later minors) unchanged
   except for the explicit scope-amendment note from the decision above.
6. Leave the README major/minor citation unchanged only if `2.5` does not
   yet warrant a citation bump per existing convention — confirm against
   the actual current README wording at implementation time rather than
   assuming.
7. **v2.5 milestone/issue-tracker reconciliation (corrects
   B-RC-TRACKER-7):** the GitLab `v2.5.0` milestone currently contains
   three open issues (#33, #67, #78) that this release does not fully
   resolve. Before the release is represented as feature-complete, add a
   precise disposition for each:
   - #33 — the minimal concurrency slice #80 required already shipped in
     this candidate; the issue's full concurrency-specification scope
     remains open and parked for a separate later plan. Add a GitLab note
     stating this and leave the issue open, unmilestoned from v2.5.0 (its
     remaining scope has no governed release destination yet) unless the
     governor names one.
   - #67 — the required legacy-URL and advisory-role compatibility slices
     already shipped (verified in `v250-rc1-contract-core.md`'s "Removed
     work packages" section). Add a GitLab note naming exactly what
     shipped and where, then close #67 only if every remaining item in its
     body is either resolved or has a named successor issue — otherwise
     leave it open with the note.
   - #78 — its reproduced manifest-overwrite and requires-ack defects are
     resolved by #79/#80 in this candidate. Add a GitLab note naming the
     resolution, then apply the same close-or-leave-open test as #67 for
     its remaining observations (some were already retracted or routed
     elsewhere per its own comment thread).
   - This step is tracker reconciliation only — it does not authorize
     implementing any parked remainder of #33/#67/#78. A final GitLab API
     check before tagging must show the milestone state and the release
     notes agree (no open issue silently implies unshipped work is
     included, no shipped resolution is silently unrecorded).

No product implementation, command semantic, schema, validator semantic,
dependency, CI behavior, new documentation policy, rule, v2.6.0/v2.7.0
scope, or branch deletion is in scope.

## Proposed changelog substance

Both changelog files' `[Unreleased]` sections already contain the
substantive entries for #79, #80, learning-supersession,
governance-layout-rule, and #101, written at each feature's own merge
time — this release converts them to a dated `[2.5.0-rc.1]` section rather
than drafting new prose. The release MR must:

- verify every `[Unreleased]` entry in both files as of the release branch
  head is one of the five in-scope items above (no stray entry from an
  unrelated merge slipped in between drafting and release) — the first
  revision's claim of "three in-scope items" was itself the
  changelog/scope-table contradiction B-RC-SCOPE-1 flagged, since root
  `CHANGELOG.md` already contained the governance-layout fix and both
  issue #101 slices at the time that claim was written;
- per **A-RC-MATURITY-1**: describe each shipped feature using accurate
  maturity language — "merged and Level 4 Validated" (per
  `implementation-honesty-rule.md`'s five-level model), not "already
  shipped" or other present-perfect phrasing that implies Level 5
  Deployable status. Each surface reaches Level 5 only after this
  publication path succeeds through post-publication verification below;
- add the SemVer classification statement above near the top of the new
  section, matching the pattern used in prior release changelogs;
- state plainly that this is a **release candidate**, not final — e.g. "RC
  checkpoint; `v2.5.0` final ships the same functional candidate unless a
  correction forces `rc.2`," so a reader does not mistake an RC tag for the
  stable release;
- use the actual test/build counts from the final release branch run, not
  assume unchanged counts from `v2.4.11`.

## Validation before Gate 2

The release MR must provide exact-head evidence for:

- Cargo metadata showing exactly the workspace package at `2.5.0-rc.1` and
  the three internal path requirements at `^2.5.0-rc.1`;
- `cargo test --manifest-path tooling/rust/Cargo.toml --workspace`, full
  pass/fail/ignored counts recorded in changelog and MR evidence;
- strict workspace Clippy with warnings denied;
- `agov validate`, including adopter-docs validation;
- `agov dist validate` and clean tar/zip builds;
- **archive-content verification specific to this release's own features**:
  a fresh **framework archive** (`agf-2.5.0-rc.1.tar.gz`/`.zip`)
  extraction confirms `agf/agents/MANIFEST.example.yaml` ships and
  `agf/agents/MANIFEST.yaml` does not; `agf/schemas/*.json` contains the
  `requires-ack`/`ack` schema constraint and the learning
  `status`/`supersedes` fields; `agf/templates/learning-record.yaml`
  reflects the new fields; all twelve `agf/agents/*.yaml` role definitions
  report `framework_version: "2.5.0-rc.1"`; `agf/rules/` carries the
  corrected `governance-file-layout-rule.md`;
- all shipped archive self-sufficiency tests for both archive formats;
- **corrects B-RC-ARTIFACT-MATRIX-4 — the upgrade/overlay and binary-parity
  checks are two distinct artifacts, tested separately, not conflated into
  one "plain archive extraction":**
  - **(a) Framework archive overlay, three baselines.** A plain
    `agf-2.5.0-rc.1` framework-archive extraction over a scratch adopter
    project seeded from each of `v2.4.5`, `v2.4.6`, and `v2.4.11` (the
    immediately prior release) must leave a divergent adopter
    `agf/agents/MANIFEST.yaml` byte-for-byte unchanged, and must show the
    updated schemas/templates/rules content (the framework archive
    contains no Rust binary — it cannot itself demonstrate a running
    command).
  - **(b) Platform tooling archive install and CLI/MCP parity.** Install
    the separate Linux Rust tooling archive's packaged `agov` and
    `agov-mcp` binaries and confirm, via the actual shipped interfaces
    (not a description of them): CLI `agov coord pending` lists the
    expected pending records; MCP tool `agov_coord_pending` returns
    equivalent output for the same fixture; CLI `agov validate` (the
    generic validator surface, not a schema-specific subcommand) accepts a
    learning record using `status`/`supersedes` and rejects a malformed
    one; packaged-binary self-reporting confirms `2.5.0-rc.1`, the release
    MR exact head, and clean source.
- `tooling/rust/scripts/check-live-tooling-surfaces.sh check` and
  `selftest`;
- tag/version guard evidence with `CI_COMMIT_TAG=v2.5.0-rc.1`;
- strict `mkdocs build` and link checks over the current site content;
- a GitLab API check immediately before merge showing the release MR's
  `closes_issues` response does not silently attempt to close #79/#80
  again (both already closed), and does not silently close #33/#67/#78 —
  their disposition is a separate, explicit GitLab-note step (release-only
  change-set item 7 above), never an implicit side effect of the release
  MR merging;
- a final pre-tag check that the GitLab `v2.5.0` milestone state and this
  release's changelog/notes agree, per B-RC-TRACKER-7's resolution
  condition;
- release-path, provenance, artifact-exclusion, SPDX/license, links, and
  `git diff --check` checks; and
- a green exact-head GitLab MR pipeline.

The release MR must target `main`, contain only the governed release-only
change set, retain its source branch, and receive exact-head Gate 2
approval.

## Merge, tag, and publication sequence

1. Merge only the exact Gate 2-approved release MR head and retain its
   source branch.
2. Confirm `origin/main` resolves to the resulting merge commit and that no
   `v2.5.0-rc.1` tag exists locally or remotely.
3. Create one annotated `v2.5.0-rc.1` tag on that exact `main` merge commit
   and push it immediately. Never move, delete, or recreate the tag.
4. Require the tag pipeline to pass, including framework archives, Linux
   Rust tooling archives, provenance/checksum validation, OCI publication,
   and GitLab Release creation. **Corrects B-RC-PRERELEASE-5**: the release
   is an **ordinary GitLab Release** (`upcoming_release: false`), matching
   the live `v2.2.0-rc.1`/`v2.2.0-rc.2` precedent exactly (independently
   confirmed via the GitLab Release API: both report
   `upcoming_release: false`) — `.gitlab-ci.yml` invokes `release-cli
   create` with name, tag, description, and assets only, with no
   pre-release flag anywhere in the current release path. The first
   revision's "mark it as a GitLab pre-release" language authorized a
   CI/publication behavior change this plan's own scope and stop
   conditions forbid; that language is withdrawn. If a real supported
   pre-release marker is ever wanted, it requires its own separately
   governed CI-change plan, not an ad hoc addition here.
5. Run `agov dist post-publication-check` against the anonymous GitLab
   Release API and a fresh download directory.
6. Verify the GitLab Release, generic package `agf/v2.5.0-rc.1`, and OCI
   tag `agov:2.5.0-rc.1` correspond to the immutable tag pipeline.
7. From a fresh git clone checked out at the exact immutable
   `v2.5.0-rc.1` tag, run both artifact checks from "Validation before
   Gate 2" above — framework-archive overlay (three baselines) and
   platform-tooling-archive CLI/MCP parity — against the real published
   archive bytes, not just the release-branch working tree.
8. Publish and merge a validated completion coordination record.
9. **Do not close issues #79 or #80 as a result of this plan — both are
   already closed.** Link the release tag/pipeline/Release in a
   coordination note for the historical record, but do not touch GitLab
   issue state.

## rc.2 trigger (explicit, per the SoW's own RC discipline)

Any functional, schema, documentation, migration, test, or artifact-layout
correction discovered after `v2.5.0-rc.1` publishes creates a new
`v2.5.0-rc.2` candidate and repeats the full exact-artifact acceptance
matrix above. **Do not schedule a ceremonial rc.2.** If `rc.1` is clean
through publication verification, `v2.5.0` final ships the same functional
candidate under its own later, separately Gate 1-approved release-publication
plan — that plan's release-only change set is limited to version and
changelog metadata, per `release-via-merge-request-rule.md`.

## Acceptance criteria

1. The scope-amendment decision above is resolved by the governor
   (Option A or B, per surface) before Gate 1 approval.
2. This plan's own Gate 1 metadata is updated and merged to `main` before
   the release branch is cut (per the corrected sequencing above) — never
   deferred into the release-only change set.
3. The release MR uses branch name `release-2.5.0-rc.1`, is release-only,
   and receives exact-head Gate 2 approval.
4. All current version surfaces — including all twelve `agf/agents/*.yaml`
   role definitions — and built binary self-reports agree on
   `2.5.0-rc.1`.
5. The immutable tag points to the exact release-MR merge commit on `main`.
6. The tag pipeline and anonymous downloaded-asset validation pass; the
   GitLab Release is ordinary (`upcoming_release: false`), matching the
   `v2.2.0-rc.1`/`v2.2.0-rc.2` precedent, with no new pre-release CI
   behavior introduced.
7. Fresh archive extraction proves both the `MANIFEST.example.yaml`/
   `MANIFEST.yaml` exclusion and the new schema/rules/command surfaces
   are present in the framework archive; a separate platform-tooling-archive
   install proves the new commands are functional via both CLI and MCP.
8. Framework-archive overlay tests pass from all three named baselines
   (`v2.4.5`, `v2.4.6`, `v2.4.11`).
9. The v2.5.0 GitLab milestone's #33/#67/#78 disposition is recorded via
   GitLab notes per release-only change-set item 7, and the milestone
   state agrees with the release notes before tagging.
10. Issues #79 and #80 are not touched by this plan's execution — they
    stay closed, unmodified. #33/#67/#78 are noted, not silently closed.
11. v2.6.0/v2.7.0 scope (per the batching amendment) does not enter this
    release.

## Stop conditions

Stop and return to the governor if release preparation requires: a product
or CI behavior change beyond the version bump (including any new
pre-release marker or flag); a dependency/checksum change; a new command
semantic not already shipped in #79/#80/learning-supersession/
governance-layout/#101's own implementation MRs; weakening any existing
check; changing an existing tag; using a release branch name other than
`release-2.5.0-rc.1` without an explicit rule amendment; including
v2.6.0/v2.7.0 scope; deleting a source branch; reopening or re-closing
issues #79/#80; silently closing #33/#67/#78 without the GitLab-note
disposition step; or discovering a `main` delta surface not accounted for
in the scope-reconciliation table above (in which case: stop, classify it
the same way, and return to the governor rather than silently including or
excluding it).
