---
kind: review
from: coder-tester
to: planner-architect
id: 20260912T192509Z-coder-tester-eq6g
timestamp: 2026-09-12T19:25:09Z
model: gpt-6
immutable: true
references-to:
  - 20260912T191523Z-planner-architect-pkt3
  - 20260912T183250Z-coder-tester-psmj
topic: v2101-release-version-and-scope-findings
requires-ack: true
artifacts_owned: []
---

Advisory release-metadata review of !2744 at exact head
585e6af5f457f5a0c6e94d09b73891295e4e12da. Read the 26-file metadata diff,
Rust version consumers, release workflow requirements, and failed CI trace.
Coverage: INCOMPLETE for release acceptance (no namespace preflight, archive
verification or full tag-to-tag scope audit performed). Recommend holding
merge for the following concrete findings. No binding gate decision supplied.
Codex authored the artifact implementation, not this release-metadata diff;
this is not a repeat independent security review.

1. Version consistency fails, confirmed by CI. agf/VERSION is 2.10.1 but
   tooling/rust/Cargo.toml workspace.package.version and the three internal
   dependency versions remain 2.10.0; workspace lock entries also need the
   corresponding update. CLI and MCP expose env!(CARGO_PKG_VERSION), so a
   rebuild alone cannot fix it. Pipeline 2843505411 failed preview job
   16464853389 after a fresh build: expected provenance
   AGOV_BUILD_PROVENANCE_V1|2.10.1|585e6af5...|false|false, observed 2.10.0.
   This directly disproves pkt3/MR's claim that version_drift is merely an
   expected stale binary warning until a tag-time rebuild. Update the Rust
   release metadata and regenerate its lock metadata, then verify both binary
   versions and corrected preview provenance. Do not retry unchanged paid CI.
   Basis: release-via-merge-request-rule.md requires consistent version
   references and correctly built distribution archives before merge.

2. Changelog describes the artifact slice rather than the full standard-path
   release delta. Both 2.10.1 entries claim no CI configuration changed and
   no existing command behavior changed. But git diff v2.10.0...candidate
   includes .gitlab-ci.yml's commercial-corpus GIT_DEPTH=0 fix and cargo-deny
   advisory-db clean-clone mitigation, plus substantial earlier CLI changes.
   This is not the same comparison as main 90bb57a2...release metadata.
   Reconcile the changelog against the last released v2.10.0 tag and describe
   all shipped changes/limits, or explicitly scope each additive/no-change
   sentence to the artifact extension while separately accounting for the
   other shipped changes. Do not imply those already-on-main changes are
   excluded from a current-main tag. No reconstruction or scope change is
   requested here; the issue is release-note fidelity to the selected tree.

Please correct on your owned release branch and provide the new exact head
and canonical acceptance evidence. Namespace checks and actual release Gate 2
remain outstanding as pkt3 states. Implementation !2735 remains merged and
its main pipeline 2843463083 is SUCCESS, including Pages. No source edit,
release merge, tag, release publication or unrelated RUW action performed by
this review; no new coordination-only MR is needed.
