<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Architecture Decision Record (ADR): Framework updates from iron-gate adopter feedback

## Status

Proposed

## Context

iron-gate (scenario definition language for virtual cybersecurity exercise environments) is the first project to adopt agents-govern (v0.13.0). Over three days of adoption (2026-04-02 to 2026-04-05), the project generated 16 learnings — 8 of which have `scope: framework` or carry recommendations for framework changes.

The learnings break down by severity:

- **Critical (2):** Security testing exemption gap; legal classification enum anti-pattern
- **Significant (4):** Definition-vs-runtime boundary; Request For Comments (RFC) 2119 adoption; naming policies at adoption time; generic naming policy concept
- **Minor (2):** Tiered artifact governance weights; Debian-style dependency pattern

These represent real friction encountered by a real adopter. Ignoring them risks repeating the same friction for every future adopter.

## Decision

Incorporate iron-gate feedback into the framework in three tiers:

### Tier 1 — Framework additions (critical)

1. **Security testing exemption concept.** Add guidance for projects that intentionally ship vulnerable code (exercise platforms, Capture The Flag (CTF) challenges, security training tools). The framework should provide a mechanism for declaring which artifacts are intentionally vulnerable and should be excluded from automated security gates while keeping the rest fully scanned. Target location: Security Guardian agent definition and ADOPTING.md Step 0.3.

2. **Legal classification enum anti-pattern.** Add a boundary principle: schemas MUST NOT enumerate real legal classification schemes (EU RESTREINT UE, Finnish ST levels, German VS-NfD) in closed enum types. Enumerating them implies compliance obligations the framework cannot enforce. Use free-form strings with a `classificationScheme` identifier instead. Target location: framework.md Section 3 (Boundary Principles).

### Tier 2 — ADOPTING.md improvements (significant)

3. **Definition-vs-runtime boundary checklist.** Add a checklist for declarative / Security Development Lifecycle (SDL) projects: "If a field requires a live engine to evaluate, it belongs outside the schema." Target location: ADOPTING.md as a recommended assessment for declarative projects.

4. **RFC 2119 keyword recommendation.** Recommend that governance documents adopt RFC 2119 keywords (MUST, SHOULD, MAY) with mapping to enforcement severity (BLOCK, WARN). Target location: ADOPTING.md best practices.

5. **Naming policies at adoption time.** Recommend establishing naming conventions (especially for fictional entities) before content creation begins. Retrofitting is always more expensive. Target location: ADOPTING.md Step 0.

6. **Generic naming policy concept.** Add a "naming policy" concept to agent definitions for projects that model real-world entities with fictional replacements. Target location: framework.md Section 5 (Agent Definition Structure).

### Tier 3 — Documentation patterns (minor)

7. **Tiered artifact governance weights.** Document the pattern where artifacts have different governance weights based on reuse scope (framework-wide vs. project-specific vs. ephemeral). Target location: framework.md Section 2.

8. **Debian-style dependency pattern.** Document provides/depends/recommends/suggests/conflicts/tags as a recommended pattern for composable artifact registries. Target location: project-specific-examples or a new patterns section.

## Alternatives considered

**A. Incorporate all feedback immediately in a single framework release.**
- Pro: All improvements land at once.
- Con: Large diff, higher review burden, risk of introducing inconsistencies across sections.

**B. Incorporate only critical items, defer the rest (chosen).**
- Pro: Focuses effort on the highest-impact gaps. Significant and minor items can land in subsequent releases with more deliberation.
- Con: Significant items (RFC 2119, naming policies) could prevent friction for near-term adopters if landed sooner.

**C. Treat all feedback as project-specific, make no framework changes.**
- Pro: Zero framework churn.
- Con: Every future adopter rediscovers the same gaps. The learnings mechanism exists precisely to avoid this.

## Counter-arguments sought

- **iron-gate project (adopter perspective):** Confirmed that the security testing exemption and legal classification gaps caused real decision friction, not just theoretical concern. The fictional naming policy migration (12 files, 2 scenarios) was concrete evidence of the "establish policies early" recommendation.
- **Framework maintainer perspective:** The legal classification anti-pattern and security exemption are genuinely outside the current framework's scope — they are not covered by existing principles. The RFC 2119 and naming policy recommendations are lower-risk additions to ADOPTING.md that don't change core framework semantics.

## Strongest objection

The framework is at v0.13.0 with exactly one adopter. Making structural additions based on a single data point risks overfitting to iron-gate's specific domain (cybersecurity exercises). The security testing exemption, in particular, may be too niche for a general-purpose governance framework — most projects don't intentionally ship vulnerable code.

**Rebuttal:** The security exemption is niche but the *pattern* (declaring exceptions to automated gates with explicit justification) is general. The legal classification anti-pattern applies to any project modeling regulated data. One well-documented adopter experience is more valuable than zero — and the learnings mechanism exists to accumulate evidence across adopters.

## Consequences

**Positive:**
- Future adopters of declarative/SDL projects benefit from boundary checklists and naming policy guidance without rediscovering these patterns.
- The security testing exemption opens the framework to an underserved project category (security training, CTF, exercise platforms).
- The legal classification guidance prevents a class of compliance-scope errors.

**Negative:**
- Framework surface area increases. Each addition must be maintained.
- Risk of over-prescribing patterns that may not generalize beyond iron-gate.

## Confidence and reevaluation triggers

**Confidence: Medium-high** for critical items (real friction, clear gap), **Medium** for significant items (reasonable recommendations, less urgency), **Low-medium** for minor items (useful patterns, could remain as project-specific examples).

**Reevaluation triggers:**
- A second adopter reports that the security testing exemption is irrelevant or harmful to their workflow.
- A second adopter in a non-regulated domain finds the legal classification guidance confusing or inapplicable.
- The tiered artifact pattern proves too abstract to be actionable without a concrete example beyond iron-gate.
