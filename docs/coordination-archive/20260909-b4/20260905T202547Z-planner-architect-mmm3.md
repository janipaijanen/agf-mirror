---
kind: note
from: planner-architect
to: all
id: 20260905T202547Z-planner-architect-mmm3
timestamp: 2026-09-05T20:25:47Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260905T202153Z-planner-architect-hr6j.md
  - 20260905T201822Z-planner-architect-rnhh.md
  - 20260905T200607Z-coder-tester-hqps.md
topic: session-run-list-canonical-queue-and-sequencing
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Canonical run-list and sequencing

Mirroring the governor's chat-defined work queue on-channel so it survives
compaction and is visible to Codex and any cold-starting session (learning
`20260905T194427Z-wq4k`, gap 3). This supersedes the informal list in
`hr6j`'s "trigger" paragraph as the queue of record; `hr6j` remains the
detail for item 3.

## The queue

1. **Deliver v2.9.0.** Release-critical runner routing and the ROI repair, in
   order: MR !2362 (base `default: tags:`) → MR !2373 (revised split
   ordinary/`dind` routing — needs an independent re-review and Gate 1) → MR
   !2392 rebased onto the new `main` → MR !2383 (ROI implementation) rebased
   and re-run → v2.9.0 pushed and delivered. Everything below waits on this.

2. **Coordination-protocol slices.** Under one shared Gate 1 design, four
   separable implementations: execution ownership (issue #121), explicit
   supersession (#122), binding-instruction publication, plan/workstream
   collision detection (#123). Sequencing and shapes per `hqps`.

3. **Rule non-conflict + `status:` state machine.** Claude and Codex jointly
   design how `agf/rules/` stays non-conflicting; argue hard against or
   improve the governor's proposed rule `status:` lifecycle. Detail and
   Claude's opening position in `hr6j`.

4. **Radar / sources reconsideration, with Codex.** Run a joint pass over
   `docs/radar.md` and the classified entries in `docs/sources.md`. For each
   active radar item and each `action: watch` source, decide whether its
   trigger-to-act condition is now met and, if so, whether it should be
   promoted to: (a) a bounded **experiment** (the OTGR experiment charter
   `20260904T104400Z-governor-reviewer-q7mv` is the template), or (b) a
   **release item** on a future version. Items not ready get a fresh
   `review-by` and a one-line note; items clearly dead get retired with a
   reason. Output routes as radar-review evidence; each promotion needs its
   own Gate 1. Feeds item 5.

5. **Next-releases roadmap.** After v2.9.0, continue governed release
   planning for the subsequent versions — reconciling the item 4 promotions,
   the existing governed-roadmap work order
   (`v290-governed-roadmap-work-order-gate1-approved`), open issues, and the
   backlog. Ongoing.

## Boundaries

Items 2–5 are design and planning only until each has its own Gate 1
approval. This note authorizes nothing; it records the queue and its order.
Items 4 and 5 are lower priority than 1–3 and than any active
release-critical or governor-dispatched work.
