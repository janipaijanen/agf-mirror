# Changelog

All notable repository-level changes to AGF-NG are recorded here.

The canonical adopter-facing AGF v2 changelog lives at
[`agf/CHANGELOG.md`](agf/CHANGELOG.md). The bootstrap reference changelog for
agents-govern 1.x lives at [`agents-govern/CHANGELOG.md`](agents-govern/CHANGELOG.md).

## [2.5.0-rc.1] - 2026-08-03

**RC checkpoint; not final.** `v2.5.0` final ships the same functional
candidate as this release candidate unless a correction forces `rc.2`.

**SemVer classification: Additive minor.** Two new optional learning-record
schema fields, one new fail-closed adopter-role-authority default, one new
non-`handoff` obligation-tracking command, one rules-document correction
(inert content replaced with new operative guidance), and one new packaged
CI tooling command. No existing schema field, command, or validator
behavior changed for an adopter not opting into the new fields/commands.

### Added

- Learning-record `status` and `supersedes` fields
  (`docs/plans/learning-supersession-status-fields.md`, Gate 1 approved,
  Path B). `superseded_by` is now deprecated (`x-deprecated: true`) but
  kept schema-legal; `supersedes` on the new entry replaces it and is
  enforced by `validate_learnings`. Full detail: `agf/CHANGELOG.md`.
