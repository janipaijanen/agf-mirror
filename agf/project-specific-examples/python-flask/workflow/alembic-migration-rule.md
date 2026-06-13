<!-- SPDX-License-Identifier: MIT -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Alembic Migrations: Safe Authoring
#
# Example: Python + SQLAlchemy + Alembic
# Copy and adapt for your project.

## HARD RULE: Never invent or hand-write a revision ID

**This rule applies to every task, not just when a migration file is open.**

The agent MUST NOT write a `revision: str = "abc123..."` value by hand -- ever.
This has caused real CI failures (duplicate IDs, broken migration chains, table-not-found errors).

**Always** generate the revision file first, then fill in the SQL:

```bash
# Step 1: let Alembic generate the file and assign a unique ID
uv run alembic revision -m "describe_the_change"

# Step 2: edit the generated file in migrations/versions/<generated_id>_describe_the_change.py
# Fill in upgrade() and downgrade() -- do NOT touch the revision: str = "..." line
```

The generated file has the correct `revision`, `down_revision`, and chained parent automatically.
A hand-picked ID silently collides if it already exists, causing a cycle or missing-table error in CI.

## Verify the chain is linear after every migration change

After creating or modifying any migration file, confirm exactly **one head**:

```bash
uv run alembic heads
```

Expected output: a single revision hash. If multiple heads appear, create a merge migration:

```bash
uv run alembic merge heads -m "merge_heads"
```

Then re-confirm `uv run alembic heads` returns exactly one line.

## Workflow for adding a new migration (required steps in order)

1. `uv run alembic revision -m "short_snake_case_description"` -- generates the file
2. Edit `upgrade()` and `downgrade()` in the generated file
3. `uv run alembic heads` -- confirm one head
4. `uv run alembic upgrade head` -- verify migration runs without error against a live DB
5. Commit both the migration file and any model changes together

## Checklist for every new migration

### Schema correctness
- [ ] File created by `alembic revision`, not written from scratch
- [ ] `revision:` and `down_revision:` values untouched from what Alembic generated
- [ ] `upgrade()` and `downgrade()` both implemented
- [ ] `uv run alembic heads` shows exactly one head
- [ ] Migration runs cleanly against a fresh DB (`uv run alembic upgrade head`)
- [ ] Model and migration are consistent (same columns, types, nullability)

### Ripple check -- all code that creates rows in affected tables

A migration that adds, removes, renames, or re-types a column may break any code
that creates rows in the affected table. After every migration, review **in this order**:

1. **Model** -- Does the SQLAlchemy model match the migration? (columns, types, nullable, defaults)
2. **Seed scripts** -- Review every script and fixture that creates rows in affected tables.
   Grep for the table name or model class in `scripts/` and fixture directories and verify each constructor call.
3. **Test fixtures** -- Do `tests/conftest.py`, `tests/*/conftest.py`, and factory helpers
   create records with valid data for the new schema?
4. **Integration tests** -- `uv run pytest tests/ -v --tb=short`

### Common pitfalls

- **Constraint tightening** (nullable -> NOT NULL, new FK): seed scripts that pass NULL or
  missing FK values will fail silently in CI.
- **Column rename or removal**: any seed script referencing the old column name will crash.
- **Type change** (e.g. Integer -> UUID): FK references across tables must all be converted
  together; seed scripts must use the new type for IDs.
- **Nested app contexts in fixtures**: ORM objects created inside an inner `with app.app_context()`
  become detached when the inner context exits. Return IDs (not ORM objects) across context
  boundaries to avoid `DetachedInstanceError` or FK violations.
