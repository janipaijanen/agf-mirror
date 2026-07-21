---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260721T170850Z-planner-architect-3q4m
timestamp: 2026-07-21T17:08:50Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260625T090052Z-coder-tester-s8np
  - 20260721T165544Z-governor-reviewer-3h53
  - 20260721T170050Z-coder-tester-ard4
  - 20260721T170051Z-governor-reviewer-kmgn
topic: legacy-ulid-reference-compat-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/coord-scanner-resolution-fix.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the revised legacy coordination-reference compatibility plan for Gate 1.
    - Approve, reject, or request changes before Rust implementation begins.
  must_read:
    - docs/plans/coord-scanner-resolution-fix.md
    - docs/coordination/20260625T090052Z-coder-tester-s8np.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 1: legacy malformed-ID reference compatibility

The coordination cleanup proved that two historical handoffs cannot be cleared
by valid later acknowledgements because Rust reference normalization rejects
their exact 20-character ULID-like filenames before graph construction.

The existing pending scanner-resolution plan has been revised to the current
implementation and exact defect. The proposed fix is deliberately narrow:

- recognize the historical 20-character Crockford form for reference
  normalization only;
- never generate or advertise it as a current ULID;
- preserve sequence, valid 26-character ULID, readable-ID, bare, `.md`, and
  path-qualified behavior;
- keep resolution-capable kinds unchanged;
- add negative near-length/non-Crockford tests and a current-main queue check.

Validation passed:

- `agov plan validate docs/plans/coord-scanner-resolution-fix.md`;
- `git diff --check`.

No Rust implementation has started. Gate 1 approval is required first.
