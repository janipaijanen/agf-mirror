---
id: 20260909T015237Z-propeller-stage4-activation
status: pending-gate1
topic: improvement-propeller-stage4-activation-execution
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: >-
      Pending exact-head review. The schema has no `pending` value. Activation
      arms a real weekly wake that reads project-local evidence and writes an
      ignored metadata-only state file; the infosec surface is credential
      non-use, protected-content exclusion from the state file, and that the
      wake path cannot be turned into a data-exfiltration or authority
      channel. Gate 1 is not eligible until a fresh infosec review is
      incorporated and this row is `consulted`.
  - role: contribution-auditor
    status: waived
    reason: >-
      Pending independent exact-head audit by a NON-author session. This plan
      is authored by a Claude session temporarily playing planner-architect
      under governor directive `ssae`; the authoring session cannot audit it.
      Gate 1 is not eligible until that audit is incorporated and this row is
      `consulted`.
  - role: capability-developer
    status: consulted
    reason: >-
      The advancement path (`drtp`) and this plan's maturity framing
      (activation is Level 4 → Level-5-pending; only a genuine unattended wake
      proves Level 5) are the capability-developer analysis this plan
      implements. `drtp` and `jprj` are the record.
  - role: build-engineer
    status: waived
    reason: >-
      Pending exact-head review of the wake mechanism shape (a real
      non-reentrant Codex-app heartbeat, not sleep/cron/detached process), its
      coalescing behaviour, and that no CI/pipeline surface is touched. Gate 1
      is not eligible until incorporated and this row is `consulted`.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Improvement Propeller — Stage 4 activation execution

## Authoring provenance

