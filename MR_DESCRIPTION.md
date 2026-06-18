## Summary

- Add `"x-deprecated": true` vendor extension to `seq` property in coordination-message schema
- Surface advisory WARN at validation time when a coord message contains a deprecated field
- Remove legacy `seq:` from the two "valid note" test fixtures, which now model current best practice

## Design decisions

**Schema-driven detection** — `advisory_findings/1` loads the raw (unresolved) schema JSON and
warns for any top-level property with `"x-deprecated": true`. Adding a deprecated field in future
requires only a schema change; no Elixir code change needed.

**`x-deprecated` vendor extension** — JSON Schema Draft-07 has no native `deprecated` keyword
(added in Draft 2019-09). Using `x-deprecated: true` follows the vendor-extension convention and
is ignored by validators that don't know it.

**`load_raw_schema/0` alongside `load_schema/0`** — `ExJsonSchema.Schema.resolve()` transforms the
schema into an opaque struct; the raw JSON map is needed to inspect property metadata. Two separate
private functions keep the concerns clear.

## Test plan

- [x] `advisory_findings/1` with `seq` present → advisory contains "seq" and "deprecated"
- [x] `advisory_findings/1` with no `seq` → no deprecated advisory
- [x] `valid_note_with_seq.md` fixture (id + seq) → deprecated advisory
- [x] `valid_note_with_id.md` fixture (id only) → no deprecated advisory
- [x] Full suite: 204 passing (0 regressions)
- [x] `mix agov.coord.validate test/fixtures/valid_note_with_seq.md` → WARN + OK
