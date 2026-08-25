---
id: issue109-runner-fleet-phasing
status: pending-gate1
topic: issue109-proxmox-runner-fleet-phasing
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: contribution-auditor
    status: waived
    reason: >-
      Single-agent Gate 1 planning session; no contribution-auditor
      available to consult directly at plan-filing time. Review is
      explicitly requested via a coordination handoff filed alongside
      this plan. The main risk this review should verify: that no phase
      grants itself authority to acquire hardware, provision
      infrastructure, or change CI ahead of its own dedicated Gate 1,
      per issue #109's explicit non-goals.
  - role: infosec-sentinel
    status: waived
    reason: >-
      Single-agent Gate 1 planning session; infosec-sentinel not
      available to consult directly at plan-filing time. Review is
      explicitly requested via the same coordination handoff. The main
      risk this review should verify: that the runner trust-class
      boundary (unprivileged / protected-ephemeral / hosted-retained)
      and the threat-model checklist in Phase C are sufficient before
      any privileged or secret-bearing job is ever scheduled on
      self-managed infrastructure.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: issue #109 phasing — Proxmox runner fleet and Podman CI migration

## Decision requested

Approve a four-phase governance structure for issue #109 (Proxmox-hosted
isolated runner fleet and rootless-Podman CI migration), where each phase
is independently Gate-1-approved before it starts and independently
reversible. This plan itself authorizes only Phase B (a zero-hardware
technical feasibility reproduction). It does not authorize a hardware
purchase, Proxmox provisioning, a runner installation, or any `.gitlab-
ci.yml` change — consistent with issue #109's own non-goals ("No purchase
or provisioning before a binding governor decision").

This is a Gate 1 plan only. It does not authorize implementation beyond
Phase B, approve any later phase's Gate, merge, release, tag, or deploy
anything.

## Problem and evidence boundary

The committed pipeline uses two container-engine paths across two job
sites in `.gitlab-ci.yml`: Podman for OCI build/scan/push/verify, and
`docker:dind` + `DOCKER_HOST`/`CROSS_REMOTE=1` for Rust `cross build`
packaging — the shared `.rust-tooling-package` template
(`.gitlab-ci.yml:1077-1090`, consumed by `rust:tooling:release` and the
OCI-image job) and the manual `rust:tooling:build:windows:preview` job
(`.gitlab-ci.yml:1341-1351`). Both are the exact two sites issue #109
names.

The v2.5.0 "Option D" release evidence
(`20260809T214119Z-contribution-auditor-6tj3`,
`20260810T045208Z-contribution-auditor-es1p`) already demonstrated,
under a temporary protected project runner, that the same unchanged
cross-build succeeds with `CROSS_CONTAINER_ENGINE=podman` and
`CROSS_REMOTE=0` instead of `docker:dind`. That adaptation was
intentionally external and temporary; the repository still defaults to
Docker-in-Docker today. Hosted CI is now budget-constrained per issue
#109, motivating a durable, governed replacement rather than a repeated
ad hoc Option D each release.

The Human Governor is independently researching host hardware in the
issue thread itself (2026-08-14/15 comments: candidate machines, a
64 GB DDR5 filter revision, and a list of required pre-purchase
questions). This plan does not repeat, second-guess, or attempt to
finalize that hardware decision — Phase A below explicitly names it as
Human-Governor-led, outside this plan's authority.

## Phase structure (each phase independently Gate 1-approved and reversible)

| Phase | Name | Authorizes | Reversal if abandoned |
|---|---|---|---|
| A | Hardware acquisition | A specific host purchase, once the Human Governor's pre-purchase question list (issue #109 comment, 2026-08-15) is answered for the chosen listing | No system state exists yet; simply do not purchase |
| B | Podman rootless feasibility reproduction | Reproducing the required Linux `cross build` (and, if pursued, the manual Windows preview) on a clean rootless-Podman environment, with no purchased host required | Discard the reproduction environment; no CI or production state touched |
| C | Proxmox provisioning + runner trust classes | Installing Proxmox on the Phase A host, defining and provisioning the three trust classes issue #109 names (unprivileged/secret-free; protected/ephemeral packaging; hosted-retained release evidence), completing the Phase C threat model | Snapshot/rebuild or destroy the VMs; the Proxmox host reverts to unprovisioned; no `.gitlab-ci.yml` change has happened yet |
| D | CI migration | Incrementally repointing named jobs from `docker:dind` to the Phase C runner classes, one job at a time, with hosted GitLab runners retained as the default until each swap has independent acceptance evidence | Revert the specific job's `tags:`/`services:` back to the prior hosted configuration; `docker:dind` definitions stay in the file, unused, until the whole migration is accepted |

Phases run in this order because each later phase depends on the
previous phase's evidence (C needs A's host; D needs both B's technical
feasibility proof and C's provisioned trust classes). A phase's approval
does not pre-approve the next phase — each requires its own Gate 1
submission naming its own exact scope, evidence, and rollback.

**This plan authorizes Phase B's execution once approved.** Phases A, C,
and D each require a separate future Gate 1 plan.

## Phase B — scope (the only phase this plan authorizes)

