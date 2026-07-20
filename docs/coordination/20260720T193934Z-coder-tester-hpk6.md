---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260720T193934Z-coder-tester-hpk6
timestamp: 2026-07-20T19:39:34Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260720T191010Z-coder-tester-5kj3.md
  - 20260720T192551Z-coder-tester-rp33.md
topic: mcp-session-resume-path-contract-gate1
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Produce and route a Gate 1 plan for the MCP coordination-directory contract fix described below.
    - Treat this as urgent because the governor directed Codex to fix it after the Sol High assessment merged.
  must_read:
    - docs/reviews/agf-framework-assessment-codex-sol-high.md
    - tooling/rust/crates/agov-mcp/src/main.rs
    - agf/docs/site/docs/adopt/mcp-server.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 1 needed: make MCP session-resume scope explicit and safe

The governor directed Codex to fix the trust-relevant MCP issue after the Sol
High assessment merged as MR !251. The merge completed successfully at
`b967b50d39ecf958b2dbb86c8c3494614a794fe2`.

## Confirmed defect

`agov_session_resume.dir` is forwarded to CLI `--dir`, where it means the
coordination-message directory rather than repository root. Passing the
repository root returns plausible `ok: true` output after scanning root
Markdown files. Passing `docs/coordination` restores CLI/MCP semantic parity.

This is not an algorithmic state divergence. It is a high-severity public tool
contract, documentation, UX, provenance, and acceptance-test defect because a
reasonable caller mistake silently queries the wrong evidence domain.

## Requested plan scope

1. Rename the public MCP field to `coordination_dir`, retaining `dir` as a
   deprecated compatibility alias if the MCP schema/runtime permits safe
   aliasing.
2. Describe the field explicitly as the coordination-message directory,
   normally `docs/coordination`, and explicitly say it is not repository root.
3. Preserve deterministic omission behavior using `docs/coordination`.
4. Return or surface resolved provenance: repository root, coordination
   directory, selected binary/version, base branch, and relevant schema/config
   source where supported by the existing CLI contract.
5. Warn or fail when a supplied path looks like repository root and contains no
   valid coordination records.
6. Add fixtures for omitted, correct, repository-root, absolute, nonexistent,
   and alternate-layout paths.
7. Add a direct CLI/MCP semantic parity test and the MCP-only lifecycle test
   promised by `docs/plans/agov-mcp-server.md`.
8. Update `agf/docs/site/docs/adopt/mcp-server.md` and the Unreleased changelog.

The existing `docs/plans/agov-mcp-server.md` remains `pending-gate1` despite
the MCP implementation having shipped into the source tree. Do not use that
historical governance gap as authorization for this new public-contract
change. Produce a new focused plan or an explicitly governed amendment, obtain
governor-reviewer approval, and then hand implementation to coder-tester.
