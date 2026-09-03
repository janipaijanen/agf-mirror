---
id: 20260830T214452Z-v290-governed-roadmap-and-work-order
status: approved
topic: v290-governed-roadmap-and-release-work-order
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      The plan creates and projects planning documents and issue dispositions.
      No credential, secret, personal-data, or cross-record content is read,
      written, or transported, and no runtime, dependency, or artifact surface
      changes. The commercial/compliance cluster is named as a v2.12 heading
      only; any work inside it returns for its own consultation.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Routed for audit of the bounded maintenance path in particular, which
      creates a reduced-ceremony route for roadmap edits and is the part of
      this plan most capable of becoming a governance loophole.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.9.0 governed roadmap and release work order

Prepared as **planner-architect** under the handoff in `gkah` and the binding
direction in `nsne`. Role note: this session has operated as `coder-tester`;
the governor authorized advancing the release roadmap work in session, and
`gkah` addresses it to `planner-architect`. The role assumption is recorded in
the accompanying coordination handoff rather than left implicit.

`nsne` conditioned this work on the v2.7.0-rc.1 release obligation being
stable. That condition is now met: `m2pr` abandons rc.1 permanently, which is
a terminal state rather than a pending one.

## What this plan is

Preparation that makes the next shipment **governable**. It does not bump a
version, move a milestone, alter an issue state, or touch a release branch or
tag. `nsne` is explicit that it authorizes planning, not retroactive approval
for mutations, and this plan inherits that boundary exactly.

## Artifact model

Three surfaces, one authority, per `nsne`:

| Surface | Role | Authority |
|---|---|---|
| `docs/roadmap.md` | Single governed internal roadmap | **Owner.** Currently absent; this plan creates it. |
| `agf/docs/site/roadmap.md` | Adopter-facing projection | Projection. Exists (9.6 KB). Must not state commitments the owner does not. |
| GitLab milestones | Mechanical projection | Projection. Derived, never a second source of truth. |

The `sources → radar → issue → roadmap → release` distinction is preserved.
Radar and sources items stay **non-committal** and do not become roadmap
commitments by being mentioned; promotion is an explicit, separately evidenced
step.

## Issue dispositions — verified, not asserted

`nsne` directed verification of stale claims before closure. Verified
mechanically against the built CLI and the crate tree at plan time:

| Issue | Claim | Verified result | Disposition |
|---|---|---|---|
| #32 write-path commands | shipped | `agov coord post`, `agov handoff take`, `agov handoff complete` all resolve `--help` | **shipped-closeout** |
| #37 MCP server wrapper | shipped | `tooling/rust/crates/agov-mcp` exists | **shipped-closeout** |
| #36 `coord watch` / incremental polling | partial | `agov coord list --since` **exists**; `agov coord watch` **absent** | **partial — split** |

#36 splits: the incremental-polling half is delivered and closes; the
`coord watch` half remains open and is re-scoped as its own item. Closing #36
whole would overclaim; leaving it whole understates delivered work.

Remaining candidates carried into clustering without disposition here, because
each needs its own evidence pass: #31, #43, #53, #75, #101, #102, #106.

**Closure is not authorized by this plan.** Dispositions are recommendations;
the state changes happen only after Gate 1 approval, and each closure cites the
verification above.

## Cluster baseline

Per `nsne`, used as the planning baseline:

