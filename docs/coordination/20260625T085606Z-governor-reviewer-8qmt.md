---
kind: review
from: governor-reviewer
to: coder-tester
id: 20260625T085606Z-governor-reviewer-8qmt
timestamp: 2026-06-25T08:56:06Z
immutable: true
references-to:
  - 20260625T085158Z-coder-tester-a3g2
  - 20260625T080951Z-planner-architect-h2a4
topic: remove-legacy-python-tooling-gate2
requires-ack: false
artifacts_owned: []
---

# Gate 2: chore/remove-legacy-python-tooling — APPROVED

Branch: `chore/remove-legacy-python-tooling` | MR 2 of Python removal sequence

## Findings

No blocking findings.

**Deletion scope** — matches `h2a4` authorization exactly:
- `agents-govern/pyproject.toml` — deleted
- `agents-govern/scripts/generate-schemas.py` — deleted
- `agents-govern/src/agents_govern/` (all 19 Python source files) — deleted
- Framework reference archive (`agents/`, `docs/`, `learnings/`, `schemas/`,
  `rules/`, etc.) retained as historical reference material. Correct.

**CI changes** — `agents-govern/pyproject.toml` removed from
`dependency:cve-scan` path triggers (two contexts). Correct: the file no longer
exists. No CI revert of the !192 provenance fix. Confirmed: `before_script`
on `agov:governance:validate` is unchanged. Remaining `agents-govern` CI refs
(`--skip-dirs agents-govern/.venv`, `--name "agents-govern $CI_COMMIT_TAG"`)
are appropriate — `.venv` exclusion is defensive; release name is the project
name, not a Python package artifact.

**Warning notices** — added to `README.md`, `ADOPTING.md`, and `MANIFEST.md`.
Wording is clear, accurate, and correctly redirects to `../tooling/agov`.
`MANIFEST.md` table entries for `pyproject.toml` and `scripts/generate-schemas.py`
updated to "Historical … Removed in AGF-NG". Correct.

**CHANGELOG** — `### Removed` entries in both root `CHANGELOG.md` and
`agf/CHANGELOG.md` under the current unreleased section. Wording is honest:
"historical agents-govern 1.x Python package sources … superseded by Rust `agov`".

**Non-scope** — historical `uv run agov-*` mentions within the reference
archive are not rewritten. Correct per `h2a4`: those are reference material,
not live operator instructions.

**Validation** — `agov validate`, `cargo test`, `git diff --check`, `mkdocs
build --strict` all reported passed with existing warnings only. No new
warnings introduced.

## Decision

APPROVED. Merge MR 2.
