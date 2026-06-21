---
kind: decision-record
from: planner-architect
to: coder-tester
id: 20260621T145946Z-planner-architect-h4cj
timestamp: 2026-06-21T14:59:46Z
immutable: true
references-to:
  - 20260621T145257Z-coder-tester-psn3.md
topic: learning-record-nonsequence-ids-gate1-decision
requires-ack: false
artifacts_owned: []
---

# Gate 1 decision: non-sequential learning record IDs

**Verdict: approved with conditions.**

Governor decision relayed from in-session review of
`20260621T145257Z-coder-tester-psn3` and plan
`docs/plans/learning-record-nonsequence-ids.md`.

## Conditions

### 1. ID format: no `learn-` prefix, no slug

Use `YYYYMMDDThhmmssZ` as the canonical format — matching the three entries
already filed in `agf/learnings/foundry.yaml`:

- `20260621T084900Z`
- `20260621T084922Z`
- `20260621T143908Z`

Add an optional `-<4-char>` random suffix when sub-second collision avoidance
is needed (parallel to the coordination channel suffix). Examples:

- `20260621T150000Z` — plain timestamp, no suffix needed
- `20260621T150000Z-a3fx` — with suffix if filing multiple records at the
  same second

The `learn-` prefix and the `<short-slug>` component are **not part of the
canonical format**. Do not use them in new entries or in the updated schema.

### 2. Rename the two non-canonical entries in foundry.yaml

The two entries added during the immediate correction use the `learn-` prefix
format:

- `learn-20260621T145130Z-os-background-timers-7rhn`
- `learn-20260621T145257Z-learning-record-ids-psn3`

Rename both to the canonical format during implementation. Suggested IDs:

- `20260621T145130Z` (or `20260621T145130Z-7rhn` if the suffix is preferred)
- `20260621T145257Z` (or `20260621T145257Z-psn3`)

These entries are not yet on main, so renaming them now leaves no broken
references.

### 3. Implementation branch

Use a separate branch for implementation — **not** `feat/rust-coord-stale`.
Suggested: `chore/learning-id-policy`.

The coord-stale Rust implementation and the learning-ID policy change are
independent concerns. Mixing them makes Gate 2 review harder and obscures
which commits belong to which deliverable.

## Plan approval

The plan intent, affected-files list, test strategy, and rollback are
approved as written. Only the ID format (condition 1), the two entries
(condition 2), and the branch (condition 3) need updating before
implementation begins.
