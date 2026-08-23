---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260823T204154Z-planner-architect-tsqg
timestamp: 2026-08-23T20:41:54Z
immutable: true
model: claude-sonnet-5
references-to: []
topic: learning-record-backlog-batch-request-review
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - >-
      Independent review of four freshly-filed Gate 1 plans un-stalling
      parked adopter learning-record issues that had no plan since
      filing. Each MR is self-contained; review in any order.
  must_read:
    - docs/plans/20260823T203534Z-upgrade-overlay-provenance-protection.md
    - docs/plans/20260823T203540Z-adoption-version-drift-check.md
    - docs/plans/20260823T203546Z-dist-path-ignore-protection.md
    - docs/plans/20260823T203552Z-review-topology-declaration.md
  do_not_touch: []
---

# Four Gate 1 plans un-stalling parked adopter learning issues — review requested

Six learning-labelled issues were found sitting stalled: #82, #91, #96,
#111, #112, #113 — only #114 (a seventh, since closed) had actually
been carried through plan → Gate 1 → implementation. Assessed each
directly rather than left parked; disposition:

- **MR !1855** — `docs/plans/20260823T203534Z-upgrade-overlay-
  provenance-protection.md`. Consolidates issues #96 (night-heron) and
  #111 (harbor-workshop): a new `agov upgrade classify` subcommand
  doing three-way tree comparison (previous pristine release / new
  pristine release / local), and an upgrade-skill rewrite so Step 3
  never deletes adopter content without an explicit confirmation gate.
  Largest and highest-severity of the batch — #111 reports a 94-line
  adopter-authored rule silently deleted and invisible for six weeks.
- **MR !1856** — `docs/plans/20260823T203540Z-adoption-version-drift-
  check.md`. Implements #112: blocking validator check that
  `adoption.framework_version` matches `agf/VERSION`.
- **MR !1857** — `docs/plans/20260823T203546Z-dist-path-ignore-
  protection.md`. Implements #113: documents and mechanically detects
  adopter `.gitignore` rules that silently exclude `agf/dist/` (release
  provenance) from version control.
- **MR !1858** — `docs/plans/20260823T203552Z-review-topology-
  declaration.md`. Implements #82's actionable portion: require a
  review-topology declaration in the AI Evaluation Dossier and External
  Review issue template, so a sequential-critique AI review round is
  never mistaken for independent corroboration. #82's own 2026-07-28
  routing to #45/#47/#49/#77 verified to have produced no movement in
  nearly a month (all four also parked) — this plan implements the
  narrow ask directly rather than waiting further.

**#91** is handled separately: too broad for a Gate 1 plan as currently
scoped (it is explicitly a discovery-phase request), so it received a
triage comment narrowing toward its one concretely-plannable piece
(an advisory pre-handoff coverage check) rather than a plan filed
today — see the issue for the full reasoning. Not part of this
handoff's review scope.

All four plans validated (`agov plan validate` OK; `agov validate` full
umbrella — no new findings beyond pre-existing, unrelated warnings) and
have MR descriptions conforming to `merge-request-description-rule.md`.
Requesting independent review before Gate 1 on each.
