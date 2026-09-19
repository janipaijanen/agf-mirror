---
id: 20260910T133436Z-agents-md-bridge-record-discipline
status: approved
gate1_approved_by: governor-reviewer
gate1_approved_at: 2026-09-11T17:13:51Z
topic: agents-md-gate-discipline-bridge-record-addition
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Documentation change to AGENTS.md gate-discipline prose. No code,
      schema, credential, CI, validator, or artifact change; no secret or
      protected-content surface. It tightens (does not relax) the authority
      required before an irreversible action.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Gate-discipline normative text in a loader file. Two review rounds:
      `20260919T174055Z-contribution-auditor-51gs` found the originally
      approved text let a plan's `status` reach `approved` from a bridge
      note alone (BRIDGE-G2-1) and overstated `agov coord pending`'s
      guarantee as governor-qualified when the implemented resolver has
      no such check (BRIDGE-G2-2) — both corrected in the operative
      `AGENTS.md` paragraph and this plan's own "Change" section.
      `20260919T182246Z-contribution-auditor-7kmt` re-reviewed the
      corrected head and confirmed both resolved, with one residual
      finding (BRIDGE-G2-R1, this section and the projection table below
      still asserted the rejected model) now also fixed. Evidence:
      Codex review `6pms` POLL-G1 (2026-09-10); the same failure class
      AGENTS.md already records as having recurred three times on
      2026-07-21.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# AGENTS.md gate discipline — bridge-record addition (Gate 1)

**Gate 1: APPROVED** at exact source head `f822ecb5374c105fca288dec2762f281a7893b0b`
per `20260911T171351Z-governor-reviewer-m15q` (governor Jani: "all at gates
are approved", transcribed via the Codex session; recorded on
`codex/ci-shallow-history-report`, MR !2705). coder-tester implements;
implementation Gate 2 remains separate.

## Provenance

- Codex non-author review `20260910T130819Z-contribution-auditor-6pms`
  **POLL-G1**: this session's gate transitions cited `planner-architect`
  bridge notes, not `governor-reviewer` records.
- Governor direction 2026-09-10 ("approved. proceed as proposed"): file
  the going-forward pattern as its own small Gate 1.
- Ratification record `20260910T133145Z-governor-reviewer-q862` establishes
  the pattern by governor decision; this plan makes it durable and
  discoverable in the loader.
- AGENTS.md already records this exact failure class recurring three times
  on 2026-07-21 (the "same-role self-ack" paragraph).

## Change (one edit)

`AGENTS.md`, **Gate discipline** section — one new paragraph
("Conversational decisions and bridge records") immediately after the
existing "A same-role self-ack does not supply governor authority"
paragraph. It states:

1. A bridge record (a role `note` transcribing a live governor decision) is
   **not binding authority on its own** and must say so.
2. A plan's `status` stays `pending-gate1` even when the bridge record
   accurately transcribes a live "approved" decision — the machine/agent-
   visible `approved` state must not appear before the governor record
   exists, since another session or agent could treat `approved` alone as
   authorization to implement.
3. **Irreversible** actions (merge to a protected branch, tag, forge
   closure) always wait for the governor record — unless the governor
   authorizes that specific action in the same session, captured in the
   next `governor-reviewer` record **before** the irreversible action
   executes.
4. An unreferenced bridge record after the session is a gap; `agov coord
   pending` surfaces it — but its resolution check has no governor-role
   qualification (any record's `references-to` clears it), so a cleared
   entry means "something referenced it," not "governor ratified it." Only
   an actual `governor-reviewer`-authored `ack`/`decision-record` supplies
   binding authority. `q862` is the worked example.

**Corrected 2026-09-19** per independent contribution-auditor review
`20260919T174055Z-contribution-auditor-51gs` (findings BRIDGE-G2-1,
BRIDGE-G2-2): the original text at Gate 1 approval permitted a plan's
`status` to move to `approved` from a bridge note alone (the authority
loophole this plan exists to prevent) and overstated `agov coord
pending`'s actual mechanical guarantee (no governor-role check exists in
the implemented resolver). Both are fixed above and in the `AGENTS.md`
edit this plan carries. This correction is a same-cycle text fix, not a
new decision — it stays inside the Gate 1 scope already approved by
`m15q`; a fresh exact-head pipeline and re-review are required before
Gate 2.

## Impact (`downstream-impact-rule`)

**PREVENTIVE → treat as ADDITIVE.** It makes an implicit constraint
explicit (a non-governor record is not governor authority — already the
rule for same-role self-acks). It does not restrict previously-permitted
behavior: bridge records were never valid standalone authority; this says
so and adds the reversible/irreversible split as guidance, which is a
narrowing of practice toward the existing requirement, not a new
prohibition on anything that was allowed.

- Foundry: no — codifies what this session already did after `6pms`.
- Direct adopters: ADDITIVE — clearer guidance for any adopter whose
  governor works conversationally with an assistant.
- Downstream: none — no adopter-produced artifact shape changes.
- Migration: n/a.
- Evidence: `6pms` POLL-G1; AGENTS.md's own 2026-07-21 recurrence note.

## Rule-projection screen

| Surface | Disposition |
|---|---|
| Normative owner | `AGENTS.md` **Gate discipline** section is itself the owner for this project's gate-discipline projection; the canonical gate model is `framework.md`. `updated` — this addition. |
| Adopter-facing explanation | `not applicable` — `AGENTS.md` *is* the adopter-facing loader; there is no separate Pages page for the conversational-decision case, and the addition is short operational guidance, not a rule needing its own explainer. If a Pages "governance workflow" page later covers gate discipline, it points here. |
| Runbook/situation map | `not applicable` — no runbook step changes; the guidance is "what a bridge record is", not a procedure. |
| AI landing/loaders | `updated` — `AGENTS.md` (this edit). `agf/templates/AGENTS.md` and `.claude/CLAUDE.md`: `not applicable` — `.claude/CLAUDE.md` is a one-line `@../AGENTS.md` import (propagates); `agf/templates/AGENTS.md` is the neutral adopter template and does not carry foundry's session-specific gate-discipline elaborations (it already omits the "same-role self-ack" paragraph's foundry incident detail). |
| Validation/drift evidence | `advisory only` — no mechanical check. `agov coord pending` surfaces a generic unresolved `requires-ack` reference obligation, but its resolver has no governor-role qualification and does not enforce or prove governor ratification — a non-governor reference also clears the row. Binding authority is supplied only by an actual `governor-reviewer`-authored `ack`/`decision-record`, confirmed at the governor's own session close. |
| Distribution | `AGENTS.md` is project-local (foundry's own loader), not shipped in an AGF release archive; `agf/templates/AGENTS.md` is what ships and is deliberately not changed here. Confirmed project-local by design. |

## Test and acceptance

- `git diff` shows exactly one paragraph added to `AGENTS.md`, nothing
  else.
- `agov validate` / `agov.validate.assistant_loader` still pass (the AGF
  version comment is untouched).
- Contribution-auditor advisory row dispositioned at Gate 1.
- Acceptance = a governor Gate 1 decision record.

## Rollback

Revert the one-paragraph edit. No other artifact touched.

## Related

- `docs/coordination/20260910T133145Z-governor-reviewer-q862.md` — the
  ratification record and the pattern's first application.
- `docs/coordination/20260910T130819Z-contribution-auditor-6pms.md` —
  POLL-G1.
- `agf/rules/always/coordination-message-rule.md`,
  `agf/rules/workflow/coordination-publication-rule.md` — the record-kind
  and publication rules this operates within (unchanged).
