---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260813T170416Z-coder-tester-ntea
timestamp: 2026-08-13T17:04:16Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260813T161358Z-coder-tester-kpt5.md
  - 20260813T163925Z-contribution-auditor-qebb.md
  - 20260813T164610Z-coder-tester-qrmm.md
topic: cross-ai-independent-review-rule-claude-consultation
requires-ack: false
artifacts_owned:
  - docs/coordination/20260813T170416Z-coder-tester-ntea.md
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Critically assess and revise the proposed cross-AI independent advisory review rule and its fresh-session fallback, then publish a recommendation for governor consideration.
  must_read:
    - docs/coordination/20260813T161358Z-coder-tester-kpt5.md
    - docs/coordination/20260813T163925Z-contribution-auditor-qebb.md
    - docs/coordination/20260813T164610Z-coder-tester-qrmm.md
    - agf/agents/MANIFEST.yaml
    - agf/rules/always/gate-self-test-rule.md
    - agf/rules/workflow/systematic-code-review-rule.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/1475
  do_not_touch:
    - .claude/settings.local.json
    - frozen v2.6.0 and v2.7.0 release contents
---

# Claude consultation: cross-AI independent advisory review rule

The governor likes the following proposal and asks for the real
Claude/planner-architect's critical assessment before deciding whether to
commission governed rule work:

> When an AI actor authors or implements a substantive governed artifact,
> another available AI actor must perform the independent advisory review
> before the human Gate decision. If no other authorized AI actor is reasonably
> available, a fresh-context session may perform the review under a different
> governed review role.

## Proposed independence ladder

1. **Different AI actor:** independent cross-AI review.
2. **Fresh isolated session, different governed review role:**
   independence-limited fallback.
3. **Same session or inherited author context:** self-review only; never claim
   independent verification.

The fallback is intended to be narrow:

- “Unavailable” means no other authorized AI actor can review within the
  applicable review window, not convenience, preference, or expected
  agreement.
- The fallback reviewer starts from the exact artifact SHA and published
  governance evidence, without the author's conversation or hidden reasoning.
- The session operates under a different authorized review role and records
  model/provider/session provenance, exact reviewed SHA, and coverage.
- The result is labelled `fresh-context same-actor review`, not fully
  independent review.
- The author does not steer the review after it starts.
- Findings and coverage are published before the human Gate decision.
- The governor decides whether the evidence is adequate.
- Same-session role switching never satisfies the fallback.
- Security, release, authority, or rule changes may require an explicit
  governor waiver before using the fallback.

## Requested adversarial assessment

Please argue both for and against the proposal, preserve material dissent, and
recommend a precise rule shape. In particular:

1. Refine or reject “available”, “reasonably available”, and “applicable review
   window” so the rule cannot be gamed or create indefinite deadlock.
2. Test `actor`, `session`, and `role` terminology against AGF's role/actor
   separation. Define what counts as a different AI actor when the provider,
   model family, account, session, or orchestration wrapper differs.
3. Identify false-independence, correlated-error, author-steering,
   provenance, cost, latency, and review-shopping risks.
4. Recommend applicability by artifact/risk class: Gate 1 plans,
   implementations, governance rules, security-sensitive changes, and release
   actions. State defensible exemptions for trivial, mechanical, and
   coordination-only changes.
5. Decide whether the fallback always needs an explicit governor waiver, only
   for high-risk classes, or can be used by a documented deterministic rule.
6. Propose the exact rule home and name, and whether this should be a new rule
   or an amendment to an existing rule. Identify any schema or coordination
   evidence fields needed; do not implement them.
7. Test the recommendation against the recent evidence: the `nqp8` actor
   provenance correction and `qebb`'s accurate disclosure that same-session
   author review was self-review rather than the outstanding non-self
   consultation.

## Scope and release boundary

This is consultation and backlog discovery only. It authorizes no rule edit,
schema edit, implementation, or Gate decision. Record any resulting candidate
as an **unnumbered backlog item**. It must not enter the frozen v2.6.0 or v2.7.0
contents. Under the governor's release-freeze decision, release placement can
be reconsidered only during post-release sequence/backlog revalidation or by a
new binding governor decision.
