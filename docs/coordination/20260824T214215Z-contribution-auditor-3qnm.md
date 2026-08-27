---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260824T214215Z-contribution-auditor-3qnm
timestamp: 2026-08-24T21:42:15Z
model: gpt-5.6-sol
immutable: true
references-to:
  - 20260824T195700Z-governor-reviewer-redm.md
  - 20260824T203312Z-contribution-auditor-61gp.md
  - 20260824T212325Z-contribution-auditor-17b8.md
topic: coordination-priority-and-material-challenge-planning
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - "Draft one Gate 1 plan for minimal effective-priority routing and a bounded material challenge-response contract for actionable coordination."
  must_read:
    - 20260824T195700Z-governor-reviewer-redm.md
    - 20260824T203312Z-contribution-auditor-61gp.md
    - 20260824T212325Z-contribution-auditor-17b8.md
  do_not_touch:
    - .claude/settings.local.json
---

# Plan actionable coordination: priority plus material challenge

The governor requests one combined proposal addressing two related gaps:

1. coordination obligations have no mechanically effective priority, so a
   governor can declare work urgent but `agov handoff next` cannot reliably
   put it first; and
2. a material proposal can cite rules without making a compact, challengeable
   case for why its authority, evidence, design, and external practice fit the
   work at hand.

Draft one proportional Gate 1 plan. Keep publication prompt and reuse existing
roles and message kinds.

## Required contract

### Effective priority

- Add the smallest structured priority surface that lets `agov handoff next`
  deterministically select the highest-priority authorized obligation, with a
  stable tie-breaker.
- Distinguish a sender's requested priority from binding priority. Only the
  governor-reviewer, or authority explicitly delegated by an existing rule,
  may elevate an obligation above the default queue.
- Require a reason and authority reference for elevated priority. Missing or
  unauthorized elevation is advisory or rejected according to named rule
  authority; it must never silently outrank valid work.
- Preserve backward compatibility: records without priority retain the
  default class. Do not retrospectively migrate historical records.
- Prevent priority inflation. Prefer a small vocabulary such as `immediate`,
  `normal`, and `deferred` unless evidence proves another tier necessary.
- Document exactly how a governor priority decision attaches to an existing
  handoff and how Codex, Claude, or another cold-starting agent sees and takes
  that work.

### Material challenge-response

- Before acting on a material proposal, the submitter states the intended
  outcome, governing authority, project evidence, applicable external or
  industry practice with relevance and limits (or reasoned not-applicable),
  simplest sufficient alternative, affected value surfaces, and what evidence
  could disprove the proposal.
- The existing contribution-auditor challenges material defects. Activate a
  specialist role only when the subject requires it; do not create a new role
  merely to provide opposition.
- The submitter responds once by accepting and correcting, defending with
  evidence, or escalating the unresolved binding decision. The result is
  `CLEAR`, `CORRECT`, or `ESCALATE` using existing coordination kinds and
  causal references unless a demonstrated gap requires more structure.
- A challenge blocks action only for a material authority, requirement,
  correctness, safety, evidence, maintainability, user-value, or required-gate
  defect. Wording preferences, optional polish, and equally sufficient
  alternatives remain advisory.
- Industry practice supports reasoning but does not override AGF rules, role
  authority, project evidence, or the governor. Require a source and relevance
  statement; prohibit cargo-cult citations.
- Do not block or delay publication of the proposal or challenge. The bounded
  review may block acting on the disputed proposal, not making the audit trail
  visible.
- Apply this only to material decisions, waivers, plans, and actionable
  handoffs. Routine notes, acknowledgements, and completions stay lightweight
  unless they themselves introduce a material claim or obligation.
- Default to one consolidated challenge and one response. Repeated rejection
  follows the approved convergence/abstraction-reset path; it does not begin a
  phrase-by-phrase loop.

## Planning and acceptance boundary

This record is the proportional pre-work plan for preparing the Gate 1 plan:
outcome and scope are above; the planner-architect owns the draft; the
governor-reviewer retains Gate 1 and priority authority; contribution-auditor
consultation is required; infosec consultation is required only if the design
creates a new blocking or trust boundary. Acceptance requires:

1. one coherent operator flow from published handoff to deterministic take;
2. one bounded challenge-response flow with explicit materiality and exit;
3. old records remaining valid;
4. no general scheduler, autonomous governor, new adversarial role, new
   coordination kind, mandatory industry citation for routine work, or
   pre-publication review gate; and
5. explicit dispositions for affected schema, `agov`, role/rule,
   adopter-documentation, testing, migration, and operational surfaces.

Return one plan candidate for a single consolidated review. If priority and
challenge cannot share one minimal implementation without avoidable coupling,
state that material reason and split them at Gate 1 rather than hiding the
coupling.
