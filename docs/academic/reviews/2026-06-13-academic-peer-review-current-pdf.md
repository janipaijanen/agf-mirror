# Academic Peer Review: Current IEEE PDF

**Artifact reviewed:** `docs/academic/governing-the-governors.pdf`
**Source consulted:** `docs/academic/governing-the-governors.tex`
**Repository snapshot:** `152ed26`
**Date:** 2026-06-13
**Reviewer role:** `academic-peer-reviewer`
**Review execution:** two delegated AI academic-review passes, synthesized by
Codex without editing the paper content

## Overall Recommendation

Major revision before external academic submission.

The paper has a credible and timely core: self-application can make governance
mechanisms observable, versioned, validated, and stressed by ordinary
development work. The current draft is coherent as an internal working paper or
position/design-case paper. It is not yet strong enough as an evidence-bearing
academic paper because the method, evidence, reproducibility, and closest-work
positioning are underdeveloped.

## Reviewer A Summary

Reviewer A found the paper promising but closer to a polished project rationale
than a fully defensible academic paper. The strongest concern was that Section
IV names an evidence base but does not explain the analysis method: no coding
procedure, selection criteria, failure taxonomy, inter-rater process, audit
method, or analytic protocol.

Reviewer A recommended major revision, with required additions:

- a dedicated research-questions or claims subsection;
- a clearer self-study method;
- a results or observations section with concrete repository evidence;
- contributions separated into demonstrated, conceptual, empirical/self-study,
  and future-research categories;
- tighter related-work gap statement;
- a mechanism-to-enforcement-status table; and
- stronger limitations around circular validation, single-human governance,
  lack of independent adoption, and missing overhead metrics.

## Reviewer B Summary

Reviewer B agreed that the core thesis is credible but judged the evidentiary
apparatus not yet reviewer-ready. The paper asserts that self-application
creates an evidence-rich environment, but does not yet show enough evidence.

Reviewer B recommended major revision, with required additions:

- a reproducibility block with commit hash, artifact inventory, commands,
  validator versions, and data availability;
- a table of concrete self-application episodes;
- closer related-work comparison to governance-as-code, policy-as-code,
  provenance/audit logs, compliance automation, supply-chain attestations,
  model cards/datasheets/system cards, and process mining;
- earlier maturity caveats distinguishing implemented, validated,
  warning-only, proposed, and future capabilities;
- claims tied to observable artifacts and measured examples; and
- explicit evaluation questions or baseline metrics.

## Consolidated Major Findings

### High: Method is under-specified for the evidence claim

Section IV lists artifact counts and describes the repository as a self-study
corpus, but it does not define how artifacts were selected, read, classified,
or evaluated. The phrase "read as a self-study corpus" is too informal for the
paper's evidentiary burden.

Required revision:

- Define corpus boundaries and snapshot.
- Explain artifact categories.
- Define what counted as a failure mode.
- Explain how conclusions were derived.
- State whether the analysis was single-reviewer, multi-reviewer, audited, or
  exploratory.

### High: Evidence is asserted more than demonstrated

The paper names failure modes such as stale handoffs, wrong-branch work,
missing advisory roles, role-status drift, sequence collisions, and unclear
downstream obligations, but does not show concrete cases or outcomes.

Required revision:

- Add a results/observations section.
- Include a table with failure mode, observed artifact, governance response,
  validator/rule response, outcome, and residual limitation.
- Prefer a compact evidence table over one of the current broad process
  diagrams if page budget is tight.

### High: Reproducibility is insufficient

An academic reviewer cannot reproduce the stated counts or validator behavior
from the paper alone.

Required revision:

- Add snapshot commit hash and date.
- Add validation commands and expected outcomes.
- Identify schema paths and tooling versions.
- State what is public, branch-local, generated, or not independently
  reproducible.

### Medium: Related work is broad but not sufficiently gap-driven

The related work covers useful literatures, but it often reads as conceptual
association rather than a focused comparison. The nearest neighbors need to be
identified more directly.

Required revision:

- Add or strengthen comparison to governance-as-code, policy-as-code,
  audit/provenance systems, compliance automation, process mining, software
  supply-chain attestations, model cards, datasheets, and system cards.
- State the specific gap: executable, self-governing process artifacts for
  multi-agent AI development governance.
- Demote or narrow tangential analogies such as federated learning unless the
  paper explains exactly what is and is not borrowed.

### Medium: Contributions mix demonstrated work with agenda-setting

The contribution list currently blends implemented artifact description,
conceptual framing, self-study evidence, and future research agenda.

Required revision:

- Separate demonstrated contributions from proposed or future contributions.
- Avoid implying that conceptual links to quality gates or corpora have been
  empirically evaluated.

### Medium: Maturity and enforcement status should appear earlier

The paper does eventually say that some capabilities are planned,
schema-defined, validated, warning-only, or deployable. Reviewers thought this
qualification should appear before strong validator/enforceability claims.

Required revision:

- Add a mechanism/status table: convention-only, schema-validated,
  validator-enforced, CI-enforced, warning-only, proposed, planned.
- Use this table to prevent overclaiming around deterministic validators.

### Medium: Human authority and AI agency deserve more analysis

The current PDF now clarifies roles, actors, provenance, and the
`governor-reviewer` human role. Reviewers still thought the governance
implications of human final authority versus AI advisory/implementation roles
deserve analysis, not only definition.

Required revision:

- Explain what is gained and what remains risky when a human role retains final
  authority.
- Clarify how mixed human-AI advisory roles should be audited.

## Minor Findings

- The title is rhetorically strong but may sound broader than the evidence if
  submitted externally.
- Figures clarify the process but do not themselves provide evidence.
- The author block is acceptable for an internal working paper but not
  conference-ready.
- Some very recent 2025/2026 bibliography entries should be rechecked before
  external submission.
- Section V-F on downstream impact and exit is one of the strongest and most
  distinctive parts of the paper; consider emphasizing it earlier.
- Future Work and Limitations overlap; Limitations should become sharper and
  less forward-looking.

## Recommended Revision Plan

1. Add research questions or explicit claims near the end of the Introduction.
2. Rewrite Method to define self-study protocol and reproducibility details.
3. Add Results/Observations with concrete self-application episodes.
4. Add an enforcement/maturity table for framework mechanisms.
5. Tighten Related Work around closest comparable systems and concepts.
6. Recast Contributions into demonstrated versus agenda-setting claims.
7. Strengthen human-AI authority analysis and circular-validation limitations.
8. Reduce or clearly subordinate speculative corpus/training claims unless a
   stronger bridge from current evidence is added.

## Disposition

This review does not block the internal branch from continuing, but it should
block any claim that the PDF is externally submission-ready. The recommended
next artifact is a focused revision plan or paper TODO list that converts the
review findings into scoped edits.
