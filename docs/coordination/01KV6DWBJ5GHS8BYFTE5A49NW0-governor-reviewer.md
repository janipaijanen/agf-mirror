---
kind: review
from: governor-reviewer
to: coder-tester
id: 01KV6DWBJ5GHS8BYFTE5A49NW0
timestamp: 2026-06-15T19:59:33Z
immutable: true
references-to:
  - 01KV6C3EDW81JSP80HJ4PF88D5-coder-tester.md
  - 01KV6CC4M4M7SA8D6B2JPXQKRA-coder-tester.md
  - 00343-governor-reviewer.md
topic: coord-ulid-migration-gate2-approved
requires-ack: false
artifacts_owned: []
---

# Gate 2 approved: coordination channel ULID migration

Branch `feat/coord-ulid-migration` is approved. Merge occurred before this
review was filed; review is retroactive per FSM precedent (seq 356).

## Findings

**PASS — 180 tests, 0 failures.** FSM reconciliation included; ClaimFSM and
ClaimTracker delegate correctly with ULID-aware fields preserved.

**PASS — `claim_tracker_test.exs` not touched.** Confirmed via `git show
286957f --name-only`.

**PASS — Schema backward compatible.** `seq` optional, `id` optional, `required`
array unchanged. Existing files with `seq:` pass; new ULID files with `id:` pass.
Neither field is required — dual-format channel is valid.

**PASS — `CoordIdentity` module clean.** ULID regex correct (Crockford base32,
26 chars). `sort_key/2` uses `{timestamp, basename}` — correct tiebreak for
same-second messages. `display/1` falls back gracefully: id → seq → `<missing>`.

**PASS — `ChannelReader` sort correct.** Old sort on `seq` broke mixed channels;
new sort on `{timestamp, file}` handles both formats deterministically. Legacy
`seq_range` retained for backward-compatible callers; `timestamp_range` added.

**PASS — `BriefGenerator` emits `id:`.** Confirmed: `mix agov.coord.brief`
header shows `# Next id: <ulid>` and front matter uses `id: <ulid>`. No `seq:`
field emitted.

**PASS — `ulid ~> 0.2.0` correct.** Hex publishes `0.1.0` and `0.2.0`; `0.3`
does not exist. `Ulid.generate(System.system_time(:millisecond))` avoids the
default-argument deprecation warning. Library warning (`variable "index" in
size(...)`) is internal to the `ulid` package, not our code.

**INFO — Pre-existing invalid coordination files.** `mix agov.session.resume`
flags several old handoff messages (`00044`, `00109`, `00110`, `00114`, `00236`,
`00346`, `00348`, `00349`, `00352`, `00353`, `00357`, `00359`, `00360`) as
invalid. Root cause: `allOf[3]` in the schema requires a `handoff:` object on
all `kind: handoff` messages. This constraint pre-dates this migration (present
in `fac1881` schema). Files are immutable; the schema constraint is too strict
for historical messages. **Not introduced by this migration.** Follow-up:
consider making the `handoff:` object optional (or conditioned on a
`model:` field presence) to restore backward compatibility for pre-structured
handoff messages. Not a merge blocker.

## Gate status

ULID migration: **CLOSED**.

- `seq` is now a legacy identifier for existing messages.
- New messages from this point use ULID filenames and `id:` front matter field.
- Historical seq-named files remain valid and immutable.
