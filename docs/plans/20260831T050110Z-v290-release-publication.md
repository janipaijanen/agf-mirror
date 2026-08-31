---
id: 20260831T050110Z-v290-release-publication
status: pending-gate1
topic: v290-release-publication
candidate_release: v2.9.0
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      The committed scope is A1 (pre-tag container CVE coverage) and A2
      (single-sourced OCI reference), both security-relevant release-path
      controls already approved in
      docs/plans/20260828T194721Z-v290-release-integrity-and-tracker-truth.md
      and consulted there. This plan publishes them; it introduces no new
      credential, network, or data-flow surface of its own. If publication
      reveals a CVE, the release stops rather than proceeding.
  - role: contribution-auditor
    status: waived
    reason: >-
      No contribution-auditor disposition exists for this plan and the
      governed enum has no pending state. Recording waived rather than
      consulted so the metadata does not assert an audit that has not
      happened. An independent coder-tester review is requested instead; the
      governor may hold Gate 1 for an auditor pass.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Publish v2.9.0

**Candidate release: v2.9.0.** First plan under decision `r2kb`, which requires
every plan to name one.

## What this plan is, and what it is not

This is a **release-publication plan**. It ships what is already built. It
proposes no new capability, no governance artifact, and no roadmap document.

That framing is the point. `nsne`'s v2.9 cluster was "tracker truth and release
cost", and the governor's direction is now explicit: *we want release version,
not internally baked gate documents as we have done for the last two months.*

**It is subordinate to an existing approved plan and does not compete with it.**
`docs/plans/20260828T194721Z-v290-release-integrity-and-tracker-truth.md`
(`status: approved`, 2026-08-30T08:36:08Z) authorizes **A1 and A2 only** as the
committed v2.9.0 work order and states that nothing else in it is proposed for
v2.9.0. This plan does not widen that scope by one item. Review `1mqg` VRW-1
found the previous roadmap plan created competing release authority against
that decision; this plan exists partly to avoid repeating it.

## The finding that shapes this plan

**The committed v2.9.0 scope appears to be already implemented and merged.**

| Item | Evidence found | Strength |
|---|---|---|
| **A1** — MR-stage container build-and-scan | `agov:oci:mr-build-scan` exists in `.gitlab-ci.yml:665`, and `:624-628` documents it as "(v2.9.0 A1)"; the job ran and passed in pipeline `2803731939` | **Existence and execution: strong. Gating: unproven** — a job passing proves it ran on a clean base, not that a vulnerable base would fail it |
| **A2** — single-sourced OCI reference | Selftest **run and mutation-tested locally** — see transcript below | **Strong: the check is proven to bind** |

**Deliberately stated as incomplete evidence.** Review `1mqg` VRW-4 found that
three `--help` invocations had been presented as proof that an issue was
shipped, when they proved only registration. That error is not repeated here:
what is above is *where the implementation is*, not proof that A1 and A2 meet
their nine acceptance criteria.

Gate 2 for this release requires binding each of those criteria to its actual
evidence — in particular criteria 1 and 2, which demand **negative-test
transcripts**.

### Criterion 2 is discharged here

`agov-oci-image.sh`'s A2 proofs need no podman and no CI, so they were run
directly. A passing test is not proof that a test binds, so the check was
mutated as well:

```
$ sh tooling/rust/scripts/agov-oci-image.sh selftest
exit=0
selftest OK: canonical targets resolve; Windows, unknown, and malformed
inputs are rejected; OCI reference is single-sourced and mismatched
spellings are rejected

# mutation: verify_ref's comparison replaced with `if false`
$ sh tooling/rust/scripts/agov-oci-image.sh selftest
exit=1
v-prefixed reference was not rejected (the v2.8.0 defect)
mismatched repository component was not rejected
selftest FAILED
```

The mutation names the v2.8.0 defect specifically, which is the incident A2
exists to prevent. Mutation reverted; working tree verified clean afterwards.

This satisfies criterion 2, and criterion 3 for A2's half: the negative
transcript is reviewer-visible and reproducible, and it is produced by a
selftest that **passes** in the acceptance pipeline while internally asserting
the negatives — so demonstrating the gate fires does not require leaving the
canonical pipeline red.

