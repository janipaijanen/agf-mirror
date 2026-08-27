---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260827T123825Z-coder-tester-n6ar
timestamp: 2026-08-27T12:38:25Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260813T044747Z-governor-reviewer-8hgm.md
  - 20260825T162538Z-coder-tester-6md8.md
  - 20260825T165302Z-planner-architect-ppan.md
  - 20260827T080728Z-coder-tester-rdsa.md
  - 20260827T101500Z-governor-reviewer-g2m8.md
topic: v270-release-objective
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - "Prepare the separate Gate 1 v2.7.0 release-publication plan now that both frozen-scope implementations are merged."
    - "Critically compare frozen-scope reconstruction from v2.6.1 with an explicit cumulative-main scope amendment; recommend one and identify the exact governor decision prerequisites."
    - "Recommend whether publication must use v2.7.0-rc.1 plus a verified hold before stable, or may proceed directly to v2.7.0, with the governing basis and cost/risk trade-off."
    - "Give issues #86, #87, #88, and #90 plus the deferred Claude /context live check explicit terminal or pre-release dispositions in the work order."
  must_read:
    - agf/skills/release/SKILL.md
    - agf/rules/workflow/release-via-merge-request-rule.md
    - agf/rules/workflow/immutable-tags-rule.md
    - docs/coordination/20260813T044747Z-governor-reviewer-8hgm.md
    - docs/coordination/20260825T162538Z-coder-tester-6md8.md
    - docs/coordination/20260825T165302Z-planner-architect-ppan.md
    - docs/plans/agov-mcp-transport-protocol-correctness.md
    - docs/plans/assistant-discovery-archive-setup.md
  do_not_touch:
    - .claude/settings.local.json
    - Git tags, GitLab Releases, packages, OCI namespaces, or release branches before their governed authorization
---

# Objective: publish AGF-NG v2.7.0 with a terminal release work order

The governor has set the next objective: **release version 2.7.0**, working
with Claude. This handoff asks planner-architect to prepare the missing
release-publication plan and decision package; it does not itself approve a
topology, Gate 1, a version bump, a candidate, a release branch, a tag, or
publication.

## Current evidence

- Binding decision `8hgm` freezes v2.7.0 to issues #86, #87, #88, and #90.
- MR !806 implemented #86/#88 and merged at `960124e1` after exact-head Gate 2.
- MR !2138 implemented #87/#90 and merged at `acaec947` after exact-head Gate 2.
- Current `main` is `efd811dd`; pipeline `2796349725` succeeded.
- Issues #87 and #90 are closed. Issues #86 and #88 remain open despite their
  implementation merge and therefore lack terminal tracker dispositions.
- The required Claude loader live check is correctly recorded as post-merge
  monitoring, but a fresh Claude Code session has not yet confirmed that
  `.claude/CLAUDE.md` and imported `AGENTS.md` content appear under `/context`
  Memory files.
- No v2.7.0 release-publication plan exists, and the v2.7.0 tag, release,
  package, and OCI publication have not been authorized.

## Required planning boundary

Do not present current cumulative `main` as compatible with `8hgm` without an
explicit scope-model amendment. Conversely, do not select frozen-scope
reconstruction until its rule prerequisites identify the exact stable base,
ordered allowlist, frozen-out set, namespace checks, and reason the standard
and maintenance paths do not apply.

The work order must distinguish substantive implementation from terminal
shipment disposition. It should not call the scope complete while #86/#88 or
the deferred live check lack an explicit close, waiver, or accepted-monitoring
outcome. It must also separate release-candidate acceptance from stable
publication and name every human decision point.

Please respond through coordination with a critical recommendation and the
smallest Gate 1-ready plan path. No release-side mutation is authorized by
this handoff.
