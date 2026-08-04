---
id: issue76-coord-visibility-performance
status: pending-gate1
topic: issue76-coord-visibility-performance
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Scheduling assessment `20260804T130826Z-contribution-auditor-3skh.md`
      found this plan's original default-branch-selection design (item 3,
      now removed) conflicted with MR !805's Slice 3 contract. Refreshed
      2026-08-04 against current `main` (line references, removed the
      conflicting design, narrowed to performance-only scope) per
      governor scheduling decision
      `20260804T134723Z-governor-reviewer-q4a8.md` and handoff
      `20260804T134725Z-contribution-auditor-nej7.md`. Requesting fresh
      coupled review alongside MR !805's refresh.
  - role: infosec-sentinel
    status: not-applicable
    reason: "No credentials, learning data, or external trust boundary is touched. The change is a local-only Git object read strategy; no new network calls or data exposure."
---

# Plan: `agov coord visibility` scaling for branch-heavy repositories

## Decision requested

Improve `agov coord visibility`'s performance for the next release while
preserving its deterministic hidden-obligation detection, its exit-code
semantics, and its token-ROI value for AI assistants. Issue #76 tracks the
work; this plan is planning only, per the requesting handoff
(`docs/coordination/20260727T161905Z-coder-tester-8dap.md`). No version,
tag, or release action is authorized here.

## Evidence

Observed on this checkout (2026-07-27 UTC, per the requesting handoff):

- Default all-local-branch scan exceeded 15 seconds and was still
  processing when traced.
- The 15-second trace recorded 787 attempted Git executions.
- Checkout scale at observation time: roughly 497 local+remote refs, 794
  coordination files.
- A targeted scan against **stale local** `main` took 6.1 seconds and
  emitted 168 KB of misleading branch-local findings (stale local `main`
  is missing coordination files that exist on `origin/main`, so the
  scanner reports them as "branch-local" when they are not).
- The same targeted branch scanned against **fetched** `origin/main` took
  5.6 seconds and emitted 2.3 KB of useful, accurate output.

### Adopter use-case input (routed from issue #78 finding 4)

Per governor-reviewer handoff `docs/coordination/
20260728T154327Z-governor-reviewer-gkj5.md` ("Route the coordination
branch/ref visibility evidence into issue #76's plan as an acceptance/
use-case input"): issue #78's adopter screening report (finding 4,
`kantele` downstream project, v2.4.5) independently reproduced the same
class of failure this plan already targets, from the opposite direction —
not slowness, but silent invisibility:

- At one point the reporting project had **14 coordination records live
  only on unmerged feature branches**, including an active handoff and
  two learnings, invisible to `agov session resume` on `main`.
- Status polls reported healthy queues repeatedly because `agov handoff
  next` and related queue commands read the working tree's checked-out
  ref, not a stated or caller-controlled one — the same class of ambiguity
  MR !805's stale-vs-fetched effective-base resolution addresses for
  `coord visibility` specifically (see "Boundary with MR !805" below),
  but the adopter's report names it as a broader pattern across
  queue-reporting commands.

This plan's scope remains `agov coord visibility`'s I/O performance only
(per "What this plan does NOT do" below); it does not extend the batching
work to `handoff next` or other queue commands, and it does not itself
implement branch-selection or stale-ref behavior (that is MR !805's
scope, see below). It is recorded here as acceptance evidence — a
concrete adopter scenario that MR !805's stale-ref/effective-base
behavior must correctly handle — and as a named, not-yet-scoped
follow-on: whether `handoff next` and similar queue commands need the
same explicit-ref treatment is a separate future issue, not authorized
by this plan.

### Relationship to issue #80 (resolved, no blocking dependency)

The prior portfolio adjudication (`docs/coordination/
20260728T181701Z-contribution-auditor-p51g.md`) placed this plan as
"Conditional v2.5 RC work after #80/ref correctness." Issue #80
(`requires-ack` obligation semantics) is now implemented: the schema
rejects `ack` + `requires-ack: true`, and `agov coord pending` tracks
every other kind's obligations via direct `references-to` resolution.
`coord pending` reads the same working-tree-relative `--dir` scope as
`coord visibility` and `handoff next` — it does not introduce a new
ref-scoping model this plan would need to reconcile with. This plan's
own scope (I/O batching and explicit branch selection for `coord
visibility`) is unaffected by and does not depend on #80's resolved
semantics; the adjudication's conditional is satisfied by #80 being
decided, not by any change to this plan's content.

## Root cause (read directly from the implementation)

