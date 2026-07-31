---
kind: handoff
from: governor-reviewer
to: planner-architect
id: 20260728T154327Z-governor-reviewer-gkj5
timestamp: 2026-07-28T15:43:27Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260727T010500Z-governor-reviewer-km7k
  - 20260728T102219Z-governor-reviewer-njpb
topic: issue78-adopter-screening-triage
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - "Triage issue #78 from its corrected current state, preserving the reporter's retraction and count correction. Do not implement the retracted top-level docs/ regression claim."
    - "Route the two surviving adopter-facing legacy Pages URLs as corrective completion of issue #67 Slice B's already-approved complete shipped-tree sweep, including a broadened negative check that catches the bare legacy project token/Pages-host form without rewriting historical evidence records."
    - "Create or identify a dedicated Gate 1 planning issue for the adopter-owned role-roster path/tooling contract; issue #67 already decided the direction but explicitly deferred the agov configurable/default path tooling sub-slice."
    - "Create or identify a dedicated Gate 1 planning issue for requires-ack semantics: decide whether non-handoff kinds create tracked obligations or are schema-rejected, then align schema, CLI queues, docs, and adversarial tests."
    - "Route the coordination branch/ref visibility evidence into issue #76's plan as an acceptance/use-case input, avoiding a duplicate issue unless the planner finds the scope genuinely distinct."
    - "File the independently reproduced shipped test self-sufficiency failure as a separate atomic bug: v2.4.5 ships test_operations_runbook.py but omits three referenced fixtures, causing 7 tests / 4 errors in a clean archive."
    - "Return a decomposition that names each issue, applicable existing/new Gate 1 authority, owner, review roles, and release priority."
  must_read:
    - docs/plans/issue67-commissioning-defects-gate1-plan.md
    - docs/plans/issue76-coord-visibility-performance.md
    - agf/dist/release-manifest.yaml
    - tooling/rust/crates/agov-coord/src/handoff_tracker.rs
    - agf/schemas/coordination-message.schema.json
  do_not_touch:
    - issue #78's original description or correction comments
    - historical coordination/plan/academic references merely because they cite the old project as history
    - GitLab workflow labels or board lists before issue #75 Gate 1 approval
    - agf/VERSION
    - any tag or release
---

# Issue #78 verified triage and atomic routing

Issue #78 is useful adopter-screening evidence but is not an implementation
unit. The reporter has already retracted one headline finding and corrected a
second. Preserve those corrections visibly and route only the surviving,
reproduced claims.

## Verified disposition

| Report element | Disposition | Evidence |
|---|---|---|
| Top-level `docs/` is an accidental release regression | **Retracted; do not implement** | Both files are explicitly declared in `agf/dist/release-manifest.yaml`; projection evidence intentionally uses `docs/reviews/`. A mixed-ownership documentation clarification may be considered separately, but relocation would break the current contract. |
| “34 legacy paths became zero” | **Incorrect; corrective Slice B work** | Tag v2.4.5 retains adopter-facing Pages URLs in `agf/docs/governance/retrospective.md` and `learning-system.md`. The original issue #67 Gate 1 plan authorized a complete shipped `agf/` sweep and a negative gate, so this is incomplete acceptance rather than new scope. |
| Adopter roster can be overwritten | **Confirmed, planned direction but tooling deferred** | The release manifest ships `agf/agents/**`; issue #67 Slice C decided on an adopter-owned location/example manifest but explicitly required a separate Gate 1 tooling plan for agov path resolution. |
| `requires-ack: true` note is not pending | **Confirmed contract ambiguity** | Schema accepts the field and graph data exposes it, while pending/handoff computation is handoff-oriented. The resolution must be designed, not guessed: broaden obligation tracking or reject meaningless combinations. |
| Branch-local coordination visibility evidence | **Relevant to issue #76** | The performance plan must preserve correctness and make ref/base scope visible while optimizing; add this adopter scenario to its acceptance matrix. |
| Shipped runbook test lacks fixtures | **Confirmed separate bug** | Running `python3 agf/docs/build/hooks/test_operations_runbook.py` from a freshly extracted v2.4.5 framework archive yields 7 tests and 4 errors. `.claude/commands/compact-agf.md`, `docs/reference/session-resume.md`, and `docs/reference/agent-cold-start-habits.md` are absent. |

## Guardrails

- Do not treat maintainer dogfooding as independent-adopter evidence.
- Do not use one broad URL replacement across immutable/historical evidence;
  scope the corrective gate to current shipped normative/adopter surfaces or
  explicitly allow documented historical citations.
- Do not fold roster ownership and `requires-ack` semantics into a quick patch;
  both change adopter contracts and need their own Gate 1 acceptance criteria.
- Keep the archive-test defect atomic so its release self-sufficiency fix and
  negative fixture can be reviewed independently.
