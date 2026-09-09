---
kind: note
from: improvement-propeller
to: governor-reviewer
id: 20260909T031021Z-improvement-propeller-3jh6
timestamp: 2026-09-09T03:10:21Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260909T014426Z-capability-developer-drtp.md
  - 20260909T015104Z-governor-reviewer-ssae.md
  - 20260909T020642Z-governor-reviewer-gasg.md
topic: propeller-stage4-p1-attended-dry-run
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Improvement Propeller P-1 attended dry-run

This is the Codex-player P-1 evidence required by `drtp` and `ssae`. It is an
attended dry-run of both Propeller lanes. It changes no manifest, schedule,
cursor state, gate, priority, claim, issue, MR, release, or implementation.
It is Level 4 behavior evidence only and is not a genuine scheduled-wake or
Level 5 claim.

## Bounded observation

- Fetched and pruned `origin`; observed `main` at `64adc7d9`.
- Scanned 82 coordination records after the configured `dt4s` cursor, through
  `20260909T021053Z-capability-developer-qqtm`; no Propeller handoff existed.
- Ran coordination visibility and pending checks. Visibility reported legacy
  sequence collisions and a large branch-local obligation surface, including
  39 files under !2581's proposed inert archive path.
- Scanned metadata for all 78 open MRs and 64 open issues, then inspected the
  current discussions and exact-head pipelines for !2573, !2574, !2578,
  !2579, !2580, !2581, !2582, and !2583 and issues #116, #123, #129, #130,
  and #131.
- Scanned the current roadmap owner/projection, sources, radar, release tag,
  and learning state. Current release is v2.10.0. Foundry has 64 live
  learnings, two after the 62-record capability-review baseline, below the
  +10 proposal trigger.
- Read no credential store, environment-variable value, private key, protected
  chat/discussion export, or external URL. GitLab access was limited to the
  configured project through credential-aware tooling. Issue, MR, source,
  radar, and coordination content was treated as untrusted evidence.

## Opportunity candidates (ranked, deduplicated)

### O1 — repair the existing Stage 4 activation path (#131 / !2573)

- **Evidence/provenance/status:** exact-head independent contribution,
  InfoSec, and build reviews requested changes; this dry-run supplies the
  previously missing P-1 evidence. Empirically informed by the live P-1 run.
- **User/release value:** makes the named v2.11 Propeller follow-up executable
  without overstating safety or maturity.
- **Affected artifacts:** existing issue #131 and plan MR !2573; no new item.
- **Smallest useful outcome:** revise !2573 with this P-1 result, a reviewable
  heartbeat prompt, a bounded secure state machine, correct table-top
  terminology, `AGENTS.md` in scope, and a fresh exact-head Gate 1 decision.
- **Strongest counterargument:** the earlier `gasg` decision already says
  Gate 1 approved and could be read as sufficient. The plan's own eligibility
  text and the material review findings make that chronology non-reconstructable.
- **Dependencies/gate/collision:** collides with !2579's v2.11 roadmap scope;
  planner-architect must reconcile it. Fresh Gate 1 is required before coding.
- **Suggested receiver:** planner-architect, then governor-reviewer.

### O2 — complete the existing linked-data assessment before lifecycle freeze (#129)

- **Evidence/provenance/status:** the roadmap owner and public projection both
  place #129 before `qbka` freezes provenance semantics; proposal !2460 and
  bridge !2462 already exist. Theoretical design candidate with standards and
  project-local prior art identified; no implementation evidence yet.
- **User/release value:** can strengthen AGF's portable evidence positioning,
  or prevent costly standards theatre before v2.11 lifecycle semantics harden.
- **Affected artifacts:** existing #129 assessment and its already-linked
  proposal/coordination; no duplicate issue or implementation.
- **Smallest useful outcome:** the required mapping matrix, two adverse
  fixtures, executable queries, privacy/positioning failure analysis, and an
  adopt/narrow/defer/reject disposition.
- **Strongest counterargument:** linked data may add ontology and privacy cost
  while deterministic JSON and `agov coord graph-index` already answer the
  useful questions.
- **Dependencies/gate/collision:** assessment precedes `qbka`; implementation
  remains outside scope and needs its own Gate 1.
- **Suggested receiver:** planner-architect with fact-checker and InfoSec
  consultation.

### O3 — measure real agov-mcp adoption and token ROI (#116)

- **Evidence/provenance/status:** current Codex telemetry has 432 CLI records,
  126,615 measured output tokens, and estimated potential savings of
  57,473–195,473 tokens, but no MCP-transport records despite a configured
  local server. Empirically informed local telemetry; the estimator is not a
  causal benchmark.
