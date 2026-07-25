---
kind: handoff
from: governor-reviewer
to: build-engineer
id: 20260723T003049Z-governor-reviewer-7e44
timestamp: 2026-07-23T00:30:49Z
model: human-jani
immutable: true
references-to:
  - 20260722T154353Z-governor-reviewer-r8tg.md
  - 20260722T154432Z-governor-reviewer-1611.md
  - 20260723T002811Z-governor-reviewer-5stq.md
topic: release-build-system-idle-time-assessment
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: build-engineer
  status: ready
  task:
    - Perform a read-only adversarial assessment of the current build systems and v2.4.0 release process during genuine release waits.
    - Classify findings as current-release blockers, post-release improvements, or accepted/intentional behavior, with exact evidence.
    - Return a validated coordination review to governor-reviewer and contribution-auditor; propose Gate 1 work only where evidence warrants it.
  must_read:
    - agf/agents/MANIFEST.yaml
    - agf/agents/build-engineer.yaml
    - docs/plans/v2.4.0-release.md
    - .gitlab-ci.yml
    - CHANGELOG.md
    - docs/reference/release-process.md
    - agf/learnings/foundry.yaml
  do_not_touch:
    - .gitlab-ci.yml
    - agf/VERSION
    - CHANGELOG.md
    - release tags or GitLab releases
    - deployment targets or rollback state
    - product or documentation files
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Build-engineer assignment: assess the build and release system during idle time

The governor directs the active build-engineer role, assigned to Claude, to
assess the project's build systems and release process while the release chain
is genuinely waiting on CI, a governor action, or another assistant's owned
work. Release execution remains the priority: stop this assessment immediately
whenever MR !429, Workstream D Class 5, or another v2.4.0 blocker becomes
actionable.

## Scope

This is a read-only assessment. Review at least:

1. **Pipeline topology and cost:** push/MR/default-branch/merge/tag/schedule/API
   event behavior; duplicate graphs; skipped coordination behavior; job
   relevance; retry evidence; and whether the exact-SHA handoff model exposes
   every material pipeline.
2. **Build reproducibility:** whether documented release commands produce the
   same archives, binaries, documentation, checksums, provenance, and metadata
   from the same source; identify ambient timestamps, host paths, network
   dependencies, or mutable inputs.
3. **Release-truth consistency:** agreement among `agf/VERSION`, changelog,
   README/site claims, generated artifacts, archive contents, container/MCP/CLI
   surfaces, checksums, and public GitLab release metadata.
4. **Operator path:** whether a clean operator can follow the documented Rust
   `agov` release path without legacy Elixir fallback, hidden machine state,
   local-only files, or undocumented credentials.
5. **Failure and recovery:** what happens when a release build, artifact upload,
   tag, documentation publish, or post-merge pipeline fails; distinguish
   recoverable retries from actions that risk split release truth.
6. **Authority boundaries:** build-engineer may assess build and packaging
   evidence but may not approve Gate 1/Gate 2, authorize release/tag/deployment,
   or silently convert advisory findings into blockers.

## Cost and execution constraints

- Do not trigger a pipeline, retry a job, push an implementation commit, create
  a tag, or publish an artifact for this assessment.
- Reuse existing pipeline/job evidence and local deterministic inspection.
- Do not run a costly full release build merely to prove it can run. Start with
  manifests, scripts, CI definitions, existing logs, and focused local checks;
  identify any full clean-build experiment as separately authorized work.
- Batch reads and record exact paths, commands, commit SHAs, pipeline IDs, and
  observed limitations. Do not estimate monetary savings without billing data.

## Required output

Return one validated `kind: review` coordination record with:

- a plain-language executive verdict;
- findings ordered by release impact and evidence strength;
- explicit `current-release blocker | post-release | accepted behavior`
  classification for every finding;
- capability maturity and evidence-status labels where claims could otherwise
  overstate what exists;
- the smallest justified next action for each unresolved finding; and
- a statement confirming that no paid CI, release mutation, or implementation
  change was performed.

Address the review to `governor-reviewer` and bridge it on MR !403. Route any
proposed plan to `planner-architect`; route any requested independent challenge
to `contribution-auditor`. Do not let this secondary assessment delay the
release.
