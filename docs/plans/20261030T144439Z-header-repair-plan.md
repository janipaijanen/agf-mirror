---
id: 20261030T144439Z-header-repair-plan
status: approved
gate1_approved_by: "20261030T151222Z-governor-reviewer-ph8p"
gate1_approved_at: "2026-10-30T15:12:22Z"
topic: inherited-spdx-header-repair-two-changelogs
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
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Independent review of exact ef1f25ca requested revisions (disposition
      `20261030T145555Z-governor-reviewer-h8bm`, findings 1 to 3); it found no
      secret exposure or security issue in a changelog-only repair. The
      independent re-review of exact 211f4175 returned CLEAR, recorded in Gate 1
      decision `20261030T151222Z-governor-reviewer-ph8p`.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Independent review of exact ef1f25ca requested revisions (same
      disposition, findings 1 to 3); it found the eight acknowledged predecessor
      plans coherent and no adversarial or boundary-relaxation pattern. The
      independent re-review of exact 211f4175 returned CLEAR, recorded in Gate 1
      decision `20261030T151222Z-governor-reviewer-ph8p`.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: repair the inherited SPDX-header omission in two changelogs

Governance mode: STRICT (licensing marking on shipped files). Gate 1 approved
by `20261030T151222Z-governor-reviewer-ph8p` at exact plan head 211f4175. This
approval is not Gate 2 and not a merge. It does not authorize any release
version, any license change or any adopter license guidance. Implementation
conditions are in "Sequencing conditions" below.

Source: handoff `20261030T141210Z-governor-reviewer-67p3`, proposal
`20261030T000640Z-coder-tester-tgrk`, decision `20261030T092039Z-governor-reviewer-6r8p`.
Revised under disposition `20261030T145555Z-governor-reviewer-h8bm` (claim
`20261030T145959Z-planner-architect-hsgm`, released by `sd3j`). Approved
metadata applied under claim `20261030T153008Z-planner-architect-s1gt`.

## Intent

Two authored changelogs in the v2.13.1 tree have no `SPDX-License-Identifier`
header in their first 20 lines. Add one header comment to each so the file
states locally the license it already has under the path-based rule. Change
nothing else.

## Evidence (read 2026-10-30, `origin/main` `b224f219`)

| File | Lines | SHA-256 (first 12) | First Markdown heading today |
|---|---|---|---|
| `CHANGELOG.md` | 1120 | `aca920587fa3` | `# Changelog` |
| `agf/CHANGELOG.md` | 1790 | `dee2e9e408a7` | `# Changelog — agents-govern 2.0` |

Both hashes match the ones bound in decision `6r8p`. No generated-file
exclusion covers these files; they are authored.

Policy text:

- `LICENCE`: framework and governance content, including documentation, is
  CC-BY-SA-4.0. "When a file has an SPDX header, that header is authoritative
  for that file. When there is no file-specific header, use the path-based
  summary."
- `agf/docs/site/docs/adopt/licensing.md`: the same split. The alternative
  expression `CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial` applies only
  "where expressly granted" and "is not implied".

Header census (tracked `.md` and `.yaml` files outside coordination, plans, the
bootstrap directory and `LICENSES/`, first 20 lines): 209 dual expression, 32
plain `CC-BY-SA-4.0`, 9 `MIT`, 202 none. No `REUSE.toml` or `.reuse/` exists. No
validator in `tooling/rust` enforces headers; the check is a release-rule
checklist item.

## Governor dispositions applied (`h8bm`)

- **D1:** plain `CC-BY-SA-4.0`. It records the existing path-based
  classification. It adds no commercial alternative, no new license offer and no
  legal opinion.
- **D1b:** identifier only. No copyright line and no attribution claim.
- **D2:** `agf/templates/learning-record.yaml` is excluded. See "Excluded".

## Proposed change (after Gate 1, as a separate implementation)

Exactly two files, one leading comment each:

- `CHANGELOG.md` and `agf/CHANGELOG.md` get, as lines 1 and 2:

  ```
  <!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
  (blank line)
  ```

  followed by the unchanged original file from its first byte.

Not touched: any other path, `VERSION`, `Cargo.lock`, generated files, license
payloads (`LICENCE`, `agf/LICENSE*`, `LICENSES/`), third-party terms, CI, rules,
enforcement, changelog entries and prose.

## Excluded: `agf/templates/learning-record.yaml` (D2)

The template also lacks a header. It is excluded from this repair and stays an
**explicit unresolved checklist residual**: the all-file SPDX-header checklist
in `release-via-merge-request-rule.md` is not satisfied for it.

Reason: `agf/ADOPTING.md` tells adopters to copy it into their own,
adopter-owned learning file. A header there is copied into adopter records.
Current policy does not say what to tell adopters about a copied header, and
this plan will not invent such guidance. A future proposal needs a separately
scoped copyright and licensing policy decision with copyright-holder or legal
review. This plan offers no replace-or-remove instruction as an option.

This exclusion is not a release exception. `6r8p` stays specific to v2.13.1.
No future release inherits an acceptance from this plan.

## Acceptance criteria (all required; checks are executable)

Let `BASE` be the commit the implementation branches from and `CAND` the
candidate head. For each `F` in `CHANGELOG.md agf/CHANGELOG.md`:

