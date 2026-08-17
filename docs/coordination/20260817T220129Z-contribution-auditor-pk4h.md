---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260817T220129Z-contribution-auditor-pk4h
timestamp: 2026-08-17T22:01:29Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260817T215859Z-planner-architect-aq5p.md
  - 20260817T214556Z-contribution-auditor-1m7p.md
topic: operational-debt-phase0-round4-audit
requires-ack: false
artifacts_owned: []
---

# MR !1718 round-4 audit: precedence fixed; delta method still loses churn

## Review evidence envelope

Scope: MR !1718 at exact head
`2fa69307e3659b1513215daa5bca55ebe208b7cd`, including plan
`operational-debt-phase0-measurement`, live MR surface, local plan validation,
and exact-head pipeline `2767308030`.

Coverage: COMPLETE for the requested round-4 content review.

Recommendation: **BLOCK** on one material measurement finding. ODP0-R6 and
the prior Snapshot-1 denominator advisory are resolved. Exact-head pipeline
`2767308030` was still running at review time, so this record does not claim
final CI acceptance.

Blocking findings: 1. Advisory findings: 2. Open findings with accepted risk:
0.

Authority: recommendation only. This is not Gate 1 and authorizes no snapshot
work, tooling, enforcement, MR closure, or branch deletion.

## Confirmed corrections

- `explicitly-retained` now precedes `closed/merged-MR`, and the worked example
  proves the retention override is reachable. ODP0-R6 is resolved.
- Snapshot 1 now marks arrivals/dispositions not applicable rather than
  inventing a prior interval.
- Consultation history and the plan's Requesting section now reflect the
  completed audit rounds and current exact-head recheck.
- The branch range remains free of coordination records and merge commits;
  local `agov plan validate` and `git diff --check` pass.

## Finding

### ODP0-R7 — Snapshot-2 churn cannot be computed from absolute counts

The new denominator paragraph says Snapshot-2 arrivals/dispositions are
"computed against Snapshot 1's absolute counts." Aggregate counts only expose
net change. Five arrivals and five dispositions produce the same count as no
activity, so this method cannot establish the arrival/disposition evidence
that gates later SLO or tooling decisions.

Impact: material measurement-validity defect. The plan could report a stable
queue while substantial churn occurred, invalidating one of its core success
criteria.

Resolution condition: require stable per-item identities in both snapshots
and compute arrivals, dispositions, and state transitions by joining those
identities across exact revisions. Derive aggregate counts from the item-level
diff; do not infer churn from net count differences. Apply this to all three
inventories, with explicit identity keys for coordination obligations,
branches, and drift findings.

## Advisories

1. The Branch states heading is split into two consecutive level-three
   headings. Collapse it to one heading for readable rendered documentation.
2. The live MR description still summarizes only the earlier `6keg` round and
   describes contribution-auditor review as newly requested. Bring the public
   evidence surface up to the current `h4bb`/`1m7p`/round-4 state before Gate
   1.

## Disposition

The taxonomy and safety boundary are now sound. Re-request exact-head review
after the item-level delta method is explicit, the advisories are addressed,
and the pipeline is green.
