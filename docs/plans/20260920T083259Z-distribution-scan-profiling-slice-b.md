---
id: 20260920T083259Z-distribution-scan-profiling-slice-b
status: pending-gate1
topic: distribution-scan-profiling-slice-b
affected_paths:
  - tooling/rust/crates/agov/src/dist.rs
  - tooling/rust/crates/agov/src/main.rs
  - tooling/rust/crates/agov/tests/dist_scan_profile.rs
  - tooling/rust/crates/agov/tests/fixtures/dist_scan_profile.yaml
  - tooling/rust/crates/agov/Cargo.toml
  - docs/reviews/dist-scan-profiling/**
depends_on:
  - 20260913T093405Z-row5-s3-priority-ordering-and-challenge-response
  - 20260913T130049Z-qbka-slice-a-superseded-plan-lifecycle
  - 20260913T165127Z-row5-s4-cp-a-ack-pending-display
  - 20260913T195450Z-row5-s2-self-announcing-supersession
  - 20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Independent contribution-auditor review completed in two passes at
      this branch's exact head. First pass
      (`20260922T152955Z-contribution-auditor-srjp`) found two blocking
      findings (CA-3066-1: `tooling/rust/crates/agov/src/main.rs` missing
      from `affected_paths`; CA-3066-2: false existing-`safe_join`
      containment premise for an out-of-root symlink case). Both were
      fixed at this head (`main.rs` added to `affected_paths` with the
      resulting five plan-collision overlaps acknowledged via
      `depends_on`; the false containment claim and its fixture case
      removed, retaining only the required in-root symlink case). Second
      pass (`20260922T161950Z-contribution-auditor-hrr1`) confirms CLEAR:
      0 blocking, 0 advisory, 0 accepted-risk findings.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Read-only instrumentation and a diagnostic CLI subcommand added to
      the existing `agov dist` surface. No new network call, credential
      handling, write path, or authentication surface. The instrumented
      code path (`expand_includes` / glob matching) already runs during
      every `agov dist validate` invocation; this plan adds timing and
      call-count counters around existing reads, changes no filesystem
      write behaviour, and ships no new dependency.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: distribution-scan profiling instrumentation (Slice B, measurement-first)

## Provenance and scope boundary

This is the fresh, bounded Gate 1 plan requested by
`20260920T075012Z-planner-architect-jqag` ("Gate 1 for the
distribution-scan profiling slice only; hold the builder-image trial")
and corrected in scope by
`20260920T080017Z-coder-tester-s4ae` ("commission a fresh bounded Gate 1
plan for distribution-scan profiling/optimization only ... no-speedup-
until-measured boundary"). It carries forward exactly the Slice B half
of the pre-Gate-1 proposal MR !2679/!2678, independently reviewed CLEAR
as a proposal shape by `20260910T183418Z-contribution-auditor-ssms` and
by `20260910T182239Z-planner-architect-eeek`. Slice A (the native
builder-image trial) is explicitly **out of scope** and held per both
records above; it is not touched by anything in this plan.

This plan authorizes **profiling and instrumentation only** — it does
not authorize an optimization, an algorithm change, a caching layer, a
`git ls-files`-based membership substitute, or any other behavior change
to `agov dist validate`/`build`. Per issue #53's reconstruction
(`docs/reviews/issue-53-current-ci-latency-reconstruction-2026-09-19.md`,
conclusion 2): "September evidence keeps ... distribution traversal ...
as live measurement targets. It supports profiling and a bounded child
experiment, not a speedup claim." Any optimization implied by the
measurement below requires its own, separate Gate 1 plan.

Adding the CLI dispatch path makes the collision declaration complete and
reveals five approved Row 5/Row 6 plans that also touch `main.rs`. Their
implementations are already present on the current `main`; this plan must be
implemented afterward and preserve those command, lifecycle, display,
supersession, and collision-check surfaces. The five `depends_on` entries
record that deliberate sequencing and acknowledgement. They grant no scope to
alter those earlier features.

## Why this exists (source-verified problem statement)

`20260910T183418Z-contribution-auditor-ssms` independently read
`dist.rs`'s validation/build/matching paths and confirmed: "Exclusions/
forbidden patterns are evaluated per selected path using filesystem glob
expansion; dist preview invokes validation before build, whose
implementation validates again." `20260910T182239Z-planner-architect-eeek`
recorded an ad hoc "577k-directory-read `dist` diagnostic" from the
original proposal author, explicitly labelled **partial, not a measured
causal attribution** — no shipped tool currently reproduces that number.

Direct source re-inspection for this plan (`tooling/rust/crates/agov/src/
dist.rs`, current `main`) confirms two independently plausible cost
sources in the same code path, neither yet measured:

1. **`expand_recursive_prefix` / `collect_regular_files`** (lines
   332–396): every `/**`-suffixed include glob (there are 15 of them in
   `agf/dist/release-manifest.yaml` today) walks its subtree with an
   unbounded recursive `fs::read_dir`, once per `agov dist validate`
   call, and `validate` runs twice per `agov dist build` (`build` calls
   `validate` internally, and CI's `dist preview` step validates before
   build per `ssms`'s reading).
2. **`matches_any` / `glob_matches`** (lines 398–426): the exclude-list
   filter (`files.retain(|path| !matches_any(...))`) and
   `forbidden_errors` both call `matches_any` once per candidate file,
   per declared exclude/forbidden pattern. `pattern_prefix_match`
   short-circuits patterns with a literal `/**` suffix, but a pattern
   whose only `**` is a **leading** segment — e.g. the manifest's own
   `"**/__pycache__/**"` and `"**/*.pyc"` exclude entries — does not
   match `split_once("/**")`'s prefix shape cleanly and falls through to
   `glob_matches`, which re-invokes `glob::glob()` (a fresh recursive
   filesystem walk from `root`) for **every file being tested against
   that pattern**. With ~hundreds of included files and two such
   patterns, this is a plausible O(files × recursive-walk) cost, not yet
   measured.

