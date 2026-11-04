---
kind: decision-record
from: governor-reviewer
to: all
id: 20261104T052528Z-governor-reviewer-2st2
timestamp: 2026-11-04T05:25:28Z
model: claude
immutable: true
references-to:
  - 20261103T211221Z-planner-architect-bks2.md
  - 20261101T195927Z-v300-release-publication.md
  - 20261101T191657Z-governor-reviewer-pr6j.md
  - 20261102T190229Z-planner-architect-rqeb.md
  - 20261103T202753Z-governor-reviewer-rkjm.md
topic: v300-gate2-date-change-and-tag-authorization-bridge-record
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# 3.0.0: Gate 2, release date 2026-11-04 and tag authorization (bridge record, provisional)

**Provenance.** The Human Governor gave these decisions in a live session on 2026-11-04 and
told the Claude session (model: claude) to file them. The `from` field names the decision-maker;
the session is the filer. This is a bridge record: provisional until the governor confirms it in a
later record, and correctable by a later record. Releases, tags, versions and dates are outside
`srra`; the authority for the actions below is this human instruction, not the delegation.

## What the governor said

> gate 2 approved. if no objections, bake the release

The session raised one objection: the changelog heading read 2026-11-03 but the date was
2026-11-04, and a tag is immutable. The governor chose "Change to 2026-11-04".

## Decision as recorded

1. **Release date:** 2026-11-04. The changelog headings become `## [3.0.0] — 2026-11-04` (and the
   hyphen form in the root file).
2. **Gate 2 for !3502** is approved for the exact head
   `bd5efb53802fbe6c18fceecd53693807b6c9d87a`. The Gate 2 words were said while the head was
   `fc5300ad`; the date change then made `bd5efb53`. The delta is exactly two changelog heading
   lines (`git diff fc5300ad bd5efb53`: 2 files, 2 insertions, 2 deletions). The session reads the
   governor's approval, given together with the choice to change the date, as covering that delta.
   If the governor meant otherwise, a later record corrects this before the action below runs.
3. **Authorized actions, in this order, only if every condition holds:**
   - merge !3502 without squash at the exact SHA `bd5efb53` (merge pipeline must be green at that
     head, head unchanged);
   - create a protected-tag rule for exactly `v3.0.0` with creation at access level 40, and re-read it;
   - run `agov release namespace-check --phase pre-push` on the merge commit, all four namespaces
     ABSENT;
   - create one annotated tag `v3.0.0` on the merge commit and push it once; never move or
     recreate it;
   - verify the tag pipeline, the GitLab Release, the generic package, the OCI image, checksums and
     provenance with anonymous downloads.
4. **Stop conditions.** Any drift in head or target, a non-green pipeline, a non-ABSENT or ambiguous
   namespace result, a protected-tag rule that does not match, or a merge whose parents or tree
   differ from the reviewed ones stops the release and returns it to the governor.

## State at authorization

- Reviews: metadata and security CLEAR (`9051f860`, `d4ca4e8f`); changelog content and delta
  reviews CLEAR; sweep blocking item fixed by !3503. Details in `bks2`.
- Namespace preflight at `bd5efb53` against main `de8e6ba1`: all four ABSENT, 2026-11-04T05:25:07Z.
- Not decided here: the BREAKING classification of the role guidance (the changelog keeps its single
  BREAKING item) and confirmation of the earlier deputized records `g8jp`, `egbp`, `rkjm`, `1jkm`,
  `rqeb`. They stay provisional. The latest-published fields are updated in a separate
  status-sync MR after publication.
