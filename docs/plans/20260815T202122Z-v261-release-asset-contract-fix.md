---
id: v261-release-asset-contract-fix
status: pending-gate1
topic: v261-release-asset-contract-fix
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Round 7 refresh: the round-5 citation (`b1jr`, at round-4 head
      `857cfcf5`) predated rounds 5-6's manifest-grammar hardening.
      Non-self contribution-auditor re-review at the current exact head
      landed CLEAR, zero findings:
      `20260816T002624Z-contribution-auditor-n2j3.md` — "the changes
      since `b1jr`'s earlier CLEAR review strengthen rather than relax
      the proposed controls" — exact head
      `81be35457a5d5c3ef5726b2f0fa0182c39b5654d`. Per `n2j3`'s own note,
      this citation-only correction does not require a further
      substantive contribution-auditor pass unless scope or reviewed
      substance changes.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Round 5 (still open, disposition unchanged from round 3's mistaken
      framing until this is genuinely resolved): coder-tester's `hegm`
      review correctly rejects treating the CI-automated `agov validate
      infosec` check as satisfying this consultation — "do not describe the
      configured CI actor's later mechanical execution as semantic review."
      An actual infosec-review-aid pass and/or a binding plan-specific
      governor waiver (mirroring the frozen-scope plan's `2r8r`/`mdth`
      pattern) is being requested separately; this entry remains
      `not-applicable` as a placeholder only because the schema requires a
      terminal value and none of `consulted`/`waived` is true yet.
      **This plan is not ready for Gate 1 while this entry is unresolved.**
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

**Round 7 (this revision): citation-only refresh.** Round 6's manifest-
grammar hardening was CLEAR per coder-tester
(`20260815T224601Z-coder-tester-sbh6.md`) and per a fresh non-self
contribution-auditor pass (`20260816T002624Z-contribution-auditor-n2j3.md`).
Frontmatter above updated to cite `n2j3` instead of the now two-rounds-stale
`b1jr`. No design change. infosec-sentinel remains the sole outstanding
prerequisite — a governor decision, not something further plan revision
can resolve.

**Round 6, per coder-tester's infosec-review-aid pass
`20260815T222421Z-coder-tester-4rrm.md` (requested via `fq2w`, MR !1592):
BLOCK, 1 blocking (`B-V261-SEC-AID-1`).** The pre-OCI ordering and the
production-path/no-override findings both passed. The one blocking finding:
the manifest file's values feed directly into build target arguments,
staging paths, and archive filenames, but the originally-specified parser
only rejected a missing slug column — not traversal characters, extra
fields, or duplicate entries. Fixed by specifying a hardened, uniformly-
enforced manifest grammar (exactly two safe path-component fields per
record, no duplicates, validated in full before any side effect) — see
"Scope" item 5, revised — with six new negative tests. Also made the
pre-OCI gate job's tag-only rule boundary and non-optional `needs` edges
explicit per the same review's implementation note — see "Scope" item 7.
This is a genuine security-hardening fix, not a wording change; still
awaiting the separate governor decision on the plan-specific infosec
consultation waiver this aid-review's own findings are evidence for
(frontmatter unchanged this round — `not-applicable` remains an honest
placeholder, not a claim the waiver has been granted).

**Round 5, per coder-tester's exact-head review
`20260815T214558Z-coder-tester-hegm.md` (MR !1587): BLOCK, 1 blocking
(`B-V261-R4-1`).** `B-V261-R3-1` and `A-V261-R3-1` both confirmed `pass` —
no changes needed to the mechanical design. `B-V261-R4-1` (the consultation-
metadata finding) is **partially resolved this round**:

- The distinct contribution-auditor-hat review requested last round landed
  CLEAR, zero findings (`20260815T214557Z-contribution-auditor-b1jr.md`,
  sibling record `b1jr`, same MR !1587). Frontmatter above updated to
  `consulted`, citing that real, non-self review by exact head — not
  asserted before it existed.
