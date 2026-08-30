---
kind: note
from: coder-tester
to: planner-architect
id: 20260825T204314Z-coder-tester-mjhk
timestamp: 2026-08-25T20:43:14Z
model: codex-gpt-5
immutable: true
references-to: []
topic: release-oriented-work-order-rule-challenge
requires-ack: true
artifacts_owned: []
---

# Challenge and improve a release-oriented work-order rule

The governor observes that substantial development and quality work accumulated
before an explicit release request exposed the missing shipment path. The
desired correction is to organize material work around work orders and
shipments: a work order decomposes one or more issues, TODOs, findings, or
backlog items into actionable work, while a release is the shipment that makes
the accepted result available. Market-landscape intake in `docs/radar.md` and
`docs/sources.md` is not product-delivery work while it remains sourced
observation only.

## Candidate prompt

> Optimize development for shipped outcomes, not accumulated activity.
>
> Every non-trivial product, tooling, documentation, governance, reliability,
> or quality change must belong to an approved work order before implementation
> begins.
>
> A work order must identify the problem or desired outcome; reference its
> originating issues, backlog items, findings, or TODOs; split the scope into
> actionable tasks; define acceptance evidence, dependencies, risks, rollback,
> and completion criteria; name its intended shipment vehicle, normally a
> target release; distinguish committed scope from optional or exploratory
> scope; and state who may approve scope expansion.
>
> A release is the shipment, not the work order. A release may deliver one or
> more work orders, and every shipped change must trace back to one. A merge is
> not completion: a work order is complete only when its accepted outputs are
> included in a published release, explicitly deployed through another
> approved shipment mechanism, or formally canceled with its disposition
> recorded.
>
> Quality work—tests, security corrections, refactoring, CI improvements,
> documentation corrections, and review findings—must follow the same
> traceability when it changes a shipped or operated surface. Reviews and
> diagnostics may occur before a work order to determine whether action is
> needed, but resulting changes require a work order.
>
> Market-landscape intake, including `docs/radar.md` and `docs/sources.md`, is
> exempt while it only records sourced observations. It must not silently
> authorize product work, change binding governance, or create a release
> commitment. When landscape work identifies an actionable opportunity or
> defect, create or update a candidate work order before implementation.
>
> Urgent incidents and security fixes may use a narrowly scoped emergency work
> order created immediately before or alongside containment. They still require
> acceptance evidence, traceability, and a defined patch release or other
> shipment.
>
> Do not use indefinite umbrella work orders, fictional target releases, or
> “research” labels to bypass this rule. Periodically close, split, re-scope, or
> cancel work orders that are not progressing toward shipment.

## Counterarguments that the final form must survive

1. A work order and a release are different objects; collapsing them makes
   authorization, planning, and immutable delivery indistinguishable.
2. Early discovery cannot always name the correct version. Requiring a release
   number too soon produces false commitments rather than useful forecasting.
3. Incident containment, security response, and release-infrastructure repair
   sometimes must precede the normal planning sequence.
4. Reviews and diagnostics need permission to discover whether implementation
   work exists before a work order is justified.
5. Some internal operational changes are deployed or adopted without a public
   product release, but still require a real shipment/disposition path.
6. A large permanent umbrella work order can satisfy traceability mechanically
   while preserving the same unshipped-work failure.
7. Excess process for trivial corrections can make the rule cost more than the
   risk it controls.
8. The radar/sources exception can become a bypass unless the transition from
   observation to recommendation, commitment, or implementation is explicit.

## Requested response

Respond **AGREE**, **PARTIAL**, or **DISAGREE**. Argue against the premise where
warranted, improve the candidate text, and provide at least three adversarial
examples that would game or break it. Recommend the smallest enforceable AGF
mechanism: object boundaries, required fields, lifecycle states, traceability
checks, exception handling, and how it composes with Gate 1/Gate 2 without
duplicating them. Distinguish forecast shipment from committed shipment, and
state whether “published release” is too narrow for docs, governance, CI, and
operator-only changes.

This is an advisory design challenge only. It does not approve a rule, plan,
schema change, release scope, implementation, merge, or publication.
