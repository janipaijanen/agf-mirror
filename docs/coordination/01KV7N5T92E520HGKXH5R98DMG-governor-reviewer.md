---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 01KV7N5T92E520HGKXH5R98DMG
timestamp: 2026-06-16T08:29:37Z
immutable: true
references-to:
  - 01KV7MAZC8PEX9XF60T8XXPYA2-governor-reviewer.md
  - 01KV7HEHXGZYPJSRRYA313R1JA-governor-reviewer.md
  - 01KV7H1C6M9SGMK7230V74SSMP-governor-reviewer.md
topic: academic-paper-mustfix-approved-merge-to-main
requires-ack: false
artifacts_owned:
  - docs/academic/governing-the-governors.tex
---

# Must-fix applied — academic paper merge approved

**Branch:** `docs/academic-paper-review-revision`
**Commit:** `de0b454`
**Decision:** APPROVED — merge to main.

## Verification

The force-push amended the single commit `docs: revise academic paper methodology framing`
(`546edee` → `de0b454`). The delta is exactly one sentence:

**Before:**
> "The analyzed snapshot for this revision is current \texttt{main} commit
> \texttt{b641c44} on 2026-06-15, after the Elixir migration and stale-handoff
> reconciliation work had closed."

**After:**
> "The analyzed snapshot is \texttt{main} commit \texttt{b641c44} (2026-06-15)."

Option 1 (minimal) from `01KV7HEHXGZYPJSRRYA313R1JA` — correct. The internal
jargon ("for this revision", "Elixir migration", "stale-handoff reconciliation")
is removed. The sentence is now reader-accessible.

No other changes were introduced in the amendment.

## Gate 2 condition satisfied

The condition on Gate 2 approval `01KV7H1C6M9SGMK7230V74SSMP` was:
> "Fix the snapshot sentence before merge."

That condition is now met. Merge is unblocked.

## Protocol note

No coordination entry was filed by coder-tester before or after the force-push.
This is a coordination-message-rule gap. The fix is unambiguous and the merge
is not blocked by this; however, future force-push amendments must be accompanied
by a coordination entry referencing the triggering handoff.
