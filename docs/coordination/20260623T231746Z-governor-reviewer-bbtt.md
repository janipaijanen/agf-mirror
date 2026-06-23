---
kind: note
from: governor-reviewer
to: all
id: 20260623T231746Z-governor-reviewer-bbtt
timestamp: 2026-06-23T23:17:46Z
immutable: true
references-to:
  - 20260623T231710Z-governor-reviewer-trqk.md
topic: rc1-step5-versioning-decisions-needed
requires-ack: false
artifacts_owned:
  - docs/coordination/20260623T231746Z-governor-reviewer-bbtt.md
---

# Step 5 blocked on governor decisions: three versioning questions

Step 4 is complete. Step 5 (tag and publish RC-1) requires three decisions
from the governor before Codex can begin the release branch.

## Decision 1 — agov version alignment

`agov --version` currently reports `0.1.0`. The framework `agf/VERSION` is `2.1.6`.

**Option A — Independent:** `agov` stays on its own `0.x` track. Release docs
must clearly map which `agov` version ships with which framework version.

**Option B — Aligned:** `agov` Cargo.toml version tracks `agf/VERSION`. The
RC release MR bumps both. One version story for adopters.

**Governor: choose A or B.**

## Decision 2 — RC version number

Steps 1-4 added: readable coordination IDs (new capability), version provenance
(new command surface), `agov version --format json` (new command), and
three-layer contract docs for `coord graph-index`. These are new minor-level
additions on top of `2.1.6`.

Conventional SemVer would call this `2.2.0`. The project has been on patch
increments (`2.1.x`) throughout. Either is defensible.

**Governor: choose the RC version string (e.g., `2.2.0-rc.1` or `2.1.7-rc.1`).**

If Decision 1 chose **Aligned**, this version also becomes the `agov` crate version.

## Decision 3 — Pre-bump vs post-bump

From `docs/plans/rc1-sequence.md` §Pre-release versioning scheme:

- **pre-bump:** main carries `X.Y.Z-dev` after each release. RC branch bumps
  to `X.Y.Z-rc.N`.
- **post-bump only:** main carries the last released version. VERSION bumps
  only in the release branch MR.

**Governor: choose pre-bump or post-bump.**

## How to unblock

Reply with three answers. Codex will then open the release branch and bump
versions per the chosen scheme.

Example reply:
> Decision 1: Aligned. Decision 2: 2.2.0-rc.1. Decision 3: post-bump only.
