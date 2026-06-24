# Changelog — agents-govern 2.0

v2.0 entries only. The v1.x changelog is not part of the `agf/`
distribution.

## [Unreleased]

### Changed

- `evaluate.md`: version and commit hash are now mentioned explicitly in the
  identity check table, pointing reviewers to the page header badge.
- `evaluate.md`: `agf/schemas/` in the technical reading list is now a
  clickable link to the GitLab repository browser.

### CI

- `rust:tooling:parity` is now manual / on-demand only (`when: manual`); it
  no longer runs automatically on every MR touching `tooling/rust/`. The job
  caches its Rust build artifacts via a dedicated `rust-parity-*` cache key so
  repeated manual runs on the same branch are faster.
- `rust:tooling:deny`: dropped `--force` from `cargo install cargo-deny`; the
  binary is now served from the `.cargo/bin` cache on subsequent runs.
- `.rust-tooling-package`: `cross` is now installed from crates.io
  (`--locked --version "^0.2"`) instead of from git source, saving 2–3 minutes
  per release build.
- `.rust-base` cache now declares `fallback_keys: [rust-main]` so new branches
  warm from the main branch cache instead of starting cold.

## [2.2.2] — 2026-06-24

### Fixed

- All Rust CI jobs now set `AGOV_BUILD_COMMIT`/`AGOV_BUILD_DIRTY` via the
  `.rust-base` template `before_script`. Previously only the dist/release job
  set these; test/clippy/deny/parity jobs fell back to git at compile time,
  and a warm cache could serve a stale binary with `(unknown dirty)` provenance.

## [2.2.1] — 2026-06-24

### Fixed

- Rust workspace version (`tooling/rust/Cargo.toml`) was `0.1.0` and had not
  been synced to the AGF framework version. `agov --version` now reports
  `2.2.1` matching `agf/VERSION`.

## [2.2.0] — 2026-06-24

### Added

- **Readable coordination IDs** — `agov coord brief` now defaults to the
  readable `YYYYMMDDThhmmssZ-<role-slug>-<4char>` format. The explicit
  `--id-format readable` flag remains valid. ULID format is still accepted
  for legacy compatibility.

- **Version provenance** — `agov --version` now includes the git commit SHA
  and dirty-flag at build time. `agov version --format json` emits a
  machine-readable provenance record. Cross-compilation CI jobs embed
  `AGOV_BUILD_COMMIT`/`AGOV_BUILD_DIRTY` to produce correct provenance in
  release archives.

- **`agov coord graph-index`** — read-only command that emits a JSON graph
  of the coordination channel: nodes, causal edges (`references-to`),
  pending handoffs, and invalid files.

- **`agf/templates/AGENTS.md`** — adopter template for the project root
  `AGENTS.md` loader (Codex / GitHub Copilot). Includes cold-start commands,
  an AGF version reference comment, rule orientation, coordination channel
  guidance, and CUSTOMIZE markers. Copy to project root after adoption.

- **`agov.validate.assistant_loader`** — new advisory in the umbrella
  `agov validate`. Warns when no loader file exists (`AGENTS.md` or
  `.claude/CLAUDE.md`) and when a loader's `<!-- AGF: X.Y.Z -->` version
  comment is stale relative to `agf/VERSION`. Advisory only; never blocks.

### Changed

- `agf/ADOPTING.md` §1.4 step 2 now names specific starter templates per
  assistant surface: Codex → `agf/templates/AGENTS.md`; Claude Code →
  `agf/project-specific-examples/python-flask/CLAUDE.md`.

- `ADOPTING.md` §2.4 prescribes the `YYYYMMDD-` filename prefix convention
  for ADR files.

- Parity test exclusion list extended to cover `assistant_loader` (Rust-only,
  parallel to the existing `assistant_drift` exclusion).

### Documentation

- RC-1 readiness audit completed: operator path verified, all v1.x legacy
  material labelled, three-layer contract documented for `agov coord
  graph-index`.

