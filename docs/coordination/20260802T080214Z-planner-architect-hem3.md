---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260802T080214Z-planner-architect-hem3
timestamp: 2026-08-02T08:02:14Z
model: codex-gpt-5
immutable: true
references-to: []
topic: expectation-signals-consensus-review
requires-ack: false
artifacts_owned:
  - docs/proposals/expectation-signals-consensus.md
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Independently try to reject or materially improve the expectation-signals foundation.
    - Answer all twelve consensus questions and separate blockers from advisories.
    - Publish a read-only review; do not modify the proposal or implement a rule, template, validator, or pilot.
  must_read:
    - docs/proposals/expectation-signals-consensus.md
    - agf/rules/always/verifiable-claims-rule.md
    - agf/docs/site/reference/maturity-vocabulary.md
    - agf/docs/site/reference/review-findings.md
    - agf/docs/site/docs/concepts/philosophical-lenses.md
    - docs/proposals/same-sha-ci-pipeline-dedup-consensus.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/60
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/100
  do_not_touch:
    - docs/proposals/expectation-signals-consensus.md
    - agf/rules
    - agf/schemas
    - main
---

# Critical consensus review requested: expectation signals

The governor asked Codex and Claude to tear down or improve a mechanism for
making material caveats harder to bury in prose, and to seek consensus only if
the foundation survives.

The draft already rejects subjective `UNEXPECTED` and rhetorical
`CONTROVERSIAL` badges, attacks warning fatigue and stale banners, separates
signals from findings/authority, and proposes a falsifiable pilot. This is not
a request to agree with that narrowing. Look for a simpler existing mechanism,
counterexamples to the threshold, semantic overlap, AI misuse, accessibility
failure, and lifecycle costs that justify rejecting the foundation entirely.

Publish the independent response through a new coordination MR and bridge it
to issue #100 and the proposal MR. Do not edit this proposal, issue a gate
decision, or implement anything. The governor will decide whether a revised
consensus draft deserves a separate Gate 1 pilot plan.
