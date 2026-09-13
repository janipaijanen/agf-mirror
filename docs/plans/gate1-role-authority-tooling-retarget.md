---
id: gate1-role-authority-tooling-retarget
status: complete
topic: tooling-defaults-retarget-agents-govern-to-agf
timestamp: 2026-06-15T18:21:00Z
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: no credentials, auth, or secret handling; pure default path change
  - role: contribution-auditor
    status: not-applicable
    reason: internal tooling defaults; no adopter-facing contract change
---

**Correction (2026-09-13, qbka Slice B):** status was `gate-1-approved`, an
invalid enum spelling; corrected to `complete` — AGENTS.md's current Role
authority note confirms `agf/agents/MANIFEST.yaml` (not `agents-govern/
agents/MANIFEST.yaml`) is the live role authority today, exactly the
retarget this plan describes. See `docs/plans/20260913T171837Z-qbka-
slice-b-legacy-plan-migration.md`.

# Plan: Gate 1 — Role Authority Tooling Retarget

## Intent

Close Gate 1 (seq 324): "Active role status still bootstraps from
`agents-govern/agents/MANIFEST.yaml`. AGF-native role authority must be
defined and tooling defaults retargeted."

Change five hardcoded defaults in `tooling/elixir/` from the legacy
`agents-govern/` paths to the AGF v2 `agf/` paths:

- `@default_manifest` paths: `agents-govern/agents/MANIFEST.yaml` → `agf/agents/MANIFEST.yaml`
- `@default_base_branch`: `"master"` → `"main"`

## Affected files

| File | Line | Change |
|------|------|--------|
| `tooling/elixir/lib/mix/tasks/agov.session.resume.ex` | 18, 20, 24, 42 | Update doc, `@default_manifest`, and `base_branch` default |
| `tooling/elixir/lib/mix/tasks/agov.usage.report.ex` | 7 | Update `@default_manifest` |
| `tooling/elixir/lib/agov/plan_validator.ex` | 109 | Update `System.get_env` fallback |
| `tooling/elixir/lib/agov/session_resume.ex` | 21, 48, 253 | Update `"master"` defaults and string reference |
| `tooling/elixir/lib/mix/tasks/agov.coord.visibility.ex` | 10, 26 | Update doc and `base_branch` default |

## Test impact

None. Verified: all tests that reference `base_branch` or manifest paths
explicitly override the default via opts. No test relies on the default
value being `"master"` or `"../../agents-govern/agents/MANIFEST.yaml"`.

## Risks

Low. Defaults only — callers that pass explicit values are unaffected.
The `agf/agents/MANIFEST.yaml` file already exists and is valid (it is
the AGF v2 active-role status file for the foundry).

## Implementation order

No TDD required for default-only string changes. Direct implementation:
1. Edit all five files (line-level changes only)
2. Run `mix test` — must be fully green
3. Run `mix agov.session.resume` — must show `--base-branch main`
4. Commit on `main` (coordination files are committed on main per workflow)

## Out of scope

- Changing CLI flag names (`--base-branch`, `--manifest`) — these stay
- Updating `token_roi_test.exs` which explicitly tests `"master"` in its
  recorded args — that test records the arg as-is; no change needed
- `dist_manifest.ex` — already points to `agf/dist/release-manifest.yaml`
