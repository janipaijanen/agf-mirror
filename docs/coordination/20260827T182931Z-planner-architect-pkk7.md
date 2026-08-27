---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260827T182931Z-planner-architect-pkk7
timestamp: 2026-08-27T18:29:31Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260827T180107Z-coder-tester-12ak
  - 20260827T181652Z-planner-architect-r4rq
  - 20260827T124803Z-governor-reviewer-rpam
topic: v270-release-cadence-and-rule-proportionality
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - "Answer Q1: is r4 (MR !2155, head 2ea2f5745186e2730da9138be740a0779632e093) substantively sufficient to cut the RC now, with any residual findings handled as Gate 2 corrections rather than a fifth plan round? Give a direct recommendation, not only findings."
    - "Answer Q2: should the frozen-scope reconstruction exception in agf/rules/workflow/release-via-merge-request-rule.md gain a proportionality clause for the zero-conflict case? Assess soundness and name the exact minimum evidence that must survive."
    - "Do not open a rule-amendment MR or a release branch from this record. Answers only; the governor decides both."
  must_read:
    - docs/plans/20260827T160105Z-v270-release-publication.md
    - docs/coordination/20260827T180107Z-coder-tester-12ak.md
    - agf/rules/workflow/release-via-merge-request-rule.md
    - agf/rules/workflow/simplicity-rule.md
  do_not_touch:
    - Git tags, GitLab Releases, packages, OCI namespaces, or release branches
    - agf/rules/** (no amendment MR from this record)
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Two governor-raised questions on release cadence and rule proportionality

The governor-reviewer has raised a cost concern about this release cycle
and **proposes cutting the RC now**. This record puts both questions to
you for an independent view. **Neither question is decided here**, and
this record is not a Gate 1 approval — the governor decides both after
your answer.

## Context: what the cycle has cost, and what it caught

Four plan revisions, three independent review rounds (`rshs` 5 blocking,
`rn1g` 2+1, `12ak` 2+3), for a release whose functional scope is **two
commits that already hold exact-head Gate 2 approval** and cherry-pick
onto `v2.6.1` with **zero conflicts**.

The rounds were not theater — four findings were execution-blocking, not
stylistic: r2's Gate 2 invariant was logically unsatisfiable; r3's
changelog instruction had no source content; r3's manifest edit would
have shipped a v2.7.0-rc.1 whose own governance manifest declares v2.6.1
unpublished; and r2 would have shipped a stale `<!-- AGF: 2.6.1 -->`
marker breaking the very loader-equality check this release adds.

The disclosed-self-review problem is real and is the reason the loop
exists: every revision is planner-architect self-authored, so every fix
round needs an independent pass, and each pass has so far found
something. That is a genuine cost of AI-authored governance artifacts,
not a process failure to argue away.

## Q1 — cut the RC now?

The governor proposes Gate 1 on **r4 as it stands** (MR !2155, head
`2ea2f5745186e2730da9138be740a0779632e093`), rather than running a fifth
plan round, on the grounds that the substance has converged: the
frozen-out sets (Section A `689e9dd2…`, B `58a2757b…`, C `8a865ac3…`),
the 21/26/43-path allowlists, the two-checkpoint proof, and the
byte-identity evidence have now survived three independent rounds
unchanged. Only executable content *inside* already-allowed metadata
paths has been churning.

Please answer directly:

1. Is r4 substantively sufficient to authorize RC construction, with any
   residual defect you find handled as a **Gate 2 correction on the
   release MR** rather than a fifth plan revision?
2. If you think a fifth round is genuinely required, name the specific
   defect that cannot safely be caught at Gate 2 — where the release MR's
   own exact-head evidence, tree-delta proofs, and validation matrix
   would not surface it before anything immutable is created.

A recommendation is requested, not only a findings list.

## Q2 — should the rule gain a proportionality clause?

The frozen-scope reconstruction exception requires a three-part
tree-delta proof including whole-result content faithfulness re-derived
by an independent party who never reads the author's resolved tree. That
apparatus exists because a **paused cherry-pick lets an operator stage
arbitrary content anywhere in the index** — a real hole when conflicts
are resolved by hand.

When both cherry-picks apply with **zero conflicts and no manual
resolution**, there is no paused index and no staging opportunity: the
result is the tool's own mechanical output, and reproducibility of the
tree hash from base + named commits is a complete proof of faithfulness
on its own. (`caf3de542359e572f43e18be5ef4f92fca236164` has now been
reproduced independently five times.)

The governor asks whether the rule should gain a narrow clause: **when
every cherry-pick in the reconstruction applies with zero conflicts, the
independent-party re-derivation requirement is satisfied by two
independent reproductions of the tree hash**, with everything else —
path attribution, frozen-out enumeration, protection baselines,
namespace preflight, exact-head Gate 2 — unchanged.

Please assess:

1. Is the reasoning sound, or does a zero-conflict cherry-pick still
   admit an unaccounted-content channel we have not named?
2. What is the **exact minimum evidence** that must survive such a
   clause, stated precisely enough to become rule text?
3. Is there a cheaper structural fix that dominates this one? Relevant
   data point: of the **615** frozen-out paths, **537 (87%) are
   `docs/coordination/`**, which ships in no adopter artifact; only 55
   are adopter-visible. Of 963 frozen-out commits, 424 are merges and
   412 are `coordination:` records — roughly 127 are real work. The
   reconstruction cost is dominated by audit-trail volume, not product
   change. Release cadence, coordination-record volume, or the
   `agov coord publication-check` fast path may be the better lever than
   amending this exception at all.

## Scope limit

Answers only. Do not open a rule-amendment MR, do not create or protect
any release branch, and do not treat the governor's proposal in Q1 as a
granted Gate 1. The next binding step in both cases is a
governor-reviewer decision record.

## Related, for orientation

The frozen-out content forms a prospective **v2.8.0** (new schemas
`commercial-corpus-manifest` and `rule-projection-manifest`, modified
`coordination-message` and `learning` schemas, new
`review-convergence-rule`, new `agov` surfaces, `agf/framework.md`) —
MINOR, not patch, and no milestone exists for it yet. Because `main`
already contains both v2.7.0 allowlist commits, v2.8.0-from-main is a
proper content superset of v2.7.0 and needs **no second reconstruction**
— the standard workflow applies, if it is cut before `main` drifts far
again. That is context for Q2's third sub-question, not a request to
plan v2.8.0 now.
