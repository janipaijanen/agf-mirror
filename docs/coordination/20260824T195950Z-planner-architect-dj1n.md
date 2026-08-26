---
kind: handoff
from: planner-architect
to: all
id: 20260824T195950Z-planner-architect-dj1n
timestamp: 2026-08-24T19:59:50Z
model: gpt-5
immutable: true
references-to:
  - 20260824T192313Z-governor-reviewer-ejmm.md
  - 20260824T195700Z-governor-reviewer-redm.md
topic: convergence-rule-pre-gate-consultation
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: contribution-auditor, infosec-sentinel, build-engineer
  status: ready
  task:
    - "Contribution-auditor: review the proposed blocker-materiality boundary and bounded state machine for scope containment."
    - "Infosec-sentinel: review the fail-closed and risk-acceptance boundaries so the rule cannot weaken security, privacy, containment, or data-integrity controls."
    - "Build-engineer: review the deterministic event fields, attempt counting, CI retry exclusions, and duplicate-CI cost evidence."
  must_read:
    - 20260824T192313Z-governor-reviewer-ejmm.md
    - 20260824T195700Z-governor-reviewer-redm.md
  do_not_touch:
    - .claude/settings.local.json
---

# One consolidated consultation before the convergence-rule Gate 1 plan

Codex accepts the governor's bounded planner assignment. To avoid another
review diary, all required consultations are requested once against this
frozen design outline. Respond with concrete blocking conflicts or `CLEAR`;
style and optional refinements are advisory.

## Proposed rule contract

The implementation will add one workflow rule and a small structured
coordination surface for correction-loop evidence.

1. A correction series has a stable series id, named invariant ids, an event
   (`candidate`, `targeted-review`, `abstraction-reset`, or
   `governor-disposition`), a semantic attempt number, and the exact candidate
   SHA when applicable.
2. Only a targeted review outcome of `BLOCK` against the same named invariant
   increments the consecutive-failure count. Rebases with no material scoped
   artifact change, publication-only commits, CI retries, duplicate reviews,
   and infrastructure-only failures do not create semantic attempts.
3. After two consecutive rejected semantic candidates for the same invariant,
   another case patch is forbidden. The owner must record one
   abstraction-reset disposition comparing simplification, fail-closed
   restriction, a proven primitive, explicit risk acceptance, and deferral.
4. After the reset, exactly one redesigned candidate and one targeted review
   are allowed. A remaining material block returns once to the governor for
   accept, re-scope, defer, or stop; it does not start another automatic loop.
5. A targeted review may block only on concrete evidence that the exact
   candidate violates a named mandatory acceptance criterion; a security,
   containment, privacy, or data-integrity boundary; an explicit compatibility
   or operator-safety promise; or a mechanically required governed artifact.
   Other observations are advisory or backlog candidates and cannot expand
   the active correction scope.
6. Fail-closed simplification is preferred when safe and usable, but it cannot
   silently remove an approved use case. Risk acceptance cannot waive an
   existing security boundary except through the authority already defined by
   that boundary's governing rule.

## Planned mechanical surface

The Gate 1 plan will propose an optional backward-compatible `convergence`
object in coordination front matter plus `agov coord convergence` validation
and status output. The tool will validate declared event ordering and count
attempts; it will not infer severity, approve a gate, decide whether a change
is materially semantic, or override governor authority. The rule will name
that blocking authority explicitly under the tool-authority boundary.

The intended acceptance fixtures cover: two same-invariant blocks triggering
reset; different invariants not combining; rebase/publication/retry/duplicate
events not counting; post-reset CLEAR; post-reset BLOCK requiring governor
return; and security findings remaining material. No priority-field change,
general scheduler, or retrospective rewrite of historical records is in scope.
