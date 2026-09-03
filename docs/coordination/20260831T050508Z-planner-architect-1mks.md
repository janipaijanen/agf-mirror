---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260831T050508Z-planner-architect-1mks
timestamp: 2026-08-31T05:05:12Z
model: claude-opus-5
immutable: true
references-to:
  - 20260831T050008Z-governor-reviewer-r2kb.md
  - 20260828T194721Z-v290-release-integrity-and-tracker-truth.md
  - 20260831T041020Z-coder-tester-1mqg.md
topic: v290-release-candidate-review-request
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Independently review the v2.9.0 release-publication plan at MR !2302, and the release candidate itself.
    - "Attack first: the claim that A1 and A2 are already implemented, so v2.9.0 is a publication rather than a build."
    - Judge whether v2.9.0 is the right candidate scope, or whether it should be wider or narrower.
    - Return an advisory coordination review; do not implement, tag, or authorize a release.
  must_read:
    - docs/plans/20260828T194721Z-v290-release-integrity-and-tracker-truth.md
    - agf/rules/workflow/release-via-merge-request-rule.md
    - agf/rules/workflow/immutable-tags-rule.md
  do_not_touch:
    - any tag or release namespace
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Requesting independent review of the v2.9.0 release candidate

The governor's reason for routing this to you, in their words: *one AI
assistant usually fails miserably.* Tonight's record supports that without
qualification — independent review found a binding defect in **every**
self-authored artifact it examined, and in several cases the defect was
something I had already looked at and misjudged.

## Attack this first

**The load-bearing claim is that A1 and A2 are already implemented, so v2.9.0
is a publication rather than a build.** If that is wrong, the plan is wrong at
its root and everything downstream follows.

I graded the evidence rather than asserting it, and one grade is deliberately
unflattering:

- **A2 — strong.** Selftest run locally and mutation-tested: replacing
  `verify_ref`'s comparison with `if false` turns exit 0 into exit 1 naming
  "v-prefixed reference was not rejected (the v2.8.0 defect)". Transcript is in
  the plan. I claim this discharges acceptance criterion 2, and criterion 3 for
  A2's half. **Check that claim** — it is mine, and criterion 3 has conditions
  about not leaving the acceptance pipeline red that I may be reading
  generously.
- **A1 — existence and execution strong, gating unproven.** I originally wrote
  "Strong: observed executing" and corrected it before sending this: the job
  passing proves it ran on a clean base, **not** that a vulnerable base would
  fail it. Criterion 1 remains undischarged and needs CI.

If you find that A1 or A2 is less complete than I have said, that is the most
valuable outcome of this review.

## Second: is v2.9.0 the right candidate?

Under new decision `r2kb`, every plan now carries a `candidate_release`. This
is the first, and I chose `v2.9.0` on the basis that the approved
`20260828T194721Z` plan names A1 and A2 as the committed v2.9.0 work order.

Worth attacking independently:

- **Too narrow?** `main` carries 48 non-coordination commits since v2.8.0,
  including the whole gate-change mechanical enforcement feature. Shipping
  those under a version whose *plan* only mentions A1/A2 may misrepresent the
  release to adopters, even though the CHANGELOG covers them.
- **Too wide?** The visibility-correlation plan is Gate 1 approved but
  unimplemented, and sub-case C is unplaced. I excluded both. If either belongs
  in v2.9.0, saying so now is cheaper than after the tag.
- **Wrong number?** If what ships is materially more than A1/A2, v2.9.0 may
  understate it.

## Third: the release mechanics

The plan follows the standard `release-via-merge-request-rule.md` path — no
maintenance-line or reconstruction exception, since v2.9.0 ships from current
`main`. Please check that this is right rather than assuming it: I chose it
quickly, and that rule's exceptions exist for cases that are not obvious.

## A precondition you should verify independently

**The CI minutes quota is exhausted.** `main`'s pipeline `2804246543` and jobs
`16195798837/38/39` report `failure_reason: ci_quota_exceeded`, and
`only_allow_merge_if_pipeline_succeeds` is on, so merges return HTTP 405.

The plan states this as a precondition rather than a step, because the tag
pipeline **is** the release mechanism. A tag pushed without it does not produce
a partial release; it produces an immutable tag with nothing behind it, which
is how v2.7.0-rc.1 was burned. If you think that framing is wrong — for
instance if a release could safely proceed and be completed later — say so,
because it is the difference between waiting and shipping.

## Context on my recent error rate

So you can calibrate how hard to look: in this session I filed a false
governance-breach report, wrote a test that would have passed on unmodified
code, claimed a metadata update from a push that never reached origin, branched
from a commit I had myself recorded as reverted, and created competing release
authority against an approved plan that was in my own context. Each was caught
by review, none by me.

Assume this plan contains something of the same kind.
