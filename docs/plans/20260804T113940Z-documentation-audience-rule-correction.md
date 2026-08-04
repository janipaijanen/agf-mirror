---
id: documentation-audience-rule-correction
status: pending-gate1
topic: documentation-audience-rule-correction
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      This plan implements Path 1 of `contribution-auditor`'s independent
      second opinion `20260803T225851Z-contribution-auditor-pjep.md`
      (finding 1: "the audience mismatch is real"), which itself confirmed
      and extended `20260803T225314Z-planner-architect-b27h`'s original
      finding. Requesting round-1 review of this plan once committed.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Documentation-rule wording change only; no credential, auth,
      network-trust-boundary, or secret handling.
---

# Plan: correct `documentation-audience-rule.md`'s audience mismatch

## Intent

`.claude/rules/docs/documentation-audience-rule.md` and its distributed
copy `agf/rules/docs/documentation-audience-rule.md` currently describe
public Pages content as targeting **generic business-application users**:
"directors, managers, employees" and "system administrators who deploy
and maintain the solution," focused on "deployment, QA verification,
operations, data entry, configuration, backup." AGF's actual public Pages
site (`agf/docs/site/docs/index.md`) is not a business application — it
is a technical governance-framework site organized around four page
kinds (Human decision, AI action, Shared concept, Reference) addressing
evaluators, adopters, human governors, developers, operators, and AI
assistants. The rule can direct an author toward unsupported maturity
claims about capabilities (data entry, backup) AGF does not have, and
directly contradicts what the shipped site already says about itself.

