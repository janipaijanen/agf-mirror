---
artifacts_owned: []
from: governor-reviewer
id: 20260809T143855Z-governor-reviewer-qpkg
immutable: true
kind: decision-record
model: claude-sonnet-5
references-to:
- 20260803T200227Z-governor-reviewer-5m41.md
- 20260804T072128Z-governor-reviewer-hhdb.md
- 20260805T090722Z-governor-reviewer-83m8.md
requires-ack: true
timestamp: 2026-08-09T14:38:55Z
to: all
topic: v250-final-hold-lifted
---

# Hold `5m41` lifted for `v2.5.0` final

**Transcription note.** This decision was given directly by the human
governor in a Claude Code session, judging `v2.5.0-rc.2`'s observation
window (tagged 2026-08-04T10:38:40Z, five calendar days elapsed as of
this record, no installation/upgrade/archive-overlay failures, CLI/MCP
mismatches, documentation-misunderstanding reports, provenance/checksum/
OCI/Release defects, or adopter/independent-review regressions recorded
against the rc.2 artifact in that window) sufficient, per `5m41`'s own
"no fixed elapsed-time threshold... the governor decides when the
observation period is sufficient" text. Claude/coder-tester files this
record verbatim at that instruction — the decision is the human
governor's; only the mechanical filing is delegated, consistent with the
precedent `hhdb` set for the narrower rc.2 lift.

## Decision

`5m41`'s prohibition on beginning "a v2.5.0 final... plan or release
process" — left standing by `hhdb` (narrowed only to `v2.5.0-rc.2`) and
by `83m8` (narrowed only to MR !1082 planning) — is now **lifted for
`v2.5.0` final specifically**.

This authorizes planner-architect (or whichever role is assigned) to
draft a `v2.5.0` final release-publication plan and take it through
Gate 1 review. Per `20260804T064132Z-v250-rc2-release-publication.md`'s
own rc.3-trigger section: that plan must be **limited to version and
changelog metadata**, promoting `v2.5.0-rc.2`'s exact functional
candidate unchanged. It does not authorize folding in any other
in-flight surface (independent-implementation-reviewer role work, the
Pages/README visual-refresh track, get-started commissioning slices,
the frozen CI-latency plans, or the v2.7 discovery plans) — none of
those are part of rc.2's shipped scope and none are prerequisites for
promoting it to stable.

## What remains bound

`5m41` remains binding, unchanged by this record, for `v2.5.1-rc.1` and
`v2.5.1-rc.2`. Any functional, schema, documentation, migration, test,
or artifact-layout correction discovered in `v2.5.0-rc.2`'s shipped
scope before the final release MR merges still triggers `rc.3` per that
plan's own trigger clause, not a silent inclusion into final.

## Next step

Draft `docs/plans/<timestamp>-v250-final-release-publication.md` and
route it through Gate 1 review before any release branch is cut, per
`development-workflow-rule.md` and `release-via-merge-request-rule.md`.
