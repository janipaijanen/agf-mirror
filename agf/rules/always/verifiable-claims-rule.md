---
provenance: verified-agf-v2
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Verifiable Claims Rule
#
# Framework principle: Implementation Honesty
# See: framework.md -- Appendix: Validation Status, rules/docs/implementation-honesty-rule.md
# Evidence: iron-gate (critical -- prevented unverifiable claims in documentation and publications)

## Rule

All factual claims in project documentation MUST be verifiable.

A claim is **verifiable** if it meets at least one of these criteria:

1. It references a specific, citable source (academic paper, standard, official documentation)
2. It describes the project's own implementation (verifiable by reading the code)
3. It is a logical consequence of referenced facts

Claims that cannot be verified MUST NOT be presented as facts. If a
claim is based on professional judgement rather than a citable source,
it MUST be qualified with hedging language:

- "Many" instead of "most" (unless a survey supports "most")
- "Typically" instead of "always"
- "In our assessment" instead of stating as fact
- "Based on publicly available documentation" when comparing alternatives

## Examples

**Incorrect** (unverifiable claim presented as fact):

> "Most platforms in this space are closed-source and GUI-based."

**Correct** (hedged or sourced):

> "Many platforms in this space are GUI-based and do not publish their
> data formats as open specifications."

Or with a source:

> "Existing platforms primarily use graphical interfaces for
> configuration (Author, Year)."

## Rationale

Project documentation may be read by evaluators, standards bodies,
competitors, and potential adopters performing due diligence. An
unverifiable claim undermines the credibility of the entire document --
even if the claim happens to be true. The cost of hedging language is
one word; the cost of a challenged unsourced claim is the reader's trust.

Comparative claims are especially sensitive. Stating that an alternative
lacks a capability requires either a citation or the qualifier "based on
publicly available documentation as of [date]."

## Enforcement

- **Per-change review**: Governor-Reviewer flags comparative claims or
  superlatives ("most", "all", "no other", "first") that lack a citation
  or hedging language. Severity: WARN.
- **Per-milestone review**: Governor-Reviewer performs a full scan for
  unqualified comparative claims before any document is shared externally.

## Remediation

Either:
1. Add a citation (prefer academic sources, standards documents, or official documentation)
2. Add hedging language ("many", "typically", "in our assessment")
3. Remove the claim if it cannot be qualified
