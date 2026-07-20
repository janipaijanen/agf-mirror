---
kind: review
from: governor-reviewer
to: all
id: 20260720T080808Z-governor-reviewer-mrp3
timestamp: 2026-07-20T08:08:08Z
immutable: true
references-to: []
topic: gate1-plan-cluster-reconciliation
requires-ack: true
artifacts_owned: []
---

# Reconciliation: seven parallel Gate 1 plan clusters (issues #17-#49)

## Scope

23 open issues (#17-#49) were split into seven independent clusters, each
planned in an isolated worktree by a separate agent instance, Gate 1
planning only (no implementation). All seven are back. This note is the
cross-cluster reconciliation pass: consistency against each other, against
existing implementation, and against the framework's own core principles.

None of the seven branches have been pushed or merged. This note records
findings; it does not itself authorize any merge.

## Branches and plan files

| Cluster | Branch | Issues | Plan files |
|---|---|---|---|
| Identity/accountability/gate-enforcement | `plan/identity-accountability-gate-enforcement` | #35, #45, #34, #47 | `identity-accountability-gate-enforcement.md` |
| Coordination CLI + CI distribution | `plan/coord-cli-ci-distribution` | #33, #32, #36, #37, #18 | `coord-channel-concurrency-semantics.md`, `coord-cli-write-and-watch.md`, `agov-mcp-server.md`, `agov-oci-image.md` |
| Incident + amendment governance | `plan/incident-amendment-governance` | #46, #43 | `governance-amendment-workflow.md`, `incident-workflow-cast.md` |
| Compliance-export + buyer-side | `plan/compliance-export-buyer-side` | #40, #41, #44 | `compliance-export-buyer-side-agf.md`, `cra-entity-structure-decision.md` |
| Threat model + governance primitives | `plan/threat-model-governance-primitives` | #42, #48, #49 | `threat-model-stpa-owasp.md`, `governance-time-budget-primitives.md`, `input-provenance-manifest.md` |
| Small fixes + docs polish | `plan/misc-small-fixes-batch` | #30, #31, #19, #17, #38 | `plan-validator-advisory-role-allowlist.md`, `gitlab-issue-templates-improvement-bug.md`, `learning-validator-external-review-status.md`, `external-reviewer-fit-assessment-proposals.md`, `multi-agent-coordination-experiment.md` |
| Removability + exit guarantee | `plan/removability-exit-guarantee` | #39 | `issue-39-removability-exit-guarantee.md` |

16 plan files total, all validated against `agf/schemas/gate1-plan.schema.json`.

## Finding 1 — naming conflict, resolvable (accountable-human field)

Three clusters independently touch "who is the named accountable human":

- AB defined `accountable_human` on a new `gate_approval` coordination-message
  object, and separately added `owner` (same concept) to
  `agf/schemas/rule.schema.json` — flagging that asymmetry itself.
- IA assumed a different shape: `proposed_by` / `approved_by`, two plain
  strings on ADRs and decision-records.

**Recommendation:** collapse to one canonical field name, `accountable_human`,
reused verbatim in `gate_approval`, `rule.schema.json` (replacing `owner`),
and IA's ADR/decision-record docs in place of `approved_by`. IA's
`proposed_by` is redundant with the coordination message's existing required
`from` field (the plan/ADR author already is the proposer) — drop it rather
than add a new field. This is a wording-level fix in both clusters' plan
files, not a redesign.

## Finding 2 — two questions deliberately left open, need a decision

- **IA**: issue #43's prose describes a `provisional -> recommended -> MUST`
  ladder; `agf/schemas/rule.schema.json` actually has two orthogonal fields
  (`status`: active/provisional/deprecated; `strength`: MUST/SHOULD/MAY) with
  no `recommended` value. IA proposed a mapping table rather than inventing a
  third enum value, and flagged it per `always/uncertainty-and-scope-rule.md`
  instead of resolving it silently. Needs a yes/no before implementation.
- **TP**: issue #49's input-provenance manifest borrows CycloneDX Agent BOM
  vocabulary; this project already chose SPDX over CycloneDX for binary SBOMs
  (`docs/coordination/00275-planner-architect.md`). Different artifacts
  (behavioral provenance vs. binary distribution), not a hard conflict, but
  worth a one-line rationale for running two BOM-adjacent vocabularies, or
  redirecting #49 at SPDX instead.

## Finding 3 — confirmed compatible, no action needed

- CD's CLI commands populate the coordination message's `from` field
  explicitly (`--from <role>`); AB's identity verification checks that field
  against git commit authorship after the fact. The two layers compose.
- REM's git-native constraint on #35 holds: AB's actual design uses
  config-file mapping (`agov-config.yaml`) plus commit authorship, no
  external identity store.
- CE's proposed `agov compliance ...` CLI namespace does not collide with
  CD's `coord`/`handoff` namespaces.
- REM's planned README "Exit guarantee" section already accounts for the
  two README edits in flight this session (`docs/append-only-audit-trail-policy`,
  `docs/readme-diagram-pointers`, `docs/readme-twenty-minute-path-clarity`) —
  it read them via `git show` and plans to link rather than duplicate.

## Finding 4 — one confirmed unblock

MISC found and fixed the actual root cause of issue #30: `allowed_roles()`
in `tooling/rust/crates/agov-coord/src/plan_validator.rs` reads
`agf/agents/MANIFEST.yaml` and only admits roles with `status: active`;
`build-engineer` is real but marked `status: suspended`, so it was
misclassified as unknown. Every other cluster was told to route around this
bug by omitting `build-engineer` from `advisory_consultation`. Once this
fix merges, that workaround is no longer necessary for future plans (existing
plan files that already avoided `build-engineer` remain valid; nothing to
retrofit).

MISC also found #19 and 7 of #17's 8 proposals already resolved on `main`
(commits `98e9fcf`, `22e9e24`) — recommend closing those with a reference
rather than planning new work for them.

## Finding 5 — branch-basis note (low risk, verified)

Five of seven branches (AB, CD, IA, CE, TP) forked from commit `870d01f`,
which is 11 commits behind `origin/main`'s current tip (`320405f` — includes
a version release and doc-only commits). Diffed the gap
(`git diff 870d01f 320405f -- agf/schemas/ tooling/rust/ agf/agents/`): only
a version-number bump in `Cargo.toml`/`Cargo.lock`, no schema or code
substance. Low risk, but all seven branches predate this session's own
`docs/radar.md`, `docs/sources.md`, and the three in-flight README branches —
rebase before merging any of them.

## Recommended merge order

1. `plan/misc-small-fixes-batch` (unblocks the `build-engineer` workaround)
2. `plan/identity-accountability-gate-enforcement` (establishes the canonical
   `accountable_human` shape)
3. `plan/incident-amendment-governance` (adopts that shape; apply Finding 1's
   wording fix)
4. `plan/removability-exit-guarantee`, `plan/coord-cli-ci-distribution`,
   `plan/compliance-export-buyer-side`, `plan/threat-model-governance-primitives`
   (no remaining ordering constraints between these four)

## Requires

Human governor decision on Finding 2's two open questions before the IA and
TP plan files move past Gate 1. Finding 1's naming fix can be applied without
further discussion.
