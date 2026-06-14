---
id: coordination-ulid-migration
status: pending-gate1
topic: coordination-message-ulid-identifiers
timestamp: 2026-06-14T20:50:25Z
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: no credentials, auth, network, or secret handling; pure file-naming and schema change
  - role: contribution-auditor
    status: pending
    reason: filename convention change affects every future coordination message; adopter migration guidance needed
---

# Plan: Coordination Channel ULID Migration

## Intent

Replace seq-based coordination filenames (`<zero-padded-seq>-<role>.md`) with
ULID-based filenames (`<ulid>-<role>.md`). Sequential seq numbers chosen
independently by concurrent agents collide when agents work in parallel without
real-time coordination. The collision that triggered this plan occurred on
2026-06-14 when two agents independently chose seq 320 for different messages.

ULIDs (Universally Unique Lexicographically Sortable Identifiers) are 26-char
Crockford base32 strings with a 48-bit millisecond timestamp prefix. They are:
- **Collision-proof**: independently generated without any counter
- **Time-sortable**: lexicographic sort = creation-time order
- **Short**: 26 chars vs 32/36 for UUID

The coordination channel already has graph structure via `references-to` (which
uses filenames as stable node IDs). Making filenames collision-proof completes
that structure.

## What does NOT change

- `references-to` — already filename-based; ULID filenames are still stable
  identifiers
- Immutability of existing files — no renaming of committed files
- Message body conventions and `kind` semantics
- Schema fields other than `seq` and the new `id`

## Affected files

### Schema
- `agf/schemas/coordination-message.schema.json`
  - Remove `seq` from `required` array (make optional for backward compat)
  - Add `id` property: `{ "type": "string", "pattern": "^[0-9A-HJKMNP-TV-Z]{26}$" }`
  - New messages: `id` required, `seq` absent
  - Old messages: `seq` present, `id` absent — both valid

### README and templates
- `docs/coordination/README.md`
  - Update filename convention: `<ulid>-<role>.md`
  - Update all front matter templates: replace `seq: N` with `id: <ulid>`
  - Remove "Sequence collisions are valid" note (structurally impossible with ULIDs)

### Elixir tooling

1. `tooling/elixir/mix.exs`
   - Add `{:ulid, "~> 0.3"}` to deps (Hex package; pure Elixir implementation)

2. `tooling/elixir/lib/agov/channel_reader.ex`
   - Line 28: replace `Enum.sort_by(fn {_file, data} -> data["seq"] end)`
     with `Enum.sort_by(fn {file, data} -> {data["timestamp"], file} end)`
   - Lines 33–41: replace `seq_range` computation with `timestamp_range`
     (min/max of `data["timestamp"]` strings — ISO 8601 sorts lexicographically)
   - Update returned map: `seq_range` → `timestamp_range`

3. `tooling/elixir/lib/mix/tasks/agov.coord.brief.ex`
   - Line 40: replace `%{next_seq: max + 1, ...}` with `%{next_id: Ulid.generate(), ...}`
   - Lines 26, 29: update default channel_info to use `next_id` instead of `next_seq`

4. `tooling/elixir/lib/agov/brief_generator.ex`
   - Line 28: update `build_header/1` pattern to use `next_id` and `timestamp_range`
   - Line 40: `"# Next seq: #{next_seq}\n"` → `"# Next id: #{next_id}\n"`
   - Line 43: update `build_front_matter/6` to accept `next_id`
   - Line 49: `"seq: #{next_seq}\n"` → `"id: #{next_id}\n"`

5. `tooling/elixir/lib/agov/session_resume.ex`
   - Lines 111, 117, 139, 146: replace `Enum.sort_by(& &1.seq, :desc)` and
     `Enum.sort_by(& &1["seq"], :desc)` with `Enum.sort_by(& &1.timestamp, :desc)`
     (or `& &1["timestamp"]` for map access)
   - Display: replace `seq N` with `id <first-8-chars>` in human-readable output
     (first 8 chars of ULID encode the timestamp; sufficient for display)

6. `tooling/elixir/lib/agov/claim_tracker.ex`
   - No filename-format dependency; processes messages by content only — no change

7. `tooling/elixir/lib/agov/handoff_tracker.ex`
   - No filename-format dependency; resolution via `references-to` values — no change

