---
id: public-corner-bulletin-board-semantics
status: pending-gate1
topic: public-corner-bulletin-board
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: Public Corner guidance must keep governance-relevant debate on-record without creating a place for secrets, private data, or unreviewed external-AI prompt material.
  - role: contribution-auditor
    status: consulted
    reason: The plan changes coordination semantics and must preserve auditability, role authority, append-only records, and the advisory-versus-binding boundary.
  - role: fact-checker
    status: consulted
    reason: Public and academic terminology must distinguish current conventions from validated, enforced, or proposed framework capabilities.
  - role: academic-peer-reviewer
    status: consulted
    reason: The terms should remain defensible for the governing-governors paper and avoid overstating what a deliberation buffer proves.
---

# Plan: Public Corner and Bulletin Board Semantics

## Intent

Rename the proposed "noisy room" or "deliberation room" pattern to
**Public Corner**, and define its relationship to the existing coordination
channel, which this plan treats as the **Bulletin Board**.

The goal is to support high-clockspeed, multi-agent review without flooding the
coordination channel or hiding the debate that influenced governance outcomes.
The Public Corner is a readability and synthesis pattern, not a second source
of authority.

## Background

Coordination records 240, 241, and 244 already converge on the useful split:

- a deliberation space for objections, reviewer notes, alternatives, evaluator
  traces, and role debate;
- a compact bulletin-board channel for stable obligations, handoffs, decisions,
  waivers, approvals, completions, and other state-changing records.

The same need appears in the academic draft's discussion of high-clockspeed
agent deliberation. As more LLMs, evaluators, local tools, and role-specific
agents participate, AGF needs a way to absorb fast advisory work while keeping
binding governance legible to humans.

"Public Corner" is the proposed public name because it signals that the space
is visible, on-record, and advisory. It avoids the unserious feel of "noisy
room" while preserving the useful idea: not every intermediate objection should
be a top-level governance event, but governance-relevant debate must not vanish.

## Definitions

**Bulletin Board** means the durable coordination channel. It contains compact,
append-only records that change or report governance state: claims, handoffs,
decision records, acknowledgements, completions, fast-track waivers, and stable
reviews or notes that other agents must be able to resume from.

**Public Corner** means an on-record deliberation buffer. It may contain rough
review notes, evaluator outputs, competing interpretations, non-binding role
positions, exploratory objections, and traces from models or tools. It is public
to the project, versioned, and reviewable.

**Synthesis record** means the compact summary that lifts Public Corner material
into the Bulletin Board. It states what was considered, which objections remain,
which objections were resolved or waived, and what decision or next action is
being requested.

**Lift rule** means that any Public Corner material that materially influences a
Gate 1 decision, Gate 2 decision, release, rule change, public claim, or
downstream obligation must be cited and summarized in a Bulletin Board record
before the outcome becomes binding.

## Policy

1. The Bulletin Board remains the coordination channel of record.
2. The Public Corner is on-record, durable, versioned, and reviewable.
3. Public Corner material is advisory by default.
4. Public Corner material does not create binding obligations until it is lifted
   into a valid coordination record.
5. Binding governor decisions remain in the Bulletin Board.
6. Tools, evaluators, and LLM agents may contribute evidence or recommendations,
   but they do not become governance authorities by volume or speed.
7. Unresolved blocking objections in the Public Corner must be summarized in the
   synthesis record before a gate can be approved.
8. If no Public Corner material was used for a gate decision, the handoff or
   decision record should say so explicitly.
9. Public Corner artifacts must not contain secrets, private data, credential
   material, or external content that the project is not allowed to store.

## Implementation Scope

This slice is documentation-first and deliberately avoids a new physical channel
until the semantics are accepted.

- Define Public Corner, Bulletin Board, synthesis record, and lift rule in the
  adopter-facing docs.
- Update coordination guidance to classify current message kinds by lifecycle
  effect: state-changing, advisory, and supporting evidence.
- Add gate guidance that handoffs should cite Public Corner material or state
  that none was used.
- Update the academic draft terminology so "noisy room" is treated as an
  internal precursor and "Public Corner" is the stable pattern name.
- Add a small synthesis checklist for future use by reviewers and evaluators.

## Out of Scope

- Creating a new `docs/public-corner/` or `docs/deliberations/` directory in
  this slice.
- Schema changes that require handoffs to cite deliberation artifacts.
- Validator enforcement of lift-rule compliance.
- UI, dashboard, routing, or notification tooling.
- Treating Public Corner material as private, ephemeral, or binding.
- Changing historic coordination records.

## Acceptance Criteria

- Public docs define Public Corner and Bulletin Board consistently.
- Coordination guidance states that the existing coordination channel remains
  the authoritative Bulletin Board.
- Gate guidance explains how to cite Public Corner material or explicitly state
  that none was used.
- The advisory versus binding boundary is stated for agents, evaluators, tools,
  and human reviewers.
- The academic draft uses terminology compatible with the public docs.
- No new record location is introduced without a follow-up plan.

## Validation

- `git diff --check`
- `mix agov.plan.validate ../../docs/plans/public-corner-bulletin-board-semantics.md`
- `mix agov.coord.validate ../../docs/coordination/<handoff>.md`
- Documentation search confirms no new public-facing use of "noisy room" as the
  stable pattern name.

## Risks

- Public Corner could become a dumping ground unless synthesis remains required.
- A separate deliberation pattern could fragment the audit trail if agents fail
  to cite it from gate records.
- The term may be too informal for academic prose; academic text should pair it
  with "on-record deliberation buffer" on first use.
- Tooling pressure may tempt the project to enforce the pattern before the
  social semantics are stable.
