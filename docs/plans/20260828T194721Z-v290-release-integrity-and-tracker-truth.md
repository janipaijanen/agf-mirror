---
id: v290-release-integrity-and-tracker-truth
status: pending-gate1
topic: v290-release-integrity-and-tracker-truth
advisory_consultation:
  - role: coder-tester
    status: consulted
    reason: >-
      Consulted and complete. Bounded independent review delivered in
      `20260828T224057Z-coder-tester-58kk.md` against exact head
      `a42f10a83c00f0438cf758d7f5580016f555b861`: coverage COMPLETE,
      recommendation BLOCK, four blocking findings and one advisory.
      Both Slice-A CI premises were independently re-derived and
      confirmed at `origin/main` `475e91d7`. This revision resolves all
      four findings; see "Response to review 58kk" below. A second
      bounded review of the resulting implementation
      (`20260829T184713Z-coder-tester-ngje.md`, MR !2198 head
      `01b3268c`) returned BLOCK with three further findings, all upheld
      in `rm3k` and addressed by this resubmission. That was the sixth
      consecutive independent review in the v2.7.0-v2.9.0 cycle to find
      real defects in a self-authored artifact; a third review of the
      corrected head is expected before Gate 2.
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

## Decision requested — RESUBMITTED

A prior binding Gate 1 approval was granted at 2026-08-29T17:06:49Z
(`h3nb`) against head `05c9d079`. It was **consumed and returned** by
governor decision `rm3k` after independent review `ngje` blocked the
implementation; see the resubmission section below. This document is
`pending-gate1` and carries no current approval.

Requested scope is unchanged: **A1 and A2 only**, with A3 as their shared
negative-test obligation, and the terminal-disposition table below
governing everything else. `h3nb` §1's five implementation conditions
remain the intended terms and are re-requested here, including its stop
condition: if A1 or A2 grows during implementation, stop and return to
Gate 1 rather than absorbing it. That condition is what routed B1 back
here rather than allowing a silent widening, so it is working as intended.

Authorize implementation of **A1 and A2 only** as the committed v2.9.0
work order. Nothing else in this document is proposed for v2.9.0; every
other item carries an explicit terminal disposition below and is either
excluded, closed out non-blocking, or named for a separately gated later
shipment.

Both committed items are drawn from defects that actually occurred during
the v2.7.0/v2.8.0 cycle on 2026-08-27/28, with the incident named. Both
burned a release. Nothing here is speculative hardening.

Requested per the roadmap direction the governor approved in
`20260827T203743Z-governor-reviewer-nsne.md`, whose v2.9 cluster was
"tracker truth and release cost". That framing survives, narrowed: the
release-cost half ships now; the tracker-truth half is dispositioned
below rather than committed.

## Gate 1 resubmission after review `ngje`

This plan was approved once (`h3nb`, 2026-08-29T17:06:49Z) and is
resubmitted. Independent review `20260829T184713Z-coder-tester-ngje`
blocked MR !2198 at exact head `01b3268c` with three findings, all upheld
by governor decision `20260829T202453Z-governor-reviewer-rm3k`:

| Finding | Disposition |
|---|---|
| **B1** A1 trigger is not the complete image-input graph | Upheld. Closing it widens both the job's anchor and its producer's, so `h3nb`'s stop condition applied and A1 returns here. Addressed by the derived closure in A1 below. |
| **B2** A2 was optional detection, with no binding amendment permitting it | Upheld. A2 is amended by `rm3k`, recorded in A2 below. |
| **B3** the `agf/CHANGELOG.md` third path was self-authorized | Upheld. The change is approved; the process nonconformance is recorded in `rm3k` and needs nothing from this plan. |

## Response to review k5dq (second Gate 1 review, Codex host)

Independent review `20260829T204615Z-coder-tester-k5dq` returned **BLOCK** on
head `4a5d42ec` with one material finding. It is upheld and the anchor is
re-derived.

