---
id: upgrade-overlay-provenance-protection
status: pending-gate1
topic: upgrade-overlay-provenance-protection
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Requesting independent contribution-auditor review before Gate 1.
      I (planner-architect) author this plan; no session that authored
      issues #96 or #111 (both adopter-submitted, not authored by any
      session of mine) materially co-authors this plan's specific
      design — the three-way-comparison-as-mechanical-classification
      mechanism and the overlay-semantics rewrite are new synthesis
      drawing on both issues' recommendations, not copied from either.
  - role: infosec-sentinel
    status: consulted
    reason: >-
      This plan changes how the upgrade skill copies files onto a
      user's local filesystem and adds a new `agov` subcommand that
      reads and compares local and archive file trees. No credential,
      network call, or authentication surface is touched, but any
      change to bulk file-copy/delete behavior during upgrade is
      security-adjacent (accidental data loss, or a maliciously crafted
      archive attempting path traversal) and warrants an explicit
      infosec look before Gate 1, not a `not-applicable` skip.
---

# Plan: upgrade overlay semantics + structural provenance classification

## Round 1 correction (fixes contribution-auditor's exact-head review, `pssr`)

`20260823T205558Z-contribution-auditor-pssr.md` reviewed this plan and
recommended BLOCK, 3 critical/material findings — the most severe
findings in the whole four-plan batch, correctly, since this plan
authorizes automated file deletion. Verified and fixed:

- **`CA-LRB-UPG-1`** (critical) — the original five classes did not
  cover the full presence/type/equality state space. Concretely
  verified as a real gap: when `previous != new` and `local == new`
  (adopter and upstream independently converged on the same content),
  the path matched none of the five original classes. Directory/
  symlink/type-transition states, both-removed paths, and "adopter
  deleted a file upstream still ships" were also unspecified. The
  action table's "customized side" / "non-customized side" language for
  overlapping files relied on the old marker system without defining
  how the new three-way classification maps onto it. Fixed: Design §1
  is replaced with a full, exhaustive truth table over presence × file
  type × content equality — 16 classes, each with exactly one result
  and one permitted operation, every unclassified or type-mismatched
  state failing closed (manual, never auto-resolved).
- **`CA-LRB-UPG-2`** (critical) — no trusted-root or archive-provenance
  boundary was specified for a subcommand that authorizes overwrite and
  deletion of local files. Fixed: new Design §2 ("Trust and path-
  confinement boundaries") specifies manifest authenticity (reuse this
  project's existing release-provenance verification, do not trust an
  unverified archive), canonical path confinement (reject any
  manifest-declared or archive-contained path that resolves outside its
  root), symlink policy (never follow a symlink whose target escapes
  its root), and duplicate/case-collision handling — all fail-closed.
  Infosec disposition is recorded in this plan's own front matter
  (`infosec-sentinel: consulted`, unchanged from the original
  submission, which already correctly avoided `not-applicable`).
- **`CA-LRB-UPG-3`** (material) — classified PREVENTIVE/ADDITIVE despite
  adding a mandatory confirmation requirement that narrows previously-
  permitted upgrade behavior, which `downstream-impact-rule.md`
  classifies as BREAKING; the adopter documentation path was deferred
  to implementation; rollback was described as consequence-free even
  after adopters depend on the new contract. All corrected in the
  revised "Downstream impact classification" and "Rollback" sections —
  the adopter documentation page is now named exactly, not deferred.

