---
id: learning-id-suffix-validator-enforcement
status: approved
topic: learning-id-suffix-validator-enforcement
gate1_approved_by: 20260623T120716Z-governor-reviewer-g1ap
gate1_approved_at: 2026-06-23T12:07:16Z
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >
      The plan changes learning identifier validation and documentation only.
      It does not add secret handling, external network access, or new data
      exposure.
  - role: contribution-auditor
    status: consulted
    reason: >
      Learning IDs are evidence references used across plans, rules, docs, and
      coordination. The plan preserves historical references while tightening
      new-record validation to reduce conflict risk.
---

# Plan: Learning ID Suffix Validator Enforcement

**Date:** 2026-06-23T11:51:46Z
**Branch:** `fix/learning-id-suffix-validator`
**Status:** Gate 1 approved

## Intent

Enforce the settled learning-ID policy in validators and schema guidance:

> New learning records use `YYYYMMDDThhmmssZ-<4-char-random>`. No new `L###`.
> No new timestamp-only IDs. Historical records remain valid and immutable.

This plan follows governor-reviewer direction in
`20260623T114836Z-governor-reviewer-lids` and Gate 1 approval in
`20260623T120716Z-governor-reviewer-g1ap`.

## Background

`docs/plans/learning-record-nonsequence-ids.md` moved AGF away from
sequence-only learning IDs, but it still allowed bare timestamp IDs with an
optional suffix. Subsequent coordination and learning evidence tightened the
policy: timestamp-only IDs can collide in asynchronous multi-agent work, so new
learning records need entropy as part of the identifier.

MR !129 updated live guidance and learning documentation, but intentionally did
not change validator behavior. This plan covers the validator-enforcement
slice.

## In Scope

- Update `agf/schemas/learning.schema.json` so the schema pattern accepts:
  - historical `L###`;
  - suffixed timestamp IDs: `YYYYMMDDThhmmssZ-[a-z0-9]{4}`.
- Remove timestamp-only IDs from the schema's accepted timestamp branch.
- Update Rust learning validator logic and tests in
  `tooling/rust/crates/agov-validators/src/learnings.rs`.
- Update Elixir learning validator logic and tests because the Elixir validator
  path still carries independent ID-format enforcement.
- Update `docs/plans/learning-record-nonsequence-ids.md` to state that the
  optional-suffix wording is superseded by the settled suffix-required policy.
- Preserve existing historical records by avoiding bulk renames.

## Out of Scope

- Renaming historical `L###` records.
- Renaming existing timestamp-only learning IDs already committed in
  `agf/learnings/foundry.yaml` or `agents-govern/learnings/foundry.yaml`.
- Changing coordination ID rules.
- Adding a new learning-ID generator command.
- Reworking the learning file layout.

## Historical Compatibility

The schema states the target policy:
`^(L[0-9]{3}|[0-9]{8}T[0-9]{6}Z-[a-z0-9]{4})$`.

Runtime validators use a transitional posture: bare timestamp learning IDs
emit non-blocking WARN findings rather than BLOCK findings. This grandfathers
existing records without hardcoding a list of known IDs and keeps validators
runnable on current learning files. BLOCK enforcement for bare timestamp IDs
can follow in a separate plan after known bare-timestamp records are migrated.

## Test Strategy

- `./tooling/agov validate learnings`
- `cargo test --manifest-path tooling/rust/Cargo.toml -p agov-validators`
- `cargo test --manifest-path tooling/rust/Cargo.toml`
- Elixir learning validator tests.
- Direct negative test fixture or unit test for timestamp-only IDs as WARN.
- Direct positive tests for historical `L###` and suffixed timestamp IDs.
- `./tooling/agov plan validate docs/plans/learning-id-suffix-validator-enforcement.md`
- `./tooling/agov coord validate <Gate 2 handoff>`

## Risk

Medium. This touches evidence identifiers and validators. The main risk is
blocking existing historical records by accident. The implementation should
prefer a narrow validator change with explicit tests over broad learning-file
rewrites.

## Rollback

Revert the validator/schema/doc commits. Historical learning records remain
unchanged.

## Downstream Impact

Adopters with existing `L###` or timestamp-only IDs keep their historical
records. New records should use suffixed timestamp IDs. Upgrade notes should
frame this as conflict prevention for asynchronous agents, not as a request to
rename old evidence.
