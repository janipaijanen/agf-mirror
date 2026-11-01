---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261031T104250Z-planner-architect-k5p4
timestamp: 2026-10-31T10:42:50Z
model: claude
immutable: true
references-to:
  - 20261031T101616Z-governor-reviewer-8ksb.md
  - 20261031T101017Z-planner-architect-b87a.md
  - 20261031T095048Z-planner-architect-6d8r.md
topic: pages-slice-2-batched-correction-ready-for-recheck
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Route the existing source and factual reviewers to recheck the corrected slice 2 head against the four findings of review 8ksb. Rendered review waits until source is accepted.
  must_read:
    - docs/reviews/pages-structure-and-voice/20261031-slice-2-preservation-and-anchor-map.md
  do_not_touch:
    - agf/rules/
    - agf/agents/
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Pages slice 2: batched correction ready for recheck

Under claim 6d8r. This record supersedes the readiness claim in b87a for head `764985c3`. Any change to the new head
invalidates this claim. I merge nothing to main until Gate 2 and the Pages-bearing main pipeline ends.

| Item | MR | Exact head (40 characters) | Pipeline |
|---|---|---|---|
| Slice 2 corrected candidate (draft) | !3462 | `8bbd049ac5b215ee7e83fa37b9813e7af50ac69b` | 3011623729, `success`, all seven jobs |

Jobs: commercial-corpus validate 17629744787, rust agov validate 17629744786, governance validate 17629744785, release paths
validate 17629744784, distribution validate 17629744783, distribution preview 17629744782, strict `site-html-build` 17629744781.
Base is `6bd6cfb02b594ecc5a2ce94eb96f56374cb330cd`. One push, one pipeline.

## Findings of review 8ksb and where each is addressed

1. **Decide structure.** `should-you-adopt.md` now puts the decision first: intro, practical test, Adopt/Wait/Skip, what adoption asks of you
   (with the privacy checklist). Then five short pointer sections: Understand the basics, Cost and accountability, Technical fit,
   Assistant workflows, Selected use cases (three summaries that link to `use-cases.md`). The retained inventory ("What AGF is / is not",
   "What ships and what you activate") follows, then Next. `downstream-impact.md` gains "What adoption asks of the people involved"
   (attention, repository changes, responsibility and privacy, terms and exit) before the implementation inventory. The gate-split FAQ
   answer is shorter and points to the roles page. I did not shorten other FAQ answers; they keep their safeguards. No heading removed or renamed.
2. **FAQ conflict rule.** The blanket prohibition is replaced by the `ADOPTING.md` section 1.1 rule: prefer an approver who did not
   implement the change; Phase 1 temporarily permits one person to play both Coder-Tester and Governor-Reviewer; record it; an
   implementing assistant never approves its own work. No general waiver.
3. **Default human path.** The Decide intro says a human approves each gate by default, and points to the narrow, default-off,
   provisional deputy route and to the roles page section. "Adopt now if" now says the governor is "accountable for" plans and merges.
4. **No self-approval beside the deputy claim.** The FAQ paragraph keeps written scope, severity ceiling, expiry, the red list,
   default-off and provisional status, no self-approval, and ultimate human authority in one place.

## Evidence

- Original A1 script (hash `aaa2cb0444c362cc`) on the rendered strict-build artifact of job 17629744781: Evaluate answer-key sections
  68, 135, 131, 71 and 265 words = 670; with Home 312 = **982**, equal to the ceiling and to the baseline. The new sections count
  58, 38, 51, 55 and 54 words and are outside the answer key. Whole Decide page: 2,005 words (1,712 before this batch, 1,690 base).
  I do not present the whole-page growth as a reading reduction. The Adopt (1,080) and Operate (1,241) totals are untouched pages.
- Preservation and anchor report regenerated: every "not found" sentence has a disposition. Added ids only: `understand-the-basics`,
  `cost-and-accountability`, `technical-fit`, `assistant-workflows`, `selected-use-cases`,
  `what-adoption-asks-of-the-people-involved`. All cross-file anchors resolve.
- Strict `agov plan projection-check` on the parent manifest: OK. Owner projection updated.

## Not done

No browser run, no rendered desktop or mobile review, no Home edit. Home links to the four Decide audience sections stay with the Home slice.
R1, R2, Home q3 and the Operate +67 cap stay with their owners. Gate 2 is not requested.