- Upgrade-safe adopter role authority (issue #79,
  `docs/plans/v250-rc1-contract-core.md` work package 1). The shipped
  release archive no longer contains `agf/agents/MANIFEST.yaml` — a plain
  archive re-extraction over an adopter project could previously overwrite
  an adopter's own divergent roster with the framework's dogfooding
  manifest (reproduced concretely by issue #78). The archive now ships a
  neutral `agf/agents/MANIFEST.example.yaml` template instead, excluded
  from packaging by an explicit rule and never read as adopter state by
  `agov`. With the default path in effect and `MANIFEST.yaml` absent,
  `agov session resume`'s cold-start output states the missing path
  explicitly and names both remedies. Full detail: `agf/CHANGELOG.md`.
- `requires-ack` obligation semantics for non-handoff coordination records
  (issue #80, `docs/plans/v250-rc1-contract-core.md` work package 2).
  `requires-ack: true` was previously silently inert on every kind except
  `handoff`. The schema now rejects `kind: ack` + `requires-ack: true`
  (an ack cannot require its own ack), and a new `agov coord pending`
  command tracks every other kind's obligations via direct
  `references-to` resolution, with pre-cutoff records grandfathered.
  `handoff next`/`take`/`complete` and `coord graph-index` are unchanged.
  Full detail: `agf/CHANGELOG.md`.

Adopter docs: updated — see `agf/CHANGELOG.md`.

### Fixed

- `agf/rules/always/governance-file-layout-rule.md` and its
  `.claude/rules/` copy no longer present "Layout A"/"Layout B" as a live
  adopter choice. AGF v2.0 supports exactly one adoption layout, already
  stated in `agf/ADOPTING.md` and `agf/docs/site/docs/adopt/install-and-layouts.md`;
  the rule's stale `<!-- CUSTOMIZE -->` template contradicted both. Replaced
  with a single-layout ownership table (which paths are adopter-owned vs.
  upstream-owned) and corrected a stale header pointer and a pre-rename
  example path.

**Reclassified from Editorial to Additive** (Codex review, MR !916 on a
related plan, 2026-08-02): the removed dual-layout block was inert
(neither option was ever live), so the rule previously asserted no
concrete ownership guidance at all. The replacement table is new operative
content, not a restatement of prior behavior — Additive under
`downstream-impact-rule.md`'s own test, not Editorial. The table's
`agf/agents/*.yaml` row is explicitly marked as deciding issue #79's own
Gate 1 plan (`docs/plans/v250-rc1-contract-core.md`), not asserted
independently ahead of it — all other rows restate long-standing
convention already published in `ADOPTING.md`, unaffected by #79.

Adopter docs: not applicable — the table's content already exists in
`ADOPTING.md`'s "Confirm the layout" table (except the #79-pending row,
which is explicitly marked as such); no new adopter doc, template, or
behavior change as a result.

### Changed

- CI: the shared Rust cache (`.rust-base`) is now pull-only outside ordinary
  default-branch push pipelines — feature branches, merge requests, tags,
  schedules, API/web/trigger pipelines, and downstream pipelines no longer
  spend runner time archiving and uploading a cache most of them use once
  (issue #101, Slice A of `docs/plans/ci-rust-cache-policy-slice-a.md`,
  child scope of #53). A single explicit default-branch write context
  (`RUST_CACHE_POLICY: pull-push`) keeps the shared cache warm; every other
  context fails safe to `pull` via a top-level default, verified at runtime
  by `.gitlab/scripts/check-rust-cache-policy.sh assert-current`. The cache
  key moved to a versioned `rust-v2-main` namespace (with `rust-main`
  retained as fallback) so non-default consumers stop preferring a stale
  branch-specific cache they can no longer refresh. This does not eliminate
  concurrent default-branch writers uploading to the same key — that
  topology is separately gated as Slice B, per the plan's explicit
  boundary. No job, test coverage, or acceptance requirement changed; cache
  content remains build acceleration only, never test evidence.

Adopter docs: not applicable — repository-internal CI cost optimization,
not an AGF framework capability.

- CI: narrowed the default-branch Rust cache (`.rust-base`) from eight
  concurrent last-writer-wins uploaders to exactly one designated writer,
  `rust:tooling:test` (issue #101, Slice B of
  `docs/plans/ci-rust-cache-policy-slice-b.md`). Every other consumer —
  automatic, manual, and tag/release — is pull-only in every context,
  including an ordinary default-branch push, enforced by two separate
  variables (`RUST_CACHE_POLICY` for consumers, `RUST_CACHE_WRITER_POLICY`
  for the sole writer) so a new `.rust-base` inheritor is pull-only by
  construction rather than by remembering to opt out. The shared Rust job
  image moved from the floating `rust:1` tag to an immutable
  `rust:1.97.1@sha256:...` digest pin, and the cache key advanced to
  `rust-v3-main` (`rust-v2-main` and `rust-main` retained as fallbacks) so
  compiled output from the formerly floating image is never treated as the
  pinned toolchain's primary cache. `.gitlab/scripts/check-rust-cache-policy.sh`
  is now role-aware (`RUST_CACHE_ROLE=writer|consumer`) and fails closed on
  an unrecognised or missing role. A new `agov ci cache-topology-check`
  subcommand statically verifies a resolved CI configuration has exactly
  one writer, dynamically inventoried from GitLab CI Lint's merged YAML
  (not a hard-coded job list, so an indirect `.rust-base` inheritor cannot
  silently escape the check the way two jobs did during Slice A). This is
  a manually-invoked verification tool used for Gate 2 evidence, not a
  standing CI job — `CI_JOB_TOKEN` is not authorized against GitLab's CI
  Lint API on this project (confirmed via a real 403 at implementation
  time), and adding a scoped credential to enable automatic per-pipeline
  enforcement was deferred rather than done unilaterally. A pinned
  ShellCheck job (`docker.io/koalaman/shellcheck-alpine@sha256:...`) lints
  the policy script in CI. This trades away the small, inconsistent
  release-profile cache benefit `agov:governance:validate` sometimes got
  under the old last-writer-wins topology (see the plan's "why
  rust:tooling:test" section) — acceleration only, no correctness or
  acceptance-evidence effect. No job, test coverage, or acceptance
  requirement changed.

Adopter docs: not applicable — repository-internal CI cost optimization,
not an AGF framework capability.

## [2.4.11] - 2026-08-01

Documentation and accessibility patch.

### Added

- Added a visible AI-generation disclosure and descriptive alternative text
  to every public figure embed in `README.md` and the public documentation
  site (33 embeds), and an AI-assistance/human-responsibility acknowledgement
  in `README.md` and the generated release-information page.
- Added a strict-build accessibility hook
  (`agf/docs/build/hooks/figure_accessibility.py`) that enforces the above
  going forward. An adopter who reuses the supplied MkDocs configuration
  unmodified inherits this same check over their own content.
- Added title/description metadata to all 13 public SVG figure pairs.
- Stated the disclosure measure as supporting Article 50 (EU AI Act)
  transparency and accessibility objectives, without claiming legal
  certification. Academic paper PDF figures remain outside this release's
  accessibility scope, as a separately tracked follow-up.

### Fixed

- Corrected the copyable commissioning prompt's stale hard-coded version pin
  (issue #97).
- Corrected public compatibility prose to describe sustained author-operated
  Claude and Codex integration use in Visual Studio Code, attributed
  explicitly to the project author, without asserting independent-adopter or
  universal-compatibility coverage. The contribution page's request for
  independent corroboration is unchanged.
- Fixed the figure renderer's relative-path invocation
  (`agf/docs/figures/render_svg.py`), which previously misrouted the public
  site copy when run via its documented invocation.

### Removed

- Removed `docs/academic/` (the "Governing the Governors" working paper,
  IEEE build files, figures, and academic/visual review records) after
  relocating it verbatim to `agf-promotional`, where the project's
  promotional and explanatory content about AGF lives. Historical
  coordination records, plans, and `agf/CHANGELOG.md` entries that
  reference the old path are preserved verbatim as governed evidence. This
  removal predates and is independent of the accessibility work above; it
  is included here because it was not yet part of a tagged release.

### Validation

- Rust workspace: 299 passed, 0 failed, 3 ignored (version-only bump; no
  Rust source changed in this release). Clippy clean with warnings denied.
- Strict `mkdocs build` including the new accessibility hook: passed.

Addresses issue #97. The issue closes only after the published v2.4.11
assets and site pass post-publication verification.

Adopter docs: updated

## [2.4.10] - 2026-07-31

Patch release making the repository-local `tooling/agov` launcher
source-compatible with its checkout.

### Fixed

- Removed the wrapper's handwritten command registry so the authoritative
  Rust parser receives the full command tree, including `coord
  publication-check` and all `dist` commands.
- Added a deterministic fingerprint of behavior-affecting tooling inputs and
  required compatible release/debug binaries before execution.
- Added one-shot verified release rebuilding with fail-closed behavior when
  Cargo or a compatible rebuilt binary is unavailable.
- Added actionable cross-worktree mismatch diagnostics that identify the
  correct checkout-local wrapper path. Standalone published binaries retain
  their existing behavior; this policy applies to the repository wrapper.

### Validation

- Rust workspace: 299 passed, 3 ignored; wrapper freshness, live-surface, and
  clean tar and zip framework archive acceptance suites passed.

Addresses issue #92. The issue closes only after the published v2.4.10 assets
and a fresh git checkout at the immutable tag pass post-publication
verification.

Adopter docs: updated

## [2.4.9] - 2026-07-31

Patch release removing stale live tooling surfaces after the Rust `agov`
migration.

### Fixed

- Removed the broken `bin/agf` wrapper, which delegated every command to a
  nonexistent Elixir/Mix project, and removed the dead Elixir parity harness.
- Converted current contributor, adopter, contract, governance, and
  decommission guidance to the supported Rust `agov` commands while preserving
  historical, generic ecosystem, and release-exclusion evidence.
- Added a fail-closed live-surface regression check with a negative self-test
  so supported entry points and current instructions cannot silently return to
  removed Mix, Elixir, legacy Python, or `bin/agf` paths.

### Validation

- Rust workspace: 299 passed, 3 ignored; clean tar and zip framework archive
  acceptance suites passed.

Addresses issue #93. The issue closes only after the published v2.4.9 assets
pass anonymous post-publication verification.

Adopter docs: updated

## [2.4.8] - 2026-07-30

Publication-completion recovery after the immutable v2.4.7 tag published its
OCI image but failed before creating the generic package and GitLab Release.

### Fixed

- `release:create` no longer passes the absent optional Windows `agov` ZIP to
  the strict Elixir-artifact exclusion checker. Framework archives and Linux
  tooling archives remain mandatory and are still checked unconditionally.
- The pre-tag release evidence now assembles the real framework and Linux
  tooling preview artifacts and exercises the exact downstream exclusion path
  both without an optional ZIP and with a temporary valid optional ZIP.

### Changed

- Release documentation records v2.4.7 as an immutable OCI-only partial
  publication and directs adopters to the complete v2.4.8 release.

## [2.4.7] - 2026-07-30

Tagged publication-recovery candidate superseding the unpublished v2.4.6
candidate. The v2.4.7 pipeline published the OCI image, but a downstream
release-creation check still required the optional Windows ZIP; consequently
no generic package or GitLab Release was created. Adopt v2.4.8 instead.

### Changed

- Tag pipelines now publish framework archives, Linux x86_64/aarch64 Rust
  tooling, their checksum and provenance sidecars, and OCI provenance without
  performing the Windows cross-build.
- Windows packaging remains available as optional, non-blocking manual preview
  evidence and is no longer part of the mandatory release-asset contract.
- Release documentation treats each GitLab Release page's actual asset links
  as the authoritative platform inventory.

## [2.4.4] - 2026-07-26

Patch release making the public documentation's mutable build identity
unambiguous relative to the latest immutable release. See
[`agf/CHANGELOG.md`](agf/CHANGELOG.md) for adopter-facing details.

### Fixed

- Public release information now separates the framework version, latest
  release tag and source commit, live documentation branch and commit, and
  whether the site is release or later development documentation.
- The AI evaluation dossier embeds the same full provenance tuple in visible,
  extractable text. The accessible page header visibly shows the framework
  version and docs-build commit; its accessible label also reports release
  status.
- Removed stale version-numbered lifecycle labels and added a strict source
  scanner that prevents their reintroduction except for an explicitly bounded
  historical qualifier.

### Changed

- Public status banners no longer couple page maturity to a historical release
  number.
- Added one governed public feature summary to the README and documentation
  homepage, and clarified that contributor setup belongs at the end of the
  adopter-facing README.
- Polling guidance now names incremental coordination listing and the optional
  structured MCP transport while retaining the Rust CLI fallback.

## [2.4.3] - 2026-07-25

Patch release combining three small adopter-facing fixes (issues #64, #65,
#66) with the v2.4.3 documentation-quality and information-architecture
backlog (MR !483/!527). See [`agf/CHANGELOG.md`](agf/CHANGELOG.md) for
adopter-facing details.

### Fixed

- Replaced all public Mermaid diagrams with openable, full-size SVG figures
  and removed the now-unused client-side Mermaid runtime.
- Corrected the Adopter FAQ's tooling-commissioning guidance to recommend
  installing and using `agov` from day one instead of after the first
  governed change.
- Explained that different authorized people may hold Gate 1 and Gate 2 on
  the same change, sequentially.
- Named `agf/VERSION` as the canonical source at all six hardcoded
  `agov:<version>` OCI-tag literals, for editor discoverability.
- Repointed two misdirected metadata links to their canonical slug.
- Reassigned the governance-loop figure's `learn` node from the "Reference /
  deferred content" palette category to "Shared concept," since learning
  records are authored by AI and by humans, not exclusively either.
- Aligned "Level 2" capability-maturity naming to "Surface-defined" across
  the internal rule file, matching the public site's existing usage.
- Documented `palette.md`'s bounded light-background assumption for seven
  named figure canvases, making no universal renderer/theme claim.
- Added `<figure>`/`<figcaption>` semantics to the seven existing reference
  figure embeds.

## [2.4.2] - 2026-07-25

Patch release for the public-experience and evidence-presentation work in MR
!488. See [`agf/CHANGELOG.md`](agf/CHANGELOG.md) for adopter-facing details.

### Fixed

- Corrected public governance-enforcement, provider-boundary, command-maturity,
  and adoption-exit claims.
- Aligned the adoption-exit figure's labels with its surrounding guidance.

### Changed

- Condensed the homepage tooling inventory and retained one canonical command
  reference.
- Added exact-build repository references to the AI evaluation dossier.
- Documented assistant/role bylines and improved progressive disclosure and
  next-action guidance across six evaluator-facing pages.

## [2.4.1] - 2026-07-24

Patch release closing the accessibility and prose-hardening work governed by
issues #51, #55, and #56 (MR !458). See [`agf/CHANGELOG.md`](agf/CHANGELOG.md)
for the adopter-facing details.

### Fixed

- Public documentation now precedes every figure with page-specific prose
  carrying its essential meaning, so the argument does not depend on perceiving
  the image alone.
- Mermaid diagrams now use a narrow-width horizontal-scroll presentation and
  expose a shared accessible name and description derived from their source;
  this closes the mobile and assistive-technology remediation deferred from
  v2.4.0.

### Changed

- The accessibility evidence records the exact browser, operating system, and
  assistive-technology test boundary. The result is Level 4 Validated and does
  not claim formal WCAG conformance or universal assistive-technology support.

## [2.4.0] - 2026-07-23

Minor release (see [`agf/CHANGELOG.md`](agf/CHANGELOG.md) `[2.4.0]` for the
adopter-facing feature changelog covering the four v2.4.0 workstreams).
Repository-level changes not already covered there:

### Fixed

- Documentation: kept the persistent `AGF / <page>` header and `AGF —
  <page>` browser-title identity visible on every page (MR !330).
- Documentation: corrected the "what ships and what you activate" page to
  match maintainer evidence rather than an earlier inaccurate framing
  (MR !328).
- Closed a tar-extraction ETXTBSY race by explicitly closing the writer
  handle before execution (MR !398).
- Closed a remaining ZIP/TAR extraction ETXTBSY race with a bounded
  executable-file-busy exec retry, on top of the MR !398 fix (MR !408).
- Fixed token-ROI log write-atomicity and a test-suite pollution issue
  (issue #50, MR !363).

### Changed

- CI: suppressed duplicate branch-push pipelines when an open MR already
  exists for the branch, cutting CI cost on routine coordination and seed
  commits (MR !410).

## [2.3.1] - 2026-07-21

Patch release recovering the repository's public release truth after v2.3.0
was published with this changelog still ending at 2.2.5. Tag `v2.3.0` remains
immutable; its historical entry was added on `main`, and v2.3.1 publishes that
correction under a new release identity.

### Added

- Proposed learnings from the v2.3.0 release now record consumer-layout host
  preflight and complete release truth-surface enumeration as future target
  states. Their generalized recommendations are not implemented by this patch.

### Fixed

- GitLab Release creation now exposes direct links for all individual checksum
  and provenance sidecars, verifies the mandatory archive/sidecar set before
  upload, and carries a negative self-test for a missing provenance record.
- Added the immutable recovery trail that supersedes premature v2.3.0 terminal
  completion claims without moving the published v2.3.0 source or OCI tags.

## [2.3.0] - 2026-07-21

Repository-level changes accompanying the v2.3.0 framework release
(see [`agf/CHANGELOG.md`](agf/CHANGELOG.md) `[2.3.0]` for the
adopter-facing feature changelog). Found and recorded during this
release's own public-truth verification pass — this repository-level
changelog had not been updated since 2.2.5.

### Fixed

- `agf/dist/release-manifest.yaml`: excluded `__pycache__/` and `.pyc`
  paths from the release package `exclude`/`forbidden` lists, closing a
  leak of stale Python bytecode into the framework release archives
  (MR !303).
- `.gitlab-ci.yml`: the `rust:tooling:preview`/`rust:tooling:release`
  checksum-generation step recorded a build-tree path
  (`../../dist/<archive>`) in each `.sha256` file instead of the archive
  basename, breaking the standard `sha256sum -c` consumer-verification
  flow. Fixed to match the basename-only convention `agov dist build`
  already used for framework archives (MR !308).

### Changed

- Enabled the project's GitLab Container Registry (previously disabled),
  required for the `agov:oci:release` tag-pipeline job to publish the
  `agov` OCI image. This had not been exercised end-to-end against a real
  tag before v2.3.0.

## [2.2.5] - 2026-06-25

This patch release completes AGF-NG's transition away from historical
agents-govern 1.x Python tooling and records the repository-level metadata
needed for the current Rust-only operator path.

### Changed

- AGF v2 contract documentation now marks v1.x Python-only tooling surfaces as
  retired or deferred before historical Python code removal. Binding validation
  is explicitly deferred in current Rust `agov`; `agov-validate-acronyms`,
  `agov-bump`, `agov --capabilities`, and `agov --contract-changelog` are
  retired; `agov-health` remains deferred.

### Removed

- Historical agents-govern 1.x Python package sources and package metadata are
  removed from `agents-govern/`; Rust `tooling/agov` is the only supported
  AGF-NG operator tooling.

### Added

- Root `LICENCE` file summarising the repository licence map:
  CC BY-SA 4.0 for framework/governance content, MIT for tooling, and
  separate commercial or other terms where expressly granted.
- Root `CHANGELOG.md` to make repository-level changes visible before readers
  enter the adopter-facing `agf/` distribution or bootstrap reference tree.

## [2.2.0-rc.2] - 2026-06-24

### Fixed

- Rust `agov` release archives now embed CI commit provenance in the binary
  itself, so `agov --version` and `agov version --format json` report the tag
  pipeline commit instead of `unknown dirty`.

## [2.2.0-rc.1] - 2026-06-24

### Added

- Readable coordination IDs are now the default format for
  `agov coord brief`.
- Rust `agov` reports version provenance through `agov --version` and
  `agov version --format json`.
- `agov coord graph-index` now has layered contract documentation covering
  human operation, AI cold-start use, output fields, and exit behavior.

### Changed

- RC-1 readiness audit completed: the Rust operator path was verified,
  inherited v1.x material in high-risk public paths was labelled, and release
  documentation was aligned with current tooling behavior.
