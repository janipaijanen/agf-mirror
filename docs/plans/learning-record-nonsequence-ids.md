---
id: learning-record-nonsequence-ids
status: approved
topic: learning-record-nonsequence-ids
gate1_approved_by: 20260621T145946Z-planner-architect-h4cj
gate1_approved_at: 2026-06-21T14:59:46Z
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >
      The plan changes identifier policy, validators, and documentation. It
      does not introduce new secret handling or external data exposure.
  - role: contribution-auditor
    status: consulted
    reason: >
      The change affects governance evidence integrity and precedent handling
      for learning records. Contribution audit should review the identifier
      policy, migration stance, and advisory-versus-blocking validator behavior.
---

# Plan: Non-Sequential Learning Record IDs

**Date:** 2026-06-21T14:52:57Z
**Branch:** chore/learning-id-policy
**Status:** Gate 1 approved with conditions

## Intent

Replace shared-counter learning identifiers such as `L021` for new learning
records with independently generated, time-sortable unique identifiers.

The approved policy:

> Historical `L###` learning IDs remain stable references. New AI-authored
> learning records MUST use timestamp IDs in the form `YYYYMMDDThhmmssZ`, with
> an optional `-<4-char>` suffix when same-second disambiguation is needed.

## Background

Learning records previously used project-local sequence IDs (`L001`, `L018`,
etc.). This has the same structural flaw as sequence-based coordination
filenames: agents working asynchronously can choose the same next ID from stale
state, especially across branches or resumed sessions.

The immediate trigger was the governor-reviewer correction on 2026-06-21:
learning records must use identifiers other than sequences because of the high
collision risk. Planner-architect recorded Gate 1 approval with conditions in
`docs/coordination/20260621T145946Z-planner-architect-h4cj.md`.

## Approved Format

Canonical new learning IDs:

- `20260621T150000Z`
- `20260621T150000Z-a3fx`

Rejected forms:

- `learn-20260621T150000Z-some-slug-a3fx`
- newly allocated `L###` sequence IDs

Historical `L###` entries remain valid and must not be renamed merely for
aesthetic consistency, because they may already be cited by plans, rules, docs,
and coordination records.

## Affected Files

| File | Change type | Description |
|------|-------------|-------------|
| `agf/learnings/foundry.yaml` | MODIFY | Document the transition from historical `L###` IDs to timestamp IDs |
| `agf/schemas/learning.schema.json` | MODIFY | Document `id` as an explicit field and allowed transitional formats |
| `tooling/elixir/lib/agov/validate_learnings.ex` | MODIFY | Validate ID format and duplicate IDs |
| `tooling/elixir/test/agov/validate_learnings_test.exs` | MODIFY | Cover timestamp, historical, invalid, and duplicate IDs |
| `tooling/rust/crates/agov-validators/src/learnings.rs` | MODIFY | Mirror Elixir learning ID validation and tests |
| `agf/skills/learning/SKILL.md` | MODIFY | Stop instructing agents to find the next sequence ID |
| `agf/skills/README.md` | MODIFY | Update skill summary |
| `agf/templates/README.md` | MODIFY | Document timestamp ID convention for projects that use IDs |
| `agf/docs/site/**` | MODIFY | Update adopter-facing learning ID guidance |

## Test Strategy

1. `agov validate learnings` passes for historical `L###` IDs and timestamp IDs.
2. Duplicate learning IDs in one file produce a blocking validation finding.
3. Non-canonical `learn-...` IDs produce a blocking validation finding.
4. Elixir tests cover accepted, duplicate, and invalid ID cases.
5. Rust validator tests cover the same accepted, duplicate, and invalid ID
   cases.
6. MkDocs strict build passes after documentation updates.

## Risks

Medium. Learning IDs are cited across rules, docs, plans, and coordination
records. The migration must preserve existing references and avoid a noisy
rename of historical records.

## Rollback

Revert the policy/tooling/docs commits. Historical IDs remain untouched. Any
timestamp IDs already committed remain valid strings.

## Downstream Impact

EDITORIAL with validator implications. Adopters can keep existing learning IDs.
New framework guidance steers them toward collision-resistant timestamp IDs
without requiring a bulk migration.
