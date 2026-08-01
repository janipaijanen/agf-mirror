---
id: v250-rc1-contract-core
status: pending-gate1
topic: v250-rc1-contract-core
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: "Self-reviewed after drafting: verified every cited defect against live source (tooling/rust/crates/agov-coord/src/handoff_tracker.rs, agf/schemas/coordination-message.schema.json) and live GitLab issue/comment threads, not taken from issue descriptions alone. A renewed independent review of this exact plan head is still required before Gate 1, matching this project's established convention for planner-authored plans."
  - role: infosec-sentinel
    status: consulted
    reason: "#79 changes where adopter role-activation state is read and persisted across upgrades -- an authorization-adjacent surface under always/high-risk-operations-rule.md. Primary concern: ambiguous dual-manifest state (old shipped path plus new adopter-owned path both present with divergent content) must fail closed -- refuse to guess, not silently prefer one -- rather than fail open by picking a default that could reactivate a previously deactivated role. This plan requires that behavior explicitly in #79's acceptance criteria rather than leaving it to implementation-time judgment. #80 and #33's minimal slice are workflow-integrity concerns (obligation tracking, ID collision), not confidentiality/authorization surfaces, and carry no comparable risk."
  - role: fact-checker
    status: consulted
    reason: "Every claim below was independently re-verified against primary sources at plan-drafting time: the actual handoff_tracker.rs pending-computation logic (not assumed from the issue report), the actual coordination-message schema (requires-ack is an unqualified boolean; kind enum includes note/claim/review/ack/handoff/decision-record/fast-track-waiver/lesson-proposal), the actual current path of agf/agents/MANIFEST.yaml (still unrenamed), and issue #78's retraction/correction comments (not just its description) per the lesson recorded in agf/learnings/foundry.yaml's dzy7 entry."
---

# Plan: v2.5.0-rc.1 contract core (issues #79, #80, remaining #67, minimal #33)

## Decision requested

Authorize implementation of v2.5.0-rc.1's contract-core work package, per
`docs/plans/v250-statement-of-work.md`'s already-approved release train
(work package 3): issue #79 (upgrade-safe adopter role authority), issue #80
(`requires-ack` obligation semantics), the remaining issue #67 scope not yet
shipped, and the minimal issue #33 concurrency slice #80's implementation
actually depends on.

This plan is the required child Gate 1 plan the SoW itself demands before
any of these four items may be implemented ("Does not implement #75, #76,
#79, #80, #81, or any other work package — each requires its own child Gate
1 plan"). Planning only. No implementation, version bump, tag, or release
action is authorized by this plan itself.

## Why now, and what changed since the SoW

The SoW was written 2026-07-28, before v2.4.7 through v2.4.11 shipped. Two
things it assumed are now different, verified directly rather than assumed:

1. **#76 is no longer part of v2.5.0-rc.1's scope.** The SoW's original text
   included "#76's performance work only if... necessary for reliable
   release operation." The later-approved
   `docs/plans/post-v248-release-batching-amendment.md` (2026-07-31)
   re-routed #76 entirely to v2.6.0 ("v2.6.0 — coordination reliability.
   #84, #85, #89, and only justified correctness-preserving #76 work").
   Confirmed against the live issue tracker: issue #76 currently carries
   the `v2.6.0` milestone, not `v2.5.0`. This plan follows the batching
   amendment as the more recent governing document and excludes #76
   entirely.
2. **T0 (portfolio reconciliation) is substantially done; issue #75
   (lifecycle workflow) is not, and does not block this plan.** MR !858's
   portfolio reconciliation is merged and approved (exact head `86fb6b3b`,
   per the roadmap). Issue #75's own Gate 1 review (MR !745, reviewed in MR
   !750) returned **ADAPT**, not approved — the lifecycle-workflow plan
   needs a bounded resolution-scope clarification before it can proceed.
   Per the SoW's own T0 amendment, issue #75's workflow "runs in parallel
   with or after v2.4.6... under its own governed child plan," so its
   incomplete state does not block v2.5.0-rc.1.

## Work package 1 — Issue #79: upgrade-safe adopter role authority

### Verified current defect

`agf/agents/MANIFEST.yaml` is still the shipped path today — confirmed by
directly checking the current tree (`ls agf/agents/MANIFEST*.yaml` shows
only `MANIFEST.yaml`, not `MANIFEST.example.yaml`). Issue #67 Slice C
(`docs/plans/issue67-commissioning-defects-gate1-plan.md`, Gate 1 approved
2026-07-27) already decided the direction — rename the shipped file to
`MANIFEST.example.yaml` and document an adopter-owned copy path — but
explicitly deferred both the rename/documentation half and the required
`agov` tooling contract to a separate plan. Neither has shipped. This plan
is that separate plan for the tooling contract; the rename/documentation
half may ship in the same implementation slice since it has no additional
tooling dependency.

