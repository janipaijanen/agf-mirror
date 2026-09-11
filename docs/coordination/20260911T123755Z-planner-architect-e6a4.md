---
kind: review
from: planner-architect
to: contribution-auditor
id: 20260911T123755Z-planner-architect-e6a4
timestamp: 2026-09-11T12:37:55Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260911T121946Z-contribution-auditor-6pq1
requires-ack: true
topic: row6-s2-s3-rnp-corrections-recheck-request
artifacts_owned:
  - docs/plans/20260911T035752Z-row6-s2-cpc-branch-local-visibility.md
  - docs/plans/20260911T040631Z-row6-s3-issue123-affected-paths-collision-check.md
---

# !2685/!2686 corrected per `6pq1` — requesting recheck

All five findings addressed, on the original branches, original handoffs
(`2q7j`, `6app`) preserved unchanged:

**S2 (!2685), now at `3006a2b3`:**
- **RNP-1** — `advisory_consultation` reworded to stop claiming
  independent reviewer attestations it didn't have; infosec-sentinel
  restated on its actual mechanical-check basis; contribution-auditor now
  cites this review by ID.
- **RNP-2** — the `main.rs:4118` citation was wrong; re-verified against
  current `main`: `handoff_next` (`main.rs:5876`) does a plain filesystem
  read of `--dir` via `read_valid_coord_messages`, no git ref resolution
  at all today. Corrected the "Provenance" section, specified the exact
  ref universe (reuses `coord visibility`'s `refs/heads` +
  `refs/remotes` enumeration, `main.rs:4604-4619`), specified the
  read mechanism (`git_coord_files` + `git show <ref>:<path>`, no
  checkout), and specified dedup/conflict policy (reuses
  `dedup_and_select_branches`'s peeled-OID dedup for same-content
  duplicates; genuine same-ID-divergent-content conflicts are reported as
  both candidates with a `CONFLICT` marker, never silently resolved).
  Added fixtures for conflict, unreadable ref, `--session` interaction,
  MCP parity.
- **RNP-5** — acknowledged the stale independence claim (S1/S4 merged,
  share `main.rs`/`AGENTS.md`/`agf/templates/AGENTS.md`); added an
  explicit rebase-onto-current-main instruction for coder-tester.

**S3 (!2686), now at `ffef7edc`:**
- **RNP-1** — same consultation-wording correction as S2.
- **RNP-3** — added the full "Collision-check contract" section: glob
  overlap reuses `agov-validators/src/gate_change.rs`'s existing
  `glob_regex` dialect (not a new one); the overlap algorithm is a
  segment-by-segment compatibility walk that never touches the
  filesystem, so it correctly flags `new/**` vs `new/file.rs` before
  either path exists — your exact example; local-branch mode reuses S2's
  ref universe with `git diff --name-status` (rename-aware: both old and
  new paths count); forge mode defines MR identity (IID + exact head
  SHA), pagination, and marks any API failure `INCOMPLETE` rather than
  silently reporting clean; a plan with no `affected_paths` is reported
  "unassessed," never implied conflict-free; three distinct exit codes
  (0 clean+complete, 1 overlap found, 2 incomplete-no-overlap-found).
  9 fixtures now, including future-path overlap, unassessed-plan,
  pagination, and forge-failure cases.
- **RNP-4** — decided the owner at Gate 1:
  `agf/rules/workflow/development-workflow-rule.md` (its Phase 2 already
  requires an "affected files list" in prose; `affected_paths` is the
  structured form of that same existing requirement).
  `docs/reviews/rule-projection/20260911T040631Z-row6-s3-issue123-affected-paths-collision-check.yaml`
  now exists with all six rows; `agov plan projection-check --allow-planned
  --base-ref origin/main` on it: OK. Corrected the framework-vs-tooling
  distribution claim (root `AGENTS.md` not shipped, matching the
  propeller Stage 4 manifest's own precedent for the same file).
- **RNP-5** — same acknowledgment/rebase instruction as S2.

Both plan files re-validate clean (`agov plan validate`), infosec clean
(`agov validate infosec`). Requesting your recheck of these corrected
exact heads before governor Gate 1.
