---
id: radar-governance-signal-gaming
status: pending-gate1
topic: radar-governance-signal-gaming
advisory_consultation:
  - role: contribution-auditor
    status: not-applicable
    reason: >
      This is a narrow, reversible documentation classification that preserves
      the radar's non-binding status and adopts no countermeasure. Independent
      factual review remains part of Gate 2 evidence.
  - role: infosec-sentinel
    status: not-applicable
    reason: >
      The change adds public-source summaries only. It touches no credentials,
      private records, executable trust boundary, schema, validator, or CI.
---

# Plan: governance-signal gaming on the radar

## Decision requested

Authorize a documentation-only update that expands the existing
**Anti-rubber-stamping gate design** radar entry to include governance-signal
gaming and adds the two supporting articles to the classified source registry.

This plan does not adopt a countermeasure, create an issue, change a rule, or
alter validator or CI behavior.

## Why this belongs on the radar

The radar is the project's pre-commitment watchlist for unresolved,
controversial, or emerging ideas. Governance-signal gaming is a credible
extension of rubber-stamping risk: a system can appear to satisfy review or
benchmark controls while optimizing the visible signal instead of the intended
outcome. The available material is useful warning evidence, but it is not a
sound basis for prescribing one control. Keeping it on the radar preserves the
question, its controversy, and its review deadline without implying adoption.

## Planned changes

1. In `docs/radar.md`, expand the existing anti-rubber-stamping entry rather
   than create a duplicate. Name evaluator or benchmark mutation, mode
   switching, and optimizing visible approval signals as candidate failure
   modes. Keep candidate defenses explicitly provisional and retain the
   `2026-10` review date.
2. In `docs/sources.md`, classify the Core Auto article as practitioner
   evidence and threat-model material. State that its strongest contribution is
   the reported systems-code benchmark-gaming pattern; do not generalize the
   anecdote into prevalence or effectiveness claims.
3. In `docs/sources.md`, classify the Tom's Hardware article as secondary
   reporting and threat-model material. Preserve the reported Amazon cost and
   quality claims as attributed allegations, record Amazon's objection that the
   cited data was not representative, and do not treat the headline as an
   independently verified measurement.
4. Remove or close the corresponding `docs/TODO.md` item only if MR !952 has
   merged before implementation. If it remains open, leave its branch untouched
   and note that this plan supersedes its earlier deferral boundary.

## Required evidence and validation

- Re-read both articles at implementation time and preserve attribution and
  caveats; use no unsupported numerical extrapolation.
- Confirm there remains exactly one active radar entry for anti-rubber-stamping
  and governance-signal gaming.
- Confirm both source-registry entries use the registry's existing
  `type`/`relation`/`action` vocabulary.
- Run `git diff --check` and the repository documentation validation applicable
  to changes under `docs/`.
- Gate 2 review must compare the wording to the source pages and verify that
  candidate countermeasures are not stated as AGF requirements.

## Acceptance criteria

1. The radar explains the signal-gaming risk without duplicating the existing
   anti-rubber-stamping item.
2. The entry remains controversial, non-binding, and review-dated.
3. Core Auto is clearly identified as practitioner evidence, not a controlled
   study.
4. Tom's Hardware is clearly identified as secondary reporting, with Amazon's
   representativeness objection visible beside the allegation.
5. No rule, schema, validator, CI configuration, issue, release scope, or
   behavioral requirement changes.

## Implementation ownership and gate

After Gate 1 approval, coder-tester owns the documentation edit and evidence.
Governor-reviewer retains the binding Gate 2 merge decision. No implementation
may begin while this plan remains `pending-gate1`.
