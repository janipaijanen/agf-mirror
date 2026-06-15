---
provenance: verified-agf-v2
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Environment Interrogation Before Deployment Recommendations
#
# Framework principle: Evidence before action, no speculative instructions
# See: framework.md — Principle 2 (Verification over Assumption)

> **Provisional status.** This rule is shipped on single-adopter
> evidence (amber-relay, 2026-04-18 — agent generated four-step
> containerised-runner setup without interrogating the host; all four
> steps failed with knowable-in-advance errors). The meta-pattern
> (preconditions matter) is corroborated by a second adopter
> (silver-quill #30), but the environment-interrogation checklist
> below is not yet independently corroborated. Adopters hitting this
> rule: please file a learning record so we can tighten or generalise
> the checklist.

**Scope:** Any recommendation for commands, config files, or
deployment instructions that will run against a specific target host,
container, or runtime environment.

## Rule

Before generating deployment instructions, an agent MUST interrogate
the target environment for the assumptions embedded in the
recommendation. Produce generic instructions only when the target is
generic; produce host-specific instructions only after host-specific
facts have been gathered.

### Mandatory pre-generation checks

1. **Query installed binary paths and versions.** Especially for
   user-space package managers (`pipx`, `cargo install`, `npm -g`,
   `go install`) — their binaries often live outside the default PATH
   of `systemd` units, cron jobs, or containerised services.

2. **Detect API/token/protocol variants.** Read the token format,
   server version, or integration-specific markers before emitting
   commands whose syntax depends on the variant. Example: GitLab runner
   `glrt-` tokens (GitLab 15.10+) forbid `--tag-list`, while older
   runner registration tokens require it.

3. **Read the beneficiary config.** If the recommendation coordinates
   two systems (e.g. a CI runner and a project's `.gitlab-ci.yml`),
   read both sides' configs and cross-check the shared fields (tags,
   image names, network names, queue identifiers) *before* generating.

4. **Check filesystem constraints.** Bind mounts, read-only filesystems,
   overlay layers, and tmpfs targets all forbid operations that work on
   ordinary disk. Before recommending in-place edits (`sed -i`,
   `>> file`, `chmod` on mounted configs), verify the target is writable
   and supports the edit strategy.

<!-- CUSTOMIZE: add project-specific environment probes
     (e.g. cloud-provider metadata, Kubernetes cluster version,
     database server vendor/version, service-mesh presence) -->

## Forbidden shortcuts

- **Do not assume default PATH.** `systemd` user services, `cron` jobs,
  and `docker exec` all use reduced PATHs that exclude common
  user-install locations.
- **Do not copy-paste command templates from training data** without
  re-deriving them from the current environment. The syntax may have
  changed; the binary may have moved; the flag may have been removed.
- **Do not generate deployment instructions from documentation alone.**
  Documentation lags reality. The running system is the source of truth
  for what its own tooling accepts.

## Example — before generating a systemd user service

> **Environment probes:**
> - [x] `which podman-compose` — resolves under `~/.local/bin/`
>       (pipx-installed; **not in default systemd PATH**).
> - [x] `podman-compose --version` — `1.0.6`
> - [x] `systemctl --user show-environment | grep PATH` — default
>       (no `~/.local/bin`).
>
> **Implication for the unit file:** must set
> `Environment=PATH=$HOME/.local/bin:/usr/bin:/bin` or use an absolute
> path in `ExecStart`.

## Why this exists

An agent that generates four-step setup instructions and produces four
sequential failures — each knowable before any command runs — burns
human iteration time and hides the actual complexity of the deployment
behind a false sense of progress. Interrogating the environment first
costs a handful of read-only tool calls. Not interrogating costs
minutes-to-hours of debugging, and the debugging happens in a context
where the agent's output is already partially trusted, making
divergence hard to spot.

## Related rules

- `workflow/destructive-op-preflight-rule.md` — gates recommendations
  that destroy data (different trigger: destructiveness, not
  environment assumptions)
- `always/uncertainty-and-scope-rule.md` — covers the broader principle
  of bounding what the agent claims to know
