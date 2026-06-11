---
kind: note
from: planner-architect
to: governor-reviewer
seq: 39
timestamp: 2026-06-11T12:58:00Z
immutable: true
references-to: []
topic: agent-yaml-spec-alignment
requires-ack: false
artifacts_owned: []
---

# Proposal: targeted revision of agent YAML communication specs

## Context

Agent YAML definitions (`planner-architect.yaml`, `coder-tester.yaml`,
`governor-reviewer.yaml`) have a `communication:` section that lists
`receives_from` and `sends_to` entries with typed `message_types`. Those
types use a logical vocabulary: `plan_submission`, `plan_approval`,
`revision_request`, `task_assignment`, and so on.

The coordination channel uses a structural vocabulary: `kind: handoff`,
`kind: decision-record`, `kind: completion`, `kind: review`, `kind: claim`,
`kind: ack`.

These two vocabularies describe the same communication at different levels
of abstraction, and the mapping between them is nowhere written down. A
cold-start agent or a future cross-referencing tool (T3-11) cannot validate
observed channel traffic against declared specs without resolving that gap.

The same `communication:` sections also reference agents that do not exist
in this project: `deployment-engineer`, `documentation-writer` (in detail),
and others listed as `status: suspended` or `status: skipped`. These add
reading overhead and quietly signal a framework template origin rather than
field-derived content.

## The question

Two options were identified in discussion:

**Option A — full fresh start:** Rewrite the agent YAMLs from scratch based
on observed coordination patterns. Evidence-grounded, breaks dependency on
the framework template shape. Higher effort; risks losing intent captured in
existing structural sections; loses template reference value for downstream
adopters.

**Option B — targeted revision (proposed):** Keep the structural sections
intact (`identity`, `jurisdiction`, `boundaries`, `capabilities`,
`accountability`, `review_checklists`). Revise only the `communication:`
section:

1. Replace `message_types` entries with channel-aligned `kind:` values.
   Example: `plan_submission` → `kind: handoff`, `plan_approval` →
   `kind: decision-record`, `code_reviewed` → `kind: review`.
2. Drop `sends_to` and `receives_from` entries for agents with
   `status: suspended` or `status: skipped` in MANIFEST. These routes
   do not exist in the current project and add noise.
3. Optionally add a short mapping note (one table) explaining the
   logical-to-structural vocabulary correspondence, so the two levels
   remain legible without being conflated.

**Why Option B is lower cost:**
- No structural redesign — only the `communication:` subsection changes.
- The rest of the YAML (identity, jurisdiction, boundaries, accountability)
  stays intact; it is working and adopted.
- The change is mechanical and verifiable: after revision, `kind:` values
  in `communication:` must be a subset of the channel schema's valid kinds.
- T3-11 (usage report) would immediately benefit: after this revision,
  the tool could cross-reference actual `from:/to:/kind:` traffic against
  declared `sends_to`/`receives_from` specs.

**Why this is Tier 3, not higher:**
No degraded functionality today. The governor-reviewer (human) routes
correctly by judgment; the AI agents follow prompts. The gap is a
machine-readability and adoption-friction problem, not an operational one.
Confirms Tier 3 classification from synthesis.md.

## Request

Per synthesis.md governance rule, Tier 3 items require Governor-Reviewer
confirmation before planning begins. Requesting confirmation to proceed
with a Gate 1 plan for this targeted revision.

Estimated scope: three agent YAML files modified (`communication:` section
only); one table in coordination README or inline in YAML mapping logical
types to structural kinds. No schema changes; no new modules; no tests
required (YAML is not validated at this level of detail by existing tooling).
