# AGF Elixir Tooling

`tooling/elixir/` contains the current reference implementation of `agov`
validators and session helpers for AGF-NG.

The tooling is implementation support. It validates and summarizes governance
artifacts, but it is not the source of governance authority. Authority lives in
`agf/` rules and schemas, in role decisions, and in the coordination record.

## What This Tooling Checks

Run commands from this directory unless noted otherwise:

```bash
cd tooling/elixir
```

| Command | Purpose |
|---|---|
| `mix agov.coord.validate ../../docs/coordination/<file>.md` | Validate coordination message front matter |
| `mix agov.plan.validate ../../docs/plans/<file>.md` | Validate Gate 1 plan front matter and advisory consultation fields |
| `mix agov.session.resume --base-branch master` | Cold-start briefing: roles, pending handoffs, project map |
| `mix agov.session.resume --role <role> --base-branch master` | Role-specific resume before acting |
| `mix agov.handoff.next <role>` | Find the next pending handoff for a role |
| `mix agov.coord.visibility --base-branch master` | Detect coordination messages hidden on non-master branches |
| `mix agov.dist.validate <path>` | Validate release distribution contents where available |

## Setup

In this repository, `mise.toml` pins the Elixir/Erlang toolchain:

```bash
mise install
cd tooling/elixir
mix deps.get
mix test
```

If you are using a release archive that contains only `agf/`, this Mix project
is not included. Use a full repository checkout or a future packaged `agov`
tooling distribution when you need validators.

## Authority Boundary

Tool output is evidence. It becomes a blocking gate only when a named AGF rule,
contract, CI policy, or recorded governor decision says that check is blocking.

Examples:

- Coordination schema validation may be blocking for files committed to
  `docs/coordination/` if the project has adopted that gate.
- A branch visibility report is usually advisory until the governor decides how
  to reconcile the hidden coordination messages.
- A session resume summary is a briefing, not permission to proceed.

When adding a new validator, document:

1. the artifact path it checks;
2. the rule, schema, or contract it implements;
3. whether failures are advisory or blocking;
4. at least one valid fixture and one invalid fixture.

## Safety

Run AGF tooling only on repositories and systems where you have authorization.
Do not feed private project names, credentials, customer data, or regulated data
into external AI systems unless your project policy permits it. Local tooling
does not remove the need for human review before commits, merges, releases, or
history-rewriting operations.
