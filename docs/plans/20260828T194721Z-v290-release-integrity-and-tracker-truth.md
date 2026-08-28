---
id: v290-release-integrity-and-tracker-truth
status: pending-gate1
topic: v290-release-integrity-and-tracker-truth
advisory_consultation:
  - role: coder-tester
    status: waived
    reason: >-
      Not yet consulted. This plan is planner-architect self-authored and
      its evidence is drawn from a session in which coder-tester was the
      counterparty for most of the incidents cited. A bounded independent
      coder-tester review is requested before binding Gate 1, on the same
      basis used throughout the v2.7.0/v2.8.0 cycle, where four
      consecutive independent reviews each found real defects in
      self-authored release artifacts.
  - role: contribution-auditor
    status: waived
    reason: >-
      No separate contribution-auditor pass is proposed. The bounded
      independent coder-tester review requested above is offered as the
      substitute independent check, on the same basis the governor
      accepted for v2.6.0 (`kpkk`, `33br`) and throughout the
      v2.7.0/v2.8.0 cycle: a genuinely independent model and session
      re-deriving the plan's factual claims. The governor ratifies or
      rejects that substitution at Gate 1; this field records the
      request, not a granted waiver.
  - role: infosec-sentinel
    status: waived
    reason: >-
      Plan-specific semantic waiver requested, not granted. Slice A
      concerns container CVE scanning and release-namespace integrity, so
      the configured infosec-sentinel CI actor's mechanical checks
      (secret scanning, dependency CVEs, SPDX) are directly relevant and
      remain mandatory throughout. They cannot, however, author the
      semantic CI-topology and release-evidence review this plan needs;
      that is the governor's decision at Gate 1.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: v2.9.0 — release integrity and tracker truth

## Decision requested

Authorize implementation of the v2.9.0 scope below. Every item is drawn
from a defect that actually occurred during the v2.7.0/v2.8.0 cycle on
2026-08-27/28, with the incident named. Nothing here is speculative
hardening.

Requested per the roadmap direction the governor approved in
`20260827T203743Z-governor-reviewer-nsne.md`, whose v2.9 cluster was
"tracker truth and release cost". That framing survives; this plan
sharpens it with the evidence the cycle produced.

## Starting state

`main`'s `## [Unreleased]` is empty — v2.8.0 consumed it. v2.9.0 begins
with zero banked commits, so its scope is entirely forward work. 52
issues are open, 47 labelled `release::unmilestoned-backlog`.

## Slice A — release integrity (highest value; both items burned a release)

**A1. Pre-tag container CVE coverage.** `agov:oci:release` is gated
`if: $CI_COMMIT_TAG` only — verified. It is the sole job that builds and
Trivy-scans the image, so a container CVE can only surface **after** the
tag is immutable. That is precisely how `v2.7.0-rc.1` was burned: the tag
published, `agov:oci:release` failed on `CVE-2026-14456` (HIGH,
`libcrypto3`/`libssl3` `3.5.7-r0`, fixed `3.5.8-r0`), and
`release:create`/`release:verify` were skipped, leaving a tag with no
Release, package, or image.

Deliver an MR-stage build-and-scan job triggered by the paths that can
change image content — at minimum `tooling/rust/Containerfile.agov`,
`tooling/rust/scripts/agov-oci-image.sh`, `tooling/rust/release-targets.txt`,
and the Rust workspace manifests. Per `always/gate-self-test-rule.md` it
ships with a negative test proving it fires: a deliberately vulnerable
base pin must fail the job, with the transcript in the MR.

Note the partial fix already on `main`: `045e514c` added the Containerfile
to the `agov-oci-image:selftest` path anchor so Containerfile-only changes
stop producing empty pipelines. That selftest validates target resolution
and **does not build or scan**; it is not coverage, and the file says so.

**A2. Single-source the OCI reference.** The immediately-pre-tag
namespace check queried `v2.8.0` while `image_ref()` publishes
`agov:$AGOV_VERSION` — verified at `agov-oci-image.sh:191`:
`printf '%s/agov:%s' "$CI_REGISTRY_IMAGE" "$AGOV_VERSION"`. So the actual
namespace was never checked for absence, and OCI tags are mutable, so the
push is not retrospective proof. Disclosed in `q5mm`, accepted in `gs18`.

Derive the checked name and the published name from one source so the
divergence is impossible rather than merely detected. Negative test: a
deliberately mismatched spelling must fail.

## Slice B — coordination visibility (the highest-frequency failure)

Three occurrences on 2026-08-27/28, one with real consequence: MR !2174
was merged over coder-tester's `k4mk` BLOCK because that record sat on an
unmerged branch, invisible to `agov coord list`, which reads the current
branch only. The same cause produced a false "missing Gate 2" alarm
(`pbpr`, withdrawn) and left `nsne` — a binding governor decision
addressed to planner-architect — unseen for hours.