8. `tooling/elixir/lib/agov/coord_visibility.ex`
   - "Sequence collision" detection becomes vestigial for ULID-named files;
     keep the detection code for old-format files but suppress reporting for
     collisions that are structurally impossible (two files with identical ULID prefix)

### Tests
- `tooling/elixir/test/agov/channel_reader_test.exs` — new or updated
- `tooling/elixir/test/agov/brief_generator_test.exs` — new or updated
- `tooling/elixir/test/mix/tasks/agov_coord_brief_test.exs` — new or updated
- `tooling/elixir/test/agov/session_resume_test.exs` — update sort assertions

## Ordering strategy: timestamp over seq

**Old sort**: `Enum.sort_by(fn {_file, data} -> data["seq"] end)` — breaks on
mixed old+new files once new files have no `seq`.

**New sort**: `Enum.sort_by(fn {file, data} -> {data["timestamp"], file} end)`

- ISO 8601 timestamps (`YYYY-MM-DDTHH:MM:SSZ`) sort lexicographically in
  chronological order. This property holds regardless of filename format.
- Filename is the tiebreaker for same-second messages. For ULID files, the
  ULID prefix itself encodes millisecond order, so same-second tiebreaks via
  filename are still deterministic.
- Existing seq-named files have valid timestamps → sort correctly.
- Mixed channels (old seq + new ULID) sort correctly.

## Dependency

`ulid` on Hex (`{:ulid, "~> 0.3"}`). Pure Elixir, no NIFs. Requires explicit
user approval per `no-auto-package-update` rule.

Alternative: inline a ULID generator (≈20 lines of Elixir) to avoid the
dependency. Decision deferred to Gate 2 — either is acceptable.

## Migration strategy: dual-format, no big bang

1. **No rename of existing files.** All 337+ committed files stay as-is.
2. **Tooling reads both formats.** `channel_reader.ex` parses any `.md` file
   in the coordination directory regardless of naming convention. Ordering by
   timestamp unifies old and new.
3. **New files use ULID.** From the merge of this implementation branch,
   `mix agov.coord.brief` emits `id: <ulid>` instead of `seq: <n>`.
4. **Schema accepts both.** `seq` optional, `id` optional. No existing file
   fails validation.

## TDD order

Write each test to RED before implementing:

1. `ChannelReader` — parse fixture directory with mixed old-format (`00001-role.md`)
   and new-format (`01ABCDEFG...-role.md`) files; assert timestamp ordering
2. `BriefGenerator` — assert generated front matter contains `id:` matching
   ULID format; assert no `seq:` field
3. `mix agov.coord.brief` — integration: channel info contains `next_id` (valid
   ULID), header prints `Next id:`, front matter uses `id:`
4. `SessionResume` — assert pending handoffs / active claims sorted by timestamp
   descend correctly with mixed file formats
5. Schema validation — old message with `seq` and no `id`: passes; new message
   with `id` and no `seq`: passes; message with neither: check (decide whether
   to require at least one)

## Risks

- **`seq` display in session.resume output**: human-readable output currently
  says "seq 337  topic  [status]". Switching to first-8-chars ULID changes the
  display; agents referencing these in coordination messages will need to use
  filename instead of seq number. Acceptable — filenames are already the stable
  reference in `references-to`.
- **`seq_range` in channel summary**: currently "seq 1–337". New display would
  be "2026-06-14T... – 2026-06-14T...". Adjust wording in brief header.
- **Adoption ceremony**: first ULID-named file in the channel creates a visible
  break from the naming convention. File a coordination note at the time of
  first use to mark the transition.

## Out of scope

- Renaming existing coordination files
- Back-filling `id` into existing file front matter (immutable)
- Changes to `mix agov.coord.visibility` collision detection beyond
  suppressing false positives on ULID names
- Adopter migration (separate adopter-facing guidance after Gate 2)

## Handoff target

Coder-tester. Implementation branch: `feat/coord-ulid-migration`.

Gate 2 submission must include:
- [ ] All tests green (`mix test`)
- [ ] Schema validated against existing 337+ messages (none fail)
- [ ] Schema validated against a synthetic ULID-format message (passes)
- [ ] `mix agov.coord.brief` output shows `id:` in generated template
- [ ] `mix agov.session.resume` output correct with mixed fixture set
- [ ] `docs/coordination/README.md` updated (convention, templates, collision note removed)
