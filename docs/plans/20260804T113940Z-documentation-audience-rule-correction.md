---
id: documentation-audience-rule-correction
status: pending-gate1
topic: documentation-audience-rule-correction
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Round 1 review (`20260804T120122Z-contribution-auditor-qhqg.md`):
      BLOCK, 6 blocking findings (B-DA-1..6) plus 1 advisory
      (A-DA-1). All six addressed in this revision -- see "Round history"
      below. Also implements Path 1 of the earlier independent second
      opinion `20260803T225851Z-contribution-auditor-pjep.md` (finding
      1: "the audience mismatch is real"). Requesting round-2 review.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Documentation-rule wording change only; no credential, auth,
      network-trust-boundary, or secret handling.
---

# Plan: correct `documentation-audience-rule.md` and `marketing-tone-rule.md`'s stale audience contract

## Intent

`.claude/rules/docs/documentation-audience-rule.md` and its distributed
copy `agf/rules/docs/documentation-audience-rule.md` describe public Pages
content as targeting a **generic business application**: "directors,
managers, employees" and "system administrators," focused on "deployment,
QA verification, operations, data entry, configuration, backup."
`.claude/rules/docs/marketing-tone-rule.md` and its distributed copy carry
the same model into tone guidance: a "Tone by audience" table naming
"Directors, managers" / "Employees" / "Business-IT, admins" as the
readership, and message-integrity examples about planning/tracking/
reporting, imports/exports, migrations, audit logs, backups.

**Root cause, corrected from this plan's own round-1 draft:** this is not
a case of the rule needing invented audience categories. `agf/docs/site/
docs/index.md` already states, live, "Every page below falls into one of
four kinds" — **Human decision, AI action, Shared concept, Reference** —
implementing an **already Gate 1-approved plan**,
`docs/plans/docs-audience-boundary-proposal.md` (approved 2026-07-20,
`docs/coordination/20260720T143255Z-governor-reviewer-s6hg.md`), which
itself defined this exact taxonomy plus a per-page `Audience:` line
convention. Both rules simply were never updated after that approved
implementation shipped — they still describe the pre-2026-07-20 model.
This plan corrects rule text to match already-approved, already-shipped
site structure; it does not design a new taxonomy.

This is the blocking prerequisite MR !1082 itself names ("Blocking
prerequisite... `documentation-audience-rule.md`'s target-audience
framing conflicts with AGF's actual technical audience — flagged, not
resolved") and that `pjep`'s independent review recommended fixing (Path
1) before any further Pages/README visual-identity work proceeds.

**Out of scope for !1082 itself**, `v2.5.1`, and governor hold `5m41`
(still binding for those — see
`docs/plans/20260804T113940Z-v260-coordination-reliability.md`'s
"Explicitly excluded" section for the same boundary reasoning applied to
that plan).

## Downstream-impact classification

Per `always/downstream-impact-rule.md`: **BREAKING**, not EDITORIAL — the
set of actions both rules require changes, and both are trigger-path
artifacts (`.claude/rules/**/*.md`, `agf/rules/**/*.md`).

### Three-circle impact statement

```
Impact:
  Foundry:          Yes, but corrective, not novel. Foundry's own Pages
                     site already follows the four-kind taxonomy
                     (implemented 2026-07-20) and does not follow the
                     rules' current generic-business-audience text. This
                     change brings both rules into agreement with
                     already-approved, already-shipped Foundry content.
  Direct adopters:  BREAKING. An adopter who copied the prior rule text
                     verbatim and wrote Pages/tone content for a generic
                     business audience (directors/managers/employees,
                     data-entry/backup framing) will find the corrected
                     rules ask for a structural taxonomy (which kind of
                     page: human-decision, AI-executed, shared-concept,
                     or reference) and an explicitly project-defined
                     primary/secondary audience, instead of a fixed
                     business-persona list.
  Downstream:       Adopter-produced Pages content shape changes going
                     forward for new/substantively-revised pages only
                     (see "Temporal contract" below -- not a retrofit
                     mandate). No schema, CLI, or MCP behavior changes.
  Migration:        See below.
  Evidence:         qhqg (20260804T120122Z-contribution-auditor-qhqg.md),
                     pjep (20260803T225851Z-contribution-auditor-pjep.md),
                     confirming and extending b27h
                     (20260803T225314Z-planner-architect-b27h.md); grounded
                     in the already-approved
                     docs-audience-boundary-proposal.md
                     (s6hg, 2026-07-20).
```

## Migration note (required — BREAKING)

1. No immediate action required for already-published pages — per
   `documentation-rule-projection-rule.md`'s baseline-audit boundary,
   this does not retroactively invalidate historical content. It also
   does not claim Foundry's own pages are already fully compliant (see
   `B-DA-3` correction below): as of this plan's own drafting,
   `docs-audience-boundary-proposal.md`'s own text records only 22 of
   34 nav pages carrying an explicit `Audience:` line — the retrofit was
   already explicitly partial and tracked, not asserted complete.
