---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260804T002719Z-contribution-auditor-rnp5
timestamp: 2026-08-04T00:27:19Z
model: gpt-5.4
immutable: true
references-to:
  - 20260804T001752Z-contribution-auditor-phpp.md
topic: v250-post-rc1-release-path-recommendation
requires-ack: false
artifacts_owned: []
---

# Recommendation: cut a bounded RC2; defer !1090 implementation

Scope: published `v2.5.0-rc.1`, current `main`, the approved v2.5.0 SoW and
RC1 publication plan, release manifest, and post-RC1 shipped-path delta.
Coverage: COMPLETE for the release-path decision requested by the governor.
Recommendation: proceed to a separately Gate 1-approved `v2.5.0-rc.2`
publication plan; do not implement MR !1090 Path A before stable 2.5.0.
Authority: recommendation only. The governor decides Gate 1 on the resulting
RC2 plan and retains tag/publication authority.

## Where Claude and Codex agree

Planner-architect's already-approved governing plans say:

- RC2 exists only if RC1's candidate changes; it must not be ceremonial.
- Any post-RC1 functional, schema, documentation, migration, test, or
  artifact-layout correction creates a new candidate and repeats the full
  exact-artifact matrix.
- Stable 2.5.0 otherwise ships the same functional candidate as the accepted
  RC, with only version/changelog/release metadata changing.

Codex independently reaches the same rule application. Since RC1, NASA
attribution MR !1105 changed three paths under `agf/docs/site/**`, an explicit
`include` glob in `agf/dist/release-manifest.yaml`. Final built from current
`main` would therefore not be the same documentation candidate as RC1.

## !1090 timing

MR !1090 Path A authorizes a new nonblocking validator diagnostic. It is not a
reproduced RC1 defect, is not required by #79/#80, and changes tooling behavior
after the candidate checkpoint. Including it would widen RC2 and restart
release validation for an unrelated governance enhancement. Merge the plan
after its approval-metadata review clears, but defer implementation to 2.5.1.

## Strongest counterargument

The NASA delta is documentation-only, independently fact-checked, and its MR
pipeline already built/validated the distribution. A new stable-publication
plan could explicitly amend the earlier candidate rule and repeat validation
without publishing another RC.

That shortcut is technically possible but rejected: it makes the published
RC cease to identify the final candidate, contradicts two approved plans, and
saves only one version ceremony while weakening the exact-artifact checkpoint
whose purpose was candidate identity. RC2 is justified by a real shipped
delta, not calendar or aesthetics.

## Minimum evidence-preserving observation

RC2 should repeat the existing exact-artifact matrix, not invent a broader
feedback programme: exact-head pipeline; strict documentation build; version
and tag guards; three-baseline framework-archive overlays; packaged CLI/MCP
parity; anonymous downloaded-asset/provenance verification; release/package/
OCI identity; and one bounded classification of external documentation review
comments. Only a reproducible shipped defect can expand RC2; enhancements move
to 2.5.1.
