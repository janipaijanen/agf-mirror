---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261103T211221Z-planner-architect-bks2
timestamp: 2026-11-03T21:12:21Z
model: claude
immutable: true
references-to:
  - 20261101T195927Z-v300-release-publication.md
  - 20261101T191657Z-governor-reviewer-pr6j.md
  - 20261102T182936Z-planner-architect-1jkm.md
  - 20261102T190229Z-planner-architect-rqeb.md
  - 20261103T202753Z-governor-reviewer-rkjm.md
topic: v300-metadata-mr-3502-ready-for-gate2-and-tag
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Decide Gate 2 for MR !3502 (release metadata) at the exact head fc5300ad7f4ea27a92c81c89c577ee81976656c1, or return it.
    - If approved, merge without squash at that exact SHA, verify first parent is main at the reviewed target and second parent is the head, then create the one annotated v3.0.0 tag on the merge commit after the protected-tag rule and the pre-push namespace check.
  must_read:
    - docs/plans/20261101T195927Z-v300-release-publication.md
  do_not_touch:
    - agf/VERSION
    - tooling/rust/Cargo.lock
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# AGF 3.0.0 metadata MR !3502: ready for your Gate 2 and the tag

This record grants no Gate 2, merge, tag, version or date. The tag and the metadata Gate 2 are the
Human Governor's (the delegation `srra` excludes them).

## Exact candidate

| Item | Value |
|---|---|
| MR | !3502, branch `release-3.0.0` |
| Head | `fc5300ad7f4ea27a92c81c89c577ee81976656c1` |
| Target | `main` at `c404391f` (the re-freeze point after !3503) |
| Pipeline at head | 3024399193, success, one MR pipeline |
| Diff against main | 29 paths: 27 version-field files, `agf/CHANGELOG.md`, `CHANGELOG.md`; 36 one-line version edits |
| Changelog date | 2026-11-03 (the Human Governor's instruction, bridge note `rqeb`) |

## Evidence

- Namespace preflight at this head against `c404391f`: git tag, GitLab Release, generic package
  `agf`, OCI `agov:3.0.0` all ABSENT, 2026-11-03T20:47:39Z. The pre-push run on the merge commit
  is still required.
- `Cargo.lock`: exactly five workspace-version lines; `cargo metadata --locked --offline` succeeds.
- `agov validate` adopter-docs, agents, learnings, adoption, infosec: OK. `readme-stats` warns
  (README says 16 agent definitions, finds 17); it predates this MR.
- Independent AI reviews (fresh, read-only, not the CI `infosec-sentinel`):
  - Metadata and security: CLEAR at `9051f860` and again at `d4ca4e8f`, no findings. Not repeated at
    the head: only the two changelogs changed after `d4ca4e8f`.
  - Changelog content: BLOCK at `9051f860` (missing path lists), fixed; CLEAR at `d4ca4e8f` with
    four advisories; a delta review of the next head CLEAR with two wording advisories; the last
    head applies those two wording fixes and was not reviewed separately.
  - Content-freeze sweep at `909feeb1`: one blocking false count (applicability catalog), fixed by
    !3503 (independent review CLEAR, merged `c404391f`). Two advisories left out on purpose: a
    3.0.0 pointer in `review-and-release.md` and a forge-rule pointer in the runbook.

## Decisions that are yours

1. **Gate 2** for !3502 at the exact head above.
2. **BREAKING classification of the role guidance.** The changelog lists one BREAKING item (the
   forge-publication-evidence rule). It also describes, as guidance, a larger solo-trial minimum
   (`planner-architect`), a `build-engineer` row for CI projects, `fact-checker` recommended, and
   `infosec-sentinel` and `contribution-auditor` required on their triggers. A reviewer rated this
   advisory because no validator or schema changed; you may classify it differently.
3. **Confirm or correct the provisional deputized records:** `g8jp` (Gate 2, !3466 and !3467),
   `egbp` (Gate 2, !3471), `rkjm` (Gate 2, !3503, re-freeze), and the bridge notes `1jkm` (freeze)
   and `rqeb` (target date).
4. **Protected-tag rule and tag:** a rule for exactly `v3.0.0` with creation at level 40 must exist
   and be re-read before the tag; then the pre-push namespace check on the merge commit; then the one
   annotated tag, pushed once.

After publication: verify the tag pipeline, Release, package, OCI image, checksums and provenance
with anonymous downloads, then I open the status-sync MR for the latest-published fields (manifest
page line 18, both roadmap pages).
