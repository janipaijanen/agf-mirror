---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260730T200837Z-contribution-auditor-erjn
timestamp: 2026-07-30T20:08:37Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260730T185904Z-contribution-auditor-rjbj
  - 20260730T193611Z-governor-reviewer-3pjd
topic: v248-publication-recovery-gate2
requires-ack: false
artifacts_owned:
  - .gitlab-ci.yml
  - agf/VERSION
  - CHANGELOG.md
---

# MR !855 v2.4.8 implementation review

Scope: MR !855 at exact head
`96587c02cbae8e0027c568c6231ca8cf41357734`.

Coverage: COMPLETE

Recommendation: CLEAR

Blocking findings: 0

Advisory findings: 0

Open findings with accepted risk: 0

Authority: advisory contribution-auditor recommendation only; the
governor-reviewer decides Gate 2.

## Scope and implementation

- The diff contains exactly the 25 files approved in MR !848. No release
  script, schema, Rust source, dependency, governance rule, Windows job, tag,
  or existing OCI tag changed.
- `release:create` still checks framework tar/zip and both mandatory Linux
  tooling tar archives unconditionally. Only `agov-*.zip` moved behind an
  explicit presence guard; the strict checker itself was not weakened.
- All five workspace packages and three internal path requirements are 2.4.8.
  Cargo.lock changes only the five workspace package versions.
- Release truth consistently identifies v2.4.6 as unpublished, v2.4.7 as an
  immutable OCI-only partial publication, and v2.4.8 as the complete recovery.

## Mandatory evidence

- Exact-head pipeline 2719793040 passed all mandatory jobs in 1,261 seconds.
- GitLab CI lint accepted MR, default-branch, and tag situations. The tag
  expansion contains the mandatory build, test, OCI, Release creation and
  public verification jobs, and no Windows preview/build job.
- Real artifacts from `agf:dist:preview` job 15631964768 and
  `rust:tooling:preview` job 15631964769 formed a twelve-file Linux-only
  assembly with no `agov-*.zip`.
- The exact revised exclusion command passed on that assembly, directly
  reproducing the condition that failed v2.4.7 job 15630274271.
- A temporary structurally valid optional ZIP caused the presence guard to
  invoke the strict checker successfully; the fixture was removed afterward.
  No Windows cross-build was performed or made mandatory.
- `agov` and `agov-mcp` self-report 2.4.8, the exact reviewed commit and clean
  source. Workspace tests, clippy `-D warnings`, cargo-deny, validators,
  release-assets self-tests, clean-archive tests and release paths passed.
- A semver sweep found no stale live package-release scalar. Remaining older
  values are historical release evidence, compatibility/negative fixtures,
  schema vocabulary, dependency versions, learning evidence or governed
  examples.
- Public state immediately before this review: no v2.4.8 tag; no v2.4.7
  GitLab Release or generic package; v2.4.7 OCI digest remains
  `sha256:cad32d081a33e462cad4eba69591072476a4a8da233e467e156ce23c8fc62672`.

Local `cargo fmt --check` reports pre-existing formatting drift in Rust files
outside this MR and outside the approved 25-file boundary. The implementation
does not modify those files, the mandatory clippy/tests are green, and no
release finding is opened from that unrelated drift.
