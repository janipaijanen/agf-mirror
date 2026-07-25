---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260725T183500Z-planner-architect-4iae
timestamp: 2026-07-25T18:35:00Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260725T182100Z-planner-architect-9bj1
topic: v243-release-cut
requires-ack: true
artifacts_owned:
  - CHANGELOG.md
  - README.md
  - agf/CHANGELOG.md
  - agf/VERSION
  - agf/docs/site/docs/adopt/install-and-layouts.md
  - agf/docs/site/docs/adopt/tooling-and-tokens.md
  - agf/docs/site/docs/advanced/contracts-and-validators.md
  - agf/templates/AGENTS.md
  - tooling/rust/Cargo.lock
  - tooling/rust/Cargo.toml
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - "Review MR !533 (release: prepare v2.4.3) at the pushed SHA — the only diff is the release-branch metadata (VERSION, Cargo.toml/Cargo.lock, CHANGELOG.md/agf/CHANGELOG.md, six OCI version-pin literals, agf/templates/AGENTS.md's version comment). No feature content changes on this branch — all four content items (#64, #65, #66, MR !483/!527) were already merged to main before this branch was cut."
    - "Verify the CHANGELOG entries accurately summarize what merged (cross-check against the commit list in the coordination note below)."
    - "Confirm no version-pin literal was missed by re-running the grep the plan's item 1 used."
  must_read:
    - docs/plans/v2.4.3-release-content-assessment.md
    - 20260725T182100Z-planner-architect-9bj1
  do_not_touch: []
---

# v2.4.3 release branch cut, per governor's go-ahead

### Claude · planner-architect

Governor authorized proceeding ("bake the release, coordinated and
collaborated"). Cut `release-2.4.3` from `main` at `dfe1528` (post-MR !527
merge, which was the last open content prerequisite). This branch carries
release metadata only, per `release-via-merge-request-rule.md`:

- `agf/VERSION`: `2.4.2` -> `2.4.3`
- `tooling/rust/Cargo.toml` + `Cargo.lock`: internal workspace crate versions
  `2.4.2` -> `2.4.3` (5 crates: `agov`, `agov-coord`, `agov-core`,
  `agov-mcp`, `agov-validators`); no external dependency changes
- Six hardcoded `agov:2.4.2` OCI-tag literals (README.md,
  `tooling-and-tokens.md`, `install-and-layouts.md` x2,
  `contracts-and-validators.md` x2) updated to `2.4.3` — the exact six sites
  MR !483's item 1 added canonical-source comments to
- `agf/templates/AGENTS.md`'s version comment: `2.4.2` -> `2.4.3`
- `CHANGELOG.md` and `agf/CHANGELOG.md`: new `[2.4.3]` entries summarizing
  #64 (openable SVG figures), #65 (agov day-one FAQ), #66 (split-person
  gates FAQ), and MR !483/!527's six documentation-quality items

Verification run before this push: `git diff --check` (clean), `agov
validate` (only pre-existing WARNs, no new findings), strict MkDocs build
(passes, only the two pre-existing unlisted-nav-page infos for
`figures.md`/`palette.md`, unrelated to this branch).

#30/MR !463 confirmed excluded per `9bj1`'s re-check (166 commits behind
`main`).

MR !533 opened. Requesting review before governor Gate 2 and the
tag-and-push step.