Issue #78 (adopter screening, kantele) independently reproduced the actual
failure mode on 2.4.5 with a concrete example: an adopter roster that had
diverged from the shipped example (`capability-developer,
documentation-writer, fact-checker, governor-reviewer, infosec-sentinel,
planner-architect`) would have `coder-tester` and `build-engineer`
silently reactivated and `documentation-writer`/`capability-developer`
silently dropped on a naive archive extract. This is not a hypothetical
risk description; it reproduces.

### Scope

- Select and document the adopter-owned canonical path for the role-roster
  manifest (candidate: project-root `governance/MANIFEST.yaml`, consistent
  with `governance-file-layout-rule.md`'s existing `GOVERNANCE_ROOT`
  convention). This is an adopter-facing tooling contract, not specific to
  this project's own layout — this project itself uses Layout B (confirmed:
  `agf/agents/MANIFEST.yaml`'s header comment states `Layout: B`), but the
  `agov` path-resolution logic must work correctly for a Layout A adopter
  too, per that rule's own detection method, not assume Layout B
  universally.
- Rename the shipped example to `agf/agents/MANIFEST.example.yaml`; update
  `ADOPTING.md` and `install-and-layouts.md` with one canonical statement
  (per `marketing-tone-rule.md`'s message-integrity rule) describing the
  adopter-owned copy step.
- Add `agov` support for a configurable/default adopter-owned manifest path,
  read in preference to the shipped example, with the shipped example never
  treated as authoritative adopter state.
- **Dual-manifest resolution MUST fail closed.** If both the legacy shipped
  path and the new adopter-owned path exist with materially different role
  rosters, `agov` MUST refuse to silently pick one — surface the conflict
  explicitly and require an adopter decision, rather than defaulting to
  either file. This is a direct requirement from the infosec-sentinel
  consultation above, not an implementation-time judgment call.
- Migration path for existing adopters whose roster already lives at the
  legacy `agf/agents/MANIFEST.yaml` path: document how to migrate without a
  window where role state is ambiguous or silently reset.
- Add an upgrade fixture that proves a divergent adopter roster survives an
  archive upgrade unchanged (the exact failure #78 reproduced).
- Align `agov validate agents`, `session resume`, documentation, templates,
  and release manifests to the new contract.

### Acceptance criteria

1. A fresh archive upgrade over an adopter project with a divergent roster
   at the new adopter-owned path leaves that roster byte-for-byte unchanged.
2. A conflicting-dual-manifest fixture (both paths present, different
   content) produces an explicit, actionable failure — not a silent
   pick-one default.
3. `agf/agents/MANIFEST.yaml` no longer exists in the shipped tree;
   `agf/agents/MANIFEST.example.yaml` does, and is never read as adopter
   state by `agov`.
4. Documentation states the adopter-owned path once, canonically, and every
   other reference links to it.

## Work package 2 — Issue #80: `requires-ack` obligation semantics

### Verified current defect

Read directly from `tooling/rust/crates/agov-coord/src/handoff_tracker.rs`:
`find_pending()` filters `.filter(|d| d.msg.kind() == Some("handoff"))` —
only `kind: handoff` messages are ever considered pending, regardless of
`requires-ack`. A test named `note_kind_is_never_pending()` confirms this is
current, deliberate, tested behavior, not an unnoticed gap. The schema
(`agf/schemas/coordination-message.schema.json`) places no such restriction:
`requires-ack` is an unqualified boolean, and the `kind` enum includes
`claim`, `completion`, `review`, `ack`, `handoff`, `decision-record`,
`fast-track-waiver`, `lesson-proposal`, and `note` — every one of which can
currently set `requires-ack: true` and have it silently ignored by pending
computation except for `handoff`.

Issue #78 independently retested this exact scenario on the published
v2.4.5 archive (`requires-ack: true, kind: note` → `graph pending: 0`) and
confirmed it reproduces identically to the original #67 finding. Not a
one-off; the same failure across two independent reproductions six days
apart on two different published versions.

### Design decision (recommended, for Gate 1 approval)

The issue frames two options: (1) broaden obligation tracking to every
permitted kind, or (2) restrict which kinds may set `requires-ack` and
reject the rest at the schema level.

**This plan recommends option 1 — broaden tracking, not restrict kinds** —
for governor decision:

- It is purely additive: nothing that currently validates stops validating;
  more things that already silently no-op start behaving as their author
  evidently intended.
- Restricting kinds is a breaking schema change for any adopter who has
  already used `requires-ack` on a non-handoff kind (schema-valid today),
  requiring a deprecation cycle under `deprecation-policy-rule.md` for a
  contract this project does not yet have confirmed external dependents on
  (Phase 0 per that rule) — broadening avoids that cost entirely, since it
  removes no capability an adopter could already be relying on.
- Every kind in the enum plausibly carries a real obligation once you read
  what it means: an unacknowledged `claim` or `decision-record` is exactly
  as legitimate a "someone needs to see this" case as an unacknowledged
  `handoff`.

If the governor prefers option 2 at Gate 1, this plan's acceptance criteria
below still apply with "broadened" read as "validated against the narrower
allowed set" — the plan does not depend on which option wins, only that one
is chosen and implemented consistently, which is the issue's own actual
requirement ("do not patch one surface before the contract is approved").

### Scope

- Implement the chosen option consistently across: schema (if option 2),
  `find_pending()` and any other pending-computation call site, graph/index
  output, `session resume`, `handoff next`, documentation, templates, and
  MCP/CLI surfaces — issue #67 finding #8 (learning-ID/coordination-ID
  format collision producing phantom `graph-index` edges) and finding #9
  (broken `agov coord graph-index docs/coordination/` template invocation)
  are both already fixed on current `main` (verified directly:
  `agf/templates/AGENTS.md` now uses the bare form at lines 100 and 157) —
  not re-scope them here, but do add a regression fixture for finding #9's
  specific defect class (a shipped template documenting a rejected CLI
  invocation) since none exists yet.
- Define recipient and resolution semantics precisely: what "acknowledges"
  an obligation for each kind, and who may resolve it.
- Add positive and adversarial fixtures for every permitted (and, if option
  2, rejected) kind × `requires-ack` combination.
- Diagnostics for already-published records that are accepted but currently
  untracked under the old behavior (do not silently reinterpret history;
  surface it).
- Preserve `agf/rules/always/tool-authority-boundary-rule.md`'s boundary:
  tooling may block only under a named rule, not an ad hoc judgment call.

### Acceptance criteria

1. A `requires-ack: true` record of the chosen permitted kind(s) appears in
   `session resume`, `handoff next`, and `coord graph-index`'s pending
   output — verified by an adversarial fixture, not just a positive case
   (per `boundary-shape-testing-rule.md`'s negative-test discipline).
2. A malformed or rejected combination (if option 2) fails as loudly and
   specifically as the existing missing-`handoff:`-block case (issue #67
   finding #10's own counter-example standard).
3. Every documented surface (docs, templates, MCP/CLI help) states the same
   contract in the same words.

## Work package 3 — Remaining issue #67 scope

Issue #67 Slices A and C's documentation half are already merged (verified:
the stale Quickstart redirect, packaged-tooling passage, ADR command paths,
and unauthenticated Releases API pointer are all corrected on current
`main`). Two items remain, per the governor's own triage
(`docs/coordination/20260728T154327Z-governor-reviewer-gkj5.md`):

- **Slice B completion:** two adopter-facing legacy Pages URLs still resolve
  to the superseded project name — confirmed present as of the triage
  (`agf/docs/governance/retrospective.md` and
  `agf/docs/governance/learning-system.md`, both citing
  `janipaijanen.gitlab.io/agent-governance/...`). Re-verify current state
  at implementation time (this plan's research is current as of drafting,
  not as of implementation). Add the CI link-check gate issue #67 already
  authorized (fails the pipeline on any occurrence of the superseded path
  or the bare Pages-host token inside the shipped `agf/` tree), with a
  negative self-test proving it actually fires, per
  `always/gate-self-test-rule.md`.
- **Slice C documentation half:** covered under work package 1 above (the
  `MANIFEST.example.yaml` rename and adopter-copy documentation) — not
  duplicated here.

### Acceptance criteria

1. Zero occurrences of the superseded repository path or bare
   `agent-governance` Pages-host token remain in the shipped `agf/` tree,
   excluding documented historical/immutable evidence citations.
2. The new CI link-check job is demonstrated to fail on a deliberately
   reintroduced occurrence, then pass once removed.

## Work package 4 — Minimal issue #33 slice

Issue #33's full scope (formal concurrency spec, merge/rebase semantics
documentation, locking redesign) is **not** in this plan — per the SoW,
only the slice #80 actually requires. What #80 requires: if obligation
tracking depends on readable-ID uniqueness to correctly attribute and
resolve obligations, concurrent writers producing colliding IDs would
corrupt that tracking silently.

**New evidence this plan can cite that issue #33 itself could not**: #33's
own "Evidence status" section states the concurrent-writer collision failure
mode is "theoretical... not yet observed in an AGF deployment." That is no
longer accurate as of this session: two agents (this session and a Codex
session) independently created git branches with the identical predictable
name `coordination/v2411-release-complete` roughly 30 seconds apart,
producing a real push rejection (`cannot lock ref... reference already
exists`) rather than a silent collision, because branch names collide
loudly while readable coordination-message IDs — generated the same way,
same timestamp granularity plus a short random suffix — are not guaranteed
to fail loudly the same way if two writers happen to generate the same
suffix in the same second. This is corroborating evidence for #33's
predicted failure mode, not yet a reproduction of the ID-collision case
specifically, and should be recorded as such (a learning record, separate
from this plan) rather than overclaimed here.

### Scope

- `agov validate` (or `coord validate`) detects readable-ID collisions
  across concurrently-created files and reports them explicitly, rather
  than silently accepting a last-write-wins file.
- Document the readable ID scheme's actual concurrency guarantee (per-writer
  ordering only; no global ordering claim) — a documentation clarification,
  not a locking redesign.
