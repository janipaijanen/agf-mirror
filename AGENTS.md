# AGF-NG — Agent and Contributor Orientation

**Project codename:** foundry
**What this is:** The next-generation agents-govern framework, developed using
agents-govern 1.1.50 as its own governance framework. The project is both the
product under development and a live proof-of-concept of the governance model
it will supersede.

---

## Project layout

```
AGF-NG/
├── agf/                      # Canonical AGF v2.0+ adopter-facing framework
│   ├── agents/               # Agent role definitions shipped to adopters
│   ├── rules/                # Governance rule templates
│   └── schemas/              # JSON Schema draft-07 definitions
├── tooling/
│   └── elixir/               # Elixir Mix project: agov 0.1.0 (mix tasks + validators)
├── docs/
│   ├── coordination/         # Immutable coordination channel (append-only)
│   ├── plans/                # Gate 1 plan files
│   └── proposals/            # Longer-form proposals
├── agents-govern/            # Bootstrap governance reference — see below
└── AGENTS.md                 # This file
```

---

## agents-govern/ is a bootstrap reference, not the product

`agents-govern/` contains the verbatim agents-govern 1.1.50 distribution used
to govern this project's own development (Layout B). It is **not** the product
being built — it is the governance tool currently in use.

Treat it as legacy/bootstrap: avoid making product changes there unless
explicitly directed. The currently permitted exception path is:
- `agents-govern/learnings/foundry.yaml` — learning records, append-only

**Role authority note:** `agf/agents/MANIFEST.yaml` is the current AGF-NG
project role authority for active agents and role status. Agent status changes
require governor-reviewer approval. `agents-govern/agents/MANIFEST.yaml`
remains historical bootstrap reference material only.

The adopter-facing product is in `agf/`. Foundry tooling sources are in
`tooling/`.

---

## Active agents

| Agent | Played by | Role |
|---|---|---|
| planner-architect | claude (AI) | Gate 1: plans, task decomposition, coordination |
| coder-tester | claude (AI) | Implementation, tests, fixtures, validators |
| governor-reviewer | Jani (human) | Gate 1 approval, Gate 2 merge, all binding decisions |
| contribution-auditor | Jani + claude | Reviews plans and proposals; filed as coordination messages |
| infosec-sentinel | CI (agov-validate-infosec) | Mechanically blocks configured checks on credential or cross-record leaks in learnings; governor-reviewer resolves the finding |
| fact-checker | Jani + AI advisory | Factual/docs claim review when activated by governor-reviewer |
| academic-peer-reviewer | Jani + AI advisory | Academic/manual review for paper-style claims and definitions |

Suspended: build-engineer (no CI pipeline yet), capability-developer.
Skipped: documentation-writer, deployment-engineer, academic-visual-reviewer.

Authoritative status: `agf/agents/MANIFEST.yaml`.

Tooling and CI may block progress mechanically when configured to do so, but
they do not approve or reject governance gates. Binding Gate 1 and Gate 2
decisions remain governor-reviewer decisions recorded in coordination.

**Automation rule:** A tool or CI check may only block work when a named rule
in `agf/rules/` explicitly grants that check blocking authority. All other
tool output is advisory evidence. See `agf/rules/always/tool-authority-boundary-rule.md`.

---

## Coordination channel

All inter-agent communication goes through immutable Markdown files at
`docs/coordination/<seq>-<sender>.md`. Front matter is validated by
`agov coord validate`.

**Sequence numbers are global and append-only.** Never reuse or reorder them.
To find the current highest: `ls docs/coordination/*.md | sort | tail -1`.

**Nine message kinds:** `claim`, `completion`, `review`, `ack`, `handoff`,
`decision-record`, `fast-track-waiver`, `lesson-proposal`, `note`.

Schema: `agf/schemas/coordination-message.schema.json`

**When to write a coordination entry:** Any action that creates a downstream
obligation for another agent or human requires a coordination entry as part
of completing that action — not as an afterthought. Write one when:

- A plan or design is submitted for Gate 1 approval (kind: `handoff`, to: `governor-reviewer`)
- An implementation is ready for Gate 2 review (kind: `handoff`, to: `governor-reviewer`)
- A decision has been made that others need to know about (kind: `decision-record`)
- A finding, alert, or escalation is routed to another agent (kind: `review`, `note`, or `handoff`)
- A task is completed and the result needs to be recorded (kind: `completion`)

