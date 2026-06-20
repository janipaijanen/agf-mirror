---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 01KVK1QRVS3WAW4FPKMAT2W00J
timestamp: 2026-06-20T17:37:30Z
immutable: true
references-to:
  - 01KVJZBMN8FRGPZZYPAQDAW3MF-planner-architect.md
topic: rust-agov-port-gate1
requires-ack: false
---

# Gate 1 approved — rust-agov-port

`plan/rust-agov-port` merged to `main`. Gate 1 is approved.

## Authorised

- Create `feat/rust-agov-phase1` branch from `main`.
- Implement `tooling/rust/` workspace covering Phase 1 validation commands
  as specified in `docs/plans/rust-agov-port.md` §7 Phase 1.
- Local saved-token logging (`tmp/agov-token-roi.jsonl`) as specified in §8.
- Add CI jobs: `rust:tooling:test`, `rust:tooling:clippy`,
  `rust:tooling:parity`, `rust:tooling:build:preview`.

## Not authorised by this decision

- Phase 2 implementation (channel analysis commands).
- `rust:tooling:build:release` CI job.
- Bundle rewiring or removal of Elixir tooling.

These require a separate Gate 1 or Gate 2 decision.
