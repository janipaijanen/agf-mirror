---
id: 20260909T170655Z-cpb1-multi-record-coordination-publication
status: pending-gate1
topic: cpb1-multi-record-coordination-publication
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      This modifies a gate (`coordination-publication-rule` /
      `agov coord publication-check`). InfoSec must confirm at this Gate 1
      that making multi-record coordination MRs explicit does not weaken any
      existing negative condition — in particular that a batch range still
      cannot smuggle a non-coordination file (condition 2), a non-pure
      change (condition 3), a merge commit (condition 4), an invalid record
      (condition 5), or a `[ci skip]` marker — and adds no channel for
      hidden implementation history. The implementing MR gets its own
      exact-head InfoSec review; this row covers the rule-direction
      decision.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Rule-direction change with a six-row projection manifest. Contribution
      auditor must confirm the batching allowance sets no false precedent
      for "release work overrides audit-trail timeliness", that
      per-record `agov coord validate`, causal `references-to`, immutable
      append-only authorship, and full-range checking are all preserved, and
      that the projection surfaces are complete and non-duplicative.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# CP-B-1 — explicit multi-record coordination publication (Gate 1 child plan)

## Provenance and authority

- Parent: `docs/plans/20260909T133651Z-cpb-coordination-publication-efficiency.md`
  (CP-B work order), Gate 1 approved by `sd23` (2026-09-09), merged MR !2610.
  The parent is a decomposition decision only; it authorizes no
  implementation.
- `eghq` routes this child Gate 1 plan to the Claude planner-architect lane
  and narrows CP-B-1 to **D-1 only**: pure-added multi-record coordination
  publication. D-2 (release-lane hold) and D-3 (urgency field) are CP-B-2, a
  separate plan. CP-B-3 stays in the later v2.11 release-publication child
  plan.
- CP-B is a **hard v2.11 release-cut precondition** (v2.11 scope plan
  row 7). CP-B-1 + CP-B-2 together are the "merged and evidenced"
  implementation row 7 requires.

This plan decides the rule-text change and its projections. It authorizes
no `agov` behavior change beyond a confirming test, no merge, no release.

## The gap

`agov coord publication-check` and `coordination-publication-rule` already
**mechanically permit** a range of N pure `docs/coordination/*.md`
additions in one MR — none of the six blocking conditions restricts the
*count* of added records. Verified this session: MRs !2599 (3 records),
!2603 (4), !2625 (1) all passed `publication-check` with multi-record
ranges.

What is missing is **explicit rule text**. Practice is one-record-per-MR by
habit; `jprj`/`5jde` CP-B measured ~25 such MRs in a single session and a
v2.10 frozen-release-base rebase chain (`58459c54 → d0ebef47 → 6b47446d →
611cfe9c`) driven by per-record coordination merges. The rule neither
blesses batching nor guides when a record must still publish on its own.

## D-1 — the change

### Rule text (`agf/rules/workflow/coordination-publication-rule.md`)

Add a short subsection stating:

1. **Multi-record coordination MRs are valid.** A commit range whose every
   change is a pure addition of a `docs/coordination/*.md` file is a valid
   coordination-only MR regardless of how many records it adds. All six
   blocking conditions apply unchanged to the full range.
2. **When to batch.** Records produced in one session/work cycle with no
   independent-visibility obligation blocking on any single one SHOULD be
   published together. This reduces branch/MR/merge count and `main` churn
   against a frozen release base.
3. **When a record must publish on its own.** Any record another agent or
   human must see immediately to act — a live obligation with a waiting
   actor, a security or gate finding, an escalation, a blocking review —
   publishes independently and immediately, never held for a batch. (The
   general urgency mechanism is CP-B-2; this clause states the principle so
   CP-B-1 does not accidentally imply "always batch".)
4. **Nothing else changes.** Conditions 1–6, the lifecycle/supersession
   semantics, the `[ci skip]` prohibition, and the required negative
   self-tests are unchanged in force.

### Preserved negative conditions (must remain, per `eghq`)

Every existing negative gate condition stays and stays tested:

| Condition | Still rejects |
|---|---|
| 1 | unresolvable base ancestry / unfetchable target |
| 2 | any commit outside `docs/coordination/*.md` anywhere in the range (MR !692 shape) |
| 3 | any non-pure change under `docs/coordination/` — modify, delete, rename, copy |
| 4 | a merge commit or imported history in the range |
| 5 | a newly added record that fails `agov coord validate` |
| 6 | a lightweight-routed range not using the `coordination:` convention |
| — | `[ci skip]` / `[skip ci]` in any coordination commit message |

### Gate-self-test (this is a gate modification — `gate-self-test-rule`)

Ineligible for fast-track. The **implementing MR** MUST show, with
transcript/pipeline evidence, that after the change the gate still fires on
every known-bad input and now passes the intended new input:

