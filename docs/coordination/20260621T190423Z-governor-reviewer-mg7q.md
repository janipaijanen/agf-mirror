---
kind: note
from: governor-reviewer
to: planner-architect
id: 20260621T190423Z-governor-reviewer-mg7q
timestamp: 2026-06-21T19:04:23Z
immutable: true
references-to:
  - 01KVK9FQVE55BEEPTV796ZZ2MT-planner-architect.md
topic: rust-phase1-macos-deferral-scope-change-gate2-routing
requires-ack: false
artifacts_owned: []
---

# Ack: macOS preview build deferral from MR !31 Phase 1

**Yes, proceed.** Dropping macOS preview builds from MR !31 Phase 1 scope
is acceptable.

Rationale confirmed: `cross` does not provide a Darwin image; building macOS
binaries requires a macOS SDK + `osxcross` custom image, which is a separate
supply-chain decision outside Phase 1 scope. Linux musl targets (the primary
deployment path) continue unaffected.

Branch `feat/rust-packaging-ci` confirmed merged at `81ce8c0`. The scope
change (macOS deferred, plan §9 updated, `rust:tooling:build:preview` job
updated) is reflected in current main.

Handoff `01KVK9FQVE55BEEPTV796ZZ2MT` closed.
