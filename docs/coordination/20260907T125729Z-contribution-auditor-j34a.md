---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260907T125729Z-contribution-auditor-j34a
timestamp: 2026-09-07T12:57:29Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260907T124837Z-coder-tester-q4p3.md
  - 20260907T062346Z-contribution-auditor-a8pj.md
  - 20260907T074750Z-contribution-auditor-8tqg.md
topic: v210-ruw1-mr2500-exact-head-contribution-audit
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Independent exact-head audit — v2.10 RUW-1 (MR !2500 @ `79de28a4`)

Per `q4p3` (handoff to contribution-auditor). Reviewed the complete
four-file diff `origin/main..79de28a43b71563b544c12c889428d51a9d7fc8d`
against RUW-1 in the approved plan
`docs/plans/20260907T045703Z-v210-release-plan.md`.

## Coverage: COMPLETE. Result: CLEAR — 0 blocking, 2 advisory.

### Verified

- **Scope stays inside the RUW-1 allowlist** — only `README.md`,
  `CHANGELOG.md`, `agf/CHANGELOG.md`, and
  `docs/reviews/rule-projection/20260905T210255Z-universal-agent-autonomous-polling.yaml`
  change. No scope creep.
- **README count fix (advisory A1 from `a8pj`/`qhbd`)** — `48` → `50`
  governance rules, matching `agov validate readme_stats` at this head
  (now OK; was WARN "actual 50"). Correct.
- **`q4p3` check — no second normative owner.** `.claude/CLAUDE.md` is a
  pure `@../AGENTS.md` import; the `.claude/rules/always/universal-polling-
  and-autonomous-continuation-rule.md` pointer (see A2) is 11 lines of
  "read and follow the canonical rule ... does not copy, narrow, or add."
  Neither is a normative owner.
- **`q4p3` check — no false background-execution claim.** Both changelogs
  state "Assistants without a real wake path poll on activation or resume
  rather than claiming background execution." Matches RUW-1 acceptance #3.
- **`q4p3` check — changelogs agree.** The `### Added` / `### Changed`
  blocks are content-identical between `CHANGELOG.md` and
  `agf/CHANGELOG.md` (one trivial formatting difference: `agf/CHANGELOG.md`
  bolds "**Adopter docs: updated.**", root does not).
- **Mechanical gates green at the exact head** (re-run in an isolated
  worktree): strict `agov plan projection-check --base-ref origin/main`
  (exit 0), `agov validate readme_stats`, `assistant_loader`,
  `adopter_docs`, umbrella `agov validate`, and `git diff --check` all
  pass. `assistant_drift` WARN stays 53 (pre-existing; this MR touches no
  `.claude/rules` file). Canonical pipeline `2826544773` is the required
  CI evidence (running at review time).

### Advisory findings

**RUW1-A1 — the changelogs describe the feature but not the
downstream-impact BREAKING axis or migration.** The revised plan's A2
disposition (confirmed in `8tqg`) required RUW-1's changelog to "state both
axes" — SemVer MINOR (backward-compatible) **and** downstream-impact
**BREAKING** for direct adopters (a new MUST-level assistant duty) — with
migration prose. The entries here give "Adopter docs: updated" and describe
behavior, but carry no impact classification and no migration steps
(record the last completed capability-review learning cursor/count;
configure a real wake path or state that polling occurs on resume). RUW-1
acceptance #3 says the changelogs must agree on "capability and **migration
meaning**"; the migration meaning is currently thin. Recommend a short
impact/migration line in the `### Changed` block or a migration note.
Whether this is blocking is the governor's read of acceptance #3.

**RUW1-A2 — the projection-YAML enumerates `.claude/CLAUDE.md`, not the
pointer file the advisory named.** The advisory RUW-1 was chartered to
close (`8tqg` / `qhbd` A2) specifically named
`.claude/rules/always/universal-polling-and-autonomous-continuation-rule.md`
— the 11-line foundry pointer MR !2463 added — as "not enumerated in
`ai_landing_loaders` (appears only in `distribution` prose)." This change
adds `.claude/CLAUDE.md` to `ai_landing_loaders.paths` instead; the
specifically-named pointer file is still listed only in the `distribution`
row. `.claude/CLAUDE.md` (a pure loader import) is a reasonable entry and
strict `projection-check` passes, but the disposition does not
unambiguously close the exact gap it targeted. Recommend either confirming
`.claude/CLAUDE.md` was the intended reading of "the foundry-only Claude
pointer" (transitive-loader coverage, and `assistant_drift` already tracks
the rule-tree file), or adding
`.claude/rules/always/universal-polling-and-autonomous-continuation-rule.md`
to the paths as well.

## Disposition

CLEAR for RUW-1's exact-head Gate 2 subject to the governor's read of
RUW1-A1. Both advisories are text-only fixes on the same branch; neither is
a scope or safety issue. The canonical pipeline `2826544773` must be green
before merge.
