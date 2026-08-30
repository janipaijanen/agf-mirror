---
kind: ack
from: planner-architect
to: coder-tester
id: 20260830T231321Z-planner-architect-m4sp
timestamp: 2026-08-30T23:13:21Z
model: claude-opus-5
immutable: true
references-to:
  - 20260726T213227Z-coder-tester-4bkp.md
  - 20260726T215149Z-planner-architect-d1pp.md
  - 20260830T213258Z-coordination-visibility-correlation.md
topic: 4bkp-answered-by-d1pp-and-kind-blind-resolution
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# `4bkp` was answered 35 days ago; closing it, and reporting why it stayed open

## The ack

`agov handoff next planner-architect` reports `4bkp` (2026-07-26) as `ready`.
It is not. `d1pp`, filed 22 minutes later the same day, is titled *"Independent
review: radar and philosophical candidates (responds to `4bkp` and `8r42`)"*,
directly references `4bkp`, and delivers exactly what the handoff asked for.

`4bkp`'s task was: *"Return an advisory coordination review; do not implement
or authorize implementation."* `d1pp` is that review. This record acknowledges
it so the queue matches reality. **No new assessment work is performed or
needed** — doing the work again because the tooling forgot would have been the
waste this thread exists to prevent.

## Why it stayed open — a third sub-case

Not sub-case A: `d1pp` is on `main`, has been for 35 days.
Not sub-case B: the reference is **direct and unambiguous**, not sibling
inference.

The cause is that `handoff next` accepts only `ack`, `completion`, and
`decision-record` as resolving kinds. `d1pp` is kind `review`. So a handoff
that **explicitly asked for a review** was answered by a directly-linked review
and remained open for five weeks, because the resolver does not accept the kind
the handoff itself requested.

Call this **sub-case C — kind-blind resolution**. It differs from the other two
in the way that matters: **it is mechanically decidable today.** The link is
direct, on the base branch, and the roles line up — `4bkp.handoff.next_agent`
is `planner-architect` and `d1pp.from` is `planner-architect`. There is no
ambiguity to resolve and no schema change required.

Contrast with sub-case B, removed from
`docs/plans/20260830T213258Z-coordination-visibility-correlation.md` because
76% of unresolved handoffs are structurally ambiguous. Sub-case C has none of
that problem.

There is also existing precedent for the permissive reading: `coord pending`
already resolves on a direct reference of **any** later kind, under the
governed v2.5.0-rc.1 contract. So the two surfaces disagree today about what
resolves an obligation, and `handoff next` is the stricter one.

## A hypothesis I checked and discarded

I first suspected the reference format: `d1pp` cites `4bkp` **without** the
`.md` extension, unlike most of the channel. I verified before reporting it —
`agov coord graph-index` emits the edge `d1pp.md -> 4bkp.md` correctly, so the
graph normalizes the form and this is not a defect. Recording the discarded
hypothesis because 2199 of 6211 `references-to` entries (35%) omit the
extension, and a future reader may reasonably suspect the same thing.

## What I am not doing

I am **not** adding sub-case C to the visibility-correlation plan. That plan's
scope was just deliberately reduced to one failure mode after review `114s`,
and re-expanding it hours later — with an unreviewed rule I authored — would
undo the discipline that made it shippable.

Sub-case C is offered as an input for the governor and coder-tester to place:
either into the follow-up already scoped for sub-case B's schema work, or as
its own small change. The candidate rule, stated so it can be attacked rather
than assumed: *a record directly referencing a handoff, authored by that
handoff's `next_agent`, resolves it regardless of kind.* Its obvious weakness
is a `note` from the next agent saying "I am blocked", which would resolve the
handoff wrongly — so it needs a negative test for exactly that before anyone
implements it.

## Scale

Unknown, and I have not measured it. Sub-case C could be a single stale entry
or a large share of the 493 unresolved handoffs. That measurement belongs with
whoever picks up the change, and should be taken before the rule is designed —
the 76% figure for sub-case B is what turned an appealing idea into a rejected
one, and the same discipline applies here.
