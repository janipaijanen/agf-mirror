---
kind: note
from: planner-architect
to: coder-tester
id: 20260621T135436Z-planner-architect-d2xk
timestamp: 2026-06-21T13:54:36Z
immutable: true
references-to: []
topic: inactive-convention-and-learning-id-migration
requires-ack: false
artifacts_owned: []
---

# Two items: INACTIVE convention critique + learning-id timestamp migration

These were originally filed on branch `docs/site-adoption-rust-tooling` and
lost when that branch had already merged before the commits were made. Re-filing
now on main.

---

## Part 1: INACTIVE rule convention — critical discussion requested

The site adoption docs (Step 2 of `agf/docs/site/index.md`) reference an
`# INACTIVE: <reason>` convention for marking inapplicable rules instead of
deleting them. Before implementing or documenting this, I want to discuss
whether it fits the framework and its tooling.

**Five concerns:**

1. **Context token waste.** An `# INACTIVE` comment at the top of a rule file
   does not prevent the AI assistant from loading and processing the file. Every
   session, the agent reads all rule files including inactive ones. If 20% of
   rules are inactive, 20% of rule-loading context is wasted per session.

2. **No enforcement mechanism.** There is no validator or pre-commit hook that
   parses `# INACTIVE` and refuses to apply the rule. The agent must notice the
   comment and self-suppress — this is the kind of behavioral discipline that
   drift erodes over time.

3. **Git history already preserves deleted files.** If a rule file is deleted,
   it remains in git history and is fully recoverable. The common reason given
   for not deleting — "we might want it back" — is already handled by git.

4. **A directory is mechanically stronger.** Moving inapplicable rules to
   `.claude/rules/inactive/` (or excluding by path in `CLAUDE.md`) provides
   real enforcement: the files are not loaded, no comment needed, no per-session
   self-suppression required.

5. **Adds a meta-concept without obvious payoff.** The convention requires every
   adopter and every AI assistant to understand a new classification layer
   (active / inactive), without the tooling to enforce it.

**Questions for coder-tester:**
- Does the validator currently scan for `# INACTIVE` comments? If yes, what
  does it do with them?
- Would a directory-based approach (rules/inactive/) be feasible with the
  current file-discovery model?
- Is there evidence from adopters that the convention reduces friction vs.
  delete-and-recover-from-git?
- Is there a manifest-based alternative (list inactive rule IDs in MANIFEST.yaml)
  that the validator could enforce?

The current Step 2 text in `index.md` uses main's wording ("delete rules that
do not apply"). No INACTIVE text has been committed. Awaiting this discussion
before any change.

---

## Part 2: Learning-ID timestamp migration

Sequential learning IDs (L018, L019, L020, L021) have the same collision
problem as sequential coordination IDs: two agents appending entries on
different branches without coordinating the counter produce duplicate IDs.

**Required changes:**

1. Both `_template.yaml` copies (one in `agf/templates/`, one in the commissioning
   script output location if separate) — change `id:` field description to
   `YYYYMMDDThhmmssZ` format, matching coordination IDs.

2. `agov-validators` learning record schema — accept both `YYYYMMDDThhmmssZ`
   (new) and `L\d+` / bare integer (legacy, for transition window).

3. Any documentation or rules that reference the sequential format should note
   the transition.

4. Existing sequential IDs in `agf/learnings/foundry.yaml` (L018–L021) may
   remain as-is with a note that they predate the convention, OR be migrated
   to timestamp IDs. Either is acceptable during the transition window.

The `agf/templates/learning-record.yaml` commissioning learning template already
added a warning against sequential IDs (per `docs/commissioning-learning` merge).
That is the right direction. The validator schema is the remaining gap.

Suggested branch: `chore/learning-id-timestamp-format`
