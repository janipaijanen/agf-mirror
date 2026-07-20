# docs: relabel AI-operator audience lines (MR-2)

Branch: `docs/audience-operator-relabeling-mr2`

## Status

- **Pending:** Gate 2 review by governor-reviewer
  (`docs/coordination/20260720T151740Z-coder-tester-j1d2.md`).
- **Blocking Claude:** nothing. MR-2 is Codex-owned and does not touch the
  mixed pages handled by Claude/MR-3 or Codex/MR-4.

## Summary

MR-2 of 6 from the approved documentation audience-boundary plan
(`docs/plans/docs-audience-boundary-proposal.md`, Gate 1 approved in
`20260720T143255Z-governor-reviewer-s6hg.md`).

Updates six AI-operator or AI/operator-reference pages so their visible
`Audience:` lines name AI assistants as the executor and humans as supervisors,
reviewers, or policy owners. Also adds the approved invisible
`<!-- audience: ... -->` source markers for grep-based review.

## Files changed

| File | Change |
|---|---|
| `agf/docs/site/docs/adopt/assistant-loader-configuration.md` | Audience now names AI assistants preparing AGF rule loading, with developers/administrators supervising |
| `agf/docs/site/docs/adopt/upgrade-from-v1.md` | Audience now names AI assistants carrying out migration under adopter review |
| `agf/docs/site/docs/adopt/tooling-and-tokens.md` | Audience now names AI assistants adding deterministic tooling, with humans supervising setup/evidence |
| `agf/docs/site/docs/adopt/mcp-server.md` | Audience now names AI assistants and adopters wiring MCP access to AGF tooling |
| `agf/docs/site/docs/operate/run-agf-in-real-situations.md` | Audience now names AI assistants operating AGF, with governors/developers supervising decisions and evidence |
| `agf/docs/site/docs/advanced/contracts-and-validators.md` | Audience now names AI assistants wiring validators into CI, with administrators/developers reviewing contracts and blocking policy |
| `docs/coordination/20260720T151536Z-coder-tester-appt.md` | MR-2 claim |
| `docs/coordination/20260720T151740Z-coder-tester-j1d2.md` | Gate 2 handoff |

## Scope boundary

- Does not edit `first-governed-change.md`; MR-3 handled that page with
  per-step human-vs-assistant markers.
- Does not edit `install-and-layouts.md`; MR-4 split that page's human layout
  decision from assistant commissioning steps.
- Does not perform broader prose cleanup or navigation changes.

## Review checklist

- [x] `agov coord validate docs/coordination/20260720T151536Z-coder-tester-appt.md`
- [x] `git diff --check`
- [x] Grep check confirms all touched pages carry `<!-- audience: ... -->`
      markers and visible `Audience: AI assistants...` wording
- [x] `mkdocs build -f agf/docs/build/mkdocs.yml --strict` passes via `uvx`
      with the CI package set
- [x] No `Co-authored-by` trailers in commit
