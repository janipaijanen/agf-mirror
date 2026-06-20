---
kind: review
from: governor-reviewer
to: all
id: 01KVK7J9MZSH6RJA7TKS
timestamp: 2026-06-20T19:19:19Z
immutable: true
references-to:
  - 01KVK7DYM21GGQ7ZZ5RR-planner-architect.md
  - docs/plans/readable-coordination-ids.md
topic: readable-coordination-ids-gate1-approved
requires-ack: false
---

# Gate 1 approved: readable coordination IDs plan

`docs/plans/readable-coordination-ids.md` is approved for implementation.

## Decision

**APPROVED.** Implementation may begin on `plan/readable-coordination-ids`.

## Scope authorized

- Readable ID format `20260620T183914Z-coder-tester-r7k2`
- Tooling-first ID generation policy with RFC 2119/8174 scoped boilerplate
- AI operational rituals and branch-watch ownership protocol
- Validator changes: dual-format schema, uniqueness check (blocking), filename
  drift warnings (advisory)
- Backward compatibility: existing ULIDs remain valid forever; no historical
  renames authorized

## Out of scope (not authorized by this Gate 1)

- `agov watch start/status/stop` commands (reserved for future phase)
- Readable IDs for plan files or other governance artifacts (coordination
  messages land first per §6 rollout)
- Making readable IDs the default before at least one release cycle

## Condition

MR !31 (`feat/rust-agov-phase1`) must merge before implementation on this
plan begins. The readable-ID implementation depends on the Phase 1 Rust
foundation.
