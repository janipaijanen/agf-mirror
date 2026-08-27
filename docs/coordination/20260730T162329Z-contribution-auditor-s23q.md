---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260730T162329Z-contribution-auditor-s23q
timestamp: 2026-07-30T16:23:29Z
immutable: true
references-to:
  - 20260730T160903Z-governor-reviewer-gn47
  - 20260730T153314Z-contribution-auditor-r61q
topic: v246-release-final-live-version-surface
requires-ack: true
artifacts_owned:
  - docs/plans/v246-release.md
---

# v2.4.6 release validation found four omitted live version surfaces

# Review evidence envelope

Scope: `docs/plans/v246-release.md` at
`2db5dc31a03e50dc90e408746d4827082e0a64d5`, plus a complete semver sweep of
the release working tree
Coverage: COMPLETE
Recommendation: CLEAR the narrowly expanded 26-file amendment; do not commit
or push the release implementation until the governor records Gate 1 at this
exact plan head
Blocking findings: 0 after the plan correction
Advisory findings: 0
Open findings with accepted risk: 0
Authority: recommendation only; the governor-reviewer decides Gate 1

## Finding and correction

The mandatory Gate 2 semver sweep found four current, copyable surfaces still
pinned to v2.4.4 despite describing `agf/VERSION` as canonical or the v2.4.4
path as current:

- `README.md` — one OCI image pin;
- `agf/docs/site/docs/adopt/install-and-layouts.md` — two OCI command pins;
- `agf/docs/site/docs/adopt/tooling-and-tokens.md` — current operator-path and
  platform baselines, one OCI pin, and one checkout example; and
- `agf/docs/site/docs/advanced/contracts-and-validators.md` — two OCI pins.

These cannot honestly be classified as historical changelog/learning
provenance, compatibility/schema/contract versions, agent-definition versions,
external dependency/tool versions, or synthetic fixtures. Three of the four
files are shipped in both framework archives; the repository README is the
public evaluator/adopter entry point. The corrected plan expands the release
diff from 22 to exactly 26 files and changes only those live v2.4.4 references
to v2.4.6.

The full sweep otherwise classifies older values into the approved categories:
historical plans, coordination, changelogs, learnings, ADRs and retrospectives;
bootstrap `agents-govern/` provenance; contract/schema introduction versions;
agent definition IDs; external Cargo/mise/tool versions; Graphviz generator
versions; and deliberate test/example fixtures. The root changelog's historical
entries are repository history and explicitly defer adopter-facing release
truth to `agf/CHANGELOG.md`; they are not a package-release scalar.

`agov plan validate docs/plans/v246-release.md`, `agov validate infosec`, and
`git diff --check` pass. No release commit, branch push, merge request, or tag
has been created.
