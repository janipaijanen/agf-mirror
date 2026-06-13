---
provenance: inherited-v1x-unverified
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# All Changes Through Merge Requests
#
# Framework principle: Quality Gates (Pattern 1), Enforcement Beyond Prompts
# See: framework.md — Pattern 1 (Pipeline), Section 5 (Enforcement Beyond Prompts)
# Companion rule: release-via-merge-request-rule.md (release-specific workflow)

**Scope:** All changes to protected branches (main, production, release/*).

## Rule

Every change to a protected branch MUST arrive via a merged merge request.
Direct commits and direct pushes to protected branches are forbidden.

This applies to:
- Feature work
- Bug fixes
- Documentation changes
- Release preparation (see release-via-merge-request-rule.md for release-specific steps)
- Framework or governance updates
- CI/CD configuration changes

## Workflow

1. Create a branch from the target branch.
2. Commit changes to the branch.
3. Open a merge request targeting the protected branch.
4. Review the MR (self-review is acceptable for single-contributor projects, but the MR must exist).
5. Merge after review passes.

## Why

Merge requests are the quality gate between "work done" and "work accepted". Without them:

- Changes bypass review, even when a reviewer is available.
- There is no diff-based audit trail separate from the commit log.
- CI pipelines that run on MR events are never triggered.
- AI coding assistants, which optimise for task completion, will push directly unless a gate prevents it.

A merge request costs minutes. A direct push to main that breaks the build, ships a broken release, or introduces an ungoverned change costs significantly more.

## For AI-assisted development

AI coding assistants SHOULD be instructed to:
- Always create a branch, never commit to protected branches directly.
- Open an MR and present it for review before merging.
- When asked to "commit and push" or "commit, push, tag", check whether the target is a protected branch and follow this rule.

<!-- CUSTOMIZE: list your protected branches below -->
## Protected branches

- `main`

## Exceptions

- Initial repository setup (first commit) before branch protection is configured.
- Emergency hotfixes MAY be pushed directly if (a) the incident is actively causing harm, (b) the push is followed by a retroactive MR documenting the change, and (c) the exception is recorded as a learning.
