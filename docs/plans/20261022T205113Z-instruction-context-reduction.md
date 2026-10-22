---
id: 20261022T205113Z-instruction-context-reduction
status: pending-gate1
topic: instruction-context-reduction-below-150k-chars
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: >
      Waived by the human governor in the live session on 2026-10-22 ("waive
      the auditor and infosec rows"), recorded in bridge note
      20261022T212922Z-planner-architect-rhqs. The plan changes only
      foundry-local loader files; it touches no credentials, learning records,
      schemas or validators. Provisional until a governor-reviewer record
      references the bridge note.
  - role: contribution-auditor
    status: waived
    reason: >
      Waived by the human governor in the live session on 2026-10-22, recorded
      in bridge note 20261022T212922Z-planner-architect-rhqs. Independent
      scrutiny already exists as official coder-tester review
      20261022T212401Z-coder-tester-epbp. That review is not an independent
      contribution audit. Provisional until a governor-reviewer record
      references the bridge note.
affected_paths:
  - AGENTS.md
  - .claude/CLAUDE.md
  - .claude/rules
  - .claude/settings.json
  - docs/reference/agent-cold-start-habits.md
  - docs/reference/foundry-polling-and-bridge-procedure.md
  - docs/reviews/rule-projection/20261022T205113Z-instruction-context-reduction.yaml
depends_on:
  - 20260913T085123Z-work-item-steward-gate1
  - 20260913T093405Z-row5-s3-priority-ordering-and-challenge-response
  - 20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion
  - 20260915T075705Z-issue130-pending-advisory-consultation-status
  - 20260921T040743Z-assistant-rule-drift-resolution
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: reduce always-loaded instruction context below 150k chars

Tracks issue #138 (work) and issue #119 (observation). Draft by
planner-architect for scrutiny by coder-tester (Codex) and the
contribution-auditor before Gate 1. Nothing here is approved.

## Decision requested

Approve a foundry-local reduction of the instruction text that Claude Code loads
in every foundry session, from 308.8k startup-report chars to **at most 140.0k
measured chars**, with no obligation dropped or weakened. Scope: Task 0 (done),
Phase 0 (exclude situational and stack-specific rules via `claudeMdExcludes`, add
an INDEX), and Phase 1 (slim `AGENTS.md`, keeping trigger lines for Codex). No
shipped `agf/rules/**` text is changed and no rule text is compressed. Because
this plan changes `AGENTS.md` and `.claude/**`, it is on the delegation red
list: only the governor decides its Gate 1 and Gate 2.

## Measured baseline (2026-10-22, main @ d4cc426a)

- Startup report: 57 instruction files, 308.8k chars, limit 150.0k.
- On-disk load set: `AGENTS.md`, `.claude/CLAUDE.md`, and every `.md` under
  `.claude/rules/**` (symlinks resolved): 57 files, 325,310 bytes.
  Bytes exceed chars because of multi-byte characters; chars are the
  acceptance unit, bytes are used below only for ranking.
- Cause: no file under `.claude/rules/` has `paths:` frontmatter, so every
  file loads unconditionally. 46 files are symlinks into `agf/rules/**`; 9 are
  foundry-local projections or pointers.
- Corrections to earlier assumptions: the canonical
  `agf/rules/workflow/simplicity-rule.md` (21k) and
  `universal-polling-and-autonomous-continuation-rule.md` (11k) are NOT in the
  load set. Only their small `.claude/rules` projections load (11.9k and 0.5k).

Top loaders (bytes): release-via-merge-request 40.3k, AGENTS.md 35.7k,
coordination-publication 22.9k, `.claude/rules` simplicity projection 11.9k,
branch-and-worktree 10.3k, deputy-governor 9.9k, documentation-rule-projection
9.2k, gate-self-test 8.8k.

## Constraints

1. No normative obligation is dropped, weakened, or silently re-owned.
   Authority model per `documentation-rule-projection-rule.md`: one owner per
   statement; every other surface points to it.
2. A rule moved off the always-load path stays discoverable: one trigger line
   in an always-loaded index, naming the situation and the file to open.
3. Shipped `agf/` text is not changed in Phase 0. Phases 1-2 change shipped or
   AI-loader text and are STRICT (`simplicity-rule.md`, `downstream-impact-rule.md`).
4. Existing path references must keep resolving. Measured by grep: only
   seven distinct `.claude/rules` paths are referenced by CI, validators, or
   the 30 projection manifests; `.gitlab-ci.yml:832-833` names `gate-self-test`
   and `release-via-merge-request`. The exclusion mechanism keeps every file and
   symlink on disk, so all references keep resolving.
5. The approved loader contract (plan `20260921T040743Z-assistant-rule-drift-resolution`,
   Gate 1 approved) chose symlinks so no copy can drift. This plan keeps every
   symlink and changes only what Claude Code loads (`claudeMdExcludes`).
6. Codex reads `AGENTS.md` natively but not `.claude/rules`. Anything moved out
   of `AGENTS.md` or off the `.claude/rules` path must stay reachable for Codex
   (see Phase 1).
7. Acceptance is measured, not asserted: the startup size report before and
   after.

## Design

### Mechanism (revised after Task 0)

Task 0 (below) found a mechanism that needs no stubs and leaves the approved
symlink contract untouched: Claude Code's `claudeMdExcludes` setting skips rule
files by glob, including files reached through a symlink, at any settings
layer. Excluded symlinks stay on disk, so every existing path reference keeps
resolving and `agov validate assistant_drift` sees the same tree as today.

| Tier | Meaning | Mechanism |
|---|---|---|
| A | Needed on nearly every task | Stays loaded: symlink, no exclusion |
| B | Needed in a recognizable situation | Excluded via `claudeMdExcludes` in `.claude/settings.json`; one trigger line in `.claude/rules/INDEX.md` says when to read the canonical file. Optional enhancement: a small path-scoped trigger stub (`paths:` frontmatter, loads when a matching file is read) |
| C | Not relevant to foundry's stack | Excluded via `claudeMdExcludes`; listed once in the INDEX |

Exclusion is foundry-local config. It does not change `agf/` text, does not
reach Codex (which reads neither `.claude/settings.json` nor `.claude/rules`),
and does not change what adopters receive. A stub, if used, is about 300 bytes,
carries no rule text, and grants no authority.

### Tier assignment (revised after Codex review, seq 50)

- Tier A (stays loaded; adds `external-anonymization` 7.3k, `branch-and-worktree`
  10.3k, `systematic-code-review` 7.3k per Codex finding 3): gate-self-test,
  high-risk-operations, downstream-impact, tool-authority-boundary,
  uncertainty-and-scope, security-hardening, verifiable-claims, answer-only,
  active-branch-preflight, coordination-* rules, debugging, tdd, destructive-op-preflight,
  agent-improvement-proposal, governance-file-layout, simplicity and
  development-workflow projections, and the small ones.
- Tier B (situational, 97.4k bytes excluded, each with an INDEX trigger line):
  `release-via-merge-request` 40.3k, `deputy-governor` 9.9k,
  `documentation-rule-projection` 9.2k, `deprecation-policy` 7.6k,
  `ci-trigger-scoping` 6.5k, `review-convergence` 5.9k,
  `environment-interrogation` 4.7k, `implementation-honesty` 3.8k, and the MR/release
  items `merge-request-description` 3.9k, `merge-request-workflow` 2.8k,
  `immutable-tags` 1.3k, `no-auto-package-update` 0.7k, `semver-no-carry` 0.9k.
  (`merge-request-workflow`'s core, "never commit to main", is also stated in
  `AGENTS.md` Branch discipline, which stays loaded.)
- Tier C (stack-specific, 60.9k bytes excluded): `visible-artifact-verification`,
  `concept-modelling`, `file-generator`, `migration-generation`,
  `audit-log-completeness`, `boundary-shape-testing`, `marketing-tone`,
  `accessibility`, `i18n-supported-locales`, `feature-flagged-docs`,
  `testing-environments`, `agf/rules/README.md`.

Arithmetic corrected after Codex finding 4: the earlier 85.8k for Tier B came from
a script key-matching bug (it missed `documentation-rule-projection`). Measured
load set: 325,310 bytes, 323,871 chars (Codex); the startup report says 308.8k
chars, so the report counts differently and is the acceptance unit (about 0.949
report-chars per byte).

Computed with the revised tiers (script over the real load set): Tier A rules
131.1k + `AGENTS.md`/`CLAUDE.md` 35.9k + INDEX 3k = about 170.0k bytes after
Phase 0, which is about 161k report-chars. Phase 0 alone does NOT reach 150.0k.
Phase 1 (`AGENTS.md` 35.7k to about 12k) gives about 146.2k bytes, about 139k
report-chars (about 146k at Codex's 0.9955 chars/byte). Codex asks for a measured
target of at most 140k chars for margin, so the acceptance target is **at most
140.0k startup-report chars**, measured; if Phase 1 lands above that, move
further clearly situational items to Tier B in a follow-up MR rather than
splitting shipped rules.

### Loader contract and drift validator

The approved drift plan (`20260921T040743Z-assistant-rule-drift-resolution`)
chose symlinks so no copy can drift. This plan keeps every symlink. The
exclusion setting changes what is loaded, not what is on disk, so
`agov validate assistant_drift` is unaffected (it reads files, not the setting;
verified in `assistant_drift.rs`: severity is Advisory, it compares file hashes
between `agf/rules` and `.claude/rules`). Only an optional trigger stub would add
one extra file; it would be an intentional foundry-local projection like
`improvement-propeller-rule.md`. No validator change is proposed.

### Compression (caveman rewrite): deferred

Terse rewriting of normative text is not part of this plan (Codex
recommendation, accepted). `AGENTS.md` content moves verbatim to reference docs
in Phase 1. A separate plan, with its own governor decision, may propose
equivalence-checked compression later (guards: modal and negation token multiset
unchanged; code spans, paths, IDs and numbers byte-identical; reviewer reads a
diff; originals kept in git history).

## Phases

### Task 0 — Spike (DONE 2026-10-22, from current Claude Code memory docs)

Source: `code.claude.com/docs/en/memory` fetched 2026-10-22; evidence class:
primary documentation, not yet confirmed by a `/context` run (see Phase 0 task 4).

- `paths:` frontmatter is supported on `.claude/rules/*.md`. A path-scoped rule
  loads when Claude uses Read, Write or Edit on a matching file (and on single-file
  `cat`/`head`), not when a situation arises. Rules without `paths` load at launch.
- `claudeMdExcludes` (any settings layer, arrays merge) skips CLAUDE.md and rules
  files by absolute-path glob; for symlinked rules a pattern may match the link
  path under `.claude/rules/` or the target.
- Combined instruction size is warned at startup; each rule file counts separately.
- `InstructionsLoaded` hooks log which instruction files load and why. `/doctor
  prompt-audit` reports stale or conflicting instruction content.
- Drift validator: advisory, hash comparison of the two trees; unaffected by the
  setting.

Consequences: Tier B/C exclusion via `claudeMdExcludes` replaces the
stub-per-path design; a `paths:` trigger stub stays optional. Canonical files do
not need `paths:` frontmatter (a shipped-text change we avoid in Phase 0).

### Phase 0 — Foundry-local tiering (no shipped text changed, no files removed)

Tasks, in order:
1. Add `.claude/rules/INDEX.md` (always loaded, about 3k): one line per excluded
   rule, "when X, read Y".
2. Add `claudeMdExcludes` globs to `.claude/settings.json` for the Tier B/C
   files. No symlink is removed.
3. Obligation coverage matrix `docs/reviews/rule-projection/20261022T205113Z-instruction-context-reduction.yaml`:
   per excluded rule, the owner path, the INDEX trigger line, and the surface
   that still loads it. Run `agov plan projection-check` on it.
4. Measure: startup size report before and after, plus an `InstructionsLoaded`
   log of the loaded set, pasted in the MR. This also confirms the documentation
   claims from Task 0 on the installed Claude Code version.

Done: measured startup report plus the `InstructionsLoaded` log. Computed
estimate in "Tier assignment" above: about 170k bytes (about 161k report-chars).
Phase 0 alone does not reach the target. Depends on approval.

### Phase 1 — Slim `AGENTS.md` (35.7k to about 12k)

Codex reads `AGENTS.md` natively but not `.claude/rules`, so `AGENTS.md` is
Codex's only always-loaded instruction file. Before moving anything, mark which
`AGENTS.md` obligations Codex needs on every task (gate discipline, bridge
records, coordination visibility, the polling contract summary) and keep those
in `AGENTS.md`. For every Tier B/C rule Codex may need, `AGENTS.md` keeps a
compact trigger line (Codex finding 1), because Codex never sees `.claude/rules`
or the INDEX. Only reference material (build commands, telemetry paths, full
command list) moves unconditionally, verbatim.

Move the polling procedure, bridge-record procedure, build/telemetry
reference, and command lists to
`docs/reference/foundry-polling-and-bridge-procedure.md` and the existing
`docs/reference/agent-cold-start-habits.md`. `AGENTS.md` keeps: project
identity, layout, active agents, gate discipline summary, and pointers.
Authority note: the polling procedure's owner is
`agf/rules/always/universal-polling-and-autonomous-continuation-rule.md`; the
foundry stricter projection moves to the reference doc and says so. Text moves
verbatim; no rewriting. Depends on Phase 0.

### Phase 2 — Split shipped rules: removed from this plan

Splitting `agf/rules/**` changes shipped text and high-risk areas. It is out of
scope here (Codex recommendation, accepted) and would need its own Gate 1 plan,
impact statement and adopter-docs-parity note.

### Phase 3 — Advisory instruction-budget report (separate plan)

Out of scope here. If proposed later, it is advisory-only; any blocking use
needs a named rule and a negative test per `gate-self-test-rule.md`.

## Classification and impact

- Mode: STRICT. `AGENTS.md` and `.claude/**` are AI loaders and a downstream-impact
  trigger path (`AGENTS.md`); the change alters what every session reads.
  Delegation red list applies (see Decision requested).
- Impact:
  - Foundry: yes, every Claude Code session sees a smaller prompt; Codex's
    `AGENTS.md` shrinks but keeps trigger lines. Main hazard: a missed obligation (R1).
  - Direct adopters: none. `agf/` and shipped templates are unchanged.
  - Downstream: none.
  - Migration: not required.
  - Evidence: issue #119 measurements and the 2026-10-22 startup report.
- Adopter docs parity: not applicable (foundry-local loader files; nothing in
  `agf/` or shipped templates changes).

## Test strategy

1. Projection check on the new manifest: passes (every moved rule has owner,
   trigger, surface).
2. Negative tests, per `gate-self-test-rule.md` where a check can block:
   - A manifest row whose owner path does not exist makes
     `agov plan projection-check` fail (known-bad input: point one row at a
     missing file in a scratch worktree and show the exit code).
   - A Tier B rule with no INDEX trigger line is reported by a one-off
     coverage script run in the MR (script not committed unless approved).
3. Size: startup report before and after, same machine, same commit base.
4. Behavior: replay three representative tasks (a coordination record, a plan,
   a release dry-run read) and confirm the agent opens the right rule. Record
   the transcript. This is evidence, not a gate.
5. Verbatim check: each block moved out of `AGENTS.md` is byte-identical in its
   new home (show a diff of the moved text against the original).

## Risks and mitigations

- R1: agents do not open a Tier B rule when needed. Mitigate: trigger lines
  in an always-loaded index; `paths:` stubs if supported; behavior replay;
  keep safety-critical small rules in Tier A.
- R2: moving `external-anonymization` off always-load risks an irreversible
  leak. Resolved (Codex finding 3, accepted): it stays Tier A, as do
  `branch-and-worktree` and `systematic-code-review`.
- R3: an obligation is lost in Phase 1 restructuring. Mitigate: coverage
  matrix; diff review of `AGENTS.md` by Codex; text moves verbatim.
- R4: `claudeMdExcludes` globs drift out of sync with the rule tree (a new rule
  is added and loads, or a renamed rule is silently not excluded). Mitigate: the
  coverage script and the `InstructionsLoaded` log in each MR; re-run on rule changes.
- R5: the advisory `assistant_drift` result changes. With exclusion, the file
  tree is unchanged, so no new drift findings are expected; the MR records the
  before/after validator output (Codex finding 2 accepted: any change is advisory
  and summarized after 8 findings).

## Rollback

Phase 0: revert the MR (removes the `claudeMdExcludes` entries and the INDEX).
Phase 1: revert the MR; text returns to `AGENTS.md`. No tag, release, or
immutable record is involved.

## Review adjudication (official review `20261022T212401Z-coder-tester-epbp`, MR !3117; ACS seq 50; reviewed head 5f9b9ea7)

epbp recommended BLOCK of the reviewed text with three blocking findings (F1 safety-critical rules, F2 arithmetic margin, F3 scope) and two advisories (A1 Codex trigger index, A2 expected advisory drift). This revision addresses all five.

1. Codex loads `AGENTS.md` only: accepted; Phase 1 keeps trigger lines.
2. Validators: accepted. `assistant_loader` ignores `.claude/rules`; CI and
   `check-live-tooling-surfaces` hard-code `gate-self-test` and
   `release-via-merge-request` paths, which keep resolving because no file is removed.
3. Tier safety: accepted. `external-anonymization`, `branch-and-worktree` and
   `systematic-code-review` stay A; five MR/release items join B.
4. Arithmetic: accepted; my Tier B figure had a script bug (corrected above). Target
   changed to at most 140.0k measured chars.
5. Scope (epbp F3): accepted. No shipped split, no normative compression in this plan.
6. A1: accepted; `AGENTS.md` keeps Codex trigger lines (Phase 1). A2: accepted; the
   MR records `agov validate` before/after output and does not claim a clean
   umbrella validation. With exclusion no new drift findings are expected, but the
   existing summarized `assistant_drift` advisory (46 differ, 7 missing, 2 extra
   at the last count) stays and is not claimed fixed.

## Open questions for review (contribution-auditor, governor)

1. Do the Task 0 documentation claims (`claudeMdExcludes` through symlinks,
   `paths:` loading) hold on the installed Claude Code version? Phase 0 task 4
   measures this.
2. Is a measured target of at most 140.0k chars right, or should it be the
   limit (150.0k) with Phase 1 as the only AGENTS.md change?
3. Should the caveman-compression follow-up be planned now or after Phase 1 lands?

## Plan collisions acknowledged

`agov plan collision-check` reports advisory overlaps with five earlier plans
(listed in `depends_on`), almost certainly on `AGENTS.md` and `.claude/rules`.
Those plans edit different sections or rules; this plan only moves or shortens
loader text. Phase 1 must re-run the check and rebase on any of them still
open when it starts.

## Out of scope

Deleting rules; changing gate or role authority; any gate mechanics; the
instruction-budget report (Phase 3); skills or agent-definition changes.
