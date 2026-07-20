---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260720T200017Z-coder-tester-7jgh
timestamp: 2026-07-20T20:00:17Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260720T193934Z-coder-tester-hpk6.md
  - 20260720T192551Z-coder-tester-rp33.md
requires-ack: false
artifacts_owned:
  - docs/plans/mcp-coordination-dir-contract-fix.md
topic: mcp-coordination-dir-contract-gate1
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review docs/plans/mcp-coordination-dir-contract-fix.md for Gate 1 approval.
    - Decide the scope-expansion question explicitly — fix all six dir-taking MCP tools uniformly, or only agov_session_resume as literally named in hpk6.
    - Decide warn-vs-fail for the likely-wrong-directory case (plan recommends warn, to stay PREVENTIVE rather than BREAKING; note the reclassification consequence if fail is chosen instead).
    - If approved, hand implementation to coder-tester (Codex or Claude, whichever picks it up next).
  must_read:
    - docs/plans/mcp-coordination-dir-contract-fix.md
    - docs/coordination/20260720T193934Z-coder-tester-hpk6.md
  do_not_touch: []
---

# Gate 1 plan ready: MCP coordination-directory contract fix

Responds to Codex's Gate 1 routing request (hpk6), itself responding to
the governor's direct instruction to fix the trust-relevant MCP defect
after the Sol High assessment merged (MR !251, `b967b50`).

## Summary

`docs/plans/mcp-coordination-dir-contract-fix.md` proposes:

- Rename the public `dir` field to `coordination_dir` on all six MCP tools
  that currently forward it to CLI `--dir` (not just `agov_session_resume`
  as literally named in hpk6 — the identical defect exists in
  `agov_handoff_next/take/complete`, `agov_coord_post`, `agov_coord_list`).
  Flagged explicitly as a scope decision for governor sign-off, not decided
  unilaterally.
- Keep `dir` as a deprecated, still-functional alias — zero migration
  required for existing callers.
- Add a structural (not prose-buried) warning field on `agov_session_resume`
  specifically when the resolved directory looks wrong (zero valid
  messages + invalid files present).
- Echo resolved directory provenance in the response.
- Fixtures covering omitted/correct/repo-root/absolute/nonexistent/
  alternate-layout paths, plus a CLI/MCP semantic parity test.
- Doc and changelog updates; an amendment pointer added to the stalled
  `docs/plans/agov-mcp-server.md` (not using its stalled `pending-gate1`
  status as authorization, per hpk6's explicit instruction).

Classified **PREVENTIVE** (three-circle downstream-impact statement
included in the plan), contingent on choosing "warn" over "fail" for the
misuse case — the plan states explicitly that choosing "fail" instead
would require reclassifying as BREAKING with a migration note, so that
decision isn't made silently during implementation.

## What I need from you

1. Approve, request changes, or reject the scope-expansion (six tools vs.
   one) and warn-vs-fail decisions named above.
2. If approved, this becomes ready for coder-tester implementation —
   either Codex (already engaged on this topic) or this session, whichever
   is available next.
