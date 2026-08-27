---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260827T060700Z-planner-architect-mcpg
timestamp: 2026-08-27T06:07:00Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260827T054800Z-planner-architect-relc.md
topic: mr806-mcp-registry-missing-convergence-tool
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - "Add an agov_coord_convergence MCP tool entry to tooling/rust/crates/agov-mcp/src/main.rs, matching the existing agov_coord_* pattern (see e.g. agov_coord_publication_check at line 691)."
    - "Confirm mcp_coordination_tool_registry_matches_live_cli_subcommands (crates/agov-mcp/tests/stdio.rs:606) passes on a fresh exact-head pipeline for MR !806."
  must_read:
    - tooling/rust/crates/agov-mcp/src/main.rs
    - tooling/rust/crates/agov-mcp/tests/stdio.rs
    - tooling/rust/crates/agov/src/main.rs
  do_not_touch:
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# MR !806: genuine test failure after branch refresh, not the CI-HISTORY-1 defect

Per `relc`, I refreshed MR !806 (`plan/agov-mcp-transport-protocol-
correctness`) against current `main` to pick up MR !1965's `GIT_DEPTH`
fix (merge commit `9b92658e`). The fresh exact-head pipeline
(`2795305745`) is genuinely red — `rust:tooling:test`, `script_failure`:

```
thread 'mcp_coordination_tool_registry_matches_live_cli_subcommands'
panicked at crates/agov-mcp/tests/stdio.rs:606:9:
live `agov coord` subcommand set implies MCP tool
agov_coord_convergence, but it is missing from the registry (live
subcommands: [..., "convergence"])
```

Root cause, confirmed by reading both files: MR !2041 (bounded review-
convergence workflow), merged to `main` earlier in this session and
unrelated to MR !806, added a new `agov coord convergence` subcommand.
`agov-mcp`'s coordination tool registry (`tooling/rust/crates/agov-mcp/
src/main.rs`, pattern at e.g. `agov_coord_publication_check`, line 691)
has no matching entry, and its own exhaustive parity test
(`stdio.rs:606`) correctly caught the gap the moment the branch was
refreshed past `convergence`'s introduction. This is the same class of
gap already learned from in MR !2045 and corroborated again in issue
#115's `roi.rs` correction — an unrelated CLI-surface addition
elsewhere in the tree that a cross-crate exhaustive-inventory test is
specifically designed to catch.

Not fixing this myself — it requires understanding `agov coord
convergence`'s actual argument contract (I see a `--series` flag
referenced at `agov/src/main.rs:3123`) to wire correct passthrough, not
a copy-paste addition. This is Rust implementation work; routing to
coder-tester per this project's normal division of labor.

MR !806's own content is otherwise unaffected — all other jobs in this
pipeline pass, including `rust:tooling:clippy`, `commercial-corpus:
validate`, and `agov:governance:validate`. Per this project's no-blind-
retry norm, not retrying the paid job. The branch is at exact head
`9b92658e` (merge-only commit; no other content changed) — please
branch your fix from there.
