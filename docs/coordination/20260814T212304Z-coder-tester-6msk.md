---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260814T212304Z-coder-tester-6msk
timestamp: 2026-08-14T21:23:04Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260809T210926Z-governor-reviewer-pqjt.md
  - 20260809T214119Z-contribution-auditor-6tj3.md
  - 20260810T045208Z-contribution-auditor-es1p.md
topic: proxmox-rootless-podman-runner-gate1-planning
requires-ack: true
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/109
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Produce a bounded Gate 1 plan for issue 109 without assigning it to a numbered release.
    - Separate hardware acquisition, Proxmox provisioning, runner trust-class deployment, and CI migration into independently approved and reversible phases.
    - Require contribution-auditor and infosec consultation before Gate 1 submission.
  must_read:
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/109
    - .gitlab-ci.yml
    - docs/coordination/20260809T210926Z-governor-reviewer-pqjt.md
    - docs/coordination/20260809T214119Z-contribution-auditor-6tj3.md
    - docs/coordination/20260810T045208Z-contribution-auditor-es1p.md
  do_not_touch:
    - .claude/settings.local.json
    - numbered release scope
    - OpenClaw issue 104
---

# Plan an isolated Proxmox-hosted rootless-Podman runner fleet

The Governor directed us to proceed with the proposed unnumbered operational
backlog work and supplied a refurbished-workstation catalogue as a potential
Proxmox host source. Issue #109 now records the evidence, boundaries, current
hardware shortlist, and minimum host envelope.

The committed CI distinction must remain explicit: Podman owns OCI
build/scan/push/verification, while two Rust packaging templates still use
`docker:dind` for `cross build`. The v2.5.0 Option D evidence proved that the
unchanged cross-build can run using rootless Podman with
`CROSS_CONTAINER_ENGINE=podman` and `CROSS_REMOTE=0`, but that external
adaptation was retired and never changed the committed pipeline.

Prepare a Gate 1 plan that preserves the following boundaries:

1. Issue #109 stays unmilestoned and outside all frozen numbered releases.
2. Discovery and planning do not authorize buying a host, provisioning
   Proxmox, registering a runner, changing CI, or consuming a release secret.
3. A binding Governor decision is required before hardware purchase and again
   before infrastructure or CI mutation.
4. Public/untrusted merge-request execution must not share a persistent host
   trust boundary with protected or privileged release execution.
5. Hosted GitLab evidence remains available for independently selected
   release/security corroboration and rollback.
6. OpenClaw remains paused; the RTX A4000 candidate must not be justified by
   speculative local-model or OpenClaw use.

The plan should define measurable acceptance and stop conditions for clean-VM
Podman cross-build equivalence, VM isolation, runner tags/protected-ref
routing, cache and token containment, power/storage economics, backup/rebuild,
and hosted-minute savings. Treat product availability and prices as a dated
snapshot requiring revalidation immediately before any purchase decision.