`agov coord visibility --base-branch main` already detects this and
reports `bridge: missing`. The gap is that nothing *requires* consulting
it. Deliver: make an unpublished record addressed to another role
mechanically visible at the point of use — minimally, surface off-main
records addressed to the querying role in `agov handoff next` and
`agov coord pending`, so an agent polling `main` cannot miss a binding
decision aimed at it. Ships with a negative test.

Related, not conflated: **duplicate transcription.** `2h4k` and `2h2s`
are two binding governor records for one decision, authored independently
by two assistants that could not see each other's work. The outcome was
coherent only because the later record superseded the earlier one on the
same subject. Candidate mitigations — publish a transcription before
acting on it, or make one role the sole transcriber for a decision thread
— are recorded here for the plan review to choose between, not decided.

## Slice C — release-mechanics rules

**C1. Do not pin a drifting target in a Gate 2 record.** `ed75` pinned
`main` at an exact SHA; publishing `ed75` itself advanced `main` and
invalidated it, deadlocking the release until `prn4` reissued it bound to
the source head and tree. Pinned targets belong to the frozen-scope and
maintenance-line exceptions, whose targets are dedicated protected refs
that must not drift. Add the distinction to
`release-via-merge-request-rule.md`.

**C2. Empty pipelines block merges.** A change matching no job's
`changes:` rule creates a zero-job pipeline that GitLab marks failed;
with `only_allow_merge_if_pipeline_succeeds` enabled it is unmergeable
(pipeline `2798869592`). This is general, not Containerfile-specific.
Deliver a always-runs lightweight job, or an equivalent guard, so no
change can produce an empty pipeline.

**C3. Fast-track eligibility.** `simplicity-rule.md` excludes security
boundaries from the fast-track waiver; planner-architect claimed one
anyway for a security fix in `85540db4`. Upheld in `j37k`. Assess whether
a mechanical check on waiver-claiming commits touching security-relevant
paths is proportionate, or whether this stays reviewer judgment.

## Slice D — tracker truth (the approved v2.9 baseline)

Close as verified-shipped: `#32` (`coord post`, `handoff take/complete`
all exist) and `#37` (`agov-mcp` shipped, hardened by #86/#88). Finish
`#36` — `coord list --since` exists, `coord watch` does not. Then `#31`
issue templates, `#75` plan lifecycle, `#53` release feedback latency,
`#101`/`#106`/`#102` CI cost items, and `#43` governance amendment
workflow with sunset clauses.

`#43` is placed here deliberately: it is the mechanism that makes C1 and
future rule corrections cheap and repeatable rather than bespoke.

## Slice E — deferred, named so it is not lost

`#109` (self-hosted runner fleet) stopped being backlog on 2026-08-28
when CI quota exhaustion halted the release mid-flight — every job
reported `ci_quota_exceeded` with `started_at: null`. Not proposed for
v2.9.0 scope; recorded as the operational risk that materialised, for the
governor to milestone separately.

Two learnings are owed and belong with this work: `hgte`'s reconstruction
reverse-consumer closure, and the fast-track waiver error in C3.

## Out of scope

`docs/roadmap.md` creation, authorized in direction by `nsne`, is a
separate Gate 1 plan; it is referenced here, not folded in. No stable
release promotion, no change to any published v2.8.0 identity, no
reconstruction work — the frozen-scope approach is abandoned per `j37k`.

## Acceptance criteria

1. A1 and A2 each ship with a negative test whose transcript is in the MR,
   per `always/gate-self-test-rule.md`.
2. A Containerfile change with a knowingly vulnerable base fails at MR
   stage, not at tag stage.
3. A deliberately mismatched OCI reference spelling fails the pre-tag
   check.
4. An off-main coordination record addressed to a role is surfaced to
   that role by the tooling it actually polls.
5. No change produces a zero-job pipeline.
6. `#32` and `#37` closed with evidence; `#36` completed or explicitly
   re-scoped.
7. No governed rule is weakened. Every check added is additive.

## Risks

The largest is scope creep: this plan names eight distinct defects and
could become a release that never ships — the failure mode the v2.7.0
cycle demonstrated. Slice A alone justifies v2.9.0; B and C are
high-value; D is mostly cheap closure. If the plan must be cut, cut from
D upward and ship A.

Second risk: A1 adds an image build to MR pipelines, which costs runner
minutes — directly relevant after the 2026-08-28 quota exhaustion. Scope
it tightly by path per `ci-trigger-scoping-rule.md` and measure the cost
before widening.
