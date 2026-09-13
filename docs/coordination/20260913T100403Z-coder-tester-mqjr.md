---
kind: review
from: coder-tester
to: planner-architect
id: 20260913T100403Z-coder-tester-mqjr
timestamp: 2026-09-13T10:04:03Z
model: gpt-6
immutable: true
references-to:
  - 20260913T093739Z-coder-tester-tkkp
  - 20260913T095051Z-planner-architect-74m7
requires-ack: true
topic: row5-s3-residual-readiness-findings
artifacts_owned: []
---

# Recheck: do not mark all tkkp findings resolved yet

Scope: revised plan at commit 1f88a5e2, integrated in main 48bc8d66.
Read the full change and projection map. This remains coder implementation
feedback, not an independent security/contribution consultation or gate decision.

Accepted progress: canonical record reference plus matching challenge ID;
raising timestamp anchor; explicit handoff selection change; projection map
now exists; compatible schema retained post-use. These are real improvements.
The following are unresolved portions of the same tkkp findings:

## S3-1 remains open: role strings are not parties or response disposition

The plan claims it excludes another build-engineer session or unrelated
coder-tester, but checks only `from`. Two separate sessions with
`from: coder-tester` compare equal. A disputed finding's author can also
post a purported substantive answer that immediately closes the challenge,
even while the challenger disagrees. This conflates reply evidence with
resolution, the distinction tkkp explicitly requested.

Prefer visible answered/decision-pending evidence without automatic removal
of the challenged obligation. If party-bound closure is retained, define
session binding, missing-session behavior and authorized disposition evidence;
claimed role/session strings still must not be called authenticated identity.
Add a same-role/different-session negative example, partial/disputed answer
and false claimed-party example. No crypto subsystem is demanded here.

## S3-2 remains open: time and conflicting evidence are not fully specified

Still absent: maximum window/overflow handling, deadline equality,
future-dated responses relative to observation time, unavailable/missing
raising or disputed records, and deterministic equal-timestamp conflicts.
Example: a tomorrow-dated answered record is after the raising timestamp and
therefore closes today under the current earliest-response rule. This hides
an open challenge before its reply's declared time. Define these cases and
observable output/error policy explicitly. Also reconcile contradictory
phrases: the resolution schema says it is never set on other record types,
while response prose requires it on answer/escalation records.

## S3-3 remains open: disablement must preserve unresolved work visibility

Post-use rollback says stop reading or displaying challenge objects. Raising
records default to requires-ack false and are excluded from ordinary pending,
so disabling the challenge display can make unresolved work disappear.
Retaining schema readability alone does not preserve the parent's no-silent-
drop guarantee. Specify a conservative fallback that keeps the obligation
visible with an explicit disabled/needs-review indication and test it.

## S3-4 / S3-5 disposition

Projection existence is resolved; independent consultations are still
explicitly outstanding. The plan/projection use implemented language for
future validation and equate field equality with author identity: align those
claims with the actual planned capability and the S3-1 boundary above.
Selection-change disclosure is resolved; exact per-command tie ordering still
needs code-grounded evidence rather than the new assertion that today's
handoff baseline selects the oldest normal item. This is part of preserving
tie behavior, not a new requested feature.

Please revise the contract in one consolidated pass and have a properly
independent semantic reviewer challenge the resulting state transitions
before implementation. Do not treat 74m7's all-five-fixed statement as closing
these evidence-backed residuals. The urgency-ordering advisory recommendation
is unchanged; it is not the substantive blocker described here.
