---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260823T204336Z-contribution-auditor-tk51
timestamp: 2026-08-23T20:43:36Z
model: codex
immutable: true
references-to: []
topic: replicated-live-source-batch29-planning
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Draft a narrow Gate 1 plan for Batch 29 classification of the Replicated Live knowledge and worktree source family.
    - Keep the planned implementation limited to docs/sources.md; do not add a radar item or implement external tooling.
    - Bind implementation evidence to the exact upstream revisions and caveats recorded below.
    - Submit the validated plan to contribution-auditor consultation and governor-reviewer for a binding Gate 1 decision.
  must_read:
    - docs/plans/20260808T183550Z-beagle-crdt-source-concurrency.md
    - docs/sources.md Batch 21
    - docs/radar.md
    - https://replicated.live/blog/wiki.html
    - https://replicated.live/blog/worktree.html
    - https://replicated.live/blog/follow-up.html
    - https://github.com/GoogleCloudPlatform/open-knowledge-format/tree/ad30107c31c06aec8a7d5636e0d1058118604e6f
    - https://github.com/gritzko/bee/tree/184fddf4ec318469d70406ff8b51b58dcb5c0ba6
    - https://github.com/gritzko/beagle/tree/c27ac6ff71aea78cdfc2864d4308e8f994cca245
  do_not_touch:
    - docs/sources.md before Gate 1 approval
    - docs/radar.md
    - agf/rules/
    - tooling/
    - .claude/settings.local.json
---

# Handoff: plan Replicated Live / OKF / Beagle source Batch 29

Governor-reviewer instructed this work to proceed after reviewing the proposed
classification. That instruction authorizes preparation of the exact plan; it
is not a substitute for a binding Gate 1 decision on a subsequently published
plan.

## Recommended source classification

Prepare a source-only Batch 29 covering this bounded source family:

1. Victor Grishchenko, “Wiki Is Not Enough,” published 2026-08-23:
   practitioner technical design/opinion; relation `prior-art, complement`;
   action `cite (with caveat), watch`.
2. Grishchenko, “Tree-of-life and evolution of code,” published 2026-07-20:
   practitioner workflow/design evidence; relation `prior-art, complement`;
   action `cite (with caveat), watch`.
3. Canonical Open Knowledge Format v0.2 specification, pinned at
   `ad30107c31c06aec8a7d5636e0d1058118604e6f`: primary specification,
   Apache-2.0; relation `prior-art, complement`; action `cite, watch`.
4. Bee, pinned at `184fddf4ec318469d70406ff8b51b58dcb5c0ba6`, and Beagle,
   pinned at `c27ac6ff71aea78cdfc2864d4308e8f994cca245`: primary
   implementation evidence. GitHub reports no recognized repository license
   for either project, so record design prior art only and infer no reuse
   permission.
5. “New model economics” (`follow-up.html`), published 2026-08-15: contextual
   commentary on Cursor's swarm experiment. Mention it within the source-family
   assessment rather than necessarily promoting it to an independent core
   entry. If its claims about Cursor are retained, cite Cursor's primary
   article separately and distinguish quotation from Grishchenko's inference.

## Evidence boundaries the plan must preserve

- “Wiki Is Not Enough” responds largely to OKF v0.1. Canonical OKF v0.2 now
  makes provenance, trust, freshness, lifecycle, and attestation first-class,
  partially answering the article's knowledge-rot critique.
- The pinned Bee revision implements edit-resilient references, `cite`, and
  backlink lookup through `lindex`; it does not expose the article's named
  `cited` command. Treat the claim that this architecture “solve[s] both
  retrieval and coherence” as an author design/effectiveness claim, not a
  reproduced result.
- The pinned Beagle revision confirms `be work`, many parallel worktrees, and
  upstream-fork organization. The article's “x100 improvement” and operational
  outcomes have no disclosed comparative benchmark and remain attributed
  self-report.
- The follow-up's evolutionary selection argument supplies no formal or
  empirical proof that candidate selection yields correct or safe software.
  It does not replace acceptance criteria, independent review, or binding human
  gates.
- Bidirectional citation and hierarchical candidate navigation are useful
  prior art for possible future `agov` derived views. They do not by themselves
  demonstrate an AGF defect or authorize implementation.

## Radar disposition

Do not add a radar item. The material overlaps Batch 21's Beagle assessment,
the existing external-context-graph and cross-harness-continuity questions,
and AGF's current worktree, coordination-graph, independent-review, and Gate 2
surfaces. A new question should be reconsidered only after a concrete observed
failure to find downstream references or to track/compare many parallel
candidate trees.

The plan should expressly prohibit changes to rules, tooling, worktree
semantics, role authority, gate authority, or external integrations.
