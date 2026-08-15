---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260814T213504Z-planner-architect-d833
timestamp: 2026-08-14T21:35:04Z
immutable: true
model: claude-sonnet-5
references-to:
  - 20260814T074427Z-governor-reviewer-33br.md
topic: frozen-scope-rule-implementation-review
requires-ack: true
artifacts_owned:
  - agf/rules/workflow/release-via-merge-request-rule.md
  - .claude/rules/workflow/release-via-merge-request-rule.md
  - agf/docs/site/docs/operate/review-and-release.md
  - agf/skills/release/SKILL.md
  - agf/rules/workflow/branch-and-worktree-rule.md
  - .claude/rules/workflow/branch-and-worktree-rule.md
  - CHANGELOG.md
  - agf/CHANGELOG.md
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Independently review this implementation against the Gate-1-approved
      plan's "Proposed rule text" section — especially the four judgment
      calls below, which are NOT a mechanical copy-paste and deserve
      independent scrutiny.
    - Confirm no substantive requirement from the approved plan was lost,
      weakened, or silently changed in the process of cleaning up the text.
    - Confirm the branch-and-worktree-rule.md fix (found during
      implementation, not in the plan's own six-row projection screen) is
      correct and complete.
  must_read:
    - docs/plans/20260813T155743Z-frozen-scope-reconstruction-rule-amendment.md
    - docs/reviews/rule-projection/20260813T155743Z-frozen-scope-reconstruction-rule-amendment.yaml
    - agf/rules/workflow/release-via-merge-request-rule.md
  do_not_touch:
    - .claude/settings.local.json
---

# Gate 1 → Gate 2: frozen-scope reconstruction exception implementation, review requested

Jani authorized implementation in-session ("proceed as you proposed, and
coordinate and collaborate with codex") after MR !1503's Gate 1 approval
(`33br`) merged. This is that implementation — not yet pushed as an MR,
branch `feat/frozen-scope-reconstruction-exception-implementation` from
current `main` (`22fa58bb`). Requesting your independent review before I
open the MR, per this project's practice of not self-declaring Gate 2
review complete.

## What changed (8 files)

1. `agf/rules/workflow/release-via-merge-request-rule.md` — new
   "Frozen-scope reconstruction exception" section, inserted after
   "Maintenance-line exception", before "## Exceptions".
2. `.claude/rules/workflow/release-via-merge-request-rule.md` — same
   section, mirrored (foundry's local projection).
3. `agf/docs/site/docs/operate/review-and-release.md` — new adopter-facing
   subsection, business/admin audience, links to the canonical rule.
4. `agf/skills/release/SKILL.md` — Step 0 gains a third named path and
   its own eligibility gate.
5. `agf/rules/workflow/branch-and-worktree-rule.md` — version-bump
   wording extended (see judgment call 4 below).
6. `.claude/rules/workflow/branch-and-worktree-rule.md` — same fix,
   mirrored.
7. `CHANGELOG.md` — repo-level entry, detailed, cites both residual-risk
   decisions by ID.
8. `agf/CHANGELOG.md` — adopter-facing entry, shorter, links to the Pages
   subsection anchor.

## Four judgment calls — please scrutinize each independently

**1. I did NOT copy the plan's "Proposed rule text" verbatim.** The
plan's own text (lines ~285-766 at merged head `a4249a10`) cites
plan-review-process artifacts throughout: round numbers ("round-twelve
infosec-aid `2egn`'s `B-FSR-SEC-R12-1`"), coder-tester finding IDs, and
"acceptance criterion 20 below." None of these exist or mean anything
outside the plan document — the actual rule file has no "acceptance
criteria" section, and a future reader of the rule has no way to look up
`B-FSR-SEC-R12-1`. I confirmed this isn't how the existing
"Maintenance-line exception" section reads either (checked before
writing — it cites zero round IDs, zero finding IDs). So I stripped
every review-round/finding-ID citation, keeping the substantive rationale
as plain prose. Please verify I did not accidentally drop a *requirement*
along with a citation — the two are supposed to be separable but this is
exactly the kind of edit that's easy to get subtly wrong at this volume.

**2. I resolved two now-stale "open decision" passages instead of just
copying them.** The plan's text (written before Gate 1) describes the
release-construction merge's target-drift residual as "a named open
decision for governor Gate 1 (acceptance criterion 20 below), not
resolved by this text." That's now false — `33br` resolved it. I rewrote
both occurrences (in "Protected target and source" step 6, and in "Fail-
closed behavior") to state the resolution directly, citing
`20260814T074427Z-governor-reviewer-33br.md` by ID — the same pattern the
plan's own text already uses for `apkm` in the metadata-forward-sync
section. Please confirm the rewritten text still accurately states
`33br`'s actual conditions (I did not weaken or broaden them — same
boundaries, same "this release-construction merge only" scope, same
platform-reassessment condition for future reuse).

**3. I generalized one platform-tier-specific sentence.** The plan's
text states as present fact: "This project's GitLab plan tier does not
expose merge-train or merged-results pipelines... do not re-investigate
this per-application." A *rule* file describing a specific project's
current platform tier as permanent fact seemed wrong to me — tiers
change, and this rule is meant to stand independent of this project's
current GitLab plan. I rewrote it as conditional: "Where the platform
tier in use does not expose a merge-train, merged-results pipeline, or
equivalent serialization control that would close this window, the
residual risk is bounded, not eliminated..." Please confirm this doesn't
weaken the actual requirement (step 7's post-merge verification is still
mandatory either way) — I read it as purely making the rule text
evergreen, not changing what's required.

**4. I found and fixed a gap the plan's own six-row projection screen
didn't name.** `branch-and-worktree-rule.md` already says version bumps
happen only on `main` or on the maintenance-line target — that line was
added when the maintenance-line exception shipped, per its own CHANGELOG
entry. The frozen-scope reconstruction exception also produces a version
bump off `main` (on the reconstruction source/target branches), so the
same sentence needed the same treatment or it would read as (wrongly)
excluding the new path. Fixed in both the canonical file and its
`.claude/` mirror. This file was not in the plan's declared
`changed_pages_or_runbooks` list — flagging explicitly since it's outside
what Gate 1 reviewed.

## Verification already run

- `git diff --check`: clean.
- `agov dist validate`: OK, 244 files, manifest unchanged in shape.
- `agov validate adopter-docs`: OK.
- `agov plan projection-check --base-ref origin/main
  docs/reviews/rule-projection/20260813T155743Z-frozen-scope-reconstruction-rule-amendment.yaml`:
  OK — this pre-declared projection record (written at plan-authoring
  time) matches what I actually implemented without needing changes,
  which is itself worth double-checking rather than taking as
  self-confirming.
- Grepped all 8 files for leaked plan-process artifacts (`acceptance
  criterion`, `round-*`, `B-FSR`, individual coder-tester coordination
  IDs): zero hits.

## Where to review

Pushing this branch and opening the MR in the same batch as this
coordination record, so you have a live diff to review rather than my
description of one. MR number and exact head follow in a same-topic
coordination note once the push completes (GitLab MR IIDs aren't known
until push time). Please review the live MR diff directly rather than
relying on this handoff's prose summary of the four judgment calls above
— the summary is orientation, not a substitute for reading the actual
text.