- infosec-sentinel remains genuinely unresolved. `hegm` correctly rejects
  treating the CI-automated `agov validate infosec` check's later mechanical
  execution as satisfying semantic consultation. Since infosec-sentinel has
  no configured human or AI player (`played_by: ci` only, per
  `agf/agents/MANIFEST.yaml`) — unlike contribution-auditor, there is no
  "route to Codex under that hat" available for this role — the only
  established path is the frozen-scope plan's precedent (`2r8r`/`mdth`): a
  coder-tester infosec-review-aid pass, explicitly not itself claiming to be
  infosec-sentinel, followed by a governor decision on a bounded waiver.
  That aid-review request is filed separately, accompanying this push. This
  plan remains **not ready for Gate 1** until it resolves.

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

**Round 4, per coder-tester's exact-head review
`20260815T211813Z-coder-tester-adjg.md` (MR !1584): BLOCK, 2 blocking + 1
advisory.**

- `B-V261-R3-1` (critical): round 3's `release:assets-verify-preview` needed
  only `rust:tooling:preview` and ran a narrow `verify-targets` check —
  proving real per-target archives, but not the exact assembled pre-OCI
  set `spmj`'s binding decision requires (mandatory manifest, checksum/
  provenance, exact artifact set). `agf:dist:preview` already runs on this
  MR too (same `.gitlab-ci.yml`-change trigger). Fixed by needing **both**
  preview jobs and running the **same** `verify-pre-oci` subcommand the tag
  pipeline uses — see "Scope" item 8, revised. This also removes the
  `verify-targets` subcommand entirely: one pre-OCI check, exercised
  against two different producer contexts (preview, real), is simpler and
  more faithful than two different checks.
- `B-V261-R2-2`/`B-V261-R3-2` (material, **still open — not resolved in
  this revision**): `adjg` is right that the frontmatter's `not-applicable`
  reclassification was a plan author narrowing another role's project-level
  jurisdiction by interpretation — `AGENTS.md` states contribution-auditor
  "reviews plans and proposals," which does cover this plan. The governor
  was asked directly which resolution path to take (route to Codex under
  the contribution-auditor hat, the governor reviews it directly, or an
  explicit bounded waiver) and chose routing to Codex, mirroring the
  precedent already used for MR !1492 (`3hgr`). That request is filed as a
  **separate** coordination record accompanying this push, addressed to
  coder-tester specifically under the contribution-auditor mandate (a
  distinct ask from the coder-tester technical reviews already done). The
  frontmatter below is **left unchanged from round 3** (still
  `not-applicable`, now known to be substantively contested rather than
  correct) rather than guessed at again — per `adjg`'s own instruction, "do
  not request Gate 1 while the plan states its own prerequisites are
  outstanding," and re-wording the status to a different unsupported claim
  a fourth time would repeat exactly the pattern this finding objects to.
  It will be corrected to `consulted`, with a real citation, once that
  routed review actually lands. Fixing `B-V261-R3-1` and `A-V261-R3-1` here
  so the mechanical design work isn't blocked on a decision that's still
  pending — this round is **not** a Gate 1 request.
- `A-V261-R3-1` (advisory): the malformed-manifest selftest case for
  `agov-oci-image.sh selftest` now uses an explicit private-function-
  parameter seam (not an environment variable) rather than editing the
  canonical file — see "Scope" item 4, revised.

**Round 3, per coder-tester's exact-head review
`20260815T205646Z-coder-tester-mqka.md` (MR !1582): BLOCK, 3 blocking + 1
advisory.** All four addressed:

- `B-V261-R2-1` (critical): confirmed by reading `.gitlab-ci.yml` directly —
  both `rust:tooling:release` and `agf:dist:release` have `rules: - if:
  $CI_COMMIT_TAG` only, so round 2's throwaway-branch design (manually
  triggering them on a branch) was never possible; a tag-only job cannot run
  on a branch pipeline regardless of branch name or manual trigger. Rather
  than adding a new rule/variable contract to expose tag-only jobs on a
  named branch (the resolution shape `mqka` sketched, which would itself
  need its own negative test proving it can't leak into publish jobs),
  found that `agf:dist:preview` and `rust:tooling:preview` **already exist,
  already run automatically on this fix's own MR** (both match on
  `.gitlab-ci.yml` changes, which this fix necessarily makes), and already
  produce archives under the exact same filenames the tag pipeline
  produces. Dropped the throwaway-branch design entirely — see "Scope" item
  8, replaced.
