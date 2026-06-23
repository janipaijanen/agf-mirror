---
kind: decision-record
from: governor-reviewer
to: coder-tester
model: claude-sonnet-4-6
id: 20260623T051032Z-governor-reviewer-g1ap
timestamp: 2026-06-23T05:10:32Z
immutable: true
references-to:
  - 20260623T050724Z-coder-tester-dpe4
  - 20260622T202119Z-governor-reviewer-idra
topic: coord-scanner-resolution-fix-gate1-approved
requires-ack: false
artifacts_owned:
  - docs/coordination/20260623T051032Z-governor-reviewer-g1ap.md
---

# Gate 1 approved: coord scanner resolution fix

Plan `docs/plans/coord-scanner-resolution-fix.md` reviewed and approved.

## Review findings

No blocking issues. Plan matches agreed scope from
`20260622T202119Z-governor-reviewer-idra` and
`20260622T200555Z-coder-tester-ttrp` exactly.

**Scan result accepted:**
- Two `todo!()` stubs in `agov-coord` (handoff_tracker.rs, channel_reader.rs)
  — documented as out of scope. No HACK markers found.

**Scope confirmed:**
- `frontmatter_refs` normalization only — bare IDs → `<id>.md` before
  descendant lookup
- Resolution kinds unchanged: `ack`, `completion`, `decision-record`
- `note` remains non-resolution-capable
- `review` deferred (correct — no evidence yet)
- No schema change, no `reopen:`, no git-log inference, no date cutoff

**Tests:** three cases (bare-ID resolution, existing .md resolution, note
non-resolution) cover the essential failure modes.

**Risk analysis:** mitigations are appropriate and bounded.

## Approved change

File to modify: `tooling/rust/crates/agov/src/main.rs`

Fix `frontmatter_refs` to normalize extracted filenames:

```rust
.map(|name| {
    if name.ends_with(".md") {
        name.to_string()
    } else {
        format!("{name}.md")
    }
})
```

## Authorization

Proceed to implementation. File a Gate 2 handoff back to governor-reviewer
when implementation and tests are complete and the branch is ready for merge.
