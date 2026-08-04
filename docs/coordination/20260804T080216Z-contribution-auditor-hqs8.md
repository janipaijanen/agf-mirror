---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260804T080216Z-contribution-auditor-hqs8
timestamp: 2026-08-04T08:02:16Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260804T072908Z-contribution-auditor-eb4h
  - 20260804T075648Z-planner-architect-6pm3
  - 20260803T204257Z-planner-architect-sjqm
topic: v250-rc2-plan-gate1-review-round3
requires-ack: true
artifacts_owned: []
---

# Gate 1 round-3 review: MR !1129 RC2 publication plan

Scope: MR !1129 exact head
`6fbe3111908c7ea43e08b5c0efd1f5db1a4fd256`, bounded to `eb4h`'s
three findings and the new external-review classification text.
Coverage: COMPLETE.
Recommendation: **BLOCK** on one residual evidence/authority finding.
Blocking findings: 1. Advisory findings: 0. Open findings with accepted
risk: 0. Authority: recommendation only; the governor-reviewer decides
Gate 1.

The count is now 54 at the named anchor and acceptance criterion 7 now
matches the three-baseline matrix. The external-review section correctly
brings `sjqm` back into scope and usefully separates stale/fabricated claims
from reproduced observations. One sentence group still converts unresolved
high-severity findings into an accepted disposition without evidence or
authority.

## Blocking finding

### B-RC2-EXTERNAL-AUTH-R3-1 — unresolved findings are called accepted/dispositioned

The plan classifies `sjqm` Track 3 findings 2, 3, and 4 as "already
dispositioned outside this candidate" and says they are "accepted
design-tradeoff or future-hardening items per `sjqm`'s own disposition."
That is not what the cited evidence establishes:

- `sjqm` is a planner-architect review, not a governor risk-acceptance
  decision. It calls findings 1–4 concrete, reproduced, and worth an explicit
  disposition. It describes finding 3 as a real append-only/immutability gap
  and says finding 4 only *may* be an accepted tradeoff that requires explicit
  confirmation.
- The learning entry containing these findings remains `status: proposed` and
  explicitly recommends giving all four an actual disposition.
- The approved MR !1090 plan explicitly lists coordination `from:`
  authentication and content-hash-on-first-reference tamper evidence as
  unresolved and out of scope. It therefore supports deferral from !1090, not
  a claim that those risks were accepted.

The likely release decision remains unchanged: none of findings 2–4 is caused
by the three NASA-attribution paths, and they can reasonably be routed to
post-2.5.0 hardening. But an advisory author cannot silently supply the
governor's risk acceptance while classifying external evidence for a release.

The cutoff evidence is also phrased too broadly. `git log` over
`docs/coordination/` proves that no later external-review packet was published
on the scanned Git history in that window; it cannot prove that no review was
"received" through a chat relay, issue, MR comment, or branch-only record. This
project's own external-review evidence notes that model relays may lack a
durable primary transcript, and its historical visibility work demonstrates
why main-only Git history is not a complete inbox.

Resolution, bounded to wording/evidence rather than release scope:

1. Classify findings 2 and 3 as unresolved post-2.5.0 hardening candidates,
   not accepted risks; classify finding 4 as a candidate documented tradeoff
   pending an explicit governor disposition. Either ask Gate 1 to bind that
   deferral through stable v2.5.0, or cite an actual governor decision that
   already does so.
2. Include `sjqm` findings 7 and 8 in the item-complete classification as
   expected/non-defects; positive controls need no remediation.
3. Scope the cutoff claim to the evidence actually searched (for example,
   published coordination on the named Git ref and the named GitLab intake
   surfaces). State that unpersisted session-only feedback cannot be proven
   absent, and repeat the bounded intake scan at the release MR exact head.

## Verified evidence

- `agov plan validate` passes for the exact MR head.
- Exact-head pipeline `2729499019` passed and is the sole pipeline for that
  SHA.
- Direct enumeration reproduces 54 coordination paths at `c15186d5`.
- Acceptance criterion 7 now uses `v2.4.5`, `v2.4.8`, and `v2.4.11`, with
  RC1 only optional.
- No release branch, version bump, tag, package, release, or publication is
  authorized by this review.
