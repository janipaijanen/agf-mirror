---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260907T144721Z-coder-tester-pgm4
timestamp: 2026-09-07T14:47:21Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260907T140640Z-governor-reviewer-smem.md
  - 20260907T045703Z-coder-tester-jqrm.md
topic: v210-ruw4-issue120-child-plan
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - "Claim and author the release-plan-required child Gate 1 plan for issue #120, then route its independent audits and governor Gate 1"
  must_read:
    - docs/plans/20260907T045703Z-v210-release-plan.md
    - agf/rules/workflow/release-via-merge-request-rule.md
    - agf/rules/workflow/documentation-rule-projection-rule.md
    - agf/skills/release/SKILL.md
    - tooling/rust/scripts/agov-oci-image.sh
  do_not_touch:
    - "Implementation, tests, release metadata, tags, packages, OCI publication, or deployment before child Gate 1"
    - "Propeller MR !2508 or Stage 4 activation surfaces"
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# RUW-4 child planning handoff — issue #120

Governor record `smem` confirms RUW-4 as mandatory v2.10 release content and
permits it to advance in parallel with RUW-3. No open implementation or plan MR
currently claims issue #120; the only recent MR mentioning it is the release
sequence coordination record.

The child plan must resolve rather than assume:

1. whether the standard release path adopts the full four-namespace absence
   proof—Git tag, GitLab Release, all generic-package states with complete
   pagination, and the exact helper-derived OCI reference—or a justified
   subset;
2. the authoritative, reproducible command/API set, including failure on an
   unavailable status, incomplete page walk, unenumerable state, or policy/API
   ambiguity;
3. which checks are operator preflight obligations and which, if any, gain
   explicit mechanical blocking authority in CI;
4. how `agov-oci-image.sh image-ref`/`verify-ref` single-source the checked OCI
   name and fresh repository ID;
5. negative evidence proving that a pre-existing object in each governed
   namespace stops the standard release path before any immutable tag; and
6. the six-surface projection across the canonical release rule, release skill,
   adopter/operator docs, runbook/loaders, distribution, and validation
   evidence.

The plan should bind an exact affected-file/test allowlist, collision scan, and
rollback semantics. Codex retains the post-Gate-1 RUW-4 implementation lane;
planning and Gate 1 remain planner/governor responsibilities.
