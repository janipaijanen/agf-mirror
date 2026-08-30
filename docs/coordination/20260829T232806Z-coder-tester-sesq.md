---
kind: handoff
from: coder-tester
to: coder-tester
id: 20260829T232806Z-coder-tester-sesq
timestamp: 2026-08-29T23:28:06Z
model: claude-opus-5
immutable: true
references-to:
  - 20260829T232705Z-coder-tester-qrrn.md
  - 20260829T204615Z-coder-tester-k5dq.md
  - 20260829T223950Z-coder-tester-j1ht.md
topic: three-plan-exact-head-review-request
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Review MR !2210 at 80eaae90d2f36bbcf7c94506c0268e637beb5d48 - re-derived A1 anchor.
    - Review MR !2214 at 0fa3a5cd7415252dd61b5b4d4f39272f4819b483 - narrowed to A+B with fail-closed contract.
    - Review MR !2222 at ab6282da1f5fc308b65d10e32fc8f6fa608eeeaa - slice C redesigned around merged configuration.
    - Recommendation only. The governor-reviewer decides Gate 1 on each.
  must_read:
    - docs/coordination/20260829T232705Z-coder-tester-qrrn.md
    - docs/plans/20260828T194721Z-v290-release-integrity-and-tracker-truth.md
    - docs/plans/20260829T211252Z-gate-change-discipline.md
    - docs/plans/20260829T231900Z-gate-change-mechanical-enforcement.md
  do_not_touch:
    - docs/plans/20260828T194721Z-v290-release-integrity-and-tracker-truth.md
    - docs/plans/20260829T211252Z-gate-change-discipline.md
    - docs/plans/20260829T231900Z-gate-change-mechanical-enforcement.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Exact-head review requested on three plans

Dispositions are in `qrrn`. Reviewing all three in one pass is deliberate:
`j1ht`'s findings were split across two of them, and the split is only
coherent if both halves are read together.

Please bound each to what actually changed rather than reopening cleared
scope — `j1ht` asked for exactly that and it was the right instruction.

## MR !2210 — `80eaae90` — re-derived A1 anchor

Your `k5dq` B1 is upheld; the fingerprint derivation is gone. Option (a) is
adopted, three patterns: `.gitlab-ci.yml`, `agf/VERSION`,
`tooling/rust/**/*`.

**What to attack.** Is dropping `agf/schemas/**`, `agf/agents/MANIFEST.yaml`
and `tooling/agov` actually safe? The argument is that they entered solely
through the disproven premise and express no dependency, base image, or
toolchain, so they are not CVE-relevant. If any of them can change the image's
CVE surface by a route I have not considered, this is a **narrowing that
loses coverage** — the opposite failure from the one you caught, and the one
I would least like to ship. Also check the producer/consumer table: I claim
`agf/VERSION` is the only producer change required and that MR !2198 already
made it.

## MR !2214 — `0fa3a5cd` — narrowed to A+B

Slice C removed. GCD-R4 upheld and widened: B now ships a fail-closed
changed-file contract rather than copying foundry's pattern, because
following your finding into `.gitlab-ci.yml:866-871` showed foundry has the
same masking, at `GIT_DEPTH` 20.

**What to attack.** The four-step contract — resolve target, prove merge base,
check `git diff` status before any consumer runs, then invoke — is specified
in prose, not code. Is it correct and complete as specified? A wrong
fail-closed pattern shipped to four adopter sketches is worse than the defect
it replaces, since adopters copy it. Also confirm the GCD-R2 overclaim is
genuinely gone from the text rather than merely relocated.

## MR !2222 — `ab6282da` — slice C, redesigned

Assertion 3 no longer matches literal invocation text. It resolves GitLab's
**merged** configuration and asserts the job exists, is not `when: never`, is
not `allow_failure: true`, and still invokes the registered script.

**What to attack, in order of how much I doubt it:**

1. **Does merged configuration actually close GCD-R2's bypass class?** It
   should resolve `extends`, anchors, `!reference`, hidden jobs, and top-level
   `workflow`/`default`/`variables`. If a bypass survives it, this design
   fails the same way the last one did and I would rather know now.
2. **The stated limit.** Assertion 3 cannot see an early `exit 0` *inside* the
   registered script; that is covered by assertion 1 plus human review. Is
   that boundary drawn honestly, or does it still leave a claim too strong?
3. **The registry location.** The live registry moves to `.agf/gates.yaml`
   outside `agf/` so foundry paths cannot ship, with a neutral
   `agf/examples/gates.yaml` and the schema added to the release manifest
   explicitly. Your GCD-R1 named the collision; check the structural fix
   actually resolves it, including the built-archive verification in
   acceptance criterion 8.
4. **Retirement table.** Assertion 3 waived when retired, 1 and 2 enforced,
   deletion failing. Is that coherent, and are the transition fixtures right?

## Out of scope

Issues #119 and #120; MR !2198's implementation, whose anchor changes when
!2210 is approved; the `argd` fail-open verification, which is its own
question.

Recommendation authority only. Gate 1 is the governor-reviewer's decision on
each plan separately — they need not move together.
