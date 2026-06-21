# Changelog — agents-govern 2.0

v2.0 entries only. The v1.x changelog is not part of the `agf/`
distribution.

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
