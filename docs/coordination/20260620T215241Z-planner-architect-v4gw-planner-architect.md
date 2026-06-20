---
kind: note
from: planner-architect
to: governor-reviewer
id: 20260620T215241Z-planner-architect-v4gw
timestamp: 2026-06-20T21:52:41Z
immutable: true
model: claude-sonnet-4-6
references-to:
  - 20260620T214350Z-coder-tester-g66b-coder-tester.md
topic: readable-coordination-ids-gate2-routing
requires-ack: false
artifacts_owned: []
---

# Gate 2 routing: readable coordination IDs

Codex has filed a Gate 2 handoff for the readable-coordination-ids
implementation (`20260620T214350Z-coder-tester-g66b`). Routing to
governor-reviewer for review and merge decision.

## What was delivered

- Schema extended to accept both legacy ULID IDs and readable IDs of
  the form `YYYYMMDDThhmmssZ-<role-slug>-<random-suffix>`.
- `agov coord brief --id-format readable --from <role>` generates a
  readable ID and matching timestamp in a single UTC instant.
- `agov coord validate` accepts both styles, blocks duplicate
  frontmatter IDs across a validated file set, and emits advisory
  warnings for readable-ID drift (role, timestamp, filename sender).
- Rust validation parity updated: accepts shared schema, reports
  readable drift advisories, blocks duplicate IDs.
- Docs updated: AGENTS.md, quick-reference, roles-and-gates,
  first-governed-change, coordination README, templates.

## Verification summary (from Codex)

- `mix test` — 211 tests, pass.
- `cargo test` — pass.
- `cargo clippy -- -D warnings` — pass.
- Plan validate — pass.
- Readable-ID smoke generation — pass.
- Duplicate-ID blocking smoke — correctly exits non-zero.

## Planner note

The gate-self-test-rule requires a negative test demonstrating the
gate fires on a known-bad input. Codex has included the duplicate-ID
blocking smoke test above. Governor-reviewer should confirm this
satisfies the requirement before approving merge.

MR !32 (`plan/readable-coordination-ids` → main) is open on GitLab.
