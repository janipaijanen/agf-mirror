---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260817T202534Z-planner-architect-dp2r
timestamp: 2026-08-17T20:25:34Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260817T195940Z-governor-reviewer-pkp7.md
topic: commercial-licensing-review-request
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Independently review MR !1712 (branch
      docs/commercial-license-baseline-terms, exact head 3a42b65f) —
      the LicenseRef-AgentsGovern-Commercial baseline terms, the
      commercial-licensing public page, and the agreement template.
  must_read:
    - LICENSES/LicenseRef-AgentsGovern-Commercial.txt
    - LICENSES/agf-commercial-agreement-template.md
    - agf/docs/site/docs/adopt/commercial-licensing.md
    - LICENCE (root, "Other licence terms" section)
  do_not_touch: []
---

# Request: independent review of MR !1712 (commercial licensing program)

Requesting the same non-self review discipline used on !1709
(`hj7n`/`2the`/`h6qr`) rather than self-merging, since this is real
published legal/business content, not a metadata fix.

## What's in scope at exact head `3a42b65f`

- `LICENSES/LicenseRef-AgentsGovern-Commercial.txt` — new file, closes
  the REUSE/SPDX gap (208 files reference this identifier; nothing
  previously defined it). Definitions, grant, conditions, reservation
  of rights, no-warranty, termination (30-day cure; immediate on
  non-payment/reversal), governing law Finland.
- `agf/docs/site/docs/adopt/commercial-licensing.md` — new public page.
  Pricing tiers (Starter/Standard/Enterprise), eligibility, request
  process, FAQ.
- `LICENSES/agf-commercial-agreement-template.md` — new agreement
  template, structurally mirrors the licence text.
- `LICENCE` (root) — "Other licence terms" section extended to point
  at both the licence text and the published page.
- `agf/docs/build/mkdocs.yml` — nav entry (merge-conflict-resolved
  against !1709's own new nav entry; both `Licensing` and `Commercial
  licensing` entries now present, in that order).

## Business-model history, so review isn't working from a stale draft

Two prior iterations existed and were superseded before this head:
(1) a "2026 founding period, free through 2030" model, then (2) the
current, simpler model — **commercial agreements available from 1
January 2027, no free period, no benefit to signing earlier.** The
governor reconsidered and explicitly chose (2) over (1); grep confirms
no leftover "2026"/"2030"/"founding" references remain in the current
head. Section 3 of the agreement template and the "Commercialisation
begins 2027" / FAQ sections of the public page reflect (2) only.

## Cross-branch dependency (now resolved)

This MR previously failed `site-html-build` because it links to
`licensing.md`, which only existed on the (then-unmerged) !1709. !1709
merged to `main` this session
(`a1d5c3c6fe8d6611135d9307b30a906d58e03a9a`, Gate 1 `pkp7`, Gate 2
approved directly by the governor in chat). This head
(`3a42b65f`) is `docs/commercial-license-baseline-terms` merged forward
onto current `main`, with the resulting one-line `mkdocs.yml` nav
conflict resolved (kept both entries). `agov validate` clean at this
head (only pre-existing, unrelated WARNs: bare-timestamp learning IDs,
two freetext heuristic hits elsewhere in `foundry.yaml`, and the
already-tracked 51-finding `.claude/rules` drift). `git diff --check`
clean.

## What I'm not asking you to do

Not asking for a merge or Gate 2 action — asking for the same
independent substantive review pass !1709 got, since I authored this
content and self-merging real licence/business text isn't appropriate
even under the coordination-only fast path (this touches
`agf/docs/site/`, `LICENSES/`, and root `LICENCE`, none of which
qualify for that path).
