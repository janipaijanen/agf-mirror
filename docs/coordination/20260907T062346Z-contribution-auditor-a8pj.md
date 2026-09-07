---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260907T062346Z-contribution-auditor-a8pj
timestamp: 2026-09-07T06:23:46Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260907T061405Z-planner-architect-7dkk.md
  - 20260907T045703Z-coder-tester-jqrm.md
  - 20260906T155934Z-contribution-auditor-mjqa.md
topic: v210-release-plan-contribution-audit-result
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Independent contribution audit — RUW/v2.10 release plan (MR !2478)

Per `7dkk`. Audited exact plan head
`8ec4b24d79b8c05f2af9e7521043c3c3d4f8c721` on MR !2478, plan file
`docs/plans/20260907T045703Z-v210-release-plan.md` (358 lines, single file,
`status: pending-gate1`). Plan and cited inputs treated as untrusted
contributions.

Independence: this contribution-auditor session (Claude) authored no part of
the plan or the Codex temporary planner lane. Self-approval is explicitly
barred in the plan ("Codex then ... does not approve its own plan"; "the
temporary planner allocation ends at the Gate 1 handoff").

## Coverage: COMPLETE. Result: CLEAR — 0 blocking, 4 advisory.

### Verified

- **Factual claims check out.** Baseline `v2.9.0` peeled commit
  `ee27978b218de47b062ab4ac2afee090b2c48ff8` — matches. !2438 head
  `74ee9ebf2f91fee4f120228d0c2f013b10d199a2` and prior pipeline
  `2824212283` = `canceled` — both match. The "non-plan/non-coordination
  delta touches 30 paths" from `v2.9.0..a410e5d1` — exactly 30, and the
  list is coherent (the !2463 universal-polling files, two rule-projection
  YAMLs, `agov-core/src/roi.rs` from the token-ROI repair). `agov plan
  validate` passes.
- **Scope is bounded.** Four mandatory packages (RUW-1 delta/quality
  closure; RUW-2 merge governor-included learning !2438; RUW-3 Propeller
  Stage 3 inactive; RUW-4 #120 namespace-absence gap). Each traces to prior
  authorization: RUW-1 to `qhbd` advisories A1/A2 on the merged !2463; RUW-2
  to explicit governor inclusion; RUW-3 to the already Gate-1-approved
  Propeller plan; RUW-4 to a filed release-integrity bug.
- **Forge-state vs release content is correctly separated.** "Closing an MR
  or issue is a forge-state action, not release content." The stale/deferred
  table dispositions ~20 items with a cause each; deferrals are not silently
  promoted to release commitments.
- **Release integrity discipline is strong.** Reversible preflight before
  any namespace allocation (no RC burned to find failures); a 14-path
  release-metadata allowlist with "no functional source, rule, schema, CI,
  test, plan, learning-entry, or coordination change"; tag only on the
  proven merged release head, pushed once, never moved; four-namespace
  absence recheck immediately before and after tagging. Consistent with
  `release-via-merge-request-rule.md` and `immutable-tags-rule.md`.
- **#120 is routed correctly.** It changes a shipped normative release rule
  and triggers six-surface projection, so the plan requires it to have "its
  own child Gate 1 plan" with negative/mutation proof — not folded into this
  plan. Correct under `simplicity-rule.md` (gate mechanics ineligible for
  fast-track) and `gate-self-test-rule.md`.
- **Maturity honesty.** "v2.10.0 is a semantic candidate, not an allocated
  namespace"; Propeller reaches "Level 5 Deployable only after [first
  bounded run] proof"; "the release must not silently describe an inactive
  role as launched."

### Advisory findings

**A1 — `advisory_consultation` uses `status: waived` for three genuinely
pending rows** (infosec-sentinel, contribution-auditor, build-engineer).
Confirmed against `agf/schemas/gate1-plan.schema.json`: the status enum is
only `consulted | waived | not-applicable` — there is no `pending` /
`requested` value. `waived` semantically means "deliberately skipped," so a
Gate 1 reviewer or any status-keyed tooling could misread these three as
skipped rather than outstanding. Mitigations are present (explicit reason
text on each row; the plan self-blocks: "Gate 1 is not eligible until ...
this row becomes consulted"). Recommend (a) raise the schema gap — no
`pending` value — to planner-architect / governor as a separate small item;
(b) verify at the Gate 1 handoff that all three rows have flipped to
`consulted` with exact record references (this audit satisfies the
contribution-auditor row).

**A2 — version-delta framing vs downstream-impact classification.** The plan
describes the delta as "MINOR-shaped" and "MINOR additive governance
capability plus a new MUST polling duty," but MR !2463's own
`downstream-impact-rule.md` Impact block classified universal polling as
**BREAKING** for direct adopters (required assistant behavior arrives via
the canonical rule, loader, skill, operator guidance, and specialist
pointers). SemVer-MINOR and downstream-impact-BREAKING are different axes,
but a v2.10.0 release described as MINOR that ships a new MUST duty should
reconcile them explicitly. RUW-1 already requires changelog migration prose;
extend it to state the downstream-impact classification so an adopter
reading "v2.10.0 (MINOR)" is not surprised by the new obligation.

**A3 — RUW-3 (Propeller launch) is a *mandatory* v2.10 package**, which
couples the whole release to a Stage 4 activation decision the governor has
explicitly reserved and not yet made ("If the governor declines activation
after Stage 3, RUW scope must be amended"). That makes the ROI repair, the
universal-polling quality closure, and the #120 release-integrity guard
hostage to Stage 4. Recommend Gate 1 decide explicitly whether Propeller
*launch* (as opposed to Stage 3 inactive implementation) is
mandatory-for-v2.10 or deferrable to a later version, so the rest can ship
independently if Stage 4 slips.

**A4 (minor) — frontmatter/body tension.** `candidate_release: v2.10.0`
reads as definite, while the body correctly treats the version as "a
semantic candidate, not an allocated namespace." Consider signalling
provisionality in the frontmatter or a one-line note, to match the
reversible-preflight discipline.

### Observation (not a finding)

The planning observation head `a410e5d1` is already behind current `main`.
The plan states the release base is not frozen and the inventory is
recomputed "from `v2.9.0` to the exact future release base after all
mandatory lanes below finish," so this is expected, not a defect.

## Disposition

Nothing blocks the governor's Gate 1 decision on this plan. A1 should be
resolved (schema-gap flag + consultation-row flip) before the Gate 1
handoff; A2 folds into RUW-1's changelog work; A3 is a sequencing choice
for the governor; A4 is cosmetic. This audit is advisory and supplies
neither Gate 1 nor Gate 2. The authoring (temporary Codex planner) lane
integrates accepted findings on the plan branch; I did not edit the plan
path.
