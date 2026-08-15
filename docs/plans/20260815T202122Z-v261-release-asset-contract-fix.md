---
id: v261-release-asset-contract-fix
status: pending-gate1
topic: v261-release-asset-contract-fix
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Not yet performed as of this plan's authoring — requested below, not
      silently assumed satisfied. Requesting review of factual accuracy against the
      cited evidence, whether the fix is proportionate to the incident (not
      over- or under-scoped), and whether the shared-manifest design in
      "Design" below actually eliminates the two-source-of-truth defect
      rather than adding a third parallel source.
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Not yet performed as of this plan's authoring. This change touches
      release-artifact publication (supply-chain relevant: which files a
      consumer can trust as mandatory) but adds no new dependency, no new
      network call, and no change to provenance/checksum content — only to
      which target set is considered mandatory and when the check runs.
      Requesting confirmation that no security-relevant surface (SSRF,
      secrets, provenance schema) is affected before Gate 1.
---

# Plan: fix the v2.6.0 release-asset contract gap (mandatory-target drift)

## Decision requested

Authorize an ordinary main-track fix (not a frozen-scope reconstruction) to
`tooling/rust/scripts/release-assets.sh` and `.gitlab-ci.yml`, correcting the
defect that caused `v2.6.0`'s tag pipeline to fail at `release:create` after
the tag was already pushed. This plan does **not** request authorization to
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
   works regardless of caller CWD. `agf-*` and `agov-oci-*` mandatory lines
   are untouched (they are not per-target).
4. **Non-self-referential cross-contract check.** Single-sourcing removes
   the possibility of the two lists drifting from each other by
   construction, but does not itself prove `expected()`'s manifest-parsing
   logic is correct. Add a check to `release-assets.sh selftest` that reads
   `tooling/rust/release-targets.txt` directly with separately-written
   parsing (not by calling `expected()`), independently computes the
   expected per-target filenames, and asserts the result equals
   `expected()`'s actual output. This is the check spmj's decision requires
   — it can catch a parsing bug in `expected()` even though both now read
   the same file.
5. **Fixture independence for `selftest`'s own positive/negative cases.**
   Per the incident's exact stated cause, replace the fixture-generation
   line `fixture_names="$(expected "$test_version")..."` with a **literal,
   hand-written filename list** for the mandatory-asset portion of the
   fixture (matching current `release-targets.txt` content at time of
   writing, with a comment stating this literal must be updated by hand
   whenever `release-targets.txt` changes). This is deliberately
   *not* derived from `expected()`, so a bug that breaks `expected()`
   cannot also break the fixture that is supposed to catch it. The
   Windows-preview and `SHA256SUMS` portions of the fixture are unaffected
   (they were never part of the drift).
6. **Pipeline ordering — verify before OCI/package/release publish.**
   Extract the `tooling/rust/scripts/release-assets.sh verify dist
   "$VERSION"` call out of `release:create`'s script into a new, earlier
   job `release:assets-verify` (stage `test`, after `rust:tooling:release`
   and `agf:dist:release` produce `dist/`). Add `needs: [job:
   release:assets-verify]` to **both** `agov:oci:release` (currently
   `needs: rust:tooling:release` only) and `release:create` (which keeps
   its existing `needs` plus this one). `release:create`'s own script keeps
   calling `verify` too, as a cheap, idempotent, defense-in-depth
   re-check — the fix is that OCI publication can no longer complete
   before the asset contract is confirmed.
7. **Namespace handling documentation for the retained `v2.6.0` failure.**
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
- `tooling/rust/scripts/release-assets.sh`
- `.gitlab-ci.yml`
- `CHANGELOG.md`, `agf/CHANGELOG.md` (entry documenting the fix and the
  `v2.6.0` disposition, once merged)

Four files/one new file — a bounded, single-purpose change, not a broader
release-process rewrite.

## Test and validation plan

- `tooling/rust/scripts/release-assets.sh selftest` — extended per items
  4-5 above, run locally and in CI (`release:assets-selftest`, which
  already triggers on every branch/MR change to this script or
  `.gitlab-ci.yml`).
- A deliberate **negative test before merging**, per `gate-self-test-rule.md`
  (this is a gate-introduction change — the new cross-contract check is
  itself a gate): temporarily add a second line to
  `tooling/rust/release-targets.txt` in a scratch commit, confirm the new
  cross-contract check in `selftest` fails with a clear message naming the
  mismatch, then revert the scratch line before the real commit. Record the
  transcript in the MR description per that rule's requirement.
- A **dry-run reproduction of the actual incident**: build a scratch `dist/`
  directory containing only the mandatory files the fixed `expected()`
  would require (x86_64 only, no aarch64), confirm `verify` now passes
  where it previously failed for exactly this input shape.
- Full `agov validate`, `agov dist validate`, `git diff --check`, and the
  existing MR pipeline (unaffected jobs must remain green — this change
  touches release-only paths, not product code).
- Manually confirm, by reading the changed `.gitlab-ci.yml` `needs:` blocks,
  that `agov:oci:release` and `release:create` both now depend on
  `release:assets-verify`, and that no existing `needs` edge was removed.

## Acceptance criteria

1. `tooling/rust/release-targets.txt` is the single source read by both the
   `rust:tooling:release` build loop and `release-assets.sh`'s `expected()`
   — no hardcoded per-target filename list remains in either location.
2. `release-assets.sh selftest`'s cross-contract check fails on a
   deliberately introduced mismatch (proven via the negative test above)
   and passes on the current, correct manifest.
3. `agov:oci:release` cannot start until `release:assets-verify` succeeds;
   confirmed by reading the `needs:` graph, not merely asserted.
4. `release-assets.sh expected 2.6.0` (or any version) no longer lists
   `agov-*-linux-aarch64.*` — matching the actual `rust:tooling:release`
   build output for any tag cut against current `main`.
5. `CHANGELOG.md`/`agf/CHANGELOG.md` document both the fix and the `v2.6.0`
   failed-publication disposition.
6. No product, schema, or command-semantic change; no touch to any file
   outside the four listed above.

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
outside the four listed above; the fix would reinstate aarch64 as mandatory;
the fix would delete, move, or retry anything against the `v2.6.0` tag,
branches, or OCI residue; or the negative self-test cannot be made to fail
convincingly (the gate would not actually gate).
