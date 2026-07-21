---
id: v230-release-preparation
status: pending-gate1
topic: v2-3-0-release-preparation
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: no separate infosec actor is available during plan drafting; the plan compensates with mandatory credential-leak validation, pinned OCI inputs, packaged-artifact provenance checks, and a blocking HIGH/CRITICAL vulnerability scan before tagging
  - role: contribution-auditor
    status: consulted
    reason: requires-ack critical review is routed to Claude through coordination record 20260721T073926Z-planner-architect-grhm and MR discussion before Gate 1 disposition
---

# Plan: prepare and publish agents-govern v2.3.0

## Status and decision requested

**Pending Gate 1. Do not start the release implementation, bump versions, or
create a tag until the governor approves this plan.**

The governor is asked to approve one executable path from the current NO-GO
state to a verified v2.3.0 release. Approval fixes the target version and the
release procedure; it does not waive any prerequisite or grant permission to
tag before the release MR is merged.

## Governing evidence

- `20260720T201654Z-coder-tester-sb14` authorizes a v2.3.0 minor release.
- `20260720T222023Z-coder-tester-ksgr` records the release NO-GO findings.
- MR !274 merged the packaged-provenance and OCI scan/vulnerability repairs.
- `20260721T062347Z-coder-tester-dkmj` makes !289 a release dependency.
- `20260721T073721Z-governor-reviewer-q77n` excludes !293 and !294 from the
  v2.3.0 scope.
- `release-via-merge-request-rule`, `immutable-tags-rule`,
  `semver-no-carry-rule`, `security-hardening-rule`, and
  `gate-self-test-rule` govern execution.

## Frozen scope boundary

### Required before the release branch is cut

1. MR !289 has a green pipeline on its final content, receives Gate 2 approval,
   merges to `main`, and the resulting `main` pipeline is green.
2. The governor records one public-truth disposition for v2.2.6:
   - publish v2.2.6 from its correct historical release commit before v2.3.0;
     or
   - explicitly mark v2.2.6 as prepared but unpublished and folded into
     v2.3.0.
3. The exact `main` commit used to cut the release branch is recorded.

### Explicitly outside v2.3.0

- MR !293, the site-wide terminology audit follow-up.
- MR !294, typed coordination relationships and scoped supersession.
- MR !117 and unrelated academic-paper work.
- New feature, rule, schema, or documentation initiatives not already merged
  into the frozen `main` commit.
- SBOM generation, release signing, installer scripts, third-party notices,
  and universal provenance graphs.

Out-of-scope work may continue separately, but it is not a release dependency
and must not delay the release. Once the release branch is cut, later merges to
`main` are not silently pulled into v2.3.0.

## Implementation branch and permitted changes

After the prerequisites and Gate 1 approval, create
`release-v2.3.0` from the recorded `main` commit, following the repository's
release rule. The release MR must remain a release-only diff. Permitted changes:

1. `agf/VERSION`: `2.2.6` to `2.3.0`.
2. Rust workspace package versions and internal dependency constraints to
   `2.3.0`, plus the mechanically regenerated lockfile.
3. `agf/templates/AGENTS.md` version marker and other governed loader/version
   markers required to match `agf/VERSION`.
4. A complete v2.3.0 entry in `agf/CHANGELOG.md`, including the mandatory
   adopter-docs disposition marker.
5. Source documentation, citation metadata, OCI examples, and release metadata
   whose literal current-version value must become `2.3.0`.
6. README inventory counts derived from the frozen release tree rather than
   copied from an earlier review.
7. Public-truth corrections required by the chosen v2.2.6 disposition and the
   approved MCP limitation language.

No product feature implementation belongs on the release branch. A discovered
product defect blocks the release and is repaired through its own governed MR;
the release branch is then recreated or cleanly updated from the newly approved
`main` state.

## Changelog content

Derive the entry from the frozen tag-to-candidate diff. At minimum, review and
accurately describe what actually ships in these areas:

- coordination write primitives and lifecycle commands;
- the MCP server, including its honest maturity and known limitation status;
- OCI distribution and the repaired package/scan path;
- material rule and schema additions;
- adopter applicability catalog from !289;
- Learning loop and evidence documentation and associated diagrams;
- material adopter-facing installation, operation, enforcement, or evaluation
  documentation changes.

Do not describe Level 4 validated work as Level 5 deployable until the release
artifacts and supported operator path contain it. Do not claim empirical
validation, ISO conformity, or completed adopter pilots without evidence.

## Release-candidate validation

Run on the exact release MR head and record command, result, commit, pipeline,
and artifact identity.

### Repository and documentation

```bash
cargo build --release --manifest-path tooling/rust/Cargo.toml -p agov
cargo test --manifest-path tooling/rust/Cargo.toml
cargo clippy --manifest-path tooling/rust/Cargo.toml --workspace --all-targets -- -D warnings
agov validate
agov validate adopter-docs
agov dist validate
agov dist build
mkdocs build -f agf/docs/build/mkdocs.yml --strict
git diff --check
```

