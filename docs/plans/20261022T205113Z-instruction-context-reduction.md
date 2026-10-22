---
id: 20261022T205113Z-instruction-context-reduction
status: pending-gate1
topic: instruction-context-reduction-below-150k-chars
advisory_consultation:
  - role: infosec-sentinel
    status: pending
  - role: contribution-auditor
    status: pending
affected_paths:
  - AGENTS.md
  - .claude/CLAUDE.md
  - .claude/rules
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

Approve a phased reduction of the instruction text that Claude Code loads in
every foundry session, from 308.8k chars (startup report) to under 150.0k
chars, with no obligation dropped or weakened. Phase 0 (foundry-local, no
shipped text changed) is separable and can be approved alone.

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
   the 30 projection manifests (`coordination-publication`, `simplicity`,
   `release-via-merge-request`, `gate-self-test`, `development-workflow`,
   `branch-and-worktree`, and the two foundry pointers); `.claude/rules/foo.md`
   is a test fixture. `.gitlab-ci.yml:832-833` names `gate-self-test` and
   `release-via-merge-request`. A file MUST exist at each of those paths. Every
   other `.claude/rules` link can be removed with no stub.
5. The approved loader contract (plan `20260921T040743Z-assistant-rule-drift-resolution`,
   Gate 1 approved) chose symlinks so that no copy can drift, and
   `agov validate assistant_drift` (`tooling/rust/crates/agov-validators/src/assistant_drift.rs`)
   compares `.claude/rules` with `agf/rules`. Replacing a symlink with a stub
   changes that contract. See Design, "Loader contract and drift validator".
6. Codex reads `AGENTS.md` natively but not `.claude/rules`. Anything moved out
   of `AGENTS.md` or off the `.claude/rules` path must stay reachable for Codex
   (see Phase 1).
7. Acceptance is measured, not asserted: the startup size report before and
   after.

## Design

### Tiers

| Tier | Meaning | Mechanism |
|---|---|---|
| A | Needed on nearly every task | Stays a symlink to the canonical rule |
| B | Needed in a recognizable situation | Foundry-local stub at the same path, with `paths:` frontmatter if supported, plus one trigger line in the index |
| C | Not relevant to foundry's stack | Stub or index entry only; canonical file still ships in `agf/` |

Only seven paths must keep a file (Constraint 4). Every other Tier B/C link is
simply removed and listed in the INDEX, so Phase 0 adds about 7 stubs, not 20.
A stub is about 300 bytes: a title, "Canonical owner: `agf/rules/...`", the
trigger sentence, and an instruction to read the canonical file. It adds no
rule text and grants no authority (same pattern as the existing
`.claude/rules/always/improvement-propeller-rule.md` projection).

### Proposed tier assignment (bytes, for ranking; for scrutiny, not final)

- Tier B (situational, 85.8k bytes removed from always-load):
  `release-via-merge-request` 40.3k (release tasks only), `deputy-governor`
  9.9k (delegation is off by default), `documentation-rule-projection` 9.2k
  (plans touching Pages/runbooks), `deprecation-policy` 7.6k,
  `ci-trigger-scoping` 6.5k, `review-convergence` 5.9k,
  `environment-interrogation` 4.7k, `implementation-honesty` 3.8k,
  `external-anonymization` 7.3k (see risk R2).
- Tier C (stack-specific, 60.9k bytes removed): `visible-artifact-verification`,
  `concept-modelling`, `file-generator`, `migration-generation`,
  `audit-log-completeness`, `boundary-shape-testing`, `marketing-tone`,
  `accessibility`, `i18n-supported-locales`, `feature-flagged-docs`,
  `testing-environments`, `agf/rules/README.md`.
- Tier A (kept): gate-self-test, high-risk-operations, downstream-impact,
  tool-authority-boundary, uncertainty-and-scope, security-hardening,
  verifiable-claims, answer-only, active-branch-preflight,
  coordination-* rules, systematic-code-review, branch-and-worktree,
  merge-request-*, immutable-tags, tdd, debugging, and the small ones.

Computed Phase 0 result (script over the real load set, Tier B 85.8k + Tier C
60.9k bytes removed, 3k INDEX and 7 stubs added): about 189k bytes, about 179k
chars at the observed 0.949 chars/byte ratio. **Phase 0 alone does NOT reach
150.0k chars.** Phase 1 (`AGENTS.md` 35.7k to about 12k) brings it to about
157k chars. Phase 2 (or moving more Tier A rules to Tier B, for example
`branch-and-worktree` 10.3k and `systematic-code-review` 7.3k) is needed to
get below 150.0k. Phase 0 acceptance replaces these estimates with the measured
startup report.

