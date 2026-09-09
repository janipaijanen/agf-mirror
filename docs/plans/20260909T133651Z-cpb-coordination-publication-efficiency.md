---
id: 20260909T133651Z-cpb-coordination-publication-efficiency
status: approved
gate1_approved_by: governor-reviewer
gate1_approved_at: 2026-09-09T16:34:09Z
topic: cpb-coordination-publication-efficiency
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      This plan changes the direction of `coordination-publication-rule`
      (publication-integrity mechanics) and defines an "urgent bypass" and a
      "release-lane hold". InfoSec must confirm at this Gate 1 that the hold
      cannot suppress urgent security or gate evidence, that the urgent
      classification is not attacker/author-gameable into either direction
      (forcing a hold, or forcing a bypass), and that batching adds no
      channel for hidden non-coordination content. The implementation MR
      gets its own exact-head InfoSec review; this row covers the design
      decision, which is not deferrable.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Rule-direction change. Contribution-auditor must confirm the batching
      allowance and hold/bypass semantics do not weaken full-range
      publication checking, per-record validation, causal `references-to`,
      immutable authorship, or the no-hidden-implementation-commits
      guarantee, and set no false precedent for "release work overrides
      audit-trail timeliness". The implementation MR gets its own exact-head
      audit.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# CP-B — coordination publication efficiency (Gate 1 work order)

## Provenance and authority

- `jprj` / `5jde` CP-B (capability-developer analysis): the strongest
  measured Propeller friction — one branch + MR + pipeline + merge +
  branch-delete per append-only coordination record, and every such merge
  advances `main`.
- `er78` promoted CP-B to **IN** for v2.11; `sh3m` approved the expanded
  v2.11 scope. Row 7 of `docs/plans/20260909T021841Z-v211-release-scope.md`
  makes CP-B a **hard v2.11 release-cut precondition** — its implementation
  must merge and be evidenced before the release-metadata MR opens; there is
  no fallback that cuts v2.11 without it (superseding that needs a new
  governor scope decision).
- `mbg6` allocated this work order to the Claude planner-architect session;
  split acknowledged in `aars`.

This plan is a **work order**: it decides the design approach and the
implementation decomposition. It authorizes no implementation, rule edit,
tooling change, merge, tag, or release. Each implementation slice returns
for its own Gate 1 evidence check and Gate 2.

## Problem (measured, not latent)

1. **Per-record ceremony.** Each append-only coordination record is
   published as its own branch/MR/pipeline/merge. The
   `coordination-publication-rule` fast path cut CI cost but not the
   branch/MR/merge count. This session alone opened ~10 coordination MRs;
   `jprj` observed ~25 in one session.
2. **`main`-churn against a frozen release base.** Every coordination MR
   merge advances `main`. During v2.10, `release-2.10.0` was rebased 3+
   times (`58459c54 → d0ebef47 → 6b47446d → 611cfe9c`) as coordination MRs
   landed and the release plan's "any target drift invalidates preflight"
   rule fired. Handoff `2brd` had to manually instruct all sessions to stop
   touching review/ack coordination MRs during the release target hold — a
   manual workaround for a structural interaction.

## Design approach requested for Gate 1 approval

### D-1 — Multi-record coordination MRs (make the existing capability explicit)