Also validate every plan and coordination file added by the release work and
run the configured infosec/credential-leak validation.

### Version and negative gate tests

- Search authoritative source files for stale `2.2.6` release examples and
  classify every remaining occurrence as historical or defective.
- Confirm `agf/VERSION`, Rust packages, internal dependency constraints,
  lockfile, templates, release information, citations, and OCI examples agree.
- Run `CI_COMMIT_TAG=v2.3.0 agov dist check-tag` and require success.
- Run a mismatched tag such as `CI_COMMIT_TAG=v9.99.99 agov dist check-tag` and
  require failure before archive creation.
- Verify the CI workflow actually selects all required jobs for a v2.3.0 tag;
  configuration lint alone is insufficient evidence that tag-only jobs work.

### Packaged artifacts and provenance

- Inspect preview `.tar.gz` and `.zip` contents for forbidden/local paths.
- Verify every generated checksum.
- Inspect framework and tooling provenance JSON for version, source commit,
  clean state, target, archive name, and checksum agreement.
- Execute every runnable packaged `agov` binary available in the validation
  environment and require `agov version --format json` to report `2.3.0`, the
  candidate commit, and `dirty: false`.
- Record an approved limitation for a target that cannot be executed locally;
  its tag-pipeline result remains mandatory.

### OCI candidate

- Build the candidate image without publishing a release tag.
- Run the containerized `agov` version/runtime smoke test.
- Confirm OCI labels match version and candidate commit.
- Run the corrected, fully qualified scanner path against the same image.
- Require zero fixed HIGH/CRITICAL findings under the configured policy.
- Record image digest and scanner database/timestamp evidence.

### GitLab evidence

- Require a green merge-request pipeline on the exact release head.
- Download and inspect preview artifacts from that pipeline rather than relying
  only on local outputs.
- A coordination-only tip must not conceal untested content. If workflow rules
  skip a mixed push, create or manually run a pipeline that covers the final
  content commit and exact review head.

## Gate 2 and publication sequence

1. Prepare `tmp/MR_DESCRIPTION.md` and synchronize it to the release MR.
2. Publish a Gate 2 handoff naming the exact head, validation commands,
   pipelines, artifacts, v2.2.6 disposition, and residual risks.
3. Governor reviews and merges the release MR.
4. Resolve the resulting `main` merge commit and verify it contains the exact
   approved release diff.
5. Create annotated tag `v2.3.0` on that merge commit and push it immediately
   in the same controlled action. Never tag the release branch.
6. Monitor all tag jobs: tag/version guard, source archives, Rust target
   archives, OCI build/smoke/scan/push, package uploads, and GitLab Release
   creation.

The merged release MR is the authorization for the tag-and-push action under
`release-via-merge-request-rule`; no second interactive confirmation is needed.

## Failure and recovery rules

- Before tag push: stop, fix through an appropriately governed MR, rerun the
  full affected validation, and update the release candidate evidence.
- After tag push: never delete, move, or recreate `v2.3.0` or its immutable OCI
  version tag. Record the failure and prepare a new patch version.
- If the GitLab Release job partially publishes assets, preserve the tag and
  record exactly which assets exist; recovery must not rewrite immutable
  history.
- Do not call a release complete while required jobs are running, skipped
  unexpectedly, or missing.

## Post-publication verification

From the public release surfaces rather than the CI workspace:

1. Confirm the GitLab Release and package-registry links exist.
2. Download every framework and supported tooling archive.
3. Verify checksums and provenance records.
4. Execute the downloadable Linux binary and verify version, commit, and clean
   state.
5. Confirm expected Linux, ARM, and Windows archives are present.
6. Pull the exact OCI version tag, record its digest, run a smoke test, and
   confirm it is not a floating tag.
7. Confirm Pages reports v2.3.0 and that download/changelog links resolve.
8. Publish a completion record containing tag, merge commit, pipeline, release,
   package, Pages, checksum, provenance, and OCI evidence.

## Acceptance criteria

Gate 2 may be approved only when:

- all frozen-scope prerequisites are satisfied;
- the v2.2.6 history discrepancy has a durable public-truth disposition;
- the release-only diff is complete and version-consistent;
- repository, documentation, distribution, provenance, negative-gate, OCI,
  vulnerability, and GitLab validations pass on the exact release head;
- the governor can identify the precise merge commit that will be tagged;
- no required evidence depends on a stale pipeline; and
- residual risks and deferred work are explicit.

Release completion requires the post-publication checks, not merely a green tag
pipeline or the existence of `v2.3.0`.

## Review questions

1. Is the v2.2.6 history decision correctly blocking release-branch creation?
2. Is the release-only branch boundary narrow enough while still allowing all
   required version/public-truth corrections?
3. Does the validation matrix prove the repaired tag-only package and OCI paths
   before the immutable tag is created?
4. Are any required v2.3.0 features missing from the changelog minimum?
5. Is any acceptance criterion impossible to demonstrate before tagging and
   therefore better classified as a post-publication completion check?