- `B-V261-R2-2` (material): re-examined both roles' actual jurisdiction
  instead of re-wording the same contradiction a third time. infosec-
  sentinel is CI-automated (`played_by: ci`), not a consultation gate at
  all; contribution-auditor's real jurisdiction (contribution proposals,
  upstream framework diffs) does not cover this plan. Both reclassified
  `not-applicable` in the frontmatter above, with reasoning — not asserted
  as `consulted` when no consultation occurred.
- `B-V261-R2-3` (material): "Files affected" always listed six paths;
  fixed the prose, acceptance criterion 7, and stop conditions to say six
  consistently, and added the preview-verify job's `.gitlab-ci.yml` change
  to the enumerated scope.
- `A-V261-R2-1` (advisory): `agov-oci-image.sh`'s manifest read is now
  explicitly CWD-independent (resolved relative to the script's own
  directory, matching `release-assets.sh`); added a new `agov-oci-image.sh
  selftest` subcommand exercising `target_slug()` against every manifest
  entry plus a malformed line and the still-special-cased Windows-target
  rejection, wired into CI — see "Scope" items 4 and 9.

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

**In scope** (the six files enumerated in "Files affected" below, ordinary
main-track change per `release-via-merge-request-rule.md`'s standard
workflow — no frozen-scope exception needed, this is exactly the kind of
individually-approved change that belongs on `main` directly):

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
3. **`release-assets.sh`'s `expected()` function, decomposed into layered
   functions** (**revised from round 3**: dropped the standalone
   `verify-targets` subcommand per `B-V261-R3-1` — a per-target-only check
   doesn't prove the exact assembled pre-OCI set `spmj`'s binding decision
   requires; one `verify-pre-oci` check, run against two different producer
   contexts, replaces it): remove the hardcoded
   `agov-${version}-linux-x86_64...`/`agov-${version}-linux-aarch64...`
   lines from `expected()`. Add `expected_targets()`: an internal helper
   (not its own subcommand) generating only the manifest-driven per-target
   archive/checksum/provenance triplets, reading
   `tooling/rust/release-targets.txt` resolved relative to the script's own
   directory (`$(dirname "$0")/release-targets.txt`) so it works regardless
   of caller CWD. `expected_pre_oci()`: the literal `agf-*` mandatory lines
   plus `expected_targets()`'s output — this is the function
   `verify-pre-oci` (item 8) checks against. `expected()` becomes
   `expected_pre_oci()` plus `expected_oci()` (the OCI provenance line,
   unchanged in content, isolated per item 7 below). Two subcommands only:
   `verify-pre-oci` (checks `expected_pre_oci()`, used by both the tag-
   pipeline gate and the preview-pipeline dry run) and the existing
   `verify` (unchanged, checks the full `expected()`).