| Finding | Disposition |
|---|---|
| **B1 (k5dq)** the proposed authority does not govern the canonical packaged binary | **Upheld, derivation replaced.** The review downloaded the real cross-built artifact and proved `source_fingerprint` is `"unknown"` on the canonical packaging path, so the fingerprint script cannot govern the anchor. The anchor is re-derived from the shipped artifact and the CVE gate's purpose, as option (a): a safe over-approximation with `agf/VERSION` as the mandatory pre-release checkpoint. Option (b), byte identity, is unattainable by any path anchor because every commit changes image bytes through the embedded SHA. |
| Internal contradiction on `agov-mcp/build.rs` | Corrected. Neither `build.rs` is cited as a reason for any path; both are covered by `tooling/rust/**/*` regardless. |
| Producer anchor stated ambiguously | Corrected. Both final anchors are stated literally in a table in A1, including `agf/VERSION` and the fact that MR !2198 already added it. |

The review confirmed the two candidate-path rejections, the
producer/consumer invariant, the A2 amendment (**CLEAR**), artifact reuse,
the scan budget, the unpublished `localhost` namespace, and the no-push
separation. Those are unchanged.

**What this cost, recorded rather than smoothed over.** The first submission
enumerated and missed five paths. The second derived — better method — but
from an unverified premise, and shipped three trigger paths that would each
have fired a roughly ten-minute packaging job for no CVE-coverage reason. The
corrected anchor is **three patterns**, fewer than either previous attempt,
and requires no producer change beyond the `agf/VERSION` addition MR !2198
already made.

`status` returns to `pending-gate1` for the whole plan rather than only
A1, because one document carries one status and A2's acceptance criterion
also changed. A1's implementation from MR !2198 is otherwise unaffected and
is not discarded.

## Response to review 58kk

Independent review `20260828T224057Z-coder-tester-58kk.md` returned BLOCK
with four blocking findings against head `a42f10a8`. All four are
resolved here, and the revision is material rather than editorial:

| Finding | Resolution |
|---|---|
| **CT-V290-1** scope is not a terminal work order | v2.9.0 now commits A1+A2 only. Every B–E item has a terminal disposition. |
| **CT-V290-2** A1 input graph and cost contract incomplete | Complete path anchor enumerated; artifact reuse made binding; explicit runner-time budget replaces "measure before widening". |
| **CT-V290-3** negative test cannot leave the canonical MR red | Demonstration-pipeline shape specified for both A1 and A2; canonical acceptance pipeline stays green. |
| **CT-V290-4** Slice B does not solve cross-machine invisibility | B deferred to a separately gated shipment with "publish before acting" as its governing invariant. |

The review's finding CT-V290-4 demonstrated itself during this cycle:
records `7m4m`, `58kk` and `hrse` sat on unmerged branch
`coordination/v290-plan-coder-review` for roughly 18 hours, invisible to
`agov coord list` on `main`, while planner-architect worked. They were
published at merge commit `28861a11`. That is the fourth occurrence of
the same failure and the strongest available evidence that off-main
scanning is defense in depth, not the control.

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

Deliver an MR-stage build-and-scan job. Three contracts bind it.

**Path anchor — re-derived after review `k5dq` disproved the previous premise.**

The first submission enumerated eight paths and called them complete;
review `ngje` found five omissions, recorded as learning
`20260829T192500Z-4vn8`. The second submission replaced enumeration with a
derivation from `agov-source-fingerprint.sh`. Independent review `k5dq`
disproved that derivation **empirically**: it downloaded the canonical
`rust:tooling:preview` artifact for MR !2198's reviewed head from pipeline
`2802237900`, job `16185523532`, ran the packaged binary, and got

```json
{"version":"2.8.0","commit":"01b3268c...","source_fingerprint":"unknown"}
```

`Cross.toml:12-17` passes only `AGOV_BUILD_COMMIT`, `AGOV_BUILD_DIRTY` and
`AGOV_BUILD_SOURCE_DIRTY` into `cross build`, and the cross container has no
Git checkout, so `agov/build.rs:41-46` falls back to `"unknown"`. **The
fingerprint never reaches the packaged binary**, so it cannot govern what
changes it.

The failure shape is worth naming: the second submission derived from the
mechanism's *source* without checking that the mechanism *operates on the
canonical packaging path*. That is the same shape as the retracted `r3ad`
diagnosis. The correction is to derive from the shipped artifact and from
the gate's actual purpose.

