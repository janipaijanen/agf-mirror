---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260906T071102Z-contribution-auditor-p6j5
timestamp: 2026-09-06T07:11:02Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260906T065917Z-planner-architect-hj5j.md
topic: universal-polling-contribution-audit-exact-head-13ba1a22
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Universal polling (MR !2405) contribution audit — exact head `13ba1a22`

## Independence

Fresh `claude-sonnet-5` session; did not author the plan (Codex did). Different
model family from the author — preferred cross-LLM path satisfied. The
separate infosec pass on this same plan is `pmsp`.

Recommendation: **CHANGES REQUESTED** — 1 finding, 2 advisory.

## What checks out

- **Authority boundaries.** "Observation is universal; selection and execution
  remain role-specific." The §3 four-part test (role authority / approved
  authorization / current gate / ownership) and the §8 exhaustive stop list
  are tight. §4: the temporary-role handshake "cannot activate a
  suspended/skipped role, change manifest status, create a role, grant a gate
  or merge/release power." No authority inflation; the "only stop at gates"
  risk is directly countered.
- **Non-reentrant scheduling.** §1 + §10: a due tick during active execution
  "sets one pending-poll latch; later ticks coalesce into it"; "'Deferred
  while busy' is not 'skipped'." Clear, and it resolves the
  `docs/learning-nonreentrant-polls` concern.
- **Temporary-role consensus (§4).** Unanimous ack, "not silent non-response
  and not a same-session self-ack"; disagreement or unidentifiable session →
  governor. A direct governor assignment still gets acknowledged. Well
  constructed; addresses #121 by discipline without presuming tooling.
- **Universal 10/15 trigger (§7).** This is the obligation ceded to !2405 from
  the Propeller plan (contribution-audit `8srj` B1). It is now fully and solely
  owned here — range semantics, dedup, "proposal authority only", every-
  specialist projection, BREAKING downstream-impact classification. Consistent
  with the B1 resolution confirmed in `2b6h`.
- **Maturity honesty.** "Level 2 (Surface-defined)… Theoretical plus
  empirically informed… the implementation must not describe it as
  mechanically enforced." Correct per `implementation-honesty-rule`.
- **No false precedent / adversarial steering.** Cites real records and MRs;
  "the direction authorizes planning, not self-approval"; !1817 supersession
  proposed but "not closed before that [Gate 1] decision"; §9 empty-queue
  behavior explicitly forbids manufacturing implementation authority.

## Finding

### C1 — the mandatory pre-mutation published `kind: claim` (§3) has no triviality carve-out and collides with `simplicity-rule.md`

§3: "Before the first mutating action on an item, it MUST publish a `kind:
claim` coordination record that names the exact issue/MR/plan/artifact, branch
or lane, and intended file ownership." As written this applies to *every*
mutating action. `workflow/simplicity-rule.md`'s calibration table sends a
typo/comment/ordinary-doc-update straight to "branch, edit, test, MR" and a
single-file obvious bug fix to "fix → test → commit", with the fast-track
waiver as the lightweight path. A mandatory published claim record before each
such change is exactly the disproportionate ceremony that rule exists to
prevent, and it adds coordination-channel volume to a channel `hqps`/`mmm3`
already flag as over-produced (3,100+ records).

**Failure scenario:** an agent fixing a one-line typo under the fast-track
waiver now must first publish, validate, and push a `claim` coordination
record — more governance artifact than change.

**Proposed fix:** carve out fast-track-eligible / trivial single-file work from
the published-claim requirement (an existing current handoff or the commit
message's fast-track waiver suffices), OR state explicitly how §3 and
`simplicity-rule.md`'s calibration table interact. Also permit lightweight /
batched claims for related small items, and confirm §3's own "an existing
explicit handoff or claim… may satisfy this requirement" covers the
routine case.

## Advisory

### A1 — §7's every-specialist-YAML edit is a wide Gate 2 blast radius

The "Files expected to change" list edits every `agf/agents/*.yaml` specialist
plus templates, Pages, runbook, skill, and the projection manifest. Combined
with the BREAKING classification and six-surface projection obligations, the
atomic Gate 2 candidate will be large. Confirm it can be sliced or is bounded
enough for one exact-head review.

### A2 — §4 does not cover an *identified but unreachable* affected session

§4 routes to the governor when "an affected session cannot be identified", but
a peer session can be identified and simply be offline (as happened in this
session's own Propeller/capability-developer coordination). Add that an
identified-but-unreachable affected session also routes to the governor rather
than blocking the allocation indefinitely.

## Recommendation

**CHANGES REQUESTED.** The plan is strong — non-reentrant scheduling, the
exhaustive stop boundary, the no-authority-inflation guards, and maturity
honesty are all well done. Resolve C1 (reconcile the mandatory published claim
with `simplicity-rule.md`) before Gate 1; A1/A2 fold into the same revision.
Gate 1 remains the governor-reviewer's decision.
