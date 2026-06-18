# Plan: Surface Advisory Warning for Deprecated `seq` Field

**Date:** 2026-06-18T11:19:25Z
**Branch:** feat/deprecated-field-detection
**Status:** approved

## Intent

Make the `seq` deprecation machine-readable and surfaced at validation time. Currently
`seq` is described as deprecated in the schema description string, but no tooling warns
when a message contains it. An agent or adopter running `agov coord validate` on an old
message that still has `seq:` should see an advisory WARN — the same non-blocking advisory
channel already used for the missing `model` field.

## Approach

Schema-driven detection via a vendor extension field `"x-deprecated": true` on the `seq`
property. `CoordValidator.advisory_findings/1` loads the raw (unresolved) schema JSON,
finds all top-level properties with `x-deprecated: true`, and emits an advisory for each
such field that is present in the message data.

This approach is forward-compatible: any future deprecated field just needs `x-deprecated: true`
in the schema; no code change is required.

## Affected files

1. `agf/schemas/coordination-message.schema.json` — add `"x-deprecated": true` to `seq` property
2. `tooling/elixir/lib/agov/coord_validator.ex` — add `check_deprecated_fields/2` to `advisory_findings/1`; add `load_raw_schema/0` private helper
3. `tooling/elixir/test/fixtures/valid_note_with_seq.md` — new fixture: valid message (has `id`) + legacy `seq`
4. `tooling/elixir/test/agov/coord_validator_test.exs` — new tests for deprecated advisory

## Migration needed

No. Schema change is additive (vendor extension key); existing consumers ignore unknown keys.

## Test strategy

- Unit: `advisory_findings/1` with inline map containing `"seq"` → 1 advisory for seq + 1 advisory for model (no model present)
- Unit: `advisory_findings/1` with `"id"` only, no `"seq"` → 0 deprecated advisories
- File-based: `valid_note_with_seq.md` (has `id` + `seq`, has `model`) → 1 deprecated advisory (seq), 0 model advisory
- Existing tests must not regress

## Risks

Low. No behaviour change for messages without `seq`. Advisory channel does not affect exit codes.

## Rollback

Revert the `x-deprecated` key from schema and remove the two private functions.
