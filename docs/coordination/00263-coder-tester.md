---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 263
timestamp: 2026-06-14T00:40:00+03:00
immutable: true
references-to:
  - docs/plans/downstream-ux-golden-path.md
topic: downstream-ux-golden-path-poc-gate2
requires-ack: true
artifacts_owned:
  - README.md
  - agf/ADOPTING.md
  - agf/templates/
  - agf/docs/site/docs/adopt/install-and-layouts.md
  - agf/docs/site/docs/start/first-governed-change.md
  - agf/docs/reference/quick-reference.md
  - agf/docs/site/docs/advanced/contracts-and-validators.md
  - agf/docs/site/docs/adopt/downstream-impact.md
  - agf/docs/site/docs/adopt/exit-and-decommission.md
  - agf/docs/governance/manifest.md
  - docs/plans/downstream-ux-golden-path.md
model: codex
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review branch `feat/downstream-ux-golden-path-poc` for Gate 2.
    - Decide whether the downstream UX golden-path POC is acceptable.
    - Confirm whether the root README and starter templates should be considered part of the provisional AGF v2 distribution.
  must_read:
    - README.md
    - docs/plans/downstream-ux-golden-path.md
    - agf/templates/README.md
    - agf/docs/site/docs/adopt/install-and-layouts.md
    - agf/docs/site/docs/start/first-governed-change.md
    - agf/docs/site/docs/advanced/contracts-and-validators.md
  do_not_touch:
    - agents-govern/
    - agents-govern/agents/MANIFEST.yaml
    - agents-govern/learnings/foundry.yaml
---

# Gate 2 Handoff: downstream UX golden-path POC

Branch: `feat/downstream-ux-golden-path-poc`

Summary:

- Adds a root `README.md` for evaluator orientation: what AGF is, whether to
  try it, repo map, five-minute path, and trial artifacts.
- Adds `agf/templates/` with copyable first-run files:
  - Gate 1 plan;
  - Gate 1 handoff;
  - Gate 1 decision record;
  - Gate 2 handoff;
  - completion;
  - learning record;
  - private codename mapping example.
- Rewrites the first governed change walkthrough around those schema-valid
  templates and exact validation commands.
- Clarifies the POC layout:
  - `agf/` for framework-owned distribution files;
  - `.claude/rules/` or equivalent for assistant-loaded rules;
  - `docs/plans/` for working plans;
  - `docs/coordination/` for coordination;
  - `agf/learnings/<codename>.yaml` for adopter learning records.
- Clarifies validator packaging: current Mix tasks live in `tooling/elixir/`
  and require a full checkout until a standalone tooling distribution exists.
- Adds a minimum role commissioning matrix and keeps gateway/runtime identities
  separate from AGF approval authority.
- Cross-links exit/decommission and keeps normal exit history-preserving.

Downstream impact:

- Direct adopters get additive/preventive docs and templates.
- Existing provisional adopters using `agf/docs/coordination/` as their working
  channel should migrate or explicitly configure that non-default path before
  agents write more records.
- No schema or runtime behavior changed.

Validation:

```bash
cd tooling/elixir
mise exec -- mix agov.plan.validate ../../docs/plans/downstream-ux-golden-path.md
mise exec -- mix agov.plan.validate ../../agf/templates/gate1-plan.md
mise exec -- mix agov.coord.validate \
  ../../agf/templates/coord-handoff.md \
  ../../agf/templates/coord-decision-record.md \
  ../../agf/templates/coord-gate2-handoff.md \
  ../../agf/templates/coord-completion.md
mise exec -- mix compile --warnings-as-errors
mise exec -- mix test
cd ../..
mise exec -- uvx --from 'mkdocs>=1.6,<2' mkdocs build -f agf/docs/build/mkdocs.yml --strict
git diff --check
```

Results: OK. Full test suite on this branch passed with 129 tests. MkDocs
strict build passed.

Stale-reference scan:

```bash
rg -n "docs/example-workflow|learnings/_template|learnings/\\.adopter|governance/agents" \
  README.md agf/ADOPTING.md agf/docs/site agf/docs/reference agf/docs/governance/manifest.md agf/templates
```

Result: no matches. The remaining `agf/docs/coordination` mention in
`agf/ADOPTING.md` is intentional as a documented failure-mode example.
