# Operations runbook

*Status: Released. Audience: operators and AI assistants running an adopted AGF project. This page is the canonical map for choosing an operating procedure; linked pages contain the detailed contracts.*

Use this map when work crosses a session boundary or the next AGF action is
unclear. GitLab Pages renders this Markdown, but the source page remains the
authority. Product- and assistant-specific sections are adapters: removing one
must not change the core workflow or role authority.

## Choose the situation

| Situation | Start here | Then |
|---|---|---|
| New or returning session | [Session continuity](session-continuity.md#start-or-resume) | Run the unscoped and role-specific resume steps before acting |
| Planned `/compact`, IDE update, reboot, model switch, or pause | [Graceful checkpoint](session-continuity.md#graceful-checkpoint) | Validate and publish the record before leaving |
| Crash, unexpected reboot, context loss, or another machine | [Abrupt-loss recovery](session-continuity.md#abrupt-loss-recovery) | Reconstruct from Git, coordination, and forge evidence without inventing state |
| Asked to poll or coordinate | [Poll and coordinate](poll-and-coordinate.md) | Scan all shared surfaces, act inside the current gate, and leave a visible result |
| Beginning ordinary governed work | [Day-to-day operation](day-to-day.md) | Claim, implement, validate, and hand off within the approved scope |
| Need a prompt for a real situation | [Run AGF in real situations](run-agf-in-real-situations.md) | Use the prompt as direction, never as authority |
| Reviewing, blocked, or preparing a release | [Review and release](review-and-release.md) | Preserve exact-head evidence and human gate decisions |
| Commissioning or upgrading AGF | [Install](../adopt/install-and-layouts.md) or [upgrade](../adopt/upgrade-from-v1.md) | Verify framework, tooling, loaders, and documentation have installation parity |
| Leaving AGF | [Exit and decommission](../adopt/exit-and-decommission.md) | Preserve project history while removing active integration |

## Authority stays with roles

The adopted role manifest is the local authority. A model name, IDE, slash
command, Git forge, or automation does not grant authority.

| Capability | Typical responsibility | Boundary |
|---|---|---|
| Structure work and recovery boundaries | planner-architect | Planning does not approve Gate 1 |
| Reconstruct and implement, test, prepare handoff | coder-tester | Cannot approve Gate 2 or merge |
| Check consolidation and contribution provenance | contribution-auditor | Advisory |
| Verify command and documentation claims | fact-checker | Advisory |
| Investigate possible credential exposure | infosec-sentinel | Blocks only where a named rule grants authority |
| Approve gates, merge, release, resolve authority | governor-reviewer | Binding human decision |

Read [roles and gates](../concepts/roles-and-gates.md) and the adopter-designated
local role authority for the project's actual role set and status. A release
archive ships `agf/agents/MANIFEST.example.yaml`, a neutral framework
example/reference; copy it to `agf/agents/MANIFEST.yaml` and edit it to become
the local authority — the archive never places a file at that path itself, so
nothing is silently overwritten by a plain re-extraction.

## Durable-state rule

Repository and forge evidence outrank session memory. A local file, untracked
file, local-only commit, stash, terminal transcript, or unsaved editor buffer
is not a shared handoff. When another machine, person, or agent may resume,
the record must validate and be pushed to a shared branch, published through
an approved shared path, or paired with a visible bridge naming where it lives.

`agov session resume` summarizes repository coordination. It does not fetch
Git, poll merge requests or issues, save dirty work, recover unsaved buffers,
approve gates, or reconstruct reasoning that was never externalized.

## Adapter boundaries

### GitLab adapter

In a GitLab-hosted project, “forge” means merge requests, discussions and
notes, issues and issue comments, pipelines, jobs, releases, and shared remote
refs. Use the GitLab UI, API, or `glab` to inspect those surfaces. The core
procedure remains the same on another forge.

### Claude adapter

`/compact-agf` is an optional project command for preparing a graceful
checkpoint. It is not an `agov` command and may not exist downstream. Its
output is draft state until validation **and publication** satisfy the
durable-state rule above. Other assistants follow the same neutral checkpoint
procedure directly.