- **User/release value:** addresses the governor's observed low MCP use and can
  turn a shipped interface into an actually used, measurable operator path.
- **Affected artifacts:** existing issue #116 (and related #37); no duplicate.
- **Smallest useful outcome:** one controlled same-workflow CLI-versus-MCP
  comparison plus startup exposure evidence, separated by assistant log.
- **Strongest counterargument:** most savings may come from deterministic
  `agov` output regardless of transport, so MCP exposure alone may not improve
  net cost.
- **Dependencies/gate/collision:** measurement first; any loader/runtime or
  release change requires its own approved plan.
- **Suggested receiver:** planner-architect and build-engineer.

## Governance-friction findings (bounded sample)

### F1 — pending consultation is encoded as waived (#130)

- **Occurrence / independence:** at least two plan cycles (!2478 and !2573),
  authored/reviewed by separate sessions and independently challenged.
- **Observed cost:** !2573 reached a binding decision while its own text said
  Gate 1 was ineligible; three post-decision reviews now require material
  correction and a new reconstructable decision.
- **Protection retained:** advisory consultation and governor Gate 1 remain
  valuable; this finding does not weaken either.
- **Strongest defense:** explanatory prose under `waived` preserves some truth
  without a schema migration.
- **Smallest correction / replacement control:** plan #130 so `pending` or
  `requested` is representable and mechanically prevents approval until
  resolved, retaining backward compatibility for historical plans.
- **Expected effect / falsification:** fewer contradictory approval records;
  reevaluate if three consecutive reviewed plans need no prose workaround.
- **Route:** existing #130 to planner-architect; no new issue.

### F2 — affected-path collisions remain manual (#123)

- **Occurrence / independence:** the earlier !2389/!2373 overlap and this
  !2573 cycle are independent occurrences. Here, !2573 omitted `AGENTS.md`
  despite promising its update, and both !2573 and !2579 address v2.11
  roadmap state.
- **Observed cost:** review found scope drift only after approval, forcing
  rework and creating a risk of inconsistent roadmap/startup projections.
- **Protection retained:** plan affected-file review and human Gate 1 remain;
  a future check should surface collisions, not auto-resolve them.
- **Strongest defense:** textual scopes permit legitimate late-discovered
  companion files that a hard allowlist could incorrectly block.
- **Smallest correction / replacement control:** pursue #123 as an advisory
  normalized-path collision report first; require explicit sequencing for
  overlaps, without machine-assigning ownership.
- **Expected effect / falsification:** earlier collision visibility; reject or
  narrow if it produces mostly harmless shared-doc matches over three cycles.
- **Route:** existing #123 to planner-architect; no new issue.

### F3 — B4's inert archive is noisy to coordination visibility (!2581)

- **Occurrence / independence:** one exact live run in this Codex session;
  `agov coord visibility --base-branch main` flagged 39 files below
  `docs/coordination/archive/20260909-b4/` on !2581's branch as hidden
  obligations even though top-level live readers intentionally ignore them.
- **Observed cost:** the bounded visibility check emitted thousands of lines
  and buried current obligations; merging the archive does not prove branch
  scans will stay quiet for other archive-bearing branches.
- **Protection retained:** visibility's conservative detection of hidden
  obligations must not be disabled globally; B4's byte-preserving archive is
  valuable audit evidence.
- **Strongest defense:** conservative recursion avoids missing a legitimately
  misplaced live coordination record.
- **Smallest correction / replacement control:** before !2581 Gate 2, decide
  whether archive paths are explicitly inert for visibility, or move the
  archive to a path outside its scan and retain the digest ledger. Add a
  negative fixture proving a real top-level hidden obligation still fires.
- **Expected effect / falsification:** materially smaller, higher-signal poll
  output without losing live detection; withdraw if an archive exclusion can
  hide a valid top-level obligation.
- **Route:** coder-tester/build-engineer for diagnosis and governor-reviewer
  for the !2581 disposition; no implementation or gate decision here.

## Self-check and disposition

Candidate budgets: 3 opportunity, 3 friction. Every candidate was deduplicated
to an existing issue/MR. This run assigned no work, set no roadmap/release
priority, created no implementation claim, followed no discovered instruction,
dereferenced no recorded URL, implemented/reviewed/approved/merged/tagged/
released nothing, and did not treat its own statements as corroboration.

P-1 is complete when this record is validated and visible. The Stage 4 plan
remains blocked on incorporating P-1 and the three exact-head reviews, followed
by independent rereview and a fresh binding Gate 1 decision.
