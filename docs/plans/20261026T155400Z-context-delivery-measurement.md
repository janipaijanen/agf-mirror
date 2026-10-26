---
id: 20261026T155400Z-context-delivery-measurement
status: pending-gate1
topic: context-delivery-measurement-phase-a
advisory_consultation:
  - role: infosec-sentinel
    status: pending
    reason: >-
      Review requested. Phase A captures client startup and debug output and
      inspects third-party plugin packages. The plan needs an infosec screen of
      the plan text now and of every captured output before any fixture or
      excerpt is committed.
  - role: contribution-auditor
    status: pending
    reason: >-
      Non-author audit requested from codex-foundry (room seq 622). The author
      is planner-architect.
affected_paths:
  - docs/plans/20261026T155400Z-context-delivery-measurement.md
  - docs/reviews/context-delivery-measurement/20261026T155400Z-context-delivery-measurement-report.md
depends_on:
  - 20261022T205113Z-instruction-context-reduction
  - 20261023T080431Z-instruction-context-followup
  - 20261026T151646Z-next-release-scope-intake
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: context-delivery measurement, Phase A

Authored by planner-architect. Version-neutral: this plan sets no release version,
scope, date or content. It follows the room debate of 2026-10-26 (seq 603 to 622),
where the Human Governor asked for a plan and codex-foundry agreed the amended
scope (seq 622). Room consensus creates no authority; this plan needs a
`governor-reviewer` Gate 1 record before any measurement starts.

## Decision requested

Gate 1 approval for a measurement-only workstream. It answers one question before
anyone designs a different delivery shape for AGF: after the foundry-local
reductions already shipped, is there material residual instruction-context cost
for clients, and what do existing plugin packages teach about delivery? Phase A
changes no rule, loader, schema, package or tool. It produces one report.

## Starting facts (checked 2026-10-26)

- Two approved plans are implemented: `20261022T205113Z-instruction-context-reduction`
  and `20261023T080431Z-instruction-context-followup`. Both are foundry-local
  (`claudeMdExcludes`, `.claude/rules/INDEX.md`, AGENTS.md trigger lines).
- Last recorded measurement (followup plan, 2026-10-23, `main` at `eb70adb8`):
  33 files, 156,887 chars on disk, about 149.5k startup-report chars at the
  recorded 0.953 ratio. Issue #138 measured 308.8k chars before the reductions.
  Neither figure is a post-reduction result for 2026-10-26.
- Shipped `agf/rules` is about 314 KB across 53 files; adopters load their own
  selection through their own loader, so the foundry measurement does not
  describe an adopter.
- Open: item 2 of issue #138 (split the largest shipped rules into core and guide)
  and item 3 (advisory budget report). Both stay out of this plan.

## Requirements

Each row is labeled `measured fact`, `assumption to test` or `decision`. A UX or
feature claim needs a baseline, target and failure condition.

### Phase A (executed after Gate 1)

| ID | Label | Requirement | Acceptance check | Failure or stop condition |
|---|---|---|---|---|
| SDD-CTX-1 | assumption to test | Effective loaded instruction size at session start is recorded for each client, for two subjects with pinned revisions: S1 the foundry checkout at one commit SHA, S2 a clean adopter install from the v2.11.0 release artifact (annotated tag `v2.11.0`) using its documented loader setup. Revisions are fixed per subject (see Method), not shared across subjects. | Each client's own report or debug surface (Claude Code `InstructionsLoaded` log and startup report; Codex's equivalent; a third client if available), not the on-disk sum. Fresh sessions, at least 3 runs where the client's loading varies. Unit (characters or tokens) recorded as the client reports it. | A client exposes no report: record `not measurable`, never an estimate. |
| SDD-CTX-2 | assumption to test | Cost of a routine task set is recorded: (a) answer "where do Gate 1 plans live and what must they contain", (b) one `agov coord list` poll and summary, (c) one read-only MR status question. Every client runs all three tasks against both subjects, S1 and S2, so each client yields one ratio R per subject; no client picks a subject. Each task runs in its own fresh session, so no task inherits history. For each client, the same task is also run once in an empty directory with no project instructions (the bare baseline, one per client, shared by both subjects). | First-request input from the client's own usage surface, for the project run and the bare baseline, in the same client-reported unit. Cost ratio R = project first-request input divided by bare-baseline first-request input. | Usage surface absent or units differ between the two runs: `not assessable`. |
| SDD-UX-1 | assumption to test | A fresh agent session, given only the v2.11.0 release artifact and its adopter docs, reaches one valid operation on a defined fixture (see Method, S2 fixture). Setup time and operation time are recorded separately, with step counts and failed attempts. | Transcript counts; 3 runs; sanitized. Success is the fixture's stated pass condition. | Any step that needs undocumented knowledge is a finding, not a pass. |
| SDD-SUPPLY-1 | assumption to test | A descriptive audit of installed Ponytail (first) and, if time allows, one more plugin: manifest layout per client, skills, hooks, update path, permissions requested, provenance, copy-drift and version checks, benchmark methodology. | Read-only inspection of the installed package and primary specifications. No install or hook script is executed. File presence and vendor claims are not quality evidence; benchmark claims are recorded as vendor-stated unless reproduced. | A claim without a primary source is labeled `unverified`. |

### Future entry conditions (not executed in Phase A)

| ID | Label | Requirement |
|---|---|---|
| SDD-GATE-1 | decision | Zero missed irreversible-action gates (merge, tag, delete, release, security) in any profile experiment. One miss rejects that profile. |
| SDD-QUAL-1 | assumption to test | Task correctness and rework are not worse than baseline. |

These two bind a later A/B experiment, which does not exist yet. Phase A does not
claim to test them.

