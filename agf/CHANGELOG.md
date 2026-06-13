# Changelog — agents-govern 2.0

v2.0 entries only. The v1.x changelog is not part of the `agf/`
distribution.

## [2.0.0] — Unreleased (Provisional)

Structural restructure into the canonical `agf/` adopter-facing directory.

### Changed (structure only — no rule or agent behaviour changes)

- Repository root reduced to essential files; governance documents moved
  under `docs/governance/` with role-named files:
  `GOVERNANCE.md` → `docs/governance/governance.md`,
  `LEARNINGS.md` → `docs/governance/learning-system.md`,
  `LESSONS-LEARNED.md` → `docs/governance/retrospective.md`,
  `MANIFEST.md` → `docs/governance/manifest.md`,
  `DISCLAIMER.md` → `docs/governance/disclaimer.md`,
  `docs/ANONYMIZATION-NOTICE.md` → `docs/governance/anonymization-notice.md`.
- Contract documents consolidated under `docs/contracts/`
  (`binding-contract.md`, `integration-contract.md`, `contract-stability.md`).
- Adoption material consolidated under `docs/adopt/`
  (`compliance-profile.md`, new `decommission.md`).
- Cross-links in carried-over files repaired to the new paths; mentions of
  adopter-side files (an adopter's own `GOVERNANCE.md`) and historical
  narrative (`docs/governance/retrospective.md`, ADRs) intentionally
  unchanged.
- `framework_version` in all agent definitions and `contract_version` in
  `framework.yaml` set to `2.0.0`.

### Added

- `docs/site/` — 16-page documentation site (MkDocs, strict mode):
  marketing lane (landing, value proposition, use cases with explicit
  non-use cases) and a progressive learning path
  (start → adopt → concepts → operate → advanced → evolution), including
  pre-adoption `downstream-impact` and `exit-and-decommission` pages.
- `docs/build/` — site build configuration; `.gitlab-ci.yml` with
  validator, strict site build, and Pages publish jobs (pinned image and
  tool versions; path-scoped MR rules).
- `docs/adopt/decommission.md` — full downgrade/pause/remove procedure
  with verification steps.
- `docs/reference/quick-reference.md` — one-page roles, pipeline,
  coordination kinds, and key rules summary.

### Deferred (tracked, not in 2.0.0 structural scope)

- Design system, semantic colour palette, and Graphviz figure sources.
- `docs/paper/` academic PDF pipeline (Pandoc → TeX → latexmk).
- Marketing copy polish passes.

Each returns as its own Gate-1-scoped change (coordination seq 107).
