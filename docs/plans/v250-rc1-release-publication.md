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
      Requesting a bounded independent review of this plan before Gate 1,
      per the governor's explicit handoff
      (`20260803T134350Z-governor-reviewer-t41t.md`). Scope: the
      scope-reconciliation table below (does every merged-since-v2.4.11
      surface get an honest disposition), the release-only change set, and
      the validation requirements. No implementation, version bump, tag, or
      release action is authorized by this plan itself.
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

Current baseline: `origin/main` at `f77d80d2d66a1a029d385f7ed5b0f7f8142c39ee`,
pipeline `2727385725` (success). Latest published release: `v2.4.11`.
`agf/VERSION` currently reads `2.4.11`. At planning time there is no
`v2.5.0-rc.1` tag or GitLab Release.

This plan authorizes only: the scope decision below, release metadata, a
version-only Rust bump, upgrade/overlay validation, the release MR, one
immutable annotated tag after the release MR merges, publication
verification, and issue #79/#80 closure-link confirmation (both issues are
already closed on GitLab; this plan does not reopen or re-close them, it
records the release that ships their implementation). No product
implementation, new command semantic, schema field, or validator behavior
is authorized here — every one of those already shipped in a separately
Gate 2-approved implementation MR, listed below.

## Scope reconciliation: the actual `main` delta since v2.4.11

Independently audited (`git log v2.4.11..origin/main --no-merges`, 121
commits at audit time, reclassified by what they touch and cross-checked
against `agf/dist/release-manifest.yaml`'s actual `include`/`exclude`
globs — the only authoritative answer to "does this ship in the archive").

| Surface | Ships in `agf-{version}` archive? | In original batching-amendment scope? | Disposition |
|---|---|---|---|
| **#79** — upgrade-safe adopter role authority | Yes (`agf/agents/**` glob; `MANIFEST.yaml` explicitly excluded, `MANIFEST.example.yaml` ships) | Yes, named explicitly | Ship as planned. |
| **#80** — `requires-ack` obligation semantics | Yes (`agf/schemas/**`; new `agov coord pending` command ships in the Rust tooling archive) | Yes, named explicitly | Ship as planned. |
| **Learning-record `status`/`supersedes`** (`docs/plans/learning-supersession-status-fields.md`, Path B, MR !1025, merged `e6dbc0a3`) | Yes (`agf/schemas/**`, `agf/templates/**`) | **No** — not named in the batching amendment's v2.5.0 scope, which lists only "#79 and #80, plus only the minimum #33/#67/#78 compatibility slices required by those contracts" | **Governor decision requested — see below.** |
| Issue #101 — CI cache-policy Slice A/B | The changed Rust source (`agov ci cache-topology-check` subcommand, `ci_cache_topology.rs`) ships as part of the Linux Rust tooling archive; the CI config/script changes it drives do not ship at all (`.gitlab-ci.yml`, `.gitlab/scripts/` are forbidden paths, not included). | No, and not required to be — the batching amendment doesn't mention #101 for any release line. | Already correctly labelled "Adopter docs: not applicable" in both changelogs at implementation time. Note in this release's changelog that the shipped `agov` binary gains one new subcommand with no adopter-facing obligation semantic, consistent with existing framing. No scope decision needed. |
| `docs/plans/historical-branch-only-coordination-recovery.md` + its inventory (MR !1010, merged `f77d80d2`) | No — `docs/plans/` and `docs/reviews/` (except the two individually-included review docs already in the manifest) are not in the `include` glob. | No, and this is repository-internal governance process, not a product surface. | No release-content impact. No changelog entry needed — no other internal plan-drafting commit gets one either. |
| Source registry / radar tracking (`docs/sources.md`, `docs/radar.md`, `docs/plans/radar-governance-signal-gaming.md`, `docs/TODO.md`) | No — repo-root `docs/`, not `agf/`. | No. | No release-content impact (confirmed against the manifest). Currently has zero coverage in either changelog's `[Unreleased]` section; this is a `CHANGELOG.md` (repository-level) completeness question, not a release-artifact question — flagged as a non-blocking observation, not a stop condition, since repo-root `CHANGELOG.md`'s own precedent already omits other purely-internal governance/plan commits (bucket B above) without complaint. |
| `agf/learnings/foundry.yaml` edits | **No** — `agf/learnings/**` is not in the `include` glob at all. This project's own learning corpus never ships. | N/A | No release-content impact. Confirms there is no latent version of the #78 MANIFEST-collision problem here (unlike `agf/agents/**`, `agf/learnings/**` was never included, so nothing needed excluding). |

