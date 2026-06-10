#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""Cross-check the JavaScript Object Notation (JSON) Schemas in
``schemas/`` against the framework's own artefacts.

Action-plan item C1 (issue #59) names this as the regeneration /
verification helper for the consumer-facing schema files. Today the
schemas are hand-curated (the validator code expresses cross-field
invariants JSON Schema cannot natively encode — see
``docs/integration-contract.md#schema-of-record-hierarchy``), so the
helper's role is verification, not generation:

    1. Confirm every ``schemas/*.schema.json`` parses as a valid
       Draft 7 schema.
    2. Confirm ``schemas/invariants.yaml`` parses as a YAML mapping
       with the C2 normalised shape.
    3. Print a per-file status line and exit 0 on success.

Adopters who want a stricter parity check (every agent / learning
file in the repo validates against the corresponding schema) can run
the test suite — ``tests/test_schemas.py::TestAgentSchemaParity``
and ``TestLearningSchemaParity`` are the canonical parity gates.

Usage::

    uv run python scripts/generate-schemas.py
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

# Add src/ so the helper can run via either `uv run` or direct python.
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "src"))

import yaml
from jsonschema import Draft7Validator


def _repo_root() -> Path:
    return Path(__file__).resolve().parent.parent


def _check_schema(path: Path) -> tuple[bool, str]:
    try:
        schema = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        return False, f"invalid JSON: {exc}"
    if not isinstance(schema, dict):
        return False, "top-level not an object"
    if "$schema" not in schema or "$id" not in schema:
        return False, "missing $schema or $id"
    try:
        Draft7Validator.check_schema(schema)
    except Exception as exc:  # jsonschema raises SchemaError subclass
        return False, f"invalid Draft 7 schema: {exc}"
    return True, "ok"


def _check_invariants(path: Path) -> tuple[bool, str]:
    try:
        data = yaml.safe_load(path.read_text(encoding="utf-8"))
    except yaml.YAMLError as exc:
        return False, f"invalid YAML: {exc}"
    if not isinstance(data, dict):
        return False, "top-level not a mapping"
    if "invariants" not in data or not isinstance(data["invariants"], list):
        return False, "missing invariants: list"
    required = {
        "id", "scope", "type", "parameters",
        "severity", "enforced_by", "introduced_in",
    }
    for inv in data["invariants"]:
        missing = required - set(inv.keys())
        if missing:
            return False, (
                f"invariant {inv.get('id', '?')!r} missing keys: "
                f"{sorted(missing)}"
            )
    return True, f"ok ({len(data['invariants'])} invariants)"


def main() -> int:
    schemas_dir = _repo_root() / "schemas"
    if not schemas_dir.is_dir():
        print(f"FAIL: {schemas_dir} does not exist", file=sys.stderr)
        return 1

    failures = 0
    for schema_file in sorted(schemas_dir.glob("*.schema.json")):
        ok, msg = _check_schema(schema_file)
        prefix = "OK  " if ok else "FAIL"
        print(f"{prefix}  {schema_file.name}: {msg}")
        if not ok:
            failures += 1

    invariants_path = schemas_dir / "invariants.yaml"
    if invariants_path.exists():
        ok, msg = _check_invariants(invariants_path)
        prefix = "OK  " if ok else "FAIL"
        print(f"{prefix}  {invariants_path.name}: {msg}")
        if not ok:
            failures += 1
    else:
        print(f"FAIL  {invariants_path.name}: not found")
        failures += 1

    if failures:
        print(f"\n{failures} schema artefact(s) failed verification.")
        return 1
    print("\nAll schema artefacts in schemas/ are well-formed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
