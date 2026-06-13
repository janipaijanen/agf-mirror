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
├── spec/                     # Language-agnostic specifications (schemas, rules, skills)
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
explicitly directed. The two currently permitted exception paths are:
- `agents-govern/agents/MANIFEST.yaml` — agent status changes, governor-reviewer approval required
- `agents-govern/learnings/foundry.yaml` — learning records, append-only

**Role authority note:** `agents-govern/agents/MANIFEST.yaml` is the current
bootstrap authority for active agents and role status. Where the non-legacy role
authority will live in AGF-NG itself (a local manifest, a spec file, or this
README) is an open question tracked in `docs/coordination/00013-planner-architect.md`.
Until that is resolved, MANIFEST.yaml is authoritative.

The product being built is in `spec/` and `tooling/`.

---

## Active agents

| Agent | Played by | Role |
|---|---|---|
| planner-architect | claude (AI) | Gate 1: plans, task decomposition, coordination |
| coder-tester | claude (AI) | Implementation, tests, fixtures, validators |
| governor-reviewer | Jani (human) | Gate 1 approval, Gate 2 merge, all binding decisions |
| contribution-auditor | Jani + claude | Reviews plans and proposals; filed as coordination messages |
| infosec-sentinel | CI (agov-validate-infosec) | Blocks on credential or cross-record leaks in learnings |

Suspended: build-engineer (no CI yet), fact-checker, capability-developer.
Skipped: documentation-writer, deployment-engineer, academic-peer-reviewer.

Authoritative status: `agents-govern/agents/MANIFEST.yaml` (bootstrap, see note above).

---

## Coordination channel

All inter-agent communication goes through immutable Markdown files at
`docs/coordination/<seq>-<sender>.md`. Front matter is validated by
`mix agov.coord.validate`.

**Sequence numbers are global and append-only.** Never reuse or reorder them.
To find the current highest: `ls docs/coordination/*.md | sort | tail -1`.

**Nine message kinds:** `claim`, `completion`, `review`, `ack`, `handoff`,
`decision-record`, `fast-track-waiver`, `lesson-proposal`, `note`.

Schema: `spec/schemas/coordination-message.schema.json`

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
  with YAML front matter conforming to `spec/schemas/gate1-plan.schema.json`.
  The `advisory_consultation` section must declare a status for both
  `infosec-sentinel` and `contribution-auditor`. Plan status must be `approved`
  before implementation begins — do not start coding on a `pending-gate1` plan.
  Validation: `mix agov.plan.validate <file>`.
- **Gate 2** (coder-tester → governor-reviewer): implementation on a feature
  branch, all tests green, governor-reviewer reviews diff and merges.

Gate 1 approval comes from a governor-reviewer coordination message. Gate 2
merge target and remote must be confirmed with the governor-reviewer before
merging — the repository may be local-only or the target branch name may differ.

---

## Tooling quick reference

All Mix commands run from `tooling/elixir/`.

Cold-start rule for humans and AI agents: use one command in two steps. First,
run `mix agov.session.resume` without `--role` to get the project map. Then
rerun the same command with the role you are actually playing.

```bash
# Cold start: discover active roles and pending queues
mix agov.session.resume --base-branch master

# Resume a session before acting or writing durable files
mix agov.session.resume --role <agent-role> --base-branch master

# Validate coordination messages
mix agov.coord.validate ../../docs/coordination/<file>.md

# Validate plan files (Gate 1 front matter + required advisory roles)
mix agov.plan.validate ../../docs/plans/<file>.md

# Find the next handoff for an agent role
mix agov.handoff.next planner-architect

# Run all tests
mix test
```

Schema path resolution uses `AGOV_SCHEMA_DIR` (default: `../../spec/schemas`
relative to cwd — correct when run from `tooling/elixir/`).

Session resume guidance for AI agents and human operators is in
`docs/reference/session-resume.md`.

---

## Current state

For live state, use:

```bash
git log --oneline --decorate -5
ls docs/coordination/ | sort | tail -5
mix agov.session.resume --base-branch master                         # cold start
mix agov.session.resume --role planner-architect --base-branch master  # run from tooling/elixir/
mix agov.handoff.next planner-architect   # run from tooling/elixir/
```

**Snapshot (2026-06-11):** Three tasks merged to master (T1-1 Phase A, T1-1 Phase B,
T1-3). T2-5 (advisory consultation gate) is on `feat/T2-5-advisory-consultation-gate`
awaiting Gate 2. A README cleanup proposal for `docs/coordination/README.md` is
tracked in seq 13 as a separate follow-on.

---

## Branch discipline

- Feature branches: `feat/<task>`, `fix/<what>`, `chore/<what>`, etc.
- Never commit directly to `master`.
- All changes merged after governor-reviewer Gate 2 approval.
- No version bumps or tags on feature branches.

See `agents-govern/` rules: `workflow/branch-and-worktree-rule.md`,
`workflow/merge-request-workflow-rule.md`.
