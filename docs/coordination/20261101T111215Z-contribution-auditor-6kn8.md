---
kind: review
from: contribution-auditor
to: all
id: 20261101T111215Z-contribution-auditor-6kn8
timestamp: 2026-11-01T11:12:15Z
model: claude
immutable: true
references-to:
  - 20261030T165922Z-contribution-auditor-q48r.md
  - 20261101T105902Z-governor-reviewer-kqgp.md
  - 20261030T155116Z-governor-reviewer-ghgs.md
topic: forge-evidence-plan-independent-review-ff368bdb
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Independent review of the forge-evidence plan at `ff368bdb`

Scope: `docs/plans/20261030T160040Z-forge-evidence-rule.md` and its projection
manifest at exact head `ff368bdb49341d67d71a5aad7e6cfee59836b289` (MR !3426).

**Provenance.** The governor asked for an independent check. The planner-architect
session started two fresh-context AI reviewers (model: claude) that did not write
the plan and worked read-only: one contribution audit (F1, F3, factual claims) and
one InfoSec and privacy review (F2, leak risk). This record transcribes their
reports. The InfoSec pass is an advisory AI review. It is **not** the CI
`infosec-sentinel` and it carries no gate authority. The planner-architect checked
the blocking findings against the cited rule files before acting. Authority:
recommendation only; the governor-reviewer decides Gate 1.

Coverage: COMPLETE for both lenses at the exact head. Recommendation: **BLOCK**
Gate 1 at `ff368bdb` until the findings below are answered and the revised head is
re-reviewed. Blocking findings: 3. Advisory findings: 6 (contribution) and 2
(InfoSec). Open findings with accepted risk: 0.

## What the reviewers found resolved

- q48r F1 is resolved apart from one missed surface (F1-miss below).
- q48r F3 is resolved from the text. Option A is ineligible for any v2.x minor
  release. The plan grants no version, date or authority and makes no D1 choice.
- q48r F2 is resolved in structure (split, destination boundary, no ban on
  access-controlled records), with the two defects below.
- The foundry direction in `ghgs` is preserved in the plan and its foundry route.
- All 22 `depends_on` plan IDs exist and are approved. The cited section in the
  day-to-day page exists. No example contains a real secret or private link.

## Blocking findings

**F1-miss (category 1; authority: q48r F1 resolution condition, plan acceptance
criterion 1).** `.claude/settings.json` (`claudeMdExcludes`) is missing from
`affected_paths` and the propagation table. Failure: an implementer adds the new
rule symlink and index row but not the exclude glob, so the rule loads at startup
in every foundry session; or edits the file and breaks "only listed paths change".
Fix: add the path, a table row and the manifest path. Intent: foundry-local routing
mirrors the existing situational-rule pattern.

**F2-R1 (category 1; authority: q48r F2 resolution condition).** Example row 3
called "a secret or private link" forbidden. "Private link" is no inherited
exclusion. On the adopter's own forge it is an Internal destination under
`external-anonymization-rule.md`. Failure: a reviewer blocks an internal
private-link citation that no owner rule forbids. Fix: name only a credential or
token in the row and add a separate not-forbidden row for a link without a secret.
Intent: examples illustrate existing exclusions and add none.

**F2-R2 (category 2; authority: `security-hardening-rule.md`,
`external-anonymization-rule.md`).** The "inherited exclusions" sentences overstated
the owners. `security-hardening-rule.md` covers committed files and detection, not
forge text. `external-anonymization-rule.md` lists project handles, not "customer
data". So "adds no new privacy prohibition" was partly false. Fix: restate the
owners' real scope. Intent: restate, extend nothing.

## Advisory findings

- Contribution A1: manifest loader paths omit `AGENTS.md` and the INDEX; record
  `.claude/CLAUDE.md` as unchanged.
- A2: the "MAJOR means breaking" claim needs a citation (`agf/framework.md`,
  versioning section).
- A3: criterion 4a should name a binding `governor-reviewer` decision as the
  approver of the major-release sequence, and the implementation MR stays unmerged.
- A4: "inherited" is slightly overstated (same cause as F2-R2).
- A5: name the placement in `agf/templates/AGENTS.md` (its Rules section).
- A6: option B text remains and the plan does not record D1 = A (governor decision
  `20261101T105902Z-governor-reviewer-kqgp`).
- InfoSec A1: the destination paragraph omits the AI-provider-prompt row of the
  anonymization boundary table.
- InfoSec A2: "chat provenance belongs in the audit trail" should say a minimal
  reference, not a transcript, with no credential or tokenized URL.

## Not checked

The reviewers did not validate the manifest against its schema, did not check
that the day-to-day page ships in the distribution artifact, and did not review
the loader and pointer edits, which are not yet drafted.

## Status

Nothing is cleared. The planner-architect's answers to these findings are in a
revision handoff; they remain unverified by the reviewers until an exact-head
re-review.
