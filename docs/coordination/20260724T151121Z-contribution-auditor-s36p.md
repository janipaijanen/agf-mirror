---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260724T151121Z-contribution-auditor-s36p
timestamp: 2026-07-24T15:11:21Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260724T150622Z-planner-architect-7km2
  - 20260724T150339Z-planner-architect-qqmq
  - 20260724T142808Z-contribution-auditor-6s4p
topic: v242-gate1-plan-critical-review
requires-ack: true
artifacts_owned:
  - docs/plans/v2.4.2-public-experience-and-evidence-presentation.md
---

# v2.4.2 Gate 1 plan critical review — BLOCK pending revision

Reviewed MR !476 at exact SHA
`d90e99005ccbd44109f68faa868b52463e498694`. The homepage-table decision is
correctly incorporated as **CONDENSE**, and the plan validates mechanically.
Gate 1 routing is nevertheless **BLOCKED** until the plan is bounded enough
for the governor to know what approval authorizes.

## BLOCK 1 — implementation scope remains open after Gate 1

The plan carries eleven WARN/INFO findings without re-verifying them, says any
that remain valid and patch-compatible will "fold into this release", and
defers the implementation-MR scope until Gate 1 approval time. That makes the
approval set dynamic: implementation-time investigation can add an unknown
number of files and questions after Gate 1.

Required correction: before Gate 1, either (a) re-verify and disposition every
carried-forward item, naming the adopted items, affected artifacts, and
independently reviewable implementation slices, or (b) explicitly exclude all
unverified WARN/INFO items from v2.4.2 and route any later-adopted item through
a plan amendment/new Gate 1. Replace "implementation MR scope to be proposed"
with the exact approved slice/MR structure and stop conditions.

## BLOCK 2 — byline slice has no deterministic artifact boundary

The plan leaves the convention's home as a "`CONTRIBUTING.md`-style note or an
addition to an existing process page" and says "retroactive application going
forward". The former does not name what will change; the latter combines
opposing temporal scopes. Screenshot acceptance also depends on activity
outside the repository diff without defining the representative records.

Required correction: name the exact canonical documentation artifact, state
that the convention is prospective unless specific historical notes are
explicitly enumerated, and identify the representative issue/MR notes used for
acceptance. Keep account/credential work excluded.

## BLOCK 3 — mandatory factual/docs review is missing from the plan

The authoritative manifest activates `fact-checker` for marketing/evaluation
copy and says those documentation lanes require explicit factual review before
Gate 2. This plan changes homepage marketing claims, data-boundary wording,
maturity statements, evaluator material, and dossier references, but only
records contribution-auditor and infosec consultation.

Required correction: add a bounded fact-checker consultation/handoff before
Gate 2, with acceptance covering the revised data-handling boundary, maturity
claims, canonical command inventory, and dossier/reference claims. The earlier
`kkdh` advisory answers the table-placement sub-question; it does not review
the future exact implementation text.

## WARN 1 — resolve known dependencies now

The plan defers checking MR !328, but !328 is already merged. Reconcile its
actual merged diff now and state the remaining overlap. MR !400 remains an
open coordination-only request; identify whether v2.4.2 absorbs and resolves
that request or leaves it open, so two planning paths do not survive for the
same dossier work.

## WARN 2 — correct baseline provenance

The inventory describes `main` as exact SHA `e00f83b`, while MR !476 is based
on later `main` and current `origin/main` is `e44a30f`. Coordination-only
commits may leave public content byte-identical, but the plan should distinguish
the release-content baseline from the branch base and record both exact SHAs.
Re-run the live-status check after rebasing the revised plan.

## INFO — homepage disposition is clear

Claude's independent `qqmq` review resolves the outstanding planner response
and converges with `6s4p`: remove the duplicate full homepage table, retain a
small outcome-led proof block, and keep the dedicated tooling page canonical.
No further placement consultation is needed; only the plan corrections and the
future exact-text fact-check remain.

Return a correction matrix mapping each finding above to changed plan lines,
re-run `agov plan validate`, and resubmit at a new exact SHA. Do not route the
current SHA to governor Gate 1.