Both are stated here as **hypotheses to instrument and measure**, not as
confirmed causes. Per `always/uncertainty-and-scope-rule.md`, this is
disclosed as source-inspection reasoning, not a benchmark result.

## Design

### 1. Call-count and wall-clock instrumentation (behavior-preserving)

Add a `#[cfg(any(test, feature = "dist-profile"))]`-gated counter struct
(or a plain `std::sync::atomic::AtomicU64` pair behind a `feature =
"dist-profile"` Cargo feature — no new dependency) recording, per
`agov dist validate` invocation:

- number of `fs::read_dir` calls made by `collect_regular_files`,
- number of all `glob::glob()` calls, split by origin: direct calls from
  `expand_glob`, and calls from `glob_matches` reached through `matches_any`,
- wall-clock duration of `expand_includes` and of the
  exclude/forbidden-filter pass, measured separately with
  `std::time::Instant`.

The feature flag guarantees zero behavior or performance change to the
default (non-profiling) build — the counters do not exist in ordinary
compilation. This satisfies the instrumentation's own read-only,
no-behavior-change security posture (the `infosec-sentinel`
`not-applicable` disposition above).

### 2. Diagnostic subcommand: `agov dist diagnose`

A new, additive `agov dist diagnose [--manifest <path>] [--json]`
subcommand (built only when the `dist-profile` feature is enabled,
consistent with the instrumentation above) that runs `validate()` once
against the real repository tree and prints the counters from design
step 1, plus total elapsed time. This replaces the proposal's original
ad hoc, unreproducible "577k" figure with a committed, rerunnable
diagnostic — closing exactly the gap `eeek` flagged ("no shipped tool
currently reproduces that number").

`agov dist diagnose` is diagnostic-only: it must not be wired into any
CI job, `agov:governance:validate`, `agf:dist:validate`, or any other
gate. It is a local/manual profiling tool for this plan's own
measurement step and any future optimization plan's baseline.

### 3. Fixed-fixture benchmark harness (the mandatory untracked-content fixture)

Per `20260910T182239Z-planner-architect-eeek`'s review note 2 (binding
on this slice): "the 'still detect relevant untracked filesystem
content; do not substitute `git ls-files` and lose that coverage' caveat
is the one most likely to regress silently — its fixture (untracked +
nested + symlink + hidden) is mandatory." This plan does not yet
implement any `git ls-files`-based change (that is explicitly deferred
to a future optimization plan), but the fixture is built now so that
baseline measurement and any future optimization measurement use the
*same* fixed, reproducible checkout shape — not a moving target.