- Explicitly **not** in this slice: rebase semantics documentation, a
  lockfile/TTL redesign, non-git transports. Full issue #33 remains parked
  for a separate, later plan if further need is demonstrated.

### Acceptance criteria

1. A fixture with two concurrently-generated coordination files sharing the
   same readable ID is detected and reported by validation, not silently
   accepted.
2. The concurrency-guarantee documentation change lands alongside, not as a
   separate release.

## Explicitly out of scope

- Issue #76 (routed to v2.6.0 by the later batching amendment — see above).
- Trust-chain themes #42/#45/#47/#49/#34/#35/#82 (parked past v2.5.0 per the
  SoW's parking boundary, unchanged by this plan).
- Issue #75's lifecycle workflow (T0, separately gated, ADAPT status
  pending its own resolution).
- MR !901 (Four-Way Test consultation) and any v2.5.0 contract-core scope
  expansion beyond the four work packages above.
- `agf/VERSION`, any release branch, tag, or GitLab Release. This plan
  authorizes implementation only; `v2.5.0-rc.1` itself requires its own
  release-publication plan and Gate 2, per `release-via-merge-request-rule.md`
  and the SoW's own RC checkpoint discipline.
- Issue #97 closure follow-through, v2.4.11 or earlier release lines — both
  already complete and out of this plan's scope entirely.