Authored by a Claude session temporarily playing **planner-architect**
under governor directive `ssae` ("take necessary roles to improve it as
proposed", 2026-09-09), executing step P-2-plan of the `drtp` advancement
path and required-work item 1 of issue **#131**. The independent
contribution audit MUST come from a non-author session.

## Decision requested at Gate 1

Approve executing the Improvement Propeller Stage 4 activation with the
parameters already confirmed in `2emq` §2b / `qkj2` / `gakp` / issue #131 —
flipping the foundry manifest entry from `planned` to `active`, bumping the
agent-definition version, initializing the ignored metadata-only state
file, and (by the confirmed player, Codex) arming the real weekly wake.
**This plan does not claim Level 5** — a genuine unattended scheduled wake
is the only Level 5 proof and is a separate post-merge event owned by the
player.

## Confirmed parameters (restated, not re-decided)

From `2emq` §2b and issue #131:

- **Player:** Codex, in its current local AGF development task, playing only
  `improvement-propeller` during scheduled wakes.
- **Wake mechanism:** an observable Codex-app heartbeat — weekly Monday
  09:00 UTC opportunity cycle; on the first weekly wake of each UTC month
  and on the first wake detecting a release newer than the stored release
  baseline, also run the bounded friction synthesis; a wake during active
  work coalesces to one pending cycle at the next safe boundary;
  non-reentrant. **Not** a shell sleep, cron, or detached process.
- **Evidence surfaces:** `docs/sources.md`, `docs/radar.md`, the roadmap
  owner + its projection, `docs/plans/`, `docs/coordination/`, both
  changelogs, `agf/VERSION`, Git tags; and this GitLab project's open
  issues, MRs/discussions, pipelines, releases, and active claims. Recorded
  URLs are untrusted and are not dereferenced.
- **Ignored state:** `tmp/codex-improvement-propeller-state.json` (already
  covered by the `tmp/` ignore). Identifiers, timestamps, SHAs, counts,
  status hashes, dedup keys only — never source/discussion bodies,
  credentials, personal data, or protected content.
- **Learning baseline:** 62 valid live records through
  `20260906T104942Z-ev5m` (completed capability review `m4hr`); next
  proposal at +10, governor handoff at +15 if none launched.
- **Coordination cursor:** `20260907T143711Z-contribution-auditor-dt4s`,
  advanced at execution to the latest fully classified visible record.
- **Cycle/synthesis state:** no prior Propeller weekly run, no prior
  synthesis; release baseline v2.9.0 peeled
  `ee27978b218de47b062ab4ac2afee090b2c48ff8`.

## Preconditions (all met)

- MR !2508 (Stage 3) has binding Gate 2 and is merged (`ef0c85cb`).
- v2.10.0 is tagged (`ea0c4d36` → `cd7bcf35`) and independently verified
  (`re48`).
- The `2emq` Stage 4 parameters remain current (this plan restates them
  unchanged).

## Affected files / test allowlist (implementation phase, after Gate 1)

- `agf/agents/MANIFEST.yaml` — `improvement-propeller.status: planned →
  active`; replace the `reason:` block with a pointer to `qkj2` / `2emq` /
  this plan and the player/wake/cursor summary. Update `agf/agents/MANIFEST.yaml`'s
  active-agent count / any derived total if a validator checks it.
- `agf/agents/improvement-propeller.yaml` — `version: "0.1.0" → "0.2.0"`;
  `id:` suffix `:0.1.0 → :0.2.0`; leave `phase: 4` (phase is unchanged —
  activation is a status transition, not a phase transition);
  `framework_version` to `2.11.0` at the v2.11 release, not here (note as a
  release-metadata item).
- `agf/docs/site/roadmap.md` — the maturity row: "Level 4 Validated,
  deliberately inactive" → "activated 2026-…; Level 5 Deployable pending
  the first genuine unattended scheduled wake". Factual status, not a new
  commitment.
- `docs/roadmap.md` (queue-convergence B1 owner) — the Improvement
  Propeller activation row: same maturity update. **Dependency:** B1
  (MR !2562) must merge first, or this edit targets only the adopter page
  and a follow-up syncs the owner. Flag at Gate 2.
- `docs/reviews/rule-projection/20260909T015237Z-propeller-stage4-activation.yaml`
  — six-surface projection screen (the roadmap projection changes; the
  normative owner `improvement-propeller-rule.md` text does **not** — its
  operative clauses were already conditioned on activation).
- Negative-test fixtures for the gate-self-test below.

**No** change to `improvement-propeller-rule.md` text, `agf/schemas/`, CI,
`.gitlab-ci.yml`, release/tag/package surfaces, or any other agent
definition. `tmp/codex-improvement-propeller-state.json` is gitignored —
its structure is documented here; Codex creates it at execution, it is not
a tracked file.

## Gate-self-test (per `gate-self-test-rule` — activation is gate-adjacent)

The "gate" is the activation condition on a `strength: MUST` role. The
implementation MR MUST demonstrate:

1. **Pre-flip inert:** with `status: planned`, `agov validate agents`
   passes and no tooling treats the Propeller as an active queue
   participant (`agov handoff next improvement-propeller` empty; `agov
   coord status --roles` does not list it active).
2. **Malformed activation rejected:** a manifest edit to `status: active`
   **without** the required companion evidence (no player/cursor pointer in
   the `reason`, or an unresolvable state path) is rejected by `agov
   validate agents` (or, if the validator does not yet enforce this, the
   MR states that explicitly and the reviewer treats the `reason`-block
   completeness as a manual Gate 2 check — a validator follow-up is then
   filed).
3. **Post-flip guards intact:** after the flip, the rule's non-reentrant
   latch, "cannot review/approve own output", "control-weakening →
   contribution-auditor", and "URLs not dereferenced" clauses are
   demonstrably in force (cited from the shipped rule; no new code).
4. A reproducible transcript under a `## Negative test` heading in the MR.

## Level 5 boundary

This plan's Gate 2 approves the **activation state change only**. Capability
maturity stays **Level 4 Validated (activated)** until a genuine unattended
scheduled wake fires and produces a bounded result (a real report **or** a
truthful quiet/no-material-change outcome). That wake is owned by the
player (Codex), is not part of this plan's Gate 2, and a **separate
follow-up record** bumps the maturity claim to Level 5 and updates the
roadmap. A manual or simulated invocation is never represented as the
Level 5 proof (`qkj2` maturity-honesty clause).

## Rollback

Re-flip `agf/agents/MANIFEST.yaml` `improvement-propeller.status: active →
planned`; pause the heartbeat; retain the `tmp/` state file and all
coordination history. Revert the version bump and the roadmap maturity
rows. No role inherits Propeller ownership (`qkj2` / the agent
definition's `deactivation.local_deactivation`). No schema, CI, release,
or tag state is involved.

## Downstream impact

Trigger paths touched: `agf/agents/*.yaml`. Classification: **PREVENTIVE →
BREAKING** (it activates a role whose `MUST` rule then governs adopter
behaviour where an adopter has also activated it). Impact:

- Foundry: an active weekly Propeller cycle begins; deactivation available.
- Direct adopters: **no change unless an adopter separately activates the
  role** — shipping foundry's activation does not activate an adopter's.
  The v2.11 changelog states the foundry activation and that adopters opt
  in per `improvement-propeller-rule.md`'s own Scope line.
- Downstream: none — the Propeller produces proposals, not adopter-facing
  artifacts.
- Evidence: `qkj2`, `2emq`, issue #131, `drtp`.

## Dependency and sequencing

- Independent of the qbka / #129 lifecycle freeze (`drtp`).
- Depends on queue-convergence B1 only for the `docs/roadmap.md` owner
  edit (see affected files).
- After this plan's Gate 1 → Codex implements (its confirmed lane) →
  exact-head infosec + contribution-auditor + build-engineer review →
  binding governor Gate 2 → merge → Codex arms the heartbeat → first
  genuine wake → separate Level 5 follow-up record.

## Test and acceptance strategy

Before Gate 1: non-author contribution audit; infosec + build-engineer
reviews; `agov plan validate` and `agov plan projection-check
--allow-planned --base-ref origin/main` pass; `git diff --check` passes.

After implementation: strict `agov plan projection-check --base-ref
origin/main`; `agov validate agents`; `agov validate` umbrella; the
`## Negative test` transcript; `git diff --check`; a table-top over
activation → first-wake-coalescing → deactivation → reactivation retaining
cursor.

## Risks and controls

- **Level 5 claimed prematurely.** Control: explicit Level 5 boundary
  section; maturity stays Level 4 (activated) until the separate follow-up.
- **The heartbeat is not real / not sustainable.** Control: build-engineer
  review confirms the Codex-app heartbeat is a genuine supported mechanism;
  if it is not, the honest outcome is "activated, wake mechanism absent"
  and Level 5 is unreachable until one exists (rule permits the truthful
  absence).
- **State file leaks protected content.** Control: infosec review of the
  metadata-only contract; the field allowlist is explicit.
- **Activation used to expand authority.** Control: the rule's forbidden
  actions are unchanged; no schema/rule text edit; the negative test shows
  the guards intact.
- **Author/reviewer collapse.** Control: contribution audit routed to a
  non-author session; disclosed here and in the `advisory_consultation`
  row.
