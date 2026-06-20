---
kind: handoff
from: planner-architect
to: coder-tester
id: 01KVK7J9N2AW8DJZQAG2
timestamp: 2026-06-20T19:19:19Z
immutable: true
references-to:
  - 01KVK7J9MZSH6RJA7TKS-governor-reviewer.md
  - 01KVK77ACVM6TF8VKHZ5ZHJQ0T-coder-tester.md
topic: readable-coordination-ids-implementation-cleared
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Implement the readable coordination IDs plan per docs/plans/readable-coordination-ids.md
    - Wait for MR !31 (feat/rust-agov-phase1) to merge before starting; this plan
      depends on the Phase 1 Rust foundation
    - Implement in rollout order per §6 of the plan (schema + validator → generator
      → guidance → default consideration)
    - File Gate 2 handoff to planner-architect when implementation is ready for review
  must_read:
    - docs/plans/readable-coordination-ids.md
    - docs/coordination/01KVK7J9MZSH6RJA7TKS-governor-reviewer.md
  do_not_touch:
    - feat/rust-agov-phase1
---

# Implementation cleared: readable coordination IDs

Gate 1 approved (`01KVK7J9MZSH6RJA7TKS`). Implementation is authorized.

## Precondition

Wait for MR !31 (`feat/rust-agov-phase1`) to merge before beginning
implementation. The Rust workspace foundation is required. CI is passing;
merge is imminent.

## Implementation order (from plan §6)

1. Schema: update `agf/schemas/coordination-message.schema.json` to accept
   both ULID and readable ID formats.
2. Validators: uniqueness check (blocking), filename/frontmatter drift warning
   (advisory), backward compatibility for ULIDs.
3. Generator: readable ID support in `agov coord brief` (explicit option
   first; default after one release cycle).
4. Guidance: update AI contributor docs to prefer tooling-generated IDs.

## Out of scope for this implementation

- `agov watch start/status/stop` (future phase)
- Readable IDs for plan files or other artifacts (coordination messages first)
- Historical ULID record renames (never authorized)
