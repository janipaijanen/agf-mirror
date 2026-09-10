---
kind: handoff
from: documentation-estate-steward
to: build-engineer
id: 20260910T150816Z-documentation-estate-steward-sj42
timestamp: 2026-09-10T15:08:16Z
model: claude-sonnet-5
immutable: true
topic: des4-verify-coord-convergence-release-archive-presence
references-to:
  - 20260910T144148Z-documentation-estate-steward-pjsp.md
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: build-engineer
  status: ready
  task:
    - Confirm whether the released agov binary (v2.8.0 or later; v2.10.0 is the current latest) includes the `agov coord convergence --series <id>` subcommand — from a downloaded release archive, not a local build.
    - Report the exact release version checked and the command-tree result.
  must_read:
    - agf/docs/site/reference/tooling-conformance.md
    - agf/docs/site/reference/maturity-vocabulary.md
    - CHANGELOG.md
  do_not_touch:
    - the tooling-conformance.md / maturity-vocabulary.md cells themselves (the steward applies the mechanical fix once you report)
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# DES-4: does the released `agov` ship `coord convergence`?

Estate-sweep finding **DES-4** (`pjsp`), routed to build-engineer per
governor "proceed as proposed" (2026-09-10).

## The discrepancy

`agf/docs/site/reference/tooling-conformance.md:52` marks
`agov coord convergence --series <id>` as **"Level 4 validated candidate;
not deployable until released"**. But:

- it was announced in the **v2.8.0 CHANGELOG** (2026-08-28) with delivered
  language ("`agov coord convergence --series <id>` add a per-invariant
  two-failure abstraction-reset boundary…"); v2.8.0 / v2.9.0 / v2.10.0
  have all since shipped;
- `agf/docs/site/docs/adopt/mcp-server.md:108` maps `agov_coord_convergence`
  → `agov coord convergence …` as a live MCP tool.

## What I need

One fact: **does the v2.8.0+ (ideally v2.10.0) release archive's `agov`
binary have `coord convergence` in its command tree?**

## Then the steward applies

- **If yes** (command shipped): change `tooling-conformance.md:52`'s status
  cell from "not deployable until released" to **"Release surface"**
  (matching the sibling rows). Separately decide whether the "Level 4"
  framing should be kept as a statement about the *review-convergence
  workflow's* evidence status (distinct from the command's maturity) or
  dropped; update `maturity-vocabulary.md` L28-29's worked example, which
  currently uses `coord convergence` as its "Level 4 in a branch"
  illustration.
- **If no** (command genuinely not in the release): the cell is correct;
  DES-4 closes as accept-as-current.

No cell edit until your result. This is the last open item for the first
estate sweep (v2.11 release-cut condition 4).
