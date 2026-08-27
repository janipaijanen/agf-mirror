---
id: v250-rc1-contract-core
status: approved
topic: v250-rc1-contract-core
gate1_approved_by: "governor-reviewer decision 20260802T072326Z-governor-reviewer; MR !913 exact head 50ce651a51e6379c0c9de737826d51b280c9407d"
gate1_approved_at: "2026-08-02T07:23:26Z"
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: "Codex's independent review at the prior exact head (20260802T054310Z-contribution-auditor-geh3.md, MR !916) returned BLOCK with four findings, all four independently re-verified against source before this revision: B1 (#79's canonical path/precedence/migration left undecided), B2 (#80's broaden-all-kinds option has no cycle/migration/CLI design), B3 (work package 3 already shipped in d9dfe51e, contained in v2.4.6-v2.4.11 -- confirmed d9dfe51e is an ancestor of this plan's prior exact head, meaning the drafting-time claim was already stale when written, not merely overtaken later), B4 (work package 4's validator already shipped in 40c77ac3, every tag since v2.3.0). This revision removes work packages 3 and 4, makes WP1 and WP2 concrete decisions instead of open choices, and requests renewed review."
  - role: infosec-sentinel
    status: consulted
    reason: "#79 changes where adopter role-activation state is read and persisted across upgrades -- an authorization-adjacent surface under always/high-risk-operations-rule.md. Primary concern: ambiguous dual-manifest state (old shipped path plus new adopter-owned path both present with divergent content) must fail closed -- refuse to guess, not silently prefer one -- rather than fail open by picking a default that could reactivate a previously deactivated role. This plan requires that behavior explicitly in #79's acceptance criteria rather than leaving it to implementation-time judgment. #80 is a workflow-integrity concern (obligation tracking), not a confidentiality/authorization surface, and carries no comparable risk."
  - role: fact-checker
    status: consulted
    reason: "Every claim below was independently re-verified against primary sources at plan-drafting time: the actual handoff_tracker.rs pending-computation logic (not assumed from the issue report), the actual coordination-message schema (requires-ack is an unqualified boolean; kind enum includes note/claim/review/ack/handoff/decision-record/fast-track-waiver/lesson-proposal), the actual current path of agf/agents/MANIFEST.yaml (still unrenamed), and issue #78's retraction/correction comments (not just its description) per the lesson recorded in agf/learnings/foundry.yaml's dzy7 entry."
---

# Plan: v2.5.0-rc.1 contract core (issues #79, #80)

## Decision requested

Authorize implementation of v2.5.0-rc.1's contract-core work package, per
`docs/plans/v250-statement-of-work.md`'s already-approved release train
(work package 3): issue #79 (upgrade-safe adopter role authority) and issue
#80 (`requires-ack` obligation semantics).

This plan is the required child Gate 1 plan the SoW itself demands before
either item may be implemented ("Does not implement #75, #76, #79, #80,
#81, or any other work package — each requires its own child Gate 1
plan"). Planning only. No implementation, version bump, tag, or release
action is authorized by this plan itself.

**Revision note.** The prior exact head (`d90c44d1`) also included a work
package 3 (remaining issue #67 scope) and a work package 4 (minimal issue
#33 slice). Codex's independent review (MR !916,
`20260802T054310Z-contribution-auditor-geh3.md`) found both already fully
satisfied by shipped code — WP3's claimed remaining Pages URLs were fixed
by `d9dfe51e` and shipped in v2.4.6 through v2.4.11; WP4's proposed
duplicate-ID validator already shipped in `40c77ac3` and every tagged
release since v2.3.0. Both findings independently reproduced against
current source before writing this revision (see "Removed work packages"
below). This plan now covers only #79 and #80, the two items where a real,
unshipped decision remains.

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

**Confirmed mechanism gap** (found while fixing `!914`, an unrelated
adjacent correction, and folded back in here since it directly bears on
this work package): `ADOPTING.md`'s "Confirm the layout" table already
documents `agf/agents/*.yaml` (agent definitions / role roster) as
adopter-owned, and separately states "**upstream-owned** files are
replaced on upgrade; **adopter-owned** files... are never touched by an
upgrade" (`install-and-layouts.md:98`). But no `agov upgrade` (or
equivalent) command exists — confirmed directly: `agov --help` lists no
upgrade/migrate subcommand, and no such command exists anywhere in
`tooling/rust/crates/`. The adopter-owned/upstream-owned distinction is
therefore a documentation promise with no mechanical enforcement today —
exactly the gap this work package needs to close. `governance-file-layout-
rule.md` was corrected in `!914` to state this distinction accurately (it
previously described a stale, unrelated dual-layout choice); this plan is
what makes the distinction real rather than documentation-only.