### Governor decision requested: fold learning-supersession into v2.5.0-rc.1, or hold it back?

The learning-record `status`/`supersedes` feature is **already merged to
`main`** (MR !1025, its own Gate 1 via decision-record `3q28` and Gate 2 via
`h75h`/`1gbs`). It is SemVer-compatible with #79/#80 — Additive minor, two
new optional schema fields, no existing behavior changed, `status` unset on
existing records remains valid. Two honest options:

- **Option A (recommended): amend the batching boundary to include it.**
  Justification: it is already on `main`, already independently Gate
  2-approved, already has complete `CHANGELOG.md`/`agf/CHANGELOG.md`
  coverage written at merge time, and is the same additive-minor SemVer
  class as the rest of this release. Excluding it would require either
  reverting a already-approved merge or branching the release from an
  older, non-`main` commit — both more costly and less honest than
  acknowledging the scope grew by one governed item since the amendment was
  written. `post-v248-release-batching-amendment.md`'s own text does not
  forbid amendment; it states the batches as the then-current decision, not
  as immutable.
- **Option B: hold it for v2.6.0.** Would require cherry-picking #79/#80
  onto a release branch that excludes the learning-supersession commits, or
  reverting the feature from `main` until v2.6.0. Both are more invasive
  than the feature itself and risk a broken cherry-pick given `main`'s
  linear history since the merge.

This plan proceeds on **Option A** pending explicit governor confirmation
at Gate 1 — do not treat silence as approval; if the governor prefers
Option B, this plan requires a revision before Gate 1 can close.

## SemVer classification (independently verified)

