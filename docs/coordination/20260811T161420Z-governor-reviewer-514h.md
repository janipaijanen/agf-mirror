---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260811T161420Z-governor-reviewer-514h
timestamp: 2026-08-11T16:14:20Z
model: human
immutable: true
references-to:
  - 20260811T160349Z-contribution-auditor-ga1r.md
  - 20260811T151701Z-planner-architect-dh7h.md
  - 20260811T105210Z-governor-reviewer-mh8p.md
topic: diligence-gate1-baseline-pinned-approved
requires-ack: true
artifacts_owned: []
---

# Governor decision: baseline-pinned due-diligence plan Gate 1 approved

Direct human decision in chat, transcribed by Codex. After receiving the
complete contribution-auditor CLEAR review `ga1r`, the Governor stated:
“Gate 1 approved.”

## Exact approved scope

Binding Gate 1 approval is granted to MR !1423 at exact reviewed head
`884edc7d1cbb36f8084ef0381cac1a9aa54f8dc5`, plan
`docs/plans/20260811T110333Z-diligence-gate1-baseline-pinned-plan.md`.

At decision time:

- MR !1423 is open, non-draft, targets `main`, is mergeable and
  conflict-free, and has all blocking discussions resolved.
- Exact-head pipeline `2751077730` is SUCCESS; required job
  `agov:governance:validate` (`15835867038`) is SUCCESS and
  non-allow-failure.
- Contribution-auditor review `ga1r` is COMPLETE/CLEAR with zero
  blockers and is durably present on `main` through MR !1435.

The approval includes the bounded, non-substantive plan metadata update
needed to project this decision into the plan front matter: change
`status` to `approved` and record this exact Governor decision as the
Gate 1 authority. That metadata-only projection does not reopen the
approved technical scope. Any substantive plan, prompt, evidence,
transport, role, sequencing, or acceptance-contract change requires a
fresh exact-head review and a new binding Gate 1 decision.

## Authorized next sequence

1. Planner-architect updates only the plan's Gate 1 status/authority
   metadata and the live MR description as needed, validates the plan,
   and lands the approved plan through MR !1423.
2. After the approved plan and this decision are both durable on
   `main`, coder-tester may begin Candidate A exactly as specified:
   acquire and assemble the pinned evidence package, run the single
   governed synthetic transport preflight, and produce the six exact
   immutable attachments and Candidate A review evidence.
3. Candidate A requires a fresh exact-head contribution-auditor
   implementation review and separate binding Governor Gate 2 A.
4. No real Gemini due-diligence submission occurs before Gate 2 A.
   Candidate B remains unauthorized until the single real response
   exists under that Gate 2 A authority.

This Gate 1 decision does not grant Gate 2 A or Gate 2 B, does not
authorize retries, does not authorize a CI change, and does not permit
planner-architect to write Candidate A implementation, fixtures, tests,
or CI. Coder-tester owns implementation; build-engineer owns any
separately justified CI work.

## Existing held candidates

This approval satisfies the binding Gate 1 portion of `mh8p`'s
containment condition because the approved plan pins the pre-candidate
v2.5.0 baseline. Treat that containment transition as operational only
after the approved plan metadata is merged to `main`. It does not by
itself merge MR !1382 or MR !1383: before either held candidate is
merged, recheck its previously approved exact head, successful pipeline,
current-main compatibility, and the continuing applicability of its
binding Gate 2 decision.

## Handoff

Planner-architect should perform the bounded metadata projection and
plan publication now, then hand Candidate A to coder-tester with this
decision and the approved plan as mandatory inputs. No further Governor
redispatch is needed for those already-authorized steps.
