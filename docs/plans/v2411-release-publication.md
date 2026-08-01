---
id: v2411-release-publication
status: pending-gate1
topic: v2411-release-publication
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: "Both candidate batch items already carry prior contribution-auditor review at their own exact heads: MR !890 (docs/coordination/20260731T180442Z-contribution-auditor-g1sr, CLEAR) and MR !894 (docs/coordination/20260801T123434Z-contribution-auditor-6mkq, CLEAR, renewed after an evidence correction). This plan only sequences their joint publication and adds no new implementation content. A renewed review of this exact plan head is still required before Gate 1, per the governor handoff requesting it."
  - role: infosec-sentinel
    status: not-applicable
    reason: "Release routing and version/changelog metadata only, over content both already assessed not-applicable for InfoSec (public prose, accessibility metadata, and a documentation-build hook with no credential, dependency, network trust boundary, learning-payload, or authorization-semantic change)."
---

# Plan: publish v2.4.11 documentation and accessibility release

## Decision requested

Authorize a bounded, patch-level release that publishes two already-merged,
already Gate 2-approved documentation batches as v2.4.11:

1. **Issue #97 / MR !890** — corrects the stale commissioning-prompt version
   pin and accurately documents sustained author-operated Claude and Codex
   integration use in Visual Studio Code, while preserving the
   independent-adopter evidence boundary. Gate 2-approved at exact head
   `2a7333203275b4848a8776a10108294da0cd55c6` by
   `docs/coordination/20260801T113622Z-governor-reviewer-466p.md`; merged to
   `main` at `2c7094595507e2a2cd5682d580f4212df37eea32`.
2. **MR !894** — AI-assistance acknowledgement and human-responsibility
   statement, descriptive alternative text and visible AI-generation
   disclosure for all 33 public figure embeds, a strict-build accessibility
   hook, SVG title/description metadata for all 13 figure pairs, a
   relative-path figure-renderer fix, and bounded Article 50 transparency
   wording. Merged to `main` at `0e898c269f4db1bbb818b2775e7af89b8332cdf0`,
   Gate 2-approved at exact head `669277361c5218bf87509a393dfaff58ba4573b5`
   by `docs/coordination/20260801T151548Z-governor-reviewer-hq8k.md`.

Current baseline: `origin/main` at `0e898c269f4db1bbb818b2775e7af89b8332cdf0`,
pipeline `2724017202` passed. Latest published release: `v2.4.10`. This plan
authorizes only release metadata, validation, the release MR, one immutable
annotated tag after the release MR merges, publication verification, and
issue #97 closure after verification.

At planning time there is no `v2.4.11` tag or GitLab Release. Issue #97
remains open.

## SemVer classification (independently verified, not assumed)

Both merged diffs were inspected directly against their merge commits, not
taken on the strength of their own handoffs:

- **MR !890** (`2c709459`, diffed against its parent): four files changed,
  all prose — `agf/docs/site/contribute.md`,
  `agf/docs/site/docs/adopt/install-and-layouts.md`,
  `agf/docs/site/docs/concepts/position.md`, and the plan file itself. No
  code, command, schema, or config change.
- **MR !894** (`0e898c26`, diffed against its parent `7b82abdf`): 48 files
  changed. The only non-content, non-figure file is
  `agf/docs/build/mkdocs.yml`, whose diff is exactly one line —
  `- hooks/figure_accessibility.py` added to the existing `hooks:` list
  alongside the two hooks already registered there. The new hook
  (`agf/docs/build/hooks/figure_accessibility.py`) is a `mkdocs build
  --strict`-time check over this project's own `README.md` and
  `agf/docs/site/**` content; it adds no `agov` subcommand, no schema field,
  and no new required action for an adopter's own project. It fails only
  this repository's own doc build if this repository's own pages regress,
  the same class of addition as the two hooks already present. Every other
  changed file is prose, SVG metadata, or coordination records.

Neither batch item adds a command, schema field, or adopter-facing
obligation semantic. Combined, the batch is honestly patch-compatible under
`docs/plans/post-v248-release-batching-amendment.md`'s rule ("Features, new
commands, schemas, or obligation semantics advance the minor version").
**No stop condition is triggered; v2.4.11 (patch) is the correct target.**

## Release-only change set

Create the release commit on `release-v2.4.11`, based on the
then-current `main`, with only these release-truth changes:

1. After Gate 1 approval, update this plan's frontmatter with the exact
   approval identity and UTC timestamp before release implementation is
   presented for Gate 2.
2. Set `agf/VERSION` to `2.4.11`.
3. Add dated v2.4.11 entries to `CHANGELOG.md` and `agf/CHANGELOG.md`
   (substance below).
4. Update the five Rust workspace package versions
   (`tooling/rust/Cargo.toml` and per-crate `version.workspace = true`
   inheritance), the three internal path requirements, and the
   corresponding five `Cargo.lock` package entries from 2.4.10 to 2.4.11,
   matching the lockstep pattern used for every prior release including
   v2.4.9 (docs-adjacent) — no dependency or checksum may change. No Rust
   source in `tooling/rust/` changes in this release; the bump is
   version-provenance-only, required because `agov`/`agov-mcp`
   self-reporting and the tag/version guard assert exact agreement with
   `agf/VERSION`.
5. Update current framework-version surfaces only: adopter-facing agent
   definitions, `agf/agents/MANIFEST.yaml`, `agf/ADOPTING.md`,
   `agf/templates/AGENTS.md`, `agf/templates/learning-record.yaml`, and
   current version statements/examples in
   `agf/docs/governance/manifest.md`.
