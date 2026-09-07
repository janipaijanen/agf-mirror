---
kind: decision-record
from: governor-reviewer
to: all
id: 20260907T140640Z-governor-reviewer-smem
timestamp: 2026-09-07T14:06:40Z
model: human
immutable: true
references-to:
  - 20260907T134221Z-governor-reviewer-p1j2.md
  - 20260907T140207Z-coder-tester-jj56.md
  - 20260907T045703Z-coder-tester-jqrm.md
topic: v210-ruw-remaining-sequence-proceed-as-proposed
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Proceed as proposed — remaining v2.10 RUW sequence

Issued as a binding in-session directive (2026-09-07, "when possible,
proceed as proposed"), transcribed to coordination. It endorses the
remaining-work sequence for the approved v2.10 release plan
(`docs/plans/20260907T045703Z-v210-release-plan.md`) as laid out this
session:

1. **RUW-3** — implement Propeller Stage 3 (#126/#127) as the approved
   plan's atomic role/rule/docs/projection unit, **planned/inactive**
   (claimed by Codex, `jj56`).
2. **RUW-4** — issue #120 pre-tag namespace-absence guard, via its **own
   child Gate 1 plan** + implementation + Gate 2, merged before the
   release cut; it also finalizes the namespace-absence command set the
   release preflight consumes.
3. Create the `v2.10.0` protected-tag rule (Maintainers-only) before the
   preflight.
4. **Reversible preflight** at the candidate `main` SHA — no commit, no
   tag; full validator/build/test run and four-namespace absence proof.
5. **Release-metadata MR** — `release-2.10.0`, the 14-path allowlist only,
   standard MR + exact-head pipeline + independent release-diff review +
   Gate 2.
6. **Tag** `v2.10.0` on the proven merge commit, with the
   immediately-before-tag namespace/policy/identity re-check, then
   post-tag artifact + namespace verification.

RUW-3 and RUW-4 may proceed in parallel. Codex owns the RUW-3 and RUW-4
implementation lanes and does not need per-lane re-authorization to
*advance the sequence*.

## What this does NOT authorize

No gate is approved by this record. Each of the following still requires
its own binding governor decision at the exact head:

- RUW-3 Propeller Stage 3 **Gate 2**;
- the **Stage 4 activation** decision (player, cadence, heartbeat, cursor
  init, first run) — mandatory for v2.10 per `rhpb`, and the genuine
  wake-path first-run proof gates the Level 5 / Level 4 maturity claim
  (`c4mt` C-A1);
- the #120 child **Gate 1** and its implementation **Gate 2**;
- the release-metadata MR **Gate 2**;
- the `v2.10.0` **tag** creation.

Independent contribution audit (and infosec/build/capability where the
plan requires them) remains distinct from the implementing session for
each RUW-3, #120-child, and release-metadata MR.

## Collision note

`jj56` records a path overlap between RUW-3 and Gate-1-pending MR !2038 on
`AGENTS.md` (RUW-3 edits the active-role discovery table; !2038 appends
stale-worktree guidance under Branch discipline — non-overlapping hunks and
semantics). The RUW-3 exact-head audit independently verifies that
non-overlap; !2038 does not displace the approved release unit.
