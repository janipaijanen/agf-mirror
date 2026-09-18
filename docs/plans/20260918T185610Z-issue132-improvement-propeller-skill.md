---
id: 20260918T185610Z-issue132-improvement-propeller-skill
status: pending-gate1
topic: issue132-improvement-propeller-skill-gate1-planning
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Ran `agov validate infosec`: OK, no findings. This plan proposes no
      code, no learning-record content, and no secrets-adjacent surface;
      it is a plan-only artifact. The skill file itself does not yet
      exist (implementation follows Gate 1 approval), so there is no
      skill content to screen yet — Gate 2 review re-screens the actual
      file.
  - role: contribution-auditor
    status: pending
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: export the Improvement Propeller method as a skill (issue #132)

**Depends on:** #131, closed Level 5 Deployable per governor decision
`20260918T162825Z-governor-reviewer-8hs2`. That dependency is satisfied;
this plan is filed per the handoff
`20260918T175756Z-capability-developer-nsha`.

## Intent

Add `agf/skills/improvement-propeller/SKILL.md`: a thin, human-invoked,
advisory skill that lets an operator run the Improvement Propeller's
evidence-gathering and ranking discipline against a concept, piece of
content, or document — including material outside the governed project
surface — and then feed the finished assessment back into AGF through
the normal gated contribution path (learning record / issue / proposal).

This is a distribution artifact only. It does not activate the
Improvement Propeller role for any adopter, does not create new
authority, and does not change `agf/rules/always/improvement-propeller-rule.md`,
which remains the single normative owner. The skill follows the same
"thin wrapper, rule is the contract" pattern already used by
`agf/skills/poll-and-coordinate/SKILL.md` (which points at
`universal-polling-and-autonomous-continuation-rule.md`) and
`agf/skills/capability-review/SKILL.md` (structured evidence → ranked
proposal → human-gate pattern, reused here for the ranking-and-evidence
shape).

## Affected files

| File | Change | Migration needed? |
|---|---|---|
| `agf/skills/improvement-propeller/SKILL.md` | New. Thin skill definition. | No |
| `agf/skills/README.md` | Add one row for the new skill in the "Available skills" table. | No |
| `README.md` | Update the "8 skills" count (line 164) to 9. | No |
| `docs/reviews/rule-projection/20260918T185610Z-issue132-improvement-propeller-skill.yaml` | New. Six-surface projection evidence per `documentation-rule-projection-rule.md`; gains the SKILL.md path once it exists post-Gate-1. | No |

No code, schema, CI, or distribution-manifest change. No test suite
touches skill Markdown files today (skills are agent-invoked
procedures, not executable code); this plan does not add one, consistent
with the existing eight skills, none of which carry an automated test.

## Skill content (what Gate 1 approves)

`agf/skills/improvement-propeller/SKILL.md` frontmatter:

```yaml
---
name: improvement-propeller
origin: upstream
description: >
  Human-invoked advisory assessment: apply the Improvement Propeller's
  evidence-gathering and ranking discipline to an operator-supplied
  concept, piece of content, or document, including material outside
  the governed project surface, then route the finished assessment
  into AGF through the normal gated contribution path.
---
```

Body, following the `poll-and-coordinate` precedent of pointing at its
governing rule rather than restating it:

1. **Normative owner statement** — first paragraph names
   `agf/rules/always/improvement-propeller-rule.md` as the single
   normative owner. The skill operationalizes that rule's "Weekly
   opportunity lane" and "Slow governance-friction lane" evidence/ranking
   shape into a one-shot, human-invoked procedure; it does not restate
   the rule's activation contract, cursor/recovery semantics, or
   role-boundary text, and does not itself activate the role for any
   adopter.
2. **Role disclaimer** — the phrase "observes and proposes, grants
   nothing, and makes no decision" is issue #132's own plain-language
   requirement text, not a verbatim quotation from the rule (correction
   per contribution-auditor finding F5 — no such sentence exists in
   `improvement-propeller-rule.md`). Stated before Step 1, paired with
   the rule's actual authority blockquote quoted verbatim: "Propel a
   small, deduplicated set of evidence-backed improvement proposals; do
   not become the project's priority setter, planner, reviewer, or
   implementer," plus the adjoining MUST-NOT list (assign work, reserve
   another role's branch, set roadmap/release priority, create
   implementation claims, approve gates or risk, implement its own
   proposal, merge, tag, release, deploy).
3. **Step 1 — Scope the input.** Operator supplies the concept, content,
   or document (a URL, a pasted excerpt, a local file, a described idea).
   Per the rule's untrusted-evidence clause, any recorded URL is named
   for the operator to dereference themselves; the skill does not fetch
   it.
4. **Step 2 — Gather evidence.** Scan configured project-local sources
   (radar, roadmap, open issues/MRs, coordination) for anything already
   representing this candidate. If found, the skill's output is "update
   or reference existing item X," not a new candidate — mirrors the
   rule's table-top case "An open issue already represents the
   candidate."
