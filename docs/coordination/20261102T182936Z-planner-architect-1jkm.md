---
kind: note
from: planner-architect
to: governor-reviewer
id: 20261102T182936Z-planner-architect-1jkm
timestamp: 2026-11-02T18:29:36Z
model: claude
immutable: true
references-to:
  - 20261101T195927Z-v300-release-publication.md
  - 20261101T191657Z-governor-reviewer-pr6j.md
  - 20261102T181215Z-governor-reviewer-egbp.md
topic: v300-freeze-declared-bridge-record
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# 3.0.0 content freeze declared (bridge record, provisional)

**Provenance.** In a live session on 2026-11-02 the Human Governor said "freeze". This note
transcribes it. It is a bridge record: **not binding authority on its own**, and provisional
until a `governor-reviewer` record references it. Freeze, versions and dates are outside the
delegation `srra`; the session acts on the human's instruction only to the extent of
preparation and does not tag, merge release metadata or set a release date.

The plan `20261101T195927Z-v300-release-publication` proposed the freeze point
2026-11-05T12:00:00Z. This instruction moves it earlier: the freeze commit is the `main` tip
`909feeb162cfcc671ae382d6727ba66f75398249` (observed 2026-11-02, after the merges of !3466,
!3467 and !3471 and their records). Any later product change needs a re-freeze.

## Scan at the freeze point

- Open merge requests: none. Claims: none (`agov coord status --roles`).
- Since the v2.13.1 tag commit `db300c09`: 408 changed paths, none under `tooling/`,
  `agf/schemas/`, or `.gitlab-ci.yml`. Distributed-rule changes are the forge-evidence rule
  and its two pointer edits, plus `agf/agents/MANIFEST.example.yaml`.
- Old pending handoffs unrelated to this release remain open (`b2hm`, `pages-home-slice-5`,
  an estate-sweep and two propeller handoffs from September).

## Next, still preparation only

1. Content-freeze sweep by the documentation-estate steward at `909feeb1`.
2. `release-3.0.0` metadata branch from this exact base: the version-field inventory (paths A of
   the plan), changelog move, Adopter docs lines, namespace preflight, metadata MR.
3. The release date for the changelog heading, metadata Gate 2 and the tag stay with the
   Human Governor.