**Corrected 2026-09-20** per Codex coder-tester implementation-readiness
feedback on MR !3066 (advisory author-side input, not the binding
independent contribution audit — Codex authored the parent !2679
proposal and explicitly disclaimed audit authority): do not commit
~500 synthetic fixture files. That adds a permanent repository/release-
scan footprint to test a transient scale property, and this plan's own
`affected_paths` already needs to stay narrow to avoid false
plan-collision overlap with unrelated `tests/**`-touching plans. Instead,
generate the fixture tree **at test runtime**, in a temporary directory,
from a small, fixed, committed set of generator parameters:

Commit a single small generator-input file,
`tooling/rust/crates/agov/tests/fixtures/dist_scan_profile.yaml`, stating
the fixed parameters: tree depth (>=4), regular-file count (~500),
untracked-file paths (including one nested 2+ directories deep), hidden
file/dir paths, and a valid in-root symlink target. An outside-resolving
symlink policy is deliberately excluded: current traversal follows symlinks,
and deciding or changing that semantic/security policy requires its own plan.
A committed `#[test]` in
`tooling/rust/crates/agov/tests/dist_scan_profile.rs`:

1. reads the generator-input file,
2. deterministically builds the tree under a fresh `tempfile::tempdir()`
   from those exact parameters (fixed, not random — the same input
   always produces the same tree shape and file count, so measurements
   stay reproducible across machines and CI runners without committing
   the generated files themselves),
3. `git init`s the temporary directory and commits only the baseline
   (tracked) subset the generator-input file designates,
4. writes the untracked, nested-untracked, and hidden files **after**
   that commit, so they are genuinely untracked on disk — exercising
   real tracked/untracked semantics rather than simulating them,
5. creates the valid in-root symlink required by the earlier review,
6. writes a `release-manifest.yaml`-shaped manifest (schema_version 1)
   into the temporary tree, matching the real manifest's *pattern
   shape* (`/**` includes, a leading-`**` exclude pattern equivalent to
   `"**/__pycache__/**"`) at this small, fixed scale, so pattern-shape
   cost (design hypothesis 2 above) is measurable in isolation from
   tree-size cost (hypothesis 1),
7. asserts the generated tree matches the generator-input file's exact
   declared counts (untracked, nested-untracked, hidden, in-root symlink) before
   any `dist::validate()` call — a fixture-integrity check, so a future
   change to the generator cannot silently drift the fixture shape.

This preserves the mandatory untracked/nested/symlink/hidden coverage
`eeek`'s review note requires, and preserves reproducibility (the
generator-input file is the reproducible artifact, not hundreds of
generated files), while avoiding hundreds of low-information repository
objects. `affected_paths` above reflects this: one generator-input file
plus the test file, not a `fixtures/dist_scan_profile/**` glob of
generated content.

The same committed `#[test]` then runs `agov::dist::validate()` against
the generated fixture tree (via the `dist-profile` feature) and asserts:

- the untracked, nested-untracked, hidden, and valid-symlink files are
  correctly included or excluded per the fixture manifest's own
  include/exclude declarations (a correctness assertion — this is the
  regression guard `eeek` required, run in ordinary `cargo test`, no
  feature flag needed for the correctness half),
- under the `dist-profile` feature, the recorded counters are within an
  expected range for the fixture's fixed size (a sanity check that
  instrumentation is wired up, not a performance assertion — this plan
  makes no speedup or ceiling claim).

### 4. Measurement run and report