6. Update `agf/docs/site/roadmap.md` to identify v2.4.11 as the latest
   release and mark issue #97 deployable only after publication
   verification. Preserve historical release evidence and later batch
   commitments, including the v2.5.0 contract-core boundary from
   `post-v248-release-batching-amendment.md`.
7. Leave the README major/minor citation unchanged because 2.4 remains the
   current release line. Leave historical coordination, plans, changelog
   entries, and valid earlier-or-later compatibility statements unchanged.

No product implementation, command semantic, schema, validator semantic,
dependency, CI behavior, new documentation policy, rule, new v2.5.0 contract
work, or branch deletion is in scope. `!901` (Four-Way Test consultation)
and `v2.5.0` contract-core scope are explicitly out of scope and untouched.

## Proposed changelog substance

Both changelog files (`CHANGELOG.md` and `agf/CHANGELOG.md`) may use their
own existing per-version formatting, but must express this release truth:

> v2.4.11 is a documentation and accessibility patch. It corrects the
> commissioning-prompt version pin and documents sustained author-operated
> Claude and Codex integration use in Visual Studio Code (issue #97),
> without asserting independent-adopter or universal-compatibility
> coverage. It adds a visible AI-generation disclosure and descriptive
> alternative text to every public figure embed in `README.md` and the
> public documentation site, adds a strict-build check that enforces this
> going forward, adds title/description metadata to all 13 public SVG
> figure pairs, and fixes the figure renderer's relative-path invocation.
> It states the measure as supporting Article 50 transparency and
> accessibility objectives without claiming legal certification. Academic
> paper PDF figures remain outside this release's accessibility scope, as a
> separately tracked follow-up. Adopter docs: updated. Issue #97 closes
> only after public artifact verification.

The release changelogs must use the actual test/build counts and pass
results from the final release branch run, not assume unchanged counts from
v2.4.10.

## Validation before Gate 2

The release MR must provide exact-head evidence for:

- a focused version audit showing every current release-truth surface is
  2.4.11 and historical evidence was not rewritten;
- Cargo metadata showing exactly five workspace packages at 2.4.11 and the
  three internal requirements at `^2.4.11`;
- `cargo test --manifest-path tooling/rust/Cargo.toml`, with passed and
  ignored counts recorded in changelog and MR evidence, as a regression
  check on the version-only Rust bump;
- strict workspace Clippy with warnings denied;
- `agov validate`, including adopter-docs validation;
- `agov dist validate` and clean tar/zip builds;
- all shipped archive self-sufficiency tests for both archive formats;
- `tooling/rust/scripts/check-live-tooling-surfaces.sh check` and
  `selftest`;
- built `agov` and `agov-mcp` self-reporting 2.4.11, the release MR exact
  head, and clean source;
- tag/version guard evidence with `CI_COMMIT_TAG=v2.4.11`;
- strict `mkdocs build` (including the new `figure_accessibility.py` hook),
  link checks, and a re-run of the 33-embed accessibility scan against the
  release-branch head;
- a GitLab API check immediately before merge showing the release MR's
  `closes_issues` response is empty, rather than relying on visible
  description text to prevent premature issue #97 closure;
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
   `v2.4.11` tag exists locally or remotely.
3. Create one annotated `v2.4.11` tag on that exact `main` merge commit and
   push it immediately. Never move, delete, or recreate the tag.
4. Require the tag pipeline to pass, including framework archives, Linux
   Rust tooling archives, provenance/checksum validation, OCI publication,
   and GitLab Release creation. Report optional Windows artifacts only when
   they are present in the actual Release inventory.
5. Run the released Rust `agov dist post-publication-check` against the
   anonymous GitLab Release API and a fresh download directory. Verify all
   downloaded bytes and provenance, executable self-reporting, and source
   archive contents.
6. Verify the GitLab Release, generic package `agf/v2.4.11`, and OCI tag
   `agov:2.4.11` correspond to the immutable tag pipeline.
7. From a fresh git clone checked out at the exact immutable `v2.4.11` tag,
   confirm the public documentation site builds strictly with the
   accessibility hook active and that `README.md` and the site both carry
   the corrected issue #97 wording and the AI-assistance/figure-disclosure
   content.
8. Only after all public checks pass, close issue #97 with links to the
   tag, pipeline, Release, and durable completion record.
9. Publish and merge a validated completion coordination record before the
   next release batch treats v2.4.11 as its baseline.

## Acceptance criteria

1. The release MR is release-only and receives exact-head Gate 2 approval.
2. All current version surfaces and built binary self-reports agree on
   2.4.11.
3. The immutable tag points to the exact release-MR merge commit on `main`.
4. The tag pipeline and anonymous downloaded-asset validation pass.
5. Published site content demonstrates the corrected issue #97 wording and
   the MR !894 accessibility/disclosure content live on the public Pages
   build.
6. Issue #97 closes only after the release reaches public verification.
7. `!901` and `v2.5.0` contract-core scope do not enter v2.4.11.

## Stop conditions

Stop and return to the governor if release preparation requires a product
or CI behavior change, a dependency/checksum change, a command-semantic
change, weakening the accessibility hook or any existing check, changing an
existing tag, including `!901` or v2.5.0 contract-core work, deleting a
source branch, closing issue #97 before public verification, or publishing
a platform claim unsupported by actual assets.
