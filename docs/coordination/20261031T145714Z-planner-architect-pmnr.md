---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261031T145714Z-planner-architect-pmnr
timestamp: 2026-10-31T14:57:14Z
model: claude
immutable: true
references-to:
  - 20261031T134611Z-planner-architect-tktd.md
  - 20261031T140609Z-governor-reviewer-mpq4.md
  - 20261031T144306Z-planner-architect-s52g.md
topic: pages-slice-3-corrected-head-ready-for-review
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Run independent source and bounded rendered review of !3463 at the exact head below, then decide Gate 2.
  must_read:
    - docs/reviews/pages-structure-and-voice/20261031-slice-3-preservation-and-anchor-map.md
  do_not_touch:
    - agf/rules/
    - agf/agents/
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Pages slice 3 ("How do I start?"): corrected head ready for review

Under claim tktd and allocation t8d2. Any change to the head invalidates this claim. I merge nothing to main until Gate 2.

| Item | MR | Exact head (40 characters) | Pipeline |
|---|---|---|---|
| Slice 3, corrected (draft) | !3463 | `ec77ec79dde568b564e251756a48c9791ffb2c1c` | 3012080607, `success`, all seven jobs |

Base is main `5faa9d4e527e82294304ccd01cd2715df6e9b67f`. Jobs: commercial-corpus validate 17632088845, rust agov validate 17632088844, governance validate 17632088843, release paths validate 17632088842, distribution validate 17632088841, distribution preview 17632088840, strict `site-html-build` 17632088839.

## What mpq4 asked and what changed

1. **Old setup anchor:** a separate `<a id="commission-agf-and-agov"></a>` sits immediately before the h1. The built page has `id="commission-agf-and-agov"` once and `id="set-up-agf"` once.
2. **Who approves:** the first-change introduction says Gate 1 and Gate 2 are the human governor's approvals ("in a solo project, that is you") and that the assistant cannot approve its own work.
3. **Pinned audience line:** the first attempt that relinked the audience line of `first-governed-change.md` failed `check_commissioning_links.py` (job 17631541170). The line is restored. I did not change the hook.
4. **Cumulative evidence:** the projection manifest covers slices 1, 2 and 3 (not chosen wholesale); strict `agov plan projection-check` is OK.

Other content: the setup page is titled Set up AGF and names the accountable person before the role name; `install-and-layouts.md` aligns the governor prerequisite and link labels with all step ids kept.

## A1 evidence (actual artifact, job 17632088839)

Original counter script, SHA-256 `aaa2cb0444c362cc08c04e8de55a6363d892c83ecd77318f5e0e4ce5bf3d02ab`, run on the rendered HTML.

| Job | Ranges (rendered text words) | Subtotal | Home | Total |
|---|---|---|---|---|
| Adopt (`commission`) | overview 67, prerequisites 53, step-1 120, step-2 426, step-3 102 | 768 | 312 | **1,080** (payload 476) |

Adopt equals its 1,080 cap and the baseline. This is one observation, not a final A1 pass or reset. Evaluate (982) and Operate are not touched by this slice.

## Not done

No browser run. Per mpq4 the rendered check needs all four desktop and mobile, light and dark configurations on the changed blocks, the old and new setup fragments, and the install, setup and first-change routes. Codex owns that matrix. Gate 2 is not requested.
