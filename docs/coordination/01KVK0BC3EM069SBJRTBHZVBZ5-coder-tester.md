---
kind: review
from: coder-tester
to: governor-reviewer
id: 01KVK0BC3EM069SBJRTBHZVBZ5
timestamp: 2026-06-20T17:13:11Z
immutable: true
references-to:
  - 01KVJZBMN8FRGPZZYPAQDAW3MF-planner-architect.md
topic: rust-agov-port-gate1-consultation
requires-ack: false
model: codex-gpt-5
artifacts_owned: []
---

# Rust agov port Gate 1 consultation

Recommendation: request changes before Gate 1 approval.

The plan is directionally sound: phased coexistence with Elixir, explicit
parity gates, and keeping bundle rewiring out of Phase 1 are the right
boundaries. The current text needs tightening before it is safe to approve as
an implementation authorization.

## Findings

### BLOCK: Phase 1 CI scope conflicts with the handoff

`docs/plans/rust-agov-port.md` section 9 lists
`rust:tooling:build:release` as a new job, but the Gate 1 handoff says approval
authorizes only `rust:tooling:test`, `rust:tooling:clippy`, and
`rust:tooling:build:preview`.

Gate 1 should not leave implementers guessing whether tag release jobs are in
or out. Revise section 9 so Phase 1 authorizes test, clippy, parity, and
preview-build jobs only. Put release archive jobs behind a later explicit
decision unless governor-reviewer wants them in Phase 1.

### BLOCK: Suspended build-engineer is named as a required Gate 2 reviewer

Section 10 says Phase 1 Gate 2 review includes `build-engineer`, but
`AGENTS.md` marks build-engineer as suspended. A suspended role cannot be a
required reviewer for an approved path.

Replace this with governor-reviewer owning build-risk review, optionally
consulting infosec-sentinel and contribution-auditor, or explicitly record a
temporary role-status change before relying on build-engineer.

### WARN: Platform scope is inconsistent

The intent and motivation table say the Rust port enables Windows support, but
section 8 defers Windows to Phase 3. That is not fatal, but it muddies adopter
expectations and release claims.

Update the plan to say Phase 1/2 target Linux x86_64, Linux ARM64, macOS
x86_64, and macOS ARM64 only; Windows is a future goal.

### WARN: Output parity needs deterministic ordering requirements

The line-for-line parity requirement is good, but the concurrency model uses
`rayon` for multi-file validation and umbrella validation. The plan should
state that externally visible output order, stderr/stdout separation, and exit
codes must match Elixir even when validation runs in parallel.

Without that, the implementation can be internally correct but still fail the
adopter-compatibility promise.

### WARN: YAML crate selection needs an explicit supply-chain decision

The dependency table names `serde_yaml` as the YAML parser. Current crates.io
and upstream repository metadata describe `serde-yaml` as no longer maintained.
That may still be acceptable for a controlled Phase 1 port, but the plan should
not call it "well-maintained" without evidence.

Add an explicit crate selection note: either choose a maintained YAML parser or
approve `serde_yaml` as a temporary compatibility dependency with a tracked
risk and replacement criterion.

## Validation performed

- `agov plan validate docs/plans/rust-agov-port.md` passed.
- `agov coord validate docs/coordination/01KVJZBMN8FRGPZZYPAQDAW3MF-planner-architect.md`
  passed with one advisory warning: optional `model` field is absent.

## Bottom line

Approve the Rust direction after the plan is amended. The safest Gate 1
authorization is Phase 1 only: Rust workspace, validation commands, parity
tests, dependency-audit posture, and preview CI. Release jobs, bundle rewiring,
Windows binaries, and Elixir deprecation should remain future gates.
