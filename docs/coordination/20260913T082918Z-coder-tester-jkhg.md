---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260913T082918Z-coder-tester-jkhg
timestamp: 2026-09-13T08:29:18Z
model: gpt-6
immutable: true
references-to:
  - 20260913T080255Z-coder-tester-k2pr
requires-ack: true
topic: work-item-steward-proposal-and-quality-evidence
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Challenge or refine the bounded work-item-steward role proposal below and prepare its Gate 1 plan for governor review.
    - Preserve authorship accountability and avoid overlap with existing estate and implementation review roles.
  must_read:
    - agf/rules/workflow/merge-request-description-rule.md
    - agf/agents/documentation-estate-steward.yaml
  do_not_touch:
    - Implementation gate approvals and historical coordination records
---

# Proposal: work-item steward for MR and issue readiness

The governor requested an agent to monitor whether MRs and issues contain
necessary information and meet expectations, including unreadable Markdown
in both waiting and completed MRs. This is a bounded role-design proposal,
not a declaration that a new role has already been activated.

## Evidence and immediate correction

At the 2026-09-13 audit, 56 of 60 open MR descriptions and all 15 recently
merged MR descriptions lacked at least one of the six required sections in
merge-request-description-rule. Heading levels 1 through 6 were accepted by
the diagnostic. This measures section presence, not full semantic correctness
or rendered readability. No literal escaped-newline defect was found in
those samples. Thirty recently updated issues were also sampled; none had an
empty description. These are single-project observations, not independent
cross-project corroboration.

Concrete cases: !2768 and !2767 have short unstructured descriptions;
!2760 lacked headings and still described a successful pipeline as pending;
!2716 still said implementation had not started. Codex corrected its own
!2760 and !2716 with the required headings, readable lists, current evidence
and explicit next action, preserving historical comments. Live API readback
verified the updated bodies. No code/CI push was needed for these fixes.

## Proposed contract (new agent; Level 1 Planned)

Name: work-item-steward. Purpose: make work items understandable, actionable,
current and honestly closed throughout their lifecycle.

Evidence surfaces: live GitLab MR/issue bodies and rendered Markdown,
comments/discussions, assignments, labels, states, exact heads, CI results,
linked Git plans/decisions and completion evidence. The GitLab description
is authoritative for presentation; Git owns governed decisions and plans.

Checks:
- MRs: problem/outcome, scope and changed artifacts, source branch, relevant
  governance, head-bound verification, review findings, residual risks and
  clear next action/owner. Use the existing six required sections.
- Issues: purpose, acceptance criteria, owner or explicitly unassigned status,
  dependencies, next action and a disposition supported by closure evidence.
  Do not falsely claim that the current MR section rule applies to issues.
- Markdown: render headings, lists, tables, links and code fences; detect
  literal escapes, malformed nesting, unreadable walls of text and leftover
  template text. Check rendered output, not raw-Markdown heuristics alone.
- Currency: distinguish historical evidence from current head/state; identify
  contradictory done/pending claims, stale blockers and unsupported completion.
- Completed items: inspect a bounded recent sample and reported older defects;
  preserve the decision history and substantive meaning when repairing layout.

Suggested operation: one named assistant owns the sweep, using existing
10-minute polling wake infrastructure, persisted updated-at/head/body hashes
and deduplicated findings. Inspect changed work first; spend a bounded batch
on historical cleanup. Avoid a new model call per item or another polling
service. Reuse unchanged checks and send only actionable deltas.

Suggested repair authority for the approved contract: fix unambiguous Markdown
and structure in descriptions, preserve a before/after audit record, and
verify rendering/readback. Route missing facts to the change owner; never
invent verification, acceptance, approval or closure evidence. Never rewrite
immutable coordination, other people's comments, or historical decisions.
Substantive disposition, scope and disputed wording require the existing owner
or governor route. No new automated merge veto is proposed.

Boundaries: change authors remain responsible for descriptions. The steward
checks continuing work-item quality; it does not replace the contribution
auditor, code/security reviewer, build-engineer or governor. The documentation
estate steward continues to own adopter-facing documentation consistency;
GitLab work-item readiness is a distinct surface.

## Acceptance and rollout proposal

Pilot on active RUW items plus ten recently completed MRs and ten issues.
Measure missing-information and rendering defects, recurrence, false positives,
repair completion and time/model usage. Every pilot finding gets a disposition
and evidence; do not call the role validated merely because its YAML parses.
Agree the named player, wake mechanism, cursor, edit boundary and initial
backlog budget in Gate 1 before activation. No promised token savings yet.

Risks: bureaucracy, destructive rewriting, competing editors, and checklist
compliance hiding poor explanations. Controls: short descriptions proportional
to the change, explicit edit ownership, preserved history, rendered examples,
and no new per-item gate. Challenge whether extending an existing role can
achieve this with less overhead before selecting a permanent new role.
