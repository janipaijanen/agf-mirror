---
id: issue97-vscode-adopter-doc-accuracy
status: pending-gate1
topic: issue97-vscode-adopter-doc-accuracy
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: "Review 20260731T180442Z-contribution-auditor-g1sr returned CLEAR and confirmed the three defects, evidence boundary, telemetry treatment, independent-adopter gap, and scope containment. This revision incorporates its explicit-author-attribution advisory; exact implementation text still requires review before Gate 2."
  - role: infosec-sentinel
    status: not-applicable
    reason: "Public prose only: no credentials, private paths, learning payloads, executable behavior, network trust boundary, authorization semantics, or security controls change. Telemetry is summarized as aggregate counts and no raw records are published."
---

# Plan: correct adopter prompt and Visual Studio Code evidence

## Decision requested

Approve a bounded public-documentation correction for issue #97, as a
candidate v2.4.11 patch. Correct one version-stale commissioning prompt and
describe the author-operated Claude and Codex integrations in Visual Studio
Code at the evidence strength actually supported by sustained foundry use.

This plan does not authorize the separate MkDocs-to-mdBook/Ayu migration,
theme or navigation work, a version bump, tag, release publication, tool
behavior, schema, validator, role, or adopter-contract change.

## Current defects

1. The copyable commissioning prompt says `Use the AGF v2.0 distribution I
   have provided.` First person is appropriate because an adopter pastes the
   prompt, but the hard-coded version is stale and the distribution location
   is underspecified.
2. The public positioning page says foundry runs on Claude Code alone and
   discusses only Codex CLI. The author confirms sustained AGF and AGOV
   development through both Claude and Codex integrations in Visual Studio
   Code.
3. The contribution page asks whether non-Claude tools work without
   distinguishing the available author-operated Codex evidence from the still
   missing independent-adopter corroboration.

## Approved wording contract

The commissioning prompt will say exactly:

> Use the pinned AGF distribution I have made available in this repository,
> either as a release archive or an extracted `agf/` directory. Do not fetch
> or substitute a different version.

Compatibility prose will state, without implying independence or universal
editor/version coverage:

> The project author has developed AGF and AGOV through sustained use of both
> the Claude and Codex integrations in Visual Studio Code. The integrations
> have been used for governed planning, implementation, validation, review,
> and release work.

The contribution page will continue asking for independent experience with
Codex, Cursor, Windsurf, and other assistants. It must frame that request as
independent corroboration, not as an assertion that author-operated Codex
compatibility is unknown.

## Evidence and claim boundary

- The human author/governor directly confirms the two Visual Studio Code
  integrations and their use while developing AGF and AGOV.
- The durable Codex AGOV telemetry contains at least 1,844 records across 34
  workflows from 2026-06-21 through 2026-07-31. It covers coordination,
  validation, plan, handoff, session-resume, distribution, and release-check
  paths.
- The telemetry identifies AGOV command transports as CLI or unspecified; the
  author confirmation, not telemetry inference, establishes the Visual Studio
  Code surface.
- This evidence is **Empirically informed** and author-operated. It is not an
  independent adoption study, a controlled comparison, or proof of identical
  behavior across every assistant, editor version, or repository.
- Token-savings ranges from AGOV telemetry are estimates rather than observed
  counterfactual measurements and will not be promoted into the compatibility
  claim.

## Implementation surfaces

- `agf/docs/site/docs/adopt/install-and-layouts.md`
- `agf/docs/site/docs/concepts/position.md`
- `agf/docs/site/contribute.md`
- other public pages only when an exact search finds a directly conflicting
  Claude-only or Codex-CLI-only statement; any expansion must remain a wording
  correction and be enumerated in the implementation MR.

## Acceptance evidence

1. The commissioning prompt uses the approved text and no longer pins v2.0.
2. Public compatibility prose explicitly attributes the evidence to the
   project author and names both Claude and Codex integrations in Visual Studio
   Code and the actual governed workflows.
3. The foundry page no longer says Claude Code is the sole development path.
4. The independent-adopter request remains visible and clearly independent.
5. No claim says the telemetry itself proves the IDE surface, universal
   compatibility, equal behavior, or independent validation.
6. Exact-text fact-check and contribution-auditor review return no blocking
   finding before Gate 2.
7. Strict public-site build, link checks, adopter-doc validation, relevant
   prose searches, and `git diff --check` pass.

## Stop conditions

Stop and return to Gate 1 if implementation would change the documentation
toolchain or theme, add a compatibility guarantee, publish raw telemetry,
claim independent validation, alter shipped commands or loaders, or expand
into the mdBook/Ayu migration.

## Release boundary

Implementation may be grouped into a v2.4.11 documentation batch only after
separate Gate 1 approval and exact-head Gate 2 review. Issue #97 closes only
after the correction is published and verified on the public Pages site.
