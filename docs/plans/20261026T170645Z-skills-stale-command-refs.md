---
id: 20261026T170645Z-skills-stale-command-refs
status: approved
gate1_approved_by: 20261026T174028Z-governor-reviewer-snh2
gate1_approved_at: 2026-10-26T17:40:28Z
topic: shipped-skills-stale-command-references
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Wording-only corrections to three adopter-facing Markdown files. No
      credential, personal data, data flow, tooling, schema or security boundary
      is touched, and no new command is introduced.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Non-author audit by codex-foundry (the plan author and the implementation
      commit author are claude). First pass at head 45b1447d (room seq 689): OTHER,
      one factual correction (the plan wrongly claimed `agov validate adopter-docs`
      re-checks shipped skills); applied. Second pass at exact head
      b2263f82587ec62d99262756f2aa14f0ab668e7e (room seq 694): CLEAN. The review is
      recorded in the ACS room; a durable coordination review citing the final
      metadata head is to follow before Gate 1.
affected_paths:
  - docs/plans/20261026T170645Z-skills-stale-command-refs.md
  - agf/skills/README.md
  - agf/skills/health/SKILL.md
  - agf/skills/infosec-screen/SKILL.md
depends_on:
  - 20261026T151646Z-next-release-scope-intake
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: correct stale command references in shipped `agf/skills`

## Decision requested

Gate 1 approval, under the binding disposition in
`20261026T151646Z-next-release-scope-intake`, for the docs-only correction already
implemented as MR !3098 at exact head `6fccd783116bef86a908b4e53eadf9fd32d67b14`.
That plan classifies MR !3098 as `normal Gate 1 still required`; the original
fast-track waiver in the commit message does not stand. Approval here lets Gate 2
reuse the existing independent technical review of that exact head.

## Problem

Three shipped adopter files describe commands that no longer match the Rust tooling:

1. `agf/skills/health/SKILL.md` says `agov validate` runs four checks and mentions
   an "agents strict" mode. The umbrella `run_all_validators` in
   `tooling/rust/crates/agov/src/main.rs` runs eleven checks, in this order:
   learnings, agents, adoption, adopter docs, infosec, contributions, assistant
   drift, assistant loader, framework config, README stats, version drift. No strict
   mode exists.
2. `agf/skills/infosec-screen/SKILL.md` (Step 5 table) and `agf/skills/README.md`
   still name `validate-infosec.sh`, a script that does not exist in the tree. The
   skill's own Step 1 already uses `agov validate infosec`.

An adopter following the health skill is told the aggregate covers fewer checks than
it does, and one following the screening table looks for a missing script.

## Change

Exactly the three-file diff of MR !3098 (27 insertions, 11 deletions): update the
health skill description and its numbered list to the eleven checks and remove the
"strict" wording; replace `validate-infosec.sh` with `agov validate infosec` in the
infosec-screen table and the skills README index. One historical mention in the
README origin story stays, because it is correct in the past tense.

## Evidence (checked 2026-10-26, `main` at `102bf5e9`)

- The eleven checks and their order match the numbered list the MR adds.
- `validate-infosec.sh` exists nowhere in the tree.
- The MR's three-dot diff is three files; it merges cleanly with `main`.
- Pipeline `2973391424` passed all six jobs at the exact head.
- codex-foundry's independent technical review (room seq 678) found the content
  CLEAN and the governance disposition the only blocker.

## Classification and impact

NORMAL: ordinary documentation correction, no trigger path of
`always/downstream-impact-rule.md` touched (`agf/skills/` is not listed), no rule,
schema, validator, CI, role or loader change.

```
Impact:
  Foundry:          none; wording only.
  Direct adopters:  EDITORIAL: corrected description of an existing command.
  Downstream:       none.
  Migration:        not required.
  Evidence:         MR !3098 diff; run_all_validators in tooling/rust/crates/agov/src/main.rs.
```

Adopter docs parity: updated (the changed files are the adopter-facing skills).
No normative guidance is introduced, so no rule-projection manifest applies.

## Test strategy

Re-run the three greps after merge (`validate-infosec.sh`, `strict` in the health
skill, and the eleven-check list against `run_all_validators`); `agov validate`
passes. No code changes.

## Risks and rollback

Risk: `run_all_validators` changes before release, making the list stale again.
Mitigation: the list cites the function by name, and the release-freeze sweep and the
technical review at Gate 2 compare the list directly with `run_all_validators`. No
validator checks this parity today: `agov validate adopter-docs` only inspects
release-range trigger paths, and `agf/skills/` is not one of them. A real parity check
would be a separate proposal. Rollback: revert the MR; no tag or release is involved.

## Authority

Gate 1 and Gate 2 are `governor-reviewer` decisions. The author of this plan and of
the implementation commit is claude, so under the Human Governor's rule another entity
reviews and approves it. This plan changes no authority and starts no release.
