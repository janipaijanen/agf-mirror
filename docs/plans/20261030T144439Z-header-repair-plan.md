---
id: 20261030T144439Z-header-repair-plan
status: pending-gate1
topic: inherited-spdx-header-repair-three-files
depends_on:
  - 20261021T220000Z-retire-windows-support
  - 20261025T183939Z-v211-changelog-and-scope-reconciliation
  - 20261025T192020Z-supersedes-adopter-docs
  - 20261027T230355Z-v212-release-freeze-corrections
  - 20261027T235807Z-v212-release-publication
  - 20261029T204751Z-v213-release-freeze-preparation
  - 20261029T220500Z-v213-release-publication
  - 20261030T064744Z-v2131-release-publication
affected_paths:
  - CHANGELOG.md
  - agf/CHANGELOG.md
  - agf/templates/learning-record.yaml
advisory_consultation:
  - role: infosec-sentinel
    status: pending
    reason: >-
      Licensing-marking change on shipped files, including a template that
      adopters copy. Consultation requested with this plan; no clearance is
      claimed.
  - role: contribution-auditor
    status: pending
    reason: >-
      Independent review requested with this plan, in particular of decisions
      D1 and D2 and of the include or exclude recommendation. No clearance is
      claimed.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: repair three inherited SPDX-header omissions

Governance mode: STRICT (legal and licensing material on shipped files; a
`agf/templates/` trigger path). Planning only. This plan does not authorize
editing any target file, any release version, or any license change.

Source: governor handoff `20261030T141210Z-governor-reviewer-67p3`, proposal
`20261030T000640Z-coder-tester-tgrk`, decision `20261030T092039Z-governor-reviewer-6r8p`.
Claim: `20261030T144438Z-planner-architect-tgjg`.

## Intent

Three authored files in the v2.13.1 tree have no `SPDX-License-Identifier`
header in their first 20 lines. The all-file header checklist in
`release-via-merge-request-rule.md` therefore is not fully satisfied. Add one
header comment to each file so the file states locally the license it already
has under the path-based rule. Change nothing else.

## Evidence (read 2026-10-30, `origin/main` `b224f219`)

| File | Lines | SHA-256 (first 12) | First line today |
|---|---|---|---|
| `CHANGELOG.md` | 1120 | `aca920587fa3` | `# Changelog` |
| `agf/CHANGELOG.md` | 1790 | `dee2e9e408a7` | `# Changelog — agents-govern 2.0` |
| `agf/templates/learning-record.yaml` | 78 | `d8e4995e9b03` | `project:` |

These hashes match the ones bound in decision `6r8p`. No generated-file
exclusion covers them. They are authored files.

Policy text:

- `LICENCE`: framework and governance content, including documentation,
  examples, plans and coordination records, is CC-BY-SA-4.0. "When a file has an
  SPDX header, that header is authoritative for that file. When there is no
  file-specific header, use the path-based summary."
- `agf/docs/site/docs/adopt/licensing.md`: the same split; the alternative
  expression `CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial` is carried by
  "some framework content". Commercial terms apply "where expressly granted"
  and "are not implied".

Census of tracked `.md` and `.yaml` files outside coordination, plans, the
bootstrap directory and `LICENSES/` (first 20 lines): 209 carry
`CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial`, 32 carry plain
`CC-BY-SA-4.0`, 9 carry `MIT`, and 202 carry no identifier. Under `agf/` only:
78, 31, 8 and 63. No `REUSE.toml` or `.reuse/` exists. No validator in
`tooling/rust` enforces headers; the check is a release-rule checklist item.

## Ambiguity needing governor disposition

The policy does not say which expression an authored changelog or template
should carry. Two readings fit it. The plan does not choose by itself.

**D1: which expression.**

- **A. `CC-BY-SA-4.0`.** Equal to what these files already have under the
  path-based rule, so the effective terms do not change. Matches `tgrk`, the
  proposal, and 32 existing headers, including `docs/roadmap.md` and
  `agf/ADOPTING.md`.
- **B. `CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial`.** Matches the
  majority of sibling files (209 of 252 headered, 78 of 109 under `agf/`). It
  would also mark these files as available under the commercial alternative.
  `licensing.md` says that alternative applies only where expressly granted.
  Adding it is therefore a licensing offer, not a repair of an omission.

Recommendation: **A**. It records the existing terms and creates no offer. B is
the copyright holder's decision, and the plan must not make it indirectly.
Sub-question D1b: identifier only (A or B), or also the copyright line
`Copyright (c) 2026 Jani Päijänen` used by most siblings. Recommendation:
identifier only, because `tgrk` and the handoff ask for the identifier.

**D2: the YAML template.** `agf/ADOPTING.md` (steps at lines 126 and 1285) tells
adopters to copy `agf/templates/learning-record.yaml` to
`agf/learnings/<codename>.yaml`. Adopter learning records are adopter-owned
(`governance-file-layout-rule.md`). A header in the template would be copied
into every adopter's own record and would label it CC-BY-SA-4.0 until they
delete it. Options: (i) add the header with a short comment that adopters
should replace or remove it in their copy; (ii) exclude the template from this
repair and leave it as a documented, accepted omission. Recommendation: **(i)**,
because the omission is real and the comment removes the downstream surprise,
but the governor decides. If (i) is chosen, the exact comment text is part of
the Gate 1 approval.

