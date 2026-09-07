---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260906T155934Z-contribution-auditor-mjqa
timestamp: 2026-09-06T15:59:34Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260906T123407Z-contribution-auditor-52nt.md
  - 20260906T143703Z-coder-tester-gjgs.md
topic: post-v290-release-scope-inventory-to-gate1-plan
requires-ack: true
artifacts_owned:
  - docs/reference/post-v290-release-scope-inventory.md
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - >-
      Take docs/reference/post-v290-release-scope-inventory.md (MR !2439,
      contribution-auditor draft, gate: none) as pre-planning input and
      convert it into the Gate 1 release plan for the post-v2.9.0 window.
      The draft is a disposition inventory (all 77 open MRs + 61 issues
      bucketed with cause); it is deliberately NOT sequenced.
    - >-
      Re-cut Part A against the now-existing v2.9.0 tag (commit ee27978b,
      published/verified per gjgs) instead of v2.8.0..main.
    - >-
      Add the two things the draft explicitly lacks: (a) a priority /
      sequencing pass, (b) a blocks / blocked-by dependency table. Known
      un-mapped dependencies to resolve: gate-self-test-rule.md changed on
      main without a visible negative test (possible release blocker);
      !2362 route-all-jobs gate-blocked per !2366; !2392 native-musl vs
      #109 runner migration; Propeller #124->#125->#126->#127 internal
      order; !2400/!2409 run-list must merge before it can be cited.
    - >-
      Required consultations for the resulting Gate 1 plan (name them in
      advisory_consultation): build-engineer and capability-developer.
      infosec-sentinel assessed per included item at that item's own
      Gate 1, not on this inventory.
  must_read:
    - docs/reference/post-v290-release-scope-inventory.md
    - 20260906T123407Z-contribution-auditor-52nt.md
  do_not_touch:
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Handoff: post-v2.9.0 release scope inventory -> Gate 1 release plan

Operator-directed (2026-09-06). v2.9.0 tagged at `ee27978b` 13:13:29Z and
verified (`gjgs`), so the post-v2.9.0 window is open and the scope boundary
is now fixable.

## What is handed over

`docs/reference/post-v290-release-scope-inventory.md` (MR !2439) — a
contribution-auditor **draft**, `status: draft`, `gate: none`. It
dispositions every open MR and issue with cause into: auto-included
(Part A), coordination-drain / near-ready content / Gate-1-pending plans /
stale-close (Part B), issues selected with cause (Part C), and a "do not
pull forward" list for items parked for this window (Part D). It grants no
approval and is not sequenced.

## Why build-engineer and capability-developer are the named consultations

- **build-engineer.** A large fraction of Part A/B is CI / runner /
  packaging: !2362, !2392, #101/#102/#106, #109, the native-musl / DinD
  work, and the release's own pipeline cost. build-engineer also owns the
  open question of whether `gate-self-test-rule.md` on `main` shipped with
  its required negative test — that is a potential release blocker, not a
  footnote.
- **capability-developer.** Part C #124-#127 and all of Part D (#128,
  #121-#123, !2027, !1817) are its lane. The "do not pull forward" calls
  and the Propeller-launch sequencing should be capability-developer's to
  confirm, since its one-shot advisory study is meant to run before
  Propeller implementation (MANIFEST, 2026-09-05 reactivation).

## Companion record

`52nt` (MR !2437) carries the four stale Gate-1-plan-MR close
recommendations (!1723, !1750 + the Part B4 list) as a separate
governor-reviewer disposition; it is not blocked by this handoff.

## Authority

This handoff routes work; it does not approve the release scope or the
version number. Gate 1 on the resulting plan, and the version-number
decision, remain governor-reviewer's.
