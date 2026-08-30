---
kind: decision-record
from: governor-reviewer
to: all
id: 20260830T142342Z-governor-reviewer-q8dz
timestamp: 2026-08-30T14:23:42Z
model: human-directed-claude-opus-5
immutable: true
references-to:
  - 20260830T135930Z-coder-tester-mp5v.md
  - 20260830T135126Z-coder-tester-3rh6.md
  - 20260830T085834Z-governor-reviewer-dq5a.md
topic: gate2-approval-gate-change-discipline-ab-and-v290-slice-a
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2 approved: gate-change discipline A+B, and v2.9.0 Slice A

Binding governor-reviewer decision, given in session and transcribed by the
assistant under the `human-directed-claude-opus-5` marker.

## Approved

**MR !2237** — gate-change discipline slices A and B, exact head
`a727bb96f7b236b07a35fc0e7aaca4d86ecb6e25`. Gate 1 was approved by `dq5a`;
independent review `3rh6` recommends CLEAR at this exact head with zero
blocking and zero advisory findings, after five earlier rounds each of which
found and closed a real defect. Canonical pipeline `2803335087` passed;
required self-test job `16190764052` reported 29 passed, 0 failed.

**MR !2198** — v2.9.0 Slice A, exact head
`9582cddc87f6`. Gate 1 approved by `h3nb`. The OCI scan anchor now matches
the three-pattern closure its own approved plan specifies; the earlier
eight-path enumeration left 37 of 84 files under `tooling/rust/` unmatched,
including `Cross.toml` and both crates' `build.rs`.

## Not approved, and why

**MR !2222** (slice C Gate 1) is **not** approved by this decision. The
independent review requested in `6bh8` has not been performed — it was
correctly queued behind the higher-priority Gate 2 obligation and remains
outstanding. Approving it now would skip the check that found five defects in
its sibling. It returns for decision once that review is filed.

## Sequencing authorized

1. Merge !2237.
2. Update MR !2206 onto the resulting `main` so its pipeline picks up the
   corrected validator feed, then re-run. Its current failure is by design:
   it deletes a plan file, and the deletion-blind validator !2237 repairs is
   what rejects it. This is acceptance criterion 12 and is verified after
   merge, not before.
3. Merge !2206, the previously ordered revert of the prematurely merged
   `pending-gate1` plan.
4. Merge !2242, learning `20260830T114500Z-tw3s`. It was correctly held so a
   record claiming `implemented` would not reach `main` ahead of the
   implementation it describes.
5. MR !2207 regains a non-empty diff once step 3 lands; it is not merged by
   this decision and returns for Gate 1 on its own terms.

## Residual accepted, restated

Until slice C lands, gate-change discipline is enforced by rule and human
review, not by mechanism. This was accepted knowingly at Gate 1 in `dq5a`
and is unchanged.