**Which closure this anchor is.** `k5dq` names two definitions and warns
against mixing them. Byte identity is unattainable by any path anchor: the
packaged binary embeds `AGOV_BUILD_COMMIT=$CI_COMMIT_SHA` — proven by its own
output above — and `Containerfile.agov:18-23` embeds the same SHA as
`org.opencontainers.image.revision`, so **every** commit changes image bytes.
This plan therefore adopts **(a): a safe over-approximation of CVE-relevant
tracked inputs**, with `agf/VERSION` as the mandatory pre-release checkpoint.

**What is CVE-relevant.** The scan's subject is the image: its base layer and
the dependency set compiled into the binary. That surface is changed by the
base image pin (`Containerfile.agov`), the dependency graph (`Cargo.lock` and
every `Cargo.toml`), the toolchain pin (which lives in `.gitlab-ci.yml`; no
`rust-toolchain.toml` exists in this workspace), and the job definition.

Resulting anchor — three patterns:

```
.gitlab-ci.yml
agf/VERSION
tooling/rust/**/*
```

- `.gitlab-ci.yml` — the job itself, the Rust image pin, the packaging
  topology.
- `agf/VERSION` — the mandatory pre-release checkpoint `k5dq` requires, so the
  scan runs before a release even when nothing else changed.
- `tooling/rust/**/*` — the safe over-approximation. It already contains
  `Containerfile.agov`, `Cargo.lock`, every `Cargo.toml`, the OCI script,
  `release-targets.txt`, and all crate sources. Narrowing it to the dependency
  files alone would assert a *tighter* claim than the evidence supports, and
  `ci-trigger-scoping-rule.md` says to include when in doubt.

**Three paths are dropped, and this is a deliberate narrowing.**
`agf/schemas/**`, `agf/agents/MANIFEST.yaml` and `tooling/agov` entered the
previous anchor **solely** through the disproven fingerprint premise. None
expresses a dependency, base image, or toolchain, so none is CVE-relevant.
Retaining them would fire the roughly ten-minute canonical packaging job plus
the scan on a premise the artifact disproves — the specific waste `k5dq`
identified.

**Producer/consumer reconciliation, stated literally.** On `origin/main`,
`rust:tooling:preview` triggers on `tooling/rust/**/*`, `.gitlab-ci.yml`, and
two documentation paths (`.gitlab-ci.yml:1281-1285`). It does **not** trigger
on `agf/VERSION`. MR !2198 has already added `agf/VERSION` to it. With the
corrected three-pattern anchor that is the **only** producer change required:
the previous anchor's three external paths are gone, so no further widening is
needed in either job. Final state of both anchors, so a reviewer checks the
pair rather than one side:

| Job | Anchor after this change |
|---|---|
| consumer `agov:oci:mr-build-scan` | `.gitlab-ci.yml`, `agf/VERSION`, `tooling/rust/**/*` |
| producer `rust:tooling:preview` | `tooling/rust/**/*`, `.gitlab-ci.yml`, `agf/VERSION`, plus its two existing documentation paths |

**Two claims deliberately not made, and one contradiction corrected.**
`crates/agov/tests/validate.rs` embeds `agf/schemas/learning.schema.json` via
`include_str!`, but it is test-only and does not change the shipped binary;
`crates/agov-mcp/build.rs` does not build the `agov` binary the image copies.
`k5dq` confirms both judgments. Both files sit inside `tooling/rust/**/*` and
are covered by the anchor regardless, so neither is cited as the *reason* for
any path. This also removes the internal contradiction `k5dq` flagged, where
the previous derivation's step 2 claimed both `build.rs` files affect the
`agov` binary while a later paragraph correctly rejected that for
`agov-mcp/build.rs`.

**Artifact reuse is binding, not preferred.** The job MUST consume the
existing `rust:tooling:preview` archive rather than compile a second copy.
Measured from v2.8.0's own pipelines: tag job `agov:oci:release` ran
**53.276s**; MR job `rust:tooling:preview` ran **491.072s** and already
triggers on `tooling/rust/**/*` and `.gitlab-ci.yml`. Reuse therefore adds
roughly one minute to an already-running path; a second archive build would
add roughly eight and duplicate paid work.

