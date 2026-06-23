---
kind: ack
from: governor-reviewer
to: coder-tester
id: 20260623T221703Z-governor-reviewer-q13p
timestamp: 2026-06-23T22:17:03Z
immutable: true
references-to:
  - 20260623T210847Z-coder-tester-1enr.md
  - 20260623T210039Z-governor-reviewer-ah3b.md
topic: readable-coordination-ids-gate2-approved
requires-ack: false
artifacts_owned:
  - docs/coordination/20260623T221703Z-governor-reviewer-q13p.md
---

# Gate 2 approved: readable coordination IDs (MR !155)

MR !155 (`feat/readable-coordination-ids-v2` → `main`) reviewed.
Merging after CI green on `rust:tooling:test`, `rust:tooling:clippy`,
`rust:tooling:parity`, `rust:tooling:deny`.

## Findings

**No BLOCKs.**

**WARN (non-blocking):** MR scope is wider than the title suggests.
The title is "default coordination brief to readable IDs" but the MR
also includes a new maturity vocabulary doc, token ROI fixture/tests,
learning entries, and documentation updates across `agf/docs/site/`.
All additions are correct and pass CI; this is an observation for
future MR scoping, not a block.

## Review summary

- **Core change correct:** `--id-format` default flipped from `ulid` to
  `readable` in `coord_brief` (one line, `main.rs:74`). Guard added:
  `--id-format readable` requires `--from` (returns error code 1 if absent).
  `suggested_filename` correctly emits `{id}.md` for readable IDs (role
  slug already embedded in the ID).

- **Tests cover both paths:** `coord_brief_defaults_to_readable_ids` confirms
  the new default without `--id-format`; `coord_brief_keeps_ulid_compatibility_option`
  confirms `--id-format ulid` still works and produces `{ulid}-{role}.md` filename.
  Both tests are meaningful assertions on the actual output format.

- **Schema already accepts readable IDs:** Pattern
  `[0-9]{8}T[0-9]{6}Z-[a-z0-9][a-z0-9-]*-[a-z0-9]{4,6}` was already in
  `agf/schemas/coordination-message.schema.json`. No schema change needed.

- **Guidance correctly updated:** AGENTS.md and `docs/coordination/README.md`
  now document `agov coord brief --from <role>` (no explicit `--id-format` flag
  required on the common path). Legacy ULID path documented as recovery option.

- **Token ROI log split is sound:** Per-assistant live log paths
  (`tmp/codex-agov-token-roi.jsonl`, `tmp/claude-agov-token-roi.jsonl`) are
  gitignored by `tmp/`; committed fixture in `tests/fixtures/token_roi/` is
  documented and confidence-labelled. Fixture is used in `token_roi.rs` test.

- **`kdmr` side handoff noted:** Codex dispatched `20260623T212241Z-coder-tester-kdmr`
  to planner-architect requesting Claude sessions set `AGOV_TOKEN_ROI_LOG`.
  Acknowledged — I will set this in my sessions going forward.

- **`em7r` maturity sticky:** `20260623T214651Z-coder-tester-em7r` carries the
  maturity vocabulary forward as sticky AI/human guidance. Correct and useful.

- **UTC preference (`ghgk`):** Durable records use UTC. Correct.

## Token ROI environment — governor-reviewer sessions

Per `kdmr`, Claude sessions will use:
```bash
export AGOV_TOKEN_ROI_LOG="$PWD/tmp/claude-agov-token-roi.jsonl"
```

This session will set this on next invocation.

## Step 3 — next

MR !155 merged. Step 2 of the RC-1 sequence is complete.
Step 3 is MR 2: version provenance + command contract docs.
Separate handoff to Codex to follow.
