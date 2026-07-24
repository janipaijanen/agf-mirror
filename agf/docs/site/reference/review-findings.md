# Review findings

*Status: Validated authoring surface; evidence status: Theoretical. Audience:
reviewers, governors, and tool authors. This page defines an advisory way to
write review evidence. It does not create a gate or grant blocking authority.*

AGF reviews need to answer several different questions without forcing them
into one word:

- What was reviewed, and is the review complete?
- What did each check establish?
- How serious is a finding?
- Does a named rule make it blocking, or is it advisory?
- Is the underlying problem still open?
- Has an authorized decision-maker accepted the risk for a bounded scope?
- What does the reviewer recommend?
- Who makes the binding gate decision?

The convention on this page keeps those answers separate. It is an authoring
aid, not a body schema. `agov coord brief --kind review` can print the skeleton,
but tooling does not validate, aggregate, approve, or reject its contents.
Failure to use this convention is not automatically a blocking finding.

## Review envelope

Start with the exact artifact and revision:

```text
Scope: <artifact and exact revision>
Coverage: COMPLETE | INCOMPLETE
```

When a recommendation can be issued, add:

```text
Recommendation: CLEAR | BLOCK
Blocking findings: <open blocking findings without applicable risk acceptance>
Advisory findings: <count>
Open findings with accepted risk: <count>
Authority: recommendation only; the authorized decision-maker decides the gate
```

`CLEAR` requires complete coverage. A confirmed blocker does not: if one area
is unreviewed while another already contains an open blocking finding, report
both `Coverage: INCOMPLETE` and `Recommendation: BLOCK`.

If coverage is incomplete and no blocker is yet known, omit the recommendation
and name the missing evidence or judgment. `INCOMPLETE` is a coverage state,
not a softer verdict.

## Separate fields

### Requirement strength

Use the optional reader labels `mandatory | recommended | optional |
not-stated`. Always cite the governing basis. Map these labels to BCP 14
`MUST | SHOULD | MAY` only when the source intentionally invokes BCP 14;
otherwise preserve the source's own wording rather than inventing a BCP 14
classification.

Requirement strength does not determine impact. A low-impact violation of a
mandatory, blocking rule may block; a critical-impact concern remains advisory
when no rule or authorized role grants it blocking effect.

### Check status and evaluation outcome

Check status is `reviewed | not-reviewed`.

A reviewed check has one outcome:

- `pass` — the evaluated requirement was satisfied;
- `fail` — the evaluated requirement was not satisfied;
- `needs-review` — a human or authorized role must resolve the judgment; or
- `not-applicable` — the requirement does not apply, with the basis recorded.

A not-reviewed check has no outcome. Record what evidence or work is missing.

| Check status | Outcome | Finding? | Consequence |
|---|---|---|---|
| `not-reviewed` | omitted | no | Coverage is incomplete; another known blocker still yields BLOCK |
| `reviewed` | `pass` | no | Contributes to complete coverage |
| `reviewed` | `not-applicable` | no | Record the basis; contributes to complete coverage |
| `reviewed` | `needs-review` | yes | Coverage stays incomplete until the judgment resolves; blocking enforcement still yields BLOCK |
| `reviewed` | `fail` | yes | Apply the finding fields below |

Information-only observations use an `Annotation` label outside this table.
They are neither check outcomes nor findings and do not affect aggregation.

### Finding fields

Use a finding for `fail` or `needs-review` when an issue or unresolved judgment
requires tracking:

```text
Finding: <identifier>
Impact: critical | material | minor | unknown
Enforcement: blocking | advisory
Basis: <named rule/requirement, or explicitly advisory judgment>
Finding state: open | resolved | superseded
Risk disposition: none | accepted
Risk decision: <required when accepted: decision ID, scope/revision, revisit condition>
Resolution condition: <observable condition>
```

Impact describes consequence, not authority. Enforcement says whether the
finding stops the workflow and why. A blocking finding needs a named rule or an
authorized role decision; volume, rhetoric, tool confidence, and impact alone
do not grant blocking authority.

Finding state describes remediation. Risk disposition describes a separate
decision. An accepted finding stays `open` until its resolution condition is
met. `accepted` is valid only when an authorized decision covers the exact
scope and revision and records when the decision must be revisited.

## Recommendation truth table

Apply these rules in order:

1. Any open blocking finding without applicable authorized risk acceptance
   yields `BLOCK`, under complete or incomplete coverage.
2. With no such blocker, a not-reviewed check, unresolved needs-review outcome,
   or missing required evidence yields `INCOMPLETE`; omit the recommendation.
3. Complete coverage with no unaccepted open blocking finding yields `CLEAR`.

Advisory findings and open findings with accepted risk remain visible under
`CLEAR`. A reviewer recommendation never approves a gate.

## Examples

### Complete review with no findings

