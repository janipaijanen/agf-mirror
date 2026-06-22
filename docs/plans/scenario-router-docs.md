---
id: scenario-router-docs
status: approved
topic: public-docs
gate1_approved_by: governor-reviewer
gate1_approved_at: "2026-06-22T14:05:58Z"
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >
      Docs-only change. No authentication, authorization, secrets handling,
      schema, CI, release, or executable tooling behavior is proposed.
  - role: contribution-auditor
    status: consulted
    reason: >
      The plan includes an explicit argument against a broad prompt catalog
      because adopter-facing examples can drift into procedural canon. The
      proposed page is scoped as scenario routing and decision support, with
      prompts treated as editable examples that point back to authoritative
      AGF commands and governance rules.
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: scenario-router docs page

**Date:** 2026-06-22
**Role:** coder-tester
**Status:** GATE 1 APPROVED - 2026-06-22

---

## Intent

Add a public documentation page that answers: "This is how you run AGF in
real situations." The page should help adopters choose the correct AGF action
for common and less-common moments without turning the docs into a brittle
prompt library or a second source of truth for governance rules.

Working title: **Run AGF in Real Situations**.

## Argument against the broad request

A large "copy this prompt" catalog would be convenient in the short term, but
it is risky for AGF:

- Prompts age faster than framework rules, CLI commands, and schemas.
- Users may treat examples as binding procedure even when the authoritative
  source is `agf/rules/`, `agf/schemas/`, or `./tooling/agov`.
- Prompt recipes can accidentally teach agents to invent approvals, bypass
  Gate 1/Gate 2, or rely on session memory instead of the pushed coordination
  audit log.
- Long prompt pages invite one-off local edits that other agents cannot audit.

The better shape is a scenario router: name the situation, explain the correct
AGF move, link to the authoritative rule/tooling page, and include a small
editable prompt only when it reduces ambiguity.

## Scope

### In scope

- One new public docs page under the start or operate path.
- Navigation link from the existing getting-started or operating docs.
- Scenario entries for common workflows:
  - cold start in an AGF repo;
  - checking pending handoffs;
  - proposing a non-trivial change;
  - recording Gate 1 approval;
  - handing implementation to Gate 2;
  - writing and pushing coordination messages;
  - deciding whether to close or keep an MR open;
  - investigating CI/storage growth without deleting audit history.
- Scenario entries for less-common workflows:
  - stale pending handoff cleanup;
  - assistant drift after framework upgrade;
  - external AI review/challenge;
  - docs-only fast path versus full Gate 1;
  - local-only worktree recovery;
  - when Rust `agov` lacks a command that old docs mention.
- Prompt examples phrased as optional starting points, not canonical commands.
- Cross-links to authoritative pages and command references instead of
  restating full procedures.

### Out of scope

- New governance rules or schema changes.
- Changes to `agov` behavior.
- A comprehensive prompt cookbook.
- Version-specific release checklists.
- Any suggestion that AI agents may approve Gate 1 or Gate 2.
- Any instruction to use Elixir/Mix tooling for routine project work.

## Proposed page structure

1. **Start Here**
   Short decision list: "I just opened the repo", "I have a change idea", "I
   need review", "something looks stale", "GitLab looks expensive".

2. **Common Situations**
   Compact scenario table with columns: situation, AGF move, command or file,
   and optional starter prompt.

3. **Less-Common Situations**
   Same shape, but focused on recovery and governance edge cases.

4. **Prompt Safety Rules**
   Five short guardrails:
   - ask agents to read coordination and rules first;
   - require pushed coordination for shared state;
   - never ask an AI agent to grant approval authority;
   - require exact file/MR/issue references;
   - keep prompts editable and local-context aware.

## Affected files

| File | Change |
|---|---|
| `agf/docs/site/docs/start/run-agf-in-real-situations.md` | New scenario-router page |
| `agf/docs/site/docs/start/index.md` or equivalent landing page | Add one link if the docs tree has a local start index |
| `agf/docs/site/mkdocs.yml` or docs nav file if present | Add page to navigation if required by the current site build |

The exact navigation file should be confirmed during implementation by reading
the current docs-site structure.

## Test strategy

- `./tooling/agov plan validate docs/plans/scenario-router-docs.md`
- `mkdocs build --strict` with the repository's docs configuration, if
  MkDocs is available in the environment.
- Manual internal-link check for every new cross-link.
- Search new page for forbidden drift patterns:
  - `mix agov`
  - "AI approves"
  - "merge without Gate 2"
  - prompt examples that omit pushed coordination when creating downstream
    obligations.

## Risks

| Risk | Mitigation |
|---|---|
| Page becomes duplicate procedural canon | Use scenario routing and links to authoritative rules/tools |
| Prompt examples become stale | Keep examples short, local-context dependent, and explicitly editable |
| Agents infer approval authority | State that Gate 1 and Gate 2 remain governor-reviewer decisions |
| Docs imply Mix tooling is current | Use only Rust `./tooling/agov` commands |
| Storage cleanup guidance deletes audit artifacts | Separate CI artifacts from repo/audit history and require explicit review |

## Rollback

Revert the docs MR. No schema, tooling, release, or generated artifacts need
to be unwound.
