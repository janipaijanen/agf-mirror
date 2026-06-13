---
id: token-roi-telemetry-poc
status: active
topic: tooling-token-roi
gate1_approved_by: "governor-reviewer direct request: plan and implement token ROI telemetry POC"
gate1_approved_at: "2026-06-13T21:20:00Z"
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: telemetry must stay local, avoid prompt/model/secret capture, and write only ignored repository-local records by default
  - role: contribution-auditor
    status: consulted
    reason: ROI reporting must not overclaim exact savings or weaken governance evidence standards
  - role: fact-checker
    status: consulted
    reason: documentation must distinguish approximate character-based estimates from exact tokenizer or billing telemetry
---

# Plan: token ROI telemetry POC

**Date:** 2026-06-13T21:20:00Z
**Status:** ACTIVE — direct governor request to plan and implement
**Priority:** Medium (measurement for AGF v2 tooling objective)

## Intent

Start collecting local data that can be reported when asked whether AGF's
deterministic tooling is reducing token consumption. The POC should measure
deterministic tool output sizes over time and allow manual baseline estimates
to be recorded, without collecting prompts, model responses, secrets, billing
data, or external telemetry.

## Scope

1. Add a small Elixir module that:
   - estimates tokens as `ceil(chars / 4)`;
   - appends JSONL records to a local ignored log;
   - loads and summarizes those records.
2. Add `mix agov.token.roi` with:
   - `report` to summarize accumulated local telemetry;
   - `record` to explicitly add measurements from commands not yet wired for
     automatic collection.
3. Automatically record successful output sizes for low-risk deterministic
   commands that already return one formatted string:
   - `mix agov.session.resume`;
   - `mix agov.handoff.next`.
4. Document the POC in `docs/reference/tooling-token-roi.md` and link it from
   `AGENTS.md`.

## Out of scope

- Exact model tokenizer integration.
- Billing-grade token accounting.
- Prompt, response, model, API, or user-content capture.
- Automatic wrapping of every Mix task.
- CI upload or remote telemetry.

## Safety and privacy

Default log path is `../../tmp/agov-token-roi.jsonl`, which is covered by the
existing root `tmp/` ignore rule. Operators may override with
`AGOV_TOKEN_ROI_LOG`. Automatic writes can be disabled with
`AGOV_TOKEN_ROI=0`.

Records contain command name, arguments, optional workflow label, character
count, approximate token count, and timestamp. They intentionally do not store
command output bodies.

## Acceptance criteria

- `mix agov.token.roi report` works with an absent log and with recorded data.
- `mix agov.token.roi record --command <name> --chars <n>` appends a record.
- Successful `session.resume` and `handoff.next` invocations append local
  output-size records by default.
- `AGOV_TOKEN_ROI=0` prevents automatic writes.
- Documentation states that estimates are approximate and local.

## Validation

- `git diff --check`
- `mix test test/agov/token_roi_test.exs`
- `mix compile --warnings-as-errors`
- `mix test`
- Manual smoke:
  - `mix agov.token.roi report --log <tmp-log>`
  - `mix agov.token.roi record --log <tmp-log> --command manual --chars 16`
