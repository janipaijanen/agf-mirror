---
kind: decision-record
from: governor-reviewer
to: all
id: 20260826T150925Z-governor-reviewer-cih1
timestamp: 2026-08-26T15:09:25Z
model: human-directed-claude-sonnet-5
immutable: true
references-to:
  - 20260826T044424Z-ci-history-1-job-local-full-history
  - 20260824T165910Z-governor-reviewer-rjse.md
  - 20260826T150924Z-build-engineer-be19.md
topic: ci-history-1-superseded-by-mr1965
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# CI-HISTORY-1 (MR !2080) superseded by MR !1965's already-approved fix

The Human Governor, directed live in this session: CI-HISTORY-1's plan
(`docs/plans/20260826T044424Z-ci-history-1-job-local-full-history.md`,
MR !2080) is **superseded, not implemented**. MR !1965 independently hit
and fixed the identical defect two days earlier
(`real_mr1712_generation_acceptance_is_142_included_plus_50_excluded`
failing under default shallow-clone depth), with the identical one-line
`GIT_DEPTH: "0"` change to the identical job, already governor-approved
in `rjse` and now unblocked by the targeted build-engineer review `be19`
this session performed. Landing both would either conflict on the same
`.gitlab-ci.yml` line or duplicate the change pointlessly.

## Disposition

- **MR !1965** proceeds as the implementation of record for this fix. It
  remains subject to its own separate Gate 2 process for the rest of its
  scope (the adoption learning-record version-drift check), unaffected
  by this decision.
- **MR !2080 / CI-HISTORY-1's plan** does not merge. Its `status` field
  remains `pending-gate1` in its own file (the Gate 1 plan schema has no
  `superseded` enum value — do not set one, it would fail `agov plan
  validate`), but this decision-record is the authoritative statement
  that Gate 1 approval will not be granted and implementation must not
  proceed. Once MR !1965 merges, close MR !2080 without merging,
  referencing this record.
- Once MR !1965 merges, fresh corrected exact-head pipelines are still
  needed for MR !2041 and MR !806 — the two MRs CI-HISTORY-1's own
  evidence-gathering diagnosed as blocked by this defect — exactly as
  CI-HISTORY-1's own "Shipment" section already specified. That
  requirement transfers to MR !1965's merge as the actual fix event; it
  is not lost by abandoning MR !2080.

## Why this happened (learning note, not blocking)

Two independent sessions (one working MR !1965, one working CI-HISTORY-1)
diagnosed and planned the identical fix without either being aware of the
other's work, because the coordination channel's polling/discovery
pattern surfaces the *newest* unresolved obligations more readily than it
surfaces *already-approved, stalled-on-an-unstaffed-role* work sitting
quietly in an older thread. This is not this session's finding to
formally file as a learning record — flagging it here for whoever next
reviews cross-session coordination discipline to decide whether it
warrants one.
