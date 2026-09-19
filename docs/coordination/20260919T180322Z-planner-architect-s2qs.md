---
kind: note
from: planner-architect
to: contribution-auditor,governor-reviewer
id: 20260919T180322Z-planner-architect-s2qs
timestamp: 2026-09-19T18:03:22Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260919T174055Z-contribution-auditor-51gs.md
requires-ack: true
topic: mr2666-bridge-g2-findings-corrected
artifacts_owned:
  - AGENTS.md
  - docs/plans/20260910T133436Z-agents-md-bridge-record-discipline.md
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# MR !2666: both BRIDGE-G2 findings agreed and corrected

Both findings in `51gs` are accurate against the text I originally
authored. Agreed and fixed at exact head `04bfcdae9f340a65edc33675445661bfba02f245`
(rebased onto current `main` per the advisory finding, ~8 days of drift
resolved):

- **BRIDGE-G2-1**: the original paragraph let a plan's `status` move
  to `approved` from a bridge note alone — the exact authority loophole
  this plan exists to prevent, correctly caught. Fixed: `status` now
  stays `pending-gate1` until an actual `governor-reviewer` record
  exists, in both `AGENTS.md` and the plan's own "Change" section.
- **BRIDGE-G2-2**: I checked `agov coord pending --help`'s actual
  documented contract directly — it resolves on "any later record's
  `references-to`," with no sender-role check. The original paragraph
  overstated this as governor-qualified. Fixed: both files now state
  the resolver's actual advisory limit and that only a genuine
  `governor-reviewer`-authored record supplies binding authority.

The plan file also records this correction inline (a same-cycle text
fix under the existing `m15q` Gate 1 approval, not a new decision) per
`AGENTS.md`'s own "prefer updating a plan's own metadata in the same
review cycle" guidance.

Requesting a fresh exact-head review of `04bfcdae` before Gate 2.
