# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""`agov` umbrella Command Line Interface (CLI) entry point.

Action-plan items A5 (Continuous Integration (CI) flag surface) and
A6 (capability report) — both ride on this single new entry point so
the framework's existing flat `agov-validate-*` script convention is
preserved and the umbrella adds new flags additively.

Today's flag surface:

    agov                           — print help
    agov --help / -h               — print help
    agov --version                 — print framework version
    agov --contract-version        — alias of --version (per A5)
    agov --contract-changelog      — print CHANGELOG.md content
    agov --capabilities            — print JavaScript Object Notation
                                     (JSON) capability report (A6)

Exactly one action flag per invocation; mixing actions exits non-zero.
Bare `agov` with no flags prints help and exits 1 (no action chosen).

The `agov-validate-*` scripts remain callable directly; nothing
changes for adopters that already use them. The umbrella is additive.
"""

from __future__ import annotations

import sys
from collections.abc import Sequence

from agents_govern._files import repo_root
from agents_govern.capabilities import to_json as capabilities_json

__all__ = ["main"]


_HELP = """\
agov — agents-govern umbrella Command Line Interface (CLI)

Usage:
  agov [FLAG]

Flags (pick exactly one per invocation):
  --version                Print framework version (= contract version).
  --contract-version       Alias of --version (per A5 contract-stability).
  --contract-changelog     Print CHANGELOG.md content. Filter for
                           entries referencing validator command names,
                           exit codes, output formats, etc. to see
                           contract-affecting changes.
  --capabilities           Print JSON capability report (A6). Lists
                           which strict-contract capabilities ship in
                           this version and which are missing for
                           Full-tier compliance (per
                           docs/adopt/compliance-profile.md).
  --help, -h               Show this message.

The individual `agov-validate-*` scripts (validate-agents,
validate-learnings, validate-infosec, etc.) remain callable
directly. This umbrella is additive — it does not replace them.

See docs/integration-contract.md and docs/contract-stability.md for
the full contract surface and stability promise.
"""


_ACTION_FLAGS = {
    "--version",
    "--contract-version",
    "--contract-changelog",
    "--capabilities",
}


def _print_version() -> int:
    version_file = repo_root() / "VERSION"
    print(version_file.read_text(encoding="utf-8").strip())
    return 0


def _print_contract_changelog() -> int:
    changelog = repo_root() / "CHANGELOG.md"
    if not changelog.exists():
        print("CHANGELOG.md not found", file=sys.stderr)
        return 4  # infrastructure failure per _exit_codes taxonomy
    print(changelog.read_text(encoding="utf-8"), end="")
    return 0


def _print_capabilities() -> int:
    print(capabilities_json())
    return 0


def main(argv: Sequence[str] | None = None) -> None:
    if argv is None:
        argv = sys.argv[1:]
    args = list(argv)

    if not args:
        print(_HELP)
        sys.exit(1)

    if "--help" in args or "-h" in args:
        print(_HELP)
        sys.exit(0)

    actions = [a for a in args if a in _ACTION_FLAGS]
    if len(actions) > 1:
        print(
            f"agov: pick exactly one action flag per invocation "
            f"(got: {' '.join(actions)})",
            file=sys.stderr,
        )
        print("Run `agov --help` for usage.", file=sys.stderr)
        sys.exit(2)

    unknown = [a for a in args if a not in _ACTION_FLAGS]
    if unknown:
        print(f"agov: unknown argument(s): {' '.join(unknown)}", file=sys.stderr)
        print("Run `agov --help` for usage.", file=sys.stderr)
        sys.exit(2)

    action = actions[0]
    if action in ("--version", "--contract-version"):
        sys.exit(_print_version())
    if action == "--contract-changelog":
        sys.exit(_print_contract_changelog())
    if action == "--capabilities":
        sys.exit(_print_capabilities())

    # Defensive — every action flag should be handled above.
    print(f"agov: action {action!r} not implemented", file=sys.stderr)
    sys.exit(2)
