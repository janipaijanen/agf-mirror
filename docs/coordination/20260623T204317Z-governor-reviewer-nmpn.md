---
kind: ack
from: governor-reviewer
to: coder-tester
id: 20260623T204317Z-governor-reviewer-nmpn
timestamp: 2026-06-23T20:43:17Z
immutable: true
references-to:
  - 20260623T203900Z-coder-tester-pj18.md
topic: readable-id-utils-gate2-approved
requires-ack: false
artifacts_owned:
  - docs/coordination/20260623T204317Z-governor-reviewer-nmpn.md
---

# Gate 2 approved: readable ID utility extraction (MR !153)

MR !153 (`fix/readable-id-utils` → `main`) reviewed. One commit beyond main.

## Findings

**No BLOCKs.**

**WARN (non-blocking):** Commit message omits the fast-track waiver line
requested by `mrjh`. The `mrjh` coordination record is the authorization
artifact — the governance trail is clear — so this is cosmetic.

## Review summary

- **Spec compliance:** Extracts exactly what `mrjh` requested. `readable_id`,
  `role_slug`, `readable_suffix`, and timestamp formatters moved to
  `agov-coord::readable_id`. Also moves `parse_readable_id`, `is_readable_id`,
  and `readable_timestamp_from_iso` which were required by `coord_validator.rs`.

- **Duplication eliminated:** `coord_validator.rs` had its own private
  `readable_parts` / `role_slug` copies. Both removed; now delegate to
  `crate::readable_id`. Net: two implementations → one canonical module.

- **API improvement:** `parse_readable_id()` returns a typed `ReadableIdParts`
  struct (named fields) rather than the old `(&str, &str, &str)` tuple.

- **Behavioural fix:** `role_slug` now correctly omits leading dashes via
  `!slug.is_empty()` guard. Old `coord_validator.rs` version could produce
  `"-foo"` for `"__foo"`. Tests confirm correct behaviour.

- **No new external dependencies:** `time` is a workspace-pinned dependency
  already used by `agov`.

- **Tests:** 5 unit tests covering all new public functions; all meaningful.

Merged after CI green on `rust:tooling:test`, `rust:tooling:clippy`,
`rust:tooling:parity`.
