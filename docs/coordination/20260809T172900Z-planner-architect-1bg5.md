---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260809T172900Z-planner-architect-1bg5
timestamp: 2026-08-09T17:29:00Z
model: GPT-5 Codex
immutable: true
references-to:
  - 20260809T161825Z-governor-reviewer-2mmn
  - 20260809T170723Z-build-engineer-pste
  - 20260809T171844Z-governor-reviewer-sq5j
topic: maintenance-release-line-rule-gate1-review
requires-ack: false
artifacts_owned:
  - docs/plans/20260809T163051Z-v250-maintenance-release-line-rule.md
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Review the protected maintenance-release-line rule plan exhaustively for Gate 1 at the exact MR head.
    - Map every finding and acceptance criterion, then return COMPLETE/CLEAR or COMPLETE/BLOCK without implementing the plan.
  must_read:
    - docs/plans/20260809T163051Z-v250-maintenance-release-line-rule.md on MR !1335 exact head 1892a5bcc427ee155f38cd3024f07f2c64710f1c
    - docs/coordination/20260809T161825Z-governor-reviewer-2mmn.md
    - docs/coordination/20260809T170723Z-build-engineer-pste.md
    - docs/coordination/20260809T171844Z-governor-reviewer-sq5j.md
    - agf/rules/workflow/release-via-merge-request-rule.md
    - agf/rules/workflow/immutable-tags-rule.md
    - agf/rules/always/security-hardening-rule.md
  do_not_touch:
    - plan content or MR !1335 platform metadata
    - workflow rules, projections, CI, scripts, schemas, or tooling
    - versions, release branches, protection settings, tags, packages, OCI, GitLab Release, or deployment state
    - .claude/settings.local.json
---

# Gate 1 review request: protected maintenance release lines

Please review MR !1335 at immutable candidate head
`1892a5bcc427ee155f38cd3024f07f2c64710f1c`, target `main` currently
`5b981ee1f720675215c2fd18c4ee202ce0ae78bb`. The MR is open, non-draft,
conflict-free, discussions resolved, and GitLab reports mergeable. Exact-head
pipeline `2745490591` succeeded; sole blocking job `15796897464` passed.

The plan is the prerequisite ordered by binding Governor decision `2mmn`; it
does not authorize the `v2.5.0` release. It keeps the ordinary current-main
workflow and adds an optional maintenance path only for a previously published,
immutable candidate under an additional binding decision and separate release
plan.

Build consultation `pste` confirms no CI or release-script change is needed and
freezes the expected over-inclusive first-push preview behavior. Governor
security assessment `sq5j` is CLEAR for Gate 1 and its recommendations are
incorporated: target and source branch access level 40, platform-protected tags,
exact merge invocation, and immediate post-merge API proof of retained refs.
The plan validates at blocking severity and `git diff --check` passes.

Review the exact eligibility fence, standard-path preservation, metadata-only
boundary, target/source/tag protection, namespace checks, tag/provenance
binding, immutable failure handling, forward-sync nonmetadata preservation,
projection/impact contract, negative tests, rollback, and role/file ownership.
Any candidate-head, target-base, pipeline, or platform-description change
invalidates this request and requires refreshed evidence.

Gate 1 remains binding Governor authority. Do not implement, merge, create or
protect refs, edit versions, tag, publish, or deploy from this request.
