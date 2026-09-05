---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260905T193012Z-contribution-auditor-4ghp
timestamp: 2026-09-05T19:30:12Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260905T145235Z-planner-architect-esr1.md
  - 20260905T123338Z-coder-tester-6bkq.md
topic: mr2373-gate-accompaniment-plan-review
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Review evidence envelope (advisory authoring aid; not a gate decision)

Scope: MR !2373 / `docs/plans/20260905T145109Z-route-all-jobs-gate-accompaniment.md`
  at branch head `aed28d761`
Coverage: COMPLETE
Recommendation: CLEAR
Blocking findings: 0
Advisory findings: 3
Open findings with accepted risk: 0
Authority: recommendation only; the governor-reviewer decides Gate 1

---

# Claude contribution-auditor review of the MR !2362 gate-accompaniment plan

The plan's own `advisory_consultation` records contribution-auditor as
`waived` and states "this is gate mechanics… an independent review is
appropriate regardless." This record supplies that independent pass. I am
independent of this plan (it was authored by a planner-architect session I did
not run; I did not contribute to it).

## Verification performed

Read the full plan, `6bkq`, `.agf/gates.yaml`, MR !2362's actual
`.gitlab-ci.yml` diff, `ci_needs_implication.rs`, `gate_change_check.rs`, and
`docs/plans/20260903T045619Z-ci-mandatory-needs-implication-correction.md`.
Every factual claim in the plan checks out:

1. `.agf/gates.yaml` declares `.gitlab-ci.yml` as a `surfaces:` entry for both
   `gate-change-check` and `ci-needs-implication` — confirmed
   (`.agf/gates.yaml` gates[0].surfaces and gates[1].surfaces).
2. `gate_change_check.rs` reads `.agf/gates.yaml` only from
   `tempfile::tempdir()` fixtures (`BASE_REGISTRY` and variants); `repo_root()`
   is used there only for `AGOV_SCHEMA_DIR`. No test reads the repository's
   live registry — the gap the plan closes is real.
3. `ci_needs_implication.rs:29`
   (`live_configuration_preserves_every_mandatory_needs_implication`) filters
   on `needs:` blocks (line 207) and asserts nothing about `default:` /
   `default.tags` — gap real.
4. `shared_surface_requires_each_active_gates_own_test_evidence` exists at
   `ci_needs_implication.rs:135` and, per the plan, is left unchanged —
   correct; it proves the generic mechanism and is not the gap.
5. MR !2362's actual change adds `default: { tags: [podman, linux, amd64] }`,
   which matches the literal the plan proposes to assert exactly. The block
   also carries its own BREAKING `downstream-impact-rule` classification and
   migration note in-comment.
6. The 2026-09-03 plan did deliberately register `.gitlab-ci.yml` as a
   `gate-change-check` surface and disclosed the doubled-accompaniment burden
   as an accepted cost ("If the governor judges the doubled burden
   unacceptable, the alternative is to merge…"). The plan's refusal to reopen
   that decision under pressure from one blocked MR is well-founded and is the
   correct application of `gate-self-test-rule.md`'s anti-quiet-weakening
   discipline.

## Assessment

The plan is correctly scoped (two additive test files, no gate-logic change),
its negative-test obligation is correctly satisfied by RED-then-GREEN on the
new tests (the gate mechanism itself is unmodified), risks and rollback are
stated, and the affected-file list is closed. It rejects the tempting
surface-narrowing shortcut and cites the specific recent governed decision it
would reopen. No blocking findings.

## Advisory findings (non-blocking)

**A1 — `live_gates_registry_matches_declared_shape()` hard-codes the entire
parsed registry (ids + surfaces + declared test paths) as one expected
literal.** The plan acknowledges the brittleness and frames it as intended
accompaniment behaviour. It is still a broad update surface: every future
gate, surface, or test-path change must edit this literal in the same commit.
A lighter assertion — e.g. "`.gitlab-ci.yml` is a surface for exactly these
gate ids" plus "every declared surface and test path resolves on disk" —
would catch the drift class the plan cares about (accidental mis-scoping,
dangling paths) at much lower maintenance cost. The maximal version mirrors
`ci_needs_implication.rs`'s own live test, so it is defensible; the governor
should decide whether the symmetry is worth the churn.

**A2 — the implementation edits MR !2362's branch, not a fresh MR.** Both
tests "land in the same commit as !2362's `default: tags:` addition," so
!2362's diff grows to three files and its head SHA changes. Two consequences
not spelled out in the plan: (a) Gate 1 here authorizes modifying another
MR's branch; (b) MR !2362's existing `x4tv` Gate 2 handoff is against a head
that will no longer exist — a fresh exact-head Gate 2 handoff for !2362 is
required after the amend. The plan's validation step 4 ("canonical exact-head
pipeline on the rebased !2362 MR") shows awareness, but the coordination
consequence should be explicit.

**A3 — collision with the now-approved native-musl plan (MR !2389).** MR
!2389 was Gate-1-approved and merged ~3 hours after this plan was written, and
its own text requires its implementation to "scan and rebase against [MR
!2373's] work before editing either Rust test file" and to "retain the
distinct packaging-path assertions." Both approved plans now modify
`ci_needs_implication.rs` and `gate_change_check.rs`. This plan predates !2389
and does not mention the reverse dependency. Whichever implements second must
rebase and preserve both sets of assertions; the governor should sequence the
two implementations explicitly. This is the finding most worth acting on
because it is new information since the plan was authored.

## Recommendation

**CLEAR.** No blocking findings. The plan is a faithful, minimal, correctly
reasoned disposition of the accompaniment finding. A1 is a design-preference
note, A2 is a coordination-hygiene note (fresh !2362 Gate 2 handoff owed after
the amend), and A3 is a sequencing note the governor should resolve when
authorizing the two overlapping implementations. Gate 1 remains the
governor-reviewer's decision.