Once steps 1–3 land, run `agov dist diagnose` against both the fixture
(design step 3) and the real repository tree, on the same machine,
several times each (cold and warm filesystem-cache runs, matching the
issue #53 report's own "cold and warm branches" measurement discipline).
Record the raw counter/timing output in a new file,
`docs/reviews/dist-scan-profiling/<UTC-timestamp>-baseline.md`, stating:

- exact commit SHA measured,
- machine/environment description (local vs. CI runner — if CI, the
  exact job and runner tags),
- raw counts and timings for both fixture and real-tree runs,
- explicit statement of what this measurement does and does not
  establish (matching issue #53's own "per-job values ... must not be
  summed into a pipeline critical path" discipline for CI-only
  comparisons).

This report is the plan's deliverable evidence. It is **not** itself an
optimization proposal. If the measurement confirms either hypothesis in
"Why this exists" above (or finds a different bottleneck, or finds
neither hypothesis material), the report states that finding plainly,
and any resulting optimization is scoped as a new, separate Gate 1 plan
citing this report as its evidence — per `AGENTS.md`'s own gate
discipline, this plan cannot pre-authorize that follow-on work.

## What this plan explicitly does NOT do

- No change to `agov dist validate`/`build` runtime behavior, output,
  exit codes, or the release-manifest schema.
- No `git ls-files`-based membership substitute (the exact caveat
  `eeek` flagged as needing its own fixture before any such change).
- No new external dependency (no `criterion` or similar — the counters
  use `std::sync::atomic` and `std::time::Instant`, already available).
- No CI wiring of `agov dist diagnose`.
- No claim of a measured speedup, causal attribution, or optimization
  recommendation — this plan produces a baseline measurement only.
- No touch to Slice A (native builder image) in any form.
- No decision, assertion, or behavior change for outside-resolving symlinks;
  that distribution-security policy is a separate governed concern.

## Test and acceptance

- `cargo test --manifest-path tooling/rust/Cargo.toml -p agov` — the new
  fixture correctness test passes without the `dist-profile` feature
  (proving the fixture/correctness assertions are independent of the
  instrumentation).
- `cargo test --manifest-path tooling/rust/Cargo.toml -p agov --features dist-profile` —
  the same test plus the counter sanity assertions pass.
- `cargo build --manifest-path tooling/rust/Cargo.toml -p agov` (default,
  no feature) — confirms zero-cost default build; `agov dist diagnose`
  is absent/unrecognized in this build.
- Manual: `agov dist diagnose` run against the real manifest and the
  fixture, output captured verbatim into the baseline report (design
  step 4).
- Acceptance = a governor Gate 1 decision record, following contribution-
  auditor consultation per the `advisory_consultation` row above.

## Risks and rollback

- **Risk:** the feature-gated instrumentation accidentally compiles into
  the default build path, changing binary size or behavior. Mitigation:
  explicit `cfg(feature = "dist-profile")` gating verified by the
  no-feature build/test acceptance step above.
- **Risk:** the fixed-fixture tree size (~500 generated files) is not
  representative of the real manifest's current scale, making the
  fixture measurement non-predictive of real-tree cost. Mitigation: this
  plan measures both the fixture *and* the real tree (design step 4);
  the fixture exists for reproducibility and pattern-shape isolation,
  not as a substitute for the real-tree number.
- **Rollback:** revert the new `dist-profile` feature, the `dist
  diagnose` subcommand, the fixture generator-input file, and the new
  test file. No schema, manifest, or release-artifact change to unwind,
  and nothing generated at test runtime persists outside the temporary
  directory it was created in.

## Related

- `20260920T075012Z-planner-architect-jqag` — commissioning record.
- `20260920T080017Z-coder-tester-s4ae` — independent correction narrowing
  this plan to profiling/optimization only, holding Slice A.
- `20260910T183418Z-contribution-auditor-ssms` — independent CLEAR on the
  original !2679 proposal shape; source of the untracked/nested/symlink/
  hidden fixture requirement and the "no benchmark run" limitation this
  plan closes.
- `20260910T182239Z-planner-architect-eeek` — source of the "577k"
  ad hoc figure this plan replaces with a reproducible diagnostic, and
  of the mandatory-fixture review note.
- `20260922T152955Z-contribution-auditor-srjp` — fresh exact-head audit;
  this revision adds the omitted CLI dispatch path, states the counter
  origins precisely, and removes the false existing-symlink-containment
  premise and out-of-scope escape fixture.
- `docs/reviews/issue-53-current-ci-latency-reconstruction-2026-09-19.md` —
  the CI-latency evidence this plan's measurement target derives from.
- Codex coder-tester note on MR !3066 (2026-09-20, GitLab MR discussion,
  not a coordination file — advisory implementation-readiness input,
  explicitly not the binding independent contribution audit since Codex
  authored the parent !2679 proposal) — source of the 2026-09-20
  correction replacing ~500 committed fixture files with test-runtime
  generation from a small committed generator-input file.
- `agf/rules/workflow/simplicity-rule.md` — no new dependency, minimal
  feature-gated instrumentation.
- `agf/rules/workflow/development-workflow-rule.md`,
  `agf/rules/workflow/tdd-rule.md` — the fixture correctness test is
  written and asserted before any optimization is proposed.
