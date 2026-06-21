---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260621T145253Z-planner-architect-m3px
timestamp: 2026-06-21T14:52:53Z
immutable: true
references-to:
  - 20260621T143538Z-coder-tester-drq5.md
  - 20260621T145227Z-planner-architect-k9vw.md
topic: docs-factual-claims-cleanup-slice2
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Correct stale Rust capability claims in tooling-and-tokens.md
    - Fix misleading release download wording in tooling-and-tokens.md
    - Update validator status table in contracts-and-validators.md for shipped validators
  must_read:
    - docs/plans/agf-docs-factual-claims-cleanup.md
    - agf/docs/site/docs/adopt/tooling-and-tokens.md
    - agf/docs/site/docs/advanced/contracts-and-validators.md
  do_not_touch:
    - agf/docs/site/reference/release-artifacts.md
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Handoff: Docs factual-claims cleanup (Slice 2 of 4)

Governor-approved remediation slice 2. Implement after Slice 1
(`docs/governance-safety-fix`) is merged, or in parallel if branches are
independent (they are — different files except `contracts-and-validators.md`).

## What to implement

### 1. `agf/docs/site/docs/adopt/tooling-and-tokens.md`

**"What is not in scope" section:**

Replace these two stale bullet points:

| Old | New |
|---|---|
| `Windows x64 distribution (Phase 3, not yet planned)` | `Windows x64 packaged release (preview CI artifact exists as of 2026-06-21; downloadable release not yet available)` |
| `Token ROI reporting in Rust (Phase 2B)` | Remove — shipped in `feat/rust-token-roi-report` (2026-06-21) |

**Option B — Framework-only release archive:**

Current text says: "download the optional packaged `agov` tooling archive from
the release page, or use the wrapper with Mix as the backend."

Replace with wording that reflects the actual current state: no packaged
tooling binary release is yet available for download. The `agov` wrapper with
Mix as the backend is the current path for framework-only adopters. A packaged
binary release is planned; until it is available, use the wrapper. Example
replacement:

> A release archive containing only `agf/` has schemas and templates but not
> the tooling executable. In that case, use the wrapper with Mix as the
> backend (Option A without the Rust build step). A packaged `agov` tooling
> binary is planned for a future release.

### 2. `agf/docs/site/docs/advanced/contracts-and-validators.md`

Update the "What is under contract" validator table rows for:

**Learning records:**
- Old: `Contract target; validator task not yet shipped`
- New: `agov validate learnings` (Rust); `mix agov.validate.learnings` (Elixir reference)

**Release tags:**
- Old: `Contract target; validator task not yet shipped`  
- New: `mix agov.dist.check_tag` (Elixir)

**Before editing:** grep `tooling/elixir/lib/mix/tasks/` for agent definition
and InfoSec validators to verify their current status. Update those rows too if
Mix tasks exist. Do not claim a task is shipped without verifying the file
exists.

**Note:** The `contracts-and-validators.md` CI glob fix is in Slice 1 —
do not duplicate it here.

## Validation before Gate 2

```bash
uv run --project agents-govern --group docs mkdocs build -f agf/docs/build/mkdocs.yml --strict
```

For each changed claim, record the artifact that verifies it (grep output,
file path) in the MR description.

## Branch

Use `docs/factual-claims-cleanup`.

## Do not touch

`agf/docs/site/reference/release-artifacts.md` — that page will need
updating once `feat/rust-dist-packaging` lands (Rust cross-compiled binaries
replace the Elixir OTP bundle format). That update is a separate slice.
