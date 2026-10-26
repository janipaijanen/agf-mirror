---
kind: decision-record
from: governor-reviewer
to: all
id: 20261026T180956Z-governor-reviewer-pbgd
timestamp: 2026-10-26T18:09:56Z
model: codex
immutable: true
references-to:
  - 20261026T171229Z-improvement-propeller-khhr
  - 20261026T173119Z-governor-reviewer-3rrh
  - 20261026T174028Z-governor-reviewer-snh2
topic: mr3098-postmerge-gate2-correction
requires-ack: false
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3098
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Correction: MR !3098 merged before its Gate 2 decision

This record transparently corrects the governance trail for MR !3098. It is
written by Codex under the Human Governor's prospective ratification of `khhr`
in `20261026T173119Z-governor-reviewer-3rrh`, but uses that authority only to
record the correction and verified evidence below. It does **not** grant
retroactive Gate 2 approval or retroactively authorize the completed merge.

## Sequence failure

`20261026T174028Z-governor-reviewer-snh2` approved Gate 1 for the
`20261026T170645Z-skills-stale-command-refs` plan. Its timestamp is
`2026-10-26T17:40:28Z`, and its publication MR !3254 became visible at
`2026-10-26T17:42:40.831Z`. The record explicitly said it did not approve
Gate 2 or merge MR !3098 and required a final exact-head Gate 2 check.

The Human Governor merged MR !3098 at `2026-10-26T17:44:28.124Z`, before that
Gate 2 check and decision occurred. The plan MR !3248 merged later, at
`2026-10-26T17:45:11.265Z`. MR !3254 carrying `snh2` merged at
`2026-10-26T17:51:37.388Z`, and the metadata-only plan update MR !3256 merged
at `2026-10-26T18:06:46.264Z`.

The shared `snh2` record supplied visible Gate 1 authority before the product
merge, but it expressly withheld Gate 2. A merge event alone is not a Gate 2
decision. The requested Gate 2 recheck arrived after the irreversible action
and therefore cannot be used as pre-merge authority.

## Exact merge evidence

- Source head: `6fccd783116bef86a908b4e53eadf9fd32d67b14`.
- Merge commit: `dc5523f6d5493b2bacbf3548927cf68d00e5dbc4`.
- Merge parents: prior `main`
  `a2506451d2b9a337d41e1b75a27b31d8beb65559` and exact source head
  `6fccd783116bef86a908b4e53eadf9fd32d67b14`.
- No squash occurred.
- Scope is exactly three files: `agf/skills/README.md` (1 insertion,
  1 deletion), `agf/skills/health/SKILL.md` (25 insertions, 9 deletions), and
  `agf/skills/infosec-screen/SKILL.md` (1 insertion, 1 deletion).
- Exact-head MR pipeline `2973391424` passed all six required jobs:
  `agf:dist:preview` `17359782506`, `agf:dist:validate` `17359782507`,
  `release:paths-validate` `17359782508`, `agov:governance:validate`
  `17359782509`, `rust:agov:validate` `17359782511`, and
  `commercial-corpus:validate` `17359782512`.
- The MR had no blocking discussion and was mergeable before the action.

The post-merge technical review is **CLEAN**: the eleven documented validators
and their order still match `run_all_validators`; no `validate-infosec.sh`
file exists; there is no agent-validator `strict` CLI mode; the deliberately
historical README reference remains historical; and current `main` contains
the intended three-file diff unchanged.

The push pipeline created directly for merge commit `dc5523f6` was pipeline
`2991061531`; it was canceled at `2026-10-26T17:45:42.726Z`. During the
post-hoc review, current-main pipeline `2991138249` at
`564e1bad4d687d8bdbc31fe2b7b7cd64ef70d894` was initially running and then
succeeded at `2026-10-26T18:09:48.135Z`. That later success is useful health
evidence but does not repair the missing pre-merge Gate 2 decision.

## Decision and prevention

MR !3098 remains recorded as technically clean but merged without the required
pre-merge Gate 2 decision. This record must not be cited as retroactive Gate 2
authorization. It also does not decide a revert, release inclusion, tag, or
deployment.

For future normal-path merges:

1. the approved plan and its approval metadata must be visible in the target
   branch before Gate 2 review begins;
2. the Gate 2 reviewer must re-read the MR's live state, exact head, discussions,
   mergeability, and required pipeline immediately before deciding;
3. the Gate 2 decision record must be published through an approved shared path
   and cite the exact head and pipeline before the merge call; and
4. the merge operator must name that pre-existing Gate 2 record in the merge
   evidence. If no such record exists, the merge stops.

This recurrence should be included in the next workflow learning/capability
review as evidence that parallel gate publication and merge activity needs a
single final pre-merge authority check.
