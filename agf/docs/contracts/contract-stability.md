<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Contract stability promise

What downstream Continuous Integration (CI) integrations can rely
on across releases of `agents-govern`. The framework's
[Integration Contract](integration-contract.md) names the
machine-checkable surface; this page names the *stability
discipline* that surface follows release-to-release.

> **AGF v2 status note:** The v1.x Python command names below remain historical
> contract context. Current AGF-NG tooling is the Rust `agov` binary under
> `tooling/rust/`; E4-deferred commands are named explicitly where they
> have not yet been implemented.

This is the canonical home. Action plan item
[A5] (historical predecessor-tracker issue 54)
from the
[2026-05-04 ChatGPT engagement](plans/2026-05-04-chatgpt-engagement-action-plan.md).

## Single-version discipline

**The contract version equals the framework version.** One number,
one source of truth, no dual-version drift. When `agents-govern`
ships v1.1.41, the integration contract is *also* at v1.1.41. The
v1.1.41 contract is whatever the validators, exit codes, output
formats, trigger set, and Command Line Interface (CLI) surface
*do* at the v1.1.41 tag.

The engagement that produced this discipline (round-4 §7) raised
the alternative — a separate contract version number tracked
independently of framework semver. The maintainer rejected that on
the grounds that two version numbers invite drift: framework
semver moves on every meaningful merge to `main`, and a
parallel contract version would lag, creating ambiguity about
which combination of numbers an adopter is actually consuming.

## The promise (patch / minor / major)

