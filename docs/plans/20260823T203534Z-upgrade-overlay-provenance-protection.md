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

### 1. New `agov` subcommand: three-way upgrade classification

Add `agov upgrade classify --previous-release <dir> --new-release <dir>
--local <dir>` (crate `agov-coord`, wired in `agov/src/main.rs`
alongside the existing `Dist`/`Plan`/`Coord` command groups — no
existing subcommand does tree comparison today, verified via `grep -n
"enum.*Command" tooling/rust/crates/agov/src/main.rs`). For every path
under the archive's distributed globs (`agf/dist/release-manifest.yaml`
is already the authoritative list of what a release ships — reuse it,
do not invent a second list), classify:

- **upstream-only** — differs between previous-release and new-release,
  identical between local and previous-release (adopter never touched
  it): safe to overwrite.
- **adopter-only** — identical between previous-release and new-release
  (upstream did not change it), differs between local and
  previous-release (adopter changed or added it): **never touched by
  the upgrade**, listed for the user's awareness only.
- **overlapping** — differs in all three: both sides changed it. Flag
  for manual merge, matching today's `customized: true` /
  `origin: project` treatment but now covering every file, not only
  marked ones. The file's own frontmatter marker (where present) is a
  secondary, non-authoritative signal only — the three-way comparison
  is what actually decides the class.
- **removed-upstream** — present in previous-release, absent from
  new-release, unchanged locally: upstream intentionally dropped it;
  safe to remove.
- **local-only** — present locally, absent from *both* pristine
  releases: adopter-owned by construction (this is the exact case
  markers currently miss — #111's core finding). **Never touched.**

Output: a structured table (human-readable and `--format json` for the
skill to consume programmatically) plus exit code 0 only when every
path resolves to one of the five classes above with no ambiguity.

This directly implements #96's recommendation ("make the previous
pristine release an explicit upgrade input... classify... before Gate
2") and #111's recommendation 2 ("detect adopter-owned content
structurally rather than by marker... needs no marker discipline and
no adopter foreknowledge").

### 2. Upgrade skill: Step 3 becomes an overlay, never a delete-by-default

Rewrite `agf/skills/upgrade/SKILL.md` Step 3 to require
`agov upgrade classify` and act only on its classification:

- **upstream-only** and **overlapping** (non-customized side): copy
  from archive, overwriting local — today's behavior, unchanged.
- **adopter-only** and **local-only**: never copied, never deleted,
  never overwritten. Listed in the Step 6 summary as "adopter content
  preserved," not silently skipped.
- **removed-upstream**: **not deleted automatically.** Per #111's
  recommendation 1 ("an upgrade adds and updates but never deletes,
  and removing a file is a separate, named decision"), present the
  list to the user at a new **GATE 2.5 — confirm removals**, requiring
  explicit per-file or explicit blanket confirmation before any
  deletion. If the user does not confirm, the file is left in place and
  listed as a manual follow-up in the Step 6 summary.
- **overlapping** (customized side): today's existing behavior (show
  diff, do not overwrite) is preserved unchanged — this plan does not
  touch that path.

This directly implements #111's recommendation 1 and folds
recommendation 2's structural detection into the mechanism from Design
§1, rather than treating them as alternatives.

### 3. Recommendation 3 (marker hygiene) — accepted as a smaller,
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

### 4. Step 5 verification: governance-inventory diff

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
  or add a `customized` schema field (see §3) — independent smaller
  follow-up, not blocking.
- Does not change the `customized: true` / `origin: project` handling
  for genuinely overlapping (both-sides-changed) files — that manual-
  merge path is unchanged.
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
- `agf/docs/site/docs/adopt/` — the adopter-facing upgrade guide (exact
  file to be confirmed against current `agf/docs/site/docs/adopt/`
  contents at implementation time) gains a short explanation of the
  three-way classification and the new removal-confirmation gate,
  satisfying `downstream-impact-rule.md`'s adopter-docs-parity
  requirement (this plan changes a tooling command and a shipped skill
  — both named trigger categories).
- Focused Rust unit tests for the classification logic (see "Test
  strategy").

## Test strategy

- Unit tests for `upgrade_classify`, covering all five classes plus the
  ambiguous/conflicting case (a path whose three-way relationship does
  not cleanly resolve — e.g., missing from the previous-release archive
  entirely because it predates that release's own distribution
  manifest) with an explicit "cannot classify, human review required"
  outcome rather than silently guessing.
- Regression fixture matching harbor-workshop's actual reported case:
  local tree has an extra rule file (local-only) that a naive two-way
  diff would flag as "removed upstream, delete it" — assert the
  three-way classifier instead marks it local-only and the skill
  instructions never touch it.
- `agov plan validate` on this plan file: must pass.
- `agov validate` (full umbrella): no new findings beyond pre-existing,
  unrelated warnings.
- `cargo test -p agov-coord` and `cargo test -p agov --bin agov`: must
  pass, including the new tests.

## Downstream impact classification (`downstream-impact-rule.md`)

This plan touches `tooling/**` and a shipped skill under `agf/skills/**`
(distributed via `agf/dist/**`) — trigger paths.

```
Impact:
  Foundry:          ADDITIVE + PREVENTIVE. This project's own future
                     upgrades gain the classification tool and the
                     overlay-by-default skill behavior; no existing
                     upgrade history is retroactively affected.
  Direct adopters:  PREVENTIVE, narrowing previously-permitted silent
                     behavior. An adopter who relied on the upgrade
                     skill's raw copy behavior (none should have, per
                     the skill's own documented intent, but the skill's
                     wording permitted it) now gets an explicit
                     removal-confirmation gate instead. This is a
                     deliberate tightening to prevent silent data loss,
                     not a capability removal — every file an adopter
                     could previously get overwritten/deleted
                     automatically, they still can, but only after
                     explicit confirmation for removals.
  Downstream:       No change to what an adopter's own users see;
                     changes only how the adopter's governance content
                     is protected during upgrade.
  Migration:        Not required for adopters who have never lost
                     content to this failure mode. Adopters who
                     previously worked around the marker gap by other
                     means (e.g., manual pre-upgrade backups) can drop
                     that workaround once this ships, but are not
                     required to change anything.
  Evidence:         GitLab issues #96 (night-heron, 2026-07-31) and
                     #111 (harbor-workshop, 2026-08-10) — two
                     independent adopters, two independent failure
                     angles on the same root cause, both citing exact
                     commit/version identities.
```

## Rollback

The new `agov upgrade classify` subcommand is purely additive (nothing
depends on it existing). The skill-file rewrite is a documentation-only
change with no schema or data implications — reverting is a one-commit
revert. No existing adoption record or manifest is modified by this
plan.
