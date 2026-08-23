---
id: dist-path-ignore-protection
status: pending-gate1
topic: dist-path-ignore-protection
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Requesting independent contribution-auditor review before Gate 1.
      I (planner-architect) author this plan; the originating issue
      (#113, adopter `harbor-workshop`) reports an observation and two
      proposed changes, neither authored by any session of mine.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Reads local filesystem paths and `git check-ignore` output; no
      credential, network call, or authentication surface. The concern
      this plan addresses (release provenance silently excluded from
      version control) is itself an integrity/auditability property,
      but the fix is a documentation addition plus a read-only
      validator check — no write-path or security-boundary change.
---

# Plan: detect and document adopter ignore-rules that exclude `agf/` paths

## Round 1 correction (fixes contribution-auditor's exact-head review, `pssr`)

`20260823T205558Z-contribution-auditor-pssr.md` reviewed this plan and
recommended BLOCK, 3 blocking findings. Verified against this
repository's actual current content (not assumed) and fixed:

- **`CA-LRB-IGN-1`** — verified directly with a throwaway repository:
  `git check-ignore -q <path>` on a **tracked, force-added** file whose
  ignore rule still effectively matches it returns exit `1` ("not
  ignored") — the opposite of the correct answer — because the default
  mode consults the index and suppresses tracked paths. `--no-index`
  (`git check-ignore -q --no-index <path>`) correctly returns exit `0`
  in the same scenario, testing the rule itself rather than tracked
  status. Fixed: Design §2 now specifies `--no-index` explicitly, with
  a fixture covering exactly this force-added case.
- **`CA-LRB-IGN-2`** — the original design said "for every path the
  manifest declares as distributed" without defining glob expansion,
  exclude precedence, optional-entry handling, or non-Git-worktree
  behavior against `agf/dist/release-manifest.yaml`'s actual mixed
  `path`/`glob`/`exclude`/`forbidden`/`optional` shape (verified by
  reading the manifest directly — it is not a flat enumerable list).
  Fixed: Design §2 now reuses the existing, already-tested manifest-
  expansion logic (`agov`'s `dist::validate_default()` /
  `DistSpec`, the same code `agov plan projection-check`'s
  distribution-completeness check already depends on) instead of
  writing new expansion logic — this closes the ambiguity by
  construction rather than by additional specification, since the
  expansion behavior is already defined, tested, and used elsewhere.
- **`CA-LRB-IGN-3`** — the canonical invocation, bypass authority, and
  CI/release/install wiring were left to the implementer, and the
  change was classified ADDITIVE despite restricting a previously-
  accepted checkout state. Fixed: Design §3 decides the invocation and
  bypass authority; "Downstream impact classification" is corrected to
  BREAKING with a named migration path.

Implements GitLab issue [#113](https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/113),
an adopter learning record (`harbor-workshop`, 2026-08-21) that has sat
in the backlog with no plan since filing.

## The problem

The release archive ships `agf/dist/`, containing the release
manifest — the record of what was actually distributed and its
provenance. An adopter's repository had an unanchored top-level
`.gitignore` rule for `dist/` (intended for their own build-tool
output), which also matched `agf/dist/`. The archive extracted
correctly; the files existed on disk exactly as expected; version
control simply never tracked them. Nothing failed, nothing warned —
the one part of the adoption whose entire purpose is auditable
provenance was the one part left untracked.

The issue notes this collision is likely common, not unusual: a
generic top-level ignore of `dist/`, `build/`, or `docs/` is ordinary in
web and application projects, and the exclusion is invisible from
inside the framework because the files are present on disk exactly as
the framework expects.

## Design

Two independent fixes, both from the issue's own proposal, implemented
together (they are complementary, not alternatives — the validator
check catches what the documentation alone cannot force an adopter to
read):

### 1. Documentation: name the collision explicitly

Add a short, explicit warning to
`agf/docs/site/docs/adopt/install-and-layouts.md` (verified: this file
currently has no mention of `.gitignore`, ignore rules, or `dist/`
collisions at all), in the post-extraction section: confirm no existing
ignore rule excludes any part of `agf/`, and either anchor project-
level patterns (e.g. `/dist/` instead of `dist/`) or explicitly negate
the framework path (`!agf/dist/`, `!agf/**`).

### 2. Validator: confirm every shipped path is version-control-visible (revised, fixes `CA-LRB-IGN-1`, `CA-LRB-IGN-2`)

New check (crate `agov-validators`, alongside the existing
`adoption`/`adopter_docs`/`agents` checks in
`tooling/rust/crates/agov-validators/src/`):

- **Enumeration (fixes `CA-LRB-IGN-2`):** do not re-implement manifest
  expansion. Call `agov`'s existing `dist::validate_default()`, which
  already resolves `agf/dist/release-manifest.yaml`'s `include`
  (`path` and `glob`, including `optional: true`), `exclude`, and
  `forbidden` sections against the local adopted tree into a single
  `DistSpec` holding the final, already-deduplicated, already-exclude-
  filtered list of distributed files. This is the same logic `agov plan
  projection-check`'s distribution-completeness check already relies
  on — reusing it means glob/exclude/optional semantics are correct by
  construction, not by re-specification. `DistSpec`'s private `files`
  field needs one small `pub fn files(&self) -> &[String]` accessor
  added (currently only `contains(&self, path)` is public); this is the
  only change needed to `dist.rs` itself.
- **Ignore check per path (fixes `CA-LRB-IGN-1`):** for each path in
  `DistSpec::files()`, run `git check-ignore -q --no-index <path>`
  (verified directly: **`--no-index` is required** — without it, a
  path that is tracked, including one an adopter force-added with `git
  add -f` over an effective ignore rule, is reported as "not ignored"
  regardless of whether the rule still matches, which is the exact
  false-negative this check exists to prevent). Exit `0` means the
  effective rule still matches (ignored) — blocking finding, naming the
  exact path plus `git check-ignore -v --no-index <path>`'s rule/file
  attribution where resolvable. Exit `1` means genuinely not ignored —
  passes. Any other exit code (a `git` invocation error, e.g. outside a
  Git worktree) is a distinct, non-silent tooling-error finding, never
  treated as "not ignored" by default.
- **Non-Git worktree behavior:** if `git check-ignore` itself is
  unavailable (no `.git` present — e.g. an archive extracted outside
  version control entirely), the check reports this explicitly as
  "cannot verify version-control visibility outside a Git worktree" and
  does not silently pass.

This also catches the issue's noted broader case: an adopter whose
ignore rules were inherited from a template they never read — the
validator does not depend on the adopter having read the install guide
at all.

### 3. Canonical invocation and bypass authority (new, fixes `CA-LRB-IGN-3`)

Wired into the default `agov validate` umbrella (not a separate opt-in
subcommand) — this matches the severity of the failure mode (silent
provenance loss should be caught by the same routine check an adopter
already runs, not one they have to know to run separately). If the
per-path `git check-ignore` subprocess cost proves material for large
manifests once implemented, an opt-out flag `--skip-dist-visibility` MAY
be added, but it defaults to unset (check always runs) and its use MUST
be logged in `agov validate`'s own output, not silent. No bypass
authority beyond that flag is granted; this check gains no new blocking
authority beyond what `agov validate`'s existing umbrella already
carries for its other checks.

## What this plan does NOT do

- Does not touch the upgrade skill's Step 3 apply logic — this is a
  detection check, not an apply-time behavior change (unlike the
  companion `upgrade-overlay-provenance-protection` plan).
- Does not modify any adopter's existing `.gitignore` — the fix is
  documentation plus a report, not an automatic rewrite of a file that
  may carry other, unrelated adopter-specific rules.
- Does not address #96, #111, or #112 — tracked as separate plans.

## Files expected to change

This plan's own diff (Gate 1 candidate): only this plan file.

On the implementation branch, after Gate 1 approval:

- `agf/docs/site/docs/adopt/install-and-layouts.md` — new post-
  extraction warning and anchoring guidance.
- `tooling/rust/crates/agov/src/dist.rs` — add `pub fn files(&self) ->
  &[String]` accessor to `DistSpec` (the only change needed to reuse its
  existing expansion logic).
- `tooling/rust/crates/agov-validators/src/dist_visibility.rs` (new) —
  the ignore-rule check, driven by `dist::validate_default().files()`.
- `tooling/rust/crates/agov/src/main.rs` — wire into the default `agov
  validate` umbrella (decided in Design §3, not deferred); optional
  `--skip-dist-visibility` logged-use opt-out.
- Focused unit/integration tests (see "Test strategy").

## Test strategy

- Positive: a repository with no ignore rule touching `agf/` — the
  check passes for every `DistSpec::files()` path.
- Negative (regression fixture matching the reported case): a
  repository with an unanchored top-level `dist/` ignore rule that
  collides with `agf/dist/` — the check fails, naming the exact path
  and the offending rule.
- Negative (fixes `CA-LRB-IGN-1`): a repository where the colliding
  path was tracked via `git add -f` before the ignore rule was added —
  the check still fails (via `--no-index`), proving the force-add case
  the plain `-q` invocation would have missed.
- Positive: an anchored rule (`/dist/` at repo root only) does not
  collide with `agf/dist/` — the check passes, proving the fix does not
  produce false positives for adopters who already anchor correctly.
- Positive (fixes `CA-LRB-IGN-2`): a manifest fixture exercising each
  entry form (literal `path`, recursive `glob`, `exclude`, `optional:
  true` glob matching zero files) — the check's file list matches
  `DistSpec::files()` exactly, proving no separate, drifting expansion
  logic was written.
- Negative: outside a Git worktree — the check reports the explicit
  "cannot verify" finding, never a silent pass.
- `agov plan validate` on this plan file: must pass.
- `agov validate` (full umbrella) on this repository itself: no new
  findings — this repository's own `agf/` paths must not be
  ignore-excluded (self-check).
- `cargo test -p agov-validators`: must pass, including the new tests.
- **Gate 2 acceptance evidence, per `gate-self-test-rule.md`:** the Gate
  2 MR description MUST include a reproducible negative CLI
  demonstration — exact command, the force-added-and-ignored fixture,
  expected and actual blocking output, and a link to the CI pipeline
  job log where it ran.

## Downstream impact classification (`downstream-impact-rule.md`)

This plan touches `tooling/**` and `agf/docs/site/**` — trigger paths.

**Classification: BREAKING, not ADDITIVE (round 1 correction, fixes
`CA-LRB-IGN-3`).** Wiring into the default `agov validate` umbrella
means a checkout that previously passed validation (its ignore rules
were never checked) can now fail on an existing, pre-this-plan
collision. That is a restriction of previously-accepted state.

```
Impact:
  Foundry:          ADDITIVE for this repository specifically — its own
                     `agf/` paths are not currently ignore-excluded
                     (verified: no matching finding expected today), so
                     no behavior change here in practice.
  Direct adopters:  BREAKING. An adopter whose checkout already has an
                     ignore-rule collision (previously undetected, their
                     `agov validate` previously passed) now gets a
                     blocking finding the next time they run it —
                     narrows previously-accepted validation state.
  Downstream:       No adopter-produced artifact shape changes; no
                     agent behavior toward an adopter's users changes.
  Migration:        Required for any adopter with an existing
                     collision: anchor or negate the colliding ignore
                     rule (documented in the install guide addition),
                     then re-run `git add` to bring the previously-
                     excluded paths under version control. No schema or
                     record change required.
  Evidence:         GitLab issue #113 (harbor-workshop, 2026-08-21).
```

**Adopter docs parity:** `updated` — `agf/docs/site/docs/adopt/
install-and-layouts.md` (Design §1) names the collision and the
anchoring/negation fix; this is the same page an adopter reads during
initial install, where the collision actually originates.

## Rollback

One documentation addition, one new read-only validator check (plus a
one-line public accessor on existing `DistSpec`). Reverting is a
one-commit revert with no schema, data, or filesystem-write
implications — the check is read-only throughout, so no adopter state
depends on it having run.
