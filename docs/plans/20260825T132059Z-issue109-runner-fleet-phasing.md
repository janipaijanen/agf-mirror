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
      Round 2 correction (per coder-tester exact-head review `rbkh`,
      finding B-I109-5): round 1's claim that a contribution-auditor
      handoff had "already been requested via a coordination handoff
      filed alongside this plan" was false — no such handoff existed.
      A genuine request is now filed alongside this correction
      (`20260826T055434Z-planner-architect-i9ca`, see coordination channel).
      `status` stays `waived` only because the schema's enum has no
      `pending`/`requested` value and no response has returned yet; this
      is a provisional filing-time placeholder, not a final Gate 1
      disposition — Gate 1 approval must not proceed while this still
      reads `waived` for single-agent unavailability. The main risk this
      review should verify: that no phase grants itself authority to
      acquire hardware, provision infrastructure, or change CI ahead of
      its own dedicated Gate 1, per issue #109's explicit non-goals.
  - role: infosec-sentinel
    status: waived
    reason: >-
      Round 2 correction (per coder-tester exact-head review `rbkh`,
      finding B-I109-5): the current infosec-sentinel player is CI only
      (`agov validate infosec`, per `agf/agents/MANIFEST.yaml`), which
      enforces mechanical checks but cannot return the human-judgment
      self-hosted-runner threat-model review this plan actually needs. A
      green governance job is not that consultation. A binding governor
      disposition is requested (see the coordination request filed
      alongside this correction) to either activate an appropriate
      advisory reviewer, waive this Phase B plan's consultation while
      reserving mandatory review for Phase C, or accept another named
      evidence path. `status` stays `waived` as a provisional
      placeholder until that binding disposition is recorded and cited
      here by reference. The main risk this review should verify: that
      the runner trust-class boundary (unprivileged / protected-
      ephemeral / hosted-retained) and the threat-model checklist in
      Phase C are sufficient before any privileged or secret-bearing job
      is ever scheduled on self-managed infrastructure.
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

**Actual dependency graph (corrected per coder-tester exact-head review
`rbkh`, finding B-I109-1):** the phase letters A-B-C-D are labels, not a
total execution order. A (hardware acquisition, Human-Governor-led) and
B (Podman rootless feasibility reproduction) are independent lanes —
neither depends on the other, and this plan authorizes B now precisely
because it requires no purchased host. C requires A's provisioned host.
D requires both B's accepted feasibility evidence and C's provisioned
trust classes. Concretely: A and B may proceed in either order or in
parallel; C cannot start before A completes; D cannot start before both
B and C have independent acceptance evidence. Approving B here does not
imply, authorize, or schedule A, C, or D — each still requires its own
future Gate 1 submission naming its own exact scope, evidence, and
rollback.

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

**Where it runs (bounded per coder-tester exact-head review `rbkh`,
finding B-I109-3):** an already-available, no-additional-cost,
non-production, disposable rootless-Podman-capable Linux environment
under the operator's existing authority — a local VM or container on
already-owned/already-provisioned hardware. Explicitly not the Phase A
host (not yet purchased) and not a runner registered against this
project's GitLab instance. Uses only public source and public
dependencies; no project secrets or protected CI variables. Any of the
following stops Phase B and requires separate authority before
proceeding: creating a new account of any kind, provisioning or paying
for external compute or storage, using a protected CI variable or any
project credential beyond an ordinary local source checkout, using a
private/paid dependency credential, or retaining a produced artifact
outside this repository's own `docs/plans/` or coordination channel. A
"cloud sandbox" is not a default option under this plan — it requires
the same stop-and-authorize path as any other new paid or externally
hosted resource.

**What it produces (extended per coder-tester exact-head review `rbkh`,
finding B-I109-4):** a single written feasibility record, checked into
`docs/plans/` as a dated Phase B evidence appendix (the durable evidence
carrier — not a coordination completion, which is not designed to hold
this much pinned technical detail), containing enough pinned inputs to
be reproducible from the record alone and useful for future host
capacity/cost sizing:

- Pinned inputs: exact Podman version, exact `cross` version, the exact
  source revision built, the Rust toolchain version, host OS/kernel and
  CPU architecture, the rootless-Podman configuration used, the
  container image identity/digest pulled, and any environment variable
  overrides from the committed `.rust-tooling-package` defaults.
- Exact command sequence run, pass/fail per target
  (`x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl`, and
  Windows if attempted), and produced-artifact checks (e.g. checksums)
  confirming the build output matches expectation.
- Resource and time observations: wall-clock build time and peak
  disk/RAM usage per target, with the measurement method and its known
  limits stated (this reproduction environment's numbers are indicative
  for future host sizing, not a controlled benchmark).
