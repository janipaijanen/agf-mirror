---
id: issue76-coord-visibility-performance
status: pending-gate1
topic: issue76-coord-visibility-performance
advisory_consultation:
  - role: contribution-auditor
    status: not-applicable
    reason: "This plan changes internal Git-object-loading performance and adds assistant-facing usage guidance; it does not change coordination-message schema, authorship attribution, or any contribution-audit-relevant surface."
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
  this plan's `--branch`/`--all-branches`/stale-vs-fetched divergence
  warning already addresses for `coord visibility` specifically, but the
  adopter's report names it as a broader pattern across queue-reporting
  commands.

This plan's scope remains `agov coord visibility` only (per "What this
plan does NOT do" below); it does not extend the batching/branch-selection
work to `handoff next` or other queue commands. It is recorded here as
acceptance evidence — a concrete adopter scenario `coord visibility`'s
explicit `--branch`/stale-ref-warning behavior must correctly handle (see
Required test 3) — and as a named, not-yet-scoped follow-on: whether
`handoff next` and similar queue commands need the same explicit-ref
treatment is a separate future issue, not authorized by this plan.

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

`tooling/rust/crates/agov/src/main.rs`:

- `visibility_scan` (line 3318) calls `git_coord_files` (an `ls-tree`) once
  per branch, then `load_git_messages` once per branch, and by default
  iterates **every local branch** (`git_local_branches`, line 3414, which
  lists `refs/heads` with no bound).
- `load_git_messages` (line 3436) calls `git_output(&["show",
  "<ref>:<file>"])` **in a loop, once per coordination file** (line 3439).
  `git_output` (line 3898) spawns a fresh `git` subprocess per call
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

`base_messages` (line 3325) is already computed once per `visibility_scan`
call, not per branch — confirm this holds after the batching change and
add a regression test asserting `load_git_messages` is called exactly
once for the base branch regardless of how many branches are scanned.

### 3. Bound or make default branch selection deliberate

Two independent problems currently collapse into one flag
(`--base-branch`):

- **Unbounded default scan.** With no `--branch` argument, the scanner
  visits every local branch. On a branch-heavy checkout (this project
  accumulated dozens of short-lived `coord/*`, `review/*`, and `fix/*`
  branches in a single session) this is the multiplicative-cost path.
- **Stale-local-vs-fetched-remote ambiguity.** `git_local_branches` never
  considers `refs/remotes/origin/*`, so a caller who has not run `git
  fetch` recently gets a scan against stale state with no warning.

Planned behavior:

- Add an explicit `--all-branches` flag (or equivalent) required to
  opt into the unbounded scan; without it, the command requires at least
  one `--branch` value or defaults to a small, clearly documented set
  (e.g., the current branch plus any branch with an open MR against the
  base, if that information is cheaply available; exact default is an
  implementation decision to resolve on the branch, not fixed here).
- When a local branch ref exists but its corresponding
  `refs/remotes/origin/<branch>` is present and diverges from it, emit an
  explicit warning identifying the local ref as potentially stale, rather
  than silently scanning it. Do not change the scanned ref automatically
  (correctness of "what did you actually ask me to scan" matters more
  than convenience here) — surface the divergence and let the caller
  decide.
- The unbounded, all-local-branches mode remains available (for
  maintenance/audit runs, per the requesting handoff) but stops being the
  silent default.

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
It" section to state explicitly:

- Fetch first (`git fetch origin`) before scanning, so `origin/main` is
  current.
- Prefer `--branch origin/main` (or the fetched equivalent of the base)
  over an unfetched local `main` for routine polling.
- Pass explicit `--branch <name>` values for routine polling against
  specific branches of interest, rather than the unbounded all-branches
  scan.
- Reserve the unbounded scan for maintenance/audit runs where covering
  every local branch matters more than latency.

This directly targets the false-positive mode observed in the evidence
(stale local `main` producing 168 KB of misleading findings vs. 2.3 KB of
accurate output against fetched `origin/main`).

## What this plan preserves (non-negotiable per the requesting handoff)

- **Deterministic output.** No change to which obligations/collisions are
  detected for a given set of inputs — only how the inputs are read.
- **Exit-code semantics.** `0` no findings, `1` unbridged obligations,
  `2` sequence collisions or scanner errors, unchanged (`visibility_exit_code`,
  line 3744) — non-zero for real obligations/collisions must remain
  distinct from a scanner operational failure (e.g., `git cat-file`
  itself failing to start), which should map to the existing `2`
  (scanner error) path, not a new exit code.
- **Output schema.** `VisibilityReport`'s JSON shape (`visibility_json`,
  line 3691) is unchanged; this is a purely internal I/O optimization.
- **Token-ROI telemetry** (`docs/reference/tooling-token-roi.md`'s
  existing redaction/attribution model) is unaffected — no new command
  surface, subcommand, or flag requiring `COMMAND_PATH_TOKENS`/
  `KNOWN_FLAG_NAMES` allowlist changes except the new `--all-branches`
  flag itself, which must be added to `agov-core/src/roi.rs`'s allowlists
  (per `roi_redaction_audit_matches_real_clap_surface`'s existing
  self-test) as part of implementation, not left to fail CI.

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
3. A test proving the stale-local-vs-fetched-remote divergence warning
   fires when a local branch ref diverges from its `refs/remotes/origin/*`
   counterpart, and does not fire when they match or no remote-tracking
   ref exists.
4. `roi_redaction_audit_matches_real_clap_surface` continues to pass with
   the new flag added to the allowlist.
5. Exit-code regression tests confirming `0`/`1`/`2` semantics are
   unchanged for representative fixtures of each case.

## Acceptance criteria

1. A scan of the evidence-scale checkout (or an equivalent-scale fixture)
   completes in materially less wall-clock time than the 15-second-plus
   baseline (an approximate, reported-in-the-MR figure — not a fixed
   numeric gate, since CI runner performance varies; the git-invocation-count
   test is the enforceable gate, wall-clock is reported evidence).
2. Scanning fetched `origin/main` and scanning stale local `main` no
   longer produce silently divergent results — the divergence is either
   surfaced as a warning or (if the caller passed an explicit ref) simply
   scanned as requested with no silent behavior change.
3. All five required tests pass.
4. `docs/reference/coord-visibility.md` reflects the new default-scan
   guidance.
