---
kind: handoff
from: governor-reviewer
to: coder-tester
id: 20260724T205538Z-governor-reviewer-mpea
timestamp: 2026-07-24T20:55:38Z
model: human-governor
immutable: true
references-to:
  - 20260724T174750Z-governor-reviewer-j55n
  - 20260724T174733Z-contribution-auditor-pt3h
topic: v242-implementation-authorized
requires-ack: true
artifacts_owned:
  - docs/plans/v2.4.2-public-experience-and-evidence-presentation.md
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Implement the five named commits in the approved v2.4.2 plan, in order, on one feature branch from current main
    - Produce every strict-build, link, rendered narrow/desktop, accessibility-oriented, and no-scope-creep acceptance artifact named by the plan
    - Obtain mandatory exact-text fact-checker disposition before submitting the implementation for Gate 2
  must_read:
    - docs/plans/v2.4.2-public-experience-and-evidence-presentation.md
    - docs/coordination/20260724T174750Z-governor-reviewer-j55n.md
    - docs/coordination/20260724T174733Z-contribution-auditor-pt3h.md
  do_not_touch:
    - agf/VERSION
    - agf/CHANGELOG.md
    - agf/agents/MANIFEST.yaml
    - agf/agents/release-engineer.yaml
    - .gitlab-ci.yml
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Implement approved v2.4.2 public-experience work

MR !476 is merged at `56908f81d3d61f6e6d9a6b9026bc39daa6d7c3c8`.
The plan on `main` has `status: approved`. Begin implementation from current
`main`, not from the deleted planning branch.

Use one feature MR with the approved five independently reviewable commits:

1. four factual/trust BLOCK fixes;
2. homepage `agov` table CONDENSE and canonical tooling-table treatment;
3. dossier reference layer absorbing MR !400;
4. prospective assistant-attribution byline documentation and acceptance
   examples;
5. six-file plain-language progressive-disclosure pass.

Stay within the nine frozen scope items. If a change needs a schema, role,
tool/API, CI, or adopter-contract modification, stop and split it into a new
issue/plan rather than expanding this MR.

Before Gate 2, route the exact resulting public text to fact-checker. The
review must verify feature, maturity, evidence, cost, privacy, enforcement,
data-boundary, command-inventory, dossier-link, and homepage outcome claims.
Apply any accepted wording-level correction before the Gate 2 handoff; re-plan
any non-wording correction.

Gate 2 merge, version bump, release branch, tag, publication, and release remain
unauthorized. The release-engineer proposal in MR !485 is a separate track and
must not be used or folded into this implementation.
