---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 01KVK9FQVE55BEEPTV796ZZ2MT
timestamp: 2026-06-20T19:53:00Z
model: claude-sonnet-4-6
immutable: true
references-to:
  - 01KVK9CH15S9R2957ACQKMB1KH-coder-tester.md
  - 01KVK8WQ8TSHBH31FNGC5BMXXD-planner-architect.md
topic: rust-phase1-macos-deferral-scope-change-gate2-routing
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Acknowledge that dropping macOS preview builds from MR !31 Phase 1
      scope is acceptable before the MR merges.
  must_read:
    - docs/coordination/01KVK9CH15S9R2957ACQKMB1KH-coder-tester.md
    - docs/plans/rust-agov-port.md (§9 Distribution strategy)
    - .gitlab-ci.yml (rust:tooling:build:preview job)
  do_not_touch:
    - plan/readable-coordination-ids
---

# Gate 2 routing: macOS preview build deferral — scope change in MR !31

The previous two post-Gate-2 CI fixes (`01KVK8HKZB64E042HGQSJA8FSW`,
`01KVK8WQ8TSHBH31FNGC5BMXXD`) were CI environment fixes only and did not
require renewed review. This commit (`be39e01`) is different: it changes
the delivery scope of MR !31 and updates the plan document. It requires
governor-reviewer acknowledgement before merge.

## What changed

`docs/plans/rust-agov-port.md` §9 and `rust:tooling:build:preview` in
`.gitlab-ci.yml` previously included four platform targets:

- `x86_64-unknown-linux-musl` ✓ (Phase 1, builds successfully)
- `aarch64-unknown-linux-musl` ✓ (Phase 1, builds successfully)
- `x86_64-apple-darwin` → **deferred** (removed from Phase 1)
- `aarch64-apple-darwin` → **deferred** (removed from Phase 1)

The plan now documents macOS as deferred until a separate decision covers
the required macOS SDK/custom-image supply chain. Windows remains deferred
to Phase 3 (unchanged).

## Why the change is justified

`cross` does not provide a Docker image for Darwin targets. Building macOS
binaries via `cross` in GitLab CI requires a custom Docker image that
bundles a macOS SDK (typically via `osxcross`). That is a separate
supply-chain decision (tool pinning, SDK license, image hosting) that was
not part of the original Phase 1 plan scope and cannot be resolved within
this MR.

## What this is NOT

This is not a scope increase or a behavioural change to `agov`. The Linux
musl preview builds (the primary deployment target for Linux users) work
correctly. The change removes something from Phase 1 that cannot be
delivered without an out-of-scope supply-chain decision.

## Question for governor-reviewer

Is dropping macOS preview builds from MR !31 Phase 1 scope acceptable?
(Linux preview targets continue; macOS deferred to a future decision on the
SDK/image supply chain.)

A simple "yes, proceed" or "yes, merge" is sufficient. If not acceptable,
please specify what macOS build approach should be used instead.
