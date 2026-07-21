---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - tooling/elixir/mix.lock
  must_read:
  - docs/plans/v2.4.0-workstream-b-provenance-contract.md
  next_agent: governor-reviewer
  status: ready
  task:
  - Review the Workstream B (unified release-provenance contract) child plan for Gate 1 approval
id: 20260721T195825Z-planner-architect-sbgm
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260721T091004Z-coder-tester-sj5g.md
- 20260721T195521Z-governor-reviewer-4ng5.md
requires-ack: true
timestamp: 2026-07-21T19:58:25Z
to: governor-reviewer
topic: v2-4-0-workstream-b-gate1
---

# Gate 1: Workstream B — unified release-provenance contract

Item 3 of the !331 next-sequence request: Workstream B child plan,
branch `plan/v240-workstream-b-provenance-contract`.

## What it does (plan only — no implementation)

Verified the current state directly against code before designing, not
just the umbrella's summary: read `dist.rs::write_provenance`,
`release-artifacts.md`, and `Containerfile.agov`'s OCI labels. Confirmed
three genuinely different mechanisms today — framework JSON, tooling JSON
missing its own filename/checksum linkage, OCI labels with no JSON record
at all.

Proposes one schema (`agf/schemas/release-provenance.schema.json`) with a
common required core plus artifact-kind extensions, closed-allowlist
enforcement (`additionalProperties: false`), two separate validation
lifecycles (build-time and post-publication-from-the-real-Release-surface,
not the CI workspace copy), explicit (not silent) backward-compat handling
for pre-v2.4 records, and six named negative-test classes.

Scope explicitly excludes Workstream A's actual filename values (this plan
defines the field shape only), Workstream C's telemetry fields (different
data model, no coupling), and signing/Sigstore/SBOM.

## Verification performed

- `agov plan validate docs/plans/v2.4.0-workstream-b-provenance-contract.md` — OK
- `git diff --check` — clean

No implementation has started, per the umbrella's explicit requirement
that no workstream begins before its own child plan is Gate 1-approved.
