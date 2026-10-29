---
id: 20261029T192922Z-release-publisher-api-amendment
status: pending-gate1
topic: maintained-create-only-release-publisher
affected_paths:
  - .gitlab-ci.yml
  - tooling/rust/scripts/create-release.sh
  - tooling/rust/crates/agov/tests/release_create.rs
  - tests/ci-pins-selftest.sh
  - tooling/rust/crates/agov/tests/gate_change_check.rs
  - tooling/rust/crates/agov/tests/ci_needs_implication.rs
  - tooling/rust/crates/agov/tests/plan_collision.rs
  - docs/plans/20261029T163752Z-v213-release-scope.md
depends_on:
  - 20261028T194451Z-ci-reproducibility-pins
  - 20261029T183000Z-ci-pins-gate-test-amendment
  - 20261029T185703Z-dist-profile-gate-test-amendment
  - 20261029T163752Z-v213-release-scope
  - 20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion
  - 20260915T075705Z-issue130-pending-advisory-consultation-status
  - 20261021T204757Z-remove-dind-ci
  - 20261021T220000Z-retire-windows-support
  - 20261023T083302Z-remove-elixir-leftovers
advisory_consultation:
  - role: contribution-auditor
    status: pending
    reason: Independent review of the concrete publisher replacement plan requested.
  - role: infosec-sentinel
    status: pending
    reason: >-
      Prior read-only design advice recommended the direct API approach; this
      concrete high-risk publication plan still requires independent review.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Replace the obsolete publisher without changing release contents

## Evidence and decision requested

MR !3358's pinned release-cli v0.24.0 image has 76 HIGH/CRITICAL package findings
across 63 CVEs. Its normal package installation supplies fixed-version evidence
for five libcurl CVEs; 58 CVEs still lack package remediation evidence, including
all Critical findings. The upstream registry check found no newer stable
release-cli tag. Evidence and explicit limits are attached to !3358, including
runtime addendum note `4119456225`. None of these risks is accepted here.

GitLab deprecates release-cli and documents its Releases API with job-token
authentication. A direct create request using the job's existing curl pattern is
smaller than adopting glab's update-or-create behavior and avoids a second Go
publisher binary. This is an implementation choice requiring Gate 1, not a claim
that a newer image is vulnerability-free.

This child replaces only the parent's Pin 1 image/publisher method and its tool
acceptance. The Rust toolchain/installer pins, advisory checker boundary, shared
gate tests and every other parent requirement remain. Update only the CI-item
acceptance wording in the approved v2.13 scope plan to name this child and the
new publisher; retain feature contents, order, version and separate freeze/tag
authority. No tag, Release, package upload, protection mutation or deployment is
authorized by this implementation plan.

## Maintained image and isolation

Candidate: official `docker.io/library/alpine:3.24.3`, the latest patch shown for
the supported v3.24 branch on 2026-10-29. Confirm that registry tag, resolve and
record its immutable manifest digest and linux/amd64 identity, and use the exact
tag plus digest in `release:create`. If unavailable or unsuitable, stop and
amend the selection; never silently substitute `latest`, edge or a custom image.

Keep existing curl/unzip/tar/gzip installation and add jq for JSON construction.
Before using any credentials, inspect image configuration and run separate
disposable credential-free, proxy-disabled containers with no project mounts:
first check shell/apk; then replay the actual package-install order and verify
curl/jq/archive tool availability. Record versions, architecture and CA-bundle
inventory. Scan both the immutable base and the post-install runtime, retaining
HIGH/CRITICAL evidence. Every new finding requires verified remediation or an
explicit governor disposition before merge. Package repositories remain mutable;
do not claim full package reproducibility. No custom/builder-image work returns.

## Create-only API client

Add one small shell helper next to the existing release-assets helper. Use jq
for JSON and the existing curl dependency for transport. A preflight phase writes
the intended payload to a caller-owned temporary file; a publication phase uses
that same intent after uploads. Required context comes from the same-project CI
variables and the existing canonical package base, not a configurable new host.
Require nonempty context; preserve the existing release name, description and
package namespace. Clean up owned temporary files; never store tokens in payloads.

Retain this job ordering:

1. Run the existing complete artifact-set and provenance verification unchanged.
2. Build links from the exact sorted output of `release-assets.sh list dist`.
   Preserve every name, canonical package URL and `link_type: package`; use JSON
   construction rather than shell-escaped JSON strings. Prove a cardinality and
   name/URL bijection, reject duplicate/empty intent, and URL-encode tag segments.
3. Before any package upload, GET the existing tag and require its commit ID to
   equal `CI_COMMIT_SHA`. Establish positive Releases-namespace access, then GET
   the exact release and require absence. Missing/mismatched tags, existing
   releases, access failures, malformed responses or ambiguous results stop.
