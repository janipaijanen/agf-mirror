---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260620T222851Z-planner-architect-828q
timestamp: 2026-06-20T22:28:51Z
immutable: true
model: claude-sonnet-4-6
references-to:
  - 20260620T221104Z-planner-architect-fg3x.md
topic: tooling-first-ids-implementation
requires-ack: false
artifacts_owned:
  - docs/plans/agf-tooling-first-ids.md
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Implement the tooling-first coordination ID policy per docs/plans/agf-tooling-first-ids.md
  must_read:
    - docs/plans/agf-tooling-first-ids.md
    - agf/learnings/foundry.yaml (L019, L020)
    - docs/coordination/20260620T221104Z-planner-architect-fg3x.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 1 approved — implement on feat/tooling-first-ids

MR !34 merged to main at c25e7d1 (2026-06-20T22:25Z). Gate 1 approved.

## Branch

`feat/tooling-first-ids` — created from main at c25e7d1. This is the
implementation branch.

## Tasks to implement

All tasks from `docs/plans/agf-tooling-first-ids.md`:

1. **`agf/rules/workflow/coordination-id-rule.md`** (NEW) — formalise the
   tooling-first ID policy with per-format filename rules. Policy wording:
   > AI agents MUST use AGF tooling to generate coordination IDs unless tooling
   > is unavailable. Humans SHOULD use the tooling. Manually authored IDs remain
   > valid when they pass schema and uniqueness validation.

2. **`AGENTS.md`** (MODIFY) — add the policy paragraph and a filename
   convention table distinguishing readable-ID (`<id>.md`) from ULID
   (`<ulid>-<role>.md`) format filenames.

3. **Generator fix** — `agov coord brief --id-format readable` currently
   outputs `<id>-<role>.md` filenames. Correct to `<id>.md`. The role is
   already embedded in the readable ID stem; the suffix is redundant.
   Exact path to fix: in `tooling/` (Elixir or Rust — confirm by tracing
   the `coord brief` command).

4. **Validator advisory** — `agov coord validate` should emit an advisory
   WARN (non-blocking, exit 0) when a coordination file's `from:` field
   contains an agent role AND the `id:` does not match the readable
   timestamp pattern AND does not match the ULID pattern. This catches
   AI-authored manual IDs.

5. **Gate self-test** (per `gate-self-test-rule.md`) — include a negative
   test demonstrating that the advisory fires on a known-bad input. The
   test must produce visible output and be reproducible from the MR
   description. Do NOT skip this.

## Already done (on main)

- L019 and L020 learning records in `agf/learnings/foundry.yaml` ✓

## Notes

- Learning records are already on main; do not add them again.
- The two wrong-suffix historical files
  (`20260620T214350Z-coder-tester-g66b-coder-tester.md`,
  `20260620T215241Z-planner-architect-v4gw-planner-architect.md`)
  are immutable — no rename or migration needed.
- Advisory infosec-sentinel and contribution-auditor consultations are
  flagged in the plan frontmatter. Consult before Gate 2 submission if
  either raises concerns.