This is the specific blocking prerequisite MR !1082 names in its own
description ("Blocking prerequisite... `documentation-audience-rule.md`'s
target-audience framing conflicts with AGF's actual technical audience —
flagged, not resolved") and that `pjep`'s independent review confirmed
and recommended fixing (Path 1) before any further Pages/README
visual-identity work proceeds.

**Out of scope for !1082 itself.** This plan does not touch !1082,
`v2.5.1`, or the still-binding governor hold `5m41` (see
`docs/plans/20260804T113940Z-v260-coordination-reliability.md`'s
"Explicitly excluded" section for why !1082 is tracked separately, not
folded into this change or into `v2.6.0`).

## Downstream-impact classification

Per `always/downstream-impact-rule.md`: this is **BREAKING**, not
EDITORIAL. `documentation-audience-rule.md` is a trigger-path artifact
(`.claude/rules/**/*.md`, `agf/rules/**/*.md`), and the set of actions
the rule requires changes — a corrected rule requires each Pages page to
state primary/secondary audience, reader task, execution owner
(human/AI/both), and a human verification path for AI-action pages; the
current rule requires none of that and instead names a wrong audience
class entirely. Both copies must change in the same governed commit —
per `pjep`'s explicit finding, "changing only one preserves the
contradiction."

### Three-circle impact statement

```
Impact:
  Foundry:          Yes. This project's own Pages site already implicitly
                     follows the corrected framing (four-kind taxonomy,
                     technical audience) and not the current rule text —
                     this change brings the rule into agreement with
                     existing shipped content, not the reverse.
  Direct adopters:  BREAKING. An adopter who copied the prior rule text
                     verbatim and wrote Pages content for a generic
                     business audience (directors/managers/employees,
                     data-entry/backup framing) will find the corrected
                     rule asks for different information: audience,
                     task, execution owner, verification path, oriented
                     around a technical/AI-governance readership.
  Downstream:       Adopter-produced Pages content shape changes going
                     forward (new required per-page metadata). No
                     schema, CLI, or MCP behavior changes.
  Migration:        See below.
  Evidence:         pjep (20260803T225851Z-contribution-auditor-pjep.md),
                     confirming and extending b27h
                     (20260803T225314Z-planner-architect-b27h.md).
```

## Migration note (required — BREAKING)

Adopters who have written Pages content following the prior generic
business-audience framing:

1. No immediate action required for already-published pages — this rule
   change does not retroactively invalidate existing content (per
   `documentation-rule-projection-rule.md`'s baseline-audit boundary:
   "prospective evidence... does not retroactively invalidate or
   rewrite historical... already-published releases").
2. **New or substantively revised pages**, from this rule's adoption
   forward, must state: primary and secondary audience; the reader's
   task and prerequisite knowledge; whether execution belongs to a
   human, an AI assistant, or both; and, for AI-action pages, the human
   verification or troubleshooting path.
3. Adopters whose actual Pages audience genuinely is a generic business
   application (not AGF's own case, but a real possibility for a fork or
   derivative) should adapt the site-wide primary-audience line to their
   own product, per this rule's existing `## CUSTOMIZE`-style convention
   used elsewhere in `agf/rules/`.

## Corrected rule content (both copies, identical substantive text)

Replace the existing "README.md" / "Public documentation site" framing
in `documentation-audience-rule.md` with:

- **README.md** (repo root): unchanged — targets **developers**,
  development experience, unchanged by this plan.
- **Public documentation site** (content under `docs/`): targets
  **technical adopters and operators, and human governors** evaluating
  or supervising AI-assisted development, as primary audiences.
  Engineering leaders and risk/security/compliance/assurance
  stakeholders are valid **secondary** audiences on decision pages.
  Every page must state its primary/secondary audience, the reader's
  task and prerequisite knowledge, whether execution belongs to a
  human, an AI assistant, or both, and — for AI-action pages — the
  human verification/troubleshooting path. Reference pages may remain
  precise and assume the relevant Concepts-page prerequisite. Convey
  that the framework is mature (schema migrations, audit logging via
  the coordination channel, learning-record evidence) without
  overclaiming capabilities AGF does not have (this is not a business
  application with data-entry/backup features).

Exact final wording is drafted at implementation time against this
plan's intent, not frozen here — the plan states the substantive
contract change; wordsmithing happens in the implementing commit and is
reviewed at Gate 2 like any other content change.

## Rule-projection screen (required — normative guidance change)

Per `documentation-rule-projection-rule.md`, this plan's implementation
MUST produce
`docs/reviews/rule-projection/documentation-audience-rule-correction.yaml`
disposing all six rows before Gate 2:

| Surface | Expected disposition |
|---|---|
| Normative owner | `updated` — `.claude/rules/docs/documentation-audience-rule.md` and `agf/rules/docs/documentation-audience-rule.md`, same commit |
| Adopter-facing explanation | `existing pointer sufficient` if `agf/ADOPTING.md` already points to the rules directory without restating content; `updated` if it restates the old framing anywhere (must be checked at implementation time) |
| Runbook/situation map | `not applicable` unless a runbook page currently restates the old audience framing (must be checked) |
| AI landing/loaders | `existing pointer sufficient` — `AGENTS.md`/`agf/templates/AGENTS.md` load the rules directory by reference, not by restating rule content |
| Validation/drift evidence | `advisory only` — no mechanical validator currently checks Pages-page audience-metadata presence; adding one is out of this plan's scope (candidate future issue, not blocking this correction) |
| Distribution | `updated` — both changed files ship in the `agf/rules/**` include glob per `agf/dist/release-manifest.yaml` |

The exact dispositions above are this plan's best current assessment,
not final — the implementing change must verify each row against the
actual file contents at implementation time, per the rule's own
requirement that a pointer be genuinely sufficient, not assumed.

## Explicitly out of scope

- **Retrofitting existing Pages pages** with the new per-page audience
  metadata. Per `documentation-rule-projection-rule.md`'s baseline-audit
  boundary, this plan fixes the rule and its direct projections; a
  full per-page metadata retrofit across the site's ~37 pages is a
  separate, later, optionally-scoped cleanup — tracked as a new issue
  if the governor wants it, not bundled here (bundling would fail the
  branch sizing test: "does the rule now say the right thing" is a
  different reviewable question from "does every page comply with it").
- Any change to !1082's content, the visual-refresh mockup, or the
  `v2.5.1` hold `5m41`.
- Any new mechanical validator for audience-metadata presence (flagged
  `advisory only` above).

## Test strategy

Documentation-only change; no unit/integration tests apply. Verification
is: `agov validate` (adopter_docs check, if it inspects rule content —
confirm at implementation time), a manual diff confirming both rule
copies are textually identical in substance, and the six-row YAML
evidence file passing whatever mechanical check
`documentation-rule-projection-rule.md`'s tooling provides (currently
human/role judgment per that rule's own text — "advisory only" for the
normative-detection step).

## Risks

- Wordsmithing drift between the two rule copies if edited separately —
  mitigate by editing both in the same commit and diffing them for
  substantive (not just textual) agreement before proposing the commit.
- Scope creep into a full page-by-page retrofit — explicitly excluded
  above; a reviewer should block if the implementing branch grows beyond
  the two rule files plus the six-row evidence file.

## Rollback

Single small commit, textual rule-content change only. Revertable with a
follow-up commit restoring prior wording if the governor later decides
the correction was wrong — no schema, code, or irreversible state
change.

## Round history

None yet — first draft, pending round-1 contribution-auditor review.
