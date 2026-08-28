---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260828T080705Z-governor-reviewer-qhk8
timestamp: 2026-08-28T08:07:05Z
model: human-directed-claude-sonnet-5
immutable: true
references-to:
  - 20260828T070826Z-governor-reviewer-m8ra.md
  - 20260828T070833Z-planner-architect-qrjq.md
  - 20260828T051652Z-governor-reviewer-qt1a.md
topic: v270-rc2-conditional-standing-authorization
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Standing conditional authorization: v2.7.0-rc.2 through publication

The governor-reviewer's in-session instruction was "gate 2 approved,
publication authorized, proceed as proposed", with the clear intent that
`-rc.2` should reach publication without further round-trips.

**That intent is granted here in the only form that can be honoured.**

## Why this is not recorded as a Gate 2 approval

At the time of the instruction, `release-v2.7.0-rc.2` existed but was
**empty** — resolving to `25e6d752b4a7f6964fecec2d5daebd0a3d8c7660`,
byte-identical to the target. No `74df7f75` replay, no release metadata,
no release MR, no pipeline, and no OCI scan existed.

An exact-head Gate 2 record names a source SHA, a source tree OID, a
target SHA, and a pipeline id — as `s3tg` did for `-rc.1`. None of those
existed to name. Publishing an approval for an artifact that does not yet
exist is a speculative approval record, which
`coordination-publication-rule.md` prohibits outright.

The concrete harm, not the procedural one: tagging that tree as it stood
would have published an `-rc.2` **without the openssl fix**, failing the
identical Trivy gate and burning a third RC number for nothing.

## What is authorized: standing conditional authority

Coder-tester is authorized to proceed **all the way through construction,
exact-head Gate 2 merge, tagging, publication, and hold start**, with no
further governor round-trip, **provided every condition below is
satisfied and recorded as evidence**. This is a single standing
authorization, not a waiver of any check.

1. `-rc.2` is constructed on `release-v2.7.0-rc.2` from target
   `reconstruct/v2.7` at `25e6d752b4a7f6964fecec2d5daebd0a3d8c7660`; the
   target is never constructed on and its protection object is unchanged
   at all three checkpoints.
2. MR !2174's merge commit `74df7f75cdd9a2ce64dc83047f837a5312f52b86` is
   replayed with a proven byte-identical first-parent patch, exactly as
   `960124e1` and `acaec947` were proven.
3. Inventory Sections A, B and C are regenerated from scratch and
   published with their hashes. The `-rc.1` figures — 45 paths,
   `58a2757b…`, `8a865ac3…` — are superseded and must not be carried
   forward.
4. Checkpoint 1 (pre-metadata, bound to its exact recorded SHA) and
   Checkpoint 2 (final path-set equality, bounded per-path edits) both
   pass.
5. Release metadata reads exactly `2.7.0-rc.2` across the same governed
   surface list, including both changelog headings, and the `#86`
   seven-subcommand wording is preserved.
6. **The OCI image is built from the exact candidate head and Trivy-
   scanned locally, clean of HIGH and CRITICAL findings, with the result
   recorded as Gate 2 evidence.** This is the condition that
   distinguishes `-rc.2` from `-rc.1`. `agov:oci:release` is
   `if: $CI_COMMIT_TAG` only, so this class of finding cannot otherwise
   surface before the tag is immutable.
7. The exact-head MR pipeline passes with zero unresolved discussions and
   an empty `closes_issues` response.
8. Immediately pre-merge: both branch heads re-read and matching.
   Immediately post-merge: first parent = reviewed target, second parent
   = reviewed source, merge tree = reviewed source tree.
9. A fresh immediately-pre-tag re-check finds all four namespaces absent
   — Git tag, GitLab Release, generic package across every status fully
   paginated, and the OCI tag with the repository id resolved fresh — and
   the exact protected-tag rule present.

If all nine hold: create the annotated `v2.7.0-rc.2` tag once on the
`reconstruct/v2.7` merge commit, push once, require the tag pipeline,
run `agov dist post-publication-check` against an anonymous download,
bind all four immutable identities, complete the fresh-clone checks
including the new-session `/context` check that closes `7bkm`, and start
the 48-hour hold. Record a completion with the hold's start timestamp.

## Fail-closed

Any condition unmet — a proof mismatch, a namespace non-absence, an API
ambiguity, a pipeline failure, or **any HIGH or CRITICAL scan finding** —
stops the release and returns to the governor. Do not weaken, waive, or
retry the Trivy gate. Do not retry a failed paid job to seek green
status. Do not touch the immutable `v2.7.0-rc.1` tag or its artifacts; a
defect after the `-rc.2` tag exists is corrected by `-rc.3`, never by
moving a tag.

## Not authorized

Stable `v2.7.0` promotion remains out of scope and still requires its own
binding candidate record and Gate 1 plan after an accepted candidate
completes its hold. Product scope remains exactly #86, #87, #88, #90.
