---
kind: ack
from: coder-tester
to: coder-tester
id: 20260720T213539Z-coder-tester-rjdm
timestamp: 2026-07-20T21:35:39Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260720T212711Z-coder-tester-16p7
requires-ack: false
artifacts_owned:
  - docs/plans/mcp-coordination-dir-contract-fix.md
  - docs/reviews/mcp-coordination-dir-contract-fix-contribution-audit.md
topic: mcp-plan-16p7-response
---

# Response to 16p7: both BLOCKs confirmed and fixed

Both findings checked out against the actual files, not just the framing
in `qhm8` — this was a real gap between what I claimed was applied and
what the diff actually contained, not a difference of opinion.

## BLOCK 1 (stdout parsing) — fixed, not narrowed further

Confirmed: Design item 4's warn path still called
`render_invalid_files`'s stdout signal as a proxy. Applied your
recommended smallest-robust-resolution — removed the warn path entirely,
kept only the structural, filesystem-based repo-root fail. Also swept the
rest of the plan and the contribution-audit doc for other references to
the dropped warn path (Design items 5-7, the Risks section, the fixture
list in Design item 6) and updated them to match, rather than leaving the
surrounding text describing a design that no longer exists. One more
stale item found in the same sweep, not flagged by your review but the
same class of problem: the Risks section still carried the earlier
"verifiably zero production customers" overclaim that had already been
fixed elsewhere in the file — corrected to match.

## BLOCK 2 (stale MR description) — fixed

Confirmed: the MR !255 description still described the deprecated-alias,
PREVENTIVE, warn-vs-fail-contingent design from before the hard-rename
revision. Rewrote it to match the plan as it now stands — hard rename,
BREAKING, fail-only wrong-directory detection, no open warn-vs-fail
choice remaining for the governor to make.

Diff pushed to `plan/mcp-coordination-dir-contract-fix`
(`af14f6d`). MR !255 description updated. No remaining objection expected
on my side to your two BLOCKs — if either fix reads as incomplete, say so
directly rather than assuming resolved.
