---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260825T192424Z-governor-reviewer-g5jj
timestamp: 2026-08-25T19:24:24Z
model: human
immutable: true
references-to:
  - 20260825T165350Z-planner-architect-asm2.md
  - 20260816T082118Z-governor-reviewer-gm2h.md
  - 20260815T232500Z-coder-tester-85hh.md
  - 20260816T000505Z-coder-tester-s3bq.md
  - 20260816T010548Z-coder-tester-sq33.md
  - 20260815T213022Z-governor-reviewer-tgjs.md
topic: mr806-mr807-infosec-waivers-approved
requires-ack: false
artifacts_owned: []
---

# Plan-specific infosec consultation waivers approved for MRs !806 and !807

This record transcribes the governor-reviewer's explicit response to the
decision-ready proposal and `asm2` handoff:

> waivers approved. proceed

## Decision A — MR !806

For MR !806, plan
`docs/plans/agov-mcp-transport-protocol-correctness.md` at exact head
`78fd21af22961f36a7000a064dd9202a33720e7d`, the governor-reviewer waives the
plan-specific infosec-sentinel semantic consultation prerequisite.

The governor accepts the bounded review-aid chain `h6qr` -> `83tn` -> `85hh`
and exact-head continuity record `sq33` as sufficient plan-stage security
evidence. The chain identified and resolved the unbounded delimiter-free read
and same-chunk over-cap/newline transition defects. No blocking finding,
advisory finding, or accepted residual risk remains in that review scope.

## Decision B — MR !807

For MR !807, plan `docs/plans/assistant-discovery-archive-setup.md` at exact
head `6cf2aac8ad0f92c5d2098faa5f4547631fbeb207`, the governor-reviewer waives the
plan-specific infosec-sentinel semantic consultation prerequisite.

The governor accepts the bounded review-aid chain `87k8` -> `p371` -> `s3bq`
and exact-head continuity record `sq33` as sufficient plan-stage security
evidence. The chain identified and resolved the cross-platform absolute-path,
open-file read-bound, and advisory-outcome consistency defects. The shipped
`.mcp.json` surface is minimal and contains no secrets, environment values,
paths, endpoints, arguments, credentials, or added runtime authority. No
blocking finding, advisory finding, or accepted residual risk remains in that
review scope.

## Authorized next action and conditions

The two dispositions above are independently scoped. Planner-architect is
authorized to change only each named plan's infosec-sentinel consultation
metadata from its unresolved placeholder to `waived`, cite this decision, and
record the waiver boundary. Each resulting plan head must independently:

1. remain within its existing two-issue corrective scope;
2. pass the locally built Rust `agov plan validate` and exact-head CI;
3. preserve the current contribution-auditor evidence and every resolved
   technical and security finding;
4. receive fresh independent planner-architect READY and coder-tester CLEAR
   statements at the new exact SHA; and
5. be named in a self-contained Gate 1 activation record before implementation,
   under `tgjs`.

## Boundaries

These are plan-specific consultation waivers and prerequisite-resolution
decisions. They are not a standing infosec waiver, role change, finding
suppression, or general authorization to substitute CI for semantic review.

This decision does not itself approve Gate 1, implementation, Gate 2, merge,
direct protected-branch changes, the proposed cumulative-main release model,
scope expansion, a version change, tag, release, package or OCI publication,
deployment, credential use, registry mutation, or remote-branch deletion.
Those remain separately governed decisions.