`tooling/rust/crates/agov/src/main.rs` (line numbers refreshed against
current `main` at `95a8a9a6`; this plan's merge base is 711 commits
behind current `main` as of this revision — a uniform +190 offset from
this plan's original line numbers, confirmed by direct re-inspection of
each cited function, not assumed from the offset alone):

- `visibility_scan` (line 3508) calls `git_coord_files` (an `ls-tree`) once
  per branch, then `load_git_messages` once per branch, and by default
  iterates **every local branch** (`git_local_branches`, line 3604, which
  lists `refs/heads` with no bound — **this default-selection behavior is
  MR !805's contract to own and change, not this plan's; see "Boundary
  with MR !805" below**).
- `load_git_messages` (line 3626) calls `git_output(&["show",
  "<ref>:<file>"])` **in a loop, once per coordination file** (line 3629).
  `git_output` (line 4088) spawns a fresh `git` subprocess per call
  (`std::process::Command::new("git")`, no batching, no reuse).

For a checkout with `B` branches averaging `F` coordination files each,
the scan spawns on the order of `B * (1 + F)` subprocesses (one `ls-tree`
plus one `show` per file, per branch) — this is exactly what the 787-call
trace shows, and why the cost scales multiplicatively with both branch
count and file count rather than with the size of the actual diff between
branches (which is what the command's *stated purpose* — finding
branch-local obligations invisible from the base — actually needs to
scale with).

`git_local_branches` also does not fetch or consider remote branches,
which is why scanning a stale local `main` produces false positives:
coordination files that exist on `origin/main` (fetched, current) but not
on the local `main` ref (stale) are misreported as branch-local.

## Planned changes

### 1. Batch Git object reads

Replace the per-file `git show <ref>:<file>` loop in `load_git_messages`
with a single `git cat-file --batch` (or `--batch-check` +
`--batch-object-info` as appropriate) invocation per branch that reads all
requested blobs from one long-lived subprocess via stdin/stdout streaming,
rather than one process per file. This is the single largest cost
reduction: it turns `1 + F` subprocess spawns per branch into `1 + 1`
(one `ls-tree` to resolve blob OIDs, one `cat-file --batch` to read them
all).

Preserve `load_git_messages`'s existing per-message parsing
(`agov_core::yaml::parse_frontmatter_json`, `markdown_body`) unchanged —
only the I/O strategy changes, not the parsed output shape, so downstream
`VisibilityMessage` consumers are unaffected.

### 2. Parse/reuse base messages once per scan

`base_messages` (line 3515) is already computed once per `visibility_scan`
call, not per branch — confirm this holds after the batching change and
add a regression test asserting `load_git_messages` is called exactly
once for the base branch regardless of how many branches are scanned.

### 3. Boundary with MR !805 — this plan does not own branch selection

**Removed in this revision, per governor decision
`docs/coordination/20260804T134723Z-governor-reviewer-q4a8.md`:** the
prior draft's `--all-branches` opt-in flag, its "small, clearly documented
default set" design, and its stale-local-vs-fetched-remote divergence
*warning* (as opposed to correction) were an independently-designed
default-branch-selection contract that conflicts with
`docs/plans/agov-coordination-read-path-reliability.md` (MR !805), which
is issues #84/#85/#89's plan and — per that same governor decision — the
sole authority for default branch selection, effective-base resolution,
and remote-branch discovery semantics (MR !805's Slice 3: widen the
default scan to local + remote-tracking branches, prefer the fetched
upstream OID as effective base when local and upstream diverge, exclude
base-OID aliases from candidates).

This plan takes MR !805's default-selection contract as given and solves
only the affordability problem it creates: MR !805's widened default scan
(local + remote-tracking branches, potentially hundreds of refs on a
branch-heavy checkout) is exactly the scale this plan's batching work
(item 1 above) makes cheap. This plan does not itself decide which
branches are scanned by default, does not add an `--all-branches` flag,
and does not implement a stale-ref divergence warning — those are MR
!805's Slice 3 acceptance criteria. If MR !805 is not yet implemented
when this plan lands, this plan's batching still benefits the *existing*
(pre-!805) branch selection unchanged; if !805 lands first or
concurrently, this plan's batching applies transparently to !805's wider
default set with no additional design needed here.

### 4. Regression/performance fixtures

Add a test fixture representative of the observed scale (hundreds of
coordination files across dozens of branches — does not need to match 787
exactly, but should be large enough to make an accidental reintroduction
of per-file subprocess spawning show up as a slow test, not just a
production incident). Assert:

- Correctness is unchanged: the same unbridged-obligation and
  sequence-collision output for a fixed fixture, before and after the
  batching change (a golden-output comparison, not a new detection
  algorithm).
- A bound on subprocess count for a scan of `N` branches × `M` files
  (e.g., asserted via a test double or a Git-invocation counter, not a
  wall-clock timing assertion — timing assertions are flaky across CI
  runners; a call-count assertion is deterministic).

### 5. Assistant-facing documentation

Update `docs/reference/coord-visibility.md`'s "How AI Agents Should Use
It" section to state that the default scan is now fast enough (per this
plan's batching) to cover MR !805's widened default branch set (local +
remote-tracking) without the caller needing to pass explicit `--branch`
values purely for performance reasons. Branch-selection guidance itself
(fetch-first, explicit-ref preferences, when to use an unbounded scan) is
MR !805's documentation responsibility, since !805 owns the flags and
default-selection behavior this guidance describes — this plan's doc
update is scoped to reporting the performance characteristic (default
scan is now O(1) subprocesses per branch, not O(files)), not to
prescribing which branches to scan.

## What this plan preserves (non-negotiable per the requesting handoff)

- **Deterministic output.** No change to which obligations/collisions are
  detected for a given set of inputs — only how the inputs are read.
- **Exit-code semantics.** `0` no findings, `1` unbridged obligations,
  `2` sequence collisions or scanner errors, unchanged (`visibility_exit_code`,
  line 3934, refreshed) — non-zero for real obligations/collisions must
  remain distinct from a scanner operational failure (e.g., `git cat-file`
  itself failing to start), which should map to the existing `2`
  (scanner error) path, not a new exit code. This plan introduces no exit
  code change of its own; MR !805's exit-code changes (legacy-collision
  tolerance, remote-discovery-driven exit `0`→`1` transitions) are that
  plan's BREAKING classification, not this one's.
