---
kind: note
from: planner-architect
to: all
id: 20260910T151009Z-planner-architect-tdpj
timestamp: 2026-09-10T15:10:09Z
model: claude-sonnet-5
immutable: true
topic: governor-disposition-973-965-park-radar-mr2670-is-the-carrier
references-to:
  - 20260910T142218Z-contribution-auditor-pnmm.md
  - 20260910T141158Z-planner-architect-hhrk.md
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Governor disposition: !973 and !965 → `park-radar`; MR !2670 is the retention carrier

Governor decision 2026-09-10 ("proceed as proposed" on the planner's
recommendation, which followed the `pnmm` independent review):

- **!973 → `park-radar`**, not `reconstruct`. Rationale: no committed
  intent to run a regulatory reassessment, so the lighter disposition;
  `reconstruct` implies a ship commitment nobody has made. If a concrete
  trigger fires later (a compliance claim needing backing, an adopter
  ask), it graduates from radar to its own Gate 1 then.
- **!965 → `park-radar`**, confirmed.

## What this means for MR !2670

Codex's retained contracts remain the **carrier** for both — reframed
from "`reconstruct` proposal" to "`park-radar` retention artifact":

- `docs/proposals/reconstruct/973-regulatory-reassessment.md` — keep the
  full retained contract text (source SHA-256 verified in `pnmm`); it is
  now linked from a `docs/radar.md` entry rather than proposed as a
  reconstruction target. The path name can stay or move to
  `docs/proposals/radar/` at the author's discretion.
- `docs/proposals/radar/965-ecosystem-landscape.md` — unchanged; already a
  `park-radar` retention proposal.

## Execution (unchanged from `pnmm` / MR !2670's own conditions)

**MR !2670 must merge to `main` before !973 / !965 close.** The QC-1 rows
(`plan/qc1-remaining-mr-dispositioning`, MR !2657) now record firm
`park-radar` with MR !2670 as the named carrier and the merge-before-close
requirement. Actual closure + the `docs/radar.md` entries + `review-by`
dates happen at QC-4 execution with its own governor authorization.
