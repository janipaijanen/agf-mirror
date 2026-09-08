---
name: release
origin: upstream
description: >
  Full release workflow: version bump, changelog, branch, commit,
  push, merge request. Codifies the mechanical steps so the human
  focuses on what changed, not how to ship it.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Release

Perform a gated release of the framework or adopter project. Every
gate requires explicit human confirmation.

## Prerequisites

- Working tree is clean (no uncommitted changes)
- All tests pass (`cargo test --manifest-path tooling/rust/Cargo.toml` or the
  project-specific test command)
- You know the bump type: `major`, `minor`, or `patch`

## Step 0 — Confirm topology (fail-closed to standard)

Almost every release uses the standard path below: branch from the
default branch, bump, tag the merge commit. Do not select the
maintenance-line path (a protected `release/X.Y` target instead of the
default branch) unless **all** of the following are already true —
if any is missing, use the standard path, even if someone asks for
the maintenance-line path by name:

- A binding governor decision-record names the exact candidate tag
  object, peeled commit, and intended stable version.
- A separate release-publication plan for that exact candidate has
  its own Gate 1 approval.
- The release changes only version, changelog, and other enumerated
  release-truth metadata — no functional, schema, migration, test,
  dependency, CI, or artifact-layout change.

If all three hold, follow
[`release-via-merge-request-rule.md`](../../rules/workflow/release-via-merge-request-rule.md)'s
"Maintenance-line exception" section instead of Steps 1-7 below — its
eligibility, protection, tag, and forward-sync contract is the
authority; do not improvise a variant.

Do not select the frozen-scope reconstruction path (a pair of
dedicated, protected `reconstruct/vX.Y` / `release-vX.Y.Z` branches
built from the last stable tag, not the default branch's current tip)
unless **all** of the following are already true — if any is missing,
use the standard path, even if someone asks for this path by name:

- A binding governor decision-record names the frozen release scope by
  tracker identifier, the exact stable tag to build from, and why
  neither the standard path nor the maintenance-line exception applies.
- A separate release-publication plan for that exact frozen scope has
  its own Gate 1 approval, naming the exact cherry-pick allowlist and
  frozen-out set.
- Every allowlisted commit is independently verified against its own
  Gate 2 approval before being treated as consumed.
- No immutable release candidate already exists to promote (if one
  does, use the maintenance-line path above instead), and the intended
  Git tag, GitLab Release, generic-package, and OCI namespaces for the
  new version are independently confirmed absent.

If all four hold, follow
[`release-via-merge-request-rule.md`](../../rules/workflow/release-via-merge-request-rule.md)'s
"Frozen-scope reconstruction exception" section instead of Steps 1-7
below — its eligibility, tree-delta proof, protection, tag, and
forward-sync contract is the authority; do not improvise a variant.

## Step 1 — Bump version

Version bump tooling is deferred. Until `agov bump` exists, perform
the release bump manually:

- `VERSION` file
- `CHANGELOG.md` (placeholder section)
- `README.md` citation (major.minor)
- All `agents/*.yaml` framework_version fields

Review the output — verify the version number is correct.

### GATE 1 — User confirms version bump

> **Wait for explicit user confirmation before continuing.**
> The user may want a different bump type.

## Step 2 — Write changelog

Fill in the `<!-- TODO: fill in release notes -->` placeholder in
`CHANGELOG.md`. The changelog entry should:

- Start with a one-line summary of the release theme
- List test count
- Reference closed issues (`Closes #N`)
- Group changes by type (BREAKING, features, fixes)
- Cite evidence (learnings, adopter projects) where applicable

Present the draft to the user.

### GATE 2 — User confirms changelog

> **Wait for explicit user confirmation before continuing.**

## Step 3 — Run health check

```bash
agov validate
```

Full `agov health` summary coverage is deferred. All available checks must
pass; if any errors, fix before proceeding.

## Step 4 — Create release branch and commit

```bash
git checkout -b release/v<version>
git add <changed files>
git commit -m "chore: bump to v<version> — VERSION, CHANGELOG, agent framework_versions"
```

Do not stage unrelated files. Stage only files changed by the version bump
and the changelog.

## Step 5 — Push and create merge request

```bash
git push -u origin release/v<version>
```

Create a merge request targeting `main` with:
- Title: `Release v<version>` (add `BREAKING:` prefix if applicable)
- Body: summary of changes, test plan, closed issues

### GATE 3 — CI passes and MR is reviewed

> **Wait for CI pipeline and human review before merging.**

### Standard-path namespace preflight

Follow the [owner rule's namespace obligation](../../rules/workflow/release-via-merge-request-rule.md#pre-tag-namespace-absence-obligation-standard-path).
The release-metadata MR must pass `release:namespace-check`. From the clean,
committed release checkout, also run the operator preflight below and attach
its JSON evidence. The checkout must contain the OCI helper; `git`, `curl`,
and a POSIX `sh` must be available. Use the configured release project's ID/path and registry
base, not a registry repository ID. `AGOV_RELEASE_TOKEN` is an already-authorized
operator API credential supplied through the environment; never echo it or
enable shell tracing. CI instead supplies its ephemeral `CI_JOB_TOKEN`.

For a local operator run, explicitly export `CI_API_V4_URL` (the GitLab API
v4 base URL), `CI_PROJECT_ID` (the numeric release project ID),
`CI_PROJECT_PATH` (its full namespace/project path), and `CI_REGISTRY_IMAGE`
(the release project registry base) from your configured release project first.
These names are inputs to the examples below; a local shell does not populate
them automatically. Supply the existing authorized `AGOV_RELEASE_TOKEN` through
your credential environment without printing it. Both operator checkpoints
use these same values.

```bash
agov release namespace-check --phase preflight \
  --api-url "$CI_API_V4_URL" --project-id "$CI_PROJECT_ID" \
  --project-path "$CI_PROJECT_PATH" --registry-image "$CI_REGISTRY_IMAGE" \
  --target-branch main
```

Only exit 0 with JSON `result: "ABSENT"` for all four namespaces passes.
Any failure stops the release; do not reinterpret 404 or an empty error response
by hand. The command reads the candidate version and package from the release
checkout and derives the OCI reference using the existing verified helper.

## Step 6 — After merge: tag and push

```bash
git checkout main && git pull &&
agov release namespace-check --phase pre-push \
  --api-url "$CI_API_V4_URL" --project-id "$CI_PROJECT_ID" \
  --project-path "$CI_PROJECT_PATH" --registry-image "$CI_REGISTRY_IMAGE" \
  --target-branch main &&
git tag v<version> &&
git push origin v<version>
```

The pre-push run is mandatory and must be immediately followed by the authorized
tag operation. It requires HEAD to equal the live release-target commit. Stop
if it fails; a successful preflight or MR check does not substitute for it.

The tag push triggers the CI release pipeline (build-dist → create-release).

## Step 7 — Verify release

Check that the GitLab release was created with dist archives:

```bash
glab release view v<version>
```

<!-- CUSTOMIZE: add project-specific post-release steps (deploy, notify, etc.) -->
