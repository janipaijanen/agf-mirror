---
kind: review
from: contribution-auditor
to: governor-reviewer
seq: 224
timestamp: 2026-06-13T21:50:00Z
immutable: true
references-to:
  - 00217-coder-tester.md
  - 00223-planner-architect.md
topic: agf-v2-layout-s1-contribution-audit
requires-ack: true
artifacts_owned: []
review:
  branch: feat/agf-v2-layout
  plan: docs/plans/agf-v2-layout-s1.md
  gate: advisory
  verdict: recommend-ready-for-gate2
  blocking_findings: 0
  warn_findings: 1
---

# Contribution-Auditor Advisory Review: AGF v2.0 Layout S1

Supersedes incorrectly-framed seq 223 (filed as Gate 2; should be CA advisory).

Branch: `feat/agf-v2-layout`
Commits: `2fa75b0 feat: establish agf v2 layout`, `e00399b coordination: request agf layout contribution audit`

## Verdict

**RECOMMEND READY FOR GATE 2** — one WARN finding; non-blocking; fast-follow eligible.

## CA checklist

### Adopter-facing `agf/` coherence

- [x] `agf/` is a single directory containing all adopter-facing content:
      ADOPTING.md, CHANGELOG.md, LICENSE, agents/, docs/, framework.md,
      project-specific-examples/, rules/, schemas/, skills/
- [x] `agf/agents/` contains 13 agent YAMLs + MANIFEST.yaml — regular files only,
      confirmed with `ls -la` (no subdirectories)
- [x] `agf/rules/`, `agf/schemas/`, `agf/skills/`, `agf/project-specific-examples/`
      all populated from retired source directories

### Hard constraint: `agents-govern/` content not shipped under `agf/`

- [x] `git diff master...feat/agf-v2-layout -- agents-govern/` returns empty —
      `agents-govern/` is untouched
- [x] No content from `agents-govern/learnings/` appears under `agf/`
- [x] Text references to "agents-govern" in `agf/docs/` are documentary (framework
      historical name in ADRs and figures), not structural violations
- [x] `agents-govern/` (v1.x) preserved at root as required by session constraint

### Retirements match seq 215 Gate 1 answers

- [x] `agents-govern-2.0/` retired immediately (Q3 answer) — absent from worktree
- [x] `spec/rules-template/` retired immediately (Q2 answer) — absent
- [x] `spec/schemas/` retired immediately — absent
- [x] `spec/` fully retired

### Tooling update (required in S1)

- [x] `AGOV_SCHEMA_DIR` default updated to `../../agf/schemas` in all three
      affected tooling files — retiring `spec/schemas/` does not break the tooling

### Tests

- [x] 125 tests pass post-retirement (confirmed in seq 216)

## Findings

### WARN — `.claude/rules/always/downstream-impact-rule.md` trigger paths stale

**Severity:** WARN (non-blocking for adopter distribution)

Foundry-internal `.claude/rules/always/downstream-impact-rule.md` still lists
`spec/rules-template/**/*.md` (line 12) and `spec/schemas/*.json` (line 15).
Both directories are retired. The `agf/rules/` copy is correctly updated.

Governor has directed a fast-follow patch to `.claude/rules/always/downstream-impact-rule.md`
immediately after merge. Patch is single-file, fast-track eligible.

## Recommendation

Branch is coherent, constraint-compliant, and test-green. Ready for governor
Gate 2 approval and merge.
