---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260902T084517Z-governor-reviewer-jqdd
timestamp: 2026-09-02T08:44:39Z
model: human-directed-claude-opus-5
immutable: true
references-to:
  - 20260831T083954Z-coder-tester-k6s2.md
  - 20260831T040443Z-coder-tester-4434.md
  - 20260901T114736Z-planner-architect-rhqb.md
  - 20260831T050008Z-governor-reviewer-r2kb.md
topic: gate1-approved-v290-release-and-calm-visual
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 approved: v2.9.0 release publication, and the calm explanatory-visual pilot

Binding Gate 1 decisions, given by the governor-reviewer in session
("!2302 and !2292 gate approved") and transcribed by Claude.

## Decisions

**MR !2302 — `docs/plans/20260831T050110Z-v290-release-publication.md`** is
approved at exact head `3cb9f28a6ac0d9fcf7515d76c57a9ad67db44d9e`.
Candidate release **v2.9.0**.

**MR !2292 — `docs/plans/20260829T183326Z-calm-explanatory-visual-system.md`**
is approved at exact head `c8fa182acd97f16e69dd634ff6cfb400476c10ba`.

Both plans' front matter is updated in the same cycle as this record, and this
record is published **before** either plan merges, so `main` never carries an
approved plan without its authority. That ordering is deliberate: on
2026-08-31 the reverse happened, the metadata push silently failed, and `jerr`
had to raise it as a stop condition.

## Evidence standing at approval

| | !2302 | !2292 |
|---|---|---|
| Independent review | **CLEAR** (`k6s2`) | content-complete (`4434`) |
| Review rounds | 4 | 4 |
| Exact-head pipeline | `2804468124` **success** | `2804253638` **success** |
| Recorded in | `rhqb` | `rhqb` |

## Scope — what this does not authorize

- **No implementation, merge to a release base, tag, or publication.** Gate 2
  and every release action remain separate decisions.
- **!2302 does not authorize tagging v2.9.0.** Its own steps make the pre-tag
  proofs, the derived-namespace absence read, and a green canonical pipeline
  preconditions. Acceptance criterion 1 — A1's vulnerable-base negative
  transcript — is still outstanding and is a precondition of Gate 2, not of
  this approval.
- **!2292 does not authorize creating any pilot artifact.** Its ten-file
  boundary and stop conditions govern implementation, which needs its own
  claim and Gate 2.
- **!2292 carries no `candidate_release`.** It predates `r2kb` and is not
  retroactively invalidated, but per that decision it must declare one — or
  `none` with a reason — at its next revision. That is an authoring
  obligation on the next edit, not a condition of this approval.
- **Nothing else is approved.** Sub-case C (`nqeg`) remains unplaced; the
  visibility-correlation implementation still needs a release disposition; the
  plan-status normalisation flagged in `a7jr` has no plan yet.

## Resolves

The Gate 1 obligations in `k6s2` and `4434`, and items 1 and 4 of the queue
map in `a7jr`.
