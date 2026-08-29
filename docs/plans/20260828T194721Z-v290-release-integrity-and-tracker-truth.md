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

**Path anchor — the derived input closure, and how it was derived.**

The first submission of this plan enumerated eight paths and called them
complete. Independent review `ngje` found five omissions, and the
generalizable failure is recorded as learning
`20260829T192500Z-4vn8`: fixing one instance of a defect class was
mistaken for covering the class. Governor decision `rm3k` returned A1 to
Gate 1 for this reason.

The anchor below is therefore **derived from the artifact's input closure
rather than enumerated from memory.** Derivation, reproducible by any
reviewer:

1. The image is `Containerfile.agov` plus one file: the `agov` binary
   extracted from `dist/agov-$VERSION-$SLUG.tar.gz`.
2. That binary's bytes depend on every Rust source, both `build.rs` files
   (`crates/agov/`, `crates/agov-mcp/`), every crate-embedded asset
   reached by `include_str!`, and the workspace manifests and lockfile.
3. `crates/agov/build.rs:51` invokes
   `tooling/rust/scripts/agov-source-fingerprint.sh` and writes its output
   into the binary as `AGOV_SOURCE_FINGERPRINT`.
4. That script (lines 29–33) hashes `tooling/agov`, `tooling/rust`,
   `agf/schemas` and `agf/agents/MANIFEST.yaml`. **Any tracked file under
   those paths changes the binary's bytes**, which is what pulls three
   paths outside `tooling/rust` into the closure.
5. `agf/VERSION` names the archive and is read into `AGOV_VERSION`.
6. `.gitlab-ci.yml` defines the job itself.

Resulting anchor — six patterns, not eight paths:

```
.gitlab-ci.yml
agf/VERSION
agf/schemas/**
agf/agents/MANIFEST.yaml
tooling/agov
tooling/rust/**/*
```

`tooling/rust/**/*` deliberately subsumes the Containerfile, the OCI
script, `release-targets.txt`, `Cargo.lock`, every `Cargo.toml`, all crate
sources, both `build.rs` files, crate-embedded schemas, and the fingerprint
script itself. Enumerating those individually is what produced the
omissions the first time; the broader pattern is both simpler and
complete, and `ci-trigger-scoping-rule.md` says to include when in doubt.

**The producer must be widened with the consumer, in the same change.**
`rust:tooling:preview` currently triggers on `tooling/rust/**/*`,
`.gitlab-ci.yml`, and two docs paths. It does **not** trigger on
`tooling/agov`, `agf/schemas`, or `agf/agents/MANIFEST.yaml`. Widening the
scan job alone would leave its `needs` dependency unbuilt for exactly those
inputs — reproducing the `agf/VERSION` defect that was found and fixed once
during the first submission. Producer and consumer anchors are reconciled
as one change, and both are stated here so a reviewer checks the pair.

**Two claims deliberately not made.** `crates/agov/tests/validate.rs`
embeds `agf/schemas/learning.schema.json` via `include_str!`, but it is a
test and does not change the shipped binary; `agf/schemas` is in the
closure through the fingerprint script instead. Likewise
`crates/agov-mcp/build.rs` does not affect the `agov` binary the image
copies; it is inside `tooling/rust/**/*` and hashed by the fingerprint
regardless. Both are recorded so a reviewer can see which justifications
were checked and rejected, not only which were accepted.

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
4. The A1 anchor matches the derived closure in A1 exactly, **and**
   `rust:tooling:preview`'s anchor covers every path the scan job triggers
   on, so no input can start the scan job with its dependency unbuilt.
   Verified by inspection of the merged `.gitlab-ci.yml`, both job rules
   read together.
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