4. **`agov-oci-image.sh`'s `target_slug()`**: read the same
   `tooling/rust/release-targets.txt`, resolved relative to the script's own
   directory (matching item 3's CWD-independence for `release-assets.sh`),
   instead of its own hardcoded `x86_64-unknown-linux-musl`/
   `aarch64-unknown-linux-musl` case arms. Third and last consumer of the
   shared manifest; no hardcoded target list remains anywhere in the tree
   after this item and items 2-3. The existing explicit rejection of
   `x86_64-pc-windows-gnu` (with its specific "not supported by this Linux
   OCI-image path" message) is preserved as a special case checked before
   the manifest lookup — the manifest only ever lists Linux targets, so this
   is not a manifest-driven entry. **New, addressing `mqka`'s
   `A-V261-R2-1`**: add an `agov-oci-image.sh selftest` subcommand (no
   network calls, no image build) that calls `target_slug()` once per
   `release-targets.txt` entry and asserts the expected slug, calls it with
   `x86_64-pc-windows-gnu` and asserts the specific rejection message, and
   calls it with a fully unknown target and a deliberately malformed
   manifest line and asserts a clear error in both cases. **Revised from
   round 3** after `adjg`'s `A-V261-R3-1`: the malformed-manifest case
   needs an explicit injection seam rather than editing the canonical file
   in place — a private function parameter, not an environment variable
   (no seam that could leak into a real build path at all). Split
   `target_slug()` into two: `target_slug_from(manifest, target)` — the
   actual lookup logic, taking the manifest path as its first argument —
   and the public `target_slug(target)` — the only form called by `build`/
   `push`/every production code path — which always calls `target_slug_from
   "$(dirname "$0")/release-targets.txt" "$1"`, hardcoded, no override
   possible. `selftest` calls `target_slug_from` directly with a scratch
   fixture path for the malformed-line case, and calls the public
   `target_slug()` (canonical path, no override) for the real-manifest and
   Windows-special-case assertions. No environment variable or global
   state is introduced; the public entry point's canonical-path resolution
   has no conditional branch to get wrong.
5. **Non-self-referential cross-contract check, with a hardened manifest
   grammar.** Single-sourcing removes the possibility of the three lists
   (build loop, `expected()`, `target_slug()`) drifting from *each other*
   by construction, but does not itself prove any one consumer's
   manifest-parsing logic is correct — or that the manifest's *content* is
   safe to use unvalidated in build target arguments, source paths,
   staging directory names, archive names, and `dist/` output paths.
   **Round 6, per coder-tester's infosec-review-aid finding
   `B-V261-SEC-AID-1`** (`20260815T222421Z-coder-tester-4rrm.md`,
   requested via `fq2w`): the original spec ("reject a malformed line —
   missing slug column") only caught one shape of bad input, not the
   general case — a manifest line containing `/`, `..`, extra fields, or a
   duplicate target/slug could escape the intended artifact namespace or
   make producer and verifier disagree, even though the parser
   "succeeds." One canonical manifest grammar, enforced identically by
   every consumer (the build loop, `expected_targets()`, and
   `target_slug_from()`), validated **in full, before any build, staging,
   archive-write, or publication side effect begins** — not line-by-line
   as each is consumed mid-build:
   - Exactly two non-empty, whitespace-separated fields per data record
     (`<target> <slug>`); blank lines and `#`-prefixed comment lines are
     the only other permitted content — anything else (wrong field count,
     stray content) is a parse error.
   - Both `target` and `slug` must be a single safe path-component token:
     reject any value containing `/`, `\`, `..`, leading `-`, whitespace,
     or control characters. This is deliberately stricter than "a valid
     Rust target triple" or "a valid filename slug" individually — it's
     the narrowest shape that is safe to interpolate into a path or
     archive name without further escaping.
   - No duplicate `target` values and no duplicate `slug` values across
     the whole file — a duplicate target is ambiguous (which line wins?);
     a duplicate slug means two different targets would produce
     colliding archive filenames.
   - Validate the **complete** file against all of the above before the
     build loop processes its first target, before `expected_targets()`
     returns a single filename, and before `target_slug_from()` resolves
     a single lookup — a manifest that fails validation must produce zero
     partial output (no half-written `dist/` files, no half-populated
     archive), not fail partway through a target loop.
   Add a check to `release-assets.sh selftest` that reads
   `tooling/rust/release-targets.txt` directly with separately-written
   parsing (e.g. `awk`, not `expected()`'s own heredoc-based logic),
   independently computes the expected per-target filenames, and asserts
   the result equals `expected()`'s actual output — this remains the
   genuinely independent check `spmj`'s decision requires, now exercising
   the hardened grammar above rather than only the missing-slug-column
   case.
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
     **`rules: - if: $CI_COMMIT_TAG`, made explicit per `4rrm`'s
     resolution condition** — the exact same tag-only rule boundary as its
     producers (`agf:dist:release`, `rust:tooling:release`) and consumer
     (`agov:oci:release`), so GitLab either instantiates the whole gate
     chain on a tag pipeline or the job simply doesn't exist — never a
     partial graph. `stage: deploy` (same stage as its producers/consumers
     — a `test`-stage job cannot depend on `deploy`-stage artifacts,
     `4h25`'s other correct point), `needs: [agf:dist:release,
     rust:tooling:release]`, both `artifacts: true` and **non-optional** —
     no `optional: true` on either edge, so a missing producer fails the
     pipeline graph rather than silently skipping the gate. Runs
     `release-assets.sh verify-pre-oci dist
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
8. **Real pre-tag acceptance path, using existing preview jobs — no
   throwaway branch.** **Revised from round 2** after `mqka`'s
   `B-V261-R2-1`: `rust:tooling:release` and `agf:dist:release` are
   `rules: - if: $CI_COMMIT_TAG` only (confirmed by reading
   `.gitlab-ci.yml` directly) — neither can run on a branch pipeline by any
   manual trigger, so round 2's throwaway-branch design was never
   achievable without also changing those jobs' rules, which would need its
   own leak-proof contract and negative test. Dropped that shape entirely.
   `agf:dist:preview` (line 416) and `rust:tooling:preview` (line 1152)
   **already exist** for exactly this purpose ("build preview archives for
   reviewers" / preview Rust tooling archives), already run on
   `merge_request_event` pipelines whose changes touch `tooling/rust/**/*`
   or `.gitlab-ci.yml`, and already write archives under the identical
   filenames the tag pipeline's real producers write — same `agov dist
   build` / `.rust-tooling-package` script logic, version-named not
   tag-named. This fix necessarily touches `.gitlab-ci.yml`, so both preview
   jobs run automatically on this fix's own MR pipeline — no throwaway
   branch, no new CI rule exposing a tag-only job, no separate later
   rehearsal needed for the contract-fix itself. **Revised again from
   round 3** after `adjg`'s `B-V261-R3-1`: needing only
   `rust:tooling:preview` and checking only per-target files proved real
   archives but not the *exact assembled pre-OCI set* — `spmj`'s binding
   decision requires the mandatory manifest, checksum/provenance, and exact
   artifact set, not a target-only subset. New job
   `release:assets-verify-preview`, `stage: test`, `needs: [{job:
   agf:dist:preview, artifacts: true}, {job: rust:tooling:preview,
   artifacts: true}]` (both mandatory — this fix touches both
   `tooling/rust/**/*` and `.gitlab-ci.yml`, so both producers always run on
   its own MR pipeline), runs the **same** `release-assets.sh verify-pre-oci
   dist "$VERSION"` subcommand item 7's tag-pipeline gate uses — one check,
   two producer contexts (preview archives here; real `release:*` archives
   at tag time), not two different checks that could drift from each
   other. Cite this job's real pipeline URL and output as this MR's own
   Gate 2 evidence — see "Test and validation plan". Rehearsing the full
   OCI-publish path end-to-end remains correctly out of scope, deferred to
   the separate `v2.6.1` cut-and-tag plan (see "Out of scope"), which is
   the only step that
   actually needs a real tag pipeline.
9. **Selftest trigger paths.** **New, addressing `4h25`'s `B-V261-P3`** —
   add `tooling/rust/release-targets.txt` to the
   `&release_assets_selftest_paths` anchor list (currently
   `tooling/rust/scripts/release-assets.sh` and `.gitlab-ci.yml` only), so
   a manifest-only change (e.g. reinstating aarch64 later) schedules
   `release:assets-selftest` instead of silently skipping it. Add a new
   `agov-oci-image:selftest` job (`stage: test`, mirroring
   `release:assets-selftest`'s shape) running item 4's new `agov-oci-image.sh
   selftest` subcommand, triggered on changes to
   `tooling/rust/scripts/agov-oci-image.sh`, `tooling/rust/release-targets.txt`,
   and `.gitlab-ci.yml` — the manifest path is in both anchors from the
   start, so this job's own trigger cannot repeat `B-V261-P3`'s gap.
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

Six files total, including one new file:

- `tooling/rust/release-targets.txt` (new)
- `tooling/rust/scripts/release-assets.sh` (`expected()` decomposed into
  `expected_targets()`/`expected_pre_oci()`/`expected_oci()`; new
  `verify-pre-oci` subcommand, used by both the tag gate and the preview
  dry run; `selftest` cross-contract check and fixture independence)
- `tooling/rust/scripts/agov-oci-image.sh` (`target_slug()` split into
  `target_slug_from(manifest, target)` and a public `target_slug(target)`
  with a hardcoded canonical path, no override; new `selftest` subcommand)
- `.gitlab-ci.yml` (build loop reads the manifest; new
  `release:assets-verify-pre-oci` job gating `agov:oci:release`; new
  `release:assets-verify-preview` job on this fix's own MR; new
  `agov-oci-image:selftest` job; `release_assets_selftest_paths` anchor
  gains the manifest path)
- `CHANGELOG.md`, `agf/CHANGELOG.md` (entry documenting the fix and the
  `v2.6.0` disposition, once merged)

Six files/one new file — larger than round 1's four and round 2's five, but
still a single bounded purpose (the release-asset contract), not a broader
release-process rewrite. The growth across rounds is entirely `4h25`'s and
`mqka`'s findings, not scope creep.

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
4. **`agov-oci-image.sh selftest` (new, addressing `A-V261-R2-1`).** Every
   `release-targets.txt` entry resolves to its correct slug via
   `target_slug()`; the Windows-target special case still produces its
   specific rejection message; a fully unknown target and a malformed
   scratch manifest line both produce clear errors. No network call, no
   image build.
5. **Manifest-grammar negative tests, added per `4rrm`'s
   `B-V261-SEC-AID-1`.** Each case is a scratch manifest fixture, never
   the real `release-targets.txt`, and each must fail **before** any
   build/staging/archive-write side effect — the test asserts zero
   partial output, not just a non-zero exit code:
   - A traversal-like value (`../etc`, `foo/bar`) in either field.
   - A field containing a control character or leading `-`.
   - A record with three fields instead of two.
   - A duplicate `target` value across two lines.
   - A duplicate `slug` value across two lines (different targets).
   - A well-formed file (all records pass the grammar) still succeeds
     unchanged — this and the six failure cases above together prove the
     grammar is exactly as strict as specified, not stricter (rejecting
     valid input) or looser (missing a case).
- A **real pre-tag dry run using existing producers, not a throwaway
  branch**: item 8's `release:assets-verify-preview` job, which needs both
  `agf:dist:preview` and `rust:tooling:preview` and runs the exact same
  `verify-pre-oci` check the tag pipeline uses, against real archives from
  this MR's own pipeline (it touches `.gitlab-ci.yml` and both producers'
  trigger paths) — cited with its exact pipeline URL and job output as this
  MR's Gate 2 evidence, not merely described as a future step, and not
  requiring any separate branch or manual pipeline trigger.
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
2a. **Added per `4rrm`'s `B-V261-SEC-AID-1`.** Every manifest-grammar
    negative test (validation-plan item 5) fails before any build/staging/
    archive-write side effect occurs — traversal characters, control
    characters, leading `-`, wrong field count, and both duplicate forms
    are all rejected; a well-formed file is unaffected.
3. `agov:oci:release` cannot start until `release:assets-verify-pre-oci`
   succeeds; confirmed by reading the `needs:` graph, not merely asserted.
   `release:create`'s existing full `verify` call, gated by its existing
   `needs: agov:oci:release`, is confirmed unchanged and still correctly
   positioned.
4. `release-assets.sh expected 2.6.0` (or any version) no longer lists
   `agov-*-linux-aarch64.*` — matching the actual `rust:tooling:release`
   build output for any tag cut against current `main`.
5. The `release:assets-verify-preview` job (validation-plan item, "real
   pre-tag dry run using existing producers") has actually run once on this
   MR's own pipeline, needing both `agf:dist:preview` and
   `rust:tooling:preview` and running the same `verify-pre-oci` check the
   tag pipeline uses, with its exact pipeline URL and job results cited as
   MR evidence before Gate 2 — no throwaway branch, no manual pipeline
   trigger.
6. `CHANGELOG.md`/`agf/CHANGELOG.md` document both the fix and the `v2.6.0`
   failed-publication disposition.
7. No product, schema, or command-semantic change; no touch to any file
   outside the six listed in "Files affected".
8. `agov-oci-image.sh selftest` (validation-plan item 4) passes for every
   real manifest entry and the Windows special case, and fails clearly for
   an unknown target and a malformed scratch manifest line.

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
outside the six listed in "Files affected"; the fix would reinstate
aarch64 as mandatory; the fix would delete, move, or retry anything against
the `v2.6.0` tag, branches, or OCI residue; the `release:assets-verify-preview`
job in item 8 cannot run without a rule change to any tag-gated job, a
throwaway branch, or publishing anything externally visible; or any of the
tests in "Test and validation plan" cannot be made to fail/pass convincingly
as designed (a gate that doesn't actually gate).
