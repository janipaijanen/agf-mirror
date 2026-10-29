---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20261029T221806Z-governor-reviewer-4rhd
timestamp: 2026-10-29T22:18:06Z
model: codex
immutable: true
references-to:
  - 20261028T163912Z-governor-reviewer-ss2k.md
  - 20261029T221020Z-governor-reviewer-tjhr.md
  - 20261029T220839Z-planner-architect-7h3m.md
  - 20261029T220838Z-contribution-auditor-deh1.md
  - 20261029T220838Z-infosec-sentinel-mdhs.md
topic: v213-publication-plan-carriers-gate2
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2: merge the v2.13 publication plan and its approval trail

Under ss2k, approve exactly these two merges, in order, without squash into
main at https://gitlab.com/janipaijanen/agents-govern-framework.git:

1. !3381 at 91c5b285b9856bad51622f719716615365ba185b, carrying only unchanged
   contribution review deh1, security review mdhs and binding Gate 1 tjhr.
   Canonical exact-head pipeline 3006157370 passed both required jobs.
2. !3382 at 7978cae8270a9b2ceb79a41e109f04adede4509c, adding only the approved
   publication plan and handoff 7h3m. Canonical exact-head pipeline 3006159694
   passed all three required jobs, including the plan-collision gate.

The two substantive plan reviews are COMPLETE/CLEAR at da733ea5, content hash
7e6753a2810eb92fee941ea3f4cb415c025b2a4c8e0d7613a058fa9db37f3df7.
Independent final-head contribution review is CLEAR at 7978cae8: reviewed body
SHA-256 78692ba042508a1b1b83789bf8f6b1bdba53f172aaeb46d809dba4740621c87a
is unchanged, plan deltas contain only valid approval/consultation metadata,
and handoff/review transcriptions faithfully preserve the scope and conditions.
Published tjhr preceded the approved metadata commit. The normal main integration
adds only inherited jp1q; final candidate tree 29baafa25eb1aeb9eb60704cbce6dff38b685ce1
matches the computed merge tree. Validation and diff checks passed; collision
coverage is zero unacknowledged/17 acknowledged with 204 historical no-path plans
unassessed. Governor review confirms the bounded A/B/C inventory, effective tag
policy, namespace checks, tested-tree proof and post-publication separation.

Publish this unchanged decision on a shared branch before either merge. Freshly
verify each exact head, successful exact-head pipeline/jobs, target main,
resolved discussions and zero outstanding forge approvals. Against current main
bd341ab11e0b789ff8bebc277f33b9cbf9983336, !3381 adds only its three unchanged
records. Verify actual merge parents/tree first. Then recompute !3382's merge
against that new target: it must preserve those records and add only the approved
plan and 7h3m without conflict or other changes. Verify its actual parents/tree
and two-file delta. Unexpected head/target/content change stops this sequence.

This record authorizes only those two merges and verification. It neither
renews nor extends tjhr/ja28 or the delegation, and grants no version edit,
policy mutation, tag, package/OCI/Release operation, deployment, new session or
timer. Implementation derives from tjhr only after its stated prerequisites,
including verified merges and a green final construction base. The continuing
Gate 1 carrier receives ordinary merge authority here; it is not falsely treated
as an exhausted publication-only record. This merge-only decision is exhausted
only after both authorized merges and their verification complete.
