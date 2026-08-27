---
id: v248-publication-recovery
status: approved
topic: v248-publication-recovery
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >
      A fresh exact-head review is required because the v2.4.7 Gate 2 evidence
      missed a downstream release consumer that still assumed a Windows zip.
      Review must challenge the complete pre-upload command path, immutable
      partial-publication handling, exact file boundary, and whether the new
      evidence would have caught job 15630274271 before tagging.
  - role: infosec-sentinel
    status: consulted
    reason: >
      This patch changes no credentials, registries, provenance schema, or
      trust boundary. The mechanical infosec validator remains mandatory at
      plan and implementation heads. Existing checksum, provenance, OCI
      digest, and public re-download checks remain blocking.
---

# Plan: v2.4.8 completion of the publication recovery

## Incident and current public state

Issue [#95](https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/95)
records both failed immutable release candidates.

- `v2.4.6` failed twice in `rust:tooling:release` because the combined Linux
  and Windows cross-build exhausted runner disk. It published no Release,
  generic package, or OCI tag.
- `v2.4.7` correctly removed Windows from the tag build and mandatory asset
  list. Its tag pipeline `2719574688` passed the Linux tooling build, OCI
  publication, and all tests, but `release:create` job `15630274271` failed
  before generic-package upload with:

  ```text
  release payload not found: dist/agov-*.zip
  ```

  The mandatory-asset verifier accepted the intended Linux-only set, but the
  immediately following Elixir-exclusion invocation still passed the absent
  optional Windows glob as a literal required path.

Read-only verification after the failure found no GitLab Release and no
generic package for v2.4.7. OCI
`registry.gitlab.com/janipaijanen/agents-govern-framework/agov:2.4.7` did
publish at digest
`sha256:cad32d081a33e462cad4eba69591072476a4a8da233e467e156ce23c8fc62672`.
Therefore v2.4.7 is immutable and partially published. It must not be described
as wholly unpublished, and neither its Git tag nor OCI tag may be moved,
deleted, or overwritten.

## Decision and rationale

Recover through v2.4.8. Keep the v2.4.7 release contract unchanged:
framework tar/zip, Linux x86_64/aarch64 tooling, their checksum/provenance
sidecars, and OCI are mandatory; Windows remains optional manual preview
evidence.

Change only the downstream exclusion check in `release:create`: validate the
mandatory framework and Linux archives unconditionally, then validate an
`agov-*.zip` only when one is actually present. Do not make
`check-no-elixir-artifacts.sh` silently ignore missing arguments; that would
weaken typo and missing-mandatory-file detection for every caller. Do not
reintroduce a Windows tag build merely to satisfy the stale glob.

This is patch-compatible. No framework behavior, CLI/MCP surface, schema,
exit code, or promised platform is removed. v2.4.8 corrects release plumbing
and release truth for a candidate that never became a complete GitLab Release.

## Exact implementation scope

Implementation changes exactly 25 files.

### Release consumer and public explanation (3)

1. `.gitlab-ci.yml`
   - in `release:create`, pass mandatory `agf` tar/zip and Linux `agov` tar
     archives to `check-no-elixir-artifacts.sh` unconditionally;
   - invoke the checker for `dist/agov-*.zip` only behind the same explicit
     `find ... -name 'agov-*.zip'` presence guard already used by the tooling
     packaging job;
   - change no build, publication, provenance, checksum, OCI, Windows-preview,
     or job rule.
2. `agf/docs/site/reference/release-artifacts.md`
   - replace the release-specific v2.4.7 wording with current-contract wording;
   - state truthfully that v2.4.7 published only its OCI tag before the
     downstream Release-creation failure and that adopters should use v2.4.8.
3. `CHANGELOG.md`
   - add the v2.4.8 repository recovery entry;
   - annotate v2.4.7 as tagged and partially published (OCI only), with no
     GitLab Release or generic package.

### Standard release-truth surfaces (22)

4. `agf/VERSION` to `2.4.8`.
5. `agf/CHANGELOG.md`: add v2.4.8; correct v2.4.7 from “no OCI release” to
   partial OCI-only publication and direct adoption to v2.4.8.
6. `agf/templates/AGENTS.md`: release comment to 2.4.8.
7. `tooling/rust/Cargo.toml`: workspace version and three internal path
   dependency requirements to 2.4.8.
8. `tooling/rust/Cargo.lock`: only the five workspace package versions to
   2.4.8; no external dependency or checksum changes.
9. `agf/docs/site/roadmap.md`: make v2.4.8 the latest published baseline and
   describe v2.4.7 as the immutable OCI-only predecessor.
10–21. All 12 `agf/agents/*.yaml` definition `framework_version` fields to
   2.4.8; definition IDs and role versions remain unchanged.
22. `agf/agents/MANIFEST.yaml`: live framework-version header to 2.4.8 only.
23. `agf/ADOPTING.md`: copyable `framework_version` to 2.4.8.
24. `agf/templates/learning-record.yaml`: copyable framework version to 2.4.8.
25. `agf/docs/governance/manifest.md`: live release claims and upgrade example
   to 2.4.8; historical v2/v1 statements remain historical.

No other file is in scope. In particular, do not modify release scripts,
schemas, Rust source, dependencies, governance rules, optional Windows jobs,
or either existing tag/OCI tag.

## Gate 2 evidence before tagging

At the exact implementation head:

1. The diff contains exactly the 25 files above and passes `git diff --check`.
2. GitLab CI lint accepts MR, default-branch, and tag situations. Tag expansion
   contains the mandatory release jobs and no Windows preview job.
3. `release-assets.sh selftest` still proves the Linux/framework/OCI mandatory
   set, optional Windows discovery, and missing-mandatory negative case.
4. Assemble the real `agf:dist:preview` and `rust:tooling:preview` artifacts
   from the exact-head MR pipeline into one Linux-only `dist/`. Execute the
   exact revised Elixir-exclusion commands from `release:create`; they must
   pass without an `agov` zip. This evidence is mandatory because it directly
   reproduces the condition that failed job `15630274271` and would have
   caught the v2.4.7 defect before tagging.
5. Add a temporary valid zip to that assembled fixture and prove the optional
   guarded invocation checks it; remove the fixture afterward. No Windows
   cross-build is required.
6. `agov` and `agov-mcp` self-report 2.4.8, the exact head, and clean source.
7. Cargo metadata shows exactly five 2.4.8 workspace packages and internal
   requirements `^2.4.8`; lockfile changes only those five versions.
8. The full Rust workspace tests, clippy, cargo-deny, CVE scan, `agov validate`,
   agent/adopter/plan/infosec validators, release-path checks, strict docs
   build, framework clean-archive tests, and Elixir exclusion all pass.
9. A semver sweep classifies every remaining older value and finds no stale
   live package-release scalar.
10. The full MR pipeline is green at the exact Gate 2 head. Gate 2 is required
    before merge; no v2.4.8 tag exists before that merge.

## Post-tag verification

After exact-head Gate 2 approval and merge:

1. Create annotated `v2.4.8` once on the resulting `main` merge commit and push
   it once. Never move/delete any existing release tag.
2. Confirm the tag pipeline publishes Linux tooling without a Windows build
   and passes every mandatory test.
3. Confirm `release:create` uploads the exact mandatory framework/Linux/OCI
   set, creates the GitLab Release, and links every checksum/provenance record.
4. Confirm `release:verify` re-downloads the public assets, validates bytes,
   checksums, provenance, packaged-binary self-reporting, and OCI digest/labels.
5. Confirm OCI `2.4.8` exists at a new digest while OCI `2.4.7` remains
   unchanged. Record completion and close issue #95 only then.

## Rollback and stop conditions

Before tagging, revert the whole implementation if any mandatory release
consumer or evidence step fails. After tagging, never rewrite the tag: stop,
preserve public-state evidence, and use a later patch only through a new gate.

Stop and return to Gate 1 if the fix needs a file outside the 25-file list, if
the assembled Linux-only pre-upload fixture still fails, if any v2.4.7 public
state differs from the inventory above, or if implementation would weaken
missing-file detection globally rather than guarding only the optional zip.
