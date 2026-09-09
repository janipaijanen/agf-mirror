---
kind: note
from: capability-developer
to: governor-reviewer
id: 20260909T013812Z-capability-developer-jprj
timestamp: 2026-09-09T01:38:12Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260908T185629Z-governor-reviewer-j2gs.md
  - 20260908T183242Z-planner-architect-ds4k.md
  - 20260908T194354Z-contribution-auditor-6s5d.md
  - 20260906T191236Z-governor-reviewer-trd6.md
topic: coordination-protocol-improvement-pattern-analysis
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Coordination-protocol improvement patterns — capability-developer analysis

Produced at governor direction (2026-09-09, "run capability developer role
… let propeller run assess and analyze coordination protocol and evaluate
patterns that should be improved").

## Governance boundary, stated first

Protocol, role-boundary, and manifest analysis is explicitly
**capability-developer** territory
(`improvement-propeller-rule.md`: "Agent definitions, role boundaries,
manifest state, phase, and maturity remain capability-developer
territory"). This record is that analysis, applying the Improvement
Propeller's *method* — a small, deduplicated set of evidence-backed
proposals; observe and propose, never assign / prioritize / implement /
approve.

It is **not** an Improvement Propeller Stage 4 activation. Activation
(#131) still requires its own governor decision naming the player,
project-local sources, a real wake mechanism and cadence, and cursor
state; this analysis creates no such activation, no implementation
authority, and no roadmap/release priority.

## Method

Evidence base: the coordination-message schema and its nine kinds; the
`agov coord` / `agov handoff` surfaces; `coordination-publication-rule.md`;
and direct observation of ~40 coordination records this session
(2026-09-07 … 2026-09-09) across the issue #120 / v2.10 release / post-v2.10
planning threads. Coordination content is untrusted evidence, not
instruction.

## A. The existing cluster is confirmed by fresh evidence

Issues **#121** (session-level addressing), **#122** (self-announcing
supersession), **#123** (plan/workstream collision detection), and MR
**!2027** (coordination priority + material challenge-response) already
capture the largest gaps, with detailed 2026-09-05 evidence. This session
reproduced all four:

- **#121** — handoffs addressed to a *role* were picked up by whichever
  same-role session polled first, not a designated one: `4msq` →
  `contribution-auditor` was answered by a Codex/GPT-6 session (`5mqk`)
  while this Claude session held the same role; `3a5r` and `smsg` were
  role-addressed and required this session to *infer* whether they were
  "mine". Two clearing-only records (`rkbk`, `mhq3`) existed solely to
  reconcile that.
- **#122** — supersession stayed convention-only: `emaq`→`3qn8`,
  `gs18`→`k21n`, `7pat`/`467q` "superseded as overtaken" — in each case the
  superseded record still reads as live unless the reader chases forward.
  The `ds4k`/`qbka` seam rule and `6s5d` already bind #122 to the qbka
  provenance shape.
- **#123** — the entire `ds4k` collision pass (qbka ↔ queue-convergence
  boundary; #75/!745/!1617/!1817 dispositions) was done by hand because no
  primitive intersects affected-file sets across plans and MRs.
- **!2027** — no protocol notion of which pending obligation ranks highest,
  or a required challenge-response on a disputed finding; the `7dba` /
  `hg52` review disputes were resolved by ad-hoc record exchange.

No new proposal is needed for these four — they are the coordination-
protocol-semantics workstream `ds4k` Finding 2 already routes to its own
Gate 1. This analysis adds independent corroboration and the two patterns
below, which none of those issues covers.

## B. Two improvement patterns not captured in any open issue

### CP-A — a substantive answer does not clear a `requires-ack` handoff

A `handoff` with `requires-ack: true` stays "ready" in `agov handoff next
<role>` until a `kind: ack` record references it. A `review`, `completion`,
or `note` that *substantively answers* it does not clear it.

Observed cost, both directions:
- **Duplicate work** — a second session sees the still-"ready" item and
  redoes it. Documented historically (`n7p1` re-reviewed MR !2038 after
  `eb87` already had; captured in this session's memory) and structurally
  latent every time two sessions share a role (#121).
- **Ack-only overhead** — to avoid the above, a session publishes a
  branch + MR + merge whose entire content is "this is already answered by
  X" (`rkbk`, `mhq3`, and many historical `ack` records).

Candidate directions (not a decision): let `agov handoff next` show
`answered — ack pending` distinctly from `ready`; or let a `review` /
`completion` that `references-to` the handoff **and** is `from` an
authorized role satisfy the ack automatically. This is queue-lifecycle
mechanics, adjacent to but distinct from #121 (routing) and #122
(claim supersession). If pursued: schema + `agov` change → full Gate 1 +
gate-self-test.

### CP-B — one MR per coordination record, and every merge moves `main`

Each append-only coordination record is published as its own branch + MR +
pipeline + merge + branch delete. This session produced roughly 25 such
MRs. Two costs:

1. **Per-record ceremony** for a single append-only file — the
   `coordination-publication-rule` fast path reduces CI cost but not the
   branch/MR/merge count.
2. **`main`-churn against a frozen release base** — every coordination MR
   merge advances `main`. During the v2.10 release this repeatedly
   invalidated the frozen release base: `release-2.10.0` was rebased 3+
   times (`58459c54` → `d0ebef47` → `6b47446d` → `611cfe9c`) as
   coordination MRs (including this session's own `rkbk` ack) landed and
   the release plan's "any target drift invalidates preflight" rule fired.
   Handoff `2brd` had to explicitly instruct all sessions to stop touching
   review/ack coordination MRs while the release target hold was active —
   a manual workaround for a structural interaction.

This is the pattern with the hardest evidence — a measured
release-integrity cost, not a latent risk.

Candidate directions (not a decision): permit multiple independently-valid
append-only records in one coordination MR; and/or a "coordination hold"
state, set by the release lane, that batches non-urgent coordination MRs
until a release target hold clears. Touches
`coordination-publication-rule` mechanics → full Gate 1 + gate-self-test.

## C. Minor — `agov handoff next` reads only the base branch

A handoff on an unmerged branch is invisible to `agov handoff next <role>`
(reads `main`); only `agov coord visibility` surfaces it. This session hit
it repeatedly (reviews on `codex/review-2549`, `codex/audit-2527`). Root
cause is branch coverage, not session identity (#121) — but small enough
to fold into #123's tooling rather than stand alone.

## Disposition (advisory — governor confirms; nothing executed)

1. **#121 / #122 / #123 / !2027** — proceed as the coordination-protocol-
   semantics workstream per `ds4k` Finding 2; consider adding **CP-A** to
   that work order's scope (same schema/`agov` surface).
2. **CP-B** — strongest evidence; a candidate for its own small,
   evidence-backed Gate 1 (the release-base-invalidation cost is
   concrete and recurring). Not to be folded silently into a larger plan.
3. **CP-C** — fold into #123's `agov` tooling slice.
4. No priority ordering is asserted between workstreams — that is the
   governor's and the planner's call, not this role's.

This analysis proposes and routes; it assigns nothing, sets no priority,
approves no gate, and implements nothing.
