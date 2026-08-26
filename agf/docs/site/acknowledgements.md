# Influences and acknowledgements

AGF combines original project decisions with ideas from organizational
learning, safety engineering, software evolution, and public lessons systems.
Credit here means a source materially shaped a shipped design or its stated
reasoning. It does not mean the source's author or organization endorses AGF.

## How AGF gives credit

- **Material design influence:** credit here and near the affected design.
- **Factual or theoretical support:** cite the source beside the claim.
- **Competitor, sibling, partner candidate, or watch item:** retain it in the
  source registry without implying adoption.
- **Standard or regulation:** cite it formally; do not describe compliance,
  certification, endorsement, or “kudos” unless independently established.

## Material design influences

### NASA Lessons Learned Information System

NASA's [Lessons Learned Information System](https://llis.nasa.gov/),
particularly public [Lesson 30101](https://llis.nasa.gov/lesson/30101),
prompted AGF's review of learning-record structure and lifecycle. AGF then
adopted its own append-safe `supersedes` field on the new record, derives the
reverse relationship instead of editing historical entries, and formalized
its existing optional `proposed | implemented` status convention.

This is design influence, not protocol adoption. AGF did not adopt NASA's
governance body, review process, classification model, assurance protocol, or
stakes. NASA has not endorsed AGF.

### Organizational learning and improvement

- Argyris and Schön's
  [double-loop learning](https://books.google.com/books?vid=ISBN0201001748)
  informs the distinction between correcting work inside existing rules and
  changing the governing assumptions or rules themselves.
- The Deming Institute's [PDSA cycle](https://deming.org/explore/pdsa/)
  informs AGF's insistence that implementation must be followed by study of
  observed use rather than treated as the end of learning.
- Nancy Leveson's [CAST method](https://sunnyday.mit.edu/CAST-Handbook.pdf)
  informs AGF's system-level control-failure lens instead of reducing an
  incident to individual blame or one root cause.

The [learning loop documentation](docs/concepts/evidence-and-learnings.md)
explains these relationships and their evidence limits in context.

### Software evolution and downstream impact

- Lehman's [laws of software evolution](https://doi.org/10.1109/PROC.1980.11805)
  inform the premise that a used framework must evolve while actively
  controlling accumulated complexity.
- E. Burton Swanson's maintenance taxonomy, extended in
  [ISO/IEC 14764:2006](https://www.iso.org/standard/39064.html), informs AGF's
  corrective, adaptive, perfective, and preventive change vocabulary. The
  cited 2006 standard has since been withdrawn and revised; it is credited as
  the historical source used by the shipped rule, not presented as current
  certification guidance.
- David Parnas's
  [information-hiding and modularity criteria](https://doi.org/10.1145/361598.361623)
  inform AGF's distinction between internal/editorial changes and changes to
  an observable behavioral contract.

These sources ground AGF's downstream-impact reasoning. They do not prove
that AGF's specific implementation improves adopter outcomes.

## Standards

[ISO/IEC 42001:2023](https://www.iso.org/standard/42001) is cited as a formal
AI-management-system standard and a useful continual-improvement comparison.
AGF may produce evidence useful within a broader management system; AGF does
not by itself implement ISO/IEC 42001, establish conformity, or confer
certification.

The wider classified source and watch registry is maintained in
[`docs/sources.md`](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/docs/sources.md).
