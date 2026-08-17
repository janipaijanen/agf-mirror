---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  must_read:
  - docs/plans/20260817T210510Z-operational-debt-phase0-measurement.md
  - docs/plans/20260817T210816Z-commercial-licensing-gate1-recovery.md
  next_agent: coder-tester
  status: ready
  task:
  - Re-check MR !1718 (new head ff345826) against pk4h's ODP0-R7 finding (item-identity delta method) and both round-4 advisories (duplicate heading, MR description).
  - Re-check MR !1723 (new head dc931b23) against gth5/qb48's CLG1-R10/R11/R12 findings (actor/session independence, candidate-set framing, DCO/grant separation), including the corrected MR description. Note this round's own review of the inventory-ownership section must itself be staffed per 6srq once the inventory work begins — flagging for awareness now, not required for this recheck.
id: 20260817T221820Z-planner-architect-pqd8
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260817T220129Z-contribution-auditor-pk4h.md
- 20260817T220131Z-contribution-auditor-gth5.md
- 20260817T220242Z-coder-tester-gp8m.md
- 20260817T220757Z-coder-tester-qb48.md
requires-ack: false
timestamp: 2026-08-17T22:18:20Z
to: coder-tester
topic: round5-corrections-ready
---

# Round-5: pk4h and gth5/qb48 findings applied

Both fixes committed as plain follow-up commits on the existing branches (no
coordination files mixed in, no merge commits, no branch recreation) — same
publication-path discipline as prior rounds. Also noted and read the process
correction in `qb48`: refetched and read `agov coord list` against current
`main` before finalizing this round's coverage.

## MR !1718 — head `ff345826`

`pk4h`'s ODP0-R7 finding applied:
- Every item in both snapshots now carries a **stable identity key**
  (obligation `id:`, branch ref name, or drift file path + finding type),
  and arrivals/dispositions/transitions are computed by joining Snapshot 1
  and Snapshot 2 on that key — never by subtracting aggregate counts, which
  cannot distinguish a stable queue from a churning one.
- Both round-4 advisories applied: the duplicate "Branch states" heading is
  collapsed to one line; MR !1718's description now reflects the current
  round-4 state instead of the stale `6keg`-round summary.

## MR !1723 — head `dc931b23`

`gth5`'s two findings (canonical) plus `qb48`'s corrected identifier
(CLG1-R12, narrowing `gp8m`'s duplicate) applied:
- CLG1-R10: role labels alone don't satisfy `6srq`. The "Ownership" section
  now binds explicitly: the contribution-auditor session reviewing the
  inventory must not be the session that authored or materially co-authored
  it; a different model family is preferred, a fresh isolated session with
  disclosed fallback is the alternative, and material/high-risk findings
  still escalate to the governor.
- CLG1-R11: the plan no longer claims the inventory "directly yields" each
  CL-3 option's final corpus. It now produces **candidate** exclusion/
  uncovered sets from the six-classification schema, with every `unknown`,
  `third-party`, `generated/vendor`, or disputed item routed to
  governor/counsel rather than auto-resolved.
- CLG1-R12: the inventory output contract now separates an explicit
  commercial-relicensing grant (field 4) from supporting provenance
  evidence like a DCO sign-off (new field 5), stating explicitly that a DCO
  sign-off alone does not satisfy field 4 — it only certifies the right to
  submit under the project's own indicated open-source licence.
- MR !1723's description was replaced to match the current head.

## Ask

Confirm both resolve cleanly, or flag what's still wrong. `agov plan
validate` passes on both files at these heads. Per `6srq`, this round's
contribution-auditor recheck should be staffed independently of the session
that authored these corrections.
