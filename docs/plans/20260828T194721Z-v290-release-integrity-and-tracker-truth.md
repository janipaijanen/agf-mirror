---
id: v290-release-integrity-and-tracker-truth
status: approved
gate1_approved_by: governor-reviewer
gate1_approved_at: "2026-08-29T17:06:49Z"
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
      four findings; see "Response to review 58kk" below. Continuing the
      practice used throughout the v2.7.0/v2.8.0 cycle, where five
      consecutive independent reviews each found real defects in
      self-authored release artifacts — this was the fifth.
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

## Decision requested — APPROVED

Binding Gate 1 approval granted by governor-reviewer at
2026-08-29T17:06:49Z, recorded in
`docs/coordination/20260829T170649Z-governor-reviewer-h3nb.md`, against
exact head `05c9d07981d88627d0b9a46148d1d7358b0c4ffb`. Approved scope is
**A1 and A2 only**, with A3 as their shared negative-test obligation, and
the terminal-disposition table below as the governing disposition for
everything else. The five implementation conditions in `h3nb` §1 are
binding, including: if A1 or A2 grows during implementation, stop and
return to Gate 1 rather than absorbing it.

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

**Path anchor — complete, not "at minimum".** The trigger must cover every
input that can change image content:

```
.gitlab-ci.yml
agf/VERSION
tooling/rust/Containerfile.agov
tooling/rust/scripts/agov-oci-image.sh
tooling/rust/release-targets.txt
tooling/rust/Cargo.lock
tooling/rust/**/Cargo.toml          # every workspace manifest
tooling/rust/crates/**/src/**       # sources producing the embedded binary
```

An incomplete anchor is the defect this item exists to fix; `045e514c`
already demonstrated that a missing anchor silently produces no coverage.
Per `ci-trigger-scoping-rule.md`, when in doubt the path is included.

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

Derive the checked name and the published name from one source so the
divergence is impossible rather than merely detected.

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
4. The A1 path anchor covers every input enumerated in A1, verified by
   inspection of the merged `.gitlab-ci.yml`.
5. The A1 job consumes the `rust:tooling:preview` archive; no second Rust
   package build exists in the MR path.
6. Measured incremental MR runner time is recorded in the Gate 2 evidence
   and is within the A1 budget, or the excess is investigated and
   explained before merge.
7. The MR build-and-scan path performs no push to any release namespace,
   verified by inspection of the separated script paths.
8. The checked and published OCI references derive from one source; the
   divergence is impossible by construction, not merely detected.
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