- Adopter learnings from amber-register (issues #9 and #10) incorporated into
  `agf/learnings/foundry.yaml`.

## [2.1.6] — 2026-06-22

### Fixed

- `agov validate` now reports advisory drift when copied `.claude/rules/` or
  `.claude/skills/` trees differ from canonical `agf/rules/` or `agf/skills/`
  content, including summarized output for large drift sets and remediation
  guidance that preserves `agov` as optional tooling.
- Adoption and upgrade documentation now explain the loader migration path for
  existing copied assistant-rule adopters: keep `.claude` files as small
  loaders, remove copied AGF content, and move intentional project-specific
  assistant rules outside copied AGF trees.
- `agov validate` now includes a coverage note clarifying that plan and
  coordination records are validated separately with `agov plan validate` and
  `agov coord validate`, or by equivalent manual review.

## [2.1.5] — 2026-06-22

### Added

- GitLab tag release pipelines now publish a Windows x64 Rust `agov` archive
  (`agov-X.Y.Z-x86_64-pc-windows-gnu.zip`) alongside the existing Linux
  tooling archives, with checksum and provenance files.

### Changed

- Release creation now uploads and links `agov` zip archives as first-class
  GitLab release assets.
- The release-artifacts reference page now documents the Windows zip package
  and distinguishes Linux `.tar.gz` tooling archives from Windows `.zip`
  tooling archives.

## [2.1.4] — 2026-06-22

### Changed

- Framework source release archives are now built by the Rust `agov` CLI via
  `agov dist validate`, `agov dist build`, and `agov dist check-tag`.
- GitLab `agf:dist:*` CI jobs now use the Rust tooling path for manifest
  validation, preview archives, and tag-only release archives.
- The release-artifacts reference page now describes framework source archives
  as the canonical versioned framework snapshot and Rust `agov` archives as the
  execution surface.

### Fixed

- Release documentation no longer presents Mix-backed source packaging as the
  current release implementation.

## [2.1.3] — 2026-06-22

### Changed

- Public README and documentation homepage now provide a three-minute entry
  path: problem, mechanism, cost, exit path, and one next step before deeper
  tooling and evidence material.
- The external evaluation page now treats the homepage as the first-scan entry
  and separates first-scan, quick, and thorough assessment tracks.
- CI now rebuilds the public documentation site when release metadata changes
  and runs Rust `agov validate` as release-facing framework validation.

### Fixed

- Release-tag CI coverage now emphasizes Rust tooling checks and no longer runs
  the Elixir test suite for release tags or CI-only changes.

## [2.1.2] — 2026-06-22

### Added

- Rust `agov` tooling release archives for supported targets, with GitLab
  release links and package-registry upload wiring.
- Public documentation for the first-run assistant loader, tooling and token
  savings, coordination protocol, released-artifact references, and
  principal-bound AI delegation as a v2.x docs-only convention.
- Readable coordination IDs and supporting `agov coord brief` guidance for new
  AI-authored coordination messages.
- Dependency CVE scanning for Rust tooling with `cargo-deny`.

### Changed

- Public documentation now presents the v2.1 line as released, clarifies the
  canonical GitLab Pages URL, and incorporates external-review follow-up from
  the Lumo review path.
- Coordination, learning, and release guidance now emphasize timestamp-based
  identifiers, session-resume hygiene, and the release-via-MR tag workflow.
- Adoption guidance now better separates advisory governance gates from
  structural CI/tooling gates and gives more realistic governor-effort ranges.

### Fixed

- Session-resume invalid-file summaries and learning ID handling were repaired
  across Elixir and Rust tooling paths.
- Documentation build and release metadata behavior were tightened, including
  release-info git environment fallbacks and stricter public-doc wording.

### Note

- `2.1.1` was merged as a release bump but no `v2.1.1` GitLab release was
  created. `2.1.2` is the next immutable release and includes the merged
  `2.1.1` changelog entry plus subsequent main-branch work.

## [2.1.1] — 2026-06-18

### Changed

- `release-via-merge-request-rule.md`: steps 7–8 (tag + push) now explicitly
  require no additional confirmation — the merged release MR is the authorization.
- Public README and documentation pages now describe v2.1 as the released
  documentation line while keeping evidence-limited rule status visible where
  it applies.

## [2.1.0] — 2026-06-18

### Added

- Coordination validator now emits an advisory WARN when a message contains
  the deprecated `seq` field. Detection is schema-driven via `x-deprecated: true`
  on the `seq` property — any future deprecated field needs only a schema change.
- `agf/schemas/coordination-message.schema.json`: `id` (ULID) is now required;
  `seq` marked deprecated. Conflict-free `COORD-YYYYMMDD-XXX` filename convention
  adopted for all coordination templates and documentation (issue #2).
- GitLab issue templates (Bug Report, External Review, Learning, Rule Proposal)
  and MR templates (Default, Learning Record, Rule or Agent) adopted from v1.

## [2.0.4] — 2026-06-18

### Fixed

- CI: `release:create` job failed with `curl: not found` on Alpine sh.
  The `release-cli:latest` image does not include `curl`; added
  `apk add --no-cache curl` as the first script step.

## [2.0.3] — 2026-06-18

### Fixed

- CI: `release:create` job failed with `syntax error: unexpected redirection`
  on Alpine sh. Removed `2>/dev/null` from the `for`-`in` glob list; the
  `[ -f "$f" ] || continue` guard inside the loop already handles unmatched
  globs.

## [2.0.2] — 2026-06-18

### Fixed

- Test: `DistManifestTest` integration test hardcoded `"2.0.0"` as the
  expected version, causing `elixir:test` to fail on every version bump.
  Fixed by reading `agf/VERSION` dynamically at test time.

## [2.0.1] — 2026-06-17

### Fixed

- CI: add `release:create` job to upload release archives to the GitLab generic
  package registry and create a GitLab release entry with download links.
  `v2.0.0` shipped without this job; archives were only accessible as CI job
  artifacts.

## [2.0.0] — 2026-06-16

First release of AGF v2. Restructures the framework into the canonical
`agf/` adopter-facing directory and ships a GitLab CI/CD pipeline with
framework source archives, a linux-x86_64 tooling binary, and a combined
release bundle.

### Added — CI/CD and release artifacts

- Root `.gitlab-ci.yml` with six jobs:
  - `elixir:test` — compile with `--warnings-as-errors` and run `mix test`
    on every MR, branch, and tag.
  - `agf:dist:validate` — validate the manifest-driven release manifest.
  - `agf:dist:preview` — build framework source archives as review artifacts
    on MRs and the default branch (1-week expiry).
  - `agf:dist:release` — build `agf-2.0.0.{tar.gz,zip}` with SHA-256
    checksums and provenance JSON on tag pipelines only; tag/VERSION
    mismatch blocks production before any upload.
  - `agov:tooling:preview` / `agov:tooling:release` — build the linux-x86_64
    Mix release archive (`agov-2.0.0-linux-x86_64-otp-29.tar.gz`) with
    SHA-256 and provenance. Release job runs on tags only; guarded by
    `mix agov.dist.check_tag`.
  - `agf:bundle:preview` / `agf:bundle:release` — assemble
    `agf-2.0.0-bundle.tar.gz` wrapping the framework archives and tooling
    binary with `SHA256SUMS` and `bundle-provenance.json`. Release job
    runs on tags only, after both `agf:dist:release` and
    `agov:tooling:release`.
- `mix agov.tooling.build` — builds the ERTS-bundled linux-x86_64 Mix
  release; derives OTP major version at build time; checks for forbidden
  paths (`agf/`, `.env`, `.git/`, `tmp/`, `agents-govern/`) before
  archiving.
- `mix agov.bundle.build` — assembles the bundle archive from framework
  archives already in `dist/`; fails cleanly if tooling archive is missing.
- Tooling binary provenance includes `otp_version`, `elixir_version`,
  `target_platform`, and `packaging` fields.

### Added — framework content

- `docs/site/` — 16-page documentation site (MkDocs, strict mode):
  marketing lane (landing, value proposition, use cases with explicit
  non-use cases) and a progressive learning path
  (start → adopt → concepts → operate → advanced → evolution), including
  pre-adoption `downstream-impact` and `exit-and-decommission` pages.
- `docs/adopt/decommission.md` — full downgrade/pause/remove procedure
  with verification steps.
- `docs/reference/quick-reference.md` — one-page roles, pipeline,
  coordination kinds, and key rules summary.
- `docs/academic/governing-the-governors.tex` — working paper describing
  the framework's design science foundations, formative evaluation, and
  design goals.
- New rules in `agf/rules/`: `tool-authority-boundary-rule`,
  `active-branch-preflight-rule`, `answer-only-rule`,
  `coordination-sequence-guard-rule`, `coordination-message-rule`,
  `deputy-governor-rule`, `downstream-impact-rule`, `gate-self-test-rule`,
  `visible-artifact-verification-rule`, `file-generator-rule`,
  `boundary-shape-testing-rule`, `concept-modelling-rule`,
  `ci-trigger-scoping-rule`, `migration-generation-rule`,
  `merge-request-description-rule`, `release-via-merge-request-rule`,
  `merge-request-workflow-rule`.
- New agents: `academic-peer-reviewer`, `academic-visual-reviewer`,
  `build-engineer`, `capability-developer`, `contribution-auditor`,
  `deployment-engineer`, `documentation-writer`, `fact-checker`.
- New skills: `capability-review`, `contribution-audit`, `health`,
  `infosec-screen`, `learning`, `release`, `upgrade`.

### Changed (structure only — no rule or agent behaviour changes)

- Repository root reduced to essential files; governance documents moved
  under `docs/governance/`.
- Contract documents consolidated under `docs/contracts/`.
- Adoption material consolidated under `docs/adopt/`.
- `framework_version` in all agent definitions and `contract_version` in
  `framework.yaml` set to `2.0.0`.

### Deferred (not in 2.0.0 scope)

- macOS, Windows, and ARM tooling binaries.
- Package registry publishing, release signing, SBOM.
- Design system, semantic colour palette, and Graphviz figure sources.