### Loader contract and drift validator

The approved drift plan chose symlinks for one reason: one file on disk means
no drift. Stubs are different files, so `agov validate assistant_drift` will
report each stub as "differ". Options for Gate 1 to choose between:
(a) accept the advisory WARN for the seven stubs and record them as intentional
foundry-local projections, as `improvement-propeller-rule.md` already is;
(b) teach the validator an explicit "projection stub" class. (b) changes a
validator, so it is gate mechanics under `gate-self-test-rule.md` and needs its
own negative test. This plan proposes (a) and treats (b) as a separate plan.
Task 0 confirms the validator's current treatment of existing projections.

### Compression lever (caveman rewrite)

Rewriting verbose text in terse form (the `caveman-compress` skill, which
keeps a readable backup) can cut size further. It is allowed only under these
guards, because normative text is meaning-sensitive:

- Scope: foundry-local files first (`AGENTS.md`, `.claude/rules` projections,
  new reference docs). Shipped `agf/rules/**` only after a separate governor
  decision, because adopters read those files.
- Equivalence check per rewritten file, recorded in the MR: the multiset of
  modal and negation tokens (MUST, MUST NOT, SHOULD, never, not, only, except,
  forbidden, blocking) is unchanged; every code span, path, command, ID, number
  and quoted error is byte-identical; the reviewer reads a diff.
- The original stays in git history and as a backup outside the load path.
- Persisted rule text stays plain prose (terse, not broken grammar).

## Phases

### Task 0 — Spike: does `paths:` frontmatter load a stub on demand?

Files: none changed; throwaway stub under `.claude/rules/` in a scratch
worktree. Done: documented result (supported or not, matching semantics, and
whether a stub for an unread path loads zero bytes). Source: current Claude
Code documentation and a `/context` run. If `paths:` is unsupported or only
fires on file reads, Tier B falls back to index-only discovery (trigger lines),
and the risk R1 mitigation changes. Depends on: nothing.

### Phase 0 — Foundry-local tiering (no shipped text changed)

Tasks, in order:
1. Add `.claude/rules/INDEX.md` (always loaded, about 3k): one line per rule
   not in Tier A, "when X, read Y". Depends on Task 0.
2. Remove each Tier B/C symlink. Where a path is referenced (Constraint 4) replace
   the link with a stub; otherwise leave no file.
3. Obligation coverage matrix `docs/reviews/rule-projection/20261022T205113Z-instruction-context-reduction.yaml`:
   per moved rule, the owner path, the trigger line, and the surface that
   still loads it. Run `agov plan projection-check` on it.
4. Measure: startup size report, before and after, pasted in the MR.

Done: report under 150.0k chars if Phase 0 alone suffices; otherwise the
measured remainder drives Phases 1-2. Depends on Task 0 and approval.

### Phase 1 — Slim `AGENTS.md` (35.7k to about 12k)

Codex reads `AGENTS.md` natively but not `.claude/rules`, so `AGENTS.md` is
Codex's only always-loaded instruction file. Before moving anything, mark which
`AGENTS.md` obligations Codex needs on every task (gate discipline, bridge
records, coordination visibility, the polling contract summary) and keep those
in `AGENTS.md` in compressed form. Only reference material (build commands,
telemetry paths, full command list) moves unconditionally.

Move the polling procedure, bridge-record procedure, build/telemetry
reference, and command lists to
`docs/reference/foundry-polling-and-bridge-procedure.md` and the existing
`docs/reference/agent-cold-start-habits.md`. `AGENTS.md` keeps: project
identity, layout, active agents, gate discipline summary, and pointers.
Authority note: the polling procedure's owner is
`agf/rules/always/universal-polling-and-autonomous-continuation-rule.md`; the
foundry stricter projection moves to the reference doc and says so. Caveman
rewrite applies to the moved text under the guards above. Depends on Phase 0.

### Phase 2 — Split the large shipped rules (only if still over limit)

Candidates: `release-via-merge-request-rule.md` (move the two exception
procedures to a guide, keep the standard workflow and namespace check),
`coordination-publication-rule.md` (move hold, session, and channel-discovery
sections to a guide), and the `.claude/rules` simplicity projection (keep the
mode classification, move the sources/radar fast path). These change shipped
text. Each needs the downstream-impact statement, an adopter-docs-parity note,
and, for `release-via-merge-request-rule.md`, a plan stop because
`high-risk-operations-rule.md` names release mechanics. Depends on Phase 0
measurement and a separate governor decision.

