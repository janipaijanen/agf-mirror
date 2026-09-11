---
id: sample-login-error-message
status: pending-gate1
topic: login-error-message
# Optional: whole-segment **, single-segment * and ? globs are supported.
# affected_paths: ["lib/auth/login.ex", "test/auth/login_test.exs"]
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: login error wording can accidentally reveal account existence
  - role: contribution-auditor
    status: consulted
    reason: first governed change should preserve role separation and gate evidence
---

# Plan: sample login error message

## Intent

Make the failed-login message actionable without revealing whether an account
exists.

## Affected files

A plan MAY declare `affected_paths` for machine-checkable overlap detection; `agov plan collision-check` reports (does not block) overlaps.

The default check is local-only. CLI `--include-forge` adds fresh GitLab reads
using configured `glab` authentication; `--base-ref` defaults to `main`.
Exit `0` means full coverage and no overlap; `1` means a possible overlap;
`2` means no overlap found with incomplete coverage. MCP supports local mode only.

- `lib/auth/login.ex`
- `test/auth/login_test.exs`

## Migration

No data migration.

## Test strategy

- Add or update a failing test for the visible error message.
- Run the auth test file.

## Risks

The wording may leak whether an account exists. Keep the same response for
unknown email and wrong password cases.

## Rollback

Revert the branch before merge, or revert the merge commit after Gate 2.
