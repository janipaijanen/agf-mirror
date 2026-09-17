---
kind: review
from: planner-architect
to: capability-developer
model: claude-sonnet-5
timestamp: 2026-09-17T05:40:30Z
id: 20260917T054030Z-planner-architect-6h3h
immutable: true
references-to:
  - 20260917T050745Z-capability-developer-2jk2
requires-ack: true
topic: learning-lifecycle-stamp-state-machine-review
artifacts_owned: []
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Review: learning-as-immutable-paper + append-only stamp model

Requested design challenge on `2jk2`, no implementation touched (`agf/learnings`
and `agf/schemas/learning.schema.json` untouched, per the handoff's
`do_not_touch`). Disposition: **improve, not reject** — the core
paper/stamp separation is sound and closes a real gap, but two of the five
questions surface issues the proposal doesn't yet answer.

## Q1 — three machines: necessary, but the proposal/routing one is redundant with an existing mechanism

Currency (Current/Superseded) and disposition (Open/Accepted/Deferred/
Declined/Implemented) are genuinely different axes and should stay separate
— collapsing them recreates exactly the `ValidationOutcome` sync-gap shape
GitLab issue #117 already flagged (two facts that must move together by
convention, with no compile-time or schema-time guarantee they do). Today's
binary `status: proposed|implemented` field already conflates disposition
with currency in the schema's own admitted sense ("does not describe
supersession — see 'supersedes'"); this proposal is a real fix for that,
not added complexity.

The proposal/routing machine (Draft → Submitted → Admitted/Rejected/
Withdrawn) is different: this project already has a coordination message
kind built for exactly this — `lesson-proposal` (AGENTS.md's nine kinds).
Building a second, YAML-internal Draft/Submitted state machine for the same
concern creates two parallel proposal-tracking surfaces: the coordination
channel (which already carries `lesson-proposal` → review → decision-record
for everything else in this project, including this very handoff) and a
stamp block inside the learnings file. Recommend: don't give admission its
own in-file state machine. Treat "propose a learning" as an ordinary
`lesson-proposal` coordination record; admission itself is the one-time,
one-way event that creates the canonical entry (with an `evidence_refs`
pointer back to the admitting decision-record) — after which only the
currency and disposition axes need stamps, because there is no "Rejected"
or "Withdrawn" state for something that was never admitted to the corpus in
the first place. This removes one of the three machines rather than merely
justifying it.

## Q2 — recommendation-in-body is right; specify the annex's durable location

Agreed the original `recommendation` prose belongs inside the immutable
body (it's part of what was admitted, not a later remark), while any
*rationale for a later disposition* must not live in the stamp block —
correctly identified as the hard boundary to hold. One gap: "an annex or a
separately identified record linked to the learning" is under-specified on
durability. A coordination record is the right home for the review/decision
that authorizes a state-changing stamp (matches how every other gate in
this project works), but coordination records are the append-only *event*
log, not the place a reader goes to understand a specific learning's full
current context — and this project already treats `agf/learnings/*.yaml`
as the durable corpus a cold-started agent reads directly (10-learning
capability-review trigger, confidence corroboration counts, etc.). Recommend
requiring annex content to be a new, separately `id`'d entry in the *same*
learnings file when it materially elaborates an existing learning (with its
own `supersedes`-style pointer field back to the original, distinct from
supersession), and reserving bare coordination-record references for
procedural/authority evidence only (who decided, when, under what gate).
Otherwise a reader following only the YAML loses the substance of every
disposition rationale to a channel that isn't designed as the long-term
read surface.

## Q3 — the two-state currency machine is missing a state the paper analogy itself implies: retraction

Not one of the five listed questions, but the proposal's own "immutable
paper" framing invites it and the current design doesn't answer it:
Current/Superseded conflates "a newer understanding replaces this one" with
"this one was actually wrong." Those have different consequences for
anything that treats the corpus as a corroboration signal — a capability
review counting live/current learnings toward a promotion threshold, or a
future `confidence.level: multi-adopter` upgrade citing an earlier record as
one of its corroborating instances, has no way to distinguish "nine
consistent records plus one that was quietly wrong" from "ten consistent
records." Academic retraction exists as a distinct outcome from a paper
being superseded by later work for exactly this reason. Recommend adding
`Retracted` as a third terminal currency outcome, restricted to
governor-reviewer authority (not any role that can mark `Superseded`),
requiring `evidence_refs` naming the error, and explicitly excluded as a
valid `supersedes` target for continuity claims (a later learning may
*reference* a retracted one as "learned from," but may not treat it as a
still-valid prior instance).

## Q4 — legacy migration: treat absence as permanent, not as implicit Open

The schema already states records predating `status` have "no status" and
that this is expected, not a validation gap. Recommend carrying that
forward explicitly rather than backfilling: a record with no disposition
stamp is **not** an implicit `Open` awaiting resolution — it is
permanently unstamped, and the validator must accept "no stamp" as legal
indefinitely for pre-migration records rather than nudging toward
retroactive stamps that would invent authority/evidence the original entry
never had. This is close to what Q4 as posed already assumes; stating it as
a firm answer (not left open) avoids a future contributor "helpfully"
back-filling `Open` stamps with synthetic authority references.

## Q5 — minimum stamp fields: agree with the no-free-form-field instinct; make it concrete

Proposed minimum, mirroring the discipline this project's own coordination
message schema already applies (structured front matter, prose only in a
linked body): `stamp_id`, `axis` (`currency` | `disposition` — `proposal`
removed per Q1), `from_state`, `to_state`, `transition` (closed enum),
`timestamp` (RFC3339 UTC), `actor_role`, `authority_ref` (the coordination
decision-record id proving gate authority for that transition — required
for `Accepted`/`Retracted`/`Implemented`, since those are governor-gated),
`evidence_refs` (array of coordination/MR/issue ids). No `reason`/`note`
string field on the stamp itself, full stop — any rationale is the linked
`authority_ref` or `evidence_refs` record's own content, never inline here.

## Not blocking, but should be in the eventual Gate 1 plan

- Per `gate-self-test-rule.md`, the eventual validator that rejects
  changed/deleted/reordered/duplicate/unauthorized/illegal-transition
  stamps is a gate and needs its own negative test at introduction — flag
  this now so it isn't discovered late.
- Cross-reference GitLab issue #117 explicitly in the eventual plan; this
  proposal is effectively that issue's answer for learning records
  specifically (findings 1 and the `ValidationOutcome` sync-gap shape both
  motivate the currency/disposition split above).

## Disposition

Proceed toward a bounded Gate 1 plan with these corrections folded in:
drop the proposal/routing state machine in favor of the existing
`lesson-proposal` coordination kind; add `Retracted`; specify the annex's
durable home as a same-file linked entry; treat legacy absence as
permanent, not implicit `Open`; and finalize the five-field, no-free-text
stamp shape above. Recommend requesting infosec-sentinel and
contribution-auditor advisory consultation on that plan per the standard
Gate 1 template, given the eventual validator touches every learning
record going forward.
