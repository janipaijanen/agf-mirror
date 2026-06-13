---
kind: review
from: planner-architect
to: governor-reviewer
seq: 223
timestamp: 2026-06-13T21:45:00Z
immutable: true
references-to:
  - 00217-coder-tester.md
  - 00215-planner-architect.md
topic: agf-v2-layout-s1-gate2
requires-ack: true
artifacts_owned: []
review:
  branch: feat/agf-v2-layout
  plan: docs/plans/agf-v2-layout-s1.md
  gate: "2"
  verdict: recommend-merge
  blocking_findings: 0
  warn_findings: 1
---

# Gate 2 Review: AGF v2.0 Layout S1

Branch: `feat/agf-v2-layout`
Plan: `docs/plans/agf-v2-layout-s1.md` (Gate 1 approved seq 215)
Governor approved CA handoff (seq 217) directly.

## Verdict

**RECOMMEND MERGE** — one WARN finding noted below; non-blocking.

## Verification checklist

### Hard constraint (governor, seq 215)

- [x] `agf/agents/` contains only regular YAML files (13 agents + MANIFEST.yaml)
      — no `agents-govern/` subdirectory present. `ls -la` confirms `drwxrwxr-x 2`
      with no subdirectories.
- [x] `agents-govern/` (v1.x) untouched at worktree root — `git diff master...feat/agf-v2-layout -- agents-govern/` returns empty.

### Retirements

- [x] `agents-govern-2.0/` retired — directory absent from worktree root.
- [x] `spec/rules-template/` retired — absent.
- [x] `spec/schemas/` retired — absent.
- [x] `spec/` directory fully retired.

### `agf/` content boundary

- [x] `agf/` top-level: ADOPTING.md, CHANGELOG.md, LICENSE, agents/, docs/,
      framework.md, project-specific-examples/, rules/, schemas/, skills/ — all
      expected content present.
- [x] `agf/agents/` has all 13 agent YAMLs + MANIFEST.yaml from `agents-govern-2.0/agents/`.
- [x] `agf/rules/`, `agf/schemas/`, `agf/skills/`, `agf/project-specific-examples/`
      populated from source directories.
- [x] `agents-govern/learnings/` not included in `agf/` (foundry-specific, confirmed absent).

### Tooling

- [x] `AGOV_SCHEMA_DIR` default updated to `../../agf/schemas` in:
  - `tooling/elixir/lib/agov/coord_validator.ex` line 44
  - `tooling/elixir/lib/agov/plan_validator.ex` line 104
  - `tooling/elixir/lib/mix/tasks/agov.coord.validate.ex` doc string
  - `tooling/elixir/lib/mix/tasks/agov.plan.validate.ex` doc string
- [x] No remaining `spec/schemas` references in `tooling/elixir/lib/`.

### AGENTS.md

- [x] `agf/` referenced as canonical v2.0+ adopter-facing landing directory (line 15).
- [x] `agents-govern/` described as bootstrap reference, not the product.

### Tests

- [x] 125 tests pass on `feat/agf-v2-layout` (post-retirement run confirmed in seq 216).

### `agf/rules/` downstream-impact trigger paths

- [x] `agf/rules/always/downstream-impact-rule.md` correctly lists `agf/rules/**/*.md`,
      `agf/agents/*.yaml`, `agf/schemas/*.json` as trigger paths — updated for v2.0.

## Findings

### WARN — `.claude/rules/always/downstream-impact-rule.md` trigger paths stale

**Severity:** WARN (non-blocking)
**Location:** `.claude/rules/always/downstream-impact-rule.md` lines 12 and 15

The foundry's own Claude config still lists:
```
- `spec/rules-template/**/*.md`
- `spec/schemas/*.json`
```
Both directories are retired by this branch. The `agf/rules/` copy of the same
rule was correctly updated with `agf/` trigger paths. The foundry's `.claude/rules/`
copy was not included in the S1 plan's affected-files list and was not updated.

**Impact:** After merge, a Claude session in the foundry editing `agf/rules/` or
`agf/schemas/` will not receive the downstream-impact warning that fires on
framework artifact changes. The adopter distribution is unaffected.

**Recommendation:** Fix in a fast-follow commit on master (single-file edit,
fast-track eligible). Replace `spec/rules-template/**/*.md` → `agf/rules/**/*.md`
and `spec/schemas/*.json` → `agf/schemas/*.json`. Also add `agf/agents/*.yaml`
to match the `agf/rules/` version.

## Merge options

**Option A — Direct merge (recommended)**
```bash
git checkout master
git merge --no-ff feat/agf-v2-layout
```

**Option B — Squash merge**
```bash
git checkout master
git merge --squash feat/agf-v2-layout
git commit -m "feat: establish agf v2.0 layout — create agf/, retire agents-govern-2.0/ and spec/"
```

The branch has two clean commits (implementation + coordination). Option A
preserves the audit trail; Option B produces a single commit. Recommendation:
Option A.