- **v2.9** — tracker truth and release cost (#53, #75, #101, #106 candidates)
- **v2.10** — independence and enforcement
- **v2.11** — adopter safety
- **v2.12** — commercial and compliance

Clusters are headings, not commitments. An item enters a release only with an
owner, acceptance evidence, and a terminal shipment disposition.

## The bounded maintenance path

`nsne` asks for a route for routine roadmap adjustment that does not require a
fresh full Gate 1 per non-material edit. This is the part of the plan most
capable of becoming a loophole, so it is modelled on the existing, proven
`docs/sources.md` / `docs/radar.md` research fast path in
`agf/rules/workflow/simplicity-rule.md` rather than invented fresh.

**Eligible** — all must hold:

1. Every commit in the range, and the aggregate diff, touches only
   `docs/roadmap.md`. Merge commits ineligible.
2. The edit is one of: correcting a typo or broken link; updating an item's
   status to match an already-recorded governed decision; adding a
   cross-reference to an existing issue or record; adjusting wording that
   changes no commitment.
3. The edit changes no scope, no cluster membership, no version target, no
   owner, no priority, and no acceptance criterion.
4. Every material claim traces to an existing governed record, cited by ID.

**Automatically disqualified** — returns to full Gate 1:

Adding or removing a roadmap item; moving an item between clusters or
versions; changing an owner, priority, target date, or acceptance criterion;
promoting a radar or sources entry to a roadmap commitment; any adopter-facing
projection change; any milestone, label, or issue-state mutation; any public
capability, maturity, or timeline claim; any security, legal, licensing, or
commercial content; or ambiguity about whether an edit is material.

**Retained regardless:** branch, MR to the protected target, the
commit-message waiver, green exact-head pipeline, and a binding
governor-reviewer Gate 2 decision. The path reduces ceremony before the MR; it
does not reduce the gate.

**Not a gate change.** This adds a documented reduced-ceremony route for one
file. It does not modify any validator, CI check, merge policy, or schema. If
the reviewer judges otherwise, the consequence is a mandatory negative test
under `always/gate-self-test-rule.md`, and the test list below already
anticipates that.

## Rule-projection screen

Required: this introduces normative operating guidance (the maintenance path)
and changes an adopter-facing surface (`agf/docs/site/roadmap.md`). Structured
manifest to be authored at
`docs/reviews/rule-projection/v290-governed-roadmap.yaml`, disposing all six
rows, with strict `agov plan projection-check` output as Gate 2 evidence.
The normative owner is expected to be a new or amended workflow rule and may
carry `stage: planned` at Gate 1 under that rule's lifecycle.

## Test and validation strategy

1. `agov plan validate` on this plan — passing at submission.
2. `agov plan projection-check` (strict, no `--allow-planned`) before Gate 2.
3. Projection consistency: every commitment in `agf/docs/site/roadmap.md` must
   exist in `docs/roadmap.md`. A projection asserting more than its owner is a
   blocking finding.
4. Issue-disposition evidence: each shipped-closeout recommendation reproduces
   its verification command and output.
5. **Negative test for the maintenance path:** a candidate edit that moves an
   item between version clusters MUST be rejected as ineligible. Whatever
   enforces eligibility — reviewer checklist or tooling — must be shown
   rejecting it, not merely described as capable of doing so.

## Open question the governor must answer

**`release-v2.7.0-rc.2` disposition.** `m2pr` abandons rc.1 and deliberately
declines to infer rc.2's fate. v2.8.0 is already published and accepted, which
argues the whole v2.7.0 line is dead, but that is not a planner decision. This
plan's scope cannot be called complete until it is answered. No rc.2 work
proceeds meanwhile — not because it is abandoned, but because starting would
presume the answer.

## Risks

- **Maintenance path widening by precedent.** The first ambiguous edit waved
  through becomes the new boundary. Mitigation: the disqualifier list resolves
  ambiguity toward full Gate 1, and test 5 exists to prove rejection works.
- **Projection drift.** Two roadmap files will disagree eventually. Mitigation
  is check 3, and it should be mechanical rather than review-dependent.
- **Overclaiming on closure.** #36 is the live example. Mitigation: split
  rather than close whole, and cite verification per issue.

## Rollback

Delete `docs/roadmap.md` and revert the projection edits. No schema change, no
data migration, no artifact format change, no issue state touched by this plan.

## Evidence

- `docs/coordination/20260827T203743Z-governor-reviewer-nsne.md` (binding direction)
- `docs/coordination/20260828T143959Z-coder-tester-gkah.md` (work order)
- `docs/coordination/20260830T214141Z-governor-reviewer-m2pr.md` (rc.1 abandoned)
- CLI verification transcripts for #32, #36, #37, reproduced above
