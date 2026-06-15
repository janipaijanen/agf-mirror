---
provenance: verified-agf-v2
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Git Tags: Immutable Once Pushed
#
# Framework principle: Immutability Constraint
# See: framework.md — Principle 3 (The Immutability Constraint)

**Scope:** All git tag operations.

## Rule

A git tag, once pushed to a remote, is **immutable**. It must never be deleted, moved, or recreated at a different commit.

## What to do instead

If a tagged release needs a fix:

1. Commit the fix.
2. Bump the version number (e.g. `v3.0.32` -> `v3.0.33`).
3. Tag the new commit with the new version.
4. Push the new tag.

The broken version stays in history. This is correct and expected.

## Forbidden operations

- `git tag -d <tag> && git push origin :refs/tags/<tag>` -- never delete a pushed tag
- `git tag -f <tag>` -- never force-move a tag
- Retagging the same version at a different commit for any reason

## No exceptions

This rule applies even when:
- Iterating fast on a deployment
- The tagged commit has a trivial bug
- CI hasn't finished building the tag yet
- No one else has pulled the tag yet (you cannot know this)

The cost of an extra version number is zero. The cost of a moved tag is broken CI caches, mismatched container images, and violated trust.
