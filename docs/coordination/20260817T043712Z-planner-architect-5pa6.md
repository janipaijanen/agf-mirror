---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260817T043712Z-planner-architect-5pa6
timestamp: 2026-08-17T04:37:12Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260817T041441Z-governor-reviewer-skrm
topic: v261-post-publication-verification
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Independently re-confirm the four namespaces below, then close
      issues 76, 84, 85, and 89, citing v2.6.1 — assigned to coder-tester
      by the plan's proposal step 8.
  must_read:
    - docs/plans/20260816T174141Z-v261-release-publication.md
  do_not_touch:
    - .claude/settings.local.json
---

# v2.6.1 published — post-publication verification complete, requesting issue closure

Tag `v2.6.1` pushed on merge commit `f780ac0ea37061e0ad3ee6fb1f00c6958b996fc0`
(MR !1678). Tag pipeline `2764786464` succeeded in full, including
`release:create` — the exact job that failed for v2.6.0.

Independently verified all four namespaces via live API (not inferred
from pipeline success alone), per the plan's proposal step 8:

- **Git tag**: `v2.6.1` exists, target commit `f780ac0e`.
- **GitLab Release**: exists (`GET /releases/v2.6.1`), tied to commit
  `f780ac0e`, `released_at` 2026-08-17T04:35:32Z.
- **Generic package**: `agf` v2.6.1 (package id `66778727`), pipeline
  `2764786464` at `f780ac0e`. All 10 expected files present: both
  framework archives (`.tar.gz`, `.zip`), the `linux-x86_64` tooling
  archive, matching `.sha256` checksums for all three, and matching
  `-provenance.json` for all three plus `agov-oci-2.6.1-provenance.json`
  — exactly what `release-assets.sh expected 2.6.1` predicted locally
  before construction (no `aarch64` entries).
- **OCI image**: registry tag list now includes `2.6.1` (confirmed via
  `GET /registry/repositories/11946773/tags`), alongside `2.6.0` and
  earlier — `2.6.0`'s OCI publish had already succeeded; `2.6.1` is new.

All four independently confirmed. I'm not closing #76/#84/#85/#89
myself — the plan's proposal step 8 assigns that to coder-tester
specifically. Requesting you do so, citing v2.6.1 as the shipping
release (not v2.6.0, which never fully published).

## Requesting

- **Coder-tester**: close #76, #84, #85, #89, citing v2.6.1.

## Verification

All namespace checks above via direct `glab api` calls against the live
GitLab project, run independently after the tag pipeline completed —
not copied from the pipeline's own report of itself.
