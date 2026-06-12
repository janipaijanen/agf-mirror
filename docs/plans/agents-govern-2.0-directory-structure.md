---
id: agents-govern-2.0-directory-structure
status: pending-gate1
gate1_approved_by: ""
gate1_approved_at: ""
topic: v2.0-directory-structure
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: no security boundaries, auth, or PII changes; structural reorganization only
  - role: contribution-auditor
    status: not-applicable
    reason: governor-reviewer Gate 1 covers structural scope; no ownership ambiguity
---

# Plan: agents-govern-2.0 — directory structure

**Date:** 2026-06-12T21:35:00Z
**Status:** PENDING GATE 1 APPROVAL
**Authorized by:** ADR `docs/adr/20260506-agents-govern-2-scope.md`
**Triggered by:** Adopter feedback (seq 86/87) — root noise, incoherent docs,
positioning unclear, no decommission guide. Constitutes the N≥1 additional
layout-blockage record that supports the v2.0 path (ADR threshold was N≥3
for *in-place* v1.x restructure; v2.0 side-by-side was already authorized).

---

## Intent

Build `agents-govern-2.0/` at repo root, side-by-side with `agents-govern/`
(v1.x canonical, untouched). v2.0 is Provisional per the ADR.

Goals (ADR-scoped):
1. **First-arrival clarity** — reduce root noise from 18 to ~9 files;
   clear reading-order gradient; "is this for you?" positioning.
2. **Better IA** — move governance-meta docs out of root into `docs/governance/`;
   rename confusable files; add missing artifacts (decommission guide).
3. **Broken-link repair** — fix the 8 broken README links identified in the ADR.
4. **Archive/source parity** — v2.0 dist includes all shipped artifacts.

Out of scope (per ADR):
- Persona-targeted content (PM, RDI, SVP audiences)
- New agents
- YAML-LD / linked-data work
- Marketing rewrite
- Modifying any v1.x file

---

## File mapping

### Root — keep (~9 files, platform-convention justified)

| v1.x file | v2.0 location | Change |
|---|---|---|
| README.md | README.md | Rewritten: 3-para pitch + "is this for you?" + links |
| ADOPTING.md | ADOPTING.md | Carried over; broken links repaired |
| framework.md | framework.md | Carried over unchanged |
| framework.yaml | framework.yaml | Version updated to 2.0.0 |
| CHANGELOG.md | CHANGELOG.md | Fresh; v2.0 entries only |
| LICENSE | LICENSE | Carried over unchanged |
| CODE_OF_CONDUCT.md | CODE_OF_CONDUCT.md | Carried over unchanged |
| CONTRIBUTING.md | CONTRIBUTING.md | Carried over; paths updated |
| SECURITY.md | SECURITY.md | Carried over unchanged |
| VERSION | VERSION | 2.0.0 |

### Move to docs/governance/ (currently at root or docs/ root)

| v1.x file | v2.0 location | Rename? |
|---|---|---|
| GOVERNANCE.md | docs/governance/governance.md | lowercase only |
| LEARNINGS.md | docs/governance/learning-system.md | Yes — describes the system |
| LESSONS-LEARNED.md | docs/governance/retrospective.md | Yes — it's a retrospective |
| MANIFEST.md | docs/governance/manifest.md | lowercase only |
| DISCLAIMER.md | docs/governance/disclaimer.md | lowercase only |
| docs/ANONYMIZATION-NOTICE.md | docs/governance/anonymization-notice.md | move + lowercase |

### docs/adopt/ — existing + new

| File | Status | Notes |
|---|---|---|
| docs/adopt/compliance-profile.md | carry over | unchanged |
| docs/adopt/decommission.md | **NEW** | How to remove adoption cleanly |

### docs/reference/ — new subdirectory

| File | Status | Notes |
|---|---|---|
| docs/reference/quick-reference.md | **NEW** | One-page role/pipeline/checklist summary |

### docs/contracts/ — reorganize from docs/ root

