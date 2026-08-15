---
id: v261-release-asset-contract-fix
status: pending-gate1
topic: v261-release-asset-contract-fix
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Round 1 (coder-tester exact-head review `4h25`, MR !1580): BLOCK, four
      bounded corrections, addressed in this revision — see "Revision
      history" below. The named contribution-auditor's own sign-off (per
      MANIFEST.yaml: human + Claude at plan review, both self-review-
      conflicted for a plan I authored — same conflict already resolved for
      MR !1492 via artifact-specific routing) is requested below and remains
      outstanding before Gate 1. This entry does not claim that sign-off has
      happened.
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Requested via this plan's own routing (see "Decision requested"); no
      change to network calls, secrets, or provenance-record content, only
      to which target set is mandatory and when the check runs. The named
      infosec-sentinel's own semantic consultation, or a plan-specific
      governor waiver matching `33br`'s established boundary, is requested
      below and remains outstanding before Gate 1.
---

# Plan: fix the v2.6.0 release-asset contract gap (mandatory-target drift)

## Decision requested

Authorize an ordinary main-track fix (not a frozen-scope reconstruction) to
`tooling/rust/scripts/release-assets.sh`, `tooling/rust/scripts/
agov-oci-image.sh`, and `.gitlab-ci.yml`, correcting the defect that caused
`v2.6.0`'s tag pipeline to fail at `release:create` after the tag was
already pushed. This plan does **not** request authorization to
cut, tag, or publish `v2.6.1` — that is a separate, later, and much smaller
decision once this fix is merged and proven on `main`. It also does not
request any action on the retained `v2.6.0` tag or its OCI residue beyond the
documentation step in "Namespace handling" below.

Requested per governor decision
`docs/coordination/20260815T193735Z-governor-reviewer-spmj.md`, which
authorized and instructed planner-architect to prepare this plan, downstream
of `docs/coordination/20260815T193127Z-coder-tester-pj71.md`'s diagnosis and
the proposed learning
`docs/coordination/20260815T193807Z-coder-tester-2k22.md` (merged,
`agf/learnings/foundry.yaml` entry `20260815T193807Z-2k22`).

## Revision history

**Round 2 (this revision), per coder-tester's exact-head review
`20260815T203020Z-coder-tester-4h25.md` (MR !1580): BLOCK, 4 blocking + 1
advisory.** All five addressed:

