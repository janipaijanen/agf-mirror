---
artifacts_owned: []
from: governor-reviewer
id: 20260804T072128Z-governor-reviewer-hhdb
immutable: true
kind: decision-record
model: claude-sonnet-5
references-to:
- 20260803T200227Z-governor-reviewer-5m41.md
- 20260803T225851Z-contribution-auditor-pjep.md
- 20260804T002719Z-contribution-auditor-rnp5.md
requires-ack: true
timestamp: 2026-08-04T07:21:28Z
to: all
topic: v250-rc1-hold-lifted-rc2-authorized
---

# Hold `5m41` lifted for `v2.5.0-rc.2`; answers `pjep`'s bounded question

**Transcription note.** This decision was given directly by the human
governor in a Claude Code session ("proceed to 2.5.0-rc.2"), after
Claude/planner-architect flagged that neither `5m41`'s hold nor `pjep`'s
unacknowledged bounded question had been formally resolved before RC2
planning began. Claude filed this record verbatim at that instruction —
the decision is the human governor's; only the mechanical filing was
delegated, consistent with the governor's own stated operating model
earlier in this session.

## Decision

`pjep` asked for a bounded binary choice: authorize an rc.2 plan
containing the accepted corrective slices, or explicitly accept the
audience-rule mismatch and structural gate-claim gap for post-2.5.0.

**Choosing option 1, narrowly.** `5m41`'s prohibition on beginning "a
v2.5.0 final, v2.5.0-rc.2, v2.5.1-rc.1, or v2.5.1-rc.2 plan or release
process" is lifted **specifically for `v2.5.0-rc.2`**, covering the
already-drafted plan `docs/plans/20260804T064132Z-v250-rc2-release-
publication.md` (MR !1129, corrected per `3r3a`'s round-1 review) and
its continuation through Gate 1 review, approval, and release execution.

`5m41` remains binding for `v2.5.0` final, `v2.5.1-rc.1`, and
`v2.5.1-rc.2` — none of those are authorized by this record. Stable
`v2.5.0` remains separately Gate 1-gated per `5m41`'s own text and per
this plan's own "draft the stable plan alongside RC2, activation
conditional on clean RC2 observation" structure.

`pjep`'s audience-rule finding (point 1) is not resolved by this
record — that remains open, separately, as its own governed rule
change with its own migration note, per `downstream-impact-rule.md`.
This decision does not fold that into rc.2's scope.

Gate 1 approval of the RC2 plan itself is a separate, still-required
step — this record authorizes the plan and process to *proceed*, it
does not itself constitute Gate 1 approval of `docs/plans/20260804T064132Z-
v250-rc2-release-publication.md`.
