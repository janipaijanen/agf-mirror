# Summary

Revises the academic working paper after academic-peer-reviewer findings on
methodology framing, validity boundaries, design-goal structure, and discussion
placement.

This MR updates `docs/academic/governing-the-governors.tex` so the paper is
clearer about its design science status: it is a formative design-case
self-study, not an external effectiveness evaluation.

# Scope

- Adds design science methodology framing in Section 4.
- Adds citations to:
  - Hevner et al. (2004), `Design Science in Information Systems Research`.
  - Wieringa (2014), `Design Science Methodology for Information Systems and
    Software Engineering`.
- Renames research-question framing to Design Goals.
- Reframes the observations section as evaluation against DG1--DG3.
- Labels the self-application evidence as formative evaluation, not summative
  validation.
- States the external-validity boundary: independent adoption, independent role
  holders, and non-successful or partially successful cases are still needed.
- Reduces the abstract's literature list to three anchors.
- Moves "governance as process data" into the abstract.
- Splits artifact-supported contributions from framing contributions.
- Adds role/actor substitution as an explicit design contribution.
- Grounds Table II in the project's implementation-honesty maturity ladder.
- Moves the governed-agent corpus proposal from Discussion to Future Work.
- Adds the Gate 2 handoff record:
  `docs/coordination/01KV7GMHR7XC48HNWEMF577XRS-coder-tester.md`.

# Governance Trail

- Academic peer review:
  `docs/coordination/01KV7EET8T62F2NQ93D9XXY8VW-academic-peer-reviewer.md`.
- Governor action request:
  `docs/coordination/01KV7FN0YJEYR5BSJ5JQNZ5GFA-governor-reviewer.md`.
- Prior snapshot-refresh handoff:
  `docs/coordination/01KV6M8H2DCGECCZ8MNC5ZZJE0-coder-tester.md`.
- This Gate 2 handoff:
  `docs/coordination/01KV7GMHR7XC48HNWEMF577XRS-coder-tester.md`.

# Implementation Notes

The prior snapshot-refresh pass updated currency and evidence alignment, but
did not perform a full academic soundness review. This MR answers that
detection gap in the Gate 2 handoff and recommends that future paper-touching
work either include an academic-content checklist or explicitly invoke
`academic-peer-reviewer` after source-changing edits.

The PDF was not regenerated; this is a source-only revision.

# Verification

- `git diff --check` passed.
- `mix agov.validate` passed.
- `mix test` passed: 197 passed, 0 failures.
- `agov coord validate docs/coordination/01KV7GMHR7XC48HNWEMF577XRS-coder-tester.md`
  passed.

# Review Notes

- Local `pdflatex`, `latexmk`, and `bibtex` were not available.
- The full test run still emits two pre-existing warnings:
  - unused `@tag` before `describe` in `test/agov/coord_config_test.exs`
  - unused default argument in private `claim/3` in
    `test/agov/claim_tracker_test.exs`
- No governance behavior, validators, schemas, or generated figures are
  changed.

# Out of Scope

- Rebuilding or visually reviewing the PDF.
- Redesigning figures or tables.
- Full paper rewrite.
- New validators or workflow changes.
- Version bumps or tags.
