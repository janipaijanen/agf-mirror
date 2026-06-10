# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""`agov-validate` orchestrator: aggregate multiple validators into one run.

Action-plan item A3 (issue #52). Lifts the per-validator
"capture-stdout-and-collect-exit-code" pattern out of
`pre_tag_check.py` so that:

    1. `agov-validate` exposes the same aggregation as a standalone
       Continuous Integration (CI)-friendly entry point.
    2. `pre_tag_check.py` consumes the same helper, removing the
       three duplicated `redirect_stdout` blocks it carried.

The aggregate exit code is computed via
`agents_govern._exit_codes.highest_precedence` (v1.1.36, item A4) so
the orchestrator emits the most-severe code across all validators.

Output:
    - default: human-readable per-validator block + summary line.
    - `--json`: single JSON object with `exit_code` and per-validator
      `name`/`exit_code`/`output` fields. The schema is part of the
      strict-contract surface and is intended for machine consumers.

The default validator set runs every validator that pre-tag-check
runs plus the repo-wide validators that are safe to run without
arguments. Individual `agov-validate-*` scripts remain callable
directly; this orchestrator is additive.
"""

from __future__ import annotations

import io
import json
import sys
from collections.abc import Callable, Sequence
from contextlib import redirect_stdout
from dataclasses import dataclass

from agents_govern._exit_codes import EXIT_PASS, highest_precedence

__all__ = [
    "ValidatorResult",
    "AggregateResult",
    "run_validator",
    "run_all",
    "default_validators",
    "to_json",
    "main",
]


@dataclass(frozen=True)
class ValidatorResult:
    """Outcome of a single validator run."""

    name: str
    exit_code: int
    output: str


@dataclass(frozen=True)
class AggregateResult:
    """Outcome of an orchestrator run across multiple validators."""

    results: tuple[ValidatorResult, ...]

    @property
    def exit_code(self) -> int:
        return highest_precedence(r.exit_code for r in self.results)


def run_validator(name: str, fn: Callable[[], int]) -> ValidatorResult:
    """Run `fn`, capturing its stdout, and wrap the result.

    The validator function is expected to return an int matching the
    taxonomy in `_exit_codes`. Stdout is captured so the orchestrator
    decides when to surface it (default: in the per-validator block;
    `--json`: as a string field on the JSON record).
    """
    buf = io.StringIO()
    with redirect_stdout(buf):
        rc = fn()
    return ValidatorResult(name=name, exit_code=rc, output=buf.getvalue())


def default_validators() -> list[tuple[str, Callable[[], int]]]:
    """The default validator set that `agov-validate` runs.

    Includes every validator that takes no required arguments. The
    eight validators listed match what pre-tag-check runs plus the
    additional repo-wide checks (`learnings`, `acronyms`,
    `contributions`, `adoption`) that are safely callable with no
    arguments.

    The `validate_adoption` callable returns `tuple[int, int]`; an
    inline adapter coerces it to the `() -> int` shape the
    orchestrator expects.
    """
    from agents_govern.validate_acronyms import validate_acronyms
    from agents_govern.validate_adoption import validate_adoption
    from agents_govern.validate_agents import validate_agents
    from agents_govern.validate_binding import validate_binding
    from agents_govern.validate_contributions import validate_contributions
    from agents_govern.validate_framework_config import validate_framework_config
    from agents_govern.validate_infosec import validate_infosec
    from agents_govern.validate_learnings import validate_learnings
    from agents_govern.validate_readme_stats import validate_readme_stats

    def adoption_int() -> int:
        rc, _warnings = validate_adoption()
        return rc

    return [
        ("validate-agents", validate_agents),
        ("validate-infosec", validate_infosec),
        ("validate-learnings", validate_learnings),
        ("validate-contributions", validate_contributions),
        ("validate-acronyms", validate_acronyms),
        ("validate-adoption", adoption_int),
        ("validate-readme-stats", validate_readme_stats),
        ("validate-binding", validate_binding),
        ("validate-framework-config", validate_framework_config),
    ]


def run_all(
    validators: Sequence[tuple[str, Callable[[], int]]] | None = None,
) -> AggregateResult:
    """Run the supplied (or default) validators and aggregate."""
    if validators is None:
        validators = default_validators()
    results = tuple(run_validator(name, fn) for name, fn in validators)
    return AggregateResult(results=results)


def to_json(result: AggregateResult) -> str:
    """Serialise an aggregate result to the strict-contract JSON shape."""
    payload = {
        "exit_code": result.exit_code,
        "validators": [
            {"name": r.name, "exit_code": r.exit_code, "output": r.output}
            for r in result.results
        ],
    }
    return json.dumps(payload, indent=2)


_HELP = """\
agov-validate — Aggregate validator runner for the agents-govern framework

Usage:
  uv run agov-validate [--json]

Runs every validator in the default set and aggregates their exit codes
through the precedence helper (infrastructure > config > invariant >
validation > pass). Each individual `agov-validate-*` script remains
callable on its own; this orchestrator is the single CI-friendly entry
point that promises a deterministic aggregate code.

Options:
  --json      Emit a single JSON object instead of human-readable blocks.
              The schema is part of the strict-contract surface (see
              docs/integration-contract.md).
  --help, -h  Show this message.

Default validator set (in run order):
  validate-agents, validate-infosec, validate-learnings,
  validate-contributions, validate-acronyms, validate-adoption,
  validate-readme-stats, validate-binding,
  validate-framework-config.

Exit code: highest-precedence code across the validators that ran.
"""


def main() -> None:
    args = sys.argv[1:]
    if "--help" in args or "-h" in args:
        print(_HELP)
        sys.exit(0)
    json_output = "--json" in args
    unknown = [a for a in args if a not in ("--json",)]
    if unknown:
        print(f"agov-validate: unknown argument(s): {' '.join(unknown)}")
        print("Run `agov-validate --help` for usage.")
        sys.exit(2)

    aggregate = run_all()

    if json_output:
        print(to_json(aggregate))
    else:
        for r in aggregate.results:
            print(f"=== {r.name} (exit={r.exit_code}) ===")
            if r.output:
                print(r.output, end="" if r.output.endswith("\n") else "\n")
        print(f"\nAggregate exit code: {aggregate.exit_code}")

    sys.exit(aggregate.exit_code)
