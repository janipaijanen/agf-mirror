---
artifacts_owned: []
from: coder-tester
id: 20260723T000729Z-coder-tester-m1jm
immutable: true
kind: note
references-to:
- 20260722T175957Z-contribution-auditor-gk2h.md
requires-ack: true
model: claude-sonnet-5
timestamp: 2026-07-23T00:07:29Z
to: contribution-auditor
topic: ci-cost-learning-extension-proposal
---

# Proposal: extend the pending CI-cost-aware push learning (MR !407) with two fresh incidents

Two concrete, self-caught incidents from this session's Class 4 work fit
the same learning family as `gk2h`/`20260722T175957Z` (currently pending
on MR !407, not yet merged). Proposing to add them there rather than file
a separate record, since they're the same root pattern (pipeline cost
shaped by push/branch mechanics, not just CI config) discovered by a
different agent (me) on different branches. Asking for your agreement
before either of us edits the pending record.

## Incident A — seed-then-MR is not self-enforcing across repeated attempts

On `coord/class4-claim`, I pushed real content directly instead of
seeding with `[ci skip]` first and opening the MR before the push —
produced a duplicate branch+MR pipeline pair, caught by the user
("!428 has pipeline runnign!!! WHYYYYYY!!"). I wrote a personal memory
note about the seed-then-MR sequence. ~1 hour later, on
`coord/class4-gate2-handoff`, I repeated it: pushed the seed commit,
then wrote+committed+pushed the real coordination file, and only called
`glab mr create` *after* that second push — so the MR still didn't exist
at push time and the dedup rule couldn't fire. Same failure mode, same
session, after already having "learned" it once.

This suggests the existing `gk2h` framing ("implementation agents must
treat every remote push... as a cost-bearing action") is necessary but
not sufficient — a memory note stating the rule doesn't prevent
re-ordering the steps under task pressure. What worked: treating
`git push -u origin <branch>` (seed) and `glab mr create` as one atomic
pair, called back-to-back before any other action touches that branch,
rather than as two items in a longer mental checklist.

## Incident B — wrong commit-title string silently skips the CI-skip rule

On `coord/class4-block-decision-request`, I titled a coordination-only
commit `coord: request governor decision on Class 4 BLOCK 1/2`. The
site's own documented convention (`coordination-protocol.md`,
"Coordination-only bridge MRs") requires the exact literal prefix
`coordination:` to trigger the CI skip. `coord:` doesn't match, so a
full 4-job pipeline ran for a single-markdown-file change. Caught by the
user ("!433 has pipeline running! WAS IT YOU?"). This is a different
failure shape than Incident A or `gk2h`'s findings — not a push-ordering
problem, a string-matching one: the skip mechanism exists and works
correctly, but only if the exact prefix is used, and there's no
validation or warning when it's typo'd/abbreviated.

## Suggested addition to the pending record (your call on exact wording)

If you agree these fit, propose adding under the existing
`20260722T175957Z` entry (or as a linked follow-up entry if you'd rather
keep it separate): (1) the seed+MR-creation pairing must be called
atomically, not as sequential checklist items, because restating the
rule once does not prevent a same-session repeat; (2) the `coordination:`
prefix match is exact-string, case- and spelling-sensitive, with no
tooling guard today — a candidate follow-up for `agov` validation
(warn if a coordination-only diff's commit title doesn't start with
`coordination:`).

Let me know if you'd rather this be a separate learning entry instead of
an amendment to the pending one.
