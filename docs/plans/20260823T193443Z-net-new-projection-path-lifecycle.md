---
id: net-new-projection-path-lifecycle
status: approved
gate1_approved_by: governor-reviewer
gate1_approved_at: "2026-08-24T20:35:40Z"
topic: net-new-projection-path-lifecycle
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      CURRENT STATE (round 14, fixes `CA-NNP-R14P-3`'s stale-round-label
      finding -- this label was not updated across rounds 12-14; per
      the review-convergence directive `20260824T163113Z-governor-
      reviewer-36rr.md`, round 14 is the final full clean-room review
      for this plan): fourteen fresh isolated Codex sessions (fifteen
      review records; round 13 had two parallel sessions) have
      reviewed this plan, each independent of the authoring session and
      of each other. Round
      1 (`20260823T195011Z-contribution-auditor-psbj.md`, disclosing it
      had earlier helped route the `am2p` obligation into issue #115
      but did not author or materially design this plan's `stage:
      planned` mechanism, granularity choice, CI proposal, tests, or
      documentation boundary): COMPLETE BLOCK, 7 findings
      (`CA-NNP-R1..7`), fixed. Round 2
      (`20260824T071632Z-contribution-auditor-s4n4.md`): COMPLETE BLOCK,
      6 findings (`CA-NNP-R2-1..6`), fixed — most notably retracting a
      false schema-rejection test claim and reclassifying a schema
      narrowing as BREAKING. Round 3
      (`20260824T074839Z-contribution-auditor-h5sr.md`): COMPLETE BLOCK,
      5 findings (`CA-NNP-R3-1..5`), fixed — the round-2 CI redesign's
      plan-status correlation broke against the real manifest corpus and
      deadlocked on `approved` status; removed entirely in favor of a
      simpler, purely per-manifest relaxation. Round 4
      (`20260824T085422Z-contribution-auditor-rrm8.md`): COMPLETE BLOCK,
      3 findings + 1 advisory (`CA-NNP-R4-1..3`, `CA-NNP-R4-A1`), fixed
      — round 3's removal of all plan-status correlation also erased the
      Gate-1-vs-Gate-2 distinction; fixed by naming the already-existing
      direct, single-manifest, no-flag `agov plan projection-check` as
      the mandatory Gate 2 acceptance evidence. Round 5
      (`20260824T093347Z-contribution-auditor-shpq.md`): COMPLETE BLOCK,
      3 findings + 1 advisory (`CA-NNP-R5-1..3`, `CA-NNP-R5-A1`), fixed
      — a stale front-matter recurrence, a contradictory Gate 2
      negative-evidence specification, a missing `CHANGELOG.md` in the
      BREAKING migration scope, and residual inaccurate
      implementation-status language. Round 6
      (`20260824T101217Z-contribution-auditor-j3q4.md`):
      COMPLETE BLOCK, 2 findings + 1 advisory (`CA-NNP-R6-1..2`,
      `CA-NNP-R6-A1`), fixed — the round-5 negative-evidence fixture
      itself used an input the new gate is designed to *accept*, not
      reject; the BREAKING migration contract omitted a second
      currently-valid manifest shape the new staleness check newly
      rejects; and the live description still called the proposed
      staleness check "existing." Round 7
      (`20260824T105026Z-contribution-auditor-sr4b.md`):
      COMPLETE BLOCK, 4 findings + 1 advisory (`CA-NNP-R7-1..4`,
      `CA-NNP-R7-A1`), fixed — the promised full-tree CI safety net is
      not selected on every pipeline that could make an owner-only path
      resolve; this implementation's own Gate 2 evidence named a real
      candidate manifest it does not actually carry, and a no-argument
      command that cannot see an isolated fixture file; and four
      operative compatibility/rollback sections still denied the second
      BREAKING case round 6 added to the formal Impact block. Round 8
      (`20260824T113551Z-contribution-auditor-d2r8.md`):
      COMPLETE BLOCK, 3 findings (`CA-NNP-R8-1..3`), fixed — the
      round-7 CI-selection fix had no self-test proving the job is
      actually selected and blocks; `--repo-root` was absent from the
      implementation manifest and its Git-boundary semantics were
      under-specified; and one operative opt-in rationale still denied
      the newly classified default-command break. Round 9
      (`20260824T122318Z-contribution-auditor-skek.md`, this revision):
      COMPLETE BLOCK, 2 findings (`CA-NNP-R9-1..2`), fixed — the
      round-8 `--repo-root` correction's "schema resolution stays
      embedded" claim was factually false (the validator reads the
      schema from the runtime filesystem) and left distribution-input
      isolation completely undecided; and the new `ci_rules` self-test
      module was never registered in `agov-validators`'s `lib.rs`, so
      it would not compile or run. Round 10
      (`20260824T131241Z-contribution-auditor-8hg1.md`, this revision):
      COMPLETE BLOCK, 2 findings (`CA-NNP-R10-1..2`), fixed — the
      round-9 `AGOV_SCHEMA_DIR`-conditional-set mechanism silently loses
      to the supported `tooling/agov` wrapper's own unconditional
      `AGOV_SCHEMA_DIR` export, so the promised fixture-schema boundary
      never actually applies on the standard operator path; and adding
      `AGOV_REPO_ROOT` to the shared `agov_core::repo_root()` function
      created an undocumented, untested global cross-command
      environment contract far beyond this one command's scope. See the
      plan body's "Round 1" through "Round 10 correction" sections. Round
      11 (`20260824T140020Z-contribution-auditor-mkps.md`, this
      revision): COMPLETE BLOCK, 2 findings (`CA-NNP-R11-1..2`), fixed —
      the round-10 explicit `schema_path` parameter stopped at
      `validate_file` and never threaded through
      `validate_file_against`, leaving the diff-scoped changed-manifest
      path still reading the caller checkout's schema; and the CI
      job-selection self-test modeled only job `rules:`, never this
      repository's own top-level `workflow: rules:` admission gate that
      suppresses a non-default-branch push pipeline whenever the branch
      already has an open MR. See the plan body's "Round 1" through
      "Round 11 correction" sections. Round 12
      (`20260824T145228Z-contribution-auditor-krpk.md`, this revision):
      COMPLETE BLOCK, 2 findings + 1 advisory (`CA-NNP-R12-1..2`,
      `CA-NNP-R12-A1`), fixed — the proposed `normativeOwnerEntry`
      schema added `stage` but never required `path`/`paths` when
      `stage: planned`, so a pathless planned marker passed vacuously
      with no owner ever named; and the existing `resolves()` path
      resolver has no repository-containment check, so an absolute,
      `..`-escaping, or outside-resolving-symlink path could still
      resolve under `--repo-root <fixture>` despite the plan's repeated
      fixture-isolation claims. Also narrowed the CI-selection
      self-test's "proves GitLab actually creates the job" language to
      "structural, local-model agreement," since no live-platform
      dependency was added. See the plan body's "Round 1" through
      "Round 12 correction" sections. Round 13: two parallel isolated
      Codex sessions reviewed the round-12 correction independently;
      `20260824T155539Z-contribution-auditor-bnt5.md` returned CLEAR,
      but `20260824T155602Z-contribution-auditor-smqm.md` (this
      revision) returned COMPLETE BLOCK, 2 findings + 1 advisory
      (`CA-NNP-R13-1..2`, `CA-NNP-R13-A1`) — this project's discipline
      treats any blocking finding from any independent reviewer session
      as authoritative regardless of a concurrent CLEAR, so all three
      were fixed: three operative plan sections ("Files expected to
      change," "Compatibility scope, precisely," "Rollback") still
      described only two of the four round-12 breaking cases; the live
      GitLab MR description retained three claims the plan body had
      already retracted rounds earlier ("unchanged everywhere,"
      "strictly additive," `plan_id` correlation); and the Gate 2
      evidence summary still overstated local-model output as
      "actually created"/"actually blocks," with no defined supported-
      expression subset or fail-closed behavior for the CI evaluator.
      See the plan body's "Round 1" through "Round 13 correction"
      sections. Round 14
      (`20260824T162316Z-contribution-auditor-4t37.md`, this revision):
      COMPLETE BLOCK, 1 finding (`CA-NNP-R14-1`), fixed — the round-12
      containment check and the pre-existing `stage: planned`
      relaxation both keyed off `resolves()`'s single boolean, so a
      repository-escaping path (absolute, `..`, or an outside-
      resolving symlink) on a `normative_owner.stage: planned` entry
      was indistinguishable from legitimate absence and was silently
      *accepted* by `--allow-planned` — reopening the exact boundary
      round 12 closed, through the interaction between that fix and
      the older lifecycle branch. Fixed by replacing the boolean with a
      three-way `PathClassification` (`ContainedExisting`,
      `ContainedMissing`, `Invalid`) and making `Invalid` unconditionally
      blocking regardless of `stage`/`--allow-planned`, never
      relaxable. See the plan body's "Round 1" through "Round 14
      correction" sections. Per convergence directive `36rr` and
      governor ratification `20260824T175134Z-governor-reviewer-q28m.md`,
      one targeted final verification
      (`20260824T175442Z-contribution-auditor-gpkh.md`) then confirmed
      `CA-NNP-R14P-2`/`CA-NNP-R14P-3` resolved but found `CA-NNP-R14-1`
      still open: `classify()`'s `ContainedMissing` branch accepted a
      missing leaf without checking whether a symlinked ancestor (or a
      dangling symlink at the leaf) escapes the repository. Fixed by
      walking every path component and rejecting any escaping symlink
      found along the way, at any depth, not only at the final leaf.
      One more targeted check of this specific residual gap — not a
      new full round — is requested next.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      This plan changes a JSON Schema, a Rust CLI validator, and a
      governance rule's documentation. It adds no credential, no
      network-facing surface, no new external dependency, and no change
      to authentication, authorization, or secrets handling. The
      validator's existing "objective blocking authority only" design
      constraint is preserved, not relaxed.
---

# Plan: net-new normative-owner paths across Gate 1 and Gate 2 projection validation

## Round 1 correction (fixes the COMPLETE independent audit, `psbj`)

A fresh isolated Codex session performed the requested independent
audit: `20260823T195011Z-contribution-auditor-psbj.md`. Recommendation:
COMPLETE BLOCK, 7 findings, all independently verified against the
actual repository state (not accepted at face value) and fixed:

- **`CA-NNP-R1`** (scope overreach) — the design added `stage` to the
  shared `surfaceEntry` definition used by all six rows, and the
  validator skipped path checks for *any* surface marked `planned`, when
  issue #115 and `am2p` concern only the normative-owner row. Fixed:
  narrowed the schema to a dedicated `normativeOwnerEntry` definition
  used *only* by `surfaces.normative_owner`; every other row keeps the
  plain `surfaceEntry` with no `stage` property at all. The validator
  change is narrowed to match — see "Validator change" below.
- **`CA-NNP-R2`** (deferred contract decisions) — both open questions
  are now decided, not left to implementation review: whole-`surfaceEntry`
  granularity (already the plan's stated default) is now the plan's
  actual decision, and the CI wiring is fully specified, naming
  `.gitlab-ci.yml` and the exact `plan_id` → plan-`status` correlation
  mechanism. See "Open design questions" (now "Design decisions") below.
- **`CA-NNP-R3`** (stale plan-stage evidence accepted at Gate 2) — the
  original test strategy called a stale `stage: planned` marker on an
  already-existing path "harmless." Fixed: this is now a rejected state.
  A new invariant — checked regardless of `--allow-planned` — flags
  `stage: planned` paired with a path that already resolves as an error
  ("lifecycle transition incomplete"), with a dedicated negative test.
- **`CA-NNP-R4`** (adopter documentation parity omitted) — fixed per
  `downstream-impact-rule.md`'s "Adopter docs parity" section (read in
  full before this correction, not assumed from a stale reading — see
  that section's exact three allowed outcomes). This plan uses
  `Adopter docs parity: updated`, citing the rule-text subsection itself
  (adopter-shipped via `agf/rules/**`) as the explanation; see "Adopter
  docs parity" below for the full disposition and why no separate Pages
  page is added.
- **`CA-NNP-R5`** (missing gate-self-test artifact requirement) — fixed:
  added an explicit Gate 2 acceptance requirement for a reproducible
  negative CLI demonstration with its exact command, expected and actual
  output/exit code, and a visible pipeline log reference, per
  `gate-self-test-rule.md`.
- **`CA-NNP-R6`** (live MR description unfilled) — fixed: replaced with
  a description conforming to `merge-request-description-rule.md`'s six
  required headings, synced to GitLab in the same action as this
  revision.
- **`CA-NNP-R7`** (compatibility/rollback claims overstated) — fixed:
  the compatibility analysis now accounts for `surfaceEntry`'s existing
  `additionalProperties` openness (an adopter manifest could already set
  an arbitrary `stage`-named key with any value on the *generic*
  definition; this plan's dedicated `normativeOwnerEntry` narrows the
  newly-constrained surface to exactly the one row, bounding — not
  eliminating — the collision risk), and rollback now gives
  lifecycle-aware steps for a manifest that already depends on
  `--allow-planned` at the time of a hypothetical revert.

## Round 2 correction (fixes the COMPLETE isolated audit, `s4n4`)

A fresh isolated Codex session performed a second independent audit:
`20260824T071632Z-contribution-auditor-s4n4.md`. Recommendation:
COMPLETE BLOCK, 6 findings, all independently verified against the
actual repository state (not accepted at face value) and fixed:

- **`CA-NNP-R2-1`** (broken plan correlation) — the CI loop constructed
  `docs/plans/${plan_id}.md` directly, but `plan_id` is the plan's
  front-matter `id:` field (`net-new-projection-path-lifecycle`), while
  the actual filename is timestamp-prefixed
  (`docs/plans/20260823T193443Z-net-new-projection-path-lifecycle.md`)
  — verified directly against this plan's own filename, which the
  proposed lookup could not find for itself. Fixed: correlation moves
  out of the CI script and into a new `agov` subcommand (see
  `CA-NNP-R2-6` below) that searches `docs/plans/*.md` for the plan
  whose parsed front-matter `id` exactly matches, failing closed on
  zero or multiple matches.
- **`CA-NNP-R2-2`** (stale manifest invisible to Gate 2) — the CI loop
  only validated files present in `git diff`, so a Gate 2 branch that
  creates the normative-owner file without also touching the manifest
  never re-triggers the check, and the staleness invariant from
  `CA-NNP-R3` never runs. Fixed: the new canonical CI entrypoint
  validates **every** manifest in the repository on every pipeline run
  (not diff-scoped), so a manifest with a newly-resolvable path is
  caught regardless of what else changed in that particular branch.
- **`CA-NNP-R2-3`** (impossible schema-rejection test) — the test
  strategy claimed `stage: planned` on a non-`normative_owner` row
  "fails schema validation," but `surfaceEntry` has no
  `additionalProperties: false` (verified directly against the schema
  file — the key is absent, so JSON Schema's default `true` applies);
  a local fixture with `ai_landing_loaders.stage: planned` passed the
  real validator with exit 0. Fixed: the false claim is retracted; the
  test instead proves the actually-true, actually-provable behavior —
  the extra `stage` key is accepted and ignored, but the planned-path
  *exemption* still never applies outside `normative_owner`, because no
  other row's parsed Rust type has a `stage` field to read at all.
- **`CA-NNP-R2-4`** (narrowing misclassified as additive) — the
  compatibility-scope paragraph (fixing `CA-NNP-R7`, round 1) already
  acknowledged that an existing manifest setting
  `normative_owner.stage` to an arbitrary non-enum value would become
  schema-invalid under the new constraint, yet the Downstream impact
  block still classified the whole change ADDITIVE. Per
  `downstream-impact-rule.md`, narrowing a previously-permitted value
  is BREAKING regardless of how unlikely or absent-in-corpus. Fixed:
  reclassified BREAKING for the `normative_owner.stage` narrowing
  specifically, with discovery/migration/rollback guidance.
- **`CA-NNP-R2-5`** (stale design text remains authoritative) — the
  "Mechanism" section (predating the `CA-NNP-R1` narrowing fix) still
  described `stage` as living on the shared `surfaceEntry` and applying
  to "normative-owner (or other surface)," and still pointed at a
  now-nonexistent "Open design questions" section instead of the
  decided "Design decisions" section. Fixed: rewritten to state only
  the corrected, decided contract; historical framing removed from
  present-tense text. The live MR description is also refreshed to name
  the actual corrected exact head and pipeline, replacing the stale
  `48f522e6` label.
- **`CA-NNP-R2-6`** (undeclared CI dependency) — the proposed CI loop
  shelled out to `python3 -c "import yaml..."`, but the job inherits
  `.rust-base`, which installs neither Python nor PyYAML — verified
  directly against `.gitlab-ci.yml`'s job definitions. Fixed: the
  correlation logic moves entirely into a new `agov` subcommand (Rust,
  already-provisioned in the same job image, already parsing this exact
  YAML shape elsewhere in the codebase) — see "Design decisions" below.
  This single fix resolves `CA-NNP-R2-1`, `CA-NNP-R2-2`, and
  `CA-NNP-R2-6` together: they were three facets of the same underlying
  flaw (bash+python glue attempting plan correlation externally,
  diff-scoped, with an unprovisioned dependency) rather than three
  independent problems.

## Round 3 correction (fixes the COMPLETE isolated audit, `h5sr`)

A third fresh isolated Codex session audited the round-2-corrected exact
head: `20260824T074839Z-contribution-auditor-h5sr.md`. Recommendation:
COMPLETE BLOCK, 5 findings, all independently verified against the
actual repository state (not accepted at face value) and fixed:

- **`CA-NNP-R3-1`** (correlation fails the real corpus) — verified
  directly against every existing manifest: `20260809T163051Z-v250-
  maintenance-release-line-rule.yaml`'s `plan_id` includes the timestamp
  prefix, but its plan's actual front-matter `id` is
  `v250-maintenance-release-line-rule` (no prefix) — zero exact-match
  correlation. `20260813T155743Z-frozen-scope-reconstruction-rule-
  amendment.yaml` has the identical mismatch shape. `cl4-immutable-
  corpus-binding-gate1-plan.yaml` has no matching plan file in the tree
  at all. Rolling out the round-2 design as written would fail every
  pipeline in the repository on three pre-existing, unrelated manifests,
  every time, for reasons that have nothing to do with this plan's own
  feature.
- **`CA-NNP-R3-2`** (approved-status deadlock) — the round-2 design
  switched to strict resolution the moment a linked plan's `status`
  left `pending-gate1`, including `approved`. But `AGENTS.md`'s own
  Gate discipline creates the owner file *during* Gate 2 implementation,
  strictly after Gate 1 approval — there is a real, expected window
  where a plan is `approved` and its owner file still does not exist.
  Under the round-2 design, every pipeline on the entire repository
  (not just this plan's own branch) would start failing the moment any
  plan's Gate 1 approval merged to `main`, until that plan's Gate 2
  implementation also landed.
- **`CA-NNP-R3-3`** (changed-pages coverage breaks for unrelated
  manifests) — the existing `validate_file_against` function also
  requires a manifest's `changed_pages_or_runbooks` to cover every
  Pages/runbook file changed against `--base-ref` in the *current diff*.
  Running that same check unconditionally against every manifest in the
  tree (not just the ones actually part of the current diff) means an
  ordinary, correctly-manifested single-page change would fail every
  *other*, unrelated manifest for not declaring that unrelated page.
- **`CA-NNP-R3-4`** (stale consultation metadata) — the front matter
  still said "CURRENT STATE (round 1)" citing only `psbj`, omitting the
  published `s4n4` round-2 audit and its six findings entirely. Fixed:
  refreshed to the accurate round-3 state.
- **`CA-NNP-R3-5`** (MR description missing pipeline evidence) — the
  live description's `Pipeline:` line was left as a placeholder
  ("re-check on push") even after the exact-head pipeline had actually
  completed successfully. Fixed: the description now names the exact
  pipeline and its passing jobs.

**`CA-NNP-R3-1` and `CA-NNP-R3-2` together mean plan-status correlation
should not exist at all, not merely be corrected.** Both findings trace
to the same root design mistake: gating the planned-path relaxation on
an external plan file's `status` field. Fixed by removing that
mechanism entirely — see the revised "Design decisions" below. The
relaxation is now purely a function of each manifest's own
`normative_owner.stage` field, exactly like the existing single-manifest
`--allow-planned` flag, with the newly-specified unconditional
staleness check (fixes `CA-NNP-R3`, round 1; round 5, fixes part of
`CA-NNP-R5-A1` — corrected from "already-implemented": this plan is
Gate-1 plan-only and the check does not exist in the shipped
validator yet) remaining the sole
mechanism preventing indefinite abuse: the moment an owner path actually
exists, a manifest still marked `stage: planned` for it fails
immediately and permanently, regardless of any plan's status. No plan
correlation, no plan-file search, and no `plan_id` lookup is needed for
this mechanism at all — `CA-NNP-R2-1`'s broken filename construction is
resolved by elimination, not by a more careful re-implementation.
`CA-NNP-R3-3`'s coverage-check conflict is fixed by splitting
`projection-check-all`'s two concerns: full-tree, non-diff-scoped
path-resolution and staleness checking (every manifest, every run —
this is what actually fixes `CA-NNP-R2-2`), plus changed-pages coverage
checking scoped *only* to manifests that are themselves part of the
current diff (this project's existing, correct, unchanged behavior for
that specific check).

## Round 4 correction (fixes the COMPLETE isolated audit, `rrm8`)

A fourth fresh isolated Codex session audited the round-3-corrected exact
head: `20260824T085422Z-contribution-auditor-rrm8.md`. Recommendation:
COMPLETE BLOCK, 3 findings + 1 advisory, all independently verified
against the actual plan text (not accepted at face value) and fixed:

- **`CA-NNP-R4-1`** (critical — Gate 2 boundary erased) — round 3
  removed plan-status correlation entirely, correctly fixing `CA-NNP-
  R3-1`/`CA-NNP-R3-2`, but in doing so removed *every* signal
  distinguishing "this owner is legitimately still pending, Gate 1
  stage" from "this owner was supposed to exist by now, Gate 2 stage."
  The unconditional staleness check only fires once the owner path
  *exists*; an owner that stays absent forever — including one
  presented as Gate 2 evidence without ever having been created —
  passes `projection-check-all` indefinitely, contradicting issue
  #115's requirement that the manifest fully resolve before Gate 2.
  Fixed: `projection-check-all` remains the tree-wide, always-relaxed
  safety net for routine CI (catching staleness once an owner
  appears, on any branch, per round 2/3) — but a **separate,
  additional requirement** is added: before requesting Gate 2 for any
  plan using `stage: planned`, the implementer MUST run the direct,
  strict, single-manifest `agov plan projection-check <manifest>
  --base-ref <ref>` (no `--allow-planned`) against the exact manifest
  being handed off, and include its passing output as Gate 2
  acceptance evidence. That command already exists, is already strict
  by default, and needs no new code — it was simply never named as
  the actual Gate 2 gate. A manifest whose owner is still absent fails
  this command every time, so Gate 2 cannot be granted while the
  owner doesn't exist, regardless of what the tree-wide relaxed check
  says.
- **`CA-NNP-R4-2`** (gate-self-test evidence targets the wrong gate) —
  the required negative demonstration exercised the pre-existing
  single-manifest strict command, proving old behavior, not the new
  `projection-check-all` route or its new staleness error. Fixed: the
  required Gate 2 evidence for *this plan's own implementation* now
  requires **both** demonstrations — `projection-check-all` against an
  unchanged `stage: planned` manifest whose owner now exists (the new
  staleness path), and the direct strict single-manifest command
  against a still-absent owner (the `CA-NNP-R4-1` fix above) — with
  exact commands, expected/actual output, and pipeline job links for
  each.
- **`CA-NNP-R4-3`** (untested `paths` array semantics) — the plan
  promised whole-entry granularity covers both `path` and every
  element of `paths`, but never specified the staleness check's exact
  behavior for a `paths` array with mixed existing/missing members, and
  no test exercised it. Fixed: the staleness check fires the moment
  **any** member of `path`/`paths` resolves while `stage` is still
  `planned` — the conservative, fail-closed reading, since a `planned`
  marker asserts none of the entry's declared paths exist yet, and any
  one resolving path already makes that assertion false. New
  positive/negative tests cover array-only and mixed-resolution
  entries in both relaxed and strict modes.
- **`CA-NNP-R4-A1`** (advisory — describes mechanisms that don't
  exist) — "auto-correlating" and "discovers and correlates its own
  inputs" are residual round-2 language; round 3 removed all
  correlation. The non-owner-row test also attributed the guard to a
  "parsed Rust type" / "type system" boundary, but the validator
  operates on raw `serde_json::Value`, not per-row typed structs.
  Fixed: correlation language removed; the guard is now described
  behaviorally (only the `normative_owner` surface's `stage` field is
  ever read or has any effect) without asserting a specific
  implementation mechanism Gate 2 hasn't actually chosen yet.

Implements GitLab issue [#115](https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/115),
the governed corrective-work carrier required by governor decision
`20260823T191334Z-governor-reviewer-am2p.md`'s one-time, non-precedential
waiver for MR !1817. This plan does not touch MR !1817, does not alter or
extend that waiver, and does not authorize implementation before its own
Gate 1 approval.

## Round 5 correction (fixes the COMPLETE isolated audit, `shpq`)

A fresh isolated Codex session performed a fifth independent audit:
`20260824T093347Z-contribution-auditor-shpq.md`. Recommendation: COMPLETE
BLOCK, 3 findings + 1 advisory, all independently verified against the
actual repository state (not accepted at face value) and fixed:

- **`CA-NNP-R5-1`** (required consultation metadata is round-3 stale
  again) — the front-matter reason still said `CURRENT STATE (round 3)`,
  a recurrence of the same cold-start evidence split previously blocked
  as `CA-NNP-R3-4`. Fixed: front matter updated to name all five rounds
  through this one, matching the plan body.
- **`CA-NNP-R5-2`** (required Gate 2 negative artifact cannot use the
  exact handed-off manifest as specified) — the round-4 "Rule text
  changes" lifecycle correctly requires the direct no-flag command to
  **pass** against the exact manifest being handed off at Gate 2, but
  the round-4 "Gate 2 acceptance evidence" bullet separately required
  that same command to run against "the specific manifest being handed
  off for Gate 2, on a still-absent owner" and **fail** — two mutually
  exclusive states for one artifact. Fixed: split into three distinct,
  clearly-labeled pieces of evidence (see "Gate 2 acceptance evidence,
  revised" under "Test strategy") — a positive tree-wide staleness
  demonstration, a negative fixture proving the gate fires on a
  known-bad input (explicitly labeled a fixture, never the real
  handed-off manifest), and a positive direct-strict-command run against
  the actual manifest being handed off. Also corrected the Test
  strategy's "(not a synthetic fixture)" phrasing, which wrongly implied
  a test fixture was the real handed-off manifest.
- **`CA-NNP-R5-3`** (promised BREAKING release note absent from
  implementation scope) — the plan promises a `CHANGELOG.md` entry under
  "Downstream impact classification" but never listed `CHANGELOG.md` in
  "Files expected to change," and no test/acceptance item required it.
  Fixed: added to "Files expected to change" and to a new Gate 2
  acceptance-evidence bullet in "Test strategy."
- **`CA-NNP-R5-A1`** (advisory — residual prose still claims nonexistent
  types and already-implemented checks) — "not part of any other row's
  schema or parsed type" implied a typed-struct boundary the validator
  does not have (it operates on raw `serde_json::Value`); "already-
  implemented" (twice) and "existing" described the proposed
  unconditional staleness check as if it already existed in the shipped
  validator, when this plan is Gate-1 plan-only and the check does not
  exist yet. Fixed: all three corrected to accurate language.

## Round 6 correction (fixes the COMPLETE isolated audit, `j3q4`)

A fresh isolated Codex session performed a sixth independent audit:
`20260824T101217Z-contribution-auditor-j3q4.md`. Recommendation:
COMPLETE BLOCK, 2 findings + 1 advisory, all independently verified
against the actual plan text (not accepted at face value) and fixed:

- **`CA-NNP-R6-1`** (the revised negative self-test uses an input the
  new gate must accept) — round 5's "Gate 2 acceptance evidence" item 2
  used a fixture with `stage: planned` on an *absent* owner path as the
  required negative demonstration for `projection-check-all` — but
  that exact shape is the case the tree-wide relaxed route is
  *designed to pass* (per the plan's own "Design decisions" and the
  Test strategy's own `CA-NNP-R3-2` regression test), making the
  required nonzero output impossible. Verified directly against the
  plan's own design text. Fixed: swapped the roles — item 1 is now the
  positive Gate-1-relaxed-pass case (absent owner, still `planned`);
  item 2 is now the negative gate-self-test case (a `stage: planned`
  owner whose path *now resolves* — the actual lifecycle-transition-
  incomplete/stale shape the check exists to reject).
- **`CA-NNP-R6-2`** (BREAKING migration contract omits an
  already-valid `stage: planned` case) — the impact classification
  named only the non-enum-`stage`-value case as BREAKING, but an
  adopter manifest that already sets `stage: planned` on a path that
  already resolves is *also* currently valid (today's validator has no
  staleness check) and becomes rejected by this plan's new
  unconditional staleness check — a second, previously-undisclosed
  breaking case. Verified: this repository's own corpus has no `stage`
  usage at all, which proves nothing about adopter-owned manifests
  outside it — the same evidentiary boundary the plan already applied
  to the non-enum case. Fixed: extended "Direct adopters," "Downstream,"
  and "Migration" to cover both breaking cases; extended the
  `CHANGELOG.md` scope note; corrected the live MR description's
  "Default (no-flag) validator behavior is unchanged everywhere" claim,
  which glossed over this same gap for the tree-wide command.
- **`CA-NNP-R6-A1`** (advisory — live description still calls the
  proposed staleness check "existing") — the MR description's Round 3
  governance-trail bullet said "the existing unconditional staleness
  check," predating round 5's fix to the plan body's own language for
  the same overstatement. Fixed in the live description as part of
  this handoff.

## Round 7 correction (fixes the COMPLETE isolated audit, `sr4b`)

A fresh isolated Codex session performed a seventh independent audit:
`20260824T105026Z-contribution-auditor-sr4b.md`. Recommendation:
COMPLETE BLOCK, 4 findings + 1 advisory, all independently verified
against the actual plan and CI text (not accepted at face value) and
fixed:

- **`CA-NNP-R7-1`** (the full-tree staleness gate is not selected on
  every owner-only pipeline) — the plan says `projection-check-all`
  validates every manifest "on every pipeline run," but its only CI
  edit swaps the command *inside* the existing `agov:governance:
  validate` job, whose `changes:` trigger list does not include
  `agf/rules/**/*` or any other open-ended owner-artifact path.
  Verified directly against `.gitlab-ci.yml`'s actual `rules:changes`
  anchor. A candidate that creates a new `agf/rules/**` owner while
  leaving its `stage: planned` manifest textually unchanged would not
  select this job at all, reproducing `CA-NNP-R2-2`'s silent-owner-only
  gap. Fixed: the job's `changes:` path-scoping is dropped entirely for
  this job, making it run unconditionally on every relevant pipeline —
  the same unconditional pattern `commercial-corpus:validate` already
  uses in this same file, affordable because this job's own measured
  runtime (~200-300s) is under `ci-trigger-scoping-rule.md`'s 5-minute
  quick-feedback threshold.
- **`CA-NNP-R7-2`** (this implementation has no real Gate 2 manifest for
  required evidence item 3) — the round-6 "Gate 2 acceptance evidence"
  required a direct-strict-command pass against "the exact manifest
  being handed off for this plan's own Gate 2," but verified directly
  that this plan's own "Files expected to change" adds no
  `docs/reviews/rule-projection/*.yaml` manifest at all — it ships the
  CLI feature, not a net-new normative-owner candidate. Fixed: removed
  the impossible "real candidate manifest" requirement from this
  plan's own Gate 2 evidence; the direct-strict-command-against-a-real-
  manifest requirement is now explicitly scoped to the future-user
  lifecycle in "Rule text changes," owed by whichever later plan
  actually adds a `stage: planned` manifest, not by this one.
- **`CA-NNP-R7-3`** (the no-argument gate cannot consume the stated
  isolated negative fixture) — `projection-check-all` takes no manifest
  argument and discovers only the real repository's own
  `docs/reviews/rule-projection/*.yaml` files, so an "isolated fixture
  file" outside that tree is invisible to it, and a fixture placed
  inside the canonical directory would permanently break real
  validation. Verified directly against the command's own documented
  discovery behavior. Fixed: added an optional `--repo-root <path>` flag
  to `projection-check-all`, mirroring the existing `commercial-corpus
  validate --repo-root` precedent already shipped in `.gitlab-ci.yml` —
  Gate 2 acceptance evidence now runs the real binary against a
  temporary, isolated fixture git repository via `--repo-root`, fully
  reproducible and never touching the canonical corpus.
- **`CA-NNP-R7-4`** (operative compatibility and rollback prose still
  denies the second breaking case) — round 6 added the `stage:
  planned`-on-resolving-path case to the formal Impact block, but four
  other sections still claimed "zero migration and zero risk," "one
  narrow BREAKING exception," or called the staleness check "strictly
  additive" in the same breath as describing a manifest that newly
  fails. Verified each of the four locations directly. Fixed: all four
  corrected to disclose both BREAKING cases consistently, and resolved
  a deeper pre-existing inconsistency this exposed — whether the
  single-manifest no-flag command's own logic gains the staleness check
  (it does, "regardless of `--allow-planned`," per this plan's own
  round-1 text, which an earlier round-7 draft of one passage had
  gotten backwards).
- **Advisory `CA-NNP-R7-A1`** (live MR verification omits the current
  exact pipeline and jobs) — fixed in the live description as part of
  this handoff.

## Round 8 correction (fixes the COMPLETE isolated audit, `d2r8`)

A fresh isolated Codex session performed an eighth independent audit:
`20260824T113551Z-contribution-auditor-d2r8.md`. Recommendation:
COMPLETE BLOCK, 3 findings, all independently verified against the
actual plan and CI text (not accepted at face value) and fixed:

- **`CA-NNP-R8-1`** (the CI-selection fix has no self-test proving the
  job is selected and blocks) — round 7 said the `agov:governance:
  validate` job's `changes:` scoping would be dropped, but neither
  "Files expected to change" nor "Test strategy" named any fixture or
  script that actually exercises the *job-selection* contract — only
  the Rust command's own behavior was tested. Verified: `CA-NNP-R7-1`'s
  own resolution condition explicitly required "a CI fixture whose only
  meaningful diff is creation of an agf/rules/** owner while its
  unchanged stage: planned manifest becomes stale, proving that the job
  is created and blocks," which the round-7 correction never added.
  Fixed: added a new `agov-validators::ci_rules` GitLab CI `rules:`
  evaluator (dependency-free, evaluates the job's real `rules:` array
  against a simulated pipeline context) as a named implementation file,
  a new Test-strategy bullet proving the job is selected across four
  pipeline contexts for exactly the owner-only stale-manifest shape
  `CA-NNP-R7-1` described, and a third Gate 2 acceptance-evidence item
  requiring both the selection proof and the created job's own blocking
  behavior as visible MR-description transcripts.
- **`CA-NNP-R8-2`** (`--repo-root` is absent from the implementation
  manifest and its Git semantics are under-specified) — "Files expected
  to change" instructed the implementer to add
  `PlanCommand::ProjectionCheckAll { base_ref: String }`, omitting the
  `repo_root` field the round-7 design already depends on throughout
  "Gate 2 acceptance evidence." Verified directly against the plan's
  own expected-file enumeration. Also verified a second, deeper gap:
  the design never specified that `--base-ref` resolution and all `git`
  operations (not merely manifest *discovery*) must run against the
  supplied `repo_root`, leaving open the possibility that `HEAD` still
  resolves against the caller's own checkout. Fixed: named the exact
  `repo_root: Option<PathBuf>` field; specified that manifest discovery,
  path resolution, all `git -C <repo_root>` operations, and `--base-ref`
  resolution all use the supplied root, with `None` falling back
  byte-identically to today's behavior; added a new integration-test
  file requirement and a Test-strategy bullet proving the command uses
  the fixture's own Git identity when invoked from outside it.
- **`CA-NNP-R8-3`** (operative opt-in rationale still denies the newly
  classified default-command break) — "Why an opt-in flag, not a new
  default" still said "No existing CI invocation's behavior changes
  unless someone deliberately opts a specific job into the relaxed
  mode" — false under this plan's own corrected contract, since every
  no-flag invocation gains the unconditional staleness *tightening*
  regardless of any opt-in. Verified directly; this is the same
  `CA-NNP-R7-4` defect class recurring at a fifth location round 7's
  four-location fix missed. Fixed: narrowed the claim to say opt-in
  governs only the *relaxation* direction, never the tightening
  direction, and re-scanned the rest of the plan for the same pattern
  (found none remaining).

## Round 9 correction (fixes the COMPLETE isolated audit, `skek`)

A fresh isolated Codex session performed a ninth independent audit:
`20260824T122318Z-contribution-auditor-skek.md`. Recommendation:
COMPLETE BLOCK, 2 findings, both independently verified against the
actual codebase and plan text (not accepted at face value) and fixed:

- **`CA-NNP-R9-1`** (the corrected `--repo-root` boundary is still
  false and incomplete for schema and distribution inputs) — round 8
  claimed "schema resolution stays embedded/compiled into the binary
  (already location-independent; not affected by `repo_root`)".
  Verified directly against the actual source: `projection_manifest.rs`
  calls `agov_core::schema_dir().join("rule-projection-manifest.schema
  .json")` and `schema::load_and_compile`, which reads the schema from
  the runtime filesystem — not an embedded string. `schema_dir()`
  honors `AGOV_SCHEMA_DIR`, falling back to `repo_root().join("agf")
  .join("schemas")` when unset; `repo_root()` itself is a pure CWD
  ancestor-walk today, with no env-var override at all. Separately,
  distribution-input isolation was never specified at all:
  `dist::validate_default()` calls `agov_core::repo_root()` directly.
  Fixed: added `AGOV_REPO_ROOT` env-var support to `repo_root()`,
  mirroring the exact existing `AGOV_SCHEMA_DIR` pattern; when
  `--repo-root <path>` is supplied, the command sets `AGOV_REPO_ROOT`
  and (unless already externally set) `AGOV_SCHEMA_DIR` for the
  process's remaining one-shot lifetime — a single mechanism that makes
  `repo_root()`, `schema_dir()`, and (transitively) `dist::
  validate_default()` all consistently resolve against the fixture
  root, replacing the false "already embedded" claim and the
  previously-undecided distribution boundary. Extended the required
  integration test with a conflicting-inputs case proving the fixture's
  schema and distribution manifest win over the caller checkout's own.
- **`CA-NNP-R9-2`** (the CI-rule self-test module is not registered in
  the expected Rust implementation scope) — "Files expected to change"
  named only `ci_rules.rs` as a new file, never
  `agov-validators/src/lib.rs`. Verified directly: this crate declares
  every module explicitly via `pub mod ...` in `lib.rs`; without adding
  `pub mod ci_rules;`, the new evaluator and its tests are not compiled
  or run by `cargo test -p agov-validators` at all — exactly the
  failure class this self-test exists to prevent. Fixed: added
  `agov-validators/src/lib.rs` to "Files expected to change" with the
  exact module declaration, and required the Gate 2 MR description to
  name the actual `cargo test -p agov-validators` output line(s)
  showing the new tests enumerated and passing, not merely that the
  file exists in the diff.

## Round 10 correction (fixes the COMPLETE isolated audit, `8hg1`)

A fresh isolated Codex session performed a tenth independent audit:
`20260824T131241Z-contribution-auditor-8hg1.md`. Recommendation:
COMPLETE BLOCK, 2 findings, both independently verified against the
actual codebase (not accepted at face value) and fixed — by replacing
the round-9 environment-variable mechanism entirely, not patching it:

- **`CA-NNP-R10-1`** (preserving the caller's schema override defeats
  the promised complete fixture boundary) — round 9's design set
  `AGOV_SCHEMA_DIR=<fixture>/agf/schemas` only when not already
  externally set. Verified directly against `tooling/agov`: the
  supported wrapper unconditionally runs
  `export AGOV_SCHEMA_DIR="${AGOV_SCHEMA_DIR:-$repo_root/agf/schemas}"`
  *before* invoking the Rust binary — so by the time the
  `ProjectionCheckAll` handler runs, `AGOV_SCHEMA_DIR` is *always*
  already set (to the caller's own checkout), and the round-9
  "only-if-unset" check never fires. The promised fixture-schema
  boundary was false on exactly the supported operator invocation path.
- **`CA-NNP-R10-2`** (the new global `AGOV_REPO_ROOT` contract is
  undocumented and under-scoped) — round 9 added `AGOV_REPO_ROOT` to
  the *shared* `agov_core::repo_root()` function, which is consumed by
  distribution validate/build/tag, umbrella and adopter validators,
  version/provenance paths, and reporting — not only this one command.
  Verified this makes the new variable a public, cross-command
  contract, which "Files expected to change" never disclosed and never
  added to the canonical `agf/docs/site/reference/environment-
  variables.md`, and which `downstream-impact-rule.md` requires be
  classified.

Both findings trace to the same root design mistake: using process
environment variables at all to carry the fixture root into functions
that could instead take it as an explicit parameter. **Fixed by
replacing the entire mechanism**, per this round's own re-audit of the
actual function signatures already in the codebase: `validate_file`,
`validate_paths_resolve`, and `check_surface_paths` in
`projection_manifest.rs` **already** take `repo_root: &Path` as an
explicit parameter — no change needed there at all. The one
non-parameterized call was `validate_file`'s internal
`agov_core::schema_dir()` lookup for the schema path — changed to an
explicit `schema_path: &Path` parameter instead. For distribution
validation, `dist::validate(manifest_path: &Path, root: &Path)`
**already** exists as an explicit-root function beneath the
convenience wrapper `dist::validate_default()` this plan had been
calling — switched to calling `validate` directly with the fixture
root when `--repo-root` is supplied. This closes both findings
simultaneously: no environment variable is read or set anywhere in the
`--repo-root` path, so there is structurally no possible conflict with
the wrapper's own `AGOV_SCHEMA_DIR` export (`CA-NNP-R10-1` is moot by
construction, not merely patched); and no shared function is modified,
so there is no new global contract to document, test, or
downstream-impact-classify (`CA-NNP-R10-2` is moot by construction).
See "Files expected to change" and "Test strategy" below for the
complete corrected design — the `AGOV_REPO_ROOT`/`AGOV_SCHEMA_DIR`
mechanism described in the "Round 9 correction" section above is
superseded, not merely amended.

## Round 11 correction (fixes the COMPLETE isolated audit, `mkps`)

A fresh isolated Codex session performed an eleventh independent audit:
`20260824T140020Z-contribution-auditor-mkps.md`. Recommendation:
COMPLETE BLOCK, 2 findings, both independently verified against the
actual current source and `.gitlab-ci.yml` (not accepted at face value)
and fixed:

- **`CA-NNP-R11-1`** (schema-root threading stops below the
  changed-manifest path) — verified directly against the actual current
  source: `validate_file_against(path, repo_root, changed_pages_or_
  runbooks, ships_in_distribution)` calls `validate_file(path,
  repo_root)` internally — today's 2-argument signature — to perform
  the diff-scoped changed-pages-coverage check. Round 10's fix added an
  explicit `schema_path` parameter to `validate_file` alone, leaving
  `validate_file_against` with no way to receive or forward a fixture
  schema path: a manifest present in the diff (the exact case this
  function exists for) would silently fall back to `agov_core::
  schema_dir()` inside the nested call, reading the caller checkout's
  schema — reproducing `CA-NNP-R10-1`'s wrapper-conflict shape one call
  frame deeper, undetected because the required test used `--base-ref
  HEAD` (an empty diff by construction) and never exercised this path.
  Fixed: `validate_file_against` also gains the explicit `schema_path:
  &Path` parameter and forwards it to its internal `validate_file`
  call; the `ProjectionCheckAll` handler passes the fixture schema path
  to both functions; and a new end-to-end test case builds a fixture
  with a real two-commit range and a non-empty `--base-ref`, so the
  coverage path is actually exercised with a conflicting caller
  `AGOV_SCHEMA_DIR` already set.
- **`CA-NNP-R11-2`** (the CI-selection self-test omits pipeline
  admission) — verified directly against this repository's actual
  `.gitlab-ci.yml`: the top-level `workflow: rules:` block suppresses a
  non-default-branch push pipeline (`when: never`) whenever
  `CI_OPEN_MERGE_REQUESTS` is truthy, since the MR-event pipeline is
  the canonical same-SHA path for that case — no pipeline means no job,
  regardless of the job's own `rules:`. The round-8 `ci_rules` design
  modeled only job-level `rules:`, never this workflow-level admission
  gate, so its claim that the job is "selected... in every simulated
  pipeline context, including a non-default branch push" was not an
  end-to-end proof for the common open-MR-push case, where no pipeline
  — and therefore no job — is ever created by design. Fixed: extended
  `ci_rules` into a two-stage evaluator (`pipeline_admitted` then
  `job_selected`), matching the resolution condition's own "simpler
  acceptable alternative" of narrowing and correcting the local model
  rather than replacing it with a live CI-lint dependency; replaced the
  ambiguous "non-default-branch push" context with its two real
  sub-cases (no open MR: admitted; open MR: correctly not admitted);
  and updated the self-test, Gate 2 evidence, and design bullets to
  assert admission is checked before job selection in every context.

## Round 12 correction (fixes the COMPLETE isolated audit, `krpk`)

A fresh isolated Codex session performed a twelfth independent audit:
`20260824T145228Z-contribution-auditor-krpk.md`. Recommendation:
COMPLETE BLOCK, 2 findings + 1 advisory, both blocking findings
independently verified against the actual current source before fixing:

- **`CA-NNP-R12-1`** (`stage: planned` can carry no owner path and
  vacuously pass Gate 2) — verified directly against the actual current
  schema: `surfaceEntry`'s `required` list is `["disposition",
  "reason"]` only; neither `path` nor `paths` is required, and `paths`
  has no `minItems`. The `normativeOwnerEntry` design (above) only adds
  the `stage` property on top via `allOf`, so `{disposition: updated,
  reason: ..., stage: planned}` — with no `path` and no `paths` at all
  — is schema-valid, and the planned-path relaxation, the unconditional
  staleness check, and the direct no-flag Gate 2 command all have
  nothing to act on: none of them reject it, so it passes without ever
  naming a concrete owner. Fixed: `normativeOwnerEntry`'s schema (below)
  now adds an `if`/`then` conditional — when `stage` is present and
  equals `"planned"`, at least one of `path` (non-empty string) or
  `paths` (non-empty array) is required. **Decided (both keys
  together):** an entry may declare both `path` and `paths`
  simultaneously — consistent with the whole-entry, all-members-
  together granularity already decided above for the staleness check —
  the conditional only requires *at least one* of the two to be
  non-empty, it does not forbid having both. Two new negative tests
  cover a `stage: planned` entry with neither `path` nor `paths`, and
  one with `paths: []`; both must fail schema validation and the direct
  no-flag Gate 2 command.
- **`CA-NNP-R12-2`** (`--repo-root` does not contain declared paths to
  that repository) — verified directly against the actual current
  source: `resolves(repo_root, relative)` is exactly
  `repo_root.join(relative).is_file()`, with no check that `relative`
  stays inside `repo_root`. In Rust, `Path::join` on an absolute
  `relative` argument discards `repo_root` entirely (documented
  standard-library behavior), and a `relative` containing `..`
  components can traverse above `repo_root`; `is_file()` also follows
  symlinks, so a symlink whose target lies outside `repo_root` still
  resolves. Round 10/11 correctly isolated the *schema* and
  *distribution-manifest* roots as explicit parameters, but this
  separate path resolver — used for every surface's `path`/`paths` and
  for the diff-scoped changed-pages/runbooks set — was never brought
  under the same fixture-containment guarantee the plan repeatedly
  claims (`--repo-root` "isolates... schema, and distribution identity,
  not the caller's checkout," lines 560, 610, 641, 1255, 1272, 1526,
  1543 of this revision): a fixture manifest can still name an absolute
  path or a `../`-escaping path into the *caller's own checkout* and
  have it resolve as "existing" under `--repo-root <fixture>`. Fixed —
  **repository-contained path resolution contract, decided:**
  `resolves()` now (a) rejects `relative` if it is absolute or contains
  any `ParentDir` (`..`) component, before ever joining it to
  `repo_root`; (b) after confirming `repo_root.join(relative).is_file()`,
  canonicalizes both the candidate and `repo_root` and requires the
  candidate's canonical form to start with `repo_root`'s canonical form
  — **symlink decision:** a symlink is followed for existence
  (unchanged), but its resolved target must still be contained in
  `repo_root`, or it is treated as non-resolving, not silently accepted.
  This is the general path-resolution primitive, not a `normative_owner`-
  only fix — it is applied identically to every surface's `path`/`paths`
  entries and to the changed-pages/runbooks coverage set, since all of
  them call the same `resolves()` function. New negative tests cover an
  absolute path, a `../`-escaping relative path, and a symlink whose
  target resolves outside `repo_root`, each under `--repo-root
  <fixture>` naming a real file that exists only in the *caller's*
  checkout. **Compatibility scope, extended:** this repository's own
  manifest corpus was scanned and uses only in-repository relative
  paths with no `..` components, so this is not a current-corpus
  compatibility problem here; adopter-owned manifests using an absolute
  or `..`-escaping path (previously accepted if it happened to resolve
  outside the repository) are addressed in the Downstream impact
  classification below as a third BREAKING case.

**Advisory `CA-NNP-R12-A1`** (the local CI evaluator still overstates
platform proof) — the resolution condition offers a choice: narrow the
claim, or add authoritative CI-lint/live-pipeline conformance evidence.
Taking the narrower, cheaper option: every claim in this plan and in
`ci_rules`'s own doc comments that the evaluator "proves GitLab actually
creates the job" is corrected to state it proves **structural,
local-model agreement with GitLab's documented `rules:`/`workflow:`
semantics for the specific contexts this plan's tests construct** — not
independent confirmation against the live platform. `ci_rules` does not
gain a CI Lint API dependency in this plan; a live-conformance check
remains a candidate follow-up, out of scope here. This does not reopen
`CA-NNP-R11-2`'s now-resolved missing-workflow-admission-stage defect.

## Round 13 correction (fixes the COMPLETE isolated audit, `smqm`)

Two fresh isolated Codex sessions independently reviewed the round-12
correction: `20260824T155539Z-contribution-auditor-bnt5.md` returned
CLEAR; `20260824T155602Z-contribution-auditor-smqm.md` returned
COMPLETE BLOCK, 2 findings + 1 advisory. Per this project's discipline
that a blocking finding from any independent reviewer session is
authoritative regardless of a concurrent CLEAR from a different
session, all three were verified against actual source and fixed:

- **`CA-NNP-R13-1`** (three operative sections still disclose only two
  of four BREAKING cases) — verified directly: round 12 correctly
  updated the central "Downstream impact classification" block to four
  breaking cases, but "Files expected to change" (the `CHANGELOG.md`
  bullet), "Compatibility scope, precisely," and "Rollback" were not
  updated and still said "both"/"two," naming only cases 1-2. An
  implementer following only those enumerated sections could omit the
  round-12 breaking changes from the release note despite the plan
  appearing complete. Fixed: all three sections now name all four
  cases explicitly; "Compatibility scope, precisely" additionally
  clarifies that case 4 (path containment) is not scoped to
  `normative_owner` alone, since `resolves()` is the shared primitive
  every surface and the changed-pages/runbooks coverage set already
  calls.
- **`CA-NNP-R13-2`** (the authoritative live MR description presents a
  removed design and false compatibility claims) — verified directly
  against the then-live GitLab MR description: it retained "Default
  (no-flag) validator behavior is unchanged everywhere" (false — every
  no-flag validation gains the staleness and containment tightenings),
  called the staleness check "strictly additive" (retracted as a
  mischaracterization back in round 7, `CA-NNP-R7-4`), and described CI
  wiring as correlating each changed manifest to its plan file via
  `plan_id`/`status: pending-gate1` (eliminated entirely in round 3
  because it deadlocked against the real manifest corpus). None of
  these three claims had been corrected across twelve subsequent
  rounds of exact-head updates. Fixed: the live MR description's
  Summary and Review Notes sections were rewritten to match the plan's
  actual current design — manifest-local CI, the direct no-flag
  `agov plan projection-check` command as mandatory Gate 2 evidence,
  and the accurate four-case BREAKING scope — then re-fetched from
  GitLab to confirm the sync.
- **Advisory `CA-NNP-R13-A1`** (round-12 local-model narrowing remains
  incomplete) — the Gate 2 evidence summary in "Test strategy" still
  said the job is "actually created" and "actually blocks" based only
  on the local evaluator's output plus a separate CLI fixture, and the
  evaluator's supported GitLab expression subset and unsupported-input
  fail-closed behavior were undefined. Fixed: narrowed the summary to
  distinguish local-model agreement (job selection) from real-binary
  evidence (the staleness check actually blocking, which is genuinely
  observed via the CLI fixture, not modeled); and added an explicit
  supported-subset contract (equality/inequality on named CI variables,
  `&&` combinations, truthy checks, and glob-matched `changes:` — the
  exact shapes this repository's own `.gitlab-ci.yml` uses) plus a
  fail-closed rule (an unsupported expression shape is a hard error,
  never a silent guess) and a dedicated self-verifying test that fails
  loudly if `.gitlab-ci.yml` ever adopts an expression outside that
  subset.

## Round 14 correction (fixes the COMPLETE isolated audit, `4t37`)

A fresh isolated Codex session performed a fourteenth independent audit:
`20260824T162316Z-contribution-auditor-4t37.md`. Recommendation:
COMPLETE BLOCK, 1 finding, independently verified against the actual
plan design before fixing.

- **`CA-NNP-R14-1`** (planned-path relaxation converts containment
  violations into accepted absence) — verified directly against this
  plan's own design text: `resolves()` is a single boolean, and the
  planned-path relaxation ("When `allow_planned` is `true` **and**
  `stage == Some("planned")`: skip the `resolves()` check for that
  entry's `path`") treats every `false` result identically, regardless
  of *why* it is false. Round 12 made `resolves()` return `false` for
  three different reasons — genuinely does not exist yet (the
  intended, legitimate `planned` case); is absolute; contains `..`; or
  is a symlink resolving outside `repo_root` — and the relaxed branch
  cannot tell them apart. A manifest with `normative_owner.stage:
  planned` and `path: ../../outside/rule.md` (an outside file that
  genuinely exists) makes `resolves()` return `false` for the
  containment reason, is indistinguishable from legitimate absence to
  the relaxed branch, and is therefore *accepted* by
  `--allow-planned`/`projection-check-all` — exactly the repository
  boundary this plan's round-12 fix was supposed to close, reopened by
  the interaction between that fix and the pre-existing lifecycle
  branch. The round-12 escape tests never put an escaping path on a
  `normative_owner.stage: planned` entry through the relaxed route
  specifically, so they never exercised this interaction.

  Fixed — **path classification separated from a single boolean,
  decided:** `resolves()`'s internal logic is restructured into a
  three-way classification, used by every caller that needs to
  distinguish these cases:

  ```rust
  enum PathClassification {
      ContainedExisting,  // repository-relative, no escape, file exists
      ContainedMissing,   // repository-relative, no escape; the deepest
                           // EXISTING component canonicalizes inside
                           // repo_root, and the first ABSENT component
                           // is genuinely absent (not an existing
                           // symlink, dangling or otherwise)
      Invalid,            // absolute, `..`-containing, or any existing
                           // component (file, dir, or symlink, at any
                           // depth) that fails to canonicalize inside
                           // repo_root -- including a dangling symlink,
                           // direct or chained, regardless of whether
                           // the final escaped-to target exists
  }

  fn classify(repo_root: &Path, relative: &str) -> PathClassification {
      let candidate = Path::new(relative);
      if candidate.is_absolute()
          || candidate.components().any(|c| matches!(c, std::path::Component::ParentDir))
      {
          return PathClassification::Invalid;
      }
      let Ok(canonical_root) = repo_root.canonicalize() else {
          return PathClassification::Invalid;
      };
      let joined = repo_root.join(candidate);

      if joined.is_file() {
          return match joined.canonicalize() {
              Ok(real) if real.starts_with(&canonical_root) => PathClassification::ContainedExisting,
              _ => PathClassification::Invalid,
          };
      }

      // Fail-closed final design (governor decision
      // 20260824T185045Z-governor-reviewer-a86h.md, replacing the prior
      // read_link-based fallback after it was shown not to close a
      // chained-relative-dangling-symlink case): walk every component
      // from repo_root down. Existence is checked with
      // `symlink_metadata`, which does NOT follow the link -- unlike
      // `Path::exists()`, it correctly reports a dangling symlink entry
      // as existing, so a chained dangling target can never be silently
      // skipped. Every existing component -- file, directory, or
      // symlink, direct or chained -- must canonicalize successfully
      // and stay inside `canonical_root`; if canonicalization fails for
      // any reason (a dangling target anywhere in a symlink chain, an
      // escaping resolution, or any other cause), the result is
      // `Invalid`. Only once an ABSENT component is reached (nothing
      // exists at that depth at all) does the walk stop and return
      // `ContainedMissing` -- this deliberately makes a repository-
      // internal dangling symlink `Invalid` too, not only an
      // outside-pointing one: a dangling symlink is never sufficient
      // evidence that a planned owner is safely repository-contained.
      let mut walked = repo_root.to_path_buf();
      for component in candidate.components() {
          walked.push(component);
          match std::fs::symlink_metadata(&walked) {
              Ok(_) => {}
              // Only a genuine "nothing exists here" stops the walk as
              // legitimately absent (final targeted proof
              // 20260824T192653Z-contribution-auditor-ktnr.md: any
              // other metadata error -- PermissionDenied, I/O error,
              // NotADirectory -- must NOT be treated as absence, or
              // containment is never actually established for that
              // component).
              Err(e) if e.kind() == std::io::ErrorKind::NotFound => break,
              Err(_) => return PathClassification::Invalid,
          }
          match walked.canonicalize() {
              Ok(real) if real.starts_with(&canonical_root) => continue,
              _ => return PathClassification::Invalid,
          }
      }
      PathClassification::ContainedMissing
  }
  ```

  This replaces the `read_link`/relative-target-resolution fallback
  entirely — no chained-symlink-following logic is needed, because any
  unresolvable canonicalization (direct dangling, chained dangling, or
  any other failure) is uniformly `Invalid`. **Accepted compatibility
  consequence, stated explicitly per the governor decision:** an
  entirely repository-internal dangling symlink (pointing nowhere, or
  to another symlink that ultimately dangles, but never leaving
  `repo_root`) is now also classified `Invalid` rather than
  `ContainedMissing`, even though it never actually escapes the
  repository. This is a deliberate, accepted false negative — a
  dangling symlink of any kind is not proof of a legitimately planned,
  not-yet-created path, and no current or historical manifest in this
  repository's corpus relies on a dangling symlink as a `stage: planned`
  path (verified: no current manifest sets `stage` on any row at all).

  `resolves(repo_root, relative)` becomes
  `classify(repo_root, relative) == PathClassification::ContainedExisting`
  — every existing caller that only needs a boolean (every surface
  except the `normative_owner.stage: planned` relaxed branch, and the
  changed-pages/runbooks coverage set) is byte-identical in behavior,
  since `Invalid` and `ContainedMissing` both still fail resolution for
  them, exactly as `false` did before. **Only the planned-path
  relaxation itself changes:** it now calls `classify()` directly and
  branches on all three states, not on `resolves()`'s collapsed
  boolean:
  - `ContainedMissing` **and** `allow_planned` **and** `stage ==
    Some("planned")`: relaxation applies, exactly as before — this is
    the sole legitimate case the relaxation exists for.
  - `ContainedExisting`: the existing staleness check applies exactly
    as today (fires when `stage == Some("planned")`, unconditional on
    the flag).
  - `Invalid`: **unconditionally a blocking error, regardless of
    `stage`, `allow_planned`, or any other setting** — a repository
    escape is never planned-path-relaxable, never staleness-checked
    (there is nothing to be "stale" about), and never silently passed.
    The error names the entry and states the path escapes the
    repository root — a distinct message from both the ordinary
    "does not resolve" failure and the staleness message, since the
    resolution condition this failure represents is different from
    either (it is not "not yet built" and not "already exists," it is
    "not a valid repository-relative reference at all").

  This is not scoped to `normative_owner` alone in its underlying
  primitive — `classify()` is the shared function every surface and
  the changed-pages/runbooks coverage set already call through
  `resolves()`'s narrowed definition above — but the *three-way branch*
  matters operationally only where a boolean `false` was ever treated
  as "acceptable, handle differently" rather than uniformly rejected,
  which is exactly the `normative_owner.stage: planned` relaxed route
  this finding is about.

  Three new negative tests, each putting the escape shape on a
  `normative_owner.stage: planned` entry and exercising it through
  *both* `--allow-planned`/`projection-check-all` (the exact route the
  finding names) and the ordinary no-flag path: an absolute `path`, a
  `../`-escaping `path`, and an in-repository symlink whose target
  resolves outside `repo_root` — all three must fail with the new
  `Invalid`-classification error in every context, never silently pass
  under `--allow-planned`.

**Concurrent parallel review, reconciled here rather than as a separate
round:** a second, independently dispatched Codex session
(`20260824T163022Z-contribution-auditor-pdrq.md`, session
`codex-subagent-mr1847-round14-parallel-fresh-audit-20260824`) reviewed
the same round-13 exact head concurrently with `4t37`, without reading
`4t37`'s in-progress record. Per the review-convergence directive
(`20260824T163113Z-governor-reviewer-36rr.md`), round 14 (`4t37`) is
this plan's final full clean-room review — `pdrq` is not a second
authorized round, and this plan does not treat it as one. It
independently confirmed `CA-NNP-R14-1` (its own numbering:
`CA-NNP-R14P-1`, same underlying defect, same fix above) and surfaced
two additional findings, both independently re-verified against actual
current source before being folded into this same correction rather
than deferred to an unauthorized round 15:

- **`CA-NNP-R14P-2`** (the round-13 CI evaluator subset rejects
  expressions the live workflow actually uses) — verified directly
  against the current `.gitlab-ci.yml`: its `workflow:` block's
  suppression rule uses `$CI_COMMIT_BRANCH != $CI_DEFAULT_BRANCH` and
  its cache-writer rule uses `$CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH`
  — both variable-to-variable comparisons, neither expressible under
  round 13's variable-to-string-literal-only subset. The promised
  self-verifying live-file test would have failed immediately against
  real content. Fixed above ("Supported expression subset and
  fail-closed contract"): the subset now includes variable-to-variable
  equality/inequality, and ordered first-match `rules:` semantics
  (including `when: never`/`when: always` precedence, which the live
  workflow's correctness depends on) are now explicitly specified,
  closing the gap the finding names.
- **`CA-NNP-R14P-3`** (consultation provenance stale and cites a
  nonexistent filename) — verified directly: the front matter's
  `CURRENT STATE (round 11)` label was never advanced across rounds
  12-14 (the same recurring metadata-staleness class round 5 fixed
  once already, `CA-NNP-R5-1`), and both the front matter and this
  section cited `20260824T155125Z-contribution-auditor-bnt5.md` — the
  actual committed filename is `20260824T155539Z-contribution-auditor-
  bnt5.md` (verified via `git ls-tree`; the wrong timestamp appears to
  have been copied from an unrelated concurrent review's ID during
  round 13's drafting). Fixed: the front-matter label now reads
  `CURRENT STATE (round 14)`, both `bnt5` citations are corrected, and
  handoff `20260824T161932Z-planner-architect-grrk.md`'s identical
  broken reference is superseded (not rewritten — that record is
  immutable) by a dedicated coordination note.

Advisory `CA-NNP-R14P-A1` (exact-head `agov validate` transcript
completeness) is not acted on in this plan revision: the reviewer's own
finding text concludes the referenced `agov.validate.agents` errors are
pre-existing on old candidate history and already corrected on current
`main`, not attributable to this plan. Per the convergence directive's
point 5, non-material advisory findings go to backlog unless the
governor explicitly promotes them — noted here for visibility, not
fixed.

**Targeted final verification, residual gap fixed (still within the
same authorized scope, not a new round):** governor ratification
`20260824T175134Z-governor-reviewer-q28m.md` authorized one targeted
final verification of `CA-NNP-R14-1`, `CA-NNP-R14P-2`, and
`CA-NNP-R14P-3` at this plan's then-exact head. That verification
(`20260824T175442Z-contribution-auditor-gpkh.md`) confirmed
`CA-NNP-R14P-2` and `CA-NNP-R14P-3` resolved, but found `CA-NNP-R14-1`
still open: `classify()`'s `ContainedMissing` branch returned as soon
as the leaf failed `is_file()`, without checking whether any ancestor
component — a symlinked directory, or the leaf itself as a dangling
symlink — resolves outside the repository. Concrete failure: `repo/link
-> /outside`, planned path `link/future-rule.md` — `is_file()` is
false (nothing exists yet at that path), so the prior code returned
`ContainedMissing` and `--allow-planned` accepted it, even though the
symlinked ancestor means the file, if ever created, would land outside
the repository. Verified directly against the plan's own `classify()`
code before fixing. Fixed above: before returning `ContainedMissing`,
`classify()` now walks every path component from `repo_root` down,
checking each with `symlink_metadata` (which does not follow the
link), and rejects as `Invalid` if any component is a symlink whose
resolved location (or, for a dangling symlink, whose resolved parent)
does not stay inside the canonical repository root. Two new negative
tests specified below (missing leaf below an outside-pointing
directory symlink; a dangling outside-pointing symlink), both through
`--allow-planned`, `projection-check-all`, and the strict route, per
the resolution condition.

**Second targeted check (`20260824T182750Z-contribution-auditor-
5tms.md`) found the dangling-symlink half still wrong, fixed per
governor authorization `20260824T183138Z-governor-reviewer-4grm.md`:**
the fallback above canonicalized the *symlink entry's own* parent
(e.g. `repo/`) when the target was missing, not the *target's* parent
(e.g. `/outside/`) — so `repo/dangling -> /outside/missing-rule.md`
was still wrongly accepted as `ContainedMissing`. Verified directly
against the plan's own code before fixing. Fixed: the fallback now
reads the link's actual target via `read_link`, resolves it against
the link entry's parent if relative, walks up from that target to its
own deepest existing ancestor, and canonicalizes *that* — closing the
exact gap `5tms` named.

**Third targeted check (`20260824T184757Z-contribution-auditor-
7km1.md`) found a chained-relative-dangling-symlink bypass** (`repo/a
-> b`, `repo/b -> /outside/missing-rule.md`: followed `Path::exists()`
skipped the existing-but-dangling `b` entry). Rather than patch the
`read_link` fallback a third time, the governor directed a simpler
fail-closed replacement
(`20260824T185045Z-governor-reviewer-a86h.md`): every existing
component must canonicalize successfully inside `repo_root`, checked
via `symlink_metadata` (not followed `exists()`) so a dangling entry at
any depth is never silently skipped; any canonicalization failure —
direct or chained dangling, or any other cause — is `Invalid`. This
removes the `read_link`/target-resolution logic entirely and accepts,
explicitly, that a purely repository-internal dangling symlink is now
also `Invalid` rather than `ContainedMissing` (see the code block
above for the accepted-consequence statement). Fixed at the code level
above; test case updated to the governor's exact four-case acceptance
list below.

## The conflict this plan resolves

`documentation-rule-projection-rule.md` requires the canonical
`docs/reviews/rule-projection/<plan-id>.yaml` manifest to be filed and
pass `agov plan projection-check` for any Gate 1 plan that triggers the
rule. That validator (`agov-coord/src/projection_manifest.rs`,
`check_surface_paths` / `validate_paths_resolve`) unconditionally
requires every declared `path`/`paths` entry on every surface to already
resolve to a real repository file.

For a plan proposing a **net-new** normative-owner file (a rule, schema,
or other artifact that does not exist yet), those two requirements
cannot both hold: the file cannot exist before Gate 1 approval (per
`AGENTS.md`'s Gate discipline — see `CT-UWD-1`,
`20260823T152413Z-coder-tester-5216.md`, from the MR !1817 review chain
that first exposed this), but the validator rejects any manifest
declaring a path that does not yet exist. MR !1817 hit this directly and
received a one-time waiver rather than a general fix, precisely because
no general fix existed at the time.

## Design

### Mechanism: an explicit, opt-in `stage: planned` marker (revised round 2, fixes `CA-NNP-R2-5`)

Add an optional `stage` field to the schema's dedicated
`normativeOwnerEntry` definition **only**
(`agf/schemas/rule-projection-manifest.schema.json` —
`surfaces.normative_owner` alone; see "Schema change" below, decided in
round 1 for `CA-NNP-R1` and restated here since the paragraph above
this one, in an earlier draft, described the pre-round-1 shared-
`surfaceEntry` design and is now corrected), with two allowed values:

- `existing` (or the field omitted — these are equivalent): the
  declared path must already resolve to a real file. This is the
  validator's current, unconditional behavior for every manifest that
  exists today.
- `planned`: the declared `normative_owner` path is the file this plan
  proposes to create, and does not exist yet. `stage` has no defined
  meaning and no effect on any other row — the validator never reads
  it outside the `normative_owner` surface (round 5, fixes part of
  `CA-NNP-R5-A1`: corrected from "schema or parsed type," which
  implied a typed-struct boundary the validator does not have — it
  operates on raw `serde_json::Value`, not per-row typed Rust structs).

**The single-manifest no-flag command's path-resolution logic is
unchanged; it gains the new staleness check regardless of flag.**
`agov plan projection-check --base-ref <ref> <file>` (no new flag)
continues to require every declared path to resolve, exactly as
today. It additionally gains one new check, applying with or without
`--allow-planned` (round 7, fixes part of `CA-NNP-R7-4`: corrected from
an earlier round-7 draft of this passage claiming `stage: planned` "has
no effect" in no-flag mode — that contradicted this same plan's own
"Compatibility scope" section below, which has said since round 1 that
the default invocation "does gain one new check that runs regardless of
the flag"): the unconditional staleness check (fixes `CA-NNP-R3`) —
`normative_owner.stage: planned` paired with a path that already
resolves is rejected as "lifecycle transition incomplete," on this
single-manifest command exactly as on `projection-check-all`. A
manifest whose `stage` is absent, `existing`, or that has no resolving
path yet is unaffected by this new check; only the specific stale-
marker shape newly fails, on every command that validates it.

**This is a BREAKING tightening for that one shape, not a "zero
migration, zero risk" change** (round 7, fixes part of `CA-NNP-R7-4`:
corrected from "keeps exactly today's strict behavior with zero
migration and zero risk" — false for exactly the second BREAKING case
in "Downstream impact classification" below: an adopter manifest that
already sets `stage: planned` on a path that already resolves
currently passes both CI's blanket `plan projection-check` loop today
and a direct by-hand invocation of the single-manifest command today;
after this plan, it newly fails both). See "Compatibility scope,
precisely" and "Downstream impact classification" below for the
complete, accurate picture.

A new opt-in flag, `agov plan projection-check --allow-planned --base-ref
<ref> <file>`, additionally skips the resolution check specifically for
`normative_owner` when it is marked `stage: planned` — whole-entry
granularity, decided (not open) per "Design decisions" below. This
single-manifest command has **two** distinct, deliberate uses (round 4,
fixes `CA-NNP-R4-1`, `CA-NNP-R4-A1`): with `--allow-planned`, it is the
direct invocation a human reviewer uses during Gate 1 review; **without**
the flag, it is the required Gate 2 acceptance-evidence command — path
resolution strict by default as it is today, plus the new staleness
check (round 7, fixes part of `CA-NNP-R7-4`: dropped "exactly as it is
today" as an unqualified claim about this command's full behavior — see
the corrected passage above) — run against the specific manifest being
handed off, proving the owner now actually resolves and that a
well-behaved implementer has correctly updated `stage` away from
`planned`. The
separate, tree-wide `agov plan projection-check-all` subcommand
described in "Design decisions" is the routine CI safety net that
relaxes every manifest's `normative_owner.stage: planned` entries
unconditionally (no correlation of any kind — see "Design decisions" for
why) and independently catches staleness once an owner appears, on any
branch; it is not itself sufficient Gate 2 evidence for any one plan's
own owner (see `CA-NNP-R4-1`'s resolution below).

### Why an opt-in flag, not a new default

Per the issue's acceptance constraints ("existing manifests and
existing-path validation remain compatible," "mechanical checks remain
limited to authority explicitly granted") and this project's general
posture toward validator changes (`always/downstream-impact-rule.md`):
a change that could silently make an existing strict check more
permissive by default is exactly the kind of regression a validator's
own users would not expect. Requiring an explicit `--allow-planned`
flag means:

- No existing CI invocation gains the **relaxation** (tolerating an
  absent-but-`planned` owner path) unless someone deliberately opts a
  specific job into `--allow-planned` or the new tree-wide command
  (round 8, fixes `CA-NNP-R8-3`: corrected from "no existing CI
  invocation's behavior changes... " full stop — false under this
  plan's own corrected contract, since every existing no-flag
  invocation *does* gain the unconditional staleness **tightening**
  described immediately above and below, whether or not anyone opts
  into anything. Opt-in governs only the relaxation direction, never
  the tightening direction).
- The relaxation is visible in the CI configuration diff wherever it is
  adopted, not hidden inside schema semantics alone.
- A manifest carrying `stage: planned` that is validated *without* the
  flag (e.g., because a reviewer forgot to add it, or because it is
  being checked at Gate 2 time) fails exactly as it would today —
  satisfying the issue's acceptance constraint that "an unresolved
  planned owner is rejected" before Gate 2, without needing any
  Gate-2-specific code path at all. The default-strict behavior *is*
  the Gate 2 check.

### Rule text changes (`agf/rules/workflow/documentation-rule-projection-rule.md`)

Add a new subsection (exact placement: after "The mandatory projection
screen," before "What mechanical validation MAY block on") documenting:

1. When `stage: planned` is appropriate: only for a surface whose file
   is proposed by the *same* Gate 1 plan and does not exist because
   creating it before Gate 1 approval would itself violate Gate
   discipline.
2. The two-invocation lifecycle, made explicit and mandatory (round 4,
   fixes `CA-NNP-R4-1`): `agov plan projection-check <manifest>
   --allow-planned` at Gate 1 review (the direct, single-manifest
   command, relaxed); **before requesting Gate 2, the implementer MUST
   run the same command *without* `--allow-planned`** against the exact
   manifest being handed off, and include its passing output as Gate 2
   acceptance evidence per `gate-self-test-rule.md` — this is the actual
   Gate 2 gate, separate from and in addition to whatever the routine,
   tree-wide `agov plan projection-check-all` CI job reports (that job
   is a safety net for staleness anywhere in the tree; it is never
   itself sufficient evidence that any one plan's own owner now
   exists).
3. That `stage: planned` is advisory metadata about the manifest
   author's *intent*, not a mechanical exemption in the default
   invocation — the default invocation is the actual Gate 2 gate, run
   directly against the specific manifest per point 2, not inferred
   from any tree-wide or automated signal.
4. A worked example, following the existing frozen-scope-reconstruction
   and MR !1817 precedent-record style, once this plan's own
   implementation lands (the worked example can only be written after
   MR !1817's manifest is filed, so it references that manifest as the
   concrete case).

### Schema change (`agf/schemas/rule-projection-manifest.schema.json`) — scoped to `normative_owner` only (fixes `CA-NNP-R1`; conditional path requirement added round 12, fixes `CA-NNP-R12-1`)

Add a new, dedicated definition rather than extending the shared
`surfaceEntry` used by all six rows:

```json
"normativeOwnerEntry": {
  "allOf": [
    { "$ref": "#/definitions/surfaceEntry" },
    {
      "type": "object",
      "properties": {
        "stage": {
          "type": "string",
          "enum": ["existing", "planned"]
        }
      },
      "if": {
        "properties": { "stage": { "const": "planned" } },
        "required": ["stage"]
      },
      "then": {
        "anyOf": [
          {
            "required": ["path"],
            "properties": { "path": { "type": "string", "minLength": 1 } }
          },
          {
            "required": ["paths"],
            "properties": {
              "paths": {
                "type": "array",
                "items": { "type": "string", "minLength": 1 },
                "minItems": 1
              }
            }
          }
        ]
      }
    }
  ]
}
```

`stage: planned` now requires at least one non-empty `path` or a
`paths` array with at least one non-empty member (round 12, fixes
`CA-NNP-R12-1`) — an entry may declare both together; the conditional
only requires that at least one resolve to something, not that only one
of the two keys be present. `stage: existing` (or `stage` absent) keeps
today's fully optional `path`/`paths`, matching every other row's
`surfaceEntry` behavior (a `not_applicable`-only disposition with no
path stays legal for `normative_owner` when it is not marked
`planned`).

Change `surfaces.properties.normative_owner`'s `$ref` from
`#/definitions/surfaceEntry` to `#/definitions/normativeOwnerEntry`.
Every other row (`adopter_facing_explanation`, `runbook_situation_map`,
`ai_landing_loaders`, `validation_drift_evidence`, `distribution`) keeps
`$ref: #/definitions/surfaceEntry`, unmodified — `stage` has no defined
meaning outside `normative_owner` and is not added to the shared
definition. No `required` list changes beyond the conditional above —
`stage` stays optional on `normative_owner`, preserving compatibility
with every manifest that exists today (none set it; see "Adopter docs
parity" and the compatibility note below for the narrowed
collision-risk analysis this scoping produces).

### Validator change (`tooling/rust/crates/agov-coord/src/projection_manifest.rs`, `tooling/rust/crates/agov/src/main.rs`) — scoped to `normative_owner` only, plus the new staleness invariant (fixes `CA-NNP-R1`, `CA-NNP-R3`)

- `PlanCommand::ProjectionCheck` (main.rs, ~line 286): add
  `#[arg(long)] allow_planned: bool` (default `false`).
- Thread `allow_planned` through `plan_projection_check` (~line 5475)
  into `agov_coord::projection_manifest::validate_file_against` (a new
  parameter, or a small config struct if the function's parameter count
  is already at the project's stylistic limit — implementer's call).
- In `check_surface_paths` (projection_manifest.rs ~line 198), the
  planned-path relaxation applies **only when the surface being checked
  is `normative_owner`** — an explicit surface-name check (round 4,
  fixes `CA-NNP-R4-A1`: the validator parses raw `serde_json::Value`,
  not a distinct typed struct per row, so the guard is this explicit
  check, not an incidental type-system boundary). For every other
  surface, behavior is unconditionally identical to today — any
  `stage` key present is simply never read. For `normative_owner`
  specifically:
  - When `allow_planned` is `true` **and** `stage == Some("planned")`:
    skip the `resolves()` check for that entry's `path`.
  - When `allow_planned` is `false` (the default): the *resolution
    check itself* is byte-identical to today regardless of `stage` — the
    flag alone gates the relaxation. (The new staleness check below is
    a separate, additional check layered on top, not part of the
    resolution check — see that bullet for exactly when it fires.)
  - **New, unconditional on the flag** (fixes `CA-NNP-R3`): if
    `stage == Some("planned")` **and** the path *does* resolve, emit a
    blocking error ("normative_owner.stage is `planned` but
    `<path>` already exists — update to `stage: existing` or remove the
    field before Gate 2"). This check runs whether or not
    `--allow-planned` is passed, so a manifest cannot carry a stale
    `planned` marker into Gate 2 undetected — the earlier design's claim
    that this case was "harmless" is retracted; see the revised test
    strategy below.
  - **`paths` array semantics, decided (round 4, fixes
    `CA-NNP-R4-3`):** whole-entry granularity (decided in "Design
    decisions" below) means `stage: planned` covers `path` and every
    element of `paths` together as one unit — but the staleness check
    above fires the moment **any single member** resolves, not only
    when every member does. Rationale: a `planned` marker asserts none
    of the entry's declared paths exist yet; the instant any one of
    them resolves, that assertion is already false for that member,
    regardless of the others' state. This is the conservative,
    fail-closed reading, consistent with "uncertainty means full Gate
    1" — partial drift is caught immediately, not deferred until every
    member happens to resolve.

### Operator documentation and adopter docs parity (fixes `CA-NNP-R4`)

No `agf/docs/site/` page currently documents the projection-manifest
process **at all** (verified: `grep -rl "rule-projection"
agf/docs/site/` returns nothing, and `grep -n "Gate 1\|projection"
agf/docs/site/docs/operate/review-and-release.md` shows no mention of
the manifest process either) — this is a pre-existing gap in the whole
projection-manifest concept, not something this plan introduces. Adding
adopter-facing Pages documentation for the *whole* projection-manifest
process (not just this lifecycle addition) is out of scope for this
plan and would be its own separate plan if ever needed — narrowing an
already out-of-scope general gap is not the same as ignoring this
plan's own adopter-docs-parity obligation, which is addressed directly
below.

Per `downstream-impact-rule.md`'s "Adopter docs parity" section (this
plan changes a distributed validator command, tooling flag, and
schema — all named trigger categories), the Gate 2 handoff for this
plan's implementation MUST include one of the three allowed
dispositions. This plan's disposition:

**`Adopter docs parity: updated`** — citing
`agf/rules/workflow/documentation-rule-projection-rule.md`'s new
subsection (see "Rule text changes" above) as the adopter-facing
explanation. That file is itself adopter-shipped (`agf/rules/**`
distribution glob) and is the canonical normative owner for the whole
projection-manifest concept — not a bare reference page, but the actual
rule text an adopter reads to understand when and how to use
`stage: planned` / `--allow-planned`, including the two-invocation
lifecycle and a worked example. This satisfies "reference-only
documentation is not enough by itself": the addition is a governance
rule explaining the *process*, not merely a CLI `--help` string or
schema comment. No separate `agf/docs/site/` Pages page is added,
consistent with the pre-existing gap noted above — if a future plan
creates the general projection-manifest Pages page, this lifecycle
addition should be folded into it at that time, not duplicated now.

`agov plan projection-check --help` output updates automatically from
the new `clap` derive doc comment on `--allow-planned`, as a secondary,
reference-level aid — not the disposition's primary evidence.

## Design decisions (were open questions; decided in this revision, fixes `CA-NNP-R2`)

1. **Granularity of `stage: planned`: whole `surfaceEntry`.** Decided,
   not merely recommended. `stage` lives on `normative_owner` as a
   whole — it exempts both `path` and every entry in `paths` together;
   there is no per-individual-path granularity. Rationale: simpler
   schema, simpler validator logic, and MR !1817's own actual manifest
   (`docs/reviews/rule-projection/20260823T151423Z-unassigned-work-
   discovery-rule.yaml`, filed during this same review cycle) needs only
   whole-entry granularity — its `normative_owner` surface is entirely
   net-new. Per-path granularity can be added later without a breaking
   schema change if a real case needs it (`stage` would move from the
   surface entry into each `paths` array element) — not designed for
   that hypothetical now, per `simplicity-rule.md`.
2. **CI wiring: decided, revised round 3 (fixes `CA-NNP-R2-1`,
   `CA-NNP-R2-2`, `CA-NNP-R2-6`, `CA-NNP-R3-1`, `CA-NNP-R3-2`,
   `CA-NNP-R3-3`).** The round-1 design pushed plan correlation into a
   bash+python CI loop (broken filename construction, diff-scoped
   blindness, an unprovisioned Python dependency). Round 2 replaced it
   with an `agov`-native subcommand that correlated each manifest to its
   linked plan's `status` field — but round 3's audit found that
   mechanism itself broken against the real manifest corpus
   (`CA-NNP-R3-1`: 2 of 5 existing manifests' `plan_id` doesn't
   exact-match their plan's front-matter `id`; a 3rd has no matching
   plan file at all) and structurally deadlocked (`CA-NNP-R3-2`: an
   `approved` plan whose Gate 2 implementation hasn't landed yet would
   fail every pipeline in the repository, not just its own branch),
   plus a separate coverage-check conflict when applied tree-wide
   (`CA-NNP-R3-3`).

   **Round 3 removes plan correlation entirely, rather than repairing
   it.** `CA-NNP-R3-1` and `CA-NNP-R3-2` both trace to the same root
   mistake: gating the relaxation on an *external* plan file's status.
   The relaxation only ever needs to be a function of the manifest's
   *own* `normative_owner.stage` field — exactly what the single-
   manifest `--allow-planned` flag already does — with the
   newly-specified **unconditional** staleness check (fixes
   `CA-NNP-R3`, round 1; round 5, fixes part of `CA-NNP-R5-A1` —
   corrected from "existing, already-implemented": this plan is
   Gate-1 plan-only and the check does not exist in the shipped
   validator yet) remaining the sole guard against indefinite
   abuse: the moment an owner path actually exists, a manifest still
   marked `stage: planned` for it fails immediately and permanently,
   independent of any plan's status. No plan file is ever read, no
   `plan_id` lookup happens, and `CA-NNP-R2-1`'s broken filename
   construction is moot by elimination rather than a more careful
   re-implementation.

   The new `agov plan projection-check-all --base-ref <ref>
   [--repo-root <path>]` subcommand (no manifest argument — it
   discovers its own inputs under the repo root) therefore does two
   things, kept deliberately separate to fix `CA-NNP-R3-3`:

   `--repo-root` (round 7, fixes part of `CA-NNP-R7-3`; optional,
   defaults to the discovered current repository) mirrors the existing
   `commercial-corpus validate --repo-root` precedent already shipped in
   `.gitlab-ci.yml`. It lets the command's full-tree discovery walk any
   directory tree, not only the real checkout — the mechanism that makes
   the CLI-level negative gate-self-test demonstration reproducible
   without touching the real canonical manifest corpus (see the
   redesigned Gate 2 acceptance evidence below).

   1. **Full-tree path-resolution and staleness check (fixes
      `CA-NNP-R2-2`) — every manifest, every run, not diff-scoped.**
      Enumerate every `docs/reviews/rule-projection/*.yaml` file
      currently in the repository tree. For each: resolve every
      declared path, relaxing only `normative_owner` entries marked
      `stage: planned` (unconditionally — no plan lookup); apply the
      existing unconditional staleness check regardless of relaxation.
      This step alone is what guarantees a manifest whose owner path
      has newly started resolving is always caught, on any pipeline,
      regardless of what that particular branch's diff touches.
   2. **Changed-pages coverage check (fixes `CA-NNP-R3-3`) — diff-scoped
      only, unchanged from today's existing behavior.** For manifests
      that are themselves part of the current diff (`git diff
      --name-only -z "$base"...HEAD -- 'docs/reviews/rule-
      projection/*.yaml'`), additionally run the existing
      `validate_file_against` changed-pages-coverage check exactly as
      it works today. Manifests *not* part of the diff never run this
      check — there is nothing for an unrelated, unchanged manifest to
      newly declare, and running it against them is what `CA-NNP-R3-3`
      showed breaks ordinary single-page changes.

   The subcommand's own exit code is non-zero if **any** manifest fails
   either check, naming every failing manifest, which check failed, and
   why, in its output.

   `.gitlab-ci.yml`'s `agov:governance:validate` job replaces its
   current blanket `xargs -0r "$agov_bin" plan projection-check
   --base-ref "$base"` invocation (over the diff's changed manifest
   files) with a single call:

   ```bash
   "$agov_bin" plan projection-check-all --base-ref "$base"
   ```

   **The job's own `rules:` trigger is also changed (round 7, fixes
   `CA-NNP-R7-1`): drop the `changes:` path-scoping for this job
   entirely and run it unconditionally on every
   `merge_request_event`/push/tag pipeline** — the same unconditional
   pattern `commercial-corpus:validate` already uses in this same
   `.gitlab-ci.yml`. This is required, not optional: the manifest owner
   path this feature exists to relax is open-ended by design (a rule
   under `agf/rules/**`, a schema, a Pages page, or any other
   normative-owner-eligible artifact), so no fixed `changes:` path list
   can guarantee the job is selected whenever a candidate could make a
   previously-absent owner start resolving. A candidate that creates
   `agf/rules/workflow/new-rule.md` while leaving its already-published
   `stage: planned` manifest textually unchanged would otherwise select
   `rust:agov:validate` (via the `agf/**/*` diff) but not this job,
   reproducing the exact silent-owner-only-selection gap `CA-NNP-R2-2`
   found. Making the job unconditional is affordable per
   `ci-trigger-scoping-rule.md`'s own quick-feedback exemption: its
   measured exact-head runtime is ~200-300 runner-seconds (well under
   that rule's 5-minute default threshold for "long-running" stages
   requiring scoping in the first place). The job's other checks
   (`coord validate`, `plan validate`, the diff-scoped projection-check
   coverage pass) remain internally diff-scoped via `git diff
   --name-only | xargs -0r` and are unaffected no-ops on pipelines where
   nothing relevant changed; only `projection-check-all`'s full-tree
   pass runs unconditionally, which is the point.

   No plan `status` transition has any effect on CI behavior any more —
   the relaxation is entirely local to each manifest's own `stage`
   field, set and cleared by whoever edits that manifest, with no
   cross-file coordination required or possible. The direct,
   single-manifest `agov plan projection-check <manifest>
   [--allow-planned]` command (unchanged from round 1) remains available
   for a human reviewer to check one file by hand during Gate 1 review;
   it is not what CI invokes.

## What this plan does NOT do

- Does not touch MR !1817, its plan, its waiver, or treat the waiver as
  precedent for this design (the waiver predates and motivates this
  plan; this plan does not retroactively justify the waiver or vice
  versa).
- Does not relax the default (no-flag) `agov plan projection-check`
  invocation's *existing* path-resolution check in any way — that
  requirement is byte-identical to today for every surface, with or
  without the flag. **Correction (round 1, fixes `CA-NNP-R3`):** the
  default invocation does gain one new check that runs regardless of
  the flag — a `normative_owner.stage: planned` entry whose path
  already resolves is now rejected. **This is a BREAKING tightening for
  that one manifest shape, not "strictly additive"** (round 7, fixes
  part of `CA-NNP-R7-4`: retracted "strictly additive" — the very next
  clause already admitted a currently-passing manifest newly fails,
  which is the definition of a breaking change, not an additive one; a
  manifest that today has `stage: planned` on an *absent* path keeps
  passing, but a manifest that today has `stage: planned` on an
  *already-resolving* path — valid and passing today, since today's
  validator does not read `stage` at all — will newly fail). This is a
  relaxation for the absent-path case and simultaneously a tightening
  for the resolving-path case; neither the design's earlier "never
  inspects `stage` at all" claim nor an unqualified "additive" label
  covered this correctly — both are retracted. See "Downstream impact
  classification" below for the complete BREAKING/migration disclosure.
- Does not add a new blocking authority beyond what
  `documentation-rule-projection-rule.md` and its Gate-2-lifecycle
  requirement already grant the projection validator —
  `--allow-planned` narrows an existing check's scope for an
  explicitly-flagged invocation; the staleness check above enforces an
  existing lifecycle requirement (the rule's own Gate-2-evidence
  standard) mechanically, rather than creating an unrelated new one.
- Does not write adopter-facing Pages documentation for the
  projection-manifest process generally (out of scope; see "Operator
  documentation and adopter docs parity" above for what this plan does
  commit to).
- Does not leave either design question open — both are decided in this
  revision (see "Design decisions" above, fixes `CA-NNP-R2`).

## Files expected to change

This plan's own diff (Gate 1 candidate): only this plan file.

On the implementation branch, after Gate 1 approval:

- `agf/schemas/rule-projection-manifest.schema.json` — add
  `normativeOwnerEntry` definition (`surfaceEntry` + optional `stage`
  enum); repoint `surfaces.normative_owner`'s `$ref` to it. No other
  row's `$ref` changes.
- `agf/rules/workflow/documentation-rule-projection-rule.md` — new
  subsection documenting the `stage: planned` convention, the
  normative_owner-only scope, and the two-invocation lifecycle (fixes
  `CA-NNP-R4`'s adopter docs parity disposition).
- `.claude/rules/workflow/documentation-rule-projection-rule.md` — check
  whether a local mirror of this rule exists and needs the same update
  (this repository's `.claude/rules/` vs `agf/rules/` drift is a known,
  separate, pre-existing 51-finding gap per
  `agov.validate.assistant_drift` — this plan fixes only its own file's
  mirror consistency, not the broader drift).
- `tooling/rust/crates/agov/src/main.rs` — add `--allow-planned` flag to
  `PlanCommand::ProjectionCheck`, thread through to
  `plan_projection_check`; add new `PlanCommand::ProjectionCheckAll {
  base_ref: String, repo_root: Option<PathBuf> }` (round 2, fixes
  `CA-NNP-R2-1`, `CA-NNP-R2-2`, `CA-NNP-R2-6`; round 4, fixes
  `CA-NNP-R4-A1` — no manifest argument, enumerates every manifest in
  the tree with no plan correlation of any kind, see "Design
  decisions"; round 8, fixes part of `CA-NNP-R8-2`: names the exact new
  `repo_root: Option<PathBuf>` field this bullet previously omitted —
  optional, `None` defaults to the discovered current repository,
  matching `commercial-corpus validate --repo-root`'s existing CLI
  contract exactly).
- `tooling/rust/crates/agov-coord/src/projection_manifest.rs` — thread
  `allow_planned` into `validate_file_against` / `validate_paths_resolve`
  / `check_surface_paths`; scope the flag-gated relaxation to
  `normative_owner` only; add the new unconditional staleness check
  (fixes `CA-NNP-R3`); add the full-tree manifest enumeration and
  path/staleness check used by `projection-check-all` (round 2, fixes
  `CA-NNP-R2-2`; round 3, fixes `CA-NNP-R3-1`, `CA-NNP-R3-2` — no plan
  correlation, no `plan_id`-to-plan-file lookup, no plan `status`
  reading; relaxation is purely local to each manifest's own `stage`
  field); split the changed-pages-coverage check to run only for
  manifests present in the current diff, never the full tree (round 3,
  fixes `CA-NNP-R3-3`). **Round 10, fixes `CA-NNP-R10-1`/`CA-NNP-R10-2`
  — replaces the round-9 global-environment-variable design entirely
  with explicit parameter threading, no new `AGOV_*` variable at all:**
  verified directly that `validate_file(path: &Path, repo_root: &Path)`
  and `validate_paths_resolve`/`check_surface_paths` **already** take
  `repo_root` as an explicit parameter, not from `agov_core::repo_root()`
  — no change needed there. The one genuinely non-parameterized call is
  `validate_file`'s own schema load
  (`agov_core::schema_dir().join("rule-projection-manifest.schema.json")`
  then `schema::load_and_compile`). Change `validate_file`'s signature
  to accept an explicit `schema_path: &Path` parameter instead of
  calling `agov_core::schema_dir()` internally; every existing caller
  (including every current test in this file) passes
  `agov_core::schema_dir().join("rule-projection-manifest.schema.json")`
  explicitly at the call site — byte-identical resulting behavior, now
  visible in the function signature instead of hidden inside it.
  **Round 11, fixes `CA-NNP-R11-1` — the same `schema_path` parameter
  must also thread through `validate_file_against`, not stop at
  `validate_file`:** verified directly against the actual current
  source: `validate_file_against(path, repo_root, changed_pages_or_
  runbooks, ships_in_distribution)` calls `validate_file(path,
  repo_root)` internally (today's 2-argument signature) to perform the
  diff-scoped changed-pages-coverage check. Round 10's fix only added
  `schema_path` to `validate_file` itself, leaving
  `validate_file_against` with no parameter through which to receive or
  forward a fixture schema path — a manifest present in the diff (the
  path this function exists for) would silently fall back to
  `agov_core::schema_dir()` inside the nested `validate_file` call,
  reading the caller checkout's schema, not the fixture's, exactly
  reproducing `CA-NNP-R10-1`'s wrapper-conflict shape one call frame
  deeper. Fixed: `validate_file_against` also gains an explicit
  `schema_path: &Path` parameter and forwards it unchanged to its
  internal `validate_file(path, repo_root, schema_path)` call — no
  other behavior changes. Every existing caller of `validate_file_
  against` (including its own current tests) passes
  `agov_core::schema_dir().join("rule-projection-manifest.schema.json")`
  explicitly, matching `validate_file`'s own migration.
- `tooling/rust/crates/agov/src/main.rs` (`ProjectionCheckAll` handler;
  round 10, fixes `CA-NNP-R10-1`/`CA-NNP-R10-2`, replacing the round-9
  `AGOV_REPO_ROOT`/`AGOV_SCHEMA_DIR` env-var mechanism, which round 10
  proved conflicts with the supported `tooling/agov` wrapper — verified
  directly: the wrapper unconditionally exports
  `AGOV_SCHEMA_DIR="${AGOV_SCHEMA_DIR:-$repo_root/agf/schemas}"`
  *before* invoking the binary, so the Rust handler's own
  "only-if-unset" check for `AGOV_SCHEMA_DIR` always finds it already
  set to the caller's own checkout under the standard wrapper-invoked
  path, defeating the fixture boundary on exactly the supported operator
  path) — **the complete `repo_root` boundary as explicit parameters,
  zero environment variables:** when `repo_root: Some(path)` is
  supplied, the handler passes `path` directly wherever it already
  passes `repo_root: &Path` to `projection_manifest`'s functions (no
  change needed — already parameterized); computes
  `path.join("agf/schemas").join("rule-projection-manifest.schema.json")`
  and passes it as the explicit `schema_path` parameter to **both**
  `validate_file` (the full-tree path/staleness pass) **and**
  `validate_file_against` (round 11, fixes `CA-NNP-R11-1`: the
  diff-scoped changed-pages-coverage pass for manifests present in the
  current diff — round 10 only threaded `schema_path` to `validate_file`
  and stopped there, leaving this second, equally-real call path
  unfixed); and constructs the `ships_in_distribution` closure using
  the existing explicit-root `dist::validate(&PathBuf::from(
  "agf/dist/release-manifest.yaml"), path)` (verified directly: this
  lower-level function already takes `root: &Path` explicitly —
  `validate_default()` is a thin wrapper hardcoding
  `agov_core::repo_root()`, not the only entry point) instead of
  `dist::validate_default()`. Every one of these inputs is supplied
  explicitly by the caller, at the call site, with no environment
  variable read or set anywhere in this path — there is structurally no
  possible conflict with the wrapper's own `AGOV_SCHEMA_DIR` export,
  because this path never consults that variable. When `repo_root` is
  `None`, the handler passes `agov_core::repo_root()`,
  `agov_core::schema_dir().join(...)`, and calls `dist::validate_
  default()` exactly as before — byte-identical to the
  pre-`--repo-root` implementation for every existing invocation that
  never passes the flag. No shared function (`agov_core::repo_root()`,
  `agov_core::schema_dir()`, `dist::validate_default()`) is modified;
  every other command that calls them is completely unaffected, so no
  `agf/docs/site/reference/environment-variables.md` entry, no new
  downstream-impact classification for a global variable, and no
  cross-command regression test are needed — there is no new global
  contract to document, because none is introduced.
- `tooling/rust/crates/agov/tests/` (or equivalent integration-test
  location) — a new test invoked from **outside** any fixture directory
  that runs `projection-check-all --repo-root <fixture> --base-ref
  HEAD` against a fixture git repository built by the test itself
  (including its own `agf/schemas/rule-projection-manifest.schema.json`
  copy and its own `agf/dist/release-manifest.yaml`), and asserts the
  result reflects the fixture's own manifest, schema, distribution, and
  Git state, not the test-runner's own checkout (round 8/9/10, fixes
  `CA-NNP-R8-2`/`CA-NNP-R9-1`/`CA-NNP-R10-1`: the specific end-to-end
  proof each round required). A second case explicitly pre-sets
  `AGOV_SCHEMA_DIR` to point at a *conflicting* schema in the caller's
  own checkout before invoking `--repo-root <fixture>` (simulating
  exactly the supported `tooling/agov` wrapper's own precondition) and
  asserts the fixture's schema still wins — the specific supported-path
  reproduction `CA-NNP-R10-1` required, proving the explicit-parameter
  design is immune to the environment-variable conflict the round-9
  mechanism had. **A third case (round 11, fixes `CA-NNP-R11-1`) builds
  the fixture repository with a real two-commit range** — a base commit
  without the projection manifest, then a second commit that adds
  `docs/reviews/rule-projection/fixture.yaml` naming the fixture's
  changed-pages entry — and invokes `--repo-root <fixture> --base-ref
  <fixture's own base commit>` (a genuinely non-empty diff, unlike the
  first two cases' `--base-ref HEAD`), with a conflicting caller-checkout
  `AGOV_SCHEMA_DIR` pre-set exactly as in the second case. This is the
  specific shape that exercises `validate_file_against`'s diff-scoped
  coverage path, not merely the full-tree pass — asserts the coverage
  check reads the fixture's own schema, not the caller's, proving the
  schema-path threading closes the gap `CA-NNP-R11-1` found at that
  second call frame. **A fourth case (round 12, fixes `CA-NNP-R12-2`)
  builds a fixture where a surface's `path` is (a) an absolute path
  naming a real file that exists only in the test-runner's own
  checkout, (b) a `../`-escaping relative path reaching the same
  outside file, and (c) a symlink inside the fixture whose target
  resolves outside the fixture root to that same outside file** — each
  asserted to fail resolution under `--repo-root <fixture>` even though
  the named file genuinely exists on disk outside the fixture, proving
  `resolves()`'s containment check (round 12 fix, above) actually
  rejects all three escape shapes rather than merely joining paths
  optimistically. **A fifth case (round 14, fixes `CA-NNP-R14-1`) puts
  each of those same three escape shapes — absolute, `..`-escaping,
  outside-resolving-symlink — specifically on a `normative_owner.stage:
  planned` entry, then exercises it through both `--allow-planned`/
  `projection-check-all` (the relaxed route) and the ordinary no-flag
  path**, asserting all three fail with the new `Invalid`-
  classification error in both contexts — the fourth case above only
  proved the escape shapes fail resolution in general; this case
  proves they are not silently accepted by the *relaxed* route
  specifically, closing the interaction round 14 found between the
  round-12 containment fix and the pre-existing `stage: planned`
  lifecycle branch. **A sixth case, finalized against the fail-closed
  design (governor decision `20260824T185045Z-governor-reviewer-
  a86h.md`; supersedes the two intermediate attempts at
  `20260824T175442Z-contribution-auditor-gpkh.md` and
  `20260824T182750Z-contribution-auditor-7km1.md`) covers exactly the
  four cases the governor named as required acceptance evidence for
  `CA-NNP-R14-1`:** (a) a `normative_owner.stage: planned` entry naming
  a path that is directly a dangling symlink resolving outside the
  repository (`repo/dangling -> /outside/missing-rule.md`); (b) a
  chained relative dangling symlink (`repo/a -> b`, `repo/b ->
  /outside/missing-rule.md`, candidate path `a`); (c) a missing leaf
  whose parent directory is an existing, outside-resolving symlink
  (`repo/link -> /outside`, path `link/future-rule.md`); and (d) a
  legitimately absent non-symlink path (no symlink anywhere in the
  chain) — asserting (a)-(c) classify `Invalid` and (d) classifies
  `ContainedMissing`, all four through `--allow-planned`,
  `projection-check-all`, and the ordinary no-flag strict route,
  proving the fail-closed `symlink_metadata`-based walk (above)
  rejects every escape shape uniformly rather than special-casing each
  one.
- `tooling/rust/crates/agov-coord/src/projection_manifest.rs`'s
  existing `#[cfg(test)]` unit tests (round 12, fixes `CA-NNP-R12-1`) —
  two new cases for `normativeOwnerEntry`: a manifest with
  `normative_owner.stage: planned` and neither `path` nor `paths` set
  at all, and one with `stage: planned` and `paths: []`. Both must fail
  schema validation and the direct no-flag Gate 2 `agov plan
  projection-check` command, proving the strict acceptance path cannot
  be satisfied by a pathless or empty-`paths` planned marker.
- `.gitlab-ci.yml` — replace the blanket `plan projection-check`
  invocation with a single call to `plan projection-check-all --base-ref
  "$base"` (round 2, fixes `CA-NNP-R2-1`, `CA-NNP-R2-2`, `CA-NNP-R2-6`;
  round 3 simplification, fixes `CA-NNP-R3-1`, `CA-NNP-R3-2`,
  `CA-NNP-R3-3`; supersedes both the round-1 bash+python loop and the
  round-2 plan-correlation design). **Also drop the `agov:governance:
  validate` job's `changes:` path-scoping and make it run
  unconditionally** (round 7, fixes `CA-NNP-R7-1`) — see "Rule text
  changes" above for the full justification.
- `tooling/rust/crates/agov-validators/src/ci_rules.rs` (new file; round
  8, fixes `CA-NNP-R8-1`; round 11, fixes `CA-NNP-R11-2` — adds the
  missing pipeline-admission stage, per the resolution condition's
  named "simpler acceptable alternative": narrow and correct the local
  model rather than replace it with a live CI-lint dependency) — a
  small, dependency-free, **two-stage** GitLab CI `rules:` evaluator:
  1. **`pipeline_admitted(workflow_rules, context) -> bool`** —
     evaluates `.gitlab-ci.yml`'s own top-level `workflow: rules:`
     block (loaded the same way as job rules) against a context that
     additionally carries `CI_OPEN_MERGE_REQUESTS` and
     `CI_DEFAULT_BRANCH` (round 11: verified directly against this
     repository's actual `workflow:` block that it depends on both —
     a non-default-branch push is suppressed, `when: never`, exactly
     when `CI_OPEN_MERGE_REQUESTS` is truthy, since the MR-event
     pipeline is the canonical same-SHA path for that case). No
     pipeline means no job, regardless of the job's own `rules:`.
  2. **`job_selected(job_rules, context) -> bool`** (the round-8
     design, unchanged) — evaluates one job's parsed `rules:` array
     against the same simulated context (`CI_PIPELINE_SOURCE`,
     `CI_COMMIT_BRANCH`, `CI_COMMIT_TAG`, changed-paths set).
  `agov:governance:validate` is created for a given context if and only
  if both stages return true. This is the mechanism the new CI-rule
  self-test (below) uses to check the job is selected, by locally and
  deterministically evaluating a structural model kept in agreement
  with GitLab's own published `workflow:`/job `rules:`/`changes:`
  semantics (round 12, narrowed from "prove" to "check," fixes
  `CA-NNP-R12-A1`: this is not independent confirmation against a live
  GitLab API call or a real pipeline run — it models the complete
  admission chain instead of only its second half, without claiming
  platform-verified proof).

  **Supported expression subset and fail-closed contract (round 13,
  fixes `CA-NNP-R13-A1`; round 14, corrected, fixes `CA-NNP-R14P-2`:
  the round-13 subset omitted two expression shapes the live
  `workflow:` block actually uses, which would have made the promised
  self-verifying test fail immediately against real `.gitlab-ci.yml`
  content):** the evaluator supports exactly the expression shapes this
  repository's own `workflow:` and `agov:governance:validate` `rules:`
  actually use — verified directly against `.gitlab-ci.yml`:
  equality/inequality comparisons (`==`/`!=`) between a named CI
  variable (`$CI_PIPELINE_SOURCE`, `$CI_COMMIT_BRANCH`, `$CI_COMMIT_TAG`,
  `$CI_OPEN_MERGE_REQUESTS`, `$CI_DEFAULT_BRANCH`) and **either** a
  string literal **or another named CI variable** (round 14: the live
  `workflow:` block's suppression rule uses `$CI_COMMIT_BRANCH !=
  $CI_DEFAULT_BRANCH`, and its cache-writer rule uses
  `$CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH` — both variable-to-variable,
  neither expressible under the round-13 variable-to-literal-only
  subset); boolean `&&` combinations of those comparisons;
  truthy/falsy single-variable checks; and a `changes:` block matched
  against the test's supplied changed-paths set using plain glob
  patterns (`**`, `*`). It does **not** support regex match operators
  (`=~`/`!~`), function-call-style expressions, or any `rules:` clause
  this repository's own CI does not currently use.

  **Ordered first-match semantics, specified (round 14, fixes
  `CA-NNP-R14P-2`'s second gap — this was previously unstated even
  though the live workflow's correctness depends on it):** `rules:`
  entries are evaluated in file order; the first entry whose `if:`
  matches (or an entry with no `if:` at all, which always matches)
  determines the outcome, and evaluation stops there — later entries
  are not consulted. `when: never` on the first matching entry means
  not admitted/not selected; `when: always` (or an implicit default
  when a matching entry sets no `when:`) means admitted/selected. The
  live `workflow:` block's trailing `- when: always` is exactly this
  pattern: an unconditional catch-all that only matters because it is
  last, catching any context the preceding `when: never` rules did not
  match.

  **Fail-closed:** if `pipeline_admitted` or `job_selected` encounters
  a `rules:` entry using an unsupported expression shape, it returns an
  error (not a silent `true`/`false` guess) that the caller must
  surface as a test failure — an unsupported expression must never be
  interpreted as "admitted" or "selected" by default. This subset is
  re-verified against `.gitlab-ci.yml` by a dedicated unit test that
  parses the live file's `workflow:` and `agov:governance:validate:`
  blocks and asserts every clause found is inside the supported subset;
  that test fails loudly (not silently passes) the day either block
  adopts an expression shape the evaluator does not model, which is the
  fail-closed boundary this advisory required in place of a live CI
  Lint dependency. (Round 14: re-ran this check by hand against the
  current `.gitlab-ci.yml` text — with the variable-to-variable
  addition above, every clause in both blocks is now inside the
  supported subset.)
- `tooling/rust/crates/agov-validators/src/lib.rs` (round 9, fixes
  `CA-NNP-R9-2`: this file was missing from "Files expected to change"
  entirely) — add `pub mod ci_rules;`, following this crate's own
  existing convention of every module being explicitly declared in
  `lib.rs`. Verified directly: without this line, `ci_rules.rs` and its
  in-file `#[cfg(test)]` tests are not compiled or run by `cargo test -p
  agov-validators` at all — a plausible gate proof would exist on disk
  while being completely absent from the executable/test graph, exactly
  the failure class this self-test exists to prevent. Gate 2 acceptance
  evidence for this bullet specifically: the MR description names the
  exact `cargo test -p agov-validators` output line(s) showing the new
  `ci_rules` tests enumerated and passing, not merely that the file
  exists in the diff.
- Focused positive/negative Rust tests (see "Test strategy"), including
  the corpus-compatibility and coverage-split regression tests
  described there.
- `CHANGELOG.md` — a `Changed` (breaking) entry covering **all four**
  breaking cases (round 13, fixes `CA-NNP-R13-1`: this bullet said
  "both" and named only the first two, omitting the round-12 cases):
  (1) the `normative_owner.stage` enum narrowing to non-enum values;
  (2) (round 6, fixes part of `CA-NNP-R6-2`) the new unconditional
  staleness check rejecting a previously-valid `stage: planned`
  manifest whose path already resolves; (3) (round 12, fixes
  `CA-NNP-R12-1`) `stage: planned` with neither a non-empty `path` nor
  a non-empty `paths`; and (4) (round 12, fixes `CA-NNP-R12-2`) any
  surface's `path`/`paths` entry that is absolute, contains `..`, or is
  a symlink resolving outside the repository root — per this plan's own
  BREAKING classification below and
  `development-workflow-rule.md`'s disclosure requirement (round 5,
  fixes `CA-NNP-R5-3`: this file was promised in the "Downstream impact
  classification" section but omitted from this list, so an
  implementer following only this enumerated scope would
  skip the promised release note).

No changes to `agf/agents/*.yaml`, no database/schema migration in the
product sense, no new external dependency.

## Test strategy

New Rust unit tests in `projection_manifest.rs`'s existing `#[cfg(test)]
mod tests`, following its existing `surface()`/`write_manifest()` helper
pattern:

- **Positive — planned entry passes with the flag:** a manifest with one
  surface at `stage: planned` naming a path that does not exist,
  validated with `allow_planned: true` (or the CLI-level `--allow-planned`
  equivalent) — passes.
- **Negative — planned entry fails without the flag:** the identical
  manifest, validated with `allow_planned: false` (today's only mode) —
  fails with the existing "does not resolve to a real file" error,
  proving the default invocation's behavior is unchanged.
- **Negative — stale planned marker still fails once flag is dropped:**
  a manifest with `stage: planned` naming a path that *still* does not
  exist, validated without the flag (simulating a Gate 2 check on a
  manifest an author forgot to update) — fails, proving `stage: planned`
  never becomes a permanent exemption.
- **Negative — stale `planned` marker on an existing path is rejected,
  with or without the flag (fixes `CA-NNP-R3`; retracts the earlier
  "harmless" claim):** a manifest with `normative_owner.stage: planned`
  naming a path that *does* exist, validated both with and without
  `--allow-planned` — fails **both times** with the new "lifecycle
  transition incomplete" error, proving a stale `planned` marker cannot
  reach Gate 2 undetected.
- **Positive — `stage: existing` (or omitted) on an existing path always
  passes:** unchanged from today's behavior, confirming the new
  staleness check is scoped to `stage: planned` specifically and does
  not affect the normal case.
- **Negative — the planned-path exemption has no effect on a
  non-`normative_owner` surface (revised round 2, fixes `CA-NNP-R1`,
  `CA-NNP-R2-3`):** the original claim here — that `stage: planned` on
  a row like `distribution` or `ai_landing_loaders` "fails schema
  validation" — is false and retracted: `surfaceEntry` has no
  `additionalProperties: false` (verified directly against the schema
  file), so an extra `stage` key on those rows is accepted and ignored
  by JSON Schema, exactly like any other unrecognized property already
  is today. A local fixture confirming this (`stage: planned` set on
  `ai_landing_loaders`) passes schema validation with exit 0. What
  *is* true and provable, and what this test now actually asserts: a
  manifest with `stage: planned` and an unresolved path on
  `distribution` or `ai_landing_loaders` still **fails** resolution —
  the planned-path exemption never applies outside `normative_owner`
  (round 4, fixes `CA-NNP-R4-A1`: described behaviorally, not by
  asserting a specific implementation mechanism this plan doesn't
  itself choose — the validator today operates on raw
  `serde_json::Value`, not a distinct typed Rust struct per row, so the
  guard is whatever explicit check the implementation uses to read
  `stage` only when the surface being checked is `normative_owner`,
  not an incidental type-system boundary), regardless of what extra key
  the raw YAML/JSON happens to carry. This proves the relaxation
  cannot be used outside `normative_owner` by construction, which is
  the property that actually matters for `CA-NNP-R1`.
- **Regression — every existing test in the file's current suite
  continues to pass unmodified.** No existing test's fixture or
  assertion changes; `stage`-unaware manifests behave identically before
  and after this change.
- **New — current-corpus compatibility, real fixture (round 3, fixes
  `CA-NNP-R3-1`):** running `projection-check-all` against this
  repository's own actual manifest set — including
  `20260809T163051Z-v250-maintenance-release-line-rule.yaml` and
  `20260813T155743Z-frozen-scope-reconstruction-rule-amendment.yaml`
  (both `plan_id` values include a timestamp prefix their plan's actual
  front-matter `id` lacks) and `cl4-immutable-corpus-binding-gate1-
  plan.yaml` (no matching plan file exists at all) — passes cleanly,
  because none of that ever matters any more: no manifest triggers a
  plan lookup, since none has `normative_owner.stage: planned` set.
  This is the regression test `CA-NNP-R3-1` explicitly asked for: a
  real-corpus run, not only a synthetic fixture.
- **New — full-tree, non-diff-scoped path/staleness enumeration (round
  2, fixes `CA-NNP-R2-2`; round 3, retimed off plan status, fixes
  `CA-NNP-R3-2`):** a repository fixture with an existing manifest whose
  `normative_owner.stage: planned` path has newly started resolving
  (simulating a Gate 2 branch that created the owner file without
  touching the manifest) — `projection-check-all` still detects and
  rejects it via the staleness check, even when that manifest file is
  **not** part of the simulated branch's own diff, proving the
  non-diff-scoped design actually closes the gap `CA-NNP-R2-2` found.
  Separately, a fixture with a `normative_owner.stage: planned` manifest
  whose linked plan (if any exists at all) carries `status: approved`
  and whose owner path still does not exist — passes cleanly (relaxed),
  proving the approved-status deadlock `CA-NNP-R3-2` found cannot recur,
  since no plan or status is ever consulted.
- **New — changed-pages coverage stays diff-scoped (round 3, fixes
  `CA-NNP-R3-3`):** a fixture with one Pages file changed and one
  relevant manifest correctly declaring it, alongside several unrelated,
  unchanged manifests in the tree that do not declare that page —
  `projection-check-all` passes: the coverage check runs only against
  the one manifest present in the diff, and the unrelated manifests are
  checked only for path-resolution/staleness, never coverage. A second
  fixture proves the negative: if the *changed* manifest itself fails to
  declare the changed page, that specific manifest's coverage check
  fails as it does today.
- **New — Gate 1 passes, Gate 2 evidence fails, on the same absent-owner
  manifest (round 4, fixes `CA-NNP-R4-1`):** a manifest with
  `normative_owner.stage: planned` naming a path that does not exist.
  Run **both** commands against it: `projection-check-all --base-ref
  <ref>` passes (tree-wide relaxation, correctly tolerating a
  legitimately-still-pending owner); the direct `agov plan
  projection-check <manifest> --base-ref <ref>` (no flag) **fails**
  with the standard "does not resolve to a real file" error. This
  fixture, run against the real built CLI rather than mocked or
  simulated logic (round 5, fixes part of `CA-NNP-R5-2`: corrected from
  "not a synthetic fixture," which wrongly implied this was the actual
  manifest being handed off for a real Gate 2 rather than a test
  fixture), proves Gate 1's relaxed check and Gate 2's strict check
  produce genuinely different results for the same manifest, closing
  the boundary `CA-NNP-R4-1` found erased.
- **New — `paths` array staleness semantics (round 4, fixes
  `CA-NNP-R4-3`):** a `normative_owner` entry using `paths` (not
  singular `path`) with `stage: planned`, where **all** members are
  still absent — passes (relaxed) under `--allow-planned` /
  `projection-check-all`. A second fixture where **one** member of
  `paths` now resolves while the rest remain absent and `stage` is
  still `planned` — fails the staleness check (fixes fire on *any*
  resolving member, not only when all resolve), under both
  `projection-check-all` and the direct strict command. A third
  fixture where **all** members resolve — also fails staleness, as
  the more obviously-stale case.
- **New — `--repo-root` uses the fixture repository's own Git,
  schema, and distribution identity, not the caller's checkout, via
  explicit parameters and zero environment variables (round 8, fixes
  `CA-NNP-R8-2`; round 10, fixes `CA-NNP-R10-1`/`CA-NNP-R10-2` —
  replaces the round-9 `AGOV_REPO_ROOT`/`AGOV_SCHEMA_DIR` env-var
  design, which conflicted with the supported `tooling/agov` wrapper's
  own unconditional `AGOV_SCHEMA_DIR` export):** invoked from a working
  directory *outside* any fixture tree, `agov plan projection-check-all
  --repo-root <fixture-dir> --base-ref HEAD` against a fixture git
  repository built by the test (its own `git init`, one commit, its own
  `agf/schemas/rule-projection-manifest.schema.json` copy, and its own
  `agf/dist/release-manifest.yaml`) — the command's manifest discovery,
  path resolution, `--base-ref` resolution, schema loading (via the new
  explicit `schema_path` parameter), and distribution validation (via
  `dist::validate` with an explicit root, not `validate_default()`) all
  operate against `<fixture-dir>`'s own tree, Git history, schema, and
  distribution manifest, never the test process's own checkout or
  working directory. A negative companion asserts that a manifest
  present only in the *caller's* checkout (not the fixture) is never
  discovered when `--repo-root` points elsewhere. **A wrapper-conflict
  case (round 10, fixes `CA-NNP-R10-1` directly) pre-sets
  `AGOV_SCHEMA_DIR` to a conflicting caller-checkout schema before
  invoking `--repo-root <fixture-dir>`** — reproducing exactly the
  supported `tooling/agov` wrapper's own precondition — and asserts the
  fixture's schema still wins, because the explicit-parameter path never
  reads `AGOV_SCHEMA_DIR` at all when `--repo-root` is supplied. **A
  diff-scoped wrapper-conflict case (round 11, fixes `CA-NNP-R11-1`)
  repeats the wrapper-conflict case against a fixture repository with a
  real two-commit range and a non-empty `--base-ref`** (not `HEAD`),
  so the manifest is actually present in the diff and
  `validate_file_against`'s coverage path — not only `validate_file`'s
  full-tree path — is exercised; asserts the fixture's schema still
  wins there too, closing the gap the first wrapper-conflict case's
  `--base-ref HEAD` (empty diff, by construction) could not detect. A
  final case confirms omitting `--repo-root` entirely falls back to
  today's behavior byte-for-byte (current directory, current checkout,
  current `AGOV_SCHEMA_DIR` if externally set — unaffected, since that
  fallback path is unchanged).
- **New — CI job selection self-test, structural/local-model evidence
  only (round 8, fixes `CA-NNP-R8-1`; round 11, fixes `CA-NNP-R11-2` —
  corrects the claimed contexts to match the repository's actual
  two-stage admission chain, per `gate-self-test-rule.md`; round 12,
  narrows "proves" to "agrees," fixes `CA-NNP-R12-A1`) that
  `agov:governance:validate` is created and blocks on an owner-only
  stale-manifest candidate:** using the
  new two-stage `agov-validators::ci_rules` evaluator
  (`pipeline_admitted` then `job_selected`) against this plan's own
  implemented `.gitlab-ci.yml` — both the top-level `workflow: rules:`
  block and `agov:governance:validate`'s own job `rules:` — across five
  simulated contexts (round 11: replaces the round-8 "non-default-branch
  push" context, which is ambiguous and usually suppressed in this
  repository's actual workflow, with its two real sub-cases):
  1. `merge_request_event` — admitted, job selected.
  2. Non-default-branch push, **no open MR** (`CI_OPEN_MERGE_REQUESTS`
     unset/false) — admitted, job selected.
  3. Non-default-branch push, **with an open MR**
     (`CI_OPEN_MERGE_REQUESTS` truthy) — **not admitted**: asserts
     `pipeline_admitted` returns false, so no job is created regardless
     of the job's own `rules:` — the correct, intentional suppression
     this repository's `workflow:` block already implements (the
     MR-event pipeline in case 1 is the canonical same-SHA path for
     this case), not a gap.
  4. Default-branch push — admitted, job selected.
  5. Tag push — admitted, job selected.
  Cases 1, 2, 4, and 5 are additionally crossed with a changed-paths set
  containing *only* `agf/rules/workflow/new-example-rule.md` (simulating
  exactly the owner-only candidate `CA-NNP-R7-1` named, with the linked
  `stage: planned` manifest itself absent from the diff) — asserts the
  job is selected (created) in each of those four admitted contexts.
  Case 3 asserts non-admission independent of changed paths, since no
  job rule is ever evaluated when the pipeline itself does not exist. A
  further assertion runs the real binary's `projection-check-all`
  against a fixture where that same owner path now resolves and the
  manifest still says `stage: planned`, confirming the created job's
  own command then fails with the staleness error. Together these
  assertions are the visible, reproducible proof the prior audits
  required: not merely that the Rust command works in isolation, but
  that this evaluator's structural, local model — kept deliberately in
  agreement with GitLab's documented `rules:`/`workflow:` semantics for
  exactly the contexts these tests construct (round 12, narrows the
  claim, fixes `CA-NNP-R12-A1`; this is *not* independent confirmation
  against the live platform, since `ci_rules` has no CI Lint API or
  live-pipeline dependency) — agrees the job is selected for the exact
  failure shape `CA-NNP-R7-1` described in every context where a
  pipeline is actually admitted, correctly does not in the one common
  context where it legitimately is not, and that the created job then
  blocks.
- `agov plan validate` on this plan file: must pass.
- `agov validate` (full umbrella) at the exact implementation head: must
  show no new findings beyond pre-existing, unrelated warnings.
- `cargo test -p agov-coord`, `cargo test -p agov --bin agov`, and
  `cargo test -p agov-validators`: must pass, including the new tests
  above.
- **Gate 2 acceptance evidence, revised (round 4, fixes `CA-NNP-R4-2`;
  round 5, fixes `CA-NNP-R5-2`; round 6, fixes `CA-NNP-R6-1`; round 7,
  fixes `CA-NNP-R7-2`/`CA-NNP-R7-3`; round 8, fixes part of
  `CA-NNP-R8-1` — adds item 3, the CI-selection proof `CA-NNP-R7-1`'s
  own resolution condition required but round 7 never added; per
  `gate-self-test-rule.md`):** the Gate 2 MR description for *this
  plan's own implementation* — which ships the CLI feature, not a
  net-new normative-owner manifest — MUST include **three** distinct
  pieces of evidence using the actual built CLI binary against an
  isolated temporary fixture tree (not only unit tests, and never
  presented as this plan's own real candidate, because this plan
  carries no such candidate manifest):
  1. **Positive (Gate 1 relaxed-pass case):** in a temporary directory
     that is its own minimal git repository (`git init`, one commit),
     create `docs/reviews/rule-projection/fixture.yaml` with
     `normative_owner.stage: planned` naming a path that does not exist
     anywhere in that fixture tree; run `agov plan projection-check-all
     --repo-root <fixture-dir> --base-ref HEAD` — expected and actual
     **passing** output (exit 0), proving the tree-wide route correctly
     tolerates a legitimately-still-pending owner. Because `--base-ref
     HEAD` points at the fixture repo's only commit, the diff-scoped
     coverage check never fires for this manifest — only the full-tree
     staleness pass is exercised, which is the point.
  2. **Negative, demonstrates-it-gates (per `gate-self-test-rule.md`;
     round 6, fixes `CA-NNP-R6-1`; round 7, fixes `CA-NNP-R7-3` — now
     reproducible via `--repo-root`, not an unreachable "isolated
     fixture file"):** in a second temporary fixture repository built
     the same way, the declared `normative_owner.stage: planned` path
     *does* resolve inside that fixture tree (the lifecycle-transition-
     incomplete case) — the same `--repo-root`/`--base-ref HEAD`
     invocation now produces **nonzero** output (the new staleness
     error), with the full expected/actual transcript and exit code
     visible in the MR description, proving the gate fires on the one
     input `stage: planned` actually forbids: a stale marker whose
     promised absence turned out to be false.
  3. **CI job selection, admission-then-selection-then-negative (round
     8, fixes `CA-NNP-R8-1`; round 9, fixes part of `CA-NNP-R9-2`;
     round 11, fixes `CA-NNP-R11-2`):** the exact `cargo test -p
     agov-validators` output line(s) showing the new `ci_rules` tests
     enumerated and passing (proving the module is actually registered
     and reachable, not merely present on disk — see "Files expected to
     change"); then the two-stage evaluator's output for
     `agov:governance:validate`'s implemented `workflow:`/job `rules:`
     against a changed-paths set containing only a new
     `agf/rules/workflow/` file, across all five contexts from "Test
     strategy" above — expected and actual output showing the job is
     selected in the four admitted contexts and correctly *not*
     admitted (no job, independent of job `rules:`) in the
     open-MR-push context; then the real binary's `projection-check-all
     --repo-root <fixture-dir> --base-ref HEAD` against that same
     fixture, where the linked `stage: planned` manifest's owner now
     resolves — expected and actual nonzero staleness-error output.
     Together these transcripts show the corrected CI-selection contract
     does what `CA-NNP-R7-1` required, to the extent local-model
     structural evidence can (round 13, narrows "prove"/"actually
     created"/"actually blocks" throughout this bullet, fixes
     `CA-NNP-R13-A1`; see the evaluator's supported-subset and
     fail-closed contract below): the self-test module runs, and
     `ci_rules`'s local model of `agov:governance:validate` agrees the
     job is selected for the owner-only case in every context where its
     model of the pipeline is admitted, correctly disagrees in the one
     context where admission legitimately fails, and the real binary
     (not the local model) is what then actually blocks against the
     staleness fixture — that last step, unlike job selection, *is*
     real-binary evidence, not local-model agreement.
  This implementation's own Gate 2 does **not** additionally require
  the direct single-manifest `agov plan projection-check <manifest>
  --base-ref <ref>` (no flag) command to pass against a real candidate
  manifest — that requirement is the future-user lifecycle obligation
  named in "Rule text changes" above, owed by whichever later plan
  actually adds a `stage: planned` normative-owner manifest, not by
  this plan, which adds none.
  "Tested locally" without all three transcripts is insufficient per
  that rule.
- **Gate 2 acceptance evidence, BREAKING release note (round 5, fixes
  `CA-NNP-R5-3`):** the Gate 2 MR diff MUST include the `CHANGELOG.md`
  `Changed` (breaking) entry named in "Files expected to change" and
  the "Downstream impact classification" below. A Gate 2 candidate
  missing that entry has not satisfied this plan's own BREAKING
  migration contract, regardless of how the schema/validator/CI changes
  otherwise test.

## Downstream impact classification (`downstream-impact-rule.md`)

This plan touches `agf/rules/**/*.md`, `agf/schemas/*.json`, and
`tooling/**` — three trigger paths.

```
Impact:
  Foundry:          ADDITIVE for this project's own manifest corpus —
                     verified, no current manifest sets `stage` on any
                     row (see "Compatibility scope" below). BREAKING at
                     the schema-contract level specifically for
                     `normative_owner.stage` (round 2, fixes
                     `CA-NNP-R2-4`): that field's value space narrows
                     from "any value accepted, unconstrained" to
                     exactly `["existing", "planned"]`, restricting
                     previously-permitted content per
                     `downstream-impact-rule.md`, regardless of whether
                     any manifest in this repository's own corpus
                     currently exercises the wider space.
  Direct adopters:  BREAKING for any adopter-maintained manifest that
                     already sets `surfaces.normative_owner.stage` to a
                     value other than "existing" or "planned" (round 2,
                     fixes `CA-NNP-R2-4`; this repository's own absence
                     of such a manifest does not prove absence in
                     adopter-owned manifests outside it). **Also
                     BREAKING** (round 6, fixes `CA-NNP-R6-2`) for any
                     adopter-maintained manifest that already sets
                     `normative_owner.stage: planned` on a path that
                     *already resolves* — today's validator has no
                     unconditional staleness check, so this shape
                     currently passes; after this plan, the new
                     staleness check rejects it as a stale marker.
                     Neither this repository's own absence of such a
                     manifest (verified — no current manifest sets
                     `stage` at all) nor the absence of an
                     `additionalProperties: false` constraint proves
                     absence in adopter-owned manifests outside it.
                     ADDITIVE only for an adopter with no manifest in
                     any of the four breaking shapes below — no
                     resolution-behavior change unless they deliberately
                     adopt `--allow-planned`. Adopter docs parity:
                     updated (see "Operator documentation and adopter
                     docs parity" above). **Also BREAKING** (round 12,
                     fixes `CA-NNP-R12-1`) for any adopter-maintained
                     manifest that already sets
                     `normative_owner.stage: planned` with neither
                     `path` nor a non-empty `paths` set — today's
                     schema accepts this (no path/paths requirement
                     exists on any row); after this plan, the new
                     conditional requirement rejects it. **Also
                     BREAKING** (round 12, fixes `CA-NNP-R12-2`) for any
                     adopter-maintained manifest, on *any* surface, that
                     declares a `path`/`paths` entry which is an
                     absolute filesystem path, contains a `..`
                     component, or is a symlink whose target resolves
                     outside the repository root — today's `resolves()`
                     has no containment check and may accept such an
                     entry if it happens to point at a real file
                     anywhere on the filesystem; after this plan, the
                     containment check rejects all three shapes
                     regardless of whether the named file exists.
  Downstream:       No adopter-produced manifest's shape changes unless
                     the adopter already has a manifest in one of the
                     four breaking shapes above (a non-enum `stage`
                     value; `stage: planned` on an already-resolving
                     path; `stage: planned` with no path/paths at all;
                     or any surface's path escaping the repository root)
                     or deliberately opts into `stage: planned` going
                     forward. No existing agent behavior toward an
                     adopter's users changes.
  Migration:        Required for all four breaking cases above (round
                     2, fixes `CA-NNP-R2-4`; round 6, extends to
                     `CA-NNP-R6-2`; round 12, extends to `CA-NNP-R12-1`
                     and `CA-NNP-R12-2`). Discovery: grep an adopter's
                     own `docs/reviews/rule-projection/*.yaml` corpus
                     for `normative_owner:` blocks whose `stage:` value
                     is (a) neither absent, `existing`, nor `planned`,
                     (b) exactly `planned` where the declared path
                     already resolves, or (c) exactly `planned` with
                     neither `path` nor a non-empty `paths`; and, on any
                     row, (d) a `path`/`paths` entry that is absolute,
                     contains `..`, or is a symlink resolving outside
                     the repository. Migration: for (a), correct the
                     value to `existing` (if the path already resolves
                     — the near-universal case) or remove the key
                     entirely; for (b), correct the value to `existing`
                     or remove the key; for (c), add a concrete `path`
                     or `paths` entry, or drop `stage: planned` if the
                     owner does not actually exist yet is not being
                     tracked; for (d), replace the entry with a
                     repository-relative path with no `..` components —
                     all four are schema-valid corrections under the
                     new constraints. Release note: call out all four
                     cases in the CHANGELOG under a `Changed` (breaking)
                     heading, not folded into `Added`, per
                     `deprecation-policy-rule.md`'s sibling principle
                     for breaking schema narrowing. Rollback: reverting
                     the schema, staleness-check, and path-containment
                     changes restores the unconstrained value space and
                     the pre-plan behavior; no adopter action needed to
                     roll back, since a migrated value remains valid
                     under the old, wider schema and resolver too.
  Evidence:         GitLab issue #115; governor decision
                     `20260823T191334Z-governor-reviewer-am2p.md`; the
                     concrete conflict `CT-UWD-1`/`CA-UWD-R3-4` exposed
                     in the MR !1817 review chain; contribution-auditor
                     finding `CA-NNP-R7` (round 1) for the compatibility-
                     scope correction below; finding `CA-NNP-R2-4`
                     (round 2) for the non-enum-value reclassification;
                     finding `CA-NNP-R6-2` (round 6) for the second,
                     previously-omitted `stage: planned`-on-resolving-
                     path breaking case; findings `CA-NNP-R12-1` and
                     `CA-NNP-R12-2` (round 12) for the pathless-planned
                     and path-containment breaking cases.
```

### Compatibility scope, precisely (fixes `CA-NNP-R7`)

`surfaceEntry` has no `additionalProperties: false` today — an adopter
manifest could already set an arbitrary `stage`-named key with any value
on *any* row, and schema validation would accept it (verified directly
against `agf/schemas/rule-projection-manifest.schema.json`). This plan's
narrowed design (fixes `CA-NNP-R1`) means the newly-constrained
`enum: [existing, planned]` applies to exactly one row,
`normative_owner`, via the new `normativeOwnerEntry` definition — every
other row's `surfaceEntry` is untouched and keeps accepting an arbitrary
`stage` value with any content, exactly as today. The
`normative_owner`-scoped schema collision risk is bounded to **two**
cases (round 7, fixes part of `CA-NNP-R7-4`: this section previously
named only the first): (1) an existing adopter manifest that already
sets `surfaces.normative_owner.stage` to some value other than
`"existing"` or `"planned"`; and (2) an existing adopter manifest that
already sets `normative_owner.stage: planned` on a path that **already
resolves** — schema-valid and passing today (today's validator does not
read `stage` at all), rejected by this plan's new unconditional
staleness check.

**Two more cases, added round 12 (fixes `CA-NNP-R13-1`: this section
was not updated when they were introduced) — the total is four, not
two:** (3) an existing adopter manifest that already sets
`normative_owner.stage: planned` with neither a non-empty `path` nor a
non-empty `paths` — schema-valid and passing today, rejected by the new
conditional schema requirement. (4) **Not scoped to `normative_owner`**
— an existing adopter manifest, on *any* surface row, whose `path` or
any `paths` member is an absolute filesystem path, contains a `..`
component, or is a symlink resolving outside the repository root —
accepted today by `resolves()`'s unguarded `repo_root.join(relative)
.is_file()` if the path happens to point at a real file anywhere on the
filesystem, rejected by this plan's new containment check. Case 4 is
the one case in this section that is not limited to `normative_owner`,
since `resolves()` is the shared primitive every surface and the
changed-pages/runbooks coverage set already calls.

No manifest in this repository's own corpus hits any of the four cases
(verified: no current manifest sets `stage` on any row, and every
current path is a relative in-repository path with no `..` component).
Adopters maintaining their own manifests outside this repository cannot
be verified from here; the rule-text addition (see "Rule text changes")
states the `normative_owner`-scoped constraints explicitly so an
adopter hitting cases 1-3 understands why, rather than encountering an
unexplained validation failure. Case 4's containment constraint is a
property of the shared resolver rather than the `normative_owner`
schema text specifically.

## Rollback

Additive in the ordinary case, with **four** narrow BREAKING exceptions
(round 2, fixes `CA-NNP-R2-4`; round 6, adds the second, fixes
`CA-NNP-R6-2`; round 7, fixes part of `CA-NNP-R7-4`; round 12, adds the
third and fourth, fixes `CA-NNP-R12-1`/`CA-NNP-R12-2`; round 13, fixes
`CA-NNP-R13-1` — this section still said "two narrow BREAKING
exceptions" after round 12 added the other two — see "Downstream impact
classification" above): one optional, scoped schema field constrained
to an enum on `normative_owner` only, now also conditionally requiring
a non-empty `path`/`paths` when `stage: planned` (ignored by every
manifest that does not set `stage` at all, and breaking only for the
four specific, verified-absent-in-this-corpus cases — a pre-existing
non-enum `stage` value; a pre-existing `stage: planned` on an
already-resolving path; a pre-existing `stage: planned` with no
`path`/`paths`; or, on any surface, a pre-existing `path`/`paths` entry
escaping the repository root), one new opt-in CLI flag (default
`false`, matching today's only behavior) plus the new non-diff-scoped
`projection-check-all` CI entrypoint (now selected unconditionally, not
path-scoped — see `CA-NNP-R7-1` above), the shared `resolves()`
containment check (round 12), one new rule subsection, and new tests.
Reverting before any manifest depends on the feature is a one-commit
revert with no data or manifest-rewrite implications; an adopter who
already migrated any of the four breaking cases stays valid after a
revert too, since every migrated value or corrected path is a subset of
the old, wider space (see "Migration" and "Rollback" in the Downstream
impact block above).

**Lifecycle-aware rollback, once the feature is in active use (fixes
`CA-NNP-R7`):** if a Gate-1-stage manifest already depends on
`stage: planned` + `--allow-planned` at the moment of a hypothetical
revert, reverting reproduces exactly the problem this plan fixes for
that one manifest: its declared not-yet-existing path fails the
(now-unconditional-again) strict check, blocking its own Gate 1 exactly
as MR !1817 was blocked before `am2p`. Rollback in that state requires
either (a) waiting until every in-flight `stage: planned` manifest's
declared path exists (i.e., its Gate 1 has already passed and
implementation has landed) before reverting, or (b) issuing a fresh
one-time governor waiver for each still-pending manifest, mirroring
`am2p`'s original shape, for the duration between the revert and that
manifest's own implementation landing. This is not a hidden risk: it is
the same structural conflict this plan exists to solve, recurring for
any manifest that was mid-flight at revert time — rollback does not
erase that conflict, it only removes the general mechanism for handling
it, requiring the ad hoc per-plan waiver path again.