1. **Paths.** `git diff --name-only BASE CAND` lists exactly those two paths.
2. **Additions only.** `git diff --numstat BASE CAND` shows `2  0  F` for each
   file: two added lines, none removed.
3. **Exact header.** `git show CAND:F | head -n 2` equals the line
   `<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->` followed by an empty line.
4. **Byte-for-byte body.** `git show CAND:F | tail -n +3 | sha256sum` equals
   `git show BASE:F | sha256sum`. Removing exactly the two added lines restores
   the base file, and its SHA-256 is the one recorded above.
5. **First heading.** The first line starting `# ` is identical in `BASE:F` and
   `CAND:F`.
6. **No other change.** Because of 1, 2 and 4, no historical prose, entry or
   other path changed.

Planning-time proof that criteria 1 to 5 can fail: the checks were run, as shell
commands, against four throwaway local candidates (not pushed). A correct
two-file header passed. A wrong expression (the dual form) failed criterion 3. A
one-line body edit failed criteria 2 and 4. A one-file header plus an extra path
failed criteria 1, 2, 3 and 4. The implementer repeats the checks at the real
candidate head and puts the transcript in the MR.

Regression checks only, not evidence for the criteria above: `agov validate
adoption`, `adopter-docs`, `infosec`, `agov dist validate` and the exact-head MR
pipeline, including the strict site build. Neither changelog is an input to
MkDocs (`docs_dir` is `agf/docs/site`), so a site build, a changed-file trigger
or a site title does not show how a changelog renders or that it is unchanged.
**This plan makes no rendered-equivalence claim.** If one is later wanted, the
claim must name an available renderer and compare its actual output for the base
and the candidate changelog; unrelated site output does not substitute.

Independent exact-head review precedes Gate 2. Adopter docs parity: not
applicable (a license-marking comment; no install, upgrade or operating
change). Published v2.13.1 tag, Release, packages and OCI image are untouched.

## Collision and dependency evidence

`agov plan collision-check --plans-only --changed-only`, run on the committed
plan, reported eight blocking overlaps with earlier plans. All eight are
`approved` and their work has landed in published releases v2.11.0 to v2.13.1.
None is an open claim on these paths: the only live claim here is
`20261030T145959Z-planner-architect-hsgm`, which owns this plan file and no
product file. All eight are in `depends_on` as sequencing acknowledgements.
Each previously edited entries or prose in the changelogs; this plan adds a
header above the first line and rewrites none of that. This acknowledgement does
not approve this plan or any predecessor. The collision set is unchanged by
removing the template path; it is rechecked at each revised head.

Before implementation the owner refetches refs, confirms no active claim on the
two paths and rebases onto any merged change to them. The next release
publication plan also edits both changelogs, so the implementation either lands
before that freeze or is re-checked against it.

## Include or exclude recommendation for the next maintenance batch

Include the two changelog repairs as candidates for the next maintenance batch
once separately approved at Gate 1 and verified against criteria 1 to 6. The
criteria are the handoff's: reproducible authored-file omissions with a proven
header-only diff.

Exclude: the learning-record template (D2, above), the other headerless files
(about 63 under `agf/`), generated files, the scalar `VERSION`, `Cargo.lock`,
any new licensing policy or adopter guidance, CI or enforcement change,
historical prose rewrite and unrelated debt. This selects no release version,
milestone or date.

## Projection assessment

`documentation-rule-projection-rule.md` applies to normative operating guidance
in a Pages page or runbook. A header comment adds no MUST, SHOULD or blocking
text and no operating step. No projection manifest is needed.

## Downstream impact

Classification: EDITORIAL for behaviour (no required, forbidden or permitted
action changes), with a license-marking effect fixed by D1 and D1b.

```
Impact:
  Foundry:          No behaviour change. Two files gain one header.
  Direct adopters:  None. The template is excluded, so nothing is copied.
  Downstream:       None for the changelogs.
  Migration:        None.
  Evidence:         6r8p, tgrk, h8bm; header census above.
```

## Risks and rollback

- **A header breaks a line-based reader.** I found none in `tooling/rust`
  scripts or crates; the regression checks look again at the exact head.
- **Scope creep** into other headerless files or the template. Control:
  criteria 1 and 2 and the exclusions.
- **Rollback:** revert the single implementation MR. No tag or artifact is
  affected.

## Sequencing conditions (from the Gate 1 decision)

- The approved plan must merge to `main` before any product work.
- Then a separate two-path implementation claim must be published for exactly
  `CHANGELOG.md` and `agf/CHANGELOG.md`. The implementer refetches, checks
  current claims and collisions, and branches from the current `main`.
- One implementation MR. It runs the six executable criteria and the regression
  checks above and puts reproducible evidence in its handoff.
- Independent exact-head implementation review and a separate binding Gate 2
  precede any implementation merge. A green pipeline alone grants no merge
  authority.
- The template and the immutable v2.13.1 artifacts stay untouched.
- A substantive change to this plan returns to review and does not inherit the
  approval.

## Gate 1 status

Approved by `20261030T151222Z-governor-reviewer-ph8p` for the two-changelog
repair, with D1, D1b and D2 as recorded. Both independent re-reviews of exact
211f4175 returned CLEAR. No release version, milestone, date, tag, package,
image or deployment is selected.