The coordination channel is the audit trail for both decisions and the work
that led to them. Other agents and the human read it asynchronously — write
entries so that a cold-starting agent can reconstruct what happened and why,
without relying on session memory.

---

## Gate discipline

Every non-trivial task requires:

- **Gate 1** (planner-architect → governor-reviewer): plan file in `docs/plans/`
  with YAML front matter conforming to `agf/schemas/gate1-plan.schema.json`.
  The `advisory_consultation` section must declare a status for both
  `infosec-sentinel` and `contribution-auditor`. Plan status must be `approved`
  before implementation begins — do not start coding on a `pending-gate1` plan.
  Validation: `agov plan validate <file>`.
- **Gate 2** (coder-tester → governor-reviewer): implementation on a feature
  branch, all tests green, governor-reviewer reviews diff and merges.

Gate 1 approval comes from a governor-reviewer coordination message. Gate 2
merge target and remote must be confirmed with the governor-reviewer before
merging — the repository may be local-only or the target branch name may differ.

---

## Tooling quick reference

Use the user-local `agov` command for core plan and coordination validation.
It points at this repository's `tooling/agov` executable.

Mix remains the Elixir reference/development path for commands that are not yet
wrapped by `agov`, such as session resume, handoff lookup, branch visibility,
distribution checks, token ROI, and the test suite.

Cold-start rule for humans and AI agents: use one command in two steps. First,
run `mix agov.session.resume` without `--role` to get the project map. Then
rerun the same command with the role you are actually playing.

```bash
# Validate coordination messages
agov coord validate docs/coordination/<file>.md

# Validate plan files (Gate 1 front matter + required advisory roles)
agov plan validate docs/plans/<file>.md

# Dotted compatibility aliases also work
agov agov.coord.validate docs/coordination/<file>.md
agov agov.plan.validate docs/plans/<file>.md

# Elixir reference/development commands run from tooling/elixir/
cd tooling/elixir

# Cold start: discover active roles and pending queues
mix agov.session.resume --base-branch master

# Resume a session before acting or writing durable files
mix agov.session.resume --role <agent-role> --base-branch master

# Find the next handoff for an agent role
mix agov.handoff.next planner-architect

# Scan local branches for coordination obligations hidden from master
mix agov.coord.visibility --base-branch master

# Report local deterministic-tool token ROI telemetry
mix agov.token.roi report

# Run all tests
mix test
```

Schema path resolution uses `AGOV_SCHEMA_DIR` (default: `../../agf/schemas`
relative to cwd for Mix commands run from `tooling/elixir/`). AGF-controlled
environment variables are documented together in
`agf/docs/site/reference/environment-variables.md`.

Session resume guidance for AI agents and human operators is in
`docs/reference/session-resume.md`. Local token ROI measurement guidance is in
`docs/reference/tooling-token-roi.md`.

---

## Current state

For live state, use:

```bash
git log --oneline --decorate -5
ls docs/coordination/ | sort | tail -5
agov coord validate docs/coordination/<file>.md
agov plan validate docs/plans/<file>.md
mix agov.session.resume --base-branch master                         # cold start
mix agov.session.resume --role planner-architect --base-branch master  # run from tooling/elixir/
mix agov.handoff.next planner-architect   # run from tooling/elixir/
mix agov.coord.visibility --base-branch master
```

**Snapshot (2026-06-14):** `agf/agents/MANIFEST.yaml` is the AGF-NG role
authority. GitLab upstream is available at
`https://gitlab.com/janipaijanen/agents-govern-framework.git`; current feature
work is tracked on branches and coordinated through `docs/coordination/`.

---

## Branch discipline

- Feature branches: `feat/<task>`, `fix/<what>`, `chore/<what>`, etc.
- Never commit directly to `master`.
- All changes merged after governor-reviewer Gate 2 approval.
- No version bumps or tags on feature branches.

**Worktree boundary:** All linked worktrees must be created under
`.worktrees/<purpose>` within the project root. Never create worktrees in
sibling directories or `/tmp` unless the governor-reviewer explicitly approves
an external path.

See `agents-govern/` rules: `workflow/branch-and-worktree-rule.md`,
`workflow/merge-request-workflow-rule.md`.
