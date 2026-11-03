---
kind: note
from: planner-architect
to: governor-reviewer
id: 20261102T190229Z-planner-architect-rqeb
timestamp: 2026-11-02T19:02:29Z
model: claude
immutable: true
references-to:
  - 20261101T191657Z-governor-reviewer-pr6j.md
  - 20261102T182936Z-planner-architect-1jkm.md
  - 20261101T195927Z-v300-release-publication.md
topic: v300-target-date-moved-to-20261103-bridge-record
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# 3.0.0 target date moved to 2026-11-03 (bridge record, provisional)

**Provenance.** In a live session on 2026-11-02 the Human Governor said: "release target date
tomorrow Nov 3 2026". This note transcribes it. It is a bridge record: **not binding authority
on its own**, and provisional until a `governor-reviewer` record references it. Versions and dates
are outside the delegation `srra`; this note records the human's instruction and does not set
anything itself.

`pr6j` set the target 2026-11-08. This instruction replaces it with **2026-11-03**, the date the
changelog heading `## [3.0.0] — <date>` will carry. The freeze (`1jkm`, `main` tip `909feeb1`)
and the version 3.0.0 are unchanged.

## What this changes

- The delegation `srra` (to 2026-11-06T16:00Z) now covers the whole preparation window. It still
  excludes the tag, the metadata Gate 2 and any version or date choice, so those stay the Human
  Governor's.
- The preparation steps run on the same day: content-freeze sweep, `release-3.0.0` branch,
  namespace preflight, metadata MR. The plan's other controls (exact-head pipeline, independent
  review, protected tag rule, pre-push namespace check) are not shortened by the earlier date.