The framework's version follows
[semantic versioning](https://semver.org/): `MAJOR.MINOR.PATCH`.
The integration contract obeys the same shape, with these rules:

| Release type | Contract may… | Contract must NOT… |
|---|---|---|
| **Patch** (`X.Y.Z` → `X.Y.Z+1`) | Fix bugs, clarify documentation, improve error messages | Add validators / exit codes / fields; remove anything; rename anything |
| **Minor** (`X.Y` → `X.Y+1`) | *Extend*: add validators, add exit codes (in additive ranges), add CLI flags, add output-format fields | Remove anything, rename anything, change exit-code meanings, change return shapes for existing fields |
| **Major** (`X` → `X+1`) | *Change*: remove deprecated surface, change return shapes, rename, restructure | (no constraint other than the deprecation cycle below) |

**Read the columns top-to-bottom.** Patch releases are the safest
to absorb; minor releases are additive and safe for adopters who
treat any non-zero exit code as failure (the framework's
recommended consumer pattern); major releases require an explicit
migration step.

## What is covered by the contract

The contract is the machine-checkable surface — what an external
CI engine consumes:

- **Validator command names** for the current AGF-NG Rust surface:
  `agov validate`, `agov validate agents`, `agov validate learnings`,
  `agov validate infosec`, `agov validate adoption`,
  `agov validate contributions`, `agov validate framework-config`,
  `agov dist check-tag`, and `agov dist build`.
- **Exit code semantics** per the
  [v1.1.36 taxonomy](integration-contract.md#exit-codes):
  `0` pass, `1` validation failure, `2` configuration error,
  `3` invariant violation, `4` infrastructure / Input/Output
  (I/O) failure. Adding new codes within the taxonomy is
  minor-additive; changing the meaning of an existing code
  requires a major release.
- **Output format shape** per the
  [Output formats section](integration-contract.md#output-formats):
  the aggregate validator output shape. In v1.x this was
  `agov-validate --json`; AGF-NG currently exposes human-readable Rust CLI
  output while JSON aggregation remains a contract item to preserve or
  reintroduce explicitly when the v2 CLI surface is finalized.
- **Trigger set** documented in the
  [Path-based revalidation triggers table](integration-contract.md#path-based-revalidation-triggers)
  — adding new path globs is minor-additive; removing one is
  major.
- **Five-layer verifiability model** — the layer numbering and
  semantics are part of the contract. Renaming a layer is major;
  splitting layer 2 into 2a / 2b (which already happened) was
  additive within the existing model.

## What is NOT covered by the contract

The framework's *loose-prose surface* (per
[`docs/position/dual-surface.md`](position/dual-surface.md)) is
intentionally not under the contract:

- **Rule wording** — the prose of framework rules under `agf/rules/` may be
  adapted by adopters. If an adopter creates project-local rules outside
  `agf/`, those local rules are owned by the adopter and are outside this
  contract.
- **Learning record content** — `learnings/*.yaml` files evolve
  on every meaningful merge; their *individual* content is
  evidence, not contract.
- **Documentation prose** — page wording on this site can change
  freely. Page *paths* and *anchor identifiers* are
  best-effort-stable but are not contract.
- **Historical Python module structure** — `src/agents_govern/` belongs to the
  v1.x Python line. It is legacy reference material in AGF-NG v2, not a
  supported v2 extension API.

The
[Dual-surface model](position/dual-surface.md) page explains why
the framework keeps both surfaces and why formalising the
loose-prose surface would break the evidence-driven evolution
model.

## Worked examples from this release thread

| Version | Type | What changed | Contract impact |
|---|---|---|---|
| v1.1.36 | Minor | Defined exit-code taxonomy (`_exit_codes.py`); validators currently still emit only `0` / `1` | **Additive** — codes 2 / 3 / 4 are new return *values* that adopters' CI scripts may see as validators adopt them opportunistically |
| v1.1.37 | Minor | Added `agov-validate` orchestrator with `--json` aggregate output | **Additive** — new entry point + new output format |
| v1.1.38 | Minor | Added [Compliance profile](adopt/compliance-profile.md) page (three tiers) | **Documentation-only** — names existing surface, adds no new validators or codes |
| v1.1.39 | Patch | Bookkeeping (`learnings/.session-notes/` gitignore) | **No contract impact** |
| v1.1.40 | Patch | Added `validate-readme-stats` and `validate-contributions` CI jobs | **Additive** — new CI gates trigger on documented path set; no validator behaviour changed |

The pattern: every increment in the v1.1.x line so far has been
patch (no contract change) or minor-additive (new surface,
existing surface unchanged).

## Adopter recommendations

The reasonably defensive consumer pattern:

1. **Pin to a major version** in your CI (e.g.,
   `pip install 'agents-govern>=1.0,<2.0'`). Patch and minor
   releases are absorbed without code changes.
2. **Branch CI scripts on `!= 0`**, not on specific non-zero
   codes. Adopters who must dispatch on exit-code semantics
   (e.g., paging on infrastructure failures but not on validation
   failures) read the
   [Exit codes section](integration-contract.md#exit-codes)
   and pin to a minor version that includes the codes they need.
3. **Read the [CHANGELOG](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/agents-govern/CHANGELOG.md)
   on minor and major bumps.** Patch releases never touch the
   contract; minor releases extend it; major releases change it.
4. **Test the upgrade against a branch** before merging the
   version-pin bump. Most adopters can absorb a minor bump in
   one Merge Request (MR); a major bump should be its own MR
   with explicit migration notes.

## Major-release deprecation cycle

When a major release removes contract surface, the previous major
is supported through one deprecation cycle. Concretely:

1. **Deprecation notice in a minor release.** The surface still
   works but is marked deprecated in this page's *Deprecations*
   section (added when the first deprecation lands).
2. **Removal in the next major release.** The deprecated surface
   is removed; the major release notes link back to the
   minor-release deprecation notice.
3. **Previous major supported in parallel** for one full minor
   cycle of the new major. Critical fixes backport to the
   previous major; new features land on the new major only.

## Retired and deferred v1.x surfaces

AGF-NG v2 uses the Rust `agov` command surface documented in the current
adopter docs. The following v1.x/Python surfaces are not current AGF v2 product
promises:

| Surface | AGF v2 status | Replacement or note |
|---|---|---|
| `agov-validate-binding` | Deferred | Planned Layer 3 / Full-tier validator; no Rust `agov` implementation yet. |
| `agov-validate-acronyms` | Retired | No current replacement; not on the critical governance path. |
| `agov-bump` | Retired | Version discipline is handled by release workflow and `agov dist check-tag`. |
| `agov-health` | Deferred | Useful future diagnostic, not required for v2 governance operation. |
| `agov --capabilities` | Retired | Use current docs, manifests, and `agov validate` output. |
| `agov --contract-changelog` | Retired | Use the changelog/release notes directly. |

## Command Line Interface (CLI) surface

**v1.x status:** The `agov` umbrella entry point shipped in v1.1.43 and the
flat `agov-validate-*` scripts continued to work unchanged.

**AGF-NG status:** Current migrated tooling runs through the Rust `agov`
binary. The v1.x/Python command names below are historical context unless a
current AGF v2 page explicitly repeats them.

- `agov --version` — framework version (= contract version per
  this page's *Single-version discipline*).
- `agov --contract-version` — alias of `--version`.
- `agov --contract-changelog` — retired in AGF v2; use changelog/release
  notes directly.
- `agov --capabilities` — retired in AGF v2; use current docs, manifests, and
  `agov validate` output.

Pick exactly one action flag per invocation; mixing actions exits
non-zero. Bare `agov` prints help.

## See also

- [Integration contract](integration-contract.md) — canonical home
  for the machine-checkable surface (validators, exit codes,
  output formats, trigger set, five-layer model).
- [Dual-surface model](position/dual-surface.md) — why the
  framework keeps both a strict-contract and a loose-prose
  surface, and why this page covers only the strict half.
- [Compliance profile](adopt/compliance-profile.md) — three-tier
  vocabulary an adopter uses to declare which verifiability
  layers they enforce; orthogonal to but consumes the contract
  stability promise.
- [`CHANGELOG.md`](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/agents-govern/CHANGELOG.md)
  — full release history; the contract changelog is a filtered
  view of this file.