- MR !692 shape (impl commit + coordination commit) → **still rejected**
- implementation-then-revert-then-coordination range → **still rejected**
- a non-pure change (modify/delete/rename of an existing record) → **still
  rejected**
- a batch range that adds N valid records **plus one non-coordination
  file** → **still rejected** (condition 2 over the whole range)
- stale / unfetchable base → **still fails closed**
- **new positive test:** a range that adds **two or more** valid
  `docs/coordination/*.md` records and nothing else → **passes**

The `agov` implementation likely needs **no logic change** (it already
inspects the full range and permits N additions); if a test reveals an
off-by-one or single-file assumption, that fix is in scope for the
implementing MR and gets its own review.

## Six-row documentation-rule projection

Structured manifest:
`docs/reviews/rule-projection/20260909T170655Z-cpb1-multi-record-coordination-publication.yaml`.
Summary of dispositions:

| Surface | Disposition |
|---|---|
| Normative owner — `agf/rules/workflow/coordination-publication-rule.md` | updated (the D-1 subsection above) |
| Adopter explanation — `agf/docs/site/docs/concepts/coordination-protocol.md` | updated (batching guidance in the publication-independence section) |
| Runbook/situation map — `agf/docs/site/docs/operate/poll-and-coordinate.md` | updated (one line in the publish-a-coordination-handoff note: "batch same-cycle records; publish an urgent one alone") |
| AI landing/loaders — `AGENTS.md`, `agf/templates/AGENTS.md` | updated (one sentence each in the Coordination channel section; `.claude/rules/workflow/coordination-publication-rule.md` is the Claude projection and is updated to match the canonical rule) |
| Validation/drift evidence — `tooling/rust/crates/agov-coord/src/publication_check.rs` (+ its tests) | implemented — the new positive test and the re-run negatives; logic change only if a test surfaces one |
| Distribution | confirm the updated rule + Claude projection + template ship in the AGF framework archive; the concept/operator pages ship in the Pages build |

`agov plan projection-check --base-ref origin/main <manifest>` must pass at
Gate 1; the strict run (no `--allow-planned`) is required Gate 2 evidence
for the implementing MR.

## Collision check — MR !2038

`eghq` asked for a collision check before fixing the affected-file set.
**MR !2038** is `plan+add: stale worktree and branch pruning rule` — it
touches `agf/rules/workflow/stale-worktree-and-branch-pruning-rule.md`,
`.claude/rules/workflow/stale-worktree-and-branch-pruning-rule.md`, and
`agf/rules/workflow/branch-and-worktree-rule.md`. **No overlap** with
`coordination-publication-rule.md`, its Claude projection, the
coordination-protocol concept page, the poll-and-coordinate operator page,
`publication_check.rs`, or the AGENTS loaders' coordination sections.
CP-B-1 and MR !2038 are independent; no sequencing constraint.

## Out of scope (explicit, per `eghq`)

- **CP-B-2** — the release-lane hold record wire shape, the source of
  deferred records, exact CLI / JSON / exit-code / MCP contracts, and
  backward-compatible `urgency` normalization. Separate plan; the parent's
  D-2/D-3 belong there.
- **CP-B-3** — the v2.11 release-publication child plan's use of the hold.
- Any change to conditions 1–6, lifecycle/supersession semantics, or the
  `coordination:` fast-path routing.

## Downstream impact

Classification: **BREAKING** for `coordination-publication-rule` (adopters
following it gain an explicit batching allowance and a "publish urgent
alone" obligation) and **EDITORIAL→ADDITIVE** for `agov` (a new passing
test; logic unchanged unless a test surfaces a defect).

```
Impact:
  Foundry:          Yes — batching becomes the explicit default for
                    same-cycle records; urgent records still publish alone.
  Direct adopters:  BREAKING (rule) — one new subsection to read; no
                    existing coordination MR becomes invalid.
  Downstream:       No adopter artifact shape changes. `agov coord
                    publication-check` output is unchanged for existing
                    inputs.
  Migration:        None required. Existing one-record-per-MR practice
                    stays valid; batching is permitted, not mandated.
  Evidence:         jprj / 5jde CP-B; the v2.10 release-base rebase chain
                    (58459c54 → d0ebef47 → 6b47446d → 611cfe9c); handoff
                    2brd's manual hold workaround; this session's ~10
                    coordination MRs.
```

## Validation before Gate 1

`agov plan validate` on this file; `agov plan projection-check --base-ref
origin/main` on the manifest; coordination visibility/collision scan;
`git diff --check`.

## Decision requested at Gate 1

Approve (a) the D-1 rule-text change as scoped above, (b) the preserved
negative-condition list, (c) the gate-self-test set including the new
positive multi-record test, (d) the six-row projection dispositions, and
(e) the CP-B-1 / CP-B-2 / CP-B-3 boundary. Implementation (rule edit +
projections + `publication_check.rs` test) begins only after this approval,
as its own MR with exact-head contribution + InfoSec + build reviews and a
binding Gate 2.
