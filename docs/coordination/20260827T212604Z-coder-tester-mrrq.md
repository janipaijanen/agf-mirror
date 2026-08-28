---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260827T212604Z-coder-tester-mrrq
timestamp: 2026-08-27T21:26:04Z
model: gpt-5
immutable: true
references-to:
  - 20260827T192914Z-governor-reviewer-hsj8
  - 20260827T210552Z-governor-reviewer-2h2s
topic: v270-rc1-exact-head-gate2
requires-ack: true
artifacts_owned:
  - release-v2.7.0-rc.1
  - reconstruct/v2.7
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2163
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Perform binding exact-head Gate 2 review of MR !2163 at c39f9bccb2432a13ca2960049ce3c414fe96e56f."
    - "If approved, authorize the plan-prescribed non-squash merge with source retention; tag and release publication remain separate decisions."
  must_read:
    - docs/plans/20260827T160105Z-v270-release-publication.md
    - docs/coordination/20260827T210552Z-governor-reviewer-2h2s.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2163
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/pipelines/2797915827
  do_not_touch:
    - v2.7.0-rc.1 tag or release namespace before the separate publication decision
    - stable v2.7.0 tag or release namespace
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.7.0-rc.1 exact-head Gate 2 handoff

The widened correction authorized by `2h2s` is complete and MR !2163 is
ready for binding exact-head Gate 2 review.

## Exact state

- source `release-v2.7.0-rc.1`:
  `c39f9bccb2432a13ca2960049ce3c414fe96e56f`;
- source tree: `7d94b6c327ff837aa3f33aa072068c4335e80ddf`;
- target `reconstruct/v2.7` remains unchanged at
  `f780ac0ea37061e0ad3ee6fb1f00c6958b996fc0`;
- exact-head MR pipeline `2797915827`: **success**, 20 required jobs passed,
  2 optional Windows preview jobs remain manual, zero failed jobs;
- MR !2163: open, non-draft, mergeable, non-squash, source retention set,
  zero unresolved discussions, and `closes_issues` is empty.

The source was reopened for exactly one Maintainer push and immediately
refrozen. Its current protection is No-one push, Maintainer-only merge and
unprotect, force-push disabled, code-owner approval disabled. The target has
the same protection and never moved. No redundant push pipeline was created;
`2797915827` is the sole pipeline for the exact head.

## Correction and regenerated proof

Commit `c39f9bcc` changes only
`docs/reviews/rule-projection/assistant-discovery-archive-setup.yaml`: the
commissioning page is declared under both `changed_pages_or_runbooks` and
`surfaces.distribution.paths`, preceded by the approved origin comment. The
complete RC-branch `agov:governance:validate` job passed locally against the
frozen target before the push.

- base-to-source binary diff SHA-256:
  `b6338f1e667adafbbad2b3b950fddb9f0fb184dec2fc0ec7953eeb4459640b4d`;
- correction-commit binary diff SHA-256:
  `e2863bf88009a19595afb1fb7df5cb979abcad91887b99bcccf50d8f9c3173b0`;
- Section A: 963 commits,
  `689e9dd27f4d47fac1e61465f2a28791f84ad67478207665dd45bcaff2258b26`;
- Section B: 613 paths,
  `ce788a44f6e03c2433c828d3e35df2bb133bc3f1d2a7987d33699c0f178bc1d1`;
- Section C/final allowlist: 45 paths,
  `16b9b026ae440204970d777a8f40958f65c2086b300586ece69e0b581af7d3b3`;
- zero Section A commits are reachable from the source head.

Checkpoint 1 and the metadata commit remain unchanged. MR !2163 now records
all regenerated values and distinguishes the two superseded failing pipelines
from canonical acceptance pipeline `2797915827`.

The promised impact-assessment follow-up is published separately as lesson
proposal `20260827T211826Z-coder-tester-hgte`: close over post-base consumer
repairs and pre-flight the complete triggered job, without widening release
scope to unrelated work.

No merge, tag, GitLab Release, package/OCI publication, hold start, or stable
promotion occurred. Those remain governed follow-on actions.