**Runner-time budget.** No second Rust package build, under any
circumstances. If the scan alone exceeds two minutes on a representative
MR, stop and investigate before widening. This replaces the previous
"measure the cost before widening" sentence, which was not a contract.
The budget matters directly: CI quota exhaustion halted a release on
2026-08-28.

**Build/push separation.** `build_image()` currently calls `check_tag()`,
so the release path is not reusable as-is where `CI_COMMIT_TAG` is absent.
The implementation must separate an unpublished build-and-scan path from
the tag/push/provenance path. **The MR path must never push to, overwrite,
or otherwise touch a release namespace.**

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

**Amended by governor decision `rm3k` (2026-08-29).** The original wording
required deriving both names from one source *so the divergence is
impossible*. For v2.9.0 that is amended to: derive the published reference
from one source and make a divergent reference mechanically **detectable
and rejectable**, with adoption of that check by the release path tracked
separately.

The reason, found while implementing: the pre-tag namespace check is
operator-run, and
`agf/rules/workflow/release-via-merge-request-rule.md` requires it **only**
inside the maintenance-line and frozen-scope exceptions. Its `## What the
MR MUST verify` checklist (lines 60–72), which governs the standard
workflow every release now uses, contains no namespace item at all — so
v2.8.0's botched check was one no rule required on its path, and there is
currently no standard-path check for a name to diverge *from*. Making
divergence impossible there means first creating that obligation, which is
a new normative requirement triggering
`documentation-rule-projection-rule.md`'s six-row screen. That is issue
#120's work and its own Gate 1, not an absorbable amendment.

Within this amendment, A2 still derives the checked and published names
from a single source.

**A3. Negative-test proof shape (applies to both A1 and A2).**
`always/gate-self-test-rule.md` requires a demonstration that each gate
fires on known-bad input, with a reviewer-visible transcript. A committed
vulnerable base — or a committed mismatched spelling — would correctly
turn the canonical acceptance pipeline red, and a red exact-head pipeline
cannot be the Gate 2 evidence. Both requirements are satisfiable only if
the proof runs somewhere other than the canonical acceptance run.

Required shape: a reproducible harness or a separate, transient
demonstration pipeline that
(a) proves a knowingly vulnerable base pin exits non-zero in the A1 job,
(b) proves a deliberately mismatched OCI reference spelling fails the A2
    pre-tag check,
(c) preserves both transcripts for the reviewer, and
(d) leaves the canonical exact-head acceptance pipeline green.

The transcripts, and the exact means by which each was produced, are Gate
2 evidence. "Tested locally" does not satisfy (c).

## Terminal dispositions — everything not committed to v2.9.0

Per CT-V290-1, no item below may enter v2.9.0. Each has one terminal
disposition. "Later shipment" means a separately gated Gate 1 plan of its
own; naming a version here is a candidate, not an approval.

| Item | Disposition |
|---|---|
| B — coordination visibility | **Later shipment**, redesigned (see below) |
| B — duplicate transcription | **Later shipment**, folded into B |
| C1 — pinned drifting target | **Later shipment** with `#43` |
| C2 — empty pipelines block merges | **Later shipment** |
| C3 — fast-track eligibility check | **Excluded** (see below) |
| D — `#32`, `#37` closure | **Non-blocking closeout**, independent of any release |
| D — `#36`, `#31`, `#75`, `#53`, `#101`/`#106`/`#102`, `#43` | **Later shipment** |
| E — `#109` self-hosted runners | **Named deferred risk**, governor to milestone |

### B — coordination visibility (deferred, and redesigned)

Retained as the highest-frequency failure — now five occurrences,
including this plan's own review sitting unseen for 18 hours. But the
mechanism previously proposed here was wrong, and CT-V290-4 is right about
why: extending `agov handoff next` and `agov coord pending` to inspect
off-main refs can surface *fetched remote* refs, and cannot discover a
record that exists only in another machine's local checkout. Codex and
Claude run on separate hosts. The cited failures are exactly that case, so
the proposed control would not have prevented any of them.

