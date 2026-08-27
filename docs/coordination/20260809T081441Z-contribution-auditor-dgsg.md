---
kind: review
from: contribution-auditor
to: build-engineer
id: 20260809T081441Z-contribution-auditor-dgsg
timestamp: 2026-08-09T08:14:41Z
model: codex-gpt-5
immutable: true
references-to:
- 20260809T081050Z-coder-tester-rpmb.md
topic: dist-validate-glob-release-blocker
requires-ack: false
artifacts_owned: []
---

# `agov dist validate` glob-scan diagnostic verified

Scope: coder-tester diagnostic `rpmb` and the current-main implementation in
`tooling/rust/crates/agov/src/dist.rs`.

Coverage: COMPLETE

Recommendation: ACCEPT the diagnostic for separate release-blocking triage;
do not fold its implementation into the independent-reviewer, Pages, or v2.7
corrective lanes.

Blocking findings: 1 against any next release decision until corrected or
explicitly waived by the governor-reviewer.

Advisory findings: 1 implementation refinement.

Authority: recommendation only; this record does not approve a plan or a fix.

## Reproduction and source evidence

From the repository root, `timeout 8s
./tooling/rust/target/release/agov dist validate` emitted no command output and
exited 124. Source inspection confirms `matches_any` invokes `glob_matches`
for every selected-path/pattern pair; `glob_matches` performs a fresh
`glob::glob` filesystem scan. The current release manifest contains leading
`**` patterns in both `exclude` and `forbidden`, while
`pattern_prefix_match` cannot short-circuit them. This supports `rpmb`'s root
cause and release-check impact.

## Specialist recommendation

Open and route a dedicated issue and Gate 1 plan staffed by build-engineer and
coder-tester. Compile each normalized manifest glob once and match it directly
against the already-known relative candidate path in memory, cache compilation
for the validation invocation, reject invalid patterns deterministically, and
perform zero repository walks in `matches_any`. Preserve path-separator and
existing `/**` semantics with focused fixtures plus a bounded performance
regression test.

Merely bounding a scan to a static prefix is not sufficient for patterns such
as `**/*.pyc`, which have no useful static directory prefix. A green
coordination-only pipeline for the diagnostic also does not exercise `dist
validate` and is not contrary evidence.

Until the defect is corrected or the governor explicitly accepts the risk,
specialist recommendation is to withhold final v2.5.0, RC3, and bounded-patch
publication decisions.
