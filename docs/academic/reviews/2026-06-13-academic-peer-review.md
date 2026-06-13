# Academic Peer Review: TeX Working Paper

**Artifact reviewed:** `docs/academic/governing-the-governors.tex`
**Bibliography reviewed:** `docs/academic/references.bib`
**Date:** 2026-06-13
**Reviewer:** academic-peer-reviewer role, performed by delegated Codex subagent

## Findings

### Medium: method evidence base underspecified

The Method section described "self-application under version control" but did
not define the evidence base, date range, number of artifacts reviewed,
selection criteria, analysis method, or evaluation criteria. This weakened
claims that self-application creates evidence and exposes real failure modes.

Disposition: addressed. The TeX now names the 2026-06-13 repository state and
counts tracked coordination messages, plan files, JSON Schemas, Mix tasks,
Elixir governance modules, and branch-local commits. It also clarifies that the
paper is a self-study corpus, not an independent empirical sample.

### Medium: conclusion overclaimed

The conclusion said "Foundry demonstrates a practical thesis" despite the
limitations section acknowledging single-project and single-governor evidence.

Disposition: addressed. The conclusion now says foundry provides preliminary
evidence, and "canonical traces" has been softened to "candidate canonical
traces."

### Medium: quality-gates citation mismatch

The quality-gates reference had incorrect authors and the paper overgeneralized
the cited work as proving reliability through executability and reproducibility.

Disposition: addressed. The BibTeX entry now uses Uzunova, Pavlic, and Beranic.
The TeX now frames quality gates more narrowly as checkpoints with criteria,
metrics, and supporting tools.

### Medium: agentic governance framing needed tightening

The title and abstract frame the work as multi-agent AI development governance,
but related work leaned heavily on general governance and software-process
literature.

Disposition: partially addressed. The TeX now integrates agentic/runtime
governance works into the related-work section. The scope remains development
governance for AI-agent-assisted work, not runtime governance for deployed AI
systems.

### Low: "immutable" overstated the mechanism

Markdown records in Git are not cryptographically immutable.

Disposition: addressed. The TeX now uses "version-controlled append-only" and
"append-only by project rule" where appropriate.

### Low: federated-learning analogy is loose

The federated-learning analogy may invite pushback because structured learning
records are not decentralized gradient training.

Disposition: addressed. The TeX now explicitly calls this a loose structural
analogy and says stronger privacy-preserving corpus claims require more directly
applicable provenance, privacy, and distributed-evaluation literature.

### Low: bibliography metadata needed verification

Several entries had incorrect or suspect author metadata.

Disposition: addressed for entries currently retained. AGORA, ARGO, AGENTSAFE,
runtime governance, and quality-gates metadata were checked against official or
primary pages and corrected in `references.bib`.

## Remaining Review Risk

- The paper still needs a real LaTeX compile in an environment with
  `apacite`, BibTeX, and a LaTeX engine.
- The method section is now more explicit but remains a self-study design, not
  an empirical evaluation.
- The paper should eventually add a table of artifact counts and selected
  incidents if it is intended for external academic submission.
