<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Migration Generation: Tool-Generated Filenames + Ripple Check
#
# Framework principle: Tool-generated artifacts are the source of truth
# for ordering and naming; downstream artifacts must be checked for
# ripple impact.
# See: framework.md — Pattern 2 (Reviewer Discipline)

> **Provisional status.** Single-adopter (Elixir/Ecto) evidence —
> silver-birch, 2026-04-18, framework v0.33.0 — issue #38.
> The underlying pattern is universal across migration systems
> (Alembic, Rails, Knex, Flyway, etc.) but the only current evidence
> is from one stack. Adopters using a non-Elixir migration system:
> please file a learning record with your stack-specific commands —
> corroboration moves the rule from provisional to recommended.

**Scope:** Every task that creates, modifies, or removes a database
migration file.

## HARD RULE: Never hand-write the migration filename or timestamp

Never construct a migration filename like
`20260425120000_add_foo.exs` (or `.py`, `.rb`, `.sql`, `.ts`) by hand.
Always generate the file first using your stack's designated
generator, then fill in the change.

<!-- CUSTOMIZE: keep only your stack's command -->

    mix ecto.gen.migration add_foo                    # Elixir / Ecto
    alembic revision -m "add foo"                     # Python / SQLAlchemy
    rails generate migration AddFoo                   # Ruby / Rails
    knex migrate:make add_foo                         # Node / Knex
    flyway migrate                                    # Java / Flyway
    sqlx migrate add foo                              # Rust / SQLx

Then edit the generated file. **Do NOT rename it or modify the
timestamp prefix.**

A hand-picked timestamp prefix silently collides if it already
exists, producing a duplicate migration or a broken migration chain
detected only when another developer pulls and runs the migrator.

## Verify the chain after every migration

After creating or modifying any migration, confirm the chain is
linear:

<!-- CUSTOMIZE: keep only your stack's command -->

    mix ecto.migrations          # Elixir / Ecto
    alembic history              # Python / SQLAlchemy
    rails db:migrate:status      # Ruby / Rails
    knex migrate:list            # Node / Knex

Then run the migrator against a fresh database to verify it executes
cleanly:

<!-- CUSTOMIZE: keep only your stack's command -->

    mix ecto.migrate             # Elixir / Ecto
    alembic upgrade head         # Python / SQLAlchemy
    rails db:migrate             # Ruby / Rails
    knex migrate:latest          # Node / Knex

## Ripple check — all downstream artifacts

A migration that adds, removes, renames, or re-types a column may
break any code that creates or reads rows in the affected table.
After every migration, review in this order:

1. **Schema / model definitions** — does the schema/model file match
   the migration (column names, types, nullability, defaults)?
2. **Validation / changeset code** — does the validation include any
   new required fields? Does it reflect new NOT NULL columns?
3. **Test factories / fixtures** — do they create records with valid
   data for the new schema?
4. **Seed scripts** — review for any constructor calls touching the
   affected table.
5. **Integration tests** — run the integration suite to confirm zero
   failures from schema drift.

The order matters: each step depends on the previous being correct.
A broken schema/model definition silently breaks everything below it.

## Common pitfalls

- **NOT NULL without default.** Existing inserts will fail until the
  factory, seed, and validation code is updated to provide the
  required value.
- **Column rename.** Any model field declaration using the old name
  will error on next compile/load.
- **Type change.** Foreign-key references across tables must be
  converted together — partial type changes break joins silently.
- **Auto-populated timestamps.** Ensure the model's create/update
  logic sets the new column if the migration adds one without a
  database default.
- **Generated migrations not committed.** Migration files generated
  locally must be committed alongside the change. A forgotten commit
  leaves teammates with broken databases on next pull.

## Why this exists

Hand-written timestamps create chain-integrity problems that are
expensive to debug: collisions, out-of-order migrations, silent
duplicates. Migrating without ripple-checking leaves downstream
artifacts (factories, seeds, tests) in a broken state that's
invisible until someone tries to use them — and a missed factory
update means new tests fail in ways that look unrelated to the
migration.

Generated timestamps are deterministic and unique. Tool-driven
generation also guarantees the file lives in the canonical
directory with the expected naming convention, which downstream
tools (CI, schema-dump comparisons, `validate_adoption` analogs)
depend on.

silver-birch (2026-03-21..22) accumulated 9 schema-migration parity
incidents in a 48-hour window. After adopting (a) tool-generated
filenames and (b) the 5-step ripple check, plus a CI job that
compares schema fields against actual database columns, the gap
cluster did not recur.

## Verify after every migration — minimum sequence

    <stack generator>      # 1. Generate the file
    # Edit the generated change/up/down — do NOT rename
    <stack migrator>       # 2. Run against dev DB
    <stack status>         # 3. Verify chain is linear
    <stack test runner>    # 4. Run integration tests

## CUSTOMIZE

Adopters MUST customize:

- **Stack-specific generator command** (`mix ecto.gen.migration`,
  `alembic revision`, etc.).
- **Stack-specific verification commands** (status, migrate, test).
- **Exact ripple-check steps** for your stack — location of factory
  definitions, seed scripts, validation code differs by framework.

Adopters MAY add:

- Additional ripple-check steps (e.g., GraphQL schema regeneration,
  ORM cache invalidation, OpenAPI doc regeneration, search-index
  rebuild).
- Stack-specific common pitfalls (Django `makemigrations` vs.
  `migrate`; Rails `schema.rb` vs. `structure.sql`; Phoenix
  `Ecto.Changeset` vs. raw `Ecto.Migration`).

## Related rules

- (Project-specific) `domain/<stack>-migration-rule.md` — adopters
  may keep stack-specific details (commands, paths, project-internal
  ripple targets) in a project-domain rule. This generic rule is the
  invariant; the project-specific one is the implementation.
- `workflow/tdd-rule.md` — schema changes are not exempt from TDD;
  the ripple-check integration tests are part of "the test exists
  before the migration is finalised".
- `workflow/destructive-op-preflight-rule.md` — column-drop and
  table-drop migrations are destructive and on the rule's red list.
