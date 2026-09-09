---
kind: note
from: planner-architect
to: all
id: 20260905T202153Z-planner-architect-hr6j
timestamp: 2026-09-05T20:21:53Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260905T194621Z-planner-architect-htar.md
  - 20260905T200607Z-coder-tester-hqps.md
  - 20260905T201822Z-planner-architect-rnhh.md
topic: deferred-rule-non-conflict-and-status-statemachine-post-release
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Queued: make `agf/rules/` non-conflicting; governor proposes a rule `status:` state machine

Mirroring a binding instruction the governor gave in chat, so it survives
context compaction and is visible to Codex (per learning
`20260905T194427Z-wq4k`, gap 3: binding instructions must reach the channel).

## Trigger

**After v2.9.0 is pushed and delivered.** Not before. Release-critical
runner routing (MR !2362 / !2373 revised / !2392 / !2383) stays ahead of
this and of the four coordination-protocol slices in `rnhh`.

## The work

Claude and Codex jointly design how to make `agf/rules/` **non-conflicting**.
The governor's proposed mechanism, to argue hard against or improve:

> A rule carries a `status:` field. Once a rule is approved, `status:` is the
> only field that may change. The lifecycle is a strict state machine, e.g.
> `draft -> in-review -> {draft | approved | discarded}`.

## Claude's opening position (for the joint design, not a decision)

1. **A lifecycle state machine is worth having** — it makes rule status
   machine-checkable the way `status: pending-gate1 -> approved` already does
   for plans.
2. **"Only `status:` mutable after approval" is too strong and contradicts
   observed practice.** The current rules are full of dated in-place
   corrections ("Revised 2026-08-02", "Corrected 2026-07-21"). Rules are
   living documents. A workable model: `approved` is a stable state; a
   substantive change transitions the rule back to `in-review` (or spawns a
   versioned successor that supersedes it), rather than freezing the text.
3. **A per-rule state machine does not make rules non-conflicting with each
   other.** Two `approved` rules can still contradict. That needs a separate
   mechanism: a canonical-owner discipline (which
   `documentation-rule-projection-rule.md` already gestures at) plus a
   semantic conflict scan — the rule-level analogue of issue #123's
   plan/file collision check.
4. **It must reconcile with the existing vocabulary**, not replace it: the
   `Provisional` header, MUST/SHOULD strength, `CUSTOMIZE` blocks, and
   `agf/docs/site/reference/maturity-vocabulary.md`.

## Boundaries

Design discussion only. Any change to the rule schema, a rule lifecycle, or
`.claude/rules` / `agf/rules` mechanics is gate mechanics — full Gate 1 plus
a `gate-self-test-rule.md` negative test. This note authorizes nothing; it
records the obligation and its trigger.
