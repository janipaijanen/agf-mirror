---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260722T224125Z-contribution-auditor-gq5a
timestamp: 2026-07-22T22:41:25Z
model: codex-gpt-5
immutable: true
references-to: []
topic: critical-screen-standardized-review-vocabulary
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Critically screen the candidate standards and the provisional AGF mapping below.
    - Return a validated coordination review that recommends adoption, adaptation, or rejection and records concrete objections.
    - Apply both a fact-checking lens and a low-friction reader lens, including readers who are not native English speakers.
  must_read:
    - https://docs.oasis-open.org/sarif/sarif/v2.1.0/sarif-v2.1.0.html
    - https://www.rfc-editor.org/info/rfc2119
    - https://www.rfc-editor.org/info/rfc8174
    - https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#diagnosticSeverity
    - https://pages.nist.gov/OSCAL-Reference/models/v1.1.2/assessment-results/json-reference/
  do_not_touch:
    - agf/schemas/coordination-message.schema.json
    - agf/rules/
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Critical screen: standard vocabulary for review findings and outcomes

The governor asks whether AGF should refer to a recognized standard for its
current prose-only `BLOCK`, `WARN`, `INFO`, and `CLEAR` review vocabulary
instead of inventing its own classification. This is a critical-screen task,
not an instruction to validate a preferred answer.

## Problem to separate

Do not collapse these four different concepts into one severity word:

1. **Requirement strength** — how binding the evaluated rule is.
2. **Evaluation result** — whether the rule passed, failed, was inapplicable,
   remains uncertain, or needs human review.
3. **Finding severity** — how serious a detected problem is.
4. **Enforcement and authority** — whether an unresolved finding stops a
   handoff or merge, and who may accept the risk or approve the gate.

AGF currently gives these distinctions only partial structure. In particular,
`BLOCK` is used like both severity and workflow disposition, while `CLEAR` can
be mistaken for binding governor approval.

## Candidate standards

### OASIS SARIF 2.1.0 plus Errata 01 — leading candidate

SARIF `result.kind` distinguishes `pass`, `open`, `informational`,
`notApplicable`, `review`, and `fail`. Only a `fail` has a substantive
`result.level`: `error`, `warning`, or `note`; non-fail kinds use `none`.
This directly separates evaluation result from problem severity and explicitly
supports human review. It also supports rule identity, locations, provenance,
suppressions, and baselines.

Objection to test: SARIF is formally a static-analysis interchange format.
AGF reviews also cover plans, prose, evidence, governance, and human judgment.
Referencing SARIF semantics may be useful, while claiming SARIF conformance or
copying its full JSON object model may be misleading and burdensome.

### BCP 14: RFC 2119 plus RFC 8174 — complementary, not a severity model

`MUST`, `SHOULD`, and `MAY` express requirement strength. They do not classify
review outcomes or finding severity. They may provide AGF's rule-strength axis,
but they cannot replace a result/finding model.

### Language Server Protocol DiagnosticSeverity — recognizable but incomplete

LSP defines `Error`, `Warning`, `Information`, and `Hint`. It is familiar to
developers but has no pass, not-applicable, uncertain, human-review, lifecycle,
or governance-authority semantics. Test whether its familiarity outweighs
those gaps.

### NIST OSCAL Assessment Results — rich but likely too heavy

OSCAL models observations, findings, risks, origins, targets, and risk status,
including open, investigating, remediating, deviation-requested,
deviation-approved, and closed. It may inform lifecycle terms, but its model is
large and security/control-assessment oriented. Test whether adopting any of
its terms without an OSCAL exchange requirement would add ceremony rather than
interoperability.

Syslog severity (RFC 5424) and OpenTelemetry log severities are deliberately
not shortlisted: they classify operational log events, not review results.

## Provisional AGF profile to attack

Prefer a small, documented profile of SARIF meanings rather than full SARIF
wire-format conformance:

- Rule strength: BCP 14 `MUST | SHOULD | MAY` where normative language is used.
- Evaluation kind: SARIF-aligned `pass | fail | review | open |
  informational | not-applicable`.
- Finding severity, only for `fail`: SARIF-aligned `error | warning | note`.
- Finding lifecycle: an explicitly AGF-defined status such as `open | resolved
  | accepted-risk | superseded`; assess whether OSCAL-aligned terms would be
  clearer or more interoperable.
- Workflow recommendation: a separate AGF field such as `changes-required |
  ready-for-governor | inconclusive`.
- Enforcement disposition: `blocking | non-blocking`, derived from a named AGF
  rule and role authority, never inferred from severity alone.
- Binding decision: remains a governor `decision-record`; neither `pass` nor
  `ready-for-governor` means approved.

Under this proposal, existing uppercase words become presentation aliases at
most:

- `BLOCK` -> unresolved finding with `blocking` enforcement; not a severity.
- `WARN` -> usually `fail + warning + non-blocking`; mapping must not be
  automatic if policy says the finding blocks.
- `INFO` -> `informational` when there is no problem, or `fail + note` when
  there is a minor problem/opportunity. These are intentionally different.
- `CLEAR` -> review recommendation `ready-for-governor` only after all relevant
  checks are accounted for; never gate approval.

## Required adversarial questions

1. Is SARIF semantically defensible for human/agent plan, documentation, and
   governance reviews, or is its static-analysis scope a category error?
2. Should AGF reference SARIF definitions, publish an explicit AGF profile, or
   avoid SARIF and use a smaller independently defined model?
3. Is the proposed multi-axis model understandable in actual coordination
   messages, or too complex for the value it creates?
4. Are the proposed reader-facing terms clear to non-native English speakers?
   Suggest plainer labels if not.
5. Does `error | warning | note` incorrectly imply that all errors block, or can
   the separate enforcement field prevent that confusion?
6. Is `ready-for-governor` too role-specific for solo-maintainer/adopter
   framing? Propose a lower-friction term that still cannot be mistaken for
   approval.
7. What lifecycle vocabulary is actually necessary, and what is the minimum
   backward-compatible migration from the existing prose reviews?
8. Identify any better open, recognized standard or taxonomy that was missed.

Record factual corrections and semantic objections explicitly. A useful
answer may reject the entire proposal. Do not change schemas, rules, or review
templates yet. This is governed post-release design work and must not displace
the active v2.4.0 release-blocking sequence.
