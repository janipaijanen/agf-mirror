---
id: 20260911T084336Z-agov-wrapper-fingerprint-collision-fix
status: approved
gate1_approved_by: governor-reviewer
gate1_approved_at: 2026-09-11T09:32:47Z
topic: agov-wrapper-fingerprint-variable-collision
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Ran `agov validate infosec` against the current tree: OK, no findings.
      This plan touches only `tooling/agov` (a local POSIX-shell variable
      rename) and its accompanying test fixture — no credential, schema, CI
      trigger, or agent-authority surface changes. The only sensitivity is
      that `tooling/agov` gates which binary every role's `agov` invocation
      trusts; the fix narrows that gate's correctness rather than weakening
      it (see "Why this is not a weakening" below).
  - role: contribution-auditor
    status: consulted
    reason: >-
      contribution-auditor (Codex) diagnosed and reproduced this exact
      defect in `20260803T235714Z-contribution-auditor-rnh2`, including the
      precise causal mechanism (shell variable collision between
      `run_rust`'s and `binary_is_fresh`'s use of the global
      `source_fingerprint` name) and independent confirmation via a manual
      build. This plan is the requested Gate 1 formalization of that
      diagnosis (`rnh2`'s own task: "Assess the reproduced wrapper
      variable-collision defect for Gate 1 planning"). Requesting a fresh
      confirmation pass on this draft before Gate 1, since a diagnosis
      authored over five weeks ago should be re-checked against current
      `main` before being treated as still accurate (it is — re-verified
      below, "Confirmation against current `main`").
---

# Plan: fix `agov` wrapper source-fingerprint variable collision

Formalizes `20260803T235714Z-contribution-auditor-rnh2` (handoff:
contribution-auditor → build-engineer, "Assess the reproduced wrapper
variable-collision defect for Gate 1 planning") into a Gate 1 plan, per that
handoff's explicit request. **Drafting only. Implementation is not
authorized until this plan is Gate 1 approved.**

## Confirmation against current `main`

Re-read `tooling/agov` at current `main` (`980f5773`, 2026-09-11) before
drafting this plan, since `rnh2`'s diagnosis is five weeks old. The defect
is unchanged and still present:

- `run_rust` (line 43) computes the expected source fingerprint into the
  global shell variable `source_fingerprint`.
- `run_rust` calls `binary_is_fresh "$rust_release" "$base_ref"
  "$source_fingerprint"` (line 48).
- `binary_is_fresh` (lines 129–170) captures the *expected* value into its
  own parameter, `expected_fingerprint=$3` — safe, a positional snapshot.
  But at line 140–141 it also parses the *candidate binary's own embedded*
  fingerprint out of that binary's `version --format json` provenance, and
  assigns it to the **same global name**, `source_fingerprint`. POSIX `sh`
  has no `local`; every un-declared assignment in every function in this
  script writes the one shared global namespace.
- After `binary_is_fresh` returns (regardless of its return code), the
  caller's `source_fingerprint` no longer holds the expected value — it
  holds whatever the last-inspected candidate binary had embedded.
- If no fresh local binary is found and `run_rust` falls through to the
  rebuild branch (line 58), `AGOV_BUILD_SOURCE_FINGERPRINT="$source_fingerprint"`
  now embeds the **stale, previously-inspected binary's** fingerprint into
  the freshly built one — not the correct expected value.
- The post-build freshness check (line 61–62) re-invokes `binary_is_fresh`
  with that same corrupted global as its "expected" argument, so it
  matches the binary that was just built with the same corrupted value —
  the rebuild is accepted as fresh, but the wrong digest is now baked in.
- The next unrelated `agov` invocation recomputes the true expected
  fingerprint from source (unaffected — that computation does not touch
  this global), compares it against the wrongly-embedded digest, finds a
  mismatch, and rebuilds again. With no intervening source change, this
  repeats every invocation — exactly the symptom `rnh2` observed (two
  consecutive `agov version --format json` calls, both rejecting and
  rebuilding).

This causal chain was re-derived independently from the current source
during this planning pass (not merely re-cited from `rnh2`), confirming the
diagnosis still holds.

## Why this is not a weakening

The existing `tooling/rust/tests/agov-wrapper-freshness.sh` harness
(`echo 'OK agov wrapper freshness gate'`) is functionally this project's
build-provenance gate: it is what makes an adopter or agent able to trust
that a locally-built `agov` binary actually reflects the checked-out
source. This fix narrows a hole in that gate (a rebuild can silently
converge on a wrong digest and then loop or, worse, in a differently-timed
race, accept a genuinely stale binary against a corrupted expectation) — it
does not relax any of the freshness checks the gate already performs
(commit provenance format, worktree-HEAD match, requested-base match all
stay exactly as strict). Per `always/gate-self-test-rule.md`, this is a
**modification to a gate** and therefore requires a negative test
demonstrating the gate still fires on its known-bad input after the change
— see "Gate 2 evidence required" below. It is not eligible for the
`simplicity-rule.md` fast-track regardless of diff size.

## Fix

Rename `binary_is_fresh`'s locally-scoped-in-intent variable so it no
longer aliases `run_rust`'s `source_fingerprint`. Minimal, single-purpose
rename — no other behavior change:

- In `binary_is_fresh` (currently lines 129–170), rename the assignment
  at line 140–141 from `source_fingerprint=...` /
  `source_fingerprint=${source_fingerprint%%\"*}` to a distinct name, e.g.
  `embedded_fingerprint`, and use that name in the line-153 comparison
  (`if [ "$embedded_fingerprint" != "$expected_fingerprint" ]`) instead of
  `source_fingerprint`.
- Audit the script's other global-reuse points for the same class of bug
  while the file is open, since POSIX `sh`'s shared-namespace hazard is
  structural, not limited to this one variable:
  - `base_ref` is reassigned identically inside `binary_is_fresh`
    (`base_ref=$2`) and again inside `base_matches_binary`
    (`base_ref=$2`) — in both cases the reassigned value is exactly the
    value already held by the caller's `base_ref` (a straight pass-through
    with no transformation), so this is redundant but not a bug. No
    rename required, but implementation should add a one-line comment
    noting why it's safe, so a future edit that *does* transform the value
    doesn't reintroduce the same class of defect silently.
  - `commit` is written inside `binary_is_fresh` and read inside
    `worktree_matches_binary`/`base_matches_binary` via explicit parameter
    passing (`commit=$1`), never read back by `run_rust` afterward — no
    bug, no rename required.
  - No other global name is written in more than one function with
    different intended meanings.
- No change to `tooling/rust/scripts/agov-source-fingerprint.sh`, cargo
  build invocation shape, or any other file.

## Gate 2 evidence required

1. **Negative-test-first proof (`always/gate-self-test-rule.md`)**: extend
   `tooling/rust/tests/agov-wrapper-freshness.sh` with a case that:
   - Seeds a `rust_release` fixture binary whose embedded
     `source_fingerprint` provenance is **wrong** (simulating exactly the
     corrupted-embed failure mode above — not merely "missing/absent",
     which the existing suite already covers).
   - Drives the fake-`cargo` rebuild path (already present in the harness,
     lines 188–208) and captures the rebuilt binary's newly embedded
     `source_fingerprint`.
   - Asserts the rebuilt binary's embedded fingerprint equals the
     **true** expected fingerprint (freshly computed via
     `agov-source-fingerprint.sh`), not the stale one that seeded the
     fixture.
   - Immediately re-invokes the wrapper a second time with **no
     intervening source change** and asserts it does **not** print
     "rebuilding stale or missing repository-local release binary" again
     — i.e., convergence in exactly one rebuild. This is the specific
     proof `rnh2` requested and the specific symptom reported.
   - Run this exact case against **unpatched** `tooling/agov` first and
     confirm it fails (demonstrating the test actually exercises the bug),
     then against the patched version and confirm it passes. Both
     transcripts are required MR evidence per the gate-self-test rule —
     "I tested it locally" without the transcript is insufficient.
2. **Full existing suite green**: `tooling/rust/tests/agov-wrapper-freshness.sh`
   passes unchanged end-to-end (no existing case's behavior changes; this
   fix only affects the corrupted-global path, which no existing case
   currently exercises).
3. **`cargo test --manifest-path tooling/rust/Cargo.toml`** green (no Rust
   source changes, but confirms nothing else assumes the old behavior).
4. **Manual spot check**: two consecutive `./tooling/agov version --format
   json` invocations with no intervening source change on this checkout,
   confirming no rebuild loop (the original symptom, reproduced live).

## Role and gate boundary

Per current role assignment: planner-architect (this plan) does not
implement. Implementation is `coder-tester` (currently played by Codex,
not this session) once this plan reaches `status: approved` via a
governor-reviewer Gate 1 coordination record. build-engineer's role here
was the diagnostic re-confirmation above and drafting-support; build-
engineer does not merge its own implementation review under
`agf/agents/MANIFEST.yaml`'s standing separation.

## Out of scope

- `h2dg`'s separate CI-prebuilt-artifact proposal (explicitly distinguished
  in `rnh2` as a different concern — CI bootstrap optimization, not local
  wrapper correctness). Not touched by this plan.
- Any change to `agov-source-fingerprint.sh`'s own hashing logic.
- Any change to what paths `freshness_paths` covers.

## Risks

- **Scope creep into a broader POSIX-sh-safety refactor**: explicitly
  rejected. The audit above identifies exactly one real bug (`source_fingerprint`)
  and two look-alike-but-safe reuses (`base_ref`, `commit`); only the one
  real bug gets a rename. Renaming already-safe variables adds churn to a
  gate file with no corresponding safety gain, and is out of scope for this
  plan.
- **Test harness complexity**: the freshness test fixture already
  simulates embedded provenance by reading env vars at fixture-binary
  invocation time rather than baking a value at "build" time in most
  existing cases; the new negative-test case must actually exercise the
  fake-`cargo` rebuild branch (like the existing "stale binary triggers
  exactly one release build" case, lines 185–208) rather than the simpler
  static-fixture cases, since the bug only manifests through that rebuild
  path. Flagged explicitly so implementation does not accidentally write a
  case that passes without ever exercising the corrupted-global write.

## Rollback

Single-file, single-variable-rename diff plus test-file addition; revert
the commit if a regression appears. No schema, data, or artifact migration.