2. **New or substantively revised pages**, from this rule's adoption
   forward, must: (a) declare which of the four kinds it is (or the
   adopter's own equivalent taxonomy, per the CUSTOMIZE boundary below),
   and (b) state its primary/secondary audience and, for AI-action pages,
   the human verification/troubleshooting path.
   **"Substantively revised"** means: any commit that changes the page's
   procedural content, its stated audience, or its execution-owner
   framing — not a typo fix, link correction, or formatting-only change.
3. Adopters whose actual Pages audience genuinely is a generic business
   application (a real possibility for a fork or derivative, not
   Foundry's own case) keep the four-kind *structural* taxonomy — it is
   a reusable documentation-engineering pattern, not AGF-specific — and
   replace only the *persona names* within it, per the rule's own
   `## CUSTOMIZE` block (new, see below).

## Corrected rule content

### `documentation-audience-rule.md` (both copies)

Distributed copy (`agf/rules/docs/`) states the **generic, reusable
invariant**:

- **README.md** (repo root): unchanged — targets **developers**,
  development experience.
- **Public documentation site** (content under `docs/`): organize pages
  into a small number of kinds by **who is meant to act on them** — at
  minimum, distinguish pages that ask a human to decide something from
  pages that are majority procedure for an AI agent to execute under
  human supervision, from pages stating a shared mental model, from
  reference/lookup pages. Every page states which kind it is and its
  primary/secondary audience. AI-action pages state the human
  verification/troubleshooting path.
- `## CUSTOMIZE`: adopters name their own kind labels and primary/
  secondary audience personas here; the structural requirement (declare
  kind + audience + verification path) is invariant, the labels are not.

Foundry's project copy (`.claude/rules/docs/`) **specializes** the
CUSTOMIZE block with AGF's actual, already-shipped taxonomy: **Human
decision, AI action, Shared concept, Reference** (verbatim from
`docs/index.md`), primary audiences **technical adopters/operators and
human governors**, secondary audiences **engineering leaders and risk/
security/compliance/assurance stakeholders on decision pages**. Points to
`docs-audience-boundary-proposal.md` as the record of how this taxonomy
was decided, rather than restating its reasoning.

### `marketing-tone-rule.md` (both copies) — resolves `B-DA-1`

The distributed copy's "Tone by audience" table and message-integrity
examples currently hard-code a business-application persona list and
capability set (planning/tracking/reporting, imports/exports,
migrations, audit logs, backups) as if every adopter's product were that
shape. Generalize: replace the fixed persona rows with a pointer to
whatever taxonomy `documentation-audience-rule.md` establishes for the
project (the same CUSTOMIZE boundary, not a second, independently
diverging one), and state the tone principles (empathy, vendor respect,
positive framing, message integrity) in persona-neutral terms. Foundry's
project copy specializes the table using AGF's own four kinds and actual
audiences, matching the audience-rule specialization above exactly —
never diverging, since both copies of both rules change in the same
commit.

## Temporal contract — resolves `B-DA-3`

**Chosen: prospective, not universal.** The corrected rules apply to new
and substantively-revised pages only, per the migration note's definition
above. This plan does **not** mandate a retrofit of the site's existing
26 `agf/docs/site/docs/**/*.md` pages (corrected count — the plan's own
round-1 draft wrongly asserted "approximately 37," which came from
`pjep`'s different, broader count basis that included non-`docs/`
sections; `find agf/docs/site/docs -name '*.md' | wc -l` is exhaustive and
reproducible). It also does not claim Foundry's pages already fully
comply — `docs-audience-boundary-proposal.md`'s own 22-of-34 figure
already establishes partial, tracked compliance; this plan does not
overstate it as complete. A full retrofit, if wanted, is a separate,
governor-approved staged migration with its own tracked deadline — not
bundled here (bundling would fail the branch sizing test: "does the rule
now say the right thing" is a different reviewable question from "does
every page comply with it").

## README treatment — resolves `B-DA-5`

`pjep`'s Path 1 recommendation states the README should "remain a
repository entry point for developers, evaluators, and assistants and
should route each reader to the appropriate Pages task." The round-1
draft incorrectly froze README's rule entry as unchanged. Corrected:
`documentation-audience-rule.md`'s README line now states README targets
developers **and routes evaluators and AI assistants to the appropriate
Pages task** — carrying `pjep`'s routing contract into the normative rule
text. This is a single, bounded rule-text sentence, not !1082's visual
layout or restructure work; whether `README.md`'s own prose already
satisfies the corrected rule, and if not, what a minimal compliant edit
looks like, is confirmed at implementation time as a small, one-file
check — in scope for this plan precisely because it is one file, not a
26-page retrofit.

## Rule-projection screen — resolves `B-DA-4`

Per `documentation-rule-projection-rule.md`, implementation MUST produce
`docs/reviews/rule-projection/documentation-audience-rule-correction.yaml`
disposing all six rows, with shipped and project-local surfaces listed
**separately** (the round-1 draft wrongly conflated them):

| Surface | Expected disposition |
|---|---|
| Normative owner | `updated` — four files, two per rule: `.claude/rules/docs/documentation-audience-rule.md` (project-local), `agf/rules/docs/documentation-audience-rule.md` (**shipped**, matches `agf/rules/**` in `agf/dist/release-manifest.yaml`'s include glob), `.claude/rules/docs/marketing-tone-rule.md` (project-local), `agf/rules/docs/marketing-tone-rule.md` (**shipped**) — all four in the same commit |
| Adopter-facing explanation | Checked at implementation time against `agf/ADOPTING.md` and any Pages page restating the old framing; `existing pointer sufficient` unless restatement is found |
| Runbook/situation map | Checked at implementation time; `not applicable` unless a runbook page restates the old audience framing |
| AI landing/loaders | `existing pointer sufficient` — `AGENTS.md`/`agf/templates/AGENTS.md` load the rules directory by reference |
| Validation/drift evidence | `advisory only` — no mechanical validator checks Pages-page audience-metadata presence; out of this plan's scope |
| Distribution | `updated` for the two `agf/rules/**` files; `not applicable (project-local only)` for the two `.claude/rules/**` files — they do not ship via the `agf/rules/**` include glob and must not be claimed as shipped |

## Explicitly out of scope

- Retrofitting existing Pages pages beyond what
  `docs-audience-boundary-proposal.md` already tracked (its own
  remaining items, if any, are that plan's follow-up, not this one's).
- Any new mechanical validator for audience-metadata presence (flagged
  `advisory only` above).
- Any change to !1082's content, the visual-refresh mockup, or the
  `v2.5.1` hold `5m41`.
- Reconciling `docs-audience-boundary-proposal.md`'s own still-open
  follow-up items (if any remain) with the two other prior docs plans
  its own text names (`agents-govern-2.0-pedagogical-docs-restructure.md`,
  `docs-assessment-2026-06-21.md`) — that reconciliation, if still
  needed, belongs to those plans' own governance thread, not this one.

## Test strategy and verification — resolves `B-DA-6`

Documentation-only change; no unit/integration tests apply. Exact
commands (corrected from the round-1 draft's bare, non-executing `agov
validate`):

- `agov validate adopter-docs` — the actual adopter-facing-documentation
  coverage check (bare `agov validate` only prints subcommand help and
  runs nothing).
- `agov plan projection-check --base-ref origin/main docs/reviews/
  rule-projection/documentation-audience-rule-correction.yaml` — the
  actual structured six-surface projection-evidence validator.
- `agov plan validate` against this plan file.
- Manual diff confirming both rule-pairs (audience-rule,
  marketing-tone-rule) are textually identical in substantive content
  between their `.claude/` and `agf/` copies.
- Strict `mkdocs build --strict` if any Pages page (e.g. README's
  cross-referenced content) is touched at implementation time.

## Risks

- Wordsmithing drift between copies if edited separately — mitigate by
  editing all four files in the same commit and diffing for substantive
  agreement before proposing the commit.
- Scope creep into a full page-by-page retrofit — explicitly excluded
  above; a reviewer should block if the implementing branch grows beyond
  the four rule files, the README one-line check, and the projection
  evidence file.
- Under-scoping if `marketing-tone-rule.md`'s generalization accidentally
  drops a real, still-useful tone principle while removing the
  business-persona specifics — implementation must preserve every
  principle (empathy, vendor respect, positive framing, message
  integrity) and only replace the persona-specific table/examples.

## Rollback

Four small textual rule-content changes plus one projection-evidence
file. Revertable with a follow-up commit restoring prior wording if the
governor later decides the correction was wrong — no schema, code, or
irreversible state change.

## Round history

**Round 1** (`qhqg`, BLOCK, 6 findings + 1 advisory) resolved in this
revision:
- `B-DA-1`: `marketing-tone-rule.md` (both copies) now included in the
  governed correction, generalized rather than left contradicting the
  corrected audience rule.
- `B-DA-2`: distributed copies now state a generic, reusable structural
  invariant with an explicit `## CUSTOMIZE` boundary; Foundry's project
  copies specialize it to AGF's actual taxonomy and audiences, grounded
  in the already-approved `docs-audience-boundary-proposal.md` rather
  than hard-coding AGF's audience into the distributed template.
- `B-DA-3`: chosen temporal contract is prospective, explicitly defined
  ("substantively revised"); inventory corrected to 26 (reproducible,
  exhaustive count); the false "Foundry already complies" claim removed
  in favor of the accurate, already-approved 22-of-34 partial-compliance
  figure.
- `B-DA-4`: distribution row now lists the two shipped `agf/rules/**`
  files and the two project-local `.claude/rules/**` files separately.
- `B-DA-5`: README's rule entry now carries `pjep`'s three-audience
  routing contract (developers/evaluators/AI assistants → route to
  Pages task) as a bounded, one-file addition.
- `B-DA-6`: verification section now names the actual commands
  (`agov validate adopter-docs`, `agov plan projection-check --base-ref
  <ref> <manifest>`).
- `A-DA-1`: MR description and labels to be updated to match this
  revision's actual scope when round 2 is requested.

Also incorporated the round-1-missed discovery that
`docs-audience-boundary-proposal.md` (approved 2026-07-20) already
defines and implements the taxonomy this plan corrects the rules to
match — the round-1 draft independently reinvented an audience contract
without checking for this prior, already-governing artifact.
