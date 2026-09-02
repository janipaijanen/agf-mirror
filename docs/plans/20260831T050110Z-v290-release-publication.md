---
id: 20260831T050110Z-v290-release-publication
status: approved
gate1_approved_by: governor-reviewer
gate1_approved_at: 2026-09-02T08:44:39Z
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
3. **Release branch.** `release-2.9.0` from `main`, carrying the exact
   release-metadata allowlist below and nothing else.

   **Two prior revisions of this allowlist were wrong (`thmr` V29P-1, then
   `1c75fb51` V29P-1-R2).** The first named three files. The second added
   eight but still omitted fourteen role definitions, `agf/ADOPTING.md`, and
   the live release-status blocks — and, worse, *misclassified* one of them as
   historical. Both revisions asserted exhaustiveness from a hand-built list.
   This one derives it.

   **Audit command** — run at the frozen release base; its output is Gate 2
   evidence:

   ```sh
   git grep -n -F '2.8.0' -- \
     ':!docs/coordination' ':!docs/plans' ':!docs/reviews' \
     ':!CHANGELOG.md' ':!agf/CHANGELOG.md' ':!agf/learnings'
   ```

   The exclusions are append-only history that must never be rewritten. At the
   reviewed base this returns **47 occurrences across 26 files**.

   **Group A — authored current-version cells. All must become 2.9.0 (32).**

   | Path | Count | Field |
   |---|---|---|
   | `agf/VERSION` | 1 | the version |
   | `AGENTS.md:1` | 1 | `<!-- AGF: … -->`; **validator-enforced** against `agf/VERSION` |
   | `agf/templates/AGENTS.md:3` | 1 | same marker, shipped template |
   | `agf/templates/learning-record.yaml:8` | 1 | `framework_version` |
   | `agf/agents/MANIFEST.yaml:11` | 1 | `Framework version:` |
   | **`agf/agents/*.yaml` role definitions** | **14** | `framework_version: "2.8.0"` — one per shipped role |
   | `agf/ADOPTING.md:333` | 1 | adopter example `framework_version` |
   | `tooling/rust/Cargo.toml` | 4 | workspace version + three path-dep constraints |
   | `tooling/rust/Cargo.lock` | 5 | workspace packages; regenerate, never hand-edit |
   | `agf/docs/governance/manifest.md` | 3 | lines 11, 38, 76 — current-version cells |

   **Group B — live release-status prose. Must be rewritten, not version-bumped (8).**

   | Path | Count | Why it cannot stand |
   |---|---|---|
   | `agf/docs/governance/manifest.md:18-21` | 2 | says v2.8.0 "is being prepared" and directs adopters to install v2.6.1. **The previous revision classified this as historical and required it byte-identical — that was wrong.** It is a live claim that becomes false the moment v2.9.0 publishes. |
   | `agf/docs/site/roadmap.md` | **6** — lines **8, 10, 11, 50, 69, 90** | publicly states v2.8.0 is "in construction" and v2.6.1 is the latest fully published stable release |

   Group B is the finding worth naming: **a version-bump substitution would
   leave both files internally consistent and factually false.** They need
   rewritten status text, and their new content is adopter-facing, so it is
   Gate 2 review material rather than mechanical.

   **Group C — genuinely historical. Must remain byte-identical (7).**

   | Path | Count | Nature |
   |---|---|---|
   | `tooling/rust/scripts/agov-oci-image.sh` | 6 | incident references — "the exact v2.8.0 defect", and the selftest's own assertion string `v-prefixed reference was not rejected (the v2.8.0 defect)`. **Changing these would break the A2 selftest that discharges criterion 2.** |
   | `.gitlab-ci.yml:653` | 1 | "Measured on v2.8.0's own pipelines" — a measurement note |

   **32 + 8 + 7 = 47.**

   The previous revision said 33 + 7 + 7 and also totalled 47 — because two
   one-count errors cancelled (Group A over by one, Group B under by one,
   from misreading my own audit's `6 agf/docs/site/roadmap.md` line as 5).
   Review `V29P-1-R3` caught it. **A total that reconciles is not proof the
   categories do**, and here the coincidence would have left roadmap line 90
   outside the edit set for an implementer working from the table.

   Because of that, acceptance below cross-checks **per-file counts, category
   counts, and total independently** — not the total alone.

   **Acceptance — exhaustiveness proven by three independent agreements.**
   Before the edit, re-run the audit at the frozen base and require all three
   to agree: **per-file counts** match the tables; **category counts** are
   32 / 8 / 7; and the **total** is 47. Any one disagreeing stops the release,
   including when the total happens to be right — that is the exact failure
   mode `V29P-1-R3` found.

   After the edit, re-run it again: every surviving `2.8.0` must be in Group C,
   matched line-for-line. A survivor outside Group C means the allowlist was
   incomplete for a fourth time. Additionally: `agov validate
   assistant_loader` passes, the built binary reports 2.9.0, and the A2
   selftest still passes.

4. **MR to `main`**, diff limited to those files, exact-head pipeline green,
   binding governor Gate 2.
5. **Fail-closed pre-tag proof (review `thmr` V29P-3).** Immediately before the
   irreversible tag, and stopping the release on any non-absence or ambiguity:

   a. Authoritative **absence** reads for the Git tag, the GitLab Release, the
      generic package version across every status with full pagination, and
      **the OCI reference as derived by `agov-oci-image.sh image-ref`** — not a
      hand-written guess at the namespace. *This specific omission is what
      went wrong at v2.8.0: the pre-tag check queried `v2.8.0` while
      publication used `agov:$AGOV_VERSION`. The first revision of this plan
      cited that incident as motivation and then left the derived-namespace
      read out of its own steps.*
   b. **Protected-tag rule with an explicit expected policy (`V29P-3-R2`).**
      "Matches the intended tag" was underspecified — a rule named `v2.9.0`
      permitting broad creation would have satisfied those words while
      supplying no protection. The required configuration, following the
      established v2.8.0 shape, is: **exact tag `v2.9.0`** (not a wildcard),
      with **creation restricted to Maintainers, access level 40**. Prove it
      by authoritative API read **before the tag exists**, and record the
      returned object. Any other policy needs an explicit governor
      disposition, not an implementer's judgment.
   c. Frozen source and target: re-read both heads and require them to equal
      the reviewed SHAs; verify the merge commit's parents and tree against the
      reviewed values. Any drift stops here, before the tag.

6. **Tag `v2.9.0` on the proven merge commit and push immediately.** The merged
   MR is the authorization and this needs no further confirmation — but only
   once step 5 has passed in full.

7. **Post-publication binding and anonymous verification.** Bind the tag object
   and its peeled commit to every archive, checksum, provenance record, package
   identity and OCI digest; re-read each namespace; then verify **on
   anonymously downloaded bytes**, not on the build's own claims.

## Shipment inventory (review `thmr` V29P-2)

The first revision said the committed work order is A1/A2 while its changelog
scope also promised gate-change enforcement — and used "48 non-coordination
commits" as if a count were an inventory. Both are wrong in the same way: **a
standard release tags the whole selected tree.** Merged adopter-visible work
does not become repository history by being called that, and a count changes
whenever `main` advances.

At the **frozen release base** — recorded by SHA, not by "current main" — every
tag-to-base non-coordination commit is enumerated and classified as exactly
one of:

- **adopter-visible included content** — bound to its plan, its Gate 2 or merge
  evidence, and its changelog entry;
- **internal governance or history** — plans, learnings, coordination records,
  review artifacts; present in the tree, absent from the changelog;
- **expressly excluded** — absent from the frozen tree.

The release *objective* stays A1/A2. The release *contents* are whatever the
frozen tree holds, stated honestly.

**If the target advances before merge, the inventory is invalidated** and
re-derived at the new frozen base, together with a fresh pipeline and a fresh
exact-head Gate 2 — the same drift discipline the release rule applies to
everything else.

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
