---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 01KVJZBMN8FRGPZZYPAQDAW3MF
timestamp: 2026-06-20T16:55:42Z
immutable: true
references-to: []
topic: rust-agov-port-gate1
requires-ack: false
artifacts_owned:
  - docs/plans/rust-agov-port.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Gate 1 review of docs/plans/rust-agov-port.md — approve, reject, or request changes.
  must_read:
    - docs/plans/rust-agov-port.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 1 handoff — rust-agov-port

Submitting `docs/plans/rust-agov-port.md` for Gate 1 review.

## Context

The current `agov` CLI is implemented in Elixir and ships a bundled ERTS
runtime (~50–70 MB, Linux x86_64 only). The plan proposes a Rust port to:

- Produce statically-linked binaries of 5–12 MB across Linux x86_64/ARM64 and
  macOS x86_64/ARM64 (Windows deferred).
- Replace `rayon` data parallelism for the file-validation hot path instead of
  OTP process-per-file startup.
- Phase the port: Phase 1 covers all validation commands; Phase 2 covers
  channel analysis (`coord status`, `session resume`, etc.).
- Keep the Elixir tooling fully operational throughout and retire it only
  after a separate future decision.

## Advisory consultations included in the plan

- **infosec-sentinel**: crate supply chain, cross toolchain, static linking,
  CI secrets for multi-platform release uploads.
- **contribution-auditor**: new `tooling/rust/` subtree and workspace layout.
- **build-engineer**: new CI build matrix (`cross`, musl, 4 platform targets),
  path-scoped triggers.

## What Gate 1 approval authorises

- Creation of `feat/rust-agov-phase1` branch.
- Implementation of the `tooling/rust/` workspace covering Phase 1 validation
  commands only.
- Addition of `rust:tooling:test`, `rust:tooling:clippy`, and
  `rust:tooling:build:preview` CI jobs.

It does not authorise Phase 2 implementation, bundle rewiring, or removal of
the Elixir tooling — those require separate Gate 1 or Gate 2 decisions.
