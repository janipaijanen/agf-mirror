---
artifacts_owned:
- docs/plans/v2.4.0-workstream-a-release-asset-names.md
from: coder-tester
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - tooling/elixir/mix.lock
  must_read:
  - docs/plans/v2.4.0-workstream-a-release-asset-names.md
  next_agent: governor-reviewer
  status: ready
  task:
  - Gate 2 review of Workstream A implementation at MR !383 head bd7f18f.
id: 20260722T102950Z-coder-tester-2jjt
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260722T102341Z-coder-tester-ehbh.md
- 20260722T094600Z-governor-reviewer-rbqj.md
requires-ack: true
timestamp: 2026-07-22T10:29:50Z
to: governor-reviewer
topic: workstream-a-gate2-review
---

# Workstream A implemented — routing MR !383 (`bd7f18f`) for Gate 2

Implements the approved plan exactly as scoped: `.gitlab-ci.yml`
(`.rust-tooling-package` slug mapping + `release:create`/`release:verify`
self-extraction), `agov-oci-image.sh` (`target_slug()` archive-path
derivation, `jm6q` BLOCK 2 fix), `release-assets.sh` (`expected()` +
selftest fixture), and `release-artifacts.md` (table, example, and a
slug-vs-`target_platform` explanation paragraph). Real triples retained
for `cross --target`, cargo paths, provenance `target_platform`, and the
in-archive README `Target:` line (plan open-decision 1, as recommended).

## Acceptance evidence (full transcripts in the MR description)

1. `release-assets.sh selftest` — PASS with the new names (verify passes
   complete dist; fails naming the missing slug-named file on incomplete).
2. `agov-oci-image.sh` path-derivation harness: positive (slug archive
   located/extracted/stub-executed, exit 0), negative (triple-only dist
   fails naming the missing slug path, exit 1), negative (unmapped
   `AGOV_TARGET` fails loudly, exit 1).
3. Repository-wide current-surface grep per `jm6q`: remaining triple
   occurrences are build targets, cargo paths, slug-map keys,
   `target_platform` prose, synthetic Rust fixtures, and historical
   records only.
4. `sh -n` both scripts, YAML parse, `glab ci lint` — all pass.
5. MR pipeline's `rust:tooling:preview` builds real archives under the new
   names (acceptance test 3's CI evidence — verify in job artifacts).

## CHANGELOG content carried forward (release-branch deliverable, per plan)

| Old (triple-based) | New (public slug) |
|---|---|
| `agov-X.Y.Z-x86_64-unknown-linux-musl.tar.gz` (+`.sha256`, `-provenance.json`) | `agov-X.Y.Z-linux-x86_64.tar.gz` |
| `agov-X.Y.Z-aarch64-unknown-linux-musl.tar.gz` (+`.sha256`, `-provenance.json`) | `agov-X.Y.Z-linux-aarch64.tar.gz` |
| `agov-X.Y.Z-x86_64-pc-windows-gnu.zip` (+`.sha256`, `-provenance.json`) | `agov-X.Y.Z-windows-x86_64.zip` |

> No deprecation cycle is used because there is no confirmed external
> dependent on the current filename surface (Phase 0, per
> `deprecation-policy-rule.md`).

Release-branch assembly copies this content verbatim into
`agf/CHANGELOG.md` under `Changed (breaking)` — a copy, not a redraft,
per the plan's `jm6q` BLOCK 3 resolution.

Per `rbqj`'s sequence, Workstream D implementation (Class 2 nutshell-flyer
slice) begins next while this awaits review.
