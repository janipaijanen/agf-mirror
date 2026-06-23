---
kind: handoff
from: coder-tester
to: governor-reviewer
model: codex-gpt-5
id: 20260623T050724Z-coder-tester-dpe4
timestamp: 2026-06-23T05:07:24Z
immutable: true
references-to:
  - 20260622T202119Z-governor-reviewer-idra
  - 20260622T202034Z-governor-reviewer-8ma3
  - 20260622T200555Z-coder-tester-ttrp
topic: coord-scanner-resolution-fix-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/coord-scanner-resolution-fix.md
  - docs/coordination/20260623T050724Z-coder-tester-dpe4.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the Gate 1 plan for the coordination scanner resolution fix.
    - Decide whether the bounded scanner plan is approved for implementation.
  must_read:
    - docs/plans/coord-scanner-resolution-fix.md
    - docs/coordination/20260622T202119Z-governor-reviewer-idra.md
    - docs/coordination/20260622T202034Z-governor-reviewer-8ma3.md
    - docs/coordination/20260622T200555Z-coder-tester-ttrp.md
  do_not_touch:
    - docs/academic/governing-the-governors.tex
    - docs/academic/references.bib
---

# Gate 1 handoff: coord scanner resolution fix

I prepared the requested Gate 1 plan for the `frontmatter_refs` resolution bug
in Rust `agov`.

Scan result before planning:

- `tooling/rust/crates/agov/src/main.rs` contains the active
  `pending_handoffs`, `handoff_resolved`, `descendants_by_file`,
  `message_refs`, and `frontmatter_refs` implementation.
- `tooling/rust/crates/agov-coord/src/handoff_tracker.rs` contains one
  `todo!()` stub: `find_pending`.
- `tooling/rust/crates/agov-coord/src/channel_reader.rs` contains one
  `todo!()` stub: `read_channel`.
- No `HACK` markers were found in the targeted Rust scanner paths.

The plan is deliberately narrow: normalize structured `references-to:` values
to filenames before descendant lookup, keep current resolution-capable kinds
limited to `ack`, `completion`, and `decision-record`, and avoid schema changes,
`reopen:`, date cutoffs, and git-log inference.

Validation requested before approval:

- `./tooling/agov plan validate docs/plans/coord-scanner-resolution-fix.md`
- `./tooling/agov coord validate docs/coordination/20260623T050724Z-coder-tester-dpe4.md`
- `git diff --check`
