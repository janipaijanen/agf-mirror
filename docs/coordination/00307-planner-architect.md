---
kind: handoff
from: planner-architect
to: governor-reviewer
seq: 307
timestamp: 2026-06-14T11:18:00Z
immutable: true
references-to:
  - 00302-governor-reviewer.md
  - 00305-planner-architect.md
topic: tool-authority-boundary-rule-gate2
requires-ack: true
artifacts_owned: []
model: claude-sonnet-4-6
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 2 review and merge of feat/tool-authority-boundary-rule"
  must_read:
    - docs/plans/tool-authority-boundary-rule.md
    - agf/rules/always/tool-authority-boundary-rule.md
    - docs/adr/20260614-dogfood-authority-boundary.md
  do_not_touch:
    - docs/coordination/00307-planner-architect.md
---

Gate 2 handoff for `feat/tool-authority-boundary-rule` (Gate 1 approved seq
302, governor-reviewer).

## What was implemented

Branch: `feat/tool-authority-boundary-rule`
Commit: `fa167fa`

### Artifacts

- `agf/rules/always/tool-authority-boundary-rule.md` — adopter-facing rule.
  States: tools may block work only when a named rule grants blocking authority;
  all other output is advisory evidence. Includes dogfooding model statement
  and exemption clause for delegated sessions.

- `docs/adr/20260614-dogfood-authority-boundary.md` — AGF-NG decision record.
  Records the three-role dogfooding model (agf/ as product, tooling/ as
  implementation, repository as first adopter). Cites seq 287 and seq 289.
  Explains why the rule belongs in agf/rules/always/ rather than only AGENTS.md.

- `tooling/elixir/lib/agov/coord_validator.ex` — `advisory_findings/1` added.
  Returns advisory warnings for soft checks (currently: missing `model:` field).
  Backing rule for blocking authority documented in module comment.

- `tooling/elixir/lib/mix/tasks/agov.coord.validate.ex` — updated output format:
  - `WARN severity: advisory <path>: <msg>` for advisory findings (non-blocking)
  - `OK  severity: blocking <path>` for passing files (was `OK  <path>`)
  - `ERR severity: blocking <path>: ...` for blocking failures (was `ERR <path>:`)

- `tooling/agov` — same advisory_coord/severity treatment in the Python fallback.

- `tooling/elixir/test/fixtures/valid_note_no_model.md` — triggers advisory.
- `tooling/elixir/test/fixtures/valid_note_with_model.md` — no advisory.
- `tooling/elixir/test/agov/coord_validator_test.exs` — 4 new advisory tests.

- `AGENTS.md` — one-line automation rule reminder pointing to the new rule.

## Validations

- `git diff --check`: pass
- `mix test`: 146 passed (4 new advisory tests)
- Smoke-test of `WARN severity: advisory` / `OK severity: blocking` output: pass
- `valid_note_no_model.md` triggers advisory; `valid_note_with_model.md` does not

## Acceptance criteria check

- [x] Rule exists at `agf/rules/always/tool-authority-boundary-rule.md`
- [x] Rule includes canonical blocking-authority statement from seq 289
- [x] ADR records dogfooding authority model and cites seq 287 and seq 289
- [x] One existing validator (coord) reports `severity: advisory | blocking`
- [x] Positive and negative tests cover severity output (146 tests pass)
- [x] Docs do not imply all validators have severity metadata yet
- [x] `AGENTS.md` carries short reminder, not normative rule body

## Merge recommendation

Recommend approve. The slice is focused, all acceptance criteria met, no
out-of-scope changes.

Note: `tooling/elixir/agov` (escript binary) is gitignored and not in this
commit. The Codex branches `docs/agov-packaging-release-plan` and
`docs/agov-installation-deployment` carry that work separately.
