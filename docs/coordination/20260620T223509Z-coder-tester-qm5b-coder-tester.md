---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260620T223509Z-coder-tester-qm5b
timestamp: 2026-06-20T22:35:09Z
immutable: true
references-to:
  - docs/coordination/20260620T222804Z-coder-tester-25sq-coder-tester.md
  - docs/coordination/01KVKBF4M1CVSH3JBR9F0C226P-coder-tester.md
topic: rust-tooling-maturity-collaboration
requires-ack: false
artifacts_owned: []
model: codex-gpt-5
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Review MR !35 for whether the Rust tooling direction is sufficient for AGF-NG dogfooding.
    - Prioritize the remaining Mix-only commands needed before Rust can be the default framework-development tooling path.
    - Recommend whether the next slice should port session resume, coord visibility/status/stale, token ROI reporting, or dist/release checks first.
    - Include any MCP/agent/sub-agent implications from the earlier capability-design handoff if they affect the tooling maturity target.
  must_read:
    - MR !35
    - docs/plans/rust-agov-port.md
    - docs/coordination/20260620T222804Z-coder-tester-25sq-coder-tester.md
    - docs/coordination/01KVKBF4M1CVSH3JBR9F0C226P-coder-tester.md
    - tooling/agov
    - tooling/rust/crates/agov/src/main.rs
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Handoff: collaborate on Rust tooling maturity target

The governor-reviewer asked Codex to collaborate with Claude and target making
the Rust implementation mature enough for framework development.

Current coder-tester status:

- MR !35 exists: `feat: prefer Rust agov for common tooling commands`.
- Rust now covers the wrapper-preferred path for:
  - `coord validate`
  - `coord brief`
  - `plan validate`
  - `validate`
  - `handoff next`
- Rust still does not cover every command AGF-NG uses during development:
  - `session resume`
  - `coord visibility`
  - `coord status` / `coord stale`
  - token ROI report commands
  - dist/release manifest and packaging checks

Collaboration ask:

Please review the maturity target from the planner-architect side and recommend
the next Gate 1/implementation slice. The practical target is not abstract
feature parity; it is making Rust good enough that AGF-NG agents can use it as
the default tooling path while developing the framework itself.
