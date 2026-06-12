# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""`agov-validate-framework-config` — `framework.yaml` validator.

Action-plan item P1 (issue #63). The machine-readable companion to
`framework.md` is the assembly point for the strict-contract surface:

    - contract_version (per A5)
    - compliance_target (per the three-tier
      `docs/adopt/compliance-profile.md`)
    - precedence + conflict_policy (P1 itself)
    - binding (consumed by validate_binding when adopters opt into
      Process-aware or Full per Layer 3)
    - validators (free-form per-validator overrides)

Per round-5 §7 of the engagement: the schema declares
``additionalProperties: false`` at the top level so unknown keys are
rejected deterministically. The validator surfaces those rejections
and additionally cross-checks ``contract_version`` against the
repository's ``VERSION`` file (per A5: contract version equals
framework version, single source).

The validator is opt-in: if no ``framework.yaml`` exists at the
repository root, the validator passes (mirrors the schema-only mode
discipline shared with ``validate_binding``).
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

from agents_govern._exit_codes import EXIT_PASS, EXIT_VALIDATION_FAILURE
from agents_govern._files import repo_root
from agents_govern._output import error, fail, ok, warn
from agents_govern._yaml import load_yaml

__all__ = [
    "validate_framework_config",
    "main",
]


def _schema_path() -> Path:
    return repo_root() / "schemas" / "framework.schema.json"


def _load_schema() -> dict:
    return json.loads(_schema_path().read_text(encoding="utf-8"))


def validate_framework_config(*, path: Path | None = None) -> int:
    """Validate ``framework.yaml`` against ``schemas/framework.schema.json``.

    Returns the strict-contract exit code. Absent file passes
    (validator is opt-in until adopters declare a tier).
    """
    target = path if path is not None else (repo_root() / "framework.yaml")
    if not target.exists():
        warn(
            f"{target.name} not found; validator opt-in (declare a tier "
            f"in framework.yaml to enable enforcement)."
        )
        ok("Layer 5 framework-config contract loadable; opt-in not exercised.")
        return EXIT_PASS

    try:
        data = load_yaml(target)
    except Exception as exc:  # noqa: BLE001 — surface root cause to the user
        fail(f"{target}: failed to parse YAML: {exc}")
        return EXIT_VALIDATION_FAILURE

    if not isinstance(data, dict):
        fail(f"{target}: top-level must be a mapping (got {type(data).__name__})")
        return EXIT_VALIDATION_FAILURE

    # Schema-side validation (additionalProperties: false enforces
    # unknown-key rejection per round-5 §7).
    from jsonschema import Draft7Validator

    schema = _load_schema()
    validator = Draft7Validator(schema)
    errors = sorted(validator.iter_errors(data), key=lambda e: list(e.path))
    if errors:
        for err in errors:
            location = "/".join(str(p) for p in err.path) or "(root)"
            error(f"{target.name}: {location}: {err.message}")
        return EXIT_VALIDATION_FAILURE

    # Contract-versioning cross-check (A5: contract version = framework version).
    declared = data.get("contract_version")
    repo_version = (repo_root() / "VERSION").read_text(encoding="utf-8").strip()
    if declared and declared != repo_version:
        fail(
            f"{target.name}: contract_version {declared!r} does not match "
            f"VERSION {repo_version!r} (per A5: single source of version truth)."
        )
        return EXIT_VALIDATION_FAILURE

    ok(f"{target.name}: valid (compliance_target={data.get('compliance_target', 'unset')!r}).")
    return EXIT_PASS


_HELP = """\
agov-validate-framework-config — framework.yaml validator

Usage:
  uv run agov-validate-framework-config

Validates the repository's framework.yaml against
schemas/framework.schema.json. Per round-5 §7 of the engagement, the
schema declares additionalProperties: false at the top level, so
unknown keys are rejected. Additionally cross-checks
contract_version against the repo's VERSION file (A5: contract
version equals framework version).

The validator is opt-in: when framework.yaml is absent, the validator
passes (mirrors the schema-only mode discipline of
agov-validate-binding).

Options:
  --help, -h  Show this message.

Exit codes (per docs/integration-contract.md#exit-codes):
  0 — pass (or framework.yaml absent — opt-in not exercised)
  1 — validation failure (unknown keys, bad enum value,
      contract_version mismatch, malformed YAML)
"""


def main() -> None:
    args = sys.argv[1:]
    if "--help" in args or "-h" in args:
        print(_HELP)
        sys.exit(0)
    if args:
        print(
            f"agov-validate-framework-config: unknown argument(s): "
            f"{' '.join(args)}"
        )
        print("Run `agov-validate-framework-config --help` for usage.")
        sys.exit(2)
    sys.exit(validate_framework_config())