Neither D1 nor D2 is resolved by a license conversion, a fresh exception or a
broad rewrite.

## Proposed change (after Gate 1, as a separate implementation)

Exactly three files, header comment only, at the top:

- `CHANGELOG.md` and `agf/CHANGELOG.md`: one HTML comment line,
  `<!-- SPDX-License-Identifier: <D1 expression> -->`, followed by one blank
  line, before the existing first line.
- `agf/templates/learning-record.yaml`: one `#` comment line with the same
  identifier (plus the D2 note if option (i)), before `project:`.

Not touched: `VERSION`, `Cargo.lock`, generated files, license payloads
(`LICENCE`, `agf/LICENSE*`, `LICENSES/`), third-party terms, CI, rules,
enforcement, historical prose.

## Acceptance criteria (all required)

1. `git diff` against the base shows only added lines, only in the first lines
   of the three files, and no other path.
2. The Markdown body after the header is byte-identical: for each changelog,
   the file minus the added header lines has the same SHA-256 as before.
3. YAML data is identical: the parsed content of the template before and after
   is equal (a recorded check with the project's YAML parser), and
   `agov validate` accepts the file as before.
4. Rendered changelog history is unchanged: the site build output for the two
   changelogs differs only by the absent HTML comment. A leading HTML comment
   must not change the page title or heading extraction; the implementer shows
   the strict site check output.
5. No existing check regresses: `agov validate adoption`, `adopter-docs`,
   `infosec`, `agov dist validate`, and the exact-head MR pipeline.
6. Independent exact-head review precedes Gate 2. Adopter docs parity: not
   applicable for the changelogs; for the template see D2.
7. Published v2.13.1 tag, Release, packages and OCI image are untouched.

## Collision and dependency evidence

`agov plan collision-check --plans-only --changed-only` (run on the committed
plan) reported eight blocking overlaps with earlier plans. All eight are
`approved` and their work has landed in the published releases v2.11.0 to
v2.13.1 (changelog edits, release metadata, freeze corrections, and the
release-publication plans). None of them is an open claim on these paths: the
only live claim in this area is `20261030T144438Z-planner-architect-tgjg`, which
owns this plan file and no product file. All eight are added to `depends_on` as
sequencing acknowledgements. Each previously edited prose or entries inside the
changelogs; this plan adds a header above the first line and rewrites none of
that content. This acknowledgement does not approve this plan or any
predecessor.

Before implementation the owner must refetch refs, confirm no active claim on
the three paths, and rebase onto any merged change to them. The next release
publication plan will also edit both changelogs, so the implementation either
lands before that freeze or is re-checked against it.

## Include or exclude recommendation for the next maintenance batch

Criteria, from the handoff: include the three reproducible authored-file
omissions with a proven header-only diff. Exclude generated files, the scalar
`VERSION`, `Cargo.lock`, new licensing policy, CI or enforcement changes,
historical prose rewrites and unrelated debt.

- **Include** the two changelog headers once D1 is decided and criteria 1, 2, 4
  and 5 pass. They are reproducible, authored, and header-only.
- **Include** the template header only with the D2 decision recorded; otherwise
  **exclude** it and keep it as a documented residual.
- **Exclude** the other 63 headerless files under `agf/` and the rest of the
  repository. They are unrelated old debt. A repository-wide pass or a new
  header check would be new licensing policy or enforcement.
- This plan chooses no release version, milestone or date.

## Projection assessment

`documentation-rule-projection-rule.md` applies to normative operating guidance
in a Pages page or runbook. A header comment adds no MUST, SHOULD or blocking
text and no operating step. No projection manifest is needed. The template
comment in option (i) is an instruction to adopters about their own copy; if the
governor chooses it, the implementation re-checks this assessment.

## Downstream impact

Classification: EDITORIAL for behaviour (no required, forbidden or permitted
action changes), with a licensing-marking effect that D1 and D2 control.

```
Impact:
  Foundry:          No behaviour change. The three files gain one header.
  Direct adopters:  Template header is copied into adopter learning records (D2).
  Downstream:       None for changelogs. Rendered pages unchanged.
  Migration:        None.
  Evidence:         6r8p, tgrk; census above.
```

## Risks and rollback

- **Choosing B silently** would extend a commercial offer. Control: D1 needs an
  explicit governor disposition.
- **A header breaks a line-based reader.** I found none in `tooling/rust`
  scripts or crates; criteria 4 and 5 check it again at exact head.
- **Scope creep** into other headerless files. Control: criterion 1 and the
  exclusions.
- **Rollback:** revert the single implementation MR. No tag or artifact is
  affected.

## Gate 1 request

Approve this plan with explicit dispositions for D1 (expression), D1b (copyright
line) and D2 (template). Implementation, if approved, is one MR.