### Phase 3 — Advisory instruction-budget report (separate plan)

Out of scope here. If proposed later, it is advisory-only; any blocking use
needs a named rule and a negative test per `gate-self-test-rule.md`.

## Classification and impact

- Mode: STRICT for Phases 1-2 (shipped rule text, AI loaders, high-risk
  area). Phase 0 touches only foundry-local loader files; still planned here
  because it changes what every agent session reads.
- Impact:
  - Foundry: yes, every session sees a smaller prompt. Risk of missed
    obligations is the main hazard (R1).
  - Direct adopters: none for Phase 0; ADDITIVE/EDITORIAL for Phase 2.
  - Downstream: none.
  - Migration: not required for Phase 0; Phase 2 needs a note if wording moves.
  - Evidence: issue #119 measurements and the 2026-10-22 startup report.
- Adopter docs parity: not applicable for Phase 0 (foundry-local loader);
  updated or not applicable per file for Phase 2.

## Test strategy

1. Projection check on the new manifest: passes (every moved rule has owner,
   trigger, surface).
2. Negative tests, per `gate-self-test-rule.md` where a check can block:
   - A stub whose pointer target does not exist makes
     `agov plan projection-check` fail (known-bad input: delete one target in
     a scratch worktree and show the exit code).
   - A Tier B rule with no INDEX trigger line is reported by a one-off
     coverage script run in the MR (script not committed unless approved).
3. Size: startup report before and after, same machine, same commit base.
4. Behavior: replay three representative tasks (a coordination record, a plan,
   a release dry-run read) and confirm the agent opens the right rule. Record
   the transcript. This is evidence, not a gate.
5. Caveman guard: for each rewritten file, show the token-multiset diff is empty.

## Risks and mitigations

- R1: agents do not open a Tier B rule when needed. Mitigate: trigger lines
  in an always-loaded index; `paths:` stubs if supported; behavior replay;
  keep safety-critical small rules in Tier A.
- R2: moving `external-anonymization` off always-load risks an irreversible
  leak (publication cannot be undone). Mitigate: keep it Tier A or give it a
  mandatory trigger line naming every external destination. For scrutiny.
- R3: an obligation is lost in Phase 1 restructuring. Mitigate: coverage
  matrix; diff review of `AGENTS.md` by Codex; move text verbatim before any
  compression.
- R4: stub drift from the canonical file. Mitigate: stubs carry no rule text.
- R5: shipped-text change surprises adopters. Mitigate: Phase 2 is a separate
  decision with an impact statement.
- R6: compression changes meaning. Mitigate: the equivalence check and a
  human diff read; shipped rules excluded until separately approved.

## Rollback

Phase 0: revert the MR; symlinks are restored by git. Phase 1: revert the MR;
text returns to `AGENTS.md`. Phase 2: gated revert as for any shipped rule
change. No tag, release, or immutable record is involved.

## Open questions for review (Codex, contribution-auditor)

1. Does `paths:` frontmatter work on stubs in the installed Claude Code
   version? Does Codex load `.claude/rules` at all (the earlier cost-audit
   addendum says it does not natively)? If not, which surface does Codex read,
   and does it need its own index?
2. Is Tier assignment right, especially R2 and `branch-and-worktree` (10k,
   frequent use)?
3. Phase 0 alone reaches about 179k chars, not 150.0k (computed above). Is the
   Phase 0 + 1 + selected Tier A demotions combination acceptable, or is Phase
   2 (splitting shipped rules) wanted?
4. Does `agov validate assistant_loader` or any validator assume specific files
   under `.claude/rules`? (Grep found `.gitlab-ci.yml:832-833` and projection
   manifests only; confirm no Rust reference.)
5. Is the caveman guard strong enough, or should compression of shipped rules
   be ruled out entirely?

## Plan collisions acknowledged

`agov plan collision-check` reports advisory overlaps with five earlier plans
(listed in `depends_on`), almost certainly on `AGENTS.md` and `.claude/rules`.
Those plans edit different sections or rules; this plan only moves or shortens
loader text. Phase 1 must re-run the check and rebase on any of them still
open when it starts.

## Out of scope

Deleting rules; changing gate or role authority; any gate mechanics; the
instruction-budget report (Phase 3); skills or agent-definition changes.
