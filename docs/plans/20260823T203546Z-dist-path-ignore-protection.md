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

### 2. Validator: confirm every shipped path is version-control-visible

New check (crate `agov-validators`, alongside the existing
`adoption`/`adopter_docs`/`agents` checks in
`tooling/rust/crates/agov-validators/src/`): for every path the current
release manifest declares as distributed
(`agf/dist/release-manifest.yaml` — the same authoritative source used
by the companion `upgrade-overlay-provenance-protection` plan), confirm
the path is not excluded by any effective `.gitignore` rule (`git
check-ignore -q <path>`; exit code `1` means not ignored, `0` means
ignored — surface any ignored path as a blocking finding naming the
exact path and, where resolvable, the ignore rule and file responsible
via `git check-ignore -v`).

This also catches the issue's noted broader case: an adopter whose
ignore rules were inherited from a template they never read — the
validator does not depend on the adopter having read the install guide
at all.

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
- `tooling/rust/crates/agov-validators/src/dist_visibility.rs` (new) —
  the ignore-rule check.
- `tooling/rust/crates/agov/src/main.rs` — wire the new check into
  `agov validate` (or a dedicated subcommand if the implementer judges
  the check too slow for the default umbrella run — `git check-ignore`
  is a per-path subprocess call, so a large release manifest may
  warrant batching or a `--skip-dist-visibility` opt-out for umbrella
  runs; implementer's call, not pre-decided here).
- Focused unit/integration tests (see "Test strategy").

## Test strategy

- Positive: a repository with no ignore rule touching `agf/` — the
  check passes for every manifest-declared path.
- Negative (regression fixture matching the reported case): a
  repository with an unanchored top-level `dist/` ignore rule that
  collides with `agf/dist/` — the check fails, naming the exact path
  and the offending rule.
- Positive: an anchored rule (`/dist/` at repo root only) does not
  collide with `agf/dist/` — the check passes, proving the fix does not
  produce false positives for adopters who already anchor correctly.
- `agov plan validate` on this plan file: must pass.
- `agov validate` (full umbrella) on this repository itself: no new
  findings — this repository's own `agf/` paths must not be
  ignore-excluded (self-check).
- `cargo test -p agov-validators`: must pass, including the new tests.

## Downstream impact classification (`downstream-impact-rule.md`)

This plan touches `tooling/**` and `agf/docs/site/**` — trigger paths.

```
Impact:
  Foundry:          ADDITIVE. This repository's own `agf/` paths are
                     not currently ignore-excluded (verified: no
                     matching finding expected), so the new check adds
                     detection capability with no behavior change here.
  Direct adopters:  ADDITIVE + advisory documentation. An adopter with
                     no collision sees no change; an adopter with a
                     collision (previously silent) now gets a blocking
                     finding naming the exact path.
  Downstream:       No adopter-produced artifact shape changes; no
                     agent behavior toward an adopter's users changes.
  Migration:        Not required for adopters with no collision. An
                     adopter with an existing collision must anchor or
                     negate their ignore rule once, the same fix the
                     issue itself recommends.
  Evidence:         GitLab issue #113 (harbor-workshop, 2026-08-21).
```

## Rollback

Pure additive: one documentation addition, one new read-only validator
check. Reverting is a one-commit revert with no schema, data, or
filesystem-write implications.