`agov coord publication-check` and `coordination-publication-rule` already
permit N added `docs/coordination/*.md` files in one commit range (verified:
this session's MRs !2599 and !2603 carried 3 and 4 records and passed).
Practice is nonetheless one-record-per-MR by habit.

- **Decision requested:** bless multi-record coordination MRs explicitly in
  `coordination-publication-rule`, and state when batching is appropriate
  (records from one session/work cycle with no independent-visibility
  obligation blocking on any single one) versus when a record must publish
  on its own (any record another agent or human must see immediately to act
  — see D-3).
- **No tooling change expected** beyond a confirming test; the check
  already inspects the full range.

### D-2 — Release-lane coordination hold (net-new)

A declared, time-bounded hold that batches **non-urgent** coordination
records until a release target hold clears, so coordination merges stop
invalidating a frozen release base.

- **Decision requested:** which mechanism —
  (a) a `hold` coordination record kind/marker the release lane publishes,
      naming the held target and an expiry, that `agov` reads to report
      "hold active" and list deferred records; or
  (b) a repo-config assertion (e.g. under `docs/coordination/agov-config.yaml`)
      the release lane sets and clears; or
  (c) a branch/label convention with CI awareness.
- Recommended: **(a)** — stays inside the append-only channel, is itself
  auditable, self-expires, and needs no new config surface. `agov` gains a
  read-only "publication hold status" surface; it never blocks a push
  mechanically (advisory, per `tool-authority-boundary-rule`) unless a named
  rule grants it blocking authority.
- The hold **defers** non-urgent records to a single post-hold batch MR; it
  never drops, rewrites, or reorders them, and never suppresses D-3 records.

### D-3 — Urgent bypass (net-new)

A record another actor must see immediately (a live obligation, a security
or gate finding, an escalation, a blocking review) publishes independently
and immediately, hold or no hold.

- **Decision requested:** how "urgent" is classified —
  (a) an explicit front-matter field (`urgency: urgent|normal`, default
      `normal`); or
  (b) derived from `kind` + `requires-ack` (e.g. any `requires-ack: true`
      `handoff`/`review`/`note` to a named recipient is urgent by default).
- Recommended: **(a) explicit, default normal**, with a rule stating the
  categories that MUST be marked urgent (security/infosec findings, gate
  blockers, live obligations with a waiting actor, escalations). Author
  self-declares; misuse in either direction is a review finding, not a
  tooling gate.

## Invariants the implementation MUST retain (non-negotiable)

- Full-range publication checking — `agov coord publication-check` inspects
  every commit in the merge-base..head range, not only the tip diff.
- Per-record `agov coord validate` (blocking) for every added record.
- Causal `references-to` intact; immutable authorship; append-only (no
  modify/delete/rename of existing records).
- No hidden implementation commits — coordination-only still means the whole
  range adds only `docs/coordination/*.md`.
- **A release hold MUST NOT suppress or delay an urgent (D-3) record**, and
  MUST NOT be usable to withhold security or gate evidence. This is the
  InfoSec-critical boundary.
- `[ci skip]` / `[skip ci]` remains forbidden in coordination commit
  messages (unchanged).

## Gate-self-test obligation (row 7 / `gate-self-test-rule`)

CP-B **modifies a gate** (`agov coord publication-check` /
`coordination-publication-rule`). The implementing MR is ineligible for the
fast-track waiver and MUST include, and show running, a negative test
demonstrating the gate **still fires after the change** on its known-bad
input:

- the MR !692 shape — an implementation commit followed by a
  coordination-only commit, such that the tip diff looks coordination-only
  but the range is not — still **rejected** by `agov coord
  publication-check`;
- the revert-in-range shape — implementation commit, its revert, then a
  coordination commit — still **rejected** (range inspection, not net
  diff);
- a non-pure-addition (modify/delete/rename of an existing record) still
  **rejected**;
- and, new for CP-B: a batch MR whose range is genuinely N pure
  `docs/coordination/*.md` additions **passes**, while a batch MR that
  smuggles one non-coordination file among N coordination additions is
  **rejected**.

Transcript/pipeline-log evidence in the MR description, per the rule.

## Implementation decomposition (each its own Gate 1 evidence + Gate 2)

| Slice | Content | Owner |
|---|---|---|
| **CP-B-1** | `coordination-publication-rule.md` edit: bless multi-record MRs (D-1), add the release-lane hold semantics (D-2) and urgent-bypass semantics (D-3), restate the retained invariants and the InfoSec boundary. Rule-only. | planner-architect drafts → governor Gate 1 |
| **CP-B-2** | `agov` support: read-only publication-hold status surface (D-2(a)); `urgency` field recognition + validation (D-3(a)); the CP-B negative tests above. Rust + `agf/schemas/coordination-message.schema.json` (`urgency` enum, optional, default `normal`). | coder-tester → exact-head contribution + InfoSec + build reviews → governor Gate 2 |
| **CP-B-3** | Wire the release-publication child plan to set/clear the hold and to treat a held batch as expected (not target drift within the hold window). Doc + release-plan cross-reference only. | whoever owns the v2.11 release-publication child plan |

CP-B-1 and CP-B-2 together are the "merged and evidenced" implementation
row 7 requires before the v2.11 cut. CP-B-3 may follow.

## Downstream impact

Classification: **BREAKING** for `coordination-publication-rule` (changes
the publication workflow contract adopters follow) and **ADDITIVE** for
`agov` and the coordination schema (`urgency` is optional, default
`normal`; the hold surface is read-only and advisory).

```
Impact:
  Foundry:          Yes — our own coordination workflow changes: batching
                    becomes explicit and preferred; a release hold defers
                    non-urgent records.
  Direct adopters:  BREAKING (rule) — adopters gain a batching allowance,
                    a hold mechanism, and an urgency classification they
                    must learn; ADDITIVE (tooling/schema) — opt-in.
  Downstream:       Adopter coordination records gain an optional `urgency`
                    field; no existing record becomes invalid.
  Migration:        Existing records without `urgency` are treated as
                    `normal`. Adopters adopting the hold must name the
                    categories that MUST be `urgent` (this plan proposes a
                    default set). CI wiring is a removable adapter around
                    the `agov` surface.
  Evidence:         jprj / 5jde CP-B; the v2.10 release-base rebase chain
                    (58459c54 → d0ebef47 → 6b47446d → 611cfe9c); handoff
                    2brd's manual hold workaround.
```

## Risks and controls

- **The hold suppresses something that needed to be seen.** Control: D-3
  urgent bypass is mandatory for security/gate/live-obligation records; the
  hold is time-bounded and self-expiring; `agov` lists every held record so
  nothing is invisible; InfoSec signs off on the boundary at CP-B-1 Gate 1.
- **"Urgent" is over- or under-claimed.** Control: explicit field, default
  `normal`, a named MUST-be-urgent category list, misuse is a review
  finding. No tooling gate on the classification itself.
- **Batching hides a non-coordination commit.** Control: full-range check
  unchanged; the CP-B-2 negative test proves a smuggled file is still
  rejected.
- **Scope balloon into a general coordination-workflow redesign.** Control:
  this work order is exactly D-1/D-2/D-3 plus the retained invariants;
  anything else (session addressing, supersession surfacing, collision
  tooling) is rows 5–6, not here.

## Validation before Gate 1

`agov plan validate docs/plans/20260909T133651Z-cpb-coordination-publication-efficiency.md`;
coordination visibility/collision scan; `git diff --check`.

## Decision requested at Gate 1

Approve (a) the D-1 explicit-batching direction, (b) D-2 mechanism **(a)**
(hold record kind/marker) — or name an alternative, (c) D-3 classification
**(a)** (explicit `urgency` field, default `normal`) — or name an
alternative, (d) the retained-invariants list including the InfoSec
boundary, and (e) the CP-B-1/-2/-3 decomposition. Implementation begins
only after this approval and each slice's own gate.