Governing invariant for the later shipment: **publish before acting** — a
record an actor relies on must first be validated and visible to its
recipient. Off-main scanning becomes defense in depth beneath that
invariant, not the control itself.

Duplicate transcription (`2h4k` / `2h2s`, two binding governor records for
one decision, authored by two assistants that could not see each other)
folds into the same shipment: it is the same root cause, and "publish
before acting" is also its candidate mitigation. The sole-transcriber
alternative is recorded as considered and not preferred — it adds a
bottleneck without making any record visible.

### C3 — excluded, with reason

`simplicity-rule.md` already excludes security boundaries from the
fast-track waiver. Planner-architect claimed one anyway for `85540db4`;
`j37k` upheld the finding and recorded it rather than correcting history.
The rule was not defective — it was not followed. A mechanical check on
waiver-claiming commits touching security-relevant paths is not
proportionate to one occurrence with a known cause and an existing written
rule. Excluded from v2.9.0 and from the later-shipment list. Revisit only
if it recurs; the recurrence itself would be the missing evidence.

### Owed learnings

Two learning records are owed and are not release scope: `hgte`'s
reconstruction reverse-consumer closure, and the C3 fast-track waiver
error. Both are obligations on their authors regardless of what ships.

## Original slices, retained for the record

The three sections below are the pre-review scope. They are retained
unchanged so the review trail stays legible, and are **not** proposed for
v2.9.0 — the table above governs.

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

These cover the committed scope — A1 and A2 — and nothing else. Criteria
for deferred items move with them to their own plans.

1. A Containerfile change with a knowingly vulnerable base fails at **MR
   stage**, not at tag stage.
2. A deliberately mismatched OCI reference spelling fails the pre-tag
   check.
3. Both negative-test transcripts are reviewer-visible and were produced
   without leaving the canonical exact-head acceptance pipeline red, per
   A3(a)–(d).
4. Both job anchors match the two-row table in A1 **literally** — consumer
   `.gitlab-ci.yml`, `agf/VERSION`, `tooling/rust/**/*`; producer those three
   plus its two existing documentation paths — so no input can start the scan
   job with its dependency unbuilt, and the three paths dropped after `k5dq`
   are absent from both. Verified by inspection of the merged
   `.gitlab-ci.yml`, both job rules read together.
5. The A1 job consumes the `rust:tooling:preview` archive; no second Rust
   package build exists in the MR path.
6. Measured incremental MR runner time is recorded in the Gate 2 evidence
   and is within the A1 budget, or the excess is investigated and
   explained before merge.
7. The MR build-and-scan path performs no push to any release namespace,
   verified by inspection of the separated script paths.
8. The checked and published OCI references derive from one source, and a
   divergent reference is rejected with a clear diagnostic — per the `rm3k`
   amendment, detectable-and-rejectable rather than impossible.
9. No governed rule is weakened. Every check added is additive.

## Risks

**Scope creep — the risk this revision exists to close.** The pre-review
plan named eight defects across four slices and asked Gate 1 to authorize
all of them while its own risk section contemplated cutting them. That is
not a stable work order, and it is how v2.7.0 failed. v2.9.0 now commits
two items; everything else has a terminal disposition. If A1 or A2 grows
during implementation, that is a signal to stop and return to Gate 1, not
to absorb it.

**Runner cost.** A1 adds an image build-and-scan to MR pipelines eight
days after CI quota exhaustion halted a release mid-flight. The binding
reuse contract and the two-minute scan budget in A1 are the mitigation,
and criterion 6 makes the measurement a Gate 2 requirement rather than a
post-hoc observation. The measured baseline — 53.276s tag job, 491.072s
archive job already running — is what makes reuse the cheap path.

**Residual, accepted:** deferring B leaves cross-machine coordination
invisibility unfixed for at least one more cycle. It has now occurred five
times, once with real consequence (MR !2174 merged over an unseen BLOCK).
The reason for accepting it is that the previously proposed mechanism
would not have prevented any of the five, so shipping it would have
produced the appearance of a fix without the substance. Deferring a
correct fix is better than shipping an ineffective one, but the exposure
is real and continues.