- `B-V261-P1` (critical): the round-1 design gated `agov:oci:release` on a
  verifier checking the *complete* mandatory set, but `expected()` includes
  `agov-oci-${version}-provenance.json`, which `agov-oci-image.sh` can only
  produce *after* the image is pushed (its own header: "using the digest
  `push` wrote" — confirmed by reading the script directly, not assumed).
  Verifying the complete set before any OCI action is impossible, not just
  mis-ordered. Fixed by splitting verification into two real phases — see
  "Scope" item 6, revised.
- `B-V261-P2` (critical): round-1's dry-run reproduction used hand-created
  empty fixture files, not real build output. Fixed with a real preflight
  job, reusing the exact throwaway-branch pattern already proven today for
  the aarch64 necessity check — see "Scope" item 8, new.
- `B-V261-P3` (material): the new manifest file was never added to
  `release:assets-selftest`'s trigger-path list. Fixed — see "Scope" item 7.
- `B-V261-P4` (material): the round-1 negative test conflated a
  cross-contract-parser test with an ordinary missing-file test; a valid
  second-target line would make both consumers agree, not disagree. Fixed
  by separating three distinct tests — see "Test and validation plan".
- Advisory: fixed the self-contradictory `consulted`/"not yet performed"
  frontmatter above — the same defect class round 1 of the *v2.6.0* plan
  itself was rejected for, which should have been caught by precedent.

Also independently found while re-reading the source for this revision (not
raised by `4h25`, but the same root-cause class): `agov-oci-image.sh`'s
`target_slug()` function is a **third** unsynchronized hardcoded copy of the
target-to-slug mapping, alongside the two `4h25` and the round-1 plan already
named. Folded into "Scope" item 5.

## Incident, independently re-verified before writing this plan

- Tag `v2.6.0` was pushed correctly (protected-tag rule, namespace checks,
  branch protections all correct — governor decision `m11h`) at merge commit
  `4890dfe9ef96abdbfc54f9e975080cedd0f280f6`.
- Tag pipeline `2762930639`: 17 jobs succeeded (Rust build/test, strict
  validation, distribution checks, x86_64 tooling archive, framework
  archive, OCI publish, OCI pull test). `release:create` (job
  `15917296084`) failed; `release:verify` was skipped as a consequence.
  Independently re-pulled the job trace directly rather than trusting
  `pj71`'s description alone: confirmed the exact three missing-file lines
  (`agov-2.6.0-linux-aarch64.tar.gz`, `.tar.gz.sha256`,
  `.tar.gz-provenance.json`) and the `verify FAILED` exit.
- Current public state: Git tag exists; GitLab Release and generic package
  `agf/v2.6.0` are absent; OCI tag `agov:2.6.0` is published (job
  `15917296086` succeeded before `release:create` failed); issues #76, #84,
  #85, #89 remain open; no forward-sync to `main` was performed.

## Root cause (read directly from source, not assumed from the trace alone)

`.gitlab-ci.yml`'s `rust:tooling:release` job builds only
`x86_64-unknown-linux-musl` (line ~1063: `for target in
x86_64-unknown-linux-musl; do`) — `aarch64-unknown-linux-musl` was dropped
from this loop on 2026-08-05 per
`docs/plans/20260805T084100Z-drop-aarch64-linux-required-target.md`, and that
drop is what commit `3456c1fa` carries. That plan never touched
`tooling/rust/scripts/release-assets.sh`.

`release-assets.sh`'s `expected()` function independently hardcodes the
mandatory-asset filename set, including
`agov-${version}-linux-aarch64.tar.gz` and its two sidecars (lines 72-74).
This is a **second, unsynchronized copy** of the target list. No commit since
2026-08-05 ever exercised a real tag pipeline against this combination —
`v2.5.0` was tagged before the drop; `v2.6.0` is the first tag pipeline to
run since. This is a genuinely new defect surfaced for the first time by
`v2.6.0`, not a regression of something previously fixed.

`release-assets.sh selftest`'s own fixtures are built from
`$(expected "$test_version")` (lines ~113, ~150 in the current script) — its
positive fixture cannot detect a stale `expected()` because it derives from
the same function it is nominally checking. This is why
`release:assets-selftest` (which runs on every branch/MR change to the
script or `.gitlab-ci.yml`, and on every tag pipeline) never caught the drift.

Separately: `agov:oci:release` (`needs: rust:tooling:release` only) and
`release:create` (`needs: agf:dist:release`, `rust:tooling:release`,
`agov:oci:release`) both start once `rust:tooling:release` finishes. The
asset-contract `verify` call lives only inside `release:create`'s own
script, which does not gate `agov:oci:release`. This is why the OCI image
published successfully while the release-asset check failed a stage later
— a partial, confusing public state that would recur with any future
target-set change, not just this one.

**A third unsynchronized copy** (found while writing this revision):
`tooling/rust/scripts/agov-oci-image.sh`'s `target_slug()` function
independently hardcodes the same `x86_64-unknown-linux-musl`/
`aarch64-unknown-linux-musl` → slug mapping a third time. It does not error
today only because nothing currently calls it with the aarch64 triple — but
it is a third place a future target-set change must remember to touch by
hand.

**Hard constraint on verification ordering** (read from
`agov-oci-image.sh`'s own header, not assumed): the OCI provenance record
(`dist/agov-oci-${version}-provenance.json`, one of `expected()`'s mandatory
entries) can only be built *after* `push`, because it records "the digest
`push` wrote" — the actual registry-side manifest digest, not a locally
computable one. A local pre-push digest is not guaranteed identical to the
registry's (compression/re-encoding/manifest-list differences are possible
depending on registry behavior), so computing provenance from a local digest
instead would weaken the provenance record's own guarantee (proving exactly
what was published) rather than merely reordering when it's computed. This
plan does not attempt to make OCI provenance available pre-push — it designs
around the constraint instead (see "Scope" item 6).

## Scope

**In scope** (all in `tooling/rust/scripts/release-assets.sh` and
`.gitlab-ci.yml`, ordinary main-track change per
`release-via-merge-request-rule.md`'s standard workflow — no frozen-scope
exception needed, this is exactly the kind of individually-approved change
that belongs on `main` directly):

1. **Single source of truth for the mandatory Linux target set.** Add
   `tooling/rust/release-targets.txt`: one line per mandatory target, format
   `<rust-triple> <public-slug>` (currently one line:
   `x86_64-unknown-linux-musl linux-x86_64`). Both consumers below read this
   file; neither hardcodes a target list independently.
