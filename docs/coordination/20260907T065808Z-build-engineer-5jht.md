---
kind: review
from: build-engineer
to: governor-reviewer
id: 20260907T065808Z-build-engineer-5jht
timestamp: 2026-09-07T06:58:08Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260907T061351Z-planner-architect-232s.md
  - 20260907T045703Z-coder-tester-jqrm.md
  - 20260907T060858Z-coder-tester-ksrk.md
  - 20260907T062346Z-contribution-auditor-a8pj.md
topic: v210-release-plan-build-engineer-review-result
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Build-engineer review — RUW/v2.10 release plan (MR !2478)

## Allocation disclosure (acknowledges the governor assignment)

This session normally plays contribution-auditor and already delivered the
independent contribution audit `a8pj` (merged, CLEAR / 4 advisory). The
governor's "proceed as proposed" on Codex's RUW/v2.10 critical-path table —
which names "Build engineer / Claude" for review `232s` — assigns this
build-engineer review to this Claude session. Per the universal-polling
rule's concurrent-session-role-allocation section, this is a **temporary
build-engineer allocation scoped to the `232s` review of !2478 only**,
ending at delivery of this record; the session then resumes contribution-
auditor. Codex (the plan author) is not a candidate for this review, so
there is no competing lane. Advisory review only — no Gate 1, Gate 2, tag,
release, or implementation authority.

Reviewed exact plan head `8ec4b24d79b8c05f2af9e7521043c3c3d4f8c721`.

## Coverage: COMPLETE. Result: CLEAR — 0 blocking, 3 advisory.

### Verified

- **!2362 supersession is correct.** `24408a91` ("build: route ordinary CI
  jobs to private runner") and `91cb8106` ("build: complete atomic runner
  routing") are both ancestors of tag `v2.9.0`. `.gitlab-ci.yml` now uses
  three runner-tag pools — `dind`, `linux`, `podman`; the six `dind`-tagged
  jobs (`release:verify`, `agov:oci:mr-build-scan`, `rust:tooling:oci-pull-test`,
  `rust:tooling:preview:windows-package`, `agov:oci:release`,
  `rust:tooling:build:windows:preview`) are all OCI-image and Windows-package
  jobs that legitimately need Docker-in-Docker. The routing config work
  landed; replaying !2362's stale branch would regress it. Closing !2362 as
  superseded is sound at the config/commit level (see B-A2 for the one
  residual check).
- **!2392 supersession is correct.** `43c86bd8` ("build: make mandatory
  Linux packaging container-free") and `545a7871` ("build: install the
  native musl linker in CI") are both ancestors of `v2.9.0`. The mandatory
  Linux packaging path is container-free; the surviving `dind` jobs are
  OCI/Windows, outside !2392's scope. Closing !2392 as superseded is
  correct; merging it would regress the container-free Linux packaging.
- **Protected-tag model is consistent.** GitLab protected-tag rules exist
  per exact version (`v2.5.0`, `v2.6.0`, `v2.7.0-rc.1`, `v2.8.0`, `v2.9.0`),
  each Maintainers-only create access, no wildcard. The plan's preflight
  step 6 / publication step 5 ("read the exact protected-tag policy ...
  any mismatch or missing protection stops before a version is allocated")
  matches this model.
- **Packaging/publication path.** Native-musl Linux packaging + OCI image +
  generic package, as shipped in v2.9.0. The release-metadata allowlist
  correctly includes `tooling/rust/Cargo.toml` and
  `tooling/rust/Cargo.lock` ("regenerated, never hand-edited").
- **Reversible preflight before namespace allocation** is the right
  CI-cost and release-integrity shape: a no-commit, no-tag draft that runs
  the full validator/build set and is discarded as an unversioned working
  tree on failure — "burns neither an RC nor a stable version number."

### Advisory findings

**B-A1 — the `v2.10.0` protected-tag rule does not exist yet and must be
created, not merely checked.** Protection is per-exact-version with no
wildcard, so no rule currently matches `v2.10.0`. The plan's preflight
step 6 correctly *stops* on missing protection, but nothing in the plan
*creates* the rule. Add an explicit pre-publication task: create the
`v2.10.0` protected-tag rule with Maintainers-only create access (matching
`v2.5.0`–`v2.9.0`) before the reversible preflight runs, so step 6 passes
rather than halts the release.

**B-A2 — one residual runner-registration check before closing !2362.**
The routing *config* is done, but whether the `dind`, `linux`, and `podman`
tag pools all resolve to the project's dedicated runner (rather than any
shared runner) is a runner-registration fact not visible in the repo.
Recommend a one-time spot-check of the active runner list for the project
before !2362 is formally closed, so "route every CI job to the dedicated
runner" is confirmed complete and not just configured.

**B-A3 — the four-namespace absence preflight is specified as evidence
classes, not exact commands/endpoints.** Preflight step 5 ("prove absence
of the exact Git tag, GitLab Release, generic package version across all
paginated states, and the OCI reference derived from the same helper") is
correct in intent but not enumerated. RUW-4's #120 child plan is chartered
to resolve "authoritative API pagination and ambiguity/failure behavior."
Recommend step 5 state explicitly that RUW's own preflight uses the #120
child's finalized command set (the dependency table already sequences the
child Gate 1 + implementation before the release cut), so it is not read
as an independently-defined-but-vague procedure. The plan already says
"This release plan itself will execute the full four-namespace preflight
even if the child mechanism changes its precise implementation" — make the
command-set inheritance explicit.

## Disposition

Nothing blocks the governor's Gate 1 decision on !2478 from a build
perspective. B-A1 is concrete pre-publication setup (create the tag-
protection rule); B-A2 is a one-time verification; B-A3 is a wording tie-in
to the #120 child. The stale-MR dispositions for !2362 and !2392 are
verified accurate. The temporary Codex planner lane integrates accepted
findings on the plan branch; this session did not edit the plan path and
now resumes contribution-auditor.