```text
Scope: MR !100 at 0123456789abcdef
Coverage: COMPLETE
Recommendation: CLEAR
Blocking findings: 0
Advisory findings: 0
Open findings with accepted risk: 0
Authority: recommendation only; the authorized decision-maker decides the gate
```

### Critical advisory finding

```text
Scope: MR !101 at 123456789abcdef0
Coverage: COMPLETE
Recommendation: CLEAR
Blocking findings: 0
Advisory findings: 1
Open findings with accepted risk: 0

Finding: F1
Impact: critical
Enforcement: advisory
Basis: advisory reviewer judgment; no named rule grants blocking authority
Finding state: open
Risk disposition: none
Resolution condition: owner evaluates the reported failure mode
```

Critical impact does not silently create authority.

### Minor but blocking finding

```text
Scope: MR !102 at 23456789abcdef01
Coverage: COMPLETE
Recommendation: BLOCK
Blocking findings: 1
Advisory findings: 0
Open findings with accepted risk: 0

Finding: F1
Impact: minor
Enforcement: blocking
Basis: named mandatory rule that grants this check blocking authority
Finding state: open
Risk disposition: none
Resolution condition: restore the required field and pass validation
```

Low impact does not cancel named authority.

### Known blocker and incomplete coverage

```text
Scope: MR !103 at 3456789abcdef012
Coverage: INCOMPLETE
Recommendation: BLOCK
Blocking findings: 1
Advisory findings: 0
Open findings with accepted risk: 0
Missing evidence: screen-reader evaluation not performed

Finding: F1
Impact: material
Enforcement: blocking
Basis: named accessibility rule
Finding state: open
Risk disposition: none
Resolution condition: correct the confirmed narrow-layout failure
```

Missing evidence does not hide a blocker already established elsewhere.

### Incomplete coverage with no known blocker

```text
Scope: MR !104 at 456789abcdef0123
Coverage: INCOMPLETE
Missing evidence: human judgment for check C4
```

No recommendation is issued.

### Open finding with bounded risk acceptance

```text
Scope: MR !105 at 56789abcdef01234
Coverage: COMPLETE
Recommendation: CLEAR
Blocking findings: 0
Advisory findings: 0
Open findings with accepted risk: 1

Finding: F1
Impact: material
Enforcement: blocking
Basis: named accessibility rule
Finding state: open
Risk disposition: accepted
Risk decision: governor decision D17; MR !105 at 56789abcdef01234; revisit before the next release
Resolution condition: complete the deferred accessibility evaluation and remediate findings
```

The issue remains open. The decision only permits the named revision and scope
to proceed. Removing the decision, changing the revision, or omitting the
revisit condition makes the acceptance inapplicable and the recommendation
BLOCK.

## Existing prose reviews

Do not edit immutable historical coordination records. Read their old labels
in context:

- historical `BLOCK` is a recommendation, but may not reveal its authority
  basis;
- historical `WARN` is non-binding prose; when restating it, use advisory
  enforcement and impact `unknown` unless the source establishes impact;
- historical `INFO` is an annotation and does not imply pass or minor failure;
  and
- historical `CLEAR` is a recommendation, not proof of complete coverage or
  gate approval.

Use the convention prospectively. A mandatory body contract, validator, or
interchange format requires its own governed plan and authority source.

## Standards boundary

The separation is informed by several established vocabularies:

- [BCP 14 (RFC 2119 and RFC 8174)](https://www.rfc-editor.org/info/bcp14)
  informs requirement strength only.
- [SARIF 2.1.0 plus Errata 01](https://docs.oasis-open.org/sarif/sarif/v2.1.0/sarif-v2.1.0.html)
  separates result kind from severity for static-analysis interchange.
- [W3C EARL 1.0](https://www.w3.org/TR/EARL10-Schema/) distinguishes passed,
  failed, uncertain, inapplicable, and untested results.
- [LSP 3.17 DiagnosticSeverity](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#diagnosticSeverity)
  was considered but lacks result, lifecycle, enforcement, and authority axes.
- [OSCAL Assessment Results 1.1.2](https://pages.nist.gov/OSCAL-Reference/models/v1.1.2/assessment-results/json-reference/)
  was considered but is substantially broader and system-assessment-specific.

AGF does not claim SARIF, EARL, LSP, or OSCAL conformance here. These sources
inform an independently defined, prose-level convention.

## Goodhart guard

Counts make one review auditable. Do not turn finding count, CLEAR rate, BLOCK
rate, advisory count, or time-to-CLEAR into quotas, rankings, rewards,
productivity measures, or quality targets. Once optimized, those numbers invite
authors to hide, merge, split, downgrade, or prematurely close findings.

Review quality rests on traceable scope, complete evidence, reproducible
checks, named authority, visible residual risk, and a correct decision chain.
The numbers are observations, not goals.
