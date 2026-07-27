---
artifacts_owned: []
from: governor-reviewer
id: 20260727T083727Z-governor-reviewer-gqpe
immutable: true
kind: decision-record
model: claude-sonnet-5
references-to:
- 20260727T083527Z-governor-reviewer-rqnp.md
- 20260727T083143Z-governor-reviewer-strh.md
requires-ack: false
timestamp: 2026-07-27T08:37:27Z
to: all
topic: mdbook-migration-sequencing
---

# Sequencing resolved: finish #70/#71 on MkDocs now, mdBook migration is separate future work

Under Gate 1 authority, resolving the open sequencing question from
`20260727T083143Z-governor-reviewer-strh`:

## Decision

- **MR !699** (issue #70 runbook, MkDocs-built, tested, pipeline green)
  proceeds to human Gate 2 review as-is. It is release-blocking v2.4.5
  content per prior governor direction; holding fully completed,
  tested work to wait on an undecided toolchain migration is not
  justified.
- **Issue #71 Slice C** (vendor the runbook into the tooling archive)
  proceeds against the MkDocs-produced path
  (`agf/docs/site/docs/operate/poll-and-coordinate.md`) once !699
  merges, per the already-approved plan.
- **The mdBook migration** (Ayu theme confirmed, `SUMMARY.md`/Rust-Book
  content-structure adaptation proposed) is **not** v2.4.5 scope. It
  becomes a separate, dedicated Gate 1 plan, to be written and
  proposed once v2.4.5 ships or capacity allows -- candidate for the
  v2.5.0 portfolio discussion already tracked in issue #57 / MR !661.

## Why this default

Two release-blocking items are fully implemented and independently
verified; discarding or freezing that work to wait on an undecided
tooling swap would waste completed, tested effort for no correctness
gain. A future mdBook migration will need to touch
`agf/docs/site/docs/operate/poll-and-coordinate.md`'s *rendering*
regardless of when it happens -- the content itself does not need to
be re-authored, only re-hosted. That re-hosting cost is bounded and
worth paying once, later, rather than blocking two approved release
items now.

## What this does NOT decide

- This is not Gate 2 approval of !699 -- that remains the human
  governor's decision.
- This does not approve an mdBook migration Gate 1 plan -- none has
  been written. `20260727T083143Z-governor-reviewer-strh`'s theming
  and structure proposal remains a proposal until a plan file exists
  and is approved.
