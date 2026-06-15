---
provenance: verified-agf-v2
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# External Anonymization: Codename Only Across the Codebase Boundary

# Origin: framework
# Status: <CUSTOMIZE: replace with your status>
# Evidence: iron-gate (2026-04-28 near-miss); proposed in issue #45

This rule applies to every artifact whose destination is **outside the project codebase**: issues filed on external trackers, MRs against upstream frameworks, public repositories, blog posts, conference submissions, content sent to external LLM APIs for analysis, anything reachable beyond the adopter's internal infrastructure.

## The rule

Across the codebase boundary, the **only project handle** that may appear is the adopter's codename. Everything else MUST be omitted or replaced with a domain-neutral generic.

| Forbidden | Replace with |
|---|---|
| Project's actual name, acronym, sector, or domain descriptor | Codename only (e.g. `iron-gate`) |
| Schema type names that reveal the domain | Generic ("the project's schema types") |
| File paths whose names reveal the domain | "the project's framework directory" / "the project's tooling" |
| Internal scenario / fixture codenames | "the project's scenario fixtures" |

The codebase itself uses the project's natural vocabulary freely — that is correct, expected, and not a leak. **Anonymization applies only at the export boundary.**

## Boundary table (canonical)

| Destination | Boundary | Anonymize? |
|---|---|---|
| Adopter's own GitLab/GitHub instance, internal | Internal | No |
| Adopter's CI logs, internal observability | Internal | No |
| AI coding-assistant local working state (no provider call) | Internal | No |
| AI coding-assistant prompts/responses sent to a provider API | **External** | **Yes — codename only** |
| Upstream framework tracker (this one) | External | **Yes — codename only** |
| Public blogs, conference submissions, papers | External | **Yes — codename only** |
| External LLM APIs / SaaS analysis | External | **Yes — codename only** |
| Pre-print servers, arXiv, datacite, DOI registrations | External | **Yes — codename only** |

When uncertain whether a destination is internal: **assume external and anonymize.**

> **Why two AI-assistant rows?** "Local memory" implies content stays on your machine, but most AI coding assistants send prompts to a remote provider (Anthropic, OpenAI, GitHub) where they may be logged, retained, or used for training depending on subscription tier and provider settings. Treat the *prompt* as external content even when the *editor* feels local.

## When this rule fires

The rule fires at content-creation time, not at submission time. Adopters MUST apply anonymization while drafting any artifact whose destination is external; the artifact is sanitized before it is written, not retroactively.

## AI-assisted development specifics

For sessions involving AI coding assistants, contributors MUST:

1. **State the boundary out loud** before drafting any external content: "this content will leave the codebase boundary; anonymization rules apply." This is a context shift the agent does not detect on its own.
2. **Treat upstream contributions as InfoSec-relevant by default.** Any issue body, MR description, or learning record destined for an external project requires anonymization gate before drafting, not after.
3. **When uncertain, stop and ask the human.** State the question explicitly: "I'm about to draft external content for [destination]. Should I apply codename-only?" Better than drafting a leak the human reviewer has to catch; also better than refusing outright (which dead-ends the session).

## Retroactive audit on adoption

When an adopter first adopts this rule, externally-published artifacts that pre-date the rule MAY contain adopter-name occurrences. Adopters MUST do a one-time sweep:

1. Inventory externally-published artifacts: filed issues, MR descriptions, blog posts, conference papers, content sent to external LLMs in the past 30 days.
2. Search each for the forbidden-term list (project name, acronym, domain descriptor, schema-revealing terms, internal scenario codenames).
3. For each occurrence: redact (if the destination supports editing), or document why the occurrence is acceptable (e.g., the project is intentionally publicly named; the term has been redacted; the artifact is no longer reachable).
4. Record the audit completion in the adopter's learning record.

After this one-time audit, the rule fires on every new artifact going forward.

## Enforcement

Per-MR: contributor verifies anonymization on any artifact destined for external publication. Reviewer cross-checks during review.

Per-milestone: framework operators audit external-facing artifacts generated in the period for forbidden-term recurrences.

Tooling: a forbidden-term scanner SHOULD be implemented as a follow-up to this rule, extending the framework's infosec validation surface to detect forbidden patterns in artifacts staged for external destinations. Pre-commit hooks SHOULD refuse the commit when forbidden patterns are present in a path tagged for external destination (e.g. `docs/external-reviews/`, files with a `destination: external` frontmatter). Follow-up tracking issue: [#46](https://gitlab.com/janipaijanen/agent-governance/-/issues/46).

## CUSTOMIZE

Adopters MUST customize:

- The codename (the only handle that crosses the boundary)
- The forbidden-term list per their project's own identity
- The boundary table — adding or removing destinations specific to their infrastructure (additional external systems, bug bounty programs, regulatory-disclosure channels)

<!-- CUSTOMIZE: replace iron-gate with your codename in the example above; replace the schema/path examples with domain-neutral placeholders that match your project. -->

## Strength: MUST or SHOULD?

**MUST** for any adopter whose project name, acronym, or domain descriptor encodes identifying information. **SHOULD** for adopters that already use a fully neutral codename publicly and have no domain-revealing project terms (rare). The customize block above is the documented opt-out path for SHOULD-only adopters.

## Why MUST on single-source evidence

Anonymization leaks are *irreversible* — once a name is published on a third-party tracker, blog, or LLM-provider's training corpus, you cannot unpublish it. The framework's symmetric evidence discipline normally requires 2+ adopter corroboration before promoting a rule to MUST status; irreversibility is the established escape hatch (the same logic that justifies pre-incident security rules over reactive growth-by-pain).

## Related rules

- [`workflow/branch-and-worktree-rule.md`](../workflow/branch-and-worktree-rule.md) — branch discipline; orthogonal to this rule (this one adds the *destination boundary* axis).
- [`workflow/systematic-code-review-rule.md`](../workflow/systematic-code-review-rule.md) — Gate 2 reviewers apply this rule's checks when an MR or issue carries externally-destined artifacts.
- [`always/security-hardening-rule.md`](security-hardening-rule.md) — secrets and environment hardening; complementary (different leak axis).
