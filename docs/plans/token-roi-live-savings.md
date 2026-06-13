---
id: token-roi-live-savings
status: active
topic: tooling-token-roi
gate1_approved_by: "governor-reviewer direct request: live potential token savings should be logged"
gate1_approved_at: "2026-06-14T01:20:00+03:00"
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: telemetry must remain local and avoid output bodies, prompts, secrets, model responses, billing data, or remote upload
  - role: contribution-auditor
    status: consulted
    reason: savings estimates must stay explicitly approximate and evidence-based
  - role: fact-checker
    status: consulted
    reason: automatic baselines must match the documented ROI ranges
---

# Plan: token ROI live savings estimates

## Intent

Extend the token ROI POC so live tool-usage records include potential
token-savings estimates, not only measured tool-output size.

The saved-token estimate follows the documented formula:

```text
estimated_tokens_saved =
  tokens(manual_context_that_would_have_been_read)
  - tokens(tool_output_that_was_read)
  - tokens(extra_prompt_needed_to_invoke_or_explain_the_tool)
```

## Scope

- Attach documented manual-baseline ranges to known live workflows:
  - `cold-start`;
  - `session-resume:<role>`;
  - `next-handoff:<role>`.
- Store baseline range, prompt-overhead estimate, and saved-token range in the
  JSONL record for automatic tool-output telemetry.
- Preserve manual `--manual-chars` measurements and compute exact saved-token
  values for those records.
- Roll up saved-token ranges by command and workflow in
  `mix agov.token.roi report`.
- Update `docs/reference/tooling-token-roi.md`.

## Out of scope

- Exact tokenizer integration.
- Billing-grade measurements.
- Capturing command output bodies or prompts.
- Automatic baselines for `usage.report`, `coord.status`, or
  `coord.visibility`; those can follow once output formatting is centralized.

## Acceptance criteria

- A live `session.resume` record contains `approx_saved_min_tokens` and
  `approx_saved_max_tokens`.
- Reports show potential saved-token ranges by command and workflow.
- Unknown workflows still record usage without inventing savings.
- Tests cover automatic baselines, manual baselines, and report aggregation.
