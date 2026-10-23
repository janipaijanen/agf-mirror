---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20261023T135456Z-coder-tester-pm7t
timestamp: 2026-10-23T13:54:56Z
model: gpt-6.1-sol
immutable: true
references-to:
  - 20261023T112836Z-governor-reviewer-dhdh
  - 20261023T132439Z-coder-tester-6hag
topic: mr3160-measurement-acceptance-amendment-request
requires-ack: true
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3160
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Decide whether to replace the unavailable measured startup-report character condition for MR !3160 exact head fe94fa8a with the five consensus acceptance conditions in this record."
    - "If accepted, either make the exact-head Gate 2 decision in the same governor record or request a final state-only coder check."
  must_read:
    - docs/plans/20261023T080431Z-instruction-context-followup.md
    - docs/coordination/20261023T132439Z-coder-tester-6hag.md
  do_not_touch:
    - agf/VERSION
    - any release branch or tag
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Human Governor decision requested: amend MR !3160 measurement acceptance

Codex and Claude reached advisory consensus on the smallest honest replacement
for the unobservable character-report condition in approved plan
`20261023T080431Z-instruction-context-followup`.

## Why an amendment is needed

The plan deliberately made an at-most-140.0k *measured startup-report character
count* mandatory and rejected ratio estimates. Claude Code 2.1.309 does not
print that processed-character count after the loaded instruction set falls
below its combined warning threshold.

Current official Claude Code memory documentation describes:

- `/context` as the loaded memory-file list;
- `InstructionsLoaded` as file, time, and reason logging;
- startup and `/status` warnings when an individual or combined limit is
  exceeded; and
- `wc -m` as the operator character-count surface.

No documented surface reports the below-threshold processed character count.
Source checked 2026-10-23: <https://code.claude.com/docs/en/memory>.

Review `20261023T132439Z-coder-tester-6hag` therefore correctly kept the
criterion blocked instead of substituting an estimate without authority.

## Exact-head replacement evidence

Candidate: MR !3160 exact head
`fe94fa8adbfaca2622295a0de01ff418e5a4303e`.

Claude measured with Claude Code 2.1.309, model `haiku`, prompt `reply ok`, and
detached worktrees whose paths had equal length. The reported total is:

`input_tokens + cache_creation_input_tokens + cache_read_input_tokens`.

Three runs on each side were identical:

| Surface | Main baseline `7588a7c5` | Exact head `fe94fa8a` | Delta |
|---|---:|---:|---:|
| startup input-token total | 81,179 | 74,125 | -7,054 (-8.69%) |
| loaded instruction files | 33 | 27 | -6 |
| raw loaded-set characters | 156,887 | 136,404 | -20,483 (-13.06%) |

The exact head starts interactively without the combined-limit warning.
`/context` separately reports the memory-file token display falling from 53.6k
to 46.8k. The six situation-trigger replays pass after the final trigger edits.
Exact-head pipeline `2981300258` succeeded.

## Consensus recommendation

For this plan and exact head, replace the unavailable processed-character
condition with all of these acceptance conditions:

1. `InstructionsLoaded` confirms the expected exact 27-file loaded set.
2. The exact raw loaded-set sum is at most 140,000 characters.
3. Controlled startup input-token total falls by at least 6,500 tokens and at
   least 8% against the exact main baseline, with the formula above.
4. Three runs per side are stable and startup shows no combined-limit warning.
5. Required situation-trigger replays and exact-head pipeline pass.

MR !3160 satisfies all five. This amendment does not authorize any further
scope, release, tag, or later measurement substitution. The Human Governor
retains Gate 2 and merge authority.

## Decision requested

Please record one of:

- accept the replacement acceptance conditions above, which resolves the sole
  open review finding for this exact head; or
- reject them and keep MR !3160 blocked pending a different supported
  measurement.

If accepted, the Governor may make the exact-head Gate 2 decision in the same
record or request one final state-only coder check first.
