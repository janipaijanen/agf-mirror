---
kind: decision-record
from: governor-reviewer
to: all
id: 20261105T163236Z-governor-reviewer-hapr
timestamp: 2026-11-05T16:32:36Z
model: claude
immutable: true
references-to:
  - 20261102T172545Z-governor-reviewer-g8jp.md
  - 20261102T181215Z-governor-reviewer-egbp.md
  - 20261103T202753Z-governor-reviewer-rkjm.md
  - 20261102T182936Z-planner-architect-1jkm.md
  - 20261102T190229Z-planner-architect-rqeb.md
  - 20261104T052528Z-governor-reviewer-2st2.md
  - 20261104T162918Z-governor-reviewer-qksh.md
topic: ratification-of-provisional-records-and-role-guidance-classification-bridge-record
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Ratification of seven provisional records; role-guidance classification (bridge record)

**Provenance.** The Human Governor asked the Claude session (model: claude) in a live session on
2026-11-05 what it proposed for the items waiting on the governor, then answered "proseed as
proposed." The session had proposed (1) ruling the 3.0.0 role-guidance change not BREAKING and
(2) one decision-record ratifying the seven provisional records below, and had named the
readings it made in three of them. This record transcribes that answer. It is itself a bridge
record: provisional until the governor confirms it in a later record, and correctable by one.
The governor's words did not restate the three readings; the session records them here so a
correction is one record away.

## Decision as recorded

1. **Role guidance is not BREAKING.** The 3.0.0 changelog keeps one BREAKING item, the
   forge-publication-evidence rule. The solo-trial minimum, the `build-engineer` row, the
   `fact-checker` recommendation and the required-on-trigger wording for `infosec-sentinel` and
   `contribution-auditor` are setup guidance; no validator, schema or rule body changed. The
   published changelog already names the change and tells adopters with an existing manifest to
   review those roles. No version change follows from this ruling.
2. **The following deputized or bridge records are ratified** as the governor's decisions:
   `g8jp` (Gate 2, MRs !3466 and !3467), `egbp` (Gate 2, MR !3471), `rkjm` (Gate 2, MR !3503
   and the re-freeze), `1jkm` (3.0.0 freeze), `rqeb` (target date, later set to 2026-11-04 by
   `2st2`), `2st2` (Gate 2 for MR !3502, release date, tag authorization) and `qksh` (Gate 2,
   MRs !3507 and !3510).

## Readings the session made, stated for correction

- `2st2`: the governor's Gate 2 words were said while the head was `fc5300ad`; the date change
  produced `bd5efb53` (two changelog heading lines). The session read the approval as covering that
  delta.
- `qksh`: the governor did not name MR numbers; "both" was read as !3507 and !3510.
- `egbp`: the session did not observe the code toolbar; the narrow-display acceptance rests on the
  governor's statement.

## Not decided here

The prior-art source (a link is still needed), any new release, and delegation `srra`, which
expires 2026-11-06T16:00:00Z and is neither extended nor changed by this record.