- Explicit comparison against the Option D baseline evidence already on
  record (`6tj3`, `es1p`), and any deviation from it.
- Full failure logs for any target that fails, not merely a fail
  marker.

This record is the acceptance evidence Phase C and D plans will cite —
it does not itself authorize either.

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

## Requirements-to-phase matrix (added per coder-tester exact-head review `rbkh`, finding B-I109-2)

Commissioning handoff `6msk` requires measurable acceptance and stop
conditions for seven named checks, plus issue #109's own requirement
that hosted GitLab evidence remain available for independent
corroboration. Every one is allocated to an owning phase below so none
can silently disappear from a future phase's own Gate 1 plan; each
future phase plan must define the actual measurable entry/exit evidence
for the checks it owns — this table only allocates ownership, it does
not itself supply that evidence.

| Commissioned check (source: `6msk`, issue #109) | Owning phase | Entry/exit evidence (defined by that phase's own future Gate 1 plan) |
|---|---|---|
| Clean-VM Podman cross-build equivalence | B (this plan) | Phase B's feasibility record, per the extended evidence contract above |
| Power/storage economics | A | Phase A's Gate 1 plan must name the host's power draw, storage capacity, and ongoing cost before purchase |
| VM isolation | C | Phase C's Gate 1 plan must define the trust-class VM isolation boundary and its verification |
| Cache and token containment | C | Phase C's Gate 1 plan must define how build caches and registration tokens are scoped and rotated per trust class |
| Backup/rebuild | C | Phase C's Gate 1 plan must define VM snapshot/rebuild procedure and its test |
| (monitoring and disk cleanup, implied by C's operational scope) | C | Phase C's Gate 1 plan must define ongoing host/VM monitoring and disk-cleanup procedure |
| Runner tags/protected-ref routing | D | Phase D's Gate 1 plan must define per-job tag/protected-ref routing before any job migrates |
| Hosted-minute savings | D | Phase D's Gate 1 plan must define how savings are measured per migrated job |
| Retained hosted-GitLab corroboration for release/security evidence | D | Phase D's Gate 1 plan must confirm hosted runners stay available for independent corroboration and rollback per migrated job, per issue #109's own requirement |

## Acceptance matrix

| Phase | Reviewable question | Fails when |
|---|---|---|
| B (this plan's scope) | Does the feasibility record name exact tool versions, exact commands, and a pass/fail per target, reproducible from the record alone? | Any target's result is asserted without the record, or the record silently reuses Option D's original evidence instead of a fresh reproduction |
| Phase boundary (all) | Does any phase's execution touch a later phase's authorized scope (e.g., Phase B touching CI config, Phase C purchasing hardware)? | Yes — this plan is violated even if the touched action is individually reasonable |
| Rollback (all) | Is the stated reversal for this phase actually sufficient to return to the pre-phase state with no residual CI, secret, or infrastructure exposure? | The rollback column above is wrong or incomplete for a phase once that phase is actually planned in detail |
| Requirements allocation (all) | Does every check in the requirements-to-phase matrix above still have an owning phase in that phase's own future Gate 1 plan? | A future phase plan omits a check the matrix allocated to it, without an explicit, reasoned reallocation |

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

## Revision history

**Round 2** (this correction), addressing coder-tester exact-head
review `20260826T053324Z-coder-tester-rbkh` at prior exact head
`f49a953e`, five blocking findings, all resolved in this one
consolidated revision:

- `B-I109-1`: replaced the contradictory total order (A->B->C->D) with
  the actual dependency graph — A and B are independent lanes; C
  requires A; D requires accepted B and C evidence.
- `B-I109-2`: added the requirements-to-phase matrix allocating every
  commissioned acceptance/stop condition from `6msk` and issue #109 to
  an owning phase.
- `B-I109-3`: bounded Phase B's execution environment and cost to an
  already-available, no-additional-cost, non-production, disposable
  environment under existing authority; named the specific triggers
  (new account, paid resource, protected variable, private credential,
  externally retained artifact) that stop for separate authority.
- `B-I109-4`: extended the Phase B evidence contract with pinned
  reproducibility inputs, resource/time observations, an explicit
  Option D comparison, and a chosen durable evidence carrier
  (`docs/plans/`, not a coordination completion).
- `B-I109-5`: corrected the false round-1 claim that a contribution-
  auditor/infosec handoff had already been filed; a genuine
  contribution-auditor request is filed alongside this correction, and
  a binding governor disposition is requested for infosec-sentinel
  (whose only current player is CI, incapable of the human-judgment
  threat-model review this plan needs).