4. Keep the existing Generic Package Registry upload loop and artifact selection.
5. Immediately before creation, repeat the tag-consistency and release-non-overwrite
   checks. POST exactly once to `/projects/$CI_PROJECT_ID/releases` with only
   `tag_name`, `name`, `description` and `assets.links`. Omit `ref` entirely.
   Require HTTP 201. Never PUT, DELETE, create a tag, follow redirects, or retry POST.
6. Verify the returned release and one follow-up GET against the intended tag,
   commit SHA, name, description and exact link bijection. Preserve the existing
   composed `release:verify` job afterward.

Use only the existing ephemeral `CI_JOB_TOKEN` in a JOB-TOKEN header. Do not print
or persist it, put it in a URL, or enable tracing/verbose transport. The JOB-TOKEN header MUST use stdin or another mechanism proven to keep the
token out of argv; MUST NOT expand the token into any external-process argument.
Ignore
ambient curl configuration; use bounded connection/operation timeouts. Emit
bounded status/reason diagnostics, not raw API bodies that may echo supplied data.

An indeterminate POST outcome permits at most one read-only GET for diagnosis.
Always stop and require operator reconciliation, even if that GET appears to
match; never infer permission to repeat POST, update or delete. Upload-first
failure may leave package files, as today; report the condition without automatic
destructive rollback. No retry/recovery automation is introduced.

## Authority and race limits

These are preconditions of the authorized publication operation, not a new
governance validator. `release-via-merge-request-rule.md` requires an unoccupied
Release namespace and explicitly permits post-tag consistency/non-overwrite
checks; `immutable-tags-rule.md` binds a pushed tag to its commit. Error messages
identify the operation failure and relevant rule; success never approves a gate.
Do not extend the separate named authority of `release:namespace-check`.

The API has no documented atomic expected-tag-SHA condition. A prohibited tag
move between checking and POST remains a race; postcondition checks detect it,
stop and escalate without destructive repair. Protected, operationally immutable
tags remain a separate release-publication prerequisite. The read-only project
inventory showed exact protection for existing releases including v2.12.0 and
no future wildcard; this amendment creates no future protection. Do not add a
resource group, new rule, role, validator or tag-management workflow here.

## Tests and acceptance

- Use the existing Rust integration-test infrastructure, real jq and a local
  fake endpoint or deterministic recorded-transport fixture. Use dummy tokens
  only, never production writes. Do not add a Rust dependency or TLS test bypass
  to the production client merely to support tests.
- Cover exact JSON/escaping and link bijection; tag URL encoding; missing/wrong
  tag SHA; inaccessible namespace; pre-existing release; 401/403, malformed
  responses, redirects, 409 and lost POST response. Assert request order/count,
  zero mutations for failed initial preflight, exactly one POST on success, no
  PUT/DELETE/redirect following, diagnostic GET without retry on ambiguity,
  postcondition mismatch failure, cleanup and token non-disclosure. A fake-curl
  or process-observation test must prove the dummy token is absent from argv,
  logs, payloads, temporary files and diagnostics.
- CI tests must prove artifact/provenance checks precede preflight/uploads, the
  two helper phases surround the existing upload loop, and `release:verify` still
  depends on creation. Keep MR trigger coverage for helper/tests through existing
  `tooling/rust/**` rules. Preserve all mandatory needs and job-token boundaries.
- Update the advisory pin fixture for the new image, retain all inherited shell
  cases and substantive shared-gate negative tests, and run gate change-check.
- Validate CI configuration, run release-assets selftests and affected Rust
  targets. Require independent implementation/infosec review and a green pipeline
  at the submitted SHA. The earlier !3358 Rust-job DNS failure is retained; the
  corrected candidate supplies the next required acceptance run.
- Before merge, verify job-token access assumptions with an authorized read-only
  CI probe of an existing protected tag and Releases endpoint, or explicitly
  record the remaining platform acceptance gap for governor decision. Never
  make a throwaway production tag/Release to test this plan. Mock success alone
  is not proof of the project's job-token settings or protected-tag permissions.

## Coordination, rollback and sources

Codex retains the CI-pin lane. Apply this amendment to that lane after Gate 1;
preserve the independently reviewed Rust pins and tests. The profiling lane owns
separate changes in two shared test files: acknowledge its plan, preserve both
deltas, and reconcile current main/open MR paths before either acceptance.
Inventory all plan overlaps before approval; acknowledged historical CI changes
must remain intact. No other live claim is reassigned.

Rollback before merge removes this delta and leaves CI pins blocked. After merge,
a reviewed revert must not silently restore a vulnerable or floating publisher;
hold publication until a separately accepted replacement is available.

Primary sources, read 2026-10-29:
- [GitLab Releases API and job-token authentication](https://docs.gitlab.com/api/releases/)
- [Release CLI deprecation](https://docs.gitlab.com/user/project/releases/release_cli/)
- [Job-token endpoint access](https://docs.gitlab.com/ci/jobs/ci_job_token/)
- [Supported Alpine releases](https://www.alpinelinux.org/releases/)