### Criterion 1 is not discharged

A1's negative test — a knowingly vulnerable base failing at MR stage — needs a
container build and therefore CI. It cannot be produced while the quota is
exhausted. If that transcript was produced during A1 implementation, this plan
cites it at Gate 2; if not, it is produced before the tag.

## Blocked precondition, stated first

**The project's CI minutes quota is exhausted.** Verified: `main`'s pipeline
`2804246543` and jobs `16195798837/38/39` all report
`failure_reason: ci_quota_exceeded`, and `only_allow_merge_if_pipeline_succeeds`
is enabled, so merge attempts return HTTP 405.

No release can proceed. The tag pipeline *is* the release mechanism — it builds
archives, checksums, provenance, the package and the OCI image. A release
attempted without CI does not produce a partial release; it produces an
immutable tag with nothing behind it, which is exactly how v2.7.0-rc.1 was
burned.

**Restoring CI capacity is therefore a precondition of this plan, not a step
inside it.**

## Steps

Per `agf/rules/workflow/release-via-merge-request-rule.md`, standard workflow —
no maintenance-line or reconstruction exception applies, since v2.9.0 ships
from current `main`.

1. **Bind A1/A2 acceptance.** For each of the nine criteria, cite the evidence
   or produce it. Criteria 1 and 2 require reviewer-visible negative-test
   transcripts. Criterion 6 requires the measured incremental MR runner time.
2. **Confirm `main` is releasable.** Green pipeline on the exact release base,
   with the three stranded records (`jsjk`, the plan metadata correction, and
   the outstanding review responses) landed first so the audit trail is whole.
3. **Release branch.** `release-2.9.0` from `main`; commit only `agf/VERSION`
   2.8.0 → 2.9.0 and the `CHANGELOG.md` / `agf/CHANGELOG.md` entries. No other
   file.
4. **MR to `main`**, diff limited to those files, exact-head pipeline green,
   binding governor Gate 2.
5. **Tag `v2.9.0` on the merge commit and push immediately.** Per the rule,
   the merged MR is the authorization; this step needs no further confirmation.
6. **Verify publication anonymously**: Release object, generic package, OCI
   digest, checksums — on the downloaded bytes, not on the build's own claims.

## CHANGELOG scope

`main` carries 48 non-coordination commits since v2.8.0. The entry states what
changed for an adopter, which is narrower than the commit count: the
gate-change mechanical enforcement work (`agov gate change-check`, the gate
registry, the shipped schema and example), the A1/A2 release-path controls, and
the changed-files fail-closed contract. Plans, learnings and coordination
records are repository history, not adopter-visible change, and are not listed
individually.

## Explicitly out of scope

- Any item `20260828T194721Z` dispositioned as deferred, excluded, or
  separately gated.
- The roadmap-owner work (`docs/roadmap.md`), which review `1mqg` showed needs
  reframing and is a separate plan.
- Sub-case C (`nqeg`), the visibility-correlation implementation (approved at
  Gate 1 but not implemented), and the calm-visual pilot. None is in the
  committed v2.9.0 work order.
- The `release-v2.7.0-rc.2` disposition, which remains an open governor
  decision and does not block v2.9.0 — v2.8.0 already shipped past that line.

## Risks

- **A1/A2 acceptance turns out incomplete.** Then v2.9.0 waits, or scope is
  cut by governor decision. The release does not ship on assumed evidence —
  that assumption is what this project has twice paid for.
- **Quota is restored but capacity is thin.** The tag pipeline is the one
  pipeline that must not fail midway. Confirm sufficient capacity before
  tagging, not after.
- **Stranded records.** Publishing a release while `jsjk` and the plan-metadata
  correction sit unmerged would tag a tree whose governance trail is knowingly
  incomplete. Step 2 exists to prevent that.

## Rollback

Before the tag: abandon the release branch; nothing is published. After the
tag: `immutable-tags-rule.md` applies without exception — the tag stays, and a
defect ships as v2.9.1. There is no path that moves or deletes a pushed tag.
