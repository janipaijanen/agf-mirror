---
id: agf-tooling-first-ids
status: pending-gate1
topic: tooling-first-coordination-ids
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >
      The plan adds coordination ID generation and validation policy, including
      advisory validation behavior. No credentials or secret handling are in
      scope, but InfoSec should confirm the advisory/manual-ID policy does not
      weaken audit integrity.
  - role: contribution-auditor
    status: consulted
    reason: >
      The plan changes workflow rules, AGENTS.md contributor guidance, tooling
      behavior, and learning records. Contribution audit should confirm the
      policy and implementation scope remain aligned with AGF governance.
---

# Plan: Tooling-First Coordination ID Policy

**Date:** 2026-06-20T22:10:23Z
**Branch:** plan/tooling-first-ids
**Status:** Gate 1 submitted

## Intent

Formalise the tooling-first coordination ID policy agreed between
planner-architect and coder-tester (ack `20260620T220515Z-planner-architect-6j0e`,
proposal `01KVK62WGAZAMKHDQD3DAXPM9R-coder-tester.md`).

The policy wording to enshrine:

> AI agents MUST use AGF tooling to generate coordination IDs unless tooling is
> unavailable. Humans SHOULD use the tooling. Manually authored IDs remain valid
> when they pass schema and uniqueness validation.

This plan also covers the filename-convention fix for `agov coord brief
--id-format readable` (L019) and adds a validator advisory for AI-authored
manual IDs.

## Background

- MR !32 (`plan/readable-coordination-ids`) merged to main on 2026-06-20.
  Readable coordination IDs (`YYYYMMDDThhmmssZ-<role-slug>-<4-char-suffix>`)
  are now the live ID format for AI agent coordination.
- The tooling-first policy should be retroactively applied as the normative
  pattern for the readable-ID implementation.
- Finding L019: `agov coord brief --id-format readable` currently appends
  a redundant `-<role>` suffix, producing `<id>-<role>.md` instead of the
  correct `<id>.md`. Two historical files with wrong suffixes are permanent
  on main and serve as the concrete motivation for the fix.

## Affected files

| File | Change type | Description |
|------|-------------|-------------|
| `agf/rules/workflow/coordination-id-rule.md` | NEW | Formalise tooling-first policy; per-format filename rules |
| `AGENTS.md` | MODIFY | Add tooling-first policy wording; correct filename convention table |
| `tooling/elixir/lib/agov/commands/coord.ex` (approx) | MODIFY | Fix `brief --id-format readable` filename output to `<id>.md` |
| `tooling/rust/src/` (coord validate path) | MODIFY | Add advisory warning when AI agent record has manually-authored ID |
| `agf/learnings/foundry.yaml` | MODIFY | Add L019 (filename convention bug) and L020 (coordination-resume discipline) |

> Note: exact Elixir/Rust paths to be confirmed by coder-tester during
> implementation. The tooling source is in `tooling/`.

## Migration needed

No schema migration. The two existing wrong-suffix files
(`20260620T214350Z-coder-tester-g66b-coder-tester.md`,
`20260620T215241Z-planner-architect-v4gw-planner-architect.md`) are immutable
historical artifacts — no rename or migration is required or possible.

## Test strategy

1. `agov coord brief --id-format readable --from planner-architect --kind note --topic test`
   must produce a file named `<id>.md` with no trailing `-planner-architect` suffix.
2. `agov coord validate <file>` on a file whose ID does not match the readable
   pattern (manually authored) must produce an advisory WARN (not a blocker).
3. All existing `agov coord validate` tests must continue to pass at exit 0 for
   well-formed coordination files.
4. Gate self-test (per `gate-self-test-rule.md`): run the validator on a
   deliberately wrong-suffix filename and confirm the advisory fires.

## Risks

Low. Changes are:
- Policy documentation (non-breaking for existing coordination files)
- A generator bug fix (output filename only; schema unchanged)
- A validator advisory (new WARN, not a new blocker)
- Learning record additions (additive only)

## Rollback

Revert the implementation commits. All coordination files committed before the
fix remain valid (the schema does not change). The advisory is non-blocking,
so reverting it has no operational impact.

## Downstream impact

EDITORIAL for adopters: the rule formalises existing best practice. The
filename fix is a generator-output change, not a schema change — adopters'
existing coordination files are unaffected.

## Decomposition (for coder-tester implementation)

1. **L019/L020 learning records** — add to `agf/learnings/foundry.yaml`
   (already done on this branch by planner-architect).
2. **coordination-id-rule.md** — new rule file with policy wording and
   per-format filename table.
3. **AGENTS.md update** — policy paragraph + filename table.
4. **Generator fix** — `agov coord brief --id-format readable` filename output.
5. **Validator advisory** — warn on AI-authored manual IDs (identify by: agent
   role in `from:` field; ID does not match the readable timestamp pattern;
   ID does not match the ULID pattern either).
6. **Gate self-test** — negative test demonstrating advisory fires (per
   `gate-self-test-rule.md`).

Tasks 2–6 may be implemented in one commit or split by task; all must be
present before Gate 2 submission.
