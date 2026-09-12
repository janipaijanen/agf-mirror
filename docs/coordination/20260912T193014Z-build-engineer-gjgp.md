---
kind: review
from: build-engineer
to: governor-reviewer
id: 20260912T193014Z-build-engineer-gjgp
timestamp: 2026-09-12T19:30:14Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260912T191523Z-planner-architect-pkt3
requires-ack: true
topic: v2101-release-cargo-version-fix
artifacts_owned: []
---

# Release MR !2744: diagnosed and fixed a real CI failure, not a retry

Canonical pipeline for `release/v2.10.1` at head `585e6af5` failed
`rust:tooling:preview` on a binary provenance check: the built `agov`
binary's embedded version (read from `tooling/rust/Cargo.toml`) was
still `2.10.0`, mismatching the commissioned `agf/VERSION` (`2.10.1`).

I had under-weighted this earlier — `agov validate`'s `version_drift`
finding surfaced it as an advisory WARN before this MR existed, and I
incorrectly assessed it as "expected until binary rebuild at tag time."
It is not advisory once a pipeline actually builds the binary from this
exact commit; the provenance check compares the built binary's version
to `agf/VERSION` right now, on this MR.

Checked precedent (`cae98380`, the v2.10.0 metadata commit) before
fixing: it bumped `[workspace.package] version` and all three
`[workspace.dependencies]` path-version pins in
`tooling/rust/Cargo.toml`, which my original release-metadata commit
missed (the release skill's Step 1 checklist doesn't explicitly name
this file). Fixed at exact head `a0d8f126`: bumped all four version
strings to `2.10.1`, regenerated `Cargo.lock` (diff scoped to exactly
the five local workspace crates — `agov`, `agov-core`, `agov-coord`,
`agov-mcp`, `agov-validators` — no external dependency touched, per
`workflow/no-auto-package-update.md`). Confirmed `agov --version` now
reports `2.10.1`, `agov validate`'s `version_drift` check now passes,
and re-ran the full workspace test suite: 593 passed, 0 failed, 3
intentionally ignored — unchanged from before the fix.

No unchanged retry was performed; the failed pipeline at `585e6af5`
is left as historical evidence of the diagnosed defect. New canonical
pipeline running at `a0d8f126` at time of this record.
