---
id: post-v248-release-batching-amendment
status: approved
topic: post-v248-release-batching-amendment
gate1_approved_by: governor-reviewer
gate1_approved_at: "2026-07-31T06:56:54Z"
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: "The reviewed portfolio-reconciliation plan established the item boundaries and SemVer stop conditions. This amendment narrows each release rather than adding portfolio scope; exact implementation children retain their own review requirements."
  - role: infosec-sentinel
    status: not-applicable
    reason: "Release routing only. No executable, credential, learning-record, trust-boundary, or security-control implementation is authorized by this amendment."
---

# Plan amendment: batch the post-v2.4.8 release train

## Binding decision

The governor-reviewer approved replacing the broad v2.5.0 candidate bundle
with smaller SemVer-honest batches:

1. **v2.4.9 — patch-only live-surface correction.** Resolve #93 and other
   independently verified defects only when they add no command, schema field,
   obligation semantic, or other contracted surface.
2. **v2.5.0 — authority and acknowledgement contract core.** #79 and #80,
   plus only the minimum #33/#67/#78 compatibility slices required by those
   contracts. Use rc.1 as the exact-artifact checkpoint; rc.2 only if the
   candidate changes.
3. **v2.6.0 — coordination reliability.** #84, #85, #89, and only justified
   correctness-preserving #76 work. Other read-path reliability work must be
   explicitly added by an approved child plan.
4. **v2.7.0 — MCP and assistant integration.** #86, #87, #88, #90, and #92
   if current-main reproduction still proves the wrapper/registry parity gap.
5. **Later minor releases.** #91, #94, and separately approved backlog themes;
   they are not silently committed to v2.7.0.

Patch releases after each minor contain fixes to that released line only.
Features, new commands, schemas, or obligation semantics advance the minor
version. No release exists merely to empty the tracker.

## Relationship to the v2.5.0 SoW

This amendment supersedes only the SoW's release placement for reliability and
integration candidates. It preserves the SoW's evidence gates, v2.5.0 contract
core, exact-artifact RC rule, upgrade tests, parking boundary, and all
release-wide stop conditions.

## Acceptance criteria

- GitLab milestones, issue ledger, MR ledger, and public roadmap use the same
  release destinations.
- Each release has a bounded adopter-facing character and separately approved
  child implementation/release plans.
- v2.4.9 remains strictly patch-compatible.
- No later-minor candidate is represented as approved implementation scope.