### Decision (concrete, replacing the prior open choice)

B1 correctly found the prior draft's Scope and Acceptance Criteria 3
described two different, mutually exclusive product contracts (keep the
path with upgrade protection, vs. relocate) without choosing between them.
Decided here, for Gate 1 review:

**Keep `agf/agents/MANIFEST.yaml` as the canonical adopter-owned path. Do
not relocate.**

Reasons this is the smaller, already-mostly-real option rather than a new
design: `agov_core::manifest_path()` (`tooling/rust/crates/agov-core/src/
lib.rs`) already resolves this path today, and already honors an
`AGOV_MANIFEST_PATH` environment-variable override with absolute,
deterministic precedence (`std::env::var_os("AGOV_MANIFEST_PATH")
.map(PathBuf::from).unwrap_or_else(default)`) — this is not new work, it
already ships and is already documented in the environment-variable
reference. Relocating would require inventing a new default path,
updating every existing adopter's already-correct file location, and
duplicating a precedence mechanism that already exists and already works.
The only actual defect (#78's reproduction) is that the framework's own
shipped release archive currently *also* places a file at this exact same
path — a packaging collision, not a path-design problem.

**Precedence (unchanged, restated as the plan's binding contract):**
`AGOV_MANIFEST_PATH` env var, if set, wins absolutely; otherwise
`agf/agents/MANIFEST.yaml`.

**Migration: none required for existing adopters.** Their roster already
lives at `agf/agents/MANIFEST.yaml` and does not move. The fix is entirely
on the framework-distribution side: the shipped release archive stops
containing a file at that path (renamed to `agf/agents/MANIFEST.example.yaml`
in the archive) so a plain archive re-extraction no longer collides with
or overwrites it. No `agov upgrade` command is built or required for this
work package — the protection comes from the archive's own file layout no
longer colliding with the adopter path, not from a new selective-merge
apply engine. Building a general `agov upgrade plan`/`apply` surface
remains explicitly out of scope here (see "Explicitly out of scope");
this narrower packaging fix resolves the actually-reproduced #78 defect
without it.

**Fail-closed scope, corrected to the real ambiguous case.** Under this
design there is no dual-manifest ambiguity in the normal case — a present
`agf/agents/MANIFEST.yaml` is always the adopter's live roster, and
`MANIFEST.example.yaml` coexisting with it is the expected, non-conflicting
steady state. The one real fail-closed requirement: if the default path
resolution is in effect (`AGOV_MANIFEST_PATH` unset) and
`agf/agents/MANIFEST.yaml` is absent, `agov` MUST NOT silently fall back to
reading `MANIFEST.example.yaml` as live adopter state — it MUST fail
explicitly ("no adopter manifest found at agf/agents/MANIFEST.yaml; copy
MANIFEST.example.yaml or set AGOV_MANIFEST_PATH"). This remains a direct
requirement from the infosec-sentinel consultation above (fail closed, no
silent default), narrowed to the case this design can actually produce.

### Scope

- Rename the shipped template to `agf/agents/MANIFEST.example.yaml` in the
  release archive; `agf/agents/MANIFEST.yaml` is no longer part of the
  shipped tree.
- Update `ADOPTING.md` and `install-and-layouts.md` with one canonical
  statement (per `marketing-tone-rule.md`'s message-integrity rule)
  describing the copy-from-example step for new adopters.
- Implement the fail-closed missing-manifest error above; add a fixture
  proving it fires (no silent fallback to the example file).
- Add a packaging fixture that proves a plain archive extraction over an
  adopter project with a divergent `agf/agents/MANIFEST.yaml` leaves that
  file byte-for-byte unchanged, because the archive no longer ships a
  colliding file at that path (the exact failure #78 reproduced).
- Align `agov validate agents`, `session resume`, documentation, templates,
  and release manifests to the renamed example file.

### Acceptance criteria

1. A fresh archive extraction over an adopter project with a divergent
   roster at `agf/agents/MANIFEST.yaml` leaves that file byte-for-byte
   unchanged, because the shipped archive contains no file at that path.
2. `agf/agents/MANIFEST.yaml` no longer exists in the shipped tree;
   `agf/agents/MANIFEST.example.yaml` does, and is never read as adopter
   state by `agov`.
3. With `AGOV_MANIFEST_PATH` unset and `agf/agents/MANIFEST.yaml` absent,
   `agov` fails explicitly rather than silently reading the example file.
4. Documentation states the adopter-owned path and the precedence rule
   once, canonically, and every other reference links to it.

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

### Design decision (concrete, replacing the prior open recommendation)

B2 correctly found the prior draft's "option 1, broaden tracking" text
underspecified in three concrete ways: (a) broadening `find_pending()`
naively would let `kind: ack` itself set `requires-ack: true`, with no
termination rule for the resulting ack-of-ack chain; (b) `handoff next` /
`handoff take` / `handoff complete` read the `handoff:` frontmatter object
directly (`render_handoff_summary()`), which non-handoff kinds do not
have, so broadening the pending filter alone gives non-handoff kinds no
actual resolution workflow; (c) ~380 already-published non-handoff
`requires-ack: true` records exist on current `main` today (independently
recounted for this revision: 228 review, 92 note, 41 decision-record, 10
lesson-proposal, 4 completion, 4 ack, 2 fast-track-waiver — order-of-
magnitude and shape match B2's cited 371, difference attributable to
records published between the two counts) with no defined cutoff, so a
naive implementation manufactures a large historical backlog on day one.

Decided here, addressing all three:

**(a) Cycle prevention — schema-level exception, not graph logic.** Add a
schema constraint: a record with `kind: ack` MUST NOT set
`requires-ack: true`. An acknowledgement requiring its own acknowledgement
is not a meaningful obligation, so forbidding it outright is a narrow,
correctly-scoped exception, not general cycle-detection machinery. Every
other kind in the enum (`claim`, `completion`, `review`, `handoff`,
`decision-record`, `fast-track-waiver`, `lesson-proposal`, `note`) remains
eligible to set `requires-ack: true`. This is the complete permitted-kind
matrix: all eight non-`ack` kinds eligible; `ack` itself excluded.

**(b) A new, separate, additive surface — `handoff next`/`take`/`complete`
are untouched.** Do not broaden `find_pending()` or any handoff-specific
command. Add a new, distinct command (`agov coord pending --all-kinds`, or
equivalent naming decided at implementation time) that lists every
`requires-ack: true` record of any non-`ack` kind not yet resolved.
**Resolution semantics, generalized from how handoff resolution already
works today:** a `requires-ack: true` record is resolved when any later
record's `references-to` includes its id — matching this project's
existing convention that "review alone does not clear pending; ack/
completion/decision-record resolves" (already the practiced norm; this
generalizes it to the mechanism rather than restricting it to specific
kinds). No change to `handoff next`/`take`/`complete`'s existing output,
behavior, or callers.

**(c) Historical cutoff — grandfather, do not backfill.** The new
`--all-kinds` surface only considers records with a `timestamp` at or
after `2026-08-02T22:01:54Z`. The ~380 existing non-handoff
`requires-ack: true` records are grandfathered — never appear in the new
pending output, and are not retroactively reinterpreted. This directly
avoids B2's "naive implementation would manufacture a very large
historical pending queue."

**Amendment (2026-08-02T22:01:54Z, governor-reviewer decision-record
`20260802T220154Z-governor-reviewer-sejb`).** The clause originally read
"this work package's own merge commit's date" — unimplementable pre-merge,
since a commit cannot embed its own future merge timestamp. Two
implementation attempts against that literal wording were correctly
rejected (`20260802T193425Z-contribution-auditor-rtga.md` finding B2;
renewed disposition `20260802T204834Z-contribution-auditor-ah7j.md`). The
governor amended the clause to the fixed value above — the amendment
decision-record's own timestamp — which is governed, known before
implementation, and later than every pre-existing non-handoff
`requires-ack: true` record at decision time, so it grandfathers that full
set rather than narrowing it. See
`20260802T210836Z-coder-tester-rmds.md` and
`20260802T213118Z-coder-tester-d7tm.md` for the request and a math
correction along the way (an earlier-timestamp option was proposed and
retracted for inverting the grandfathering direction).

### Scope

- Add the `kind: ack` + `requires-ack: true` schema rejection, with a
  negative-fixture test.
- Add the new `--all-kinds` pending surface (`agov coord pending` or the
  chosen equivalent name) as a distinct command/flag from `handoff next`;
  zero changes to `handoff next`/`take`/`complete`'s existing behavior.
- Implement the cutoff-timestamp grandfathering rule; add a fixture proving
  a pre-cutoff record does not appear and a post-cutoff record does.
- Add positive and adversarial fixtures for every permitted kind ×
  `requires-ack` combination, and the rejected `ack` + `requires-ack: true`
  combination.
- Preserve `agf/rules/always/tool-authority-boundary-rule.md`'s boundary:
  tooling may block only under a named rule, not an ad hoc judgment call.

### Acceptance criteria

1. A post-cutoff `requires-ack: true` record of any non-`ack` kind appears
   in the new `--all-kinds` pending output until a later record references
   its id; a pre-cutoff record of the same shape never appears — both
   verified by fixture, per `boundary-shape-testing-rule.md`'s
   negative-test discipline.
2. `handoff next`, `handoff take`, `handoff complete`, and `coord
   graph-index`'s existing pending/graph output are provably unchanged
   (existing tests for these commands pass unmodified).
3. A `kind: ack` record with `requires-ack: true` fails schema validation
   explicitly, with a fixture proving it.
4. Every documented surface (docs, templates, MCP/CLI help) states the same
   permitted-kind matrix, cutoff rule, and resolution semantics in the same
   words.

## Removed work packages (were 3 and 4 in the prior draft)

**Former work package 3 (remaining issue #67 scope) — removed, already
shipped.** The prior draft cited the governor's 2026-07-28 triage record
for two remaining legacy Pages URLs in `agf/docs/governance/
retrospective.md` and `agf/docs/governance/learning-system.md`. Directly
re-verified for this revision: zero `agent-governance` matches in either
file on current `main`. Commit `d9dfe51e` (2026-07-29) fixed both and is
an ancestor of this plan's own prior exact head — the claim was already
stale at drafting time, not merely overtaken afterward. The associated
advisory-role validator defect is also already fixed:
`plan_validator.rs` has a direct `fact-checker`-as-active-role acceptance
test on current `main`. Nothing remains to implement.

**Former work package 4 (minimal issue #33 slice) — removed, already
shipped.** Directory-aware duplicate-readable-ID detection
(`duplicate_id_errors_in_dirs`, `tooling/rust/crates/agov/src/main.rs`)
already exists, is directly tested
(`coord_validate_blocks_duplicate_ids_in_same_directory`), and has shipped
in every tagged release since v2.3.0 (commit `40c77ac3`), confirmed by
direct inspection for this revision. Work package 2's revised design above
does not introduce any new causal dependency on ID-collision detection
beyond what already exists and already ships — the #80 redesign resolves
obligations via `references-to`, which already relies on the same
existing ID-uniqueness guarantees `handoff` resolution has always used.
Issue #33's own concurrency-spec scope remains parked, unchanged, for a
separate later plan if further need is demonstrated — this plan does not
reopen or reclassify it.

## Explicitly out of scope

- Issue #76 (routed to v2.6.0 by the later batching amendment — see above).
- Trust-chain themes #42/#45/#47/#49/#34/#35/#82 (parked past v2.5.0 per the
  SoW's parking boundary, unchanged by this plan).
- Issue #75's lifecycle workflow (T0, separately gated, ADAPT status
  pending its own resolution).
- MR !901 (Four-Way Test consultation) and any v2.5.0 contract-core scope
  expansion beyond the two work packages above.
- Issue #67's remaining scope and issue #33's full concurrency spec — both
  addressed above under "Removed work packages": already shipped or
  already parked, neither reopened by this plan.
- A general `agov upgrade plan`/`apply` surface. Work package 1 resolves
  #78's reproduced defect via a packaging change (no colliding shipped
  file), not a new upgrade-mechanism engine. Building that engine, if ever
  needed, is a separate future plan's decision — not pre-empted here.
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
existing command, schema field, or documented behavior changes: work
package 1 only renames a shipped template file and adds a new fail-closed
error path; work package 2 adds an entirely new, separate pending surface
and leaves `handoff next`/`take`/`complete` provably unchanged (acceptance
criterion 2). The one narrowing — `kind: ack` may no longer set
`requires-ack: true` — rejects a combination that was schema-valid but
never functionally meaningful (silently inert under all current tooling);
governor judgment on whether this narrow exception needs its own
deprecation note under `deprecation-policy-rule.md` is requested explicitly
at Gate 1, not assumed here.

## Validation requirements (Gate 2, for the eventual implementation MRs)

- Each work package ships as its own reviewable slice/branch, per
  `branch-and-worktree-rule.md`'s sizing test — this plan explicitly
  authorizes up to two separate implementation branches/MRs, not one
  combined branch, so a reviewer can answer one yes/no question per branch.
- Positive and adversarial fixtures for every acceptance criterion above,
  written before implementation per `tdd-rule.md`.
- `agov validate`, `agov coord validate`, `agov plan validate` all pass on
  every touched surface.
- No existing test weakened or deleted without a replacement, per
  `feature-regression-guard-rule.md`. Work package 2's acceptance
  criterion 2 (handoff commands provably unchanged) is this rule applied
  directly to this plan's own design.
- Full workspace `cargo test` and strict Clippy pass for any Rust-side
  change (both work packages touch `agov` behavior).

## Stop conditions

Stop and return to the governor if implementation of either work package
requires: expanding into #76 or any trust-chain theme; a breaking schema
change beyond the single narrow `ack`-cannot-require-`ack` exception
without an explicit governor-approved deprecation note; any change to
`handoff next`/`take`/`complete`'s existing behavior; the fail-closed
missing-manifest error defaulting to any answer other than an explicit
failure (#79); or building a general `agov upgrade` planner/apply engine
beyond the packaging fix work package 1 defines.

## Acceptance criteria (this plan)

1. Governor-reviewer approves, amends, or rejects the two work packages,
   the #79 and #80 design decisions, and the explicit out-of-scope list.
2. Each work package's own acceptance criteria are treated as the binding
   Gate 2 bar for its implementation slice.
3. No implementation begins on either work package before this plan's
   Gate 1 approval.