| v1.x file | v2.0 location |
|---|---|
| docs/binding-contract.md | docs/contracts/binding-contract.md |
| docs/contract-stability.md | docs/contracts/contract-stability.md |
| docs/integration-contract.md | docs/contracts/integration-contract.md |

### Carried over unchanged (same relative paths)

- `agents/` — all 12 YAMLs + MANIFEST.yaml
- `rules-template/` — all rules
- `schemas/` — JSON schemas + invariants
- `scripts/` — generate-schemas.py
- `skills/` — all 7 skills
- `project-specific-examples/` — all stacks
- `learnings/` — foundry.yaml + _template.yaml + .adopter-mapping.example
- `docs/adr/` — all ADRs
- `docs/agent-communication-*.md` — generated graphs
- `pyproject.toml` + `src/agents_govern/` — tooling
- `.pre-commit-config.yaml`
- `.gitlab/`

---

## New files (content to write)

### docs/adopt/decommission.md

Covers:
1. Files to delete (agent YAMLs, rules-template copy, skills copy, learnings codename file)
2. Hooks to remove (.pre-commit-config.yaml, .claude/settings.json changes)
3. CI changes to revert (validator jobs, path-scoped triggers)
4. Git history note (commit the removal as a single atomic commit with a clear message)
5. Verification: run `agov-validate-agents` before removal to confirm current state;
   confirm no remaining `agov-*` references in CI config after removal

### docs/reference/quick-reference.md

One-page summary covering:
- Agent roles (Phase 1 names, responsibilities, 1 sentence each)
- Pipeline steps (Gate 1 / Gate 2 / fast-track)
- Coordination channel kinds (9 kinds, 1 line each)
- Key rules checklist (5 most-referenced rules)

---

## Affected files summary

| Location | Action | Count |
|---|---|---|
| `agents-govern-2.0/` root | New directory | — |
| Root files carried over | Copy + minor edits | 9 |
| Root files moved to docs/governance/ | Copy + rename | 6 |
| docs/ root files moved to docs/contracts/ | Copy | 3 |
| docs/adopt/ | Carry over + 1 new | 2 |
| docs/reference/ | 1 new | 1 |
| All other dirs | Copy unchanged | ~8 dirs |

**v1.x is untouched throughout.**

---

## Implementation order

1. Create `agents-govern-2.0/` skeleton (dirs only)
2. Copy unchanged directories
3. Copy + lightly edit root files (broken link repair in README, ADOPTING)
4. Copy + rename governance-meta files to `docs/governance/`
5. Write `docs/adopt/decommission.md`
6. Write `docs/reference/quick-reference.md`
7. Write new README.md (3-para pitch + positioning)
8. Update internal cross-links in all moved/renamed files
9. Update `dist.py` to build from `agents-govern-2.0/`
10. Validate: `agov-validate-agents` passes from v2.0 root

---

## Downstream impact

**Classification:** ADDITIVE (Provisional)

- **Foundry:** New directory alongside v1.x. v1.x untouched. No existing
  pipeline or tooling broken.
- **Direct adopters:** New Provisional option. v1.x remains canonical.
  No migration required. No existing contract removed.
- **Downstream:** None until v2.0 promotes to canonical.
- **Migration:** Not required. v2.0 is opt-in.

---

## Risks

- Cross-link rot: files moved/renamed need all internal links updated.
  Mitigation: step 8 (link audit) is explicit in the implementation order.
- `dist.py` must be pointed at v2.0 root for the v2.0 archive build;
  v1.x dist target must be preserved. Mitigation: separate dist invocation
  per version, not a shared build.
- v2.0 Provisional banner must be visible at README entry point so adopters
  mid-evaluation know v1.x is still canonical. Mitigation: first line of
  v2.0 README.

---

## Rollback

`agents-govern-2.0/` is a new directory. Rollback = `rm -rf agents-govern-2.0/`.
No v1.x file is modified, so rollback has zero blast radius on existing adopters.
