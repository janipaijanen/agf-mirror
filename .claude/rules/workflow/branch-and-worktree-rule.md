<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Git Branch and Worktree Discipline
#
# Framework principle: Boundary enforcement — branch discipline, pre-push quality gate
# See: framework.md — Pattern 1 (Pipeline), Principle 4 (Pipeline Topology)

## Before writing any code for a non-trivial task

**Step 1 -- Confirm current branch**
```bash
git branch --show-current
git status
```
If on `main` or `master`: do not start. Create a branch first.

**Step 2 -- Create a feature branch**
```bash
git checkout -b feat/short-description   # or fix/ refactor/ migration/ security/
```

Naming conventions (extended list):
- `feat/what-it-adds`           -- new feature
- `fix/what-was-wrong`           -- bug fix
- `refactor/what-changed`        -- code quality, no behaviour change
- `migration/table-or-column`    -- schema change
- `security/cve-or-description`  -- security fix
- `docs/what-document`           -- documentation-only change
- `chore/what-mechanical`        -- mechanical / metadata / dependency
- `plan/what-is-planned`         -- plan file (no implementation yet)
- `proposal/what-is-proposed`    -- proposed rule, policy, or design
- `adr/what-decision`            -- architecture decision record
- `infosec/what-evidence-or-fix` -- InfoSec-relevant change or evidence
- `release/X.Y.Z`                -- release-prep branch

Naming is a convention, not a hard rule. Adopters MAY extend or override the prefix list per their tooling; the only hard requirement is *some* prefix that signals the change kind to reviewers.

**Sizing test -- "one yes/no question"**

A branch passes the sizing test if a reviewer can read its diff and answer one question with a yes/no. Examples:

- "Do these scenario edits resolve the four findings of class C?"
- "Is rule X the right project rule?"
- "Should helper Y treat case Z as a finding?"

If a reviewer would have to break the question into "well, on this commit yes, but on that one I'd want to discuss…", the branch is too big. Split it.

The sizing test is SHOULD-strength guidance — reviewer judgement is the final arbiter — but a branch that fails the test reliably produces messy reviews and partial-merge requests.

**Step 3 -- For larger features: use a git worktree**
```bash
git worktree add ../your-app-feat-name feat/short-description
cd ../your-app-feat-name
```
This keeps `main` completely intact. The agent works in the worktree;
the original checkout is never touched.

**Step 4 -- Verify baseline before writing any code**

<!-- CUSTOMIZE: adapt the baseline test command to your project -->
```bash
# Run a fast test suite to confirm baseline is green
```
If tests are already failing: document the failures and do not proceed
without understanding why. Never start on a broken baseline.

## During development

- Commit after each completed micro-task (not after each file save)
- Each commit should be independently passable (no broken intermediate state)
- Never force-push to `main` or any shared branch
- Never commit directly to `main`
- **No version bump or tag on feature branches** -- version bumps happen only during the release workflow on `main`, or on a protected maintenance-line target (`release/X.Y`) under `release-via-merge-request-rule.md`'s narrowly eligible maintenance-line exception

### Multi-commit per branch — when it's OK

Multiple commits per branch is fine **if** they together answer the same yes/no question (the sizing test above). A multi-batch upgrade with eight commits is one MR if the question is "should we adopt version X.Y with this exact split?". A bug fix and a refactor in the same touch are two MRs even if they live in the same file — the reviewer's yes/no on the bug fix should not pull in the refactor.

Test: if a reviewer wants to approve commit A but discuss commit B, and the same branch hosts both, the branch was sized wrong.

### AI-assisted branch state verification

When working alongside a human IDE user, the IDE can switch the working tree branch out from under the agent's expectations. The agent issues `git checkout topic/X`, then the user's IDE switches to a different branch in response to user activity, and the agent's next `git commit` lands on the IDE's branch.

To prevent silent collisions:

1. **Always run `git branch --show-current` immediately before any commit, push, force-push, or branch-rewrite operation.** The cost is one tool call; the cost of getting it wrong is a force-push to the user's branch.
2. **If the current branch differs from the intended one, stop and investigate before correcting.** Do not `git reset --hard` the surprise away — it might represent the user's in-progress IDE work.
3. **When force-push is the right fix, use `--force-with-lease`, never `--force`.** With-lease aborts cleanly if the remote has moved, which is a better failure mode than overwriting unseen commits.
4. **If a wayward commit has reached origin on the wrong branch, recovery is: cherry-pick to the intended branch, then `--force-with-lease` to roll back the wrong branch.** Do not leave the polluted branch in place hoping the next merge will sort it out.

This subsection is **MUST**-strength for any adopter using AI coding assistants alongside IDE work. **MAY** for human-only workflows where IDE-vs-agent collisions cannot occur.

## Pre-push gate

**BLOCK: Do not run `git push origin` until every step below passes.**

Run all checks in this order. All must pass:

<!-- CUSTOMIZE: adapt test/lint/migration commands to your project -->
```bash
# 1. All tests green
# 2. No linting errors
# 3. Migration chain is linear (if applicable)
# 4. Full local CI parity (adapt to your CI setup)
```

If any step fails: fix, re-run from step 1. Do not push a failing branch.

### Cross-branch dependency surfacing

**Surface cross-branch dependencies in the MR description, never in the branch.** When MR B depends on MR A (B's tests assume A's schema; B's count expectations assume A's scenario fixes; B's ADR cites A's implementation), state the dependency at the top of B's description:

> **Depends on:** `feat/A` (must merge first)
> **Behaviour if A merges first:** B is unchanged
> **Behaviour if B merges first:** A's MR will need a small rebase touching `<files>`

Do **not** rebase B onto A's tip and present a combined branch. Rebasing onto A loses the audit trail of what B changed independently and makes A's individual review harder.

Do **not** wait for A to merge before opening B. Open B as draft/WIP if needed; let reviewers see the dependency tree.

## Present push/merge options to the user

Once **all pre-push gate steps pass**, present these options clearly:

**Option A -- Direct merge (small change, single reviewer)**
```bash
git checkout main
git merge --no-ff feat/short-description
git push
```

**Option B -- Merge/Pull Request (preferred for review trail)**
```bash
git push origin feat/short-description
# Open MR/PR in your hosting platform
```

**Option C -- Squash merge (messy commit history)**
```bash
git checkout main
git merge --squash feat/short-description
git commit -m "feat: description of the feature"
```

State your recommendation and why. Do not push or merge without explicit user confirmation.

## Worktree and branch cleanup after merge

```bash
# If using worktree:
git worktree remove ../your-app-feat-name

# Delete the feature branch:
git branch -d feat/short-description
git push origin --delete feat/short-description
```
