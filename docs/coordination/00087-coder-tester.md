---
kind: note
from: coder-tester
to: governor-reviewer
seq: 87
timestamp: 2026-06-12T21:05:00Z
immutable: true
references-to:
  - 00086-coder-tester.md
topic: docs-restructure-ca-inventory
requires-ack: false
artifacts_owned: []
model: claude-sonnet-4-6
---

# CA Inventory: agents-govern docs classification

Inventory pass complete. Input for Planner-Architect IA design.

## Classification results

| File | Class | Notes |
|---|---|---|
| README.md | entry-point | Three reader-intent paths; links to framework.md, ADOPTING |
| ADOPTING.md | entry-point | 7-step adoption guide; most concrete implementation reference |
| framework.md | reference | Core spec (~1800 lines); theory, pipeline, patterns, evidence |
| framework.yaml | reference | Machine-readable contract declaration; pairs with framework.md |
| docs/binding-contract.md | reference | UUID/correlation-ID trace binding spec |
| docs/contract-stability.md | reference | Semver/deprecation stability promise |
| docs/integration-contract.md | reference | Machine-checkable surface spec; CI integration target |
| docs/agent-communication-0.26.0.md | reference | Generated Mermaid comm-graph at v0.26.0 |
| docs/adopt/compliance-profile.md | reference | Three-tier compliance model (Artefact/Process/Full) |
| docs/adr/README.md | reference | ADR format and lifecycle spec |
| GOVERNANCE.md | governance-meta | Role assignments, pipeline mapping, gate rules template |
| LEARNINGS.md | governance-meta | Learning system spec: categories, severity, Sentinel role |
| LESSONS-LEARNED.md | governance-meta | Retrospective narrative: what worked, gaps, surprises |
| MANIFEST.md | governance-meta | Distribution provenance, file purposes, upgrade instructions |
| CHANGELOG.md | governance-meta | Version history with evidence citations |
| CODE_OF_CONDUCT.md | governance-meta | Community standards |
| CONTRIBUTING.md | governance-meta | Contribution guidelines, evidence requirements, anonymization |
| SECURITY.md | governance-meta | Security scope, reporting process, incident log |
| DISCLAIMER.md | governance-meta | Advisory-only disclaimer, liability, anonymization warning |
| docs/ANONYMIZATION-NOTICE.md | governance-meta | Codename requirement effective 2026-04-06 |
| docs/adr/20260405-iron-gate-adopter-feedback.md | governance-meta | First adopter feedback ADR (Proposed) |
| docs/adr/20260405-license-cc-by-sa-4.md | governance-meta | License change rationale ADR (Proposed) |
| docs/adr/20260505-lean-agile-test-fly.md | governance-meta | Lean-Agile mapping profile ADR (Provisional) |
| docs/adr/20260505-yaml-ld-provisional-artifact-shape.md | governance-meta | YAML-LD artifact shape ADR (Provisional) |
| docs/adr/20260506-agents-govern-2-scope.md | governance-meta | agents-govern 2.0 scope ADR (Provisional) |
| learnings/foundry.yaml | governance-meta | Phase 1 foundry learning records |
| learnings/_template.yaml | reference | Learning record template |
| learnings/.adopter-mapping.example | reference | Codename mapping example |

## No content duplicates

Each file has a distinct purpose. However, LEARNINGS.md and LESSONS-LEARNED.md
are **confusable from the outside** — similar names, both about learnings. A new
reader cannot distinguish them without opening both. This is a naming/location
problem, not a content problem.

## Root-level noise: quantified

**18 files at repo root** (14 markdown + 4 config: pyproject.toml,
.pre-commit-config.yaml, .infosec-ack.yaml, VERSION).

Of the 14 markdown files, platform conventions justify keeping at root:
CODE_OF_CONDUCT.md, CONTRIBUTING.md, SECURITY.md, LICENSE, CHANGELOG.md (5).

Candidates to move to docs/: GOVERNANCE.md, LEARNINGS.md, LESSONS-LEARNED.md,
MANIFEST.md, DISCLAIMER.md. framework.md and framework.yaml stay at root
(core spec, README links to them directly).

## Gaps confirmed

| Gap | Priority |
|---|---|
| Decommission/purge guide — how to remove adoption cleanly | High |
| Quick reference card — one-page role/pipeline/checklist summary | Medium |
| Troubleshooting guide — "when X happens, do Y" for common friction | Medium |
| Version migration guide — jumping multiple major versions | Medium |
| Glossary — central definition of framework concepts | Low (framework.md serves this but buried) |

## Proposed IA direction (for Planner-Architect plan)

**Keep at root (~9 files):** README, ADOPTING, framework.md, framework.yaml,
CHANGELOG, LICENSE, CODE_OF_CONDUCT, CONTRIBUTING, SECURITY

**Move to docs/governance/ (~5 files):** GOVERNANCE, LEARNINGS, LESSONS-LEARNED,
MANIFEST, DISCLAIMER

**Move to docs/governance/ (~1 file):** ANONYMIZATION-NOTICE (already in docs/
but not in a subdirectory)

**docs/adr/ stays as-is**

**New files needed:** docs/adopt/decommission.md (high priority);
docs/reference/quick-reference.md (medium)

This reduces visible root noise from 14 markdown files to 9 without removing
any content. Reading-order gradient becomes clear: README → ADOPTING →
framework.md for concepts, docs/governance/ for meta.
