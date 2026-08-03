---
kind: note
from: coder-tester
to: governor-reviewer
id: 20260802T213118Z-coder-tester-d7tm
timestamp: 2026-08-02T21:31:18Z
immutable: true
references-to:
  - 20260802T210836Z-coder-tester-rmds.md
topic: v250-wp2-cutoff-amendment-correction
requires-ack: true
artifacts_owned: []
---

# Correction to rmds: Option A's grandfathering direction was inverted

Relayed by the human governor from Codex's independent audit of `rmds`'s
(MR !968) B2 amendment reasoning (out-of-band review, not yet a
coordination-channel record itself). Codex is correct and I was wrong.
This note supersedes `rmds`'s Option A recommendation — it does not
retract the request for a governor decision, only the flawed argument
for one of the two options offered.

## The error

`rmds` claimed: "[Option A] is strictly earlier than any possible merge
date, so it grandfathers a superset of what 'merge date' would have
grandfathered."

That is backwards. Grandfathering excludes records with `timestamp <
cutoff`. For two cutoffs where `cutoff_A < cutoff_B`:
`{records < cutoff_A} ⊆ {records < cutoff_B}` — the **earlier** cutoff
produces the **smaller** grandfathered set, not the larger one. An
earlier cutoff brings *more* records into pending-tracking scope, not
fewer. I asserted the opposite direction.

## Independent verification, not just accepting the correction on say-so

Per `verifiable-claims-rule.md`, checked this myself rather than only
relaying Codex's finding. Scanning `docs/coordination/*.md` on current
`main` for non-`handoff`, non-`ack`, `requires-ack: true` records with
`timestamp` strictly between the two candidate cutoffs
(`2026-08-02T07:23:26Z` Gate 1 approval / `2026-08-02T20:09:55Z` current
shipped default):

- `20260802T193425Z-contribution-auditor-rtga.md` (the original B1-B4
  BLOCK review) falls in that window. Confirmed via
  `agov coord pending --cutoff 2026-08-02T07:23:26Z`: it does **not**
  currently appear as pending under that earlier cutoff, because
  `20260802T194806Z-governor-reviewer-gmk7.md` (MR !949's Gate 2
  approval, already on main) happens to name it in its own
  `references-to`, which mechanically resolves it under the direct-
  reference rule — even though `gmk7` is not substantively "about"
  `rtga`'s findings. That is a real but incidental resolution, not
  evidence the direction of my claim was safe.

This confirms the *structural* problem Codex raised even though today's
concrete pending count under Option A happens to be zero right now: the
outcome depends on which unrelated later records incidentally reference
an in-window record, which is fragile and not something a cutoff choice
should depend on. I no longer recommend Option A.

## Endorsing Option C (Codex's proposal): the amendment decision-record's own timestamp

Use the timestamp of the governor's amendment decision-record itself as
the cutoff. This is:

- **Governed** — it is a governor-reviewer decision-record by
  construction, not an implementation-time guess.
- **Immutable and precisely known** — fixed the moment the governor
  files the decision, before I implement anything against it.
- **Not circular** — unlike "this work package's own merge commit's
  date," the decision-record necessarily exists before the code that
  reads it is written, so there is no future-timestamp problem.
- **Close to shipment** — the gap between decision and merge should be
  small (implementation + test + push + Gate 2), keeping the newly-in-
  scope window narrow, unlike the ~13-hour Gate-1-to-now gap Option A
  would have opened.

## Status

Per the critical-path ordering relayed with this correction: not
implementing anything against either option. Waiting for the governor's
decision-record naming the exact chosen boundary (Option C, or another
option the governor selects) before any further B2/B-DOC-5 work.
