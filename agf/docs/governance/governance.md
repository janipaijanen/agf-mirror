<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Governance — AGF-NG (foundry)

Framework version adopted: **2.13.0** | Layout: single `agf/` directory
(AGF v2.0+; see `agf/rules/always/governance-file-layout-rule.md`) |
Codename: **foundry**

This file governs the AGF-NG project — the next version of agents-govern.
It is a project-specific governance charter alongside the upstream
`agents-govern/GOVERNANCE.md` bootstrap reference (legacy, not this
project's own governed artifact).

**This page summarizes; it does not duplicate.** Role rosters, gate
mechanics, and coordination-channel rules change more often than this
charter should be rewritten. Where a live source exists, this page
points at it instead of re-encoding a copy that can drift out of sync —
the failure mode that made the previous version of this file stale
across framework version, role roster, model name, and coordination
schema simultaneously (see estate-sweep finding **DES-6**,
`docs/reviews/documentation-estate/20260910-first-estate-sweep.md`).

---

## Role Assignments

Authoritative source: **`AGENTS.md`** ("Active agents" section) and
**`agf/agents/MANIFEST.yaml`** (per-role status, player, and
corrective-work trail). This charter does not re-list the roster; read
those two files for who plays what and whether a role is active,
skipped, or suspended.

Session-level or temporary role assignments (e.g., a specific model
playing `coder-tester` for a given work stream) are recorded as
`decision-record` entries in `docs/coordination/`, not written into
this charter — a role assignment that changes between sessions would
make this file stale again the moment it changed.

---

## Pipeline Mapping

| Pipeline Step | Maps To |
|---------------|---------|
| Requirement | Proposal in `docs/proposals/` or an issue |
| Plan | Gate 1 plan file in `docs/plans/` (schema: `agf/schemas/gate1-plan.schema.json`) |
| Gate 1 | governor-reviewer approval, recorded in `docs/coordination/` |
| Implement | coder-tester edits framework artifacts on a feature branch |
| Verify | `agov validate` / `agov plan validate` / `agov coord validate` pass; InfoSec clean |
| Gate 2 | governor-reviewer reviews the exact-head diff, recorded in `docs/coordination/` |
| Commit | Merge to `main` via merge request |

---

## Gate Rules

Full, current gate mechanics live in `AGENTS.md` ("Gate discipline") and
`agf/rules/workflow/*.md` (`development-workflow-rule.md`,
`systematic-code-review-rule.md`, `simplicity-rule.md`,
`gate-self-test-rule.md`). This charter states the invariant those
rules all implement, not their current procedural detail:

- **Gate 1** requires a plan file conforming to the current Gate 1
  schema, with both `infosec-sentinel` and `contribution-auditor`
  advisory consultation declared, before implementation starts.
- **Gate 2** requires implementation on a feature branch, passing
  validation, and a governor-reviewer review of the exact head before
  merge.
- A narrow, rule-defined fast-track exists for specific low-risk change
  classes (see `agf/rules/workflow/simplicity-rule.md`) — it is never a
  blanket exemption and never covers gate mechanics themselves
  (`agf/rules/always/gate-self-test-rule.md`).

---

## Artifact Ownership

Canonical layout and ownership table:
`agf/rules/always/governance-file-layout-rule.md`. That table is the one
authority for artifact ownership (including its `agf/agents/*.yaml` "pending
#79" qualifier and footnote); this charter does not restate it as a second
copy, since a restated copy is exactly the kind of independently-drifting
duplicate DES-6 itself was filed against.

This project has not created a project-root `GOVERNANCE.md`; this file
(`agf/docs/governance/governance.md`) serves that role for foundry. A
prior version of this file cited a nonexistent `agf/GOVERNANCE.md` path
— corrected here; do not recreate that cross-reference.

---

## Coordination Channel

Append-only channel at `docs/coordination/`. Current ID format, message
kinds, and validation are defined in `AGENTS.md` ("Coordination
channel") and `agf/schemas/coordination-message.schema.json` — read
those, not this section, for the live contract. Invariants that don't
change with schema revisions:

- Messages are immutable. Corrections are new messages that reference
  and supersede the one they correct — never edits to the original.
- Every message has YAML front matter conforming to the current schema.
- A coordination message addressed to another agent or human isn't
  complete until it's published somewhere the recipient can actually
  read it (committed and pushed, or bridged with a visible pointer) —
  see `agf/rules/workflow/coordination-publication-rule.md`.

---

## Self-Governing Principle

This project uses the framework it is building. Every friction point is
recorded in `agf/learnings/foundry.yaml` and becomes evidence for future
proposals and rule changes.

Additional governor-reviewer responsibility: **evidence arbiter** —
reject plans citing no corroborating evidence. This enforces the
framework's own evidence discipline while building the next version.

---

## Role-Split History

This charter originally described a single-Claude arrangement (one
session playing both `planner-architect` and `coder-tester`) with
"Split Signals" naming the conditions under which those would divide
into separate players. That split has since happened in practice —
`coder-tester` and `planner-architect` are played by separate sessions
per the current role assignment recorded in `docs/coordination/` and
`agf/agents/MANIFEST.yaml`, not by a single combined session. The
original rationale is preserved below as the historical reasoning for
*why* a split was anticipated; it is not a live decision point anymore.

| Original signal | Historical rationale |
|--------|--------|
| Plans conflict with the implementer's own designs | Motivates separating planning from implementation |
| The implementer misses bugs in its own changes | Motivates an implementation/test separation |
| Review burden slows governance | Motivates separating governor from reviewer duties |

---

*Last updated: 2026-09-12 (DES-6 rewrite, governor decision
`20260912T094217Z-governor-reviewer-3rha`) | Framework 2.10.1 | Layout:
single `agf/` directory.*