5. **Step 3 — Rank and bound.** At most three opportunity candidates
   and at most three governance-friction findings (the rule's own caps).
   Each candidate/finding states: evidence + provenance, evidence status
   (Empirically validated / Empirically informed / Theoretical, per
   `agf/docs/site/reference/maturity-vocabulary.md`), user/release value
   or measured cost, affected artifacts, smallest useful outcome,
   strongest counterargument, dependencies/current gate, collision/dedup
   disposition, and suggested receiving role.
6. **Step 4 — Reject weak friction complaints.** A complaint labeling a
   control "ceremony," "stupid," slow, or annoying without cost,
   protection, and counterargument is rejected — reproduces the rule's
   table-top case verbatim as a check, not a restatement of the rule's
   general contract.
7. **Step 5 — Route, never bypass.** The assessment becomes a learning
   record, issue, or proposal through the normal gated contribution
   path. The skill has no merge, tag, release, gate-approval, or
   implementation authority — restates the rule's authority boundary at
   the exit point where an operator might otherwise be tempted to skip
   it.
8. **CUSTOMIZE comment** — adopters name their own project-local
   evidence surfaces (radar file path, roadmap doc, forge), matching the
   pattern already used in `poll-and-coordinate/SKILL.md`'s Inputs
   section.

## Maturity (per `implementation-honesty-rule.md`)

The skill's own maturity, distinct from dependency #131's already-closed
Level 5 disposition:

- **At this Gate 1 plan:** Level 1 Planned. Named in this plan; no file
  exists yet.
- **After implementation, table-top verification (Test strategy step
  2), and contribution-auditor review pass:** Level 4 Validated. Schema
  n/a (skills aren't schema-governed); "validated" here means the
  worked-example table-top output matches the required shape and the
  normative-text/disclaimer-attribution checks pass.
- **Level 5 Deployable** only once the skill ships in a supported
  release artifact with `agf/skills/README.md` and `README.md`'s skill
  count both current — not merely present on `main`.

## Explicitly out of scope

- The optional capability-developer "does this warrant agent evolution?"
  triage skill named in issue #132's "Optional" section. Not built here;
  `agf/skills/capability-review/SKILL.md` already covers that analysis.
  Evaluate need after this skill ships, per the issue's own text.
- Any change to `improvement-propeller-rule.md` itself.
- Any adopter activation decision (governor decision naming player,
  sources, wake mechanism, cadence, cursor state remains required
  per the rule's Activation contract, unaffected by shipping this
  skill).
- Any distribution-manifest/packaging automation change — noted as a
  pre-existing gap below, not fixed by this plan.

## Documentation-rule-projection-rule six-surface screen

Full structured evidence: `docs/reviews/rule-projection/20260918T185610Z-issue132-improvement-propeller-skill.yaml`.
Summary:

| Surface | Disposition |
|---|---|
| Normative owner | `existing pointer sufficient` — `improvement-propeller-rule.md`, unchanged |
| Adopter-facing explanation | `not applicable` — the skill is the operational artifact; the rule's own Pages/runbook projections (if any exist today) are unaffected |
| Runbook/situation map | `updated` — one row added to `agf/skills/README.md`'s "Available skills" table |
| AI landing/loaders | `not applicable` — skills are discovered by an agent reading `agf/skills/` directly (per `agf/skills/README.md`'s own "Adoption" section), not via `AGENTS.md`/`CLAUDE.md` pointer text; no loader change needed |
| Validation/drift evidence | `advisory only` — no automated skill-content validator exists for any of the eight current skills; contribution-auditor review at Gate 2 is the check, named authority: this plan/Gate 2 review |
| Distribution | `implemented` — `agf/skills/**` is a listed glob in `agf/dist/release-manifest.yaml`; root `README.md` is not a distribution surface (not matched by any manifest glob) and its count update is ordinary repo maintenance, not a distribution-row concern |

## Pre-existing gap observed, not fixed by this plan

`agf/skills/README.md`'s "Available skills" table currently lists 4 of
the 8 skills that exist in `agf/skills/` (missing `capability-review`,
`contribution-audit`, `health`, `release`). This plan adds a correct
5th row for `improvement-propeller` but does not backfill the other
four missing rows — that is a documentation-estate-steward-shaped
finding (estate consistency), out of this issue's scope. Flagging here
so it is not lost; a separate coordination note names it for the
steward's next pass (per contribution-auditor finding A1).

## Test strategy

No automated test exists for any current skill's content (all eight are
Markdown procedure definitions, not executable code). This plan does not
introduce one — consistent with existing skills. Verification is:

1. `agov coord validate` / schema checks are not applicable (skills
   are not coordination or plan files).
2. Manual table-top: run the skill's Step 1-5 procedure against one
   worked example (a real, already-known AGF-adjacent piece of content,
   e.g. one of the sources already classified in `docs/sources.md`) and
   confirm the output matches the shape (dedup disposition correctly
   finds the existing entry rather than proposing a duplicate).
3. Independent contribution-auditor review confirms the skill does not
   restate or fork the rule's normative text (checked by diff/grep
   against the rule body) and correctly attributes the disclaimer
   language per the correction above (not claimed as a verbatim rule
   quotation).

## Risks

- **Restating the rule instead of pointing at it.** The single biggest
  failure mode for a "thin wrapper" skill — drift between the skill's
  copy and the rule's actual text. Mitigated by the explicit
  normative-owner-statement requirement above and the Gate 2 diff/grep
  check.
- **Skill used to imply role activation.** An operator could mistake
  running this skill for activating the Improvement Propeller role for
  their adopter project. Mitigated by the role disclaimer being placed
  before Step 1, and an explicit sentence that this skill does not
  activate the role (the rule's Activation contract is the only path
  to that).
- **Scope creep into the optional triage skill.** Mitigated by the
  explicit "out of scope" section above; a separate governed change is
  required to add that.

## Rollback

Delete `agf/skills/improvement-propeller/SKILL.md` and revert the
`agf/skills/README.md` row. No schema, data, or artifact migration; no
adopter has activated anything based on this file's mere existence.

## Role and gate boundary

planner-architect (this session) files this plan per the `nsha` handoff.
coder-tester implements after Gate 1 approval. Gate 1 and Gate 2 approval
remain governor-reviewer's sole authority. No release, tag, or deploy
authority is requested or implied.
