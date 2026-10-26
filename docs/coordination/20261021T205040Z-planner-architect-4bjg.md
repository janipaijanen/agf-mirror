---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261021T205040Z-planner-architect-4bjg
timestamp: 2026-10-21T20:50:40Z
model: claude-sonnet-5
immutable: true
references-to: []
requires-ack: true
topic: remove-dind-ci-gate1
artifacts_owned:
  - docs/plans/20261021T204757Z-remove-dind-ci.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 1 approval for MR !3100 (docs/plans/20261021T204757Z-remove-dind-ci.md)"
  must_read:
    - docs/plans/20261021T204757Z-remove-dind-ci.md
  do_not_touch: []
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1 submission: remove Docker-in-Docker (DinD) from CI

MR !3100, `docs/plans/20261021T204757Z-remove-dind-ci.md` — requesting
governor-reviewer Gate 1 approval.

**Bridge record note (`AGENTS.md`'s "Conversational decisions and
bridge records"):** in the live session, Jani said "we want to remove
dind from this project" and, via Claude Code's own plan-mode
confirmation (a separate, lighter-weight UX gate than this project's
Gate 1), approved the plan content now committed at this path. That
conversational approval is **not** this project's binding Gate 1
authority — only this coordination record, and the governor-reviewer
decision that answers it, supply that. The plan file's `status` stays
`pending-gate1` until a governor-reviewer `ack` or `decision-record`
references this handoff.

`agov plan validate` passes. `agov plan collision-check --plans-only
--changed-only --fail-on-unacknowledged-collision` reports 0
unacknowledged collisions (one plan-vs-plan `affected_paths` overlap
against `docs/plans/20260914T130217Z-row6-s4-issue123-collision-check-
blocking-promotion.md`, a different-subject gate-mechanics plan,
acknowledged via `depends_on`).

STRICT classification per `simplicity-rule.md`: `agov:oci:release` and
`release:verify` are named in `high-risk-operations-rule.md`
("`release:*` and `agov:oci:release` CI jobs"), so this plan is the
required stop-and-plan step — no `.gitlab-ci.yml` change has been made.

Summary of the two-part plan (full detail in the plan file):

- **Part 1** (low risk): re-tag four jobs (`release:verify`,
  `agov:oci:mr-build-scan`, `rust:tooling:oci-pull-test`,
  `agov:oci:release`) from the dead `dind` runner pool to the healthy
  `podman` pool — they declare no DinD service today and their logic
  is untouched. This directly fixes the class of outage that stuck
  MR !3096's pipeline for 28+ minutes.
- **Part 2** (feasibility spike): test `cross build --release --target
  x86_64-pc-windows-gnu` under `CROSS_CONTAINER_ENGINE=podman`, same
  bounded method as issue #109 Phase B, which only ever tested the two
  Linux musl targets. If it passes, migrate the remaining two
  genuinely DinD-dependent jobs too and `dind` has zero consumers left
  — no Proxmox fleet (issue #109 Phase C/D) required. If it fails, the
  fallback is a separate future governor decision.

Requesting: Gate 1 approval (or specific findings) on MR !3100.
