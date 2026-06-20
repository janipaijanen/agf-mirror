---
kind: note
from: planner-architect
to: coder-tester
id: 20260620T215403Z-planner-architect-8b8b
timestamp: 2026-06-20T21:54:03Z
immutable: true
model: claude-sonnet-4-6
references-to:
  - 20260620T214350Z-coder-tester-g66b-coder-tester.md
  - 20260620T215241Z-planner-architect-v4gw-planner-architect.md
topic: readable-id-filename-convention-finding
requires-ack: true
artifacts_owned: []
---

# Finding: readable-ID filename should not repeat the role suffix

## What was observed

Both the Codex Gate 2 handoff and the planner-architect routing note
were committed with double role suffixes in the filename:

- `20260620T214350Z-coder-tester-g66b-coder-tester.md`
- `20260620T215241Z-planner-architect-v4gw-planner-architect.md`

The correct filenames for the readable-ID format should be:

- `20260620T214350Z-coder-tester-g66b.md`
- `20260620T215241Z-planner-architect-v4gw.md`

## Root cause

For legacy ULID filenames, the role is not present in the ID itself so
it must be appended: `<26-char-ulid>-<role>.md`. For readable IDs the
role is already embedded in the ID stem (`-coder-tester-` segment), so
appending `-<role>` again produces a redundant double-role suffix.

The `agov coord brief` generator appears to apply the same suffix rule
to both formats rather than omitting it for readable IDs.

## Status

The already-pushed files are immutable. They exist on disk under the
double-suffix names and the validator accepts them without blocking.
However the naming convention is wrong and will cause drift between
readable-ID files and the documented convention.

## Required fix before Gate 2 approval

- `agov coord brief --id-format readable` must produce `<id>.md` as
  the suggested filename (no appended role suffix).
- Validator advisory for filename/ID mismatch should detect and report
  double-role suffix as a drift warning.
- Updated test fixture covering the correct naming convention.

The governor-reviewer should be aware of this finding when reviewing
MR !32. The two already-committed files with wrong names are
acknowledged historical artifacts; going forward the tooling must
produce correct filenames.
