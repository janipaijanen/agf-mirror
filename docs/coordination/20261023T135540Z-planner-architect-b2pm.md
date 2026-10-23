---
kind: note
from: planner-architect
to: governor-reviewer
id: 20261023T135540Z-planner-architect-b2pm
timestamp: 2026-10-23T13:55:40Z
immutable: true
references-to:
  - 20261023T112836Z-governor-reviewer-dhdh.md
  - 20261023T132439Z-coder-tester-6hag.md
topic: gate2-3160-amendment-plan-merges
requires-ack: true
artifacts_owned: []
---

# Bridge record: live amendment, Gate 2 and merge approvals, 2026-10-23

Provisional. Not binding until a governor-reviewer record references it.

I put five numbered decisions to the human governor in a live session. The reply was:
"accept. approved - merge. merge. approved. proceed." My reading, which the
governor can correct, answers them in order:

1. **Amendment accepted** for the acceptance condition of plan
   `20261023T080431Z-instruction-context-followup` (record `dhdh`), replacing the
   unobtainable "measured startup-report chars" with: loaded-file set confirmed;
   loaded-set count at most 140,000 chars (measured 136,404 at `fe94fa8a`);
   controlled exact-head startup-input total down at least 6,500 tokens or 8%
   (measured 81,179 to 74,125, down 7,054, 8.69%, three equal runs each, formula
   input_tokens + cache_creation_input_tokens + cache_read_input_tokens, Claude Code
   2.1.309); no combined-limit warning. Processed-character output is unavailable
   in Claude Code 2.1.309 and the current official docs.
2. **Gate 2 approved and merge authorized** for MR !3160 at exact head
   `fe94fa8adbfaca2622295a0de01ff418e5a4303e` (chore/instruction-context-followup to main).
   Codex's only open blocker (`6hag`) was that condition.
3. **Merge authorized** for the two approved plan MRs: !3144 at exact head
   `04ab395d911ab7536a5776b242e5ec260e9c5d31` and !3146 at exact head
   `e9b346b56d2a09afc32f2c130f4a56f571cef497` (both status approved under `dhdh`).
4. Shelved split MR !3163: not closed. It stays Draft.
5. Recall-suite corpus: read as an instruction to draft a Gate 1 plan, **not** a
   Gate 1 approval of a plan that does not exist yet.

Any new head on these MRs voids this authorization for that MR. It is captured here
before the irreversible merges. No delegation is used: `qpda` expired with 0 of 5.
