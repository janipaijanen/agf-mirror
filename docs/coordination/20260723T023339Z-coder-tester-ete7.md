---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - tooling/elixir/mix.lock
  must_read:
  - docs/coordination/20260723T012716Z-governor-reviewer-re1q.md
  next_agent: contribution-auditor
  status: ready
  task:
  - Cross-check this MR triage; flag disagreements before I start acting on
    it
id: 20260723T023339Z-coder-tester-ete7
immutable: true
kind: handoff
model: claude-sonnet-5
references-to: []
requires-ack: true
timestamp: 2026-07-23T02:33:39Z
to: contribution-auditor
topic: open-mr-triage-post-v240
---

# Open-MR triage: which can close after enrichment, which need Gate work

Jani asked me to jointly assess the ~19 remaining open MRs (post the three
I just merged directly: !443 Class 5 Gate 2 review, !444/!445 my own
coordination completion records — all pure documentation of already-
resolved decisions, verified their content wasn't on `main` before
merging) and identify which can naturally close after extracting a
learning record or improvement idea, then start on those. My read below —
please cross-check before I act, especially on anything you'd classify
differently.

## Group A — real fixes/docs, low risk, recommend merge as-is

- **!340** `fix/legacy-ulid-reference-compat` — recognizes legacy 20-char
  Crockford IDs for coordination reference resolution. Self-contained bug
  fix, 6 commits, 8 files. No obvious reason to hold.
- **!360** `chore/agov-mcp-install-parity` — aligns agov-mcp install
  source/hashes. Single commit, narrow chore fix.
- **!341** `docs/adr-tracker-coexistence-page` — explicitly already
  "fresh branch, per Gate 2 BLOCK" in its own title, meaning a prior
  attempt was blocked and this is the corrected resubmission. Worth a
  quick Gate 2 re-check before merge, not a full re-review.
- **!117** `docs/academic-tex-paper-rebase` — oldest MR here (updated
  2026-07-21T16:29, before this whole release cycle started). Single
  commit, 3 files. Given `docs/plans/v2.4.0-release.md` explicitly lists
  "MR !117 in its current form" under **Explicitly out of scope** ("its
  academic snapshot remains stale and requires a separately governed
  refresh"), I don't think this is a quiet-merge candidate — flagging for
  your read rather than acting on it myself.

## Group B — learning records, should merge (that's their whole purpose)

- **!349** `learn/coordination-before-bridge` — require coordination
  before bridge notes.
- **!358** `coord/adversarial-boundary-and-gitlab-channel` — jointly
  formalize boundary and command-channel learnings.
- **!407** `learning/ci-cost-aware-agent-push-discipline` — make agent
  push cadence CI-cost-aware.

All three look like exactly the "enrichment" Jani meant — durable learning
records that should land on `main`, not sit open. Recommend merging all
three once you confirm no open finding blocks them; I haven't independently
re-verified each one's validator status yet.

## Group C — plan/governance proposals, need Gate 1 before merge, not closeable yet

- **!361** `plan/public-experience-release-review`
- **!430** `feat/tooling-doc-impact-rule`
- **!434** `coord/joint-review-taxonomy-planning`

None of these are "close after enrichment" candidates — they're active
proposals awaiting a governor Gate 1 decision. Leaving these alone unless
you disagree.

## Group D — small post-release queue notes, safe to merge as records

- **!346** `coord/clickable-fullres-figures` — queues post-release work,
  doesn't implement it.
- **!347** `coord/homepage-tooling-ia-post-release` — same pattern.
- **!400** `coord/dossier-reference-layer` — requests a Gate 1 plan, not
  implementation.
- **!402** `coord/post-workstream-d-marketing-audit-reassessment` —
  reassessment note, not implementation.

These are just durable records of "do this later" — recommend merging as
coordination-only, same as the three I already merged.

## Group E — large, actively evolving, do not touch without your/governor sign-off

- **!403** `coord/claude-next-expectations` — 55 files, 52 commits ahead
  of `main`. This is the live coordination trunk carrying `re1q`, `krna`,
  `qkks`, `3d27`, and everything else driving current priority. I am
  **not** merging this myself — it's still being actively appended to and
  merging it mid-stream risks a conflict with your next push. Flagging
  only.
- **!350** `coord/postrelease-marketing-audit` — 28 files, 25 commits.
  Same reasoning: actively evolving (I pushed to it twice this session),
  not a close-after-enrichment candidate.
- **!438** `coord/propose-codex-read-efficiency-doc` — 5 commits, appears
  to be an active back-and-forth, not a closed thread.

## Group F — possibly already superseded, candidate for extract-then-close

- **!413** `coord/eli5-critique-step-order` — single commit, "critical
  review of ELI5 step order and placement (r4ag)". The step-order question
  itself already got a full answer in `j7gp` (my own earlier response,
  merged separately) and further refined by `4qq2` and `mj2g`. If `!413`'s
  content is the same critique already folded into `j7gp`, this looks like
  a genuine "extract the useful part, then close without merging"
  candidate rather than a merge candidate — but I haven't diffed it
  against `j7gp` closely enough to be sure it isn't the earlier draft that
  `j7gp` was actually answering. Your call before I touch it.

## !354 — separately, not triaged above

`coord/postrelease-marketing-audit-findings`, 1 commit. Referenced
repeatedly from `!350`'s description as the consolidated Codex marketing
audit. Given `!350` is still active, leaving this alone too unless you
say otherwise.

## Proposed next action

With your ack (or corrections), I'll start on: merge Group B (learning
records) and Group D (queue notes) directly — lowest risk, matches
"enrichment already done, just needs to land." I'll hold Group A's !341
and !117 for your explicit read given the Gate 2 BLOCK history and the
release plan's explicit out-of-scope note. I won't touch Group C or E.
Group F (!413) I'll diff against `j7gp` myself and report back rather than
waiting on you for that one specific check.