All three in-scope feature surfaces (#79, #80, learning-supersession) add
optional new capability with no removed or redefined existing behavior:

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

**Additive minor. `v2.5.0-rc.1` is the correct target**, matching
`v250-rc1-contract-core.md`'s own classification and the SoW's release-train
shape (RC checkpoint before final).

## Release-only change set

Create the release commit on `release-v2.5.0-rc.1`, based on the
then-current `main`, with only these release-truth changes:

1. After Gate 1 approval, update this plan's frontmatter with the exact
   approval identity and UTC timestamp before release implementation is
   presented for Gate 2.
2. Set `agf/VERSION` to `2.5.0-rc.1` (full prerelease string, matching the
   `v2.2.0-rc.1`/`v2.2.0-rc.2` precedent — `agf/VERSION` carries the
   complete SemVer prerelease identifier, not just the numeric core).
3. Add dated `[2.5.0-rc.1]` entries to `CHANGELOG.md` and
   `agf/CHANGELOG.md`, converting the current `[Unreleased]` content
   (substance already drafted at each feature's merge time — see "Proposed
   changelog substance" below) rather than rewriting it from scratch.
4. Update the Rust workspace version: `tooling/rust/Cargo.toml`'s
   `[workspace.package] version` and its three internal path-dependency
   pins (`agov-core`, `agov-coord`, `agov-validators`) from `2.4.11` to
   `2.5.0-rc.1`, and the corresponding `Cargo.lock` entries. The other two
   crates (`agov`, `agov-mcp`) inherit via `version.workspace = true` and
   need no direct edit. No Rust source changes in this release beyond the
   version bump itself — all functional Rust changes already merged in
   #79/#80/learning-supersession's own implementation MRs.
5. Update current framework-version surfaces: `agf/agents/MANIFEST.yaml`
   (this project's own roster — not shipped, but kept current for
   dogfooding honesty), `agf/ADOPTING.md`, `agf/templates/AGENTS.md`,
   `agf/templates/learning-record.yaml`, and current version
   statements/examples in `agf/docs/governance/manifest.md`.
6. Update `agf/docs/site/roadmap.md` to identify `v2.5.0-rc.1` as the
   latest release candidate, preserving the batching amendment's later
   commitments (v2.6.0, v2.7.0, later minors) unchanged except for the
   explicit scope-amendment note from the decision above.
7. Leave the README major/minor citation unchanged only if `2.5` does not
   yet warrant a citation bump per existing convention — confirm against
   the actual current README wording at implementation time rather than
   assuming.

No product implementation, command semantic, schema, validator semantic,
dependency, CI behavior, new documentation policy, rule, v2.6.0/v2.7.0
scope, or branch deletion is in scope.

## Proposed changelog substance

Both changelog files' `[Unreleased]` sections already contain the
substantive entries for #79, #80, and learning-supersession, written at
each feature's own merge time — this release converts them to a dated
`[2.5.0-rc.1]` section rather than drafting new prose. The release MR must:

- verify every `[Unreleased]` entry in both files as of the release branch
  head is one of the three in-scope items above (no stray entry from an
  unrelated merge slipped in between drafting and release);
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
  a fresh archive extraction confirms `agf/agents/MANIFEST.example.yaml`
  ships and `agf/agents/MANIFEST.yaml` does not; `agf/schemas/*.json`
  contains the `requires-ack`/`ack` schema constraint and the learning
  `status`/`supersedes` fields; `agf/templates/learning-record.yaml`
  reflects the new fields;
- all shipped archive self-sufficiency tests for both archive formats;
- **upgrade/overlay tests from three baselines**, per the governor's
  explicit requirement (not just the two the SoW originally named): a
  plain archive extraction of `v2.5.0-rc.1` over a scratch adopter project
  seeded from each of `v2.4.5`, `v2.4.6`, and `v2.4.11` (the immediately
  prior release) — each must leave a divergent adopter
  `agf/agents/MANIFEST.yaml` byte-for-byte unchanged, and each must show
  the new `agov coord pending` command and the learning `status`/
  `supersedes` fields are present and functional post-overlay;
- CLI and MCP parity check for the new surfaces: `agov coord pending`
  (and its `--all-kinds`-equivalent naming as actually shipped) and
  learning `status`/`supersedes` validation are reachable identically via
  the CLI and `agov-mcp`, with packaged-binary self-reporting confirming
  `2.5.0-rc.1`, the release MR exact head, and clean source;
- `tooling/rust/scripts/check-live-tooling-surfaces.sh check` and
  `selftest`;
- tag/version guard evidence with `CI_COMMIT_TAG=v2.5.0-rc.1`;
- strict `mkdocs build` and link checks over the current site content;
- a GitLab API check immediately before merge showing the release MR's
  `closes_issues` response does not silently attempt to close #79/#80
  again (both already closed) or any unrelated issue;
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
   and GitLab Release creation (marked as a pre-release in GitLab, per
   existing convention for `-rc.N` tags if one exists — confirm at
   implementation time; if no such convention exists yet, this plan
   authorizes marking it as a GitLab pre-release, since a `v2.2.0-rc.1`/
   `v2.2.0-rc.2` precedent tag exists and its Release marking should be
   checked for consistency).
5. Run `agov dist post-publication-check` against the anonymous GitLab
   Release API and a fresh download directory.
6. Verify the GitLab Release, generic package `agf/v2.5.0-rc.1`, and OCI
   tag `agov:2.5.0-rc.1` correspond to the immutable tag pipeline.
7. From a fresh git clone checked out at the exact immutable
   `v2.5.0-rc.1` tag, run the three-baseline upgrade/overlay tests above
   against the real published archive bytes, not just the release-branch
   working tree.
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

1. The scope-reconciliation decision above is resolved by the governor
   (Option A or B) before Gate 1 approval.
2. The release MR is release-only and receives exact-head Gate 2 approval.
3. All current version surfaces and built binary self-reports agree on
   `2.5.0-rc.1`.
4. The immutable tag points to the exact release-MR merge commit on `main`.
5. The tag pipeline and anonymous downloaded-asset validation pass.
6. Fresh archive extraction proves both the `MANIFEST.example.yaml`/
   `MANIFEST.yaml` exclusion and the new schema/command surfaces are
   present and functional.
7. Upgrade/overlay tests pass from all three named baselines
   (`v2.4.5`, `v2.4.6`, `v2.4.11`).
8. Issues #79 and #80 are not touched by this plan's execution — they stay
   closed, unmodified.
9. v2.6.0/v2.7.0 scope (per the batching amendment) does not enter this
   release.

## Stop conditions

Stop and return to the governor if release preparation requires: a product
or CI behavior change beyond the version bump; a dependency/checksum
change; a new command semantic not already shipped in #79/#80/
learning-supersession's own implementation MRs; weakening any existing
check; changing an existing tag; including v2.6.0/v2.7.0 scope; deleting a
source branch; reopening or re-closing issues #79/#80; or discovering a
`main` delta surface not accounted for in the scope-reconciliation table
above (in which case: stop, classify it the same way, and return to the
governor rather than silently including or excluding it).