## Predeclared threshold: "material residual cost"

Set before any result is collected. Every numeric value below is a proposed value
until the governor approves it at Gate 1. Residual cost is material for a client
and subject when any assessable condition holds:

1. **Event:** the client emits a context or instruction warning or a hard-limit
   event at session start.
2. **Same-unit share:** effective loaded instructions are at least 75% of the
   client's documented instruction or context limit, only when the client reports
   both values in the same unit (or documents the conversion). Characters are
   never divided by a token limit, and no cross-unit ratio is computed.
   Rationale for 75%: it leaves 25% of the limit for the conversation, tool output
   and the model's own work, which a long agent session consumes. Sensitivity: the
   report also states the outcome at 50% and 90%; the decision uses 75%.
3. **Within-client cost ratio:** the CTX-2 ratio R (project first-request input over
   the bare baseline, same unit) is at least 2.0. This needs no documented limit.
   Sensitivity: the report also states the outcome at 1.5 and 3.0; the decision
   uses 2.0. Rationale: at R = 2.0 the project's instructions alone double the
   per-request input before the task begins.

A condition whose inputs are missing, mismatched in unit or undocumented is
`not assessable`. It is never counted as "not triggered". Outcomes:

- at least one assessable condition holds for any measured client and subject:
  `material residual cost`; the report recommends a separate Gate 1 plan for an
  A/B experiment and starts none;
- at least one condition is assessable for every measured client and none holds:
  `no profile experiment`; the workstream stops;
- otherwise (some client has no assessable condition): `inconclusive`; the report
  names the gap and returns to the governor.

## Method

- Pinned revisions, recorded in the report before the first run: S1 is one `main`
  commit SHA, the same for every client; S2 is the `v2.11.0` release artifact with
  its recorded SHA-256 checksum and tag commit, the same for every client. A later
  `main` advance does not change a run already started; a rerun on a new revision
  is a new, separately labeled run. Record client name, version, date, model, and
  a confidence note per figure.
- S2 fixture, fixed before any run. Setup boundary: a new empty git repository in
  a new directory, then the install and loader configuration that the v2.11.0
  adopter documentation prescribes (ADOPTING.md and the assistant-loader
  configuration page in the artifact), using the neutral codename
  `measure-fixture`; no foundry file, memory or prior session is available. The
  exact command sequence is transcribed from that documentation into the report
  before the first run and is not written from memory. Operation fixture: generate
  one coordination `note` with the documented ID tooling, then run the documented
  coordination validator on it. Pass: the validator exits 0 on that file. Fail:
  any other result, or a step that needs knowledge outside the documentation.
  Setup time is counted from first opening the documentation until the loader
  is configured; operation time from then until pass or fail; the two are reported
  separately.
- Collision evidence: after the final rebase, run the deterministic form pinned to
  the fetched remote target, `agov plan collision-check --plans-only --changed-only
  --base-ref origin/main --fail-on-unacknowledged-collision` (an unpinned run reads
  a possibly stale local `main` and can report collisions between unrelated older
  plans), and record its exit code and any stated incomplete historical coverage in
  the MR.
- Raw machine-local traces stay under `tmp/`. Nothing is committed from a trace
  until `agov validate infosec` and a read by an infosec-sentinel pass: no
  credentials, home paths beyond the repository, or cross-project identifiers.
- The report quotes figures and short excerpts only.
- Roles: planner-architect runs the Claude Code measurements; codex-foundry the
  Codex measurements; a third-client run by acs-server-dev is optional and its
  absence blocks nothing.
- No claim, tag, merge or release. The report is evidence, not a decision.

## Out of scope

Any change to `agf/rules/**`, loaders, `AGENTS.md`, `.claude/**`, schemas, tooling,
packaging, manifests or CI; generated profiles or a profile compiler; the
issue #138 item 2 and 3 work; choosing a version or release scope; editing the
files of the two depended-on 138 plans or of
`20261026T151646Z-next-release-scope-intake`.

## Classification and impact

NORMAL: a new plan and one report, no trigger-path file touched.
```
Impact:
  Foundry:          none; report only.
  Direct adopters:  none.
  Downstream:       none.
  Migration:        not required.
  Evidence:         issue #138 and #119; plans 20261022T205113Z and 20261023T080431Z; room seq 603-622.
```
Adopter docs parity: not applicable (no distributed behavior changes).
No normative guidance is introduced, so no rule-projection manifest applies.

## Deliverable

`docs/reviews/context-delivery-measurement/20261026T155400Z-context-delivery-measurement-report.md`
(qualified by this plan's ID so a later run cannot overwrite it; once merged it is
not edited, and a rerun needs a new plan or a governor-approved amendment): per requirement, the method,
figures with client/version/date, the threshold outcome, `not measurable` entries,
and a recommendation limited to `no profile experiment` or `propose a separate
Gate 1 for an A/B experiment`. The report states its evidence status: measured
facts are Empirically informed for the tested clients only; any adopter-benefit
claim stays Theoretical.

## Authority

Gate 1 and Gate 2 are `governor-reviewer` decisions. codex-foundry's delegated
authority ends 2026-10-26T16:00:00Z; unless re-delegated, the Human Governor
decides afterwards. Do not shorten review to fit that time.

## Risks and rollback

- R1: client reports differ in units (characters, tokens), so figures are not
  comparable. Mitigation: record units; compare each client to its own limit only.
- R2: a client changes its loading between runs. Mitigation: record version; at
  least 3 runs; report the range.
- R3: captured output leaks local data. Mitigation: the infosec step above.
- R4: the 75% threshold is arbitrary. Mitigation: it is predeclared and
  governor-approved at Gate 1, not tuned after results.
Rollback: delete the report; nothing else changes.