**What:** Reproduce the required Linux `x86_64`/`aarch64` `cross build`
packaging steps that `.rust-tooling-package` currently runs under
`docker:dind`, instead under rootless Podman with
`CROSS_CONTAINER_ENGINE=podman`/`CROSS_REMOTE=0` — the same combination
Option D already proved once, but reproduced fresh, deliberately, and
documented as a durable feasibility record rather than an emergency
workaround. Optionally attempt the manual Windows preview job under the
same substitution if time and evidence quality allow; if Windows
reproduction fails or is skipped, that is an explicit finding for the
Phase D plan, not a Phase B blocker.

**Where it runs:** Any available rootless-Podman-capable Linux
environment reachable today — a disposable local VM, container, or cloud
sandbox. Explicitly not the Phase A host (not yet purchased) and not a
runner registered against this project's GitLab instance. No CI
configuration changes; no runner registration; no project credentials
beyond what a normal local build already needs (source checkout only —
no protected CI variables).

**What it produces:** A written feasibility record (a coordination
completion or a short report checked into `docs/plans/` as a Phase B
evidence appendix) stating: exact Podman version, exact `cross`
version, exact command sequence, pass/fail per target
(`x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl`, and
Windows if attempted), and any deviation from the Option D evidence
already on record. This record is the acceptance evidence Phase C and D
plans will cite — it does not itself authorize either.

**Explicitly out of scope for Phase B:** any hardware purchase, any
Proxmox installation, any `.gitlab-ci.yml` edit, any GitLab runner
registration, any change to project CI variables or protected refs.

## Affected files

- New: this plan file only, for Gate 1.
- Phase B execution (once approved) produces a feasibility record under
  `docs/plans/` as a dated appendix or follow-up note — no code, schema,
  or CI file changes.
- No `.gitlab-ci.yml` change in this plan or in Phase B.

## Migration needed

No. No schema, data, or code change in this plan or in Phase B.

## Threat-model checklist (per issue #109 discovery slice 6 — tracked here for Phase C, not resolved by this plan)

Recorded now so Phase C's future plan has a fixed starting checklist and
cannot silently narrow issue #109's own requirement list:

- Public merge-request code executing on any runner class.
- Privileged execution / container-escape surface per trust class.
- Poisoned build/dependency caches shared across jobs or VMs.
- Runner-registration-token exposure and rotation.
- Lateral movement between VMs on the same Proxmox host.
- Proxmox management-plane access (network exposure, credential storage).
- No personal AI credentials, developer SSH keys, or general-purpose PATs
  in any runner VM (issue #109's explicit hard constraint).

## Acceptance matrix

| Phase | Reviewable question | Fails when |
|---|---|---|
| B (this plan's scope) | Does the feasibility record name exact tool versions, exact commands, and a pass/fail per target, reproducible from the record alone? | Any target's result is asserted without the record, or the record silently reuses Option D's original evidence instead of a fresh reproduction |
| Phase boundary (all) | Does any phase's execution touch a later phase's authorized scope (e.g., Phase B touching CI config, Phase C purchasing hardware)? | Yes — this plan is violated even if the touched action is individually reasonable |
| Rollback (all) | Is the stated reversal for this phase actually sufficient to return to the pre-phase state with no residual CI, secret, or infrastructure exposure? | The rollback column above is wrong or incomplete for a phase once that phase is actually planned in detail |

## Downstream impact classification (`downstream-impact-rule.md`)

This plan does not touch `.claude/rules/**/*.md`, `agf/rules/**/*.md`,
`agf/agents/*.yaml`, `agf/schemas/*.json`, or `AGENTS.md` — it is a
plan/coordination-shaped change to `docs/plans/` only, outside this
rule's trigger paths. No impact statement is required; noted here for
reviewer clarity given the plan's operational subject matter.

## Test strategy

No automated test applies to the phase-structure plan itself. Phase B's
own acceptance evidence (exact tool versions, exact commands, pass/fail
per target) is the test strategy for that phase, per the acceptance
matrix above.

1. `agov plan validate` on this plan file.

## Risks

- **Risk:** Phase boundaries could blur in practice — e.g., "just try it
  on the real host" once Phase A hardware arrives, before Phase C's own
  Gate 1. **Mitigation:** the acceptance matrix's phase-boundary row is
  an explicit, standing reviewable question at every later phase's Gate
  1 and Gate 2.
- **Risk:** Phase B's feasibility record could be treated as sufficient
  evidence to skip Phase C's threat model. **Mitigation:** the threat-
  model checklist above is recorded now, independent of Phase B's
  outcome, and Phase C's future plan must address every item on it
  explicitly, not merely cite Phase B's success.
- **Risk:** This plan could be read as implicitly endorsing a specific
  hardware candidate from the issue's ongoing comment thread.
  **Mitigation:** Phase A is explicitly named as Human-Governor-led and
  this plan makes no hardware recommendation of its own.

## Rollback

This plan's own rollback: if Phase B's reproduction fails or the
approach is abandoned, no system state exists to unwind — discard the
disposable reproduction environment. If the phase structure itself is
later found wrong, a superseding plan replaces it; this plan is not
self-implementing beyond Phase B.
