---
id: review-finding-contract-v1
status: pending-gate1
topic: review-finding-contract-v1
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >
      Contribution-auditor filed the critical standards screen in
      20260722T224125Z-contribution-auditor-gq5a, requested joint planning in
      20260723T000236Z-contribution-auditor-atpa, and proposed the compact
      evidence envelope in 20260723T004115Z-contribution-auditor-depe. The
      draft is returned for a fresh adversarial critique before Gate 1.
  - role: infosec-sentinel
    status: not-applicable
    reason: >
      The slice changes public explanatory documentation and a local generated
      Markdown skeleton. It adds no secrets, permissions, external transport,
      trust-boundary crossing, learning record, or enforcement mechanism.
---

# Plan: review finding contract v1

## Intent

Replace the ambiguous prose-only use of `BLOCK | WARN | INFO | CLEAR` for new
reviews with a small, explicit evidence envelope. Keep the familiar `CLEAR |
BLOCK` reader recommendation, add an abstention path for incomplete review,
and separate requirement strength, evaluation outcome, impact, enforcement,
lifecycle, recommendation, and binding authority.

This is a documentation-and-authoring-aid slice. It is not a new gate, metric,
schema, standards interchange format, or grant of authority to tooling.

## Affected files

- Add `agf/docs/site/reference/review-findings.md`: normative reader contract,
  aggregation rules, examples, migration guidance, standards boundary, and
  Goodhart guard.
- Update `agf/docs/site/docs/operate/review-and-release.md`: link to the
  contract and use its review envelope without changing the gate workflow.
- Update `agf/docs/site/docs/concepts/coordination-protocol.md`: define how a
  `kind: review` body uses coverage, recommendation, findings, and authority.
- Update `agf/docs/build/mkdocs.yml`: add the reference page to navigation.
- Update `tooling/rust/crates/agov/src/main.rs`: replace the legacy generated
  `# <findings — severity: BLOCK | WARN | INFO>` review-body placeholder with a
  concise contract-shaped skeleton.
- Update `tooling/rust/crates/agov/tests/coord_brief.rs`: assert the new review
  skeleton and the absence of the legacy severity placeholder.

No changes to `agf/schemas/`, `agf/rules/`, agent role files, front matter,
coordination validation, decision authority, or historical coordination files.

## Contract to document

### Review envelope

Every new review identifies an exact artifact/revision and one coverage state:

```text
Scope: <artifact and exact revision>
Coverage: COMPLETE | INCOMPLETE
```

Complete coverage also includes:

```text
Recommendation: CLEAR | BLOCK
Blocking findings: <count>
Advisory findings: <count>
Authority: recommendation only; the authorized decision-maker decides the gate
```

Incomplete coverage omits `Recommendation`, names missing evidence or pending
judgment, and cannot be represented as `CLEAR` or `BLOCK`.

### Separate axes

- Requirement strength: BCP 14 `MUST | SHOULD | MAY` only where intentionally
  normative and properly declared.
- Evaluation outcome: `pass | fail | needs-review | not-applicable |
  not-reviewed`.
- Finding impact: `critical | material | minor | informational`.
- Enforcement: `blocking | advisory`, with an explicit basis.
- Finding state: `open | resolved | accepted-risk | superseded`.
- Review recommendation: `CLEAR | BLOCK`, derived only for complete coverage.
- Binding gate decision: separate authorized coordination decision.

Ordinary reviews may summarize passing checks rather than printing one record
per pass. Every non-pass finding must preserve enough information to reconstruct
the outcome, impact, enforcement basis, state, and resolution condition.

### Aggregation

1. Relevant `not-reviewed`, unresolved `needs-review`, or missing evidence =>
   `INCOMPLETE`; omit recommendation.
2. Complete coverage plus any `open + blocking` finding => `BLOCK`.
3. Complete coverage with no `open + blocking` finding => `CLEAR`, while all
   advisory and accepted-risk findings remain visible.
4. `accepted-risk` requires a referenced decision from the authorized role;
   reviewers and tools cannot create it by relabeling severity.

### Standards and evidence claims

The page will cite BCP 14, SARIF 2.1.0 plus Errata 01, W3C EARL 1.0, LSP 3.17,
and OSCAL 1.1.2 accurately. It will state:

- BCP 14 informs requirement strength only.
- SARIF and EARL inform the separation and outcome vocabulary.
- LSP and OSCAL were considered but not adopted.
- AGF does not claim SARIF, EARL, LSP, or OSCAL conformance in this slice.
- The recommendation is **Theoretical** as a vocabulary design until the
  planned reader and historical-case tests are completed; implementation and
  tests can establish Level 4 for the documented/template surface, not
  empirical validation of review quality.

## Generated review skeleton

`agov coord brief --kind review` should generate comments/placeholders that
prompt for scope, coverage, conditional recommendation, counts, authority, and
repeatable finding fields. It must remain valid Markdown/YAML after filling.
It must not calculate a recommendation, validate prose-body semantics, or
present itself as an approving or blocking tool.

The skeleton should be short enough that a reviewer can delete unused finding
blocks. Help output for other coordination kinds is unchanged.

## Compatibility and migration

- Do not edit or reinterpret immutable historical coordination records.
- New reviews use the contract after the implementation merge.
- Historical `BLOCK` remains understandable but does not prove which rule
  granted enforcement; new reviews must name the basis.
