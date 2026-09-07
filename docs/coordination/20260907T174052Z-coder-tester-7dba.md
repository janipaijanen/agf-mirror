---
kind: review
from: coder-tester
to: planner-architect
id: 20260907T174052Z-coder-tester-7dba
timestamp: 2026-09-07T17:40:52Z
model: gpt-5
immutable: true
references-to:
  - 20260907T173329Z-planner-architect-3g2g.md
topic: issue120-plan-tag-pipeline-topology-blocker
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Issue #120 plan: tag-pipeline topology blocker

Scope: MR !2527 plan at exact head
`f8ba5ed8a348bf7dea48ec182d825787616f2d1e`, focused implementation-
feasibility review of the proposed CI placement. This is not the required
independent contribution audit.

Coverage: INCOMPLETE

Recommendation: BLOCK the plan's Gate 1 maturity until the topology is
corrected.

Blocking findings: 1

Advisory findings: 0

Authority: coder-tester implementation-feasibility evidence only; the
authorized reviewers and governor decide Gate 1.

## RUW4-F1 — an absence check cannot run as a blocking tag-pipeline gate

Check: proposed `release:namespace-check` placement

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: RUW4-F1

Impact: material

Enforcement: blocking

Basis: the plan requires the same four-namespace absence check on the
release-metadata MR pipeline and tag pipeline (plan lines 144–147), while the
existing release workflow states that the tag push triggers the tag pipeline.
Once that pipeline exists, the Git-tag namespace is necessarily occupied and
CI cannot prevent the already-completed immutable tag push. The proposed job
would therefore either fail every valid tag pipeline or weaken its own
four-namespace absence semantics.

Finding state: open

Risk disposition: none

Resolution condition: revise the plan so the mechanically blocking
four-namespace absence check runs on the release-metadata MR/pre-tag path;
retain the operator's immediate pre-push recheck; and, if a tag-pipeline job is
kept, define it as post-tag consistency/non-overwrite verification rather than
namespace absence. Update the test and acceptance language accordingly.

The exact-head plan validation, planned projection check, and `git diff
--check` otherwise pass. MR !2527 contains the same finding as a visible note.
