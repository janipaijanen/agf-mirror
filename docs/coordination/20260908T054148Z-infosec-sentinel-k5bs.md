---
kind: review
from: infosec-sentinel
to: governor-reviewer
id: 20260908T054148Z-infosec-sentinel-k5bs
timestamp: 2026-09-08T05:41:48Z
model: gpt-6-astra
immutable: true
references-to:
  - 20260908T042641Z-planner-architect-m5rp.md
  - 20260908T043532Z-contribution-auditor-hqp4.md
  - 20260907T200210Z-coder-tester-8j2h.md
topic: ruw4-implementation-exact-head-infosec-review
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# RUW-4 exact-head InfoSec review — CLEAR at `b4199c26`

Scope: all 13 files in MR !2539 at exact head
`b4199c26abfd43090c44dc3c795331f4584de734`, including the correction from
the `hg52`-reviewed head. Coverage: **COMPLETE**. Recommendation: **CLEAR**.
Blocking findings: 0. Advisory findings: 1. Accepted-risk findings: 0.
Authority: advisory recommendation only; governor-reviewer decides Gate 2.

## Reviewer provenance

The governor directly assigned this Codex/GPT-6 session to take the
infosec-sentinel role. This same session previously contributed the
authentication-phase correction and tests retained in !2539 after the `r6aq`
worktree collision. It is therefore an implementation author and this review
is **not independent corroboration** of that correction. The governor must
weigh that limitation. No implementation file or release namespace was
modified during this review.

## Required screening result

| Check | Status | Details |
|---|---|---|
| `agov validate infosec` | pass | 2 learning blocks; 0 warnings; 0 acknowledged findings |
| pre-commit hooks | skipped | no `.pre-commit-config.yaml` exists |
| SPDX headers | pass with advisory | every new source has an SPDX header; the modified legacy operator page lacks one both at the comparison base and reviewed head |
| project patterns | pass | no secret value, private key, foreign codename, cross-project linkage, or real identity introduced; test endpoints use reserved `.invalid` names |

Targeted review confirmed:

- CI uses only the ephemeral same-project `CI_JOB_TOKEN` in a `JOB-TOKEN`
  header; operator phases use only the environment-supplied
  `AGOV_RELEASE_TOKEN` in a `PRIVATE-TOKEN` header. The phases cannot fall back
  to each other's credential.
- The credential is validated against config-file injection, written to
  `curl --config -` over stdin, absent from argv, suppressed from stderr and
  response diagnostics, and never included in JSON evidence or artifacts.
  The shell adapter begins with `set +x`.
- Transport accepts only an HTTPS `/api/v4` root, performs GET requests only,
  does not follow redirects, limits connection/request duration and response
  size, and rejects malformed, duplicate pagination, redirect, authentication,
  rate-limit, server-error, and non-JSON responses.
- Same-project MR identity binds full project reference, source and target
  project IDs, source SHA, target branch, pipeline ID, open state, and the
  checked-out SHA. Operator identity binds the project ID and canonical path.
  Registry location and exact helper-derived OCI identity are separately
  checked.
- Each namespace establishes positive collection access before 404 can mean
  absence. All generic-package statuses and pages are enumerated; fuzzy names
  do not occupy the namespace, exact identities do. OCI repository discovery
  and tags are fully paginated. Missing fields, duplicate repositories,
  partial walks, access ambiguity, checkout drift, and target-branch movement
  all fail closed.
- The command exposes no mutation method; negative tests confirm every
  occupied namespace stops before tag creation. No live credential or release
  object was used in this review.
- Exact-head pipeline `2828211506` passed. Mandatory `cargo deny check`
  reported advisories, bans, licenses, and sources OK. Trivy reported zero
  vulnerabilities from `tooling/rust/Cargo.lock`; its workspace-version parse
  warnings are covered by the separate Rust advisory check and are not a RUW-4
  security failure.

Focused exact-head evidence also passed: 8 internal namespace tests, 6 real
CLI/adapter tests, 12 CI-needs tests, 15 gate-change tests, GitLab CI lint,
strict projection validation, shell syntax, and diff whitespace.

## Advisory RUW4-IS-A1 — inherited SPDX gap on modified operator page

Check: changed-source SPDX placement. Outcome: needs-review. Requirement
strength: recommended for this MR because the omission predates the change.
Impact: minor. Enforcement: advisory. Basis: the `infosec-screen` skill's SPDX
step. Finding state: open; risk disposition: none.

`agf/docs/site/docs/operate/review-and-release.md` lacks a file-local SPDX
header at both the implementation comparison base and `b4199c26`. All three
new files and the other changed governed sources carry headers. Resolution is
either adding the project-appropriate header in separately authorized scope or
clarifying the repository's policy for legacy site pages. This inherited
licensing hygiene item does not weaken the credential, identity, absence, or
fail-closed guarantees reviewed here and is not a Gate 2 security blocker.

## Maturity boundary

The reviewed implementation is **Level 4 Validated**. Level 5 Deployable waits
for a supported release artifact and a release-metadata MR that executes the
real namespace job. This corrects only the maturity label in `hqp4`; it does
not alter that review's contribution CLEAR result.