- **Output schema.** `VisibilityReport`'s JSON shape (`visibility_json`,
  line 3881, refreshed) is unchanged; this is a purely internal I/O
  optimization.
- **Token-ROI telemetry** (`docs/reference/tooling-token-roi.md`'s
  existing redaction/attribution model) is unaffected — this plan adds no
  new command surface, subcommand, or flag, so no
  `COMMAND_PATH_TOKENS`/`KNOWN_FLAG_NAMES` allowlist change is needed
  here (the round-1 draft's `--all-branches` flag and its associated
  allowlist requirement are removed along with the rest of the
  branch-selection design — see "Boundary with MR !805" above; any new
  flag MR !805 introduces is that plan's allowlist obligation).

## What this plan does NOT do

- Does not change `tooling/elixir` (out of scope; do-not-touch per the
  requesting handoff).
- Does not change detection logic for unbridged obligations or sequence
  collisions — only the I/O path that feeds that logic.
- Does not bump `agf/VERSION`, create a tag, or touch a release branch.
- Does not change the coordination-message schema or any other command's
  behavior.

## Required tests

1. Golden-output regression: identical `VisibilityReport` JSON for a fixed
   fixture, before and after the batching change.
2. Git-invocation-count assertion proving the batched path issues O(1)
   subprocesses per branch rather than O(files).
3. `roi_redaction_audit_matches_real_clap_surface` continues to pass
   unchanged (this plan adds no new flag).
4. Exit-code regression tests confirming `0`/`1`/`2` semantics are
   unchanged for representative fixtures of each case, under this plan's
   change alone (independent of MR !805's separately-tested exit-code
   BREAKING changes).

Removed in this revision: the round-1 draft's required test 3 (stale-ref
divergence-warning firing/non-firing) — that behavior belongs to MR
!805's Slice 3 and is tested there, not here.

## Acceptance criteria

1. A scan of the evidence-scale checkout (or an equivalent-scale fixture)
   completes in materially less wall-clock time than the 15-second-plus
   baseline (an approximate, reported-in-the-MR figure — not a fixed
   numeric gate, since CI runner performance varies; the git-invocation-count
   test is the enforceable gate, wall-clock is reported evidence).
2. All four required tests pass.
3. `docs/reference/coord-visibility.md` reflects the new performance
   characteristic (per item 5 above), without prescribing branch-selection
   behavior that belongs to MR !805's documentation scope.

Removed in this revision: the round-1 draft's acceptance criterion 2
(stale-vs-fetched divergence surfaced as a warning) — that is MR !805's
acceptance criterion, not this plan's.
