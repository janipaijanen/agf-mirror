---
provenance: inherited-v1x-unverified
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Pre-flight Check Before Destructive Recommendations
#
# Framework principle: Boundary enforcement, evidence before action
# See: framework.md — Principle 5 (Escalation over Improvisation)

> **Provisional status.** This rule is shipped on single-adopter
> evidence (silver-quill, 2026-04-18 — `git filter-repo` silently
> discarded 8 uncommitted fixes). The meta-pattern (preconditions
> matter) is corroborated by a second adopter (amber-relay #32), but
> the destructive-op checklist below is not yet independently
> corroborated. Adopters hitting this rule: please file a learning
> record so we can tighten or generalise the checklist.

**Scope:** Any recommendation to execute a destructive or irreversible
operation.

## Rule

Before recommending an operation that cannot be undone, an agent MUST:

1. **List preconditions** — state what must be true for the operation to
   be safe (e.g. "working tree must be clean", "remote must be synced",
   "a backup exists").
2. **Verify each precondition** — check the current state against each
   precondition using available tools (`git status`, `ls`, `df`, etc.).
3. **Block on failure** — if any precondition is unmet, do not issue
   the recommendation. Instead, provide the remediation step for the
   unmet precondition and re-evaluate.

The rule fires at the *recommendation* boundary, not at the execution
boundary. An agent that emits a destructive recommendation without this
pre-flight has already failed, even if the human user catches it before
running the command.

## Destructive operations requiring pre-flight

- **Git history rewrites:** `git filter-repo`, `git filter-branch`,
  `git rebase -i` with squash/drop, `git reset --hard`
- **Forced pushes:** any `git push --force`, `git push +branch`
- **Filesystem destruction:** `rm -rf`, `truncate`, overwriting
  unversioned files, `dd of=...` on non-empty targets
- **Database operations:** table/schema drops, `TRUNCATE`, migrations
  with no rollback path, column-drop alterations

<!-- CUSTOMIZE: extend this list with project-specific destructive ops
     (e.g. terraform destroy, kubectl delete on protected namespaces,
     queue-purge commands, encryption-key rotation) -->

## Example pre-flight output

> **Preconditions for `git filter-repo`:**
> - [ ] Working tree clean — **FAIL: 8 modified files not committed.**
>       Fix: commit or stash before proceeding.
> - [x] Remote in sync — OK.
> - [x] Current branch matches rewrite target — OK.
>
> Cannot recommend until failing preconditions are resolved.

## Why this exists

An agent that knows how to use `git filter-repo` also knows, in
principle, that it rewrites history and performs a fresh checkout. But
recommending it correctly requires also knowing *what is currently in
the working tree*. Correct-in-isolation remedies destroy work when
their assumed preconditions are not verified. The cost of verifying is
a few tool calls; the cost of missing is unrecoverable.

## Related rules

- `always/high-risk-operations-rule.md` — gates edits to sensitive code
  areas (different axis: code content, not operation type)
- `always/environment-interrogation-rule.md` — gates recommendations
  that assume target-host state (different trigger: environment
  assumptions, not destructiveness)