2. **`.gitlab-ci.yml`'s build loop** (`rust:tooling:release`'s script,
   currently `for target in x86_64-unknown-linux-musl; do ... case
   "$target" in x86_64-unknown-linux-musl) slug="linux-x86_64" ;;
   aarch64-unknown-linux-musl) slug="linux-aarch64" ;; *) exit 1 ;; esac
   ...`): replace the hardcoded `for target in ...` list and the `case`
   slug mapping with a loop that reads `tooling/rust/release-targets.txt`
   line by line, splitting `target` and `slug` per line. Removes the
   now-dead `aarch64-unknown-linux-musl) slug="linux-aarch64"` case arm —
   it was leftover cruft with no corresponding loop entry, not a live
   contract.
3. **`release-assets.sh`'s `expected()` function**: remove the hardcoded
   `agov-${version}-linux-x86_64...`/`agov-${version}-linux-aarch64...`
   lines; generate the per-target archive/checksum/provenance triplet from
   the same `tooling/rust/release-targets.txt`, resolved relative to the
   script's own directory (`$(dirname "$0")/release-targets.txt`) so it
   works regardless of caller CWD. `agf-*` mandatory lines are untouched
   (not per-target). The `agov-oci-*-provenance.json` line moves to a
   **separate function**, `expected_oci()` — see item 6 below for why.
4. **`agov-oci-image.sh`'s `target_slug()`**: read the same
   `tooling/rust/release-targets.txt` instead of its own hardcoded
   `x86_64-unknown-linux-musl`/`aarch64-unknown-linux-musl` case arms. Third
   and last consumer of the shared manifest; no hardcoded target list
   remains anywhere in the tree after this item and items 2-3.
5. **Non-self-referential cross-contract check.** Single-sourcing removes
   the possibility of the three lists (build loop, `expected()`,
   `target_slug()`) drifting from *each other* by construction, but does
   not itself prove any one consumer's manifest-parsing logic is correct.
   Add a check to `release-assets.sh selftest` that reads
   `tooling/rust/release-targets.txt` directly with separately-written
   parsing (e.g. `awk`, not `expected()`'s own heredoc-based logic),
   independently computes the expected per-target filenames, and asserts
   the result equals `expected()`'s actual output. Also assert the parser
   rejects a malformed line (missing slug column) with a clear error rather
   than silently producing a wrong filename — this is the genuinely
   independent check spmj's decision requires, and it can catch a parsing
   bug even though all consumers now read the same file.
6. **Fixture independence for `selftest`'s own positive/negative cases.**
   Per the incident's exact stated cause, replace the fixture-generation
   line `fixture_names="$(expected "$test_version")..."` with a **literal,
   hand-written filename list** for the mandatory-asset portion of the
   fixture (matching current `release-targets.txt` content at time of
   writing, with a comment stating this literal must be updated by hand
   whenever `release-targets.txt` changes). This is deliberately *not*
   derived from `expected()`, so a bug that breaks `expected()` cannot also
   break the fixture that is supposed to catch it. The Windows-preview and
   `SHA256SUMS` portions of the fixture are unaffected.
7. **Pipeline ordering — two real verification phases, not one.**
   **Revised from round 1** after `4h25`'s `B-V261-P1`: a single
   "verify-everything-before-any-publish" gate is impossible, because the
   OCI provenance record cannot exist before OCI push (see "Root cause").
   Two phases instead:
   - **Phase 1 (pre-OCI).** New job `release:assets-verify-pre-oci`,
     `stage: deploy` (same stage as its producers/consumers — a `test`-stage
     job cannot depend on `deploy`-stage artifacts, `4h25`'s other correct
     point), `needs: [agf:dist:release, rust:tooling:release]` (both
     `artifacts: true`). Runs `release-assets.sh verify-pre-oci dist
     "$VERSION"` — a new subcommand checking every mandatory file *except*
     `expected_oci()`'s OCI provenance line (the framework archives and the
     manifest-driven per-target Linux archives). Add `needs: [job:
     release:assets-verify-pre-oci]` to `agov:oci:release` (currently
     `needs: rust:tooling:release` only) — OCI push cannot start until
     everything that can be verified before it, is.
   - **Phase 2 (post-OCI, pre-publish).** `release:create`'s existing
     `tooling/rust/scripts/release-assets.sh verify dist "$VERSION"` call
     (full, unchanged subcommand name — now checks the complete set
     including OCI provenance, which exists by this point since
     `release:create` already `needs: agov:oci:release`) is **already
     correctly positioned** — this is exactly what stopped `v2.6.0`'s
     GitLab Release from being created on incomplete input. No new job
     needed here; phase 2 was never the bug, only phase 1's absence was.