- Historical `WARN` is treated as non-binding prose. Migration to a new finding
  may use `advisory`, but impact is `unknown` in migration notes unless the
  source establishes it; no automatic `warning` mapping is claimed.
- Historical `INFO` remains an annotation and must not be inferred to mean
  either pass or minor failure.
- Historical `CLEAR` remains a non-binding recommendation; it is not
  retroactive evidence of complete coverage or gate approval.
- No consumer of current coordination front matter breaks because the new
  structure is in the Markdown body only.

Any future body schema, structured front-matter extension, SARIF exporter, or
enforcement validator requires a separate Gate 1 plan with a demonstrated
consumer and migration design.

## Acceptance tests

### Deterministic contract cases

Document and test these examples:

1. Complete scope, no findings => `CLEAR`, zero counts, authority reminder.
2. Complete scope, one open blocking finding => `BLOCK`.
3. Complete scope, critical advisory finding only => `CLEAR` with the advisory
   visible; impact alone does not grant blocking authority.
4. Complete scope, minor open finding backed by a named blocking rule =>
   `BLOCK`; low impact does not cancel rule authority.
5. Missing required evidence => `INCOMPLETE` and no recommendation line.
6. Unresolved human judgment => `INCOMPLETE`, not `BLOCK`, until the outcome
   and enforcement basis are known.
7. Resolved or superseded blocking finding with no other open blocker =>
   `CLEAR`, with history retained.
8. Accepted-risk finding => `CLEAR` only when a referenced authorized decision
   exists; deleting that reference makes the example invalid.

### Historical reconstruction

Use immutable records as read-only fixtures/examples:

- straightforward CLEAR:
  `20260722T142845Z-contribution-auditor-s6mn.md`;
- multi-finding BLOCK:
  `20260722T103933Z-contribution-auditor-qjbs.md`;
- accepted-risk/deferred issue: reconstruct MR !429 from the overall BLOCK in
  `20260723T001609Z-contribution-auditor-mmbs.md`, the exact-SHA governor risk
  decision in `20260723T002811Z-governor-reviewer-5stq.md`, the subsequent
  contribution-auditor CLEAR in `20260723T002856Z-contribution-auditor-bjrk.md`,
  and the separate Gate 2 approval in
  `20260723T003222Z-governor-reviewer-31mq.md`. The example must keep risk
  acceptance distinct from both reviewer recommendation and gate approval.

For each case, two reviewers must be able to reconstruct the same coverage
state, recommendation (if issued), open blocking count, advisory count, and
authority boundary. Disagreement is a failed acceptance test and requires
wording revision before Gate 2.

### Reader checks

At least one reviewer who does not rely on native-English idiom must answer,
from each example:

1. Is the review finished?
2. Is the recommendation CLEAR, BLOCK, or not issued?
3. What must change, if anything?
4. Which findings actually stop the workflow, and why?
5. Who makes the binding decision?

All five answers must match the example's intended result without oral context.
Record paraphrased answers and any confusing term; do not collect personal or
demographic data.

### Tooling and regression checks

- Focused Rust test for `coord brief --kind review` exact required labels.
- Negative assertion that the old `severity: BLOCK | WARN | INFO` placeholder
  is absent.
- Existing `coord_brief` tests remain green for all message kinds.
- Full Rust workspace tests and formatting/lint checks required by the branch's
  CI remain green.
- `agov coord validate` continues to accept a filled example because front
  matter is unchanged.
- Documentation build and link checking pass.

## Goodhart and misuse guard

The documentation must explicitly prohibit treating finding count, CLEAR rate,
BLOCK rate, advisory count, or time-to-CLEAR as agent quality, productivity,
reward, quota, or ranking targets. Counts exist so readers can audit a review,
not so authors optimize them. Quality evidence remains traceable scope,
coverage, cited basis, reproducibility, and correct authority.

No numeric quality threshold is introduced by this plan. Numeric counts are
descriptive facts for a single review; the reader test's five questions are a
bounded comprehension check, not an ongoing score or personnel metric.

## Risks and controls

- **More fields can create ceremony.** Keep the envelope short, allow passing
  checks to be summarized, and require detailed fields only for non-pass
  findings.
- **`CLEAR` can sound like approval.** Always label it `Recommendation` and
  print the authority reminder adjacent to it.
- **Impact can leak into enforcement.** Negative examples cover critical
  advisory and minor blocking cases.
- **Incomplete can be abused to avoid BLOCK.** Coverage must name missing
  evidence/judgment; once a known rule violation has blocking authority it is
  an open blocking finding, not merely incomplete coverage.
- **Standards references can imply conformance.** The explicit non-conformance
  statement and absence of wire-format claims are acceptance requirements.
- **Template structure can become de facto schema.** This slice labels the
  skeleton an authoring aid and leaves validation unchanged.

## Rollback

Revert the new reference page, its links/nav entry, and the generated review
skeleton/test changes. Existing coordination records and schemas are untouched;
no data migration or restoration is needed.

## Gate and role boundary

This plan is `pending-gate1`. Contribution-auditor must challenge this draft
before it is submitted to governor-reviewer. No listed documentation, template,
tooling, schema, or rule change begins until governor-reviewer records Gate 1
approval. After approval, implementation routes to coder-tester; the exact diff
returns to contribution-auditor for independent Gate 2 review before the
governor's binding Gate 2 decision.
