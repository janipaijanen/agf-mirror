---
id: T2-8-small-task-fast-path
status: pending-gate1
gate1_approved_by: ""
gate1_approved_at: ""
topic: fast-track-formal-criteria
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: no security boundaries, auth, or PII changes; prose update to framework document
  - role: contribution-auditor
    status: not-applicable
    reason: governor-reviewer Gate 1 covers the framework change; no scope or ownership ambiguity
---

# Plan: T2-8 — Small-task fast-path formalization

**Date:** 2026-06-12T16:58:14Z
**Status:** PENDING GATE 1 APPROVAL
**Depends on:** none

---

## Intent

Pattern 6 (Fast Track) in framework.md describes the concept correctly but
leaves the allowability criteria as prose bullet points. Without
machine-readable criteria, every fast-track decision is Governor-Reviewer
memory. Two gaps result:

1. Agents cannot self-check whether a task qualifies — they must ask.
2. Reviewers cannot verify a waiver against a checklist — they rely on
   implicit judgement.

`kind: fast-track-waiver` already exists in the coordination channel
(AGF-NG README). The missing piece is a formal criteria table that makes
the `allowed_when` and `forbidden_when` conditions explicit, so:
- A coder-tester can produce a well-formed waiver request.
- A governor-reviewer can approve or deny against a written standard.
- A contribution-auditor can flag `fast-track-waiver` records that don't
  meet the criteria.

---

## Scope

**One file changed:** `agents-govern/framework.md`

Replace the existing "When to use fast track" / "When NOT to use fast track"
prose bullets in Pattern 6 (lines ~948–960) with a formal criteria table.

No schema changes. No new message kinds. No new tooling. The
`fast-track-waiver` coordination kind already has its shape in the README.

---

## Implementation

### Change to framework.md Pattern 6

Replace the two prose bullet lists with:

**Allowability criteria (ALL must hold):**

| Criterion | Threshold |
|---|---|
| Files touched | ≤ 3 files |
| Shared interface change | No |
| Data model or schema change | No |
| Security / privacy / auth boundary | No |
| Active claim conflict | No — no other agent has a live claim on any file |
| Scope uncertainty | No — intent is fully stated; no design ambiguity |

**Automatic disqualifiers (ANY triggers return to full pipeline):**

- Task touches PII, session tokens, credentials, or permissions logic
- Task modifies architecture (new service, new external dependency, new data model)
- Task affects more than one agent's declared jurisdiction
- Coder-Tester is uncertain whether the task qualifies

**Required actions when fast-track is granted:**

1. Governor-Reviewer files `kind: fast-track-waiver` naming: task identifier,
   which criteria were checked, who granted the waiver.
2. Coder-Tester checks ownership before editing (no silent claim conflicts).
3. Gate 2 still applies in full — only Gate 1 is waived (per existing Pattern 6).
   After Gate 2 approval and merge, Coder-Tester files a `kind: completion`
   to close the audit trail.
4. If the task grows beyond the criteria during implementation, Coder-Tester
   stops, files a `kind: note` to governor-reviewer, and waits for a new
   Gate 1 plan before continuing.

---

## Affected files

| File | Change |
|---|---|
| `agents-govern/framework.md` | Replace Pattern 6 prose bullets with formal criteria table |

---

## Downstream impact

**Classification:** ADDITIVE

- **Foundry:** Adds explicit criteria to existing Pattern 6 prose. No
  existing pipeline step removed or changed. The `fast-track-waiver` kind
  already exists.
- **Direct adopters:** Optional adoption of the formal criteria. No
  existing contract removed.
- **Downstream:** No impact on adopters who haven't distributed
  framework.md to their own downstream.
- **Migration:** Not required. The prose is replaced with a table; the
  concept and rules are unchanged.

---

## Test strategy

No automated tests. Verification:
- Read the modified Pattern 6 section and confirm the table is present.
- Confirm the two prose bullet lists are replaced (not duplicated).
- Confirm no other framework.md sections are modified.

---

## Risks

- `agents-govern/framework.md` is a governance artifact distributed to
  adopters. The change is additive prose, not a breaking change, but any
  wording error ships to all adopters.
- Mitigation: Gate 2 review checks the diff is confined to Pattern 6
  lines only.

---

## Rollback

The change is a prose replacement in a markdown file. Rollback is a
one-commit revert with no migration required.