8. **Real pre-tag acceptance path.** **New, addressing `4h25`'s
   `B-V261-P2`** — round 1's hand-created-empty-files dry run did not
   exercise real producers. Reuse the exact throwaway-branch pattern
   already proven today for the aarch64 necessity check
   (`docs/coordination/20260815T174820Z-governor-reviewer-jj5s.md`,
   branch `throwaway/v260-aarch64-necessity-20260815`): before requesting
   authority to cut `v2.6.1`, create a non-protected, never-merged
   throwaway branch at the fix's exact commit, manually trigger
   `rust:tooling:release`, `agf:dist:release`, and
   `release:assets-verify-pre-oci` (real producers, real `verify-pre-oci`
   call, no `CI_COMMIT_TAG` set so `agov:oci:release`/`release:create`
   never run and nothing is published), and publish the exact pipeline
   URL, job results, and assembled `dist/` file list in a coordination
   completion record. This is preflight evidence for the *next* plan (the
   one that actually requests `v2.6.1` authority), not evidence this plan
   itself needs to reach Gate 1 — recorded here so that later plan can
   cite a real run instead of inventing a new one.
9. **`release:assets-selftest`'s trigger paths.** **New, addressing
   `4h25`'s `B-V261-P3`** — add `tooling/rust/release-targets.txt` to the
   `&release_assets_selftest_paths` anchor list (currently
   `tooling/rust/scripts/release-assets.sh` and `.gitlab-ci.yml` only), so
   a manifest-only change (e.g. reinstating aarch64 later) schedules this
   job instead of silently skipping it.
10. **Namespace handling documentation for the retained `v2.6.0` failure.**
   `agf/docs/site/reference/release-artifacts.md` already accurately
   documents the current aarch64-not-built state — no change needed there.
   Instead, add the `v2.6.0` disposition directly to
   `CHANGELOG.md`/`agf/CHANGELOG.md`, under the entry that documents this
   fix: state plainly that `v2.6.0`
   is a permanently retained, immutable, failed publication attempt (tag
   exists, GitLab Release does not, OCI tag `agov:2.6.0` exists but is
   failed-publication residue and must not be treated as a usable release),
   per governor decision `spmj`. This is documentation of the historical
   fact, not a request to touch the tag, the OCI image, or either retained
   branch.

**Out of scope:**

- Cutting, tagging, or publishing `v2.6.1` (or any version). A follow-up
  plan, after this fix is merged to `main` and proven, will address the
  frozen-scope reconstruction needed to actually republish issues #76, #84,
  #85, #89 plus this fix — mirroring how the frozen-scope reconstruction
  *rule* (MR !1503) was its own separate, earlier Gate 1 from the v2.6.0
  *release-publication* plan (MR !1492) that consumed it.
- Deleting, moving, or retrying anything against the `v2.6.0` tag, its
  failed pipeline, or the `agov:2.6.0` OCI tag.
- Reinstating `aarch64-unknown-linux-musl` as a mandatory target. That
  remains excluded per governor decisions `jj5s`/`kpkk` pending sponsored
  runner capacity; this plan's manifest file makes reinstating it (when
  that happens) a one-line change to `tooling/rust/release-targets.txt`
  plus a corresponding governed decision, not a repeat of this incident.
- Any other CI job, release step, or documentation not named above.
- Product code, schema, or command-semantic changes.

## Files affected

- `tooling/rust/release-targets.txt` (new)
- `tooling/rust/scripts/release-assets.sh` (`expected()` split, new
  `verify-pre-oci` subcommand, `selftest` cross-contract check and fixture
  independence)
- `tooling/rust/scripts/agov-oci-image.sh` (`target_slug()` reads the shared
  manifest)
- `.gitlab-ci.yml` (build loop reads the manifest; new
  `release:assets-verify-pre-oci` job; `agov:oci:release`'s `needs:` gains
  that job; `release_assets_selftest_paths` anchor gains the manifest path)
- `CHANGELOG.md`, `agf/CHANGELOG.md` (entry documenting the fix and the
  `v2.6.0` disposition, once merged)

Five files/one new file — larger than round 1's four, but still a single
bounded purpose (the release-asset contract), not a broader release-process
rewrite. The growth from round 1 is entirely `4h25`'s findings, not scope
creep.

## Test and validation plan

**Revised from round 1** after `4h25`'s `B-V261-P4`: a valid second manifest
line makes both consumers agree, not disagree — that shape cannot be a
cross-contract mismatch test. Three distinct tests, each proving a different
thing:

