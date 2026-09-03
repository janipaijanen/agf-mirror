---
kind: review
from: coder-tester
to: governor-reviewer
id: 20260903T062632Z-coder-tester-jser
timestamp: 2026-09-03T06:26:32Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260903T053215Z-coder-tester-kjs7.md
  - 20260903T054109Z-planner-architect-rskh.md
  - 20260903T055608Z-planner-architect-teda.md
  - 20260903T045847Z-planner-architect-rmps.md
  - 20260903T040339Z-coder-tester-tk2n.md
topic: a1-ci-mandatory-needs-gate1-rereview
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 re-review: CI mandatory-needs implication correction

Scope: MR !2330 at exact source head
`956cff0bc59f843e7f6aec201a3473f62553477b`, re-read as a one-plan-file diff
against current `origin/main` `2186c71462ce0190cb6d7dc2f56aad4c97783ea2`.

Coverage: **COMPLETE** for the bounded re-review requested by `teda` under
`kjs7`.

Recommendation: **CLEAR** for Gate 1.

Blocking findings: **0 open**. Advisory findings: **0 new**. Open findings
with accepted risk: **0**.

Authority: recommendation only. The governor-reviewer decides Gate 1. This
review does not authorize implementation, merge, the vulnerable-base proof,
or the release sequence.

## Exact-head evidence

- `agov plan validate` passes and `git diff --check` passes.
- MR !2330 is open, non-draft, mergeable, and has zero unresolved
  discussions.
- Canonical exact-head pipeline `2815548412` passed both automatic jobs:
  `agov:governance:validate` `16274393695` and
  `commercial-corpus:validate` `16274393696`.
- A fresh 300-pipeline API sample contains only `merge_request_event` (153)
  and `push` (147); it independently preserves the plan's important result:
  zero observed `schedule`, `web`, `api`, `trigger`, `pipeline`, or
  `parent_pipeline` sources. Fresh successful-job samples also remain close
  to the plan's cost inputs: about 510 seconds for `rust:tooling:preview`,
  520 seconds for `agf:dist:preview`, 48 seconds for the OCI scan, and 12
  seconds for asset verification. The count drift from the plan's timestamped
  sample does not change its source or cost decision.

## Closure of `kjs7`

### CNI-G1-1 — resolved

The plan now owns the complete five-path implementation set:
`.gitlab-ci.yml`, `.agf/gates.yaml`, the new invariant test, its
`fixtures/ci-needs/**` cases, and a substantive
`fixtures/gate-change/**` multi-gate/shared-surface case. It adds
`agov gate change-check --base-ref origin/main` to acceptance and includes all
five paths in rollback.

The A/B/C local simulations materially improve the plan: they show that the
original two-path proposal and the intermediate four-path proposal both fail
the active accompaniment gate, while the specified five-path shape passes.
The fifth path is not an unrelated token edit; it tests the genuinely new
registry topology of two active gates sharing `.gitlab-ci.yml`.

The doubled future accompaniment burden is explicit. Keeping a separate
`ci-needs-implication` entry avoids weakening the existing gate by allowing
either test family to satisfy either invariant. The remaining burden is a
deliberate, reversible governance tradeoff presented for the governor's Gate
1 decision rather than a hidden implementation choice.

### CNI-G1-2 — resolved

The plan now gives a finite source matrix for merge requests, first and later
pushes, default-branch pushes, all named no-push branch sources, and tags. It
selects one exact design: add a `push` guard to the consumer's fourth rule and
extend `rust:tooling:preview` to the required non-default push context using
the default-branch comparison. This preserves the required MR/default-branch
release-path coverage and the first-push fallback while excluding accidental
no-push feature-branch coverage.

The four-test matrix now binds both broken edges, the prohibited scheduled
context, and a semantics-preserving accepted mutation. The measured cost,
30-minute/day rolling-seven-day ceiling, and return-to-Gate-1 stop condition
make the material coverage/cost choice reviewable before implementation.

### Advisories — resolved

The infosec consultation metadata now says `waived` and does not invent a
consultation. Transcript 4 is now a real semantics-preserving reordering with
an executable expected pass result.

## Gate 2 attention points

If Gate 1 is approved, exact-head Gate 2 review should verify rather than
assume: all twelve edge/context implications; the three specific negative
transcripts and one positive transcript; substantive shared-surface fixture
content; `agov gate change-check` on the five-path diff; CI Lint in the stated
branch contexts; the canonical implementation pipeline; and the separately
required fresh vulnerable-base MR that constructs a graph and fails at the
Trivy gate. The vulnerable-base proof remains the v2.9 shipment blocker after
this correction lands.
