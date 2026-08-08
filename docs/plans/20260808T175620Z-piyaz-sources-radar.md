---
id: piyaz-sources-radar
status: approved
topic: piyaz-sources-radar
gate1_approved_by: 20260808T175620Z-governor-reviewer-m2rk
gate1_approved_at: 2026-08-08T17:56:20Z
advisory_consultation:
  - role: contribution-auditor
    status: not-applicable
    reason: >
      This is a narrow, reversible classification of a public primary source
      and a non-binding radar question. It adopts no external workflow or
      authority; independent source fidelity remains Gate 2 evidence.
  - role: infosec-sentinel
    status: not-applicable
    reason: >
      The change summarizes public documentation only and touches no secrets,
      personal records, executable integration, authentication boundary,
      schema, validator, or CI configuration.
---

# Plan: classify Piyaz and add the external-context-graph question to radar

## Decision and scope

The governor directly approved adding Piyaz as proposed: a pinned primary-
source classification in `docs/sources.md` and a bounded, review-dated question
in `docs/radar.md`. The approval is transcribed in
`20260808T175620Z-governor-reviewer-m2rk`.

This plan does not adopt Piyaz, connect its hosted MCP service, install its
plugin, create an integration issue, change AGF authority, or claim independent
validation of Piyaz's effectiveness.

## Planned changes

1. Add a new dated batch to `docs/sources.md`, pinned to Piyaz v0.4.13 commit
   `9a7a0fd363daa0a954ee05e9cc4be98c042b9fbd`.
2. Classify it as tooling, agentic workspace, and MCP server; relation:
   complement, competitor in coordination/context management, and potential
   integration candidate; action: evaluate, compare-in-docs, watch.
3. Describe only source-supported mechanics: graph-backed tasks and
   dependencies, execution/decision records, resumable activity, human-owned
   default completion, and the read-only structured review agent.
4. Preserve evidence limits: project-authored claims, no local trial, hosted
   beta, editable Postgres-backed state, optional automatic merge policy, and
   unverified reviewer independence across session/model/vendor.
5. Add one radar item for the architectural fork between AGF's canonical
   repository trail and an external live context graph. State split-brain,
   authority, reconstruction, privacy, migration, and removability questions;
   set `review-by: 2026-11`.

## Acceptance and validation

- Source links are primary and the repository link is pinned to the reviewed
  commit/version.
- The radar entry is explicitly non-binding and does not imply adoption or
  partnership.
- Piyaz tool verdicts and status transitions acquire no AGF gate authority.
- The entry distinguishes observed/documented mechanisms from effectiveness
  claims that remain self-reported and untested here.
- `git diff --check` and applicable documentation validation pass.
- Gate 2 verifies wording against the pinned repository and official docs.

Coder-tester owns the documentation edit. Governor-reviewer retains the
binding Gate 2 merge decision.