1. **Cross-contract parser test (proves the *new* check in item 5 actually
   catches a parser bug).** Feed `expected()`'s manifest-reading logic a
   deliberately malformed line (e.g. a line with only a target, no slug
   column) in a scratch fixture directory — not the real
   `release-targets.txt`. Confirm both the independent parser and
   `expected()` itself reject it with a clear error, and that a
   *hand-constructed* divergence (temporarily hand-editing one of the two
   parsing implementations in a scratch commit to silently accept the
   malformed line while the other still rejects it) makes the
   cross-contract check fail with a message naming the mismatch. Revert the
   scratch edit before the real commit. This is the actual gate-self-test
   negative case, per `gate-self-test-rule.md` — it proves the *check*
   fires, not that `verify` fires (that's test 3, already proven, unchanged
   from before this incident).
2. **Manifest-growth consistency test (proves single-sourcing works, not
   that it detects drift).** Add a second, valid target line to a scratch
   copy of `release-targets.txt`, confirm `expected()`'s output grows to
   include that target's triplet, and confirm the cross-contract check
   still passes (both consumers read the same file, so they necessarily
   agree) — a passing result here, not a failing one, is correct and
   expected.
3. **Existing negative test, unchanged in shape.** The current
   `release-assets.sh selftest`'s missing-sidecar case (one mandatory file
   deliberately absent, `verify` must fail and name it) already works
   correctly today — re-run after the fixture-independence change (item 6)
   to confirm it still passes against the manifest-driven `expected()`.
- A **real pre-tag dry run**, not a synthetic one: item 8 above (the
  throwaway-branch preflight), run once before this MR's Gate 2, with its
  exact pipeline URL and job results cited as MR evidence — not merely
  described as a future step.
- Full `agov validate`, `agov dist validate`, `git diff --check`, and the
  existing MR pipeline (unaffected jobs must remain green — this change
  touches release-only paths, not product code).
- Manually confirm, by reading the changed `.gitlab-ci.yml` `needs:` blocks,
  that `agov:oci:release` now depends on `release:assets-verify-pre-oci`,
  that `release:create`'s existing `needs` (including `agov:oci:release`)
  is unchanged, and that no existing `needs` edge was removed.

## Acceptance criteria

1. `tooling/rust/release-targets.txt` is the single source read by the
   `rust:tooling:release` build loop, `release-assets.sh`'s `expected()`,
   and `agov-oci-image.sh`'s `target_slug()` — no hardcoded per-target
   filename or triple list remains in any of the three.
2. The cross-contract parser test (validation-plan item 1) fails on a
   deliberately introduced parser divergence and passes against the real
   manifest; the manifest-growth test (item 2) shows both consumers
   expanding consistently on a valid new line; the existing missing-sidecar
   test (item 3) still fails correctly, naming the missing file.
3. `agov:oci:release` cannot start until `release:assets-verify-pre-oci`
   succeeds; confirmed by reading the `needs:` graph, not merely asserted.
   `release:create`'s existing full `verify` call, gated by its existing
   `needs: agov:oci:release`, is confirmed unchanged and still correctly
   positioned.
4. `release-assets.sh expected 2.6.0` (or any version) no longer lists
   `agov-*-linux-aarch64.*` — matching the actual `rust:tooling:release`
   build output for any tag cut against current `main`.
5. The throwaway-branch preflight (validation-plan item, "real pre-tag dry
   run") has actually run once against real producers, with its exact
   pipeline URL and job results cited as MR evidence before Gate 2.
6. `CHANGELOG.md`/`agf/CHANGELOG.md` document both the fix and the `v2.6.0`
   failed-publication disposition.
7. No product, schema, or command-semantic change; no touch to any file
   outside the five listed in "Files affected".

## Downstream impact (per `downstream-impact-rule.md`, `tooling/**` trigger path)

Impact:
  Foundry:          Fixes the exact defect that stopped `v2.6.0`'s
                     publication; no behavior change to any already-tagged
                     release.
  Direct adopters:  ADDITIVE/PREVENTIVE — corrects an internal release-CI
                     script; no adopter-facing contract, schema, or CLI
                     behavior changes.
  Downstream:       None. This governs only this project's own release
                     pipeline.
  Migration:        None required.
  Evidence:         This incident (`pj71`, `m11h`, `spmj`, tag pipeline
                     `2762930639`, job `15917296084`), independently
                     re-verified above.

To be restated in the implementation commit's own three-circle statement per
that rule — this plan file itself is exempt (plan files are outside the
rule's trigger scope).

## Stop conditions

Stop and return to the governor if: the fix requires touching any file
outside the five listed in "Files affected"; the fix would reinstate
aarch64 as mandatory; the fix would delete, move, or retry anything against
the `v2.6.0` tag, branches, or OCI residue; the throwaway-branch preflight
in item 8 cannot be run without pushing anything to a protected ref or
publishing anything externally visible; or any of the three tests in "Test
and validation plan" cannot be made to fail/pass convincingly as designed
(a gate that doesn't actually gate).
