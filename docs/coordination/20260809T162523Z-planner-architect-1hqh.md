---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
    - release branches, branch protection, versions, tags, packages, OCI, or GitLab Release state
    - current release rule or its projections before Gate 1
    - .claude/settings.local.json
  must_read:
    - docs/coordination/20260809T155404Z-planner-architect-d58p.md
    - docs/coordination/20260809T161825Z-governor-reviewer-2mmn.md
    - agf/rules/workflow/release-via-merge-request-rule.md
    - agf/rules/workflow/immutable-tags-rule.md
  next_agent: infosec-sentinel
  status: ready
  task:
    - Assess the proposed protected maintenance-line exception for branch/tag integrity and supply-chain trust.
    - Return concrete blocking requirements for protection, exact-SHA invalidation, provenance, namespace preflight, retained refs, and metadata-forward synchronization.
id: 20260809T162523Z-planner-architect-1hqh
immutable: true
kind: handoff
model: gpt-5
references-to:
  - 20260809T155404Z-planner-architect-d58p.md
  - 20260809T161825Z-governor-reviewer-2mmn.md
requires-ack: true
timestamp: 2026-08-09T16:25:23Z
to: infosec-sentinel
topic: maintenance-line-release-rule-security-consultation
---

# Security consultation: maintenance-line release exception

The Governor approved planning a narrow exception that permits an immutable
prior release candidate to promote on a protected maintenance target rather
than current `main`. The intended first use is exact-RC2 `v2.5.0` final, but
the rule text must be safe and reusable rather than a release-specific bypass.

Please assess the proposed contract: both target and source start at one
verified immutable release-candidate commit; the target is protected before
any release work; one exact-head-reviewed metadata-only MR creates the tag
commit; tag, Release, generic-package and OCI namespaces are preflighted twice;
tag and branches are retained; all built bytes bind to the reviewed commit;
and a later metadata-only MR makes the tagged commit reachable from current
main without replacing main's newer tree.

Identify any missing fail-closed requirement, authorization ambiguity,
credential/trust-boundary concern, race, ref-substitution risk, or provenance
check. No implementation or release mutation is authorized by this request.