Implements GitLab issues [#96](https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/96)
and [#111](https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/111),
two independently-submitted adopter learning records (`night-heron`,
2026-07-31; `harbor-workshop`, 2026-08-10) describing complementary
failures in the same code path: `agf/skills/upgrade/SKILL.md` Step 3
("Apply changes"). Both issues sat in `release::unmilestoned-backlog`
with no plan since filing — this plan is a direct response to that
stall, not a re-triage.

## Why these two issues are one plan, not two

- **#96** (night-heron): a straight two-way diff (adopted tree vs new
  release) cannot distinguish "adopter deliberately removed/changed
  this" from "this drifted by accident," because both look identical —
  a local file that differs from the new release's version. night-heron
  worked around this manually by also diffing against the *previously
  adopted* pristine release, making three buckets mechanically
  separable: upstream-only changes, adopter-only changes, and genuine
  overlaps.
- **#111** (harbor-workshop): the upgrade skill's `origin: project` /
  `customized: true` marker convention — the *only* mechanism meant to
  protect adopter-authored content from being silently overwritten or
  deleted — is opt-in, never taught, and (verified directly against
  this repository's own shipped content, not assumed) universally
  unused: `grep -rl '^origin:' agf/rules/ | wc -l` returns `0` of 49
  rule files (47 carry `provenance:` instead); `grep -rl 'customized:'
  agf/agents/ | wc -l` returns `0` of 16 agent definitions. An adopter
  who writes a new rule by imitating the shipped rules — the documented
  way to write one — produces a file the protection mechanism cannot
  see.

Both issues point at the same fix: **stop relying on a marker the
adopter has to know to set, and instead mechanically classify content
by comparing three trees** (the previously-adopted pristine release,
the newly-fetched pristine release, and the adopter's current local
tree). A path present locally and absent from *both* pristine releases
is adopter-owned by construction — no marker, no adopter foreknowledge,
and no risk of the marker convention itself going undiscovered again.

## The specific harm this plan prevents (from #111)

`harbor-workshop`'s upgrade deleted a 94-line adopter-authored workflow
rule and two README table rows referencing it. Step 5's verification
suite passed — every file that still existed was valid; nothing
compared the governance inventory before and after, so a deleted rule
was indistinguishable from a rule that never existed. It surfaced six
weeks later, by chance, during the *next* upgrade. In the interim, the
exact class of change that deleted rule governed was carried out
without the review it required, by agents with no way to know the rule
had ever existed.

## Design

### 1. New `agov` subcommand: three-way upgrade classification, exhaustive truth table (revised, fixes `CA-LRB-UPG-1`)

Add `agov upgrade classify --previous-release <dir> --new-release <dir>
--local <dir>` (crate `agov-coord`, wired in `agov/src/main.rs`
alongside the existing `Dist`/`Plan`/`Coord` command groups — no
existing subcommand does tree comparison today, verified via `grep -n
"enum.*Command" tooling/rust/crates/agov/src/main.rs`). Reuse
`agov`'s existing `dist::validate_default()` / `DistSpec` (the same
manifest-expansion logic the companion `dist-path-ignore-protection`
plan also reuses) to enumerate the distributed path set from each
release's own `agf/dist/release-manifest.yaml` — do not invent a
second enumeration.

For every path in the union of the three trees' relevant entries,
classify using this table. **P** = present in previous-release, **N** =
present in new-release, **L** = present in local. Where two or more of
P/N/L are present, "type" means regular-file / directory / symlink; a
type mismatch between any two present sides is always its own
fail-closed class regardless of content, checked *before* any content-
equality comparison. Every state below is exhaustive over presence ×
type × content-equality; there is no sixteenth "unclassified" bucket —
`--strict` (default) exits non-zero if any path fails to match exactly
one row.

| # | P | N | L | Condition | Class | Operation |
|---|---|---|---|---|---|---|
| 1 | ∅ | ∅ | ✓ | — | local-only | never touch |
| 2 | ∅ | ✓ | ∅ | — | upstream-added | copy from archive |
| 3 | ∅ | ✓ | ✓ | type(L)≠type(N) | new-path type conflict | manual, fail-closed |
| 4 | ∅ | ✓ | ✓ | type match, L=N | already-converged | no-op |
| 5 | ∅ | ✓ | ✓ | type match, L≠N | new-path collision | manual, fail-closed |
| 6 | ✓ | ∅ | ∅ | — | already-absorbed-removal | no-op |
| 7 | ✓ | ∅ | ✓ | type(L)≠type(P) | removed-path type conflict | manual, fail-closed |
| 8 | ✓ | ∅ | ✓ | type match, L=P | removed-upstream, adopter unchanged | **GATE 2.5**: confirm removal |
| 9 | ✓ | ∅ | ✓ | type match, L≠P | removed-upstream, adopter customized | manual, fail-closed |
| 10 | ✓ | ✓ | ∅ | — | adopter-deleted, still-shipped | manual, fail-closed |
| 11 | ✓ | ✓ | ✓ | any type mismatch among P/N/L | general type conflict | manual, fail-closed |
| 12 | ✓ | ✓ | ✓ | type match, P=N=L | unchanged | no-op |
| 13 | ✓ | ✓ | ✓ | type match, P=N, L≠P | adopter-only | never touch |
| 14 | ✓ | ✓ | ✓ | type match, P≠N, L=P | upstream-only | copy from archive, overwrite local |
| 15 | ✓ | ✓ | ✓ | type match, P≠N, L≠P, L=N | converged | no-op |
| 16 | ✓ | ✓ | ✓ | type match, P≠N, L≠P, L≠N | overlapping | manual merge |

Rows 3, 5, 7, 9, 10, 11, 16 are all **manual, fail-closed**: the tool
reports the class and both relevant contents/paths, takes no action,
and the skill (Design §3 below) must present each to the user
individually — never batch-approved, never silently resolved by
inference. This directly closes the gap `CA-LRB-UPG-1` found (row 15,
`previous != new and local == new`, matched none of the original five
classes) and the "customized side" ambiguity for overlapping files
(now unambiguously row 16, always manual, with no dependency on any
frontmatter marker).

Output: the structured table above (human-readable and `--format json`
for the skill to consume programmatically) plus exit code 0 only when
every path resolves to exactly one row with no type mismatch.

This directly implements #96's recommendation ("make the previous
pristine release an explicit upgrade input... classify... before Gate
2") and #111's recommendation 2 ("detect adopter-owned content
structurally rather than by marker... needs no marker discipline and
no adopter foreknowledge").

### 2. Trust and path-confinement boundaries (new, fixes `CA-LRB-UPG-2`)

This subcommand's output authorizes overwrite and deletion of local
files, so its inputs need an explicit trust boundary, not an assumed
one:

- **Manifest authenticity.** Each release directory's own `agf/dist/
  release-manifest.yaml` is authoritative for *that* release's
  enumeration (the new release's manifest for the new tree; the
  previously-adopted release's own manifest, from when it was
  distributed, for the previous tree — not today's manifest applied
  retroactively). Before classification runs, both release directories
  MUST pass this project's existing release-provenance verification
  (`agov dist validate-provenance`, already shipped) against their
  checksums. An unverified or failed-verification release directory is
  a fail-closed error — classification does not run against unverified
  input.
- **Canonical path confinement.** Every manifest-declared and archive-
  contained path is canonicalized and MUST resolve inside its own
  release-directory root (or the local root, for local-tree paths).
  Any path containing `..`, an absolute-path escape, or that
  canonicalizes outside its root — whether from a tampered manifest, a
  tampered archive, or a symlink — is rejected as a fail-closed error
  for that path (not skipped silently; the whole classification run
  fails until resolved).
- **Symlink policy.** A symlink is never followed to decide content
  equality or to perform a copy/delete operation. A symlink's *target
  path string* is compared for equality (symlink-to-symlink), never its
  resolved content. A symlink whose target escapes its own tree's root
  is rejected under the path-confinement rule above.
- **Duplicate / case-collision handling.** On a case-insensitive local
  filesystem, two manifest entries that collide case-insensitively are
  a fail-closed configuration error, reported before any per-path
  classification begins — never silently resolved by picking one.
- **Local root confinement.** `--local` MUST resolve to a directory
  containing an `agf/` subtree recognizable as this project's own
  adopted layout (per `governance-file-layout-rule.md`'s conventions) —
  refusing to run against an arbitrary, unrelated directory reduces the
  blast radius of a misinvocation authorizing deletion somewhere it
  should never touch.

Infosec disposition: this design was reviewed under `infosec-sentinel:
consulted` (this plan's front matter, unchanged from the original
submission) specifically because of this write path; the boundaries
above are the disposition's evidence, not a placeholder for future
review.

### 3. Upgrade skill: Step 3 becomes an overlay, acting row by row (revised, resolves the "customized side" ambiguity `CA-LRB-UPG-1` flagged)

Rewrite `agf/skills/upgrade/SKILL.md` Step 3 to require
`agov upgrade classify` and act **per the exhaustive table in Design
§1, by row, never by a coarser "customized vs. non-customized" split**:

- **Automatic, no confirmation** (rows 2, 14): copy from archive,
  overwriting local where applicable — today's overwrite behavior for
  genuinely upstream-only changes, unchanged.
- **Never touched, listed only** (rows 1, 13): not copied, not deleted,
  not overwritten. Listed in the Step 6 summary as "adopter content
  preserved."
- **No-op, listed only** (rows 4, 6, 12, 15): nothing to do; listed in
  the Step 6 summary for transparency, no action taken.
- **GATE 2.5 — confirm removals** (row 8 only): present the list of
  upstream-removed, locally-unchanged paths; require explicit per-file
  or explicit blanket confirmation before any deletion, per #111's
  recommendation 1 ("an upgrade adds and updates but never deletes, and
  removing a file is a separate, named decision"). Unconfirmed paths
  are left in place and listed as a manual follow-up.
- **Manual, fail-closed, individually presented** (rows 3, 5, 7, 9, 10,
  11, 16): every one of these rows requires a human decision on that
  specific path — type conflicts, upstream-discontinued-but-locally-
  customized files, adopter-deleted-but-still-shipped files, and
  genuine overlapping changes (row 16 — this replaces the old
  "customized side" language entirely; there is no longer a frontmatter-
  marker-dependent branch, only the table's own row).

This directly implements #111's recommendation 1 and folds
recommendation 2's structural detection into the mechanism from Design
§1, rather than treating them as alternatives.

### 4. Recommendation 3 (marker hygiene) — accepted as a smaller,
   independent follow-on, not blocking this plan

#111's third recommendation ("if marker-based skipping stays, make the
markers real: ship every rule with `origin: upstream`, give
`customized` a defined home in the learning schema, `agov validate`
flags rule files with no `origin`") is **not required** by this plan's
design — the structural classification in §1 supersedes marker reliance
entirely, which is exactly what makes local-only detection work without
teaching adopters a convention. Retrofitting `origin: upstream` onto
all 49 shipped rule files and a schema field for `customized` remains a
reasonable smaller cleanup, but is independent, lower-value once §1
ships, and not blocking — tracked as a follow-up note in this plan's
"What this plan does NOT do," not implemented here.

### 5. Step 5 verification: governance-inventory diff

Add a Step 5 check comparing the governance-artifact inventory (rule
count, agent-definition count — the same counts `agov.validate.
readme_stats` already computes and reports, per #111's own observation
that "the number is genuinely useful; it is filed under the wrong
heading") *before* and *after* the upgrade, specifically flagging a
**decrease** as a blocking finding requiring explicit user
acknowledgement, not the current advisory `WARN` about a stale README
claim. An increase or unchanged count stays informational, matching
today's behavior.

## What this plan does NOT do

- Does not retrofit `origin: upstream` onto all 49 shipped rule files
  or add a `customized` schema field (see §4) — independent smaller
  follow-up, not blocking.
- Does not keep any frontmatter-marker-dependent branch for overlapping
  files (round 1 correction) — genuinely overlapping changes are always
  row 16 of the Design §1 table, always manual, regardless of what any
  `customized`/`origin` marker says. The old marker-dependent split is
  fully superseded, not preserved alongside the new table.
- Does not address #113 (ignore-rule collisions hiding `agf/dist/` from
  version control) — different failure mode, different area of the
  upgrade process, tracked as its own plan
  (`docs/plans/20260823T203534Z-dist-path-ignore-protection.md`).
- Does not address #112 (adoption.framework_version drift) — unrelated
  validator gap, tracked as its own small plan.
- Does not change Step 1, Step 2, or Step 4 of the upgrade skill.

## Files expected to change

This plan's own diff (Gate 1 candidate): only this plan file.

On the implementation branch, after Gate 1 approval:

- `tooling/rust/crates/agov-coord/src/upgrade_classify.rs` (new) — the
  three-way classification logic.
- `tooling/rust/crates/agov/src/main.rs` — new `Command::Upgrade { sub:
  UpgradeCommand::Classify { .. } }` wiring.
- `agf/skills/upgrade/SKILL.md` — Step 3 rewritten to an overlay
  (never-delete-by-default) using the new subcommand's output; new GATE
  2.5 for removal confirmation; Step 5 gains the governance-inventory
  decrease check.
- `agf/docs/site/docs/adopt/upgrade.md` (new page — named exactly,
  not deferred, per round-1 correction fixing `CA-LRB-UPG-3`; verified
  no such page currently exists and the closest existing page,
  `install-and-layouts.md`, covers initial install/layout rather than
  ongoing upgrades, so a dedicated page is warranted rather than
  cramming GATE 2.5 and the classification contract into an unrelated
  page) — explains the three-way classification table, GATE 2.5, and
  what "adopter content preserved" / "manual review required" mean for
  an adopter reading their own upgrade output.
- Focused Rust unit tests for the classification logic (see "Test
  strategy").

## Test strategy

- Unit tests for `upgrade_classify`, one fixture per row of the
  16-row table in Design §1 (fixes `CA-LRB-UPG-1`) — table-driven,
  keyed by row number, so a future edit to the table is directly
  traceable to its corresponding test.
- Negative: any path whose type differs between two present sides
  (rows 3, 7, 11) — classified as a type conflict, fail-closed, never
  auto-resolved.
- Regression fixture matching harbor-workshop's actual reported case:
  local tree has an extra rule file (row 1, local-only) that a naive
  two-way diff would flag as "removed upstream, delete it" — assert the
  three-way classifier instead marks it local-only (row 1) and the
  skill instructions never touch it.
- Regression fixture matching night-heron's reported need: a path that
  differs between previous and new releases, with local matching
  previous (row 14, upstream-only) — safe overwrite, no manual step.
- Adversarial fixtures (fixes `CA-LRB-UPG-2`): a manifest entry with a
  parent-traversal (`../`) path — rejected fail-closed; a symlink whose
  target escapes its tree root — rejected fail-closed; an unverified
  (checksum-mismatched) release directory — classification refuses to
  run; a case-colliding pair of manifest entries — rejected fail-closed
  before any per-path classification begins.
- `agov plan validate` on this plan file: must pass.
- `agov validate` (full umbrella): no new findings beyond pre-existing,
  unrelated warnings.
- `cargo test -p agov-coord` and `cargo test -p agov --bin agov`: must
  pass, including the new tests.
- **Gate 2 acceptance evidence, per `gate-self-test-rule.md`:** the Gate
  2 MR description MUST include a reproducible negative CLI
  demonstration — exact command, one adversarial fixture (e.g. the
  parent-traversal manifest entry), expected and actual blocking
  output/exit code, and a link to the CI pipeline job log where it ran.

## Downstream impact classification (`downstream-impact-rule.md`)

This plan touches `tooling/**` and a shipped skill under `agf/skills/**`
(distributed via `agf/dist/**`) — trigger paths.

**Classification: BREAKING, not ADDITIVE/PREVENTIVE alone (round 1
correction, fixes `CA-LRB-UPG-3`).** A confirmation gate that did not
exist before now blocks an operation (removal) that the skill's prior
wording permitted an agent to perform without it — a restriction of
previously-permitted behavior, which `downstream-impact-rule.md`
classifies as BREAKING regardless of how well-justified.

```
Impact:
  Foundry:          BREAKING for this project's own future upgrades —
                     the next upgrade this project runs on itself will
                     hit GATE 2.5 and the manual-review rows instead of
                     completing without them. No existing upgrade
                     history is retroactively affected.
  Direct adopters:  BREAKING. An adopter's existing upgrade automation
                     or muscle-memory that assumed Step 3 completes
                     without additional confirmation now stops at GATE
                     2.5 whenever a removed-upstream-unchanged path
                     exists, and stops entirely (manual review required)
                     for any row 3/5/7/9/10/11/16 path. This is a
                     deliberate tightening to prevent silent data loss,
                     not a capability removal — every file an adopter
                     could previously get overwritten/deleted
                     automatically, they still can, but only after the
                     new confirmation step.
  Downstream:       No change to what an adopter's own users see;
                     changes only how the adopter's governance content
                     is protected during upgrade.
  Migration:        Required: adopters following the upgrade skill must
                     read the new GATE 2.5 and manual-review steps
                     documented in agf/docs/site/docs/adopt/upgrade.md
                     (named above) before their next upgrade. No file or
                     schema migration is required — only a changed
                     interaction contract during the upgrade skill
                     itself.
  Evidence:         GitLab issues #96 (night-heron, 2026-07-31) and
                     #111 (harbor-workshop, 2026-08-10) — two
                     independent adopters, two independent failure
                     angles on the same root cause, both citing exact
                     commit/version identities.
```

**Adopter docs parity:** `updated` — `agf/docs/site/docs/adopt/
upgrade.md` (new page, named in "Files expected to change") is the
adopter-facing explanation of the full classification table, GATE 2.5,
and every manual-review row, written for the adopter reading their own
upgrade output, not merely a reference to this plan or the rule text.

## Rollback

Before any adopter relies on the new contract, this is a clean
one-commit revert: the `agov upgrade classify` subcommand is additive
(nothing depends on it existing), the skill-file rewrite has no schema
or data implications, and no existing adoption record or manifest is
modified by this plan.

**Lifecycle-aware rollback, once adopters rely on the new contract
(round 1 correction, fixes `CA-LRB-UPG-3`):** reverting removes GATE
2.5 and the manual-review rows, returning the skill to its pre-plan
wording — which is exactly the behavior issues #96 and #111 reported as
unsafe. Rollback is not consequence-free once this ships: it
re-introduces the silent-deletion and marker-blindness failure modes
for any adopter mid-upgrade at revert time, and any adopter who has
already come to rely on GATE 2.5 loses that safeguard. A revert
decision after adoption should be paired with an explicit interim
mitigation (e.g., a documented manual pre-upgrade backup step) rather
than treated as a no-consequence rollback.
