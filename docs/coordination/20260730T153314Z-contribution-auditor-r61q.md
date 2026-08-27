---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260730T153314Z-contribution-auditor-r61q
timestamp: 2026-07-30T15:33:14Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260730T152746Z-contribution-auditor-de8p
  - 20260730T150606Z-contribution-auditor-tsgk
topic: v246-release-complete-version-surface-audit
requires-ack: false
artifacts_owned:
  - docs/plans/v246-release.md
---

# Complete v2.4.6 version-surface audit supersedes six-file count

# Review evidence envelope

Scope: MR !834 at prior head
`8e46ac76c98f1a737a3bf0ad1e1852a55b185931`, plus all packaged `agf/`
surfaces, Rust workspace metadata, root assistant loaders, and release rules
Coverage: COMPLETE
Recommendation: BLOCK the prior six-file amendment; request Gate 1 only at the
corrected exact head containing this record
Blocking findings: 4, resolved in the accompanying plan correction
Advisory findings: 0
Open findings with accepted risk: 0
Authority: recommendation only; the governor-reviewer decides Gate 1

## Findings

### R1 — Agent framework versions omitted

`agf/skills/release/SKILL.md` requires every `agents/*.yaml`
`framework_version` to be bumped. All 12 shipped definitions still say 2.0.0,
and `agf/schemas/agent.schema.json` says release-time equality with VERSION is
required, although the current Rust validator checks only schema shape.

### R2 — Rust internal dependency requirements stale

`tooling/rust/Cargo.toml` declares workspace package version 2.4.5 but three
internal path-dependency requirements at 2.4.4. `cargo metadata --no-deps`
confirms that the five actual packages are 2.4.5 while their internal
requirements resolve as `^2.4.4`. Release commits before v2.4.5 updated these
requirements on every patch; v2.4.5 omitted them. The v2.4.6 release must
align all four Cargo.toml values to 2.4.6 while leaving external dependencies
and checksums unchanged.

### R3 — Copyable adopter values stale

`agf/ADOPTING.md` tells adopters to copy framework version 0.4.0 and agent IDs
that no longer match the shipped definitions. `agf/templates/learning-record.yaml`
still says 2.0.0-provisional. These are active instructions/templates, not
historical evidence.

### R4 — Live packaged manifests stale

`agf/agents/MANIFEST.yaml` labels the current framework 2.0.0.
`agf/docs/governance/manifest.md` calls 2.0.0 provisional/unreleased, calls
VERSION 2.0.0, and uses 2.0.0 in its upgrade example. Both files ship in the
framework archive. The latter's separate stale Elixir-path language remains
parked under issue #93 and is not pulled into this release.

## Values deliberately preserved

- Historical changelog and learning provenance, including the foundry's
  recorded agents-govern 1.1.50 bootstrap adoption.
- Agent definition `version` and `id` values; these identify definition
  revisions and are not the framework release scalar.
- Schema/contract/provenance compatibility versions.
- External package and tool versions, generated Graphviz comments, and
  synthetic test/failure examples.
- Tracked Claude example loaders and root AGENTS.md: neither contains a pinned
  AGF release scalar, so no version edit is required.

The corrected plan defines 22 release files and requires the Gate 2 evidence
to attach a complete semver classification so no older value is silently
assumed stale or silently ignored.
