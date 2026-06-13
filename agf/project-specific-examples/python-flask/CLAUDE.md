<!-- SPDX-License-Identifier: MIT -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Example CLAUDE.md for Python + Flask/Django/FastAPI projects
#
# Copy this to your project's `.claude/CLAUDE.md` and adapt.
# See: agents-govern framework — ADOPTING.md, rules/

Read all files under `.claude/rules/` before making changes.

## Rule groups

| Directory | Scope |
|-----------|-------|
| `rules/always/` | Core constraints -- apply to every change |
| `rules/workflow/` | Process rules -- commits, migrations, testing, releases, TDD, debugging, code review |
| `rules/ui/` | Templates, CSS, accessibility, translations |
| `rules/docs/` | Documentation audience, content rules, marketing tone |

## Quick facts

<!-- Adapt these to your project -->
- **Language:** Python 3.x+
- **Framework:** Flask / Django / FastAPI (pick one)
- **ORM:** SQLAlchemy / Django ORM
- **Database:** PostgreSQL (specify version)
- **Migrations:** Alembic / Django migrations
- **Package manager:** `uv`
- **Auth:** (describe your auth approach)
- **Frontend:** (describe: server-rendered templates, SPA, etc.)
- **Deployment:** (describe: Docker, Podman, Kubernetes, etc.)
- **i18n locales:** (list supported locales)

## What NOT to suggest

<!-- Adapt these constraints to your project -->
- Adding frameworks or infrastructure not already in use without explicit request
- Running package upgrade commands without approval
- Committing secrets or environment files
- Skipping tests or weakening test coverage

## Rule index

### always/ (core constraints)
- `python-rule.md` -- Python coding standards, immutability, type safety, repository hygiene
- `security-hardening-rule.md` -- Secrets, supply chain, CSRF, audit logging, container hygiene
- `high-risk-operations-rule.md` -- Stop-and-plan for audit, auth, access control, migrations
- `uncertainty-and-scope-rule.md` -- When uncertain, read tests first; conflicts are explicit

### workflow/ (process rules)
- `tdd-rule.md` -- Test-driven development: RED-GREEN-REFACTOR, anti-patterns
- `debugging-rule.md` -- Four-phase structured debugging: reproduce, locate, understand, fix
- `simplicity-rule.md` -- Simplicity over complexity, workflow calibration by task size
- `systematic-code-review-rule.md` -- Pre-commit checklist: spec, security, tests, quality, i18n, docs
- `branch-and-worktree-rule.md` -- Branch naming, worktrees, pre-push gate, merge options
- `development-workflow-rule.md` -- Five phases: clarify, plan, decompose, implement, review
- `alembic-migration-rule.md` -- Never hand-write revision IDs, verify chain, ripple check
- `testing-environments-rule.md` -- Three tiers: unit, integration, E2E; decision tree
- `feature-regression-guard-rule.md` -- No silent test deletion, characterisation tests first
- `no-auto-package-update.md` -- Never upgrade packages without explicit request
- `commit-no-cursor-trailer.md` -- No Co-authored-by or AI-tool trailers
- `immutable-tags-rule.md` -- Tags are immutable once pushed; bump version instead
- `semver-no-carry-rule.md` -- Version components grow beyond 99; never carry

### ui/ (templates, CSS)
- `accessibility-rule.md` -- WCAG AA: labels, contrast, keyboard, no hardcoded colours
- `template-translatability-rule.md` -- All user-facing strings must use i18n functions

### docs/ (documentation)
- `documentation-audience-rule.md` -- README for devs, docs site for business users and admins
- `marketing-tone-rule.md` -- Empathy, vendor respect, positive framing, message integrity