## SemVer classification

Additive minor, matching the SoW's own framing ("v2.5.0-rc.1 — feature-
complete additive minor candidate"). #79 and #80 both add new adopter-facing
obligation/authority semantics that do not exist today — by definition not
patch-compatible under this project's own stability-promise discipline
(`docs/plans/v250-statement-of-work.md`'s "Why this matters" section). No
existing command, schema field, or documented behavior is removed by this
plan; #80's chosen option (if option 1) only adds tracking where none
existed, and if option 2, restricts a combination that was never
functionally meaningful in practice (schema-valid but silently inert) —
governor judgment on whether that restriction needs its own migration
note is requested explicitly at Gate 1, not assumed here.

## Validation requirements (Gate 2, for the eventual implementation MRs)

- Each work package ships as its own reviewable slice/branch, per
  `branch-and-worktree-rule.md`'s sizing test — this plan explicitly
  authorizes up to four separate implementation branches/MRs, not one
  combined branch, so a reviewer can answer one yes/no question per branch.
- Positive and adversarial fixtures for every acceptance criterion above,
  written before implementation per `tdd-rule.md`.
- `agov validate`, `agov coord validate`, `agov plan validate` all pass on
  every touched surface.
- No existing test weakened or deleted without a replacement, per
  `feature-regression-guard-rule.md`.
- Full workspace `cargo test` and strict Clippy pass for any Rust-side
  change (work packages 1, 2, 4 all touch `agov` behavior).

## Stop conditions

Stop and return to the governor if implementation of any work package
requires: expanding into #76 or any trust-chain theme; a breaking schema
change without an explicit governor-approved deprecation note (relevant if
#80's option 2 is chosen); ambiguous dual-manifest behavior that defaults
to any answer other than an explicit failure (#79); or scope creep into
issue #33's full concurrency-spec/locking redesign beyond the minimal slice
defined here.

## Acceptance criteria (this plan)

1. Governor-reviewer approves, amends, or rejects the four work packages,
   the #80 design-decision recommendation, and the explicit out-of-scope
   list.
2. Each work package's own acceptance criteria are treated as the binding
   Gate 2 bar for its implementation slice.
3. No implementation begins on any work package before this plan's Gate 1
   approval.
