<!-- SPDX-License-Identifier: MIT -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Senior Python Development Guidelines
#
# Example: Python + Flask/Django/FastAPI + SQLAlchemy + PostgreSQL + uv
# Copy and adapt for your project.

## Core Principles
- **Immutability First**: Prefer immutable data structures, dataclasses with frozen=True; avoid in-place mutations
- **Type Safety**: Type hints everywhere, strict type checking
- **Security by Design**: Auth, authorization, input validation, secrets management
- **Data Quality**: Validate at boundaries, Pydantic for data models; immutable models preferred
- **Intention**: Read module docstring before editing any service

## Code Quality Standards
- Use `uv` for dependency management and execution
- Use `typer` for all command line applications
- Follow PEP 8 with Black formatting
- Use Ruff for linting and import sorting
- Implement comprehensive testing with pytest
- Use pre-commit hooks for quality gates

## Local Development Requirements
- **Database**: Use a containerised PostgreSQL instance. The app should refuse to start if `DATABASE_URL` is missing or invalid. All environments use the same DB type.
- **Timestamps**: Always store times in **UTC** (e.g. `DateTime(timezone=True)` in SQLAlchemy, `datetime.now(timezone.utc)` in code). Convert to the user's timezone or locale only when displaying or accepting input.
- **Time format**: By default use **24-hour clock** with **HH:MM** (hours, minutes) for displaying times in the UI. Allow user overrides via settings.
- **Package Management**: Use `uv` (not pip/poetry). `uv sync`, `uv run python script.py`, `uv run pytest`, `uv run black .`
- **CLI Applications**: Use `typer` for CLIs (`typer.Typer()`, `@app.command()`).

## Immutable Patterns
```python
# Preferred: Immutable dataclass
@dataclass(frozen=True)
class User:
    id: str
    name: str
    email: str

# Avoid: Mutable state
class User:
    def __init__(self, name):
        self.name = name  # Mutable!
```

## UI and internationalization
- **Translatable strings**: All user-facing strings must be translatable. Use `gettext`/`_()` in Python and the appropriate template i18n function.
- **No bare `_` throwaway variable**: Do not use `_` as a discard or loop variable — it conflicts with the `gettext` alias `_()`. Use a descriptive name or prefix with an underscore: `_unused`, `_idx`, `_key`, etc.
  - Bad: `for _ in range(n):` or `_, val = pair`
  - Good: `for _i in range(n):` or `_key, val = pair`

## Repository hygiene
- Root clean; tmp under `./tmp/`; tests under `tests/`; docs under `./docs/`.
- **Plan files**: Always write to `./docs/plans/` as `YYYY-MM-DD_HHMMSS-<name>.md`; save before implementation. The timestamp **must** be obtained from the OS at creation time by running `date +%Y-%m-%d_%H%M%S` — never hardcode or invent a timestamp (e.g. `120000`, `000000`).
- **Output**: UTF-8 for all output; `PYTHONIOENCODING=utf-8` when redirecting; commands support `--log-file`.
- Tooling configs at root; exclude transient outputs; large samples in `data/` (git-ignored).
