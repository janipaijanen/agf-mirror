---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - agf/agents/ (other than the new role definition already added)
  - agf/docs/site/ pages outside those this MR names
  - broader Pages-rework work
  must_read:
  - docs/plans/20260808T181423Z-independent-implementation-reviewer.md
  - 20260808T194048Z-governor-reviewer-pspp.md
  - MR !1265 (exact head cc2e22a80f55e41bee2d94d7771e1e53174396cd)
  next_agent: contribution-auditor
  status: ready
  task:
  - Independently review MR !1265's exact head against the plan's own
    acceptance criteria (11 items) and required validation list.
  - Verify the role definition matches the plan's role contract (purpose,
    independence minimum, jurisdiction, review-output contract,
    forbidden_actions, escalation_triggers) rather than paraphrasing it.
  - Confirm no Rust, schema, CI, release, version, or MR !1224/!1252
    implementation change appears in the diff, per the plan's explicit
    exclusions.
  - Return COMPLETE/INCOMPLETE coverage and CLEAR/BLOCK for a fresh
    binding governor Gate 2 decision.
id: 20260809T061547Z-coder-tester-qpp4
immutable: true
kind: handoff
model: claude-sonnet-4-6
references-to:
- 20260808T194048Z-governor-reviewer-pspp.md
- 20260808T193155Z-governor-reviewer-get2.md
requires-ack: false
timestamp: 2026-08-09T06:15:47Z
to: contribution-auditor
topic: independent-implementation-reviewer-implementation-review
---

# Implementation ready for review: independent-implementation-reviewer role

Under `pspp`'s handoff, implemented the Gate-1-approved plan
(`docs/plans/20260808T181423Z-independent-implementation-reviewer.md`) in
a fresh worktree/branch from current `main`. MR !1265, exact head
`cc2e22a80f55e41bee2d94d7771e1e53174396cd`.

## Scope

- New `agf/agents/independent-implementation-reviewer.yaml`.
- Activated in `agf/agents/MANIFEST.yaml`; neutral skipped entry in
  `MANIFEST.example.yaml`.
- Rostered in `AGENTS.md` and `agf/docs/reference/quick-reference.md`;
  explained in `agf/docs/site/docs/concepts/roles-and-gates.md` (not as a
  fourth binding gate); added as a fourth, explicitly-optional
  commissioning shape in `agf/docs/site/docs/adopt/install-and-layouts.md`
  and `agf/ADOPTING.md`.
- Six-surface rule-projection manifest at
  `docs/reviews/rule-projection/20260808T181423Z-independent-
  implementation-reviewer.yaml`.
- CHANGELOG entries in both `CHANGELOG.md` and `agf/CHANGELOG.md`.

## Gap fixes found and corrected in the same pass

All pre-date this commit, from the earlier `web-ux-reviewer` activation
(MR !1235): that role was missing from `AGENTS.md`, `quick-reference.md`,
and `MANIFEST.example.yaml`'s role tables, and its CHANGELOG entry landed
only in root `CHANGELOG.md`, not the canonical `agf/CHANGELOG.md` (the
two files are kept in sync by convention — confirmed via direct diff of
their Unreleased sections, not assumed). `README.md`'s "13 agent
definitions" claim was stale; `agov validate readme_stats` caught this
mechanically after my changes (actual: 15). All four fixed in this MR
since they touch the same files already in scope.

## Validation

- `agov validate agents` / `agov validate` (bare, all sub-checks): pass —
  `readme_stats` now clean.
- `mkdocs build --strict --config-file agf/docs/build/mkdocs.yml`: clean.
- `agov plan projection-check --base-ref main <manifest>`: pass.
- `agov session resume --role independent-implementation-reviewer
  --base-branch main` and `agov handoff next
  independent-implementation-reviewer`: both resolve the new role
  identity correctly with no Rust code change — confirms the plan's own
  hedged claim ("`agov` already treats manifest role keys generically;
  implementation must first verify that assumption").
- `git diff --check`: clean.

Please use a session/model distinct from this implementation (Codex, if
available) for the exact-head review, consistent with the independence
principle this very role formalizes.
