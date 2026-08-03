---
kind: decision-record
from: governor-reviewer
to: all
id: 20260803T065103Z-governor-reviewer-se21
timestamp: 2026-08-03T06:51:03Z
immutable: true
references-to:
  - 20260802T232120Z-contribution-auditor-8mmm.md
  - 20260802T222231Z-coder-tester-62p5.md
  - 20260802T233548Z-contribution-auditor-jgjh.md
  - 20260802T233331Z-contribution-auditor-rbdm.md
  - 20260802T231600Z-contribution-auditor-7eek.md
topic: governor-decision-packet-mrs-947-967-979-981-984-986
requires-ack: false
artifacts_owned: []
model: codex-gpt-5
---

# Governor decision packet: contract core, CI, research plans, radar, learning

The human governor explicitly approved this packet in the active session on
2026-08-03 and authorized execution in the order below. Codex records and
operationalizes that decision; it does not independently supply the authority.

## Binding decisions

1. **MR !947 — Gate 2 APPROVED and merge authorized** at exact head
   `e4ad42689df6f03ac9d5c59ad29f1e07675dfb58`, pipeline 2725664383
   (success). The decision expressly includes A-MERGE-1: the merge-conflict
   resolution's WP2 changelog cutoff correction to `2026-08-02T22:01:54Z`
   is accepted because it matches shipped behavior, rule documentation, and
   decision `20260802T220154Z-governor-reviewer-sejb`. Close issue #79 after
   verifying the merge on `main`.
2. **MR !979 — fast-track Gate 2 APPROVED and merge authorized** at exact
   head `3e8cb0af084b18bb44de91b8e012e841814babd3`, pipeline 2725614555
   (success). This is the bounded, single-file CI path-scoping correction
   supported by root-cause record `62p5`; it changes no product or release
   contract and unlocks exact-head acceptance for top-level registry/radar
   documentation changes.
3. **MR !986 — merge authorized after MR-metadata cleanup** at exact head
   `a1fce7bf1c143105ab48724f4a96a74f56c197ed`, pipeline 2725678022
   (success). First correct the stale MR title from “8 of 9” to the reviewed
   seven-current-non-legacy-schema framing and resolve the obsolete BLOCK
   discussion. No content change or new pipeline is required for those MR
   metadata operations.
4. **MR !984 — HOLD.** Do not merge before !962. After !979 merges, refresh
   !962 onto current `main`, obtain an exact-head pipeline, review and merge
   it; then refresh !984 so Batch 14 follows Batches 7–13, record the explicit
   docs-only fast-track waiver in its commit message, obtain a new exact-head
   green pipeline, and return it for merge.
5. **MR !967 — Gate 1 HOLD.** Contribution-auditor findings are CLEAR at
   `a78dd266b7c68c73eee1aad4fac48978c5d702e0`, but approval is withheld
   until the plan completes its declared infosec-sentinel review of AI-2/AI-5,
   primary-source fact-checker verification, !962 dependency disposition,
   remaining scope choices, and an exact-head metadata/checklist update.
   Resolve the obsolete contribution-auditor BLOCK discussion by linking the
   final CLEAR; do not set `status: approved` yet.
6. **MR !981 — Gate 1 HOLD.** Contribution-auditor findings are CLEAR at
   `ef570df4767b8dc8ed68abe8485172040dac90ca`, but approval is withheld
   pending the narrower fact-checker pass over LLIS system-process claims,
   the candidate-versus-child-plan scope decision, and an exact-head
   metadata/checklist update recording review `7eek`. Resolve the obsolete
   contribution-auditor BLOCK discussion; do not set `status: approved` yet.

## Coordination housekeeping authorization

Preserve, do not discard, the unique records in the open coordination MRs and
the branch-only review/ack publications they reference. Create one fresh-main
coordination-only consolidation MR, copy every in-scope immutable record
unchanged, validate the records and graph, run `coord publication-check`, and
merge after its exact-head pipeline is green. Then close each source MR as
**merged by content**, linking the consolidation MR. This recovery path also
covers the four older source MRs whose original pipelines are skipped; do not
merge those unverifiable source heads directly.

## Operational sequence

Publish this decision, merge !947, merge !979, clean and merge !986, recover
and merge !962, then refresh !984. Close issues #79 and #80 only after their
respective merged implementations are verified on `main`. Release publication
is not authorized by this packet and still requires its own plan.
