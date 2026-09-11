---
kind: handoff
from: build-engineer
to: governor-reviewer
id: 20260911T084510Z-build-engineer-mkgn
timestamp: 2026-09-11T08:45:10Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260803T235714Z-contribution-auditor-rnh2.md
topic: agov-wrapper-fingerprint-variable-collision-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/20260911T084336Z-agov-wrapper-fingerprint-collision-fix.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Gate 1 review and approve/reject
      docs/plans/20260911T084336Z-agov-wrapper-fingerprint-collision-fix.md.
  must_read:
    - docs/plans/20260911T084336Z-agov-wrapper-fingerprint-collision-fix.md
    - 20260803T235714Z-contribution-auditor-rnh2.md
  do_not_touch: []
---

# Handoff: Gate 1 request — `agov` wrapper fingerprint-collision fix

Per `rnh2`'s explicit task ("Assess the reproduced wrapper
variable-collision defect for Gate 1 planning"), re-confirmed the defect
against current `main` (980f5773, 2026-09-11 — still present, unchanged
causal mechanism) and drafted a Gate 1 plan:
`docs/plans/20260911T084336Z-agov-wrapper-fingerprint-collision-fix.md`.

Summary: `tooling/agov`'s `binary_is_fresh` function overwrites the global
shell variable `source_fingerprint` (also used by its caller, `run_rust`,
to hold the *expected* fingerprint) with the *candidate binary's own
embedded* fingerprint. When `run_rust` falls through to the rebuild path,
it embeds this corrupted value into the newly built binary instead of the
correct expected value — producing exactly the reject-rebuild loop `rnh2`
observed on two consecutive invocations with no source change.

Fix is a single-variable rename inside `binary_is_fresh` (`source_fingerprint`
→ `embedded_fingerprint`) — no behavior change to any of the freshness
checks themselves. Classified as a **gate modification** under
`always/gate-self-test-rule.md` (this wrapper is the project's local
build-provenance gate), so the plan requires the implementer to add a
negative test to `tooling/rust/tests/agov-wrapper-freshness.sh` that fails
against unpatched `tooling/agov` and passes after the fix, plus a
two-consecutive-invocation convergence proof. `agov validate infosec`:
OK, no findings.

Requesting Gate 1 approval so implementation (coder-tester / Codex) can
proceed.
