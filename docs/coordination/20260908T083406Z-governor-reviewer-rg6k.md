---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260908T083406Z-governor-reviewer-rg6k
timestamp: 2026-09-08T08:34:06Z
model: human
immutable: true
references-to:
  - 20260908T082056Z-coder-tester-6phq.md
  - 20260907T082727Z-governor-reviewer-kg6s.md
  - 20260907T151325Z-governor-reviewer-2emq.md
topic: v210-preflight-sequence-amendment-gate1-approved
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 approved — v2.10 reversible-preflight sequence correction

Binding governor instruction in this Codex task on 2026-09-08:
**"gate 1 approved. proceed"**. Codex transcribes that human decision; it is
not an AI-issued approval or same-role self-ack.

The approval binds the sequence-only correction proposed in handoff `6phq` to
the existing approved v2.10 release work order. The correction is:

1. reconstruct the release-plan allowlisted metadata as an uncommitted draft
   at a freshly verified green `main` base;
2. run every reversible validation that supports the uncommitted draft, except
   the namespace command whose owner contract requires a clean commit;
3. after the version and changelog gates are satisfied, create one unpublished
   local-only metadata commit in the disposable release worktree;
4. run `agov release namespace-check --phase preflight` from that exact clean
   commit before any branch push or merge-request creation;
5. on failure, delete the disposable worktree and unpublished commit; on
   success, retain that exact commit as the release branch head and proceed to
   the standard release MR.

An unpublished local-only commit under this amended sequence does not allocate
or burn a version namespace. Public allocation still begins only when the
proven release branch is pushed. The namespace check's clean-tree, source-SHA,
target-SHA, project-identity, package-pagination, OCI-derivation, and
protected-tag-policy contracts remain unchanged. No rule, skill, command,
test, CI, or release content is amended by this decision.

This approval accepts the disclosed evidence limitation that no new advisory
review record was produced between `6phq` and this binding decision; the
correction preserves rather than relaxes the already reviewed mechanical
guard. Any implementation change remains outside scope and requires its own
plan, reviews, and negative evidence.

This is Gate 1 for the sequence correction only. It does not approve the
release metadata diff or changelog, Gate 2, a push, merge, tag, Release,
package, OCI publication, deployment, or issue #120 closure.
