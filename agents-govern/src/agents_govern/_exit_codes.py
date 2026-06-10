# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""Exit code taxonomy for agov-* validators.

Defines the five-code taxonomy from action-plan item A4 (issue #53)
and a precedence helper for the orchestrator (action-plan item A3,
issue #52).

The taxonomy:
    EXIT_PASS                   0 — pass (no findings)
    EXIT_VALIDATION_FAILURE     1 — validation failure (single-artefact field-level)
    EXIT_CONFIG_ERROR           2 — configuration error (validator cannot determine what to check)
    EXIT_INVARIANT_VIOLATION    3 — invariant violation (cross-artefact rule break)
    EXIT_INFRASTRUCTURE         4 — infrastructure / Input/Output (I/O) failure (validator could not run)

Precedence (orchestrator emits highest):
    infrastructure > config > invariant > validation > pass

Adopters branching CI scripts on `!= 0` are unaffected by the taxonomy
expansion — both the existing 0/1 returns and the new 2/3/4 returns
are non-zero. Adopters branching on `== 1` may see different non-zero
codes in some scenarios when validators opportunistically begin using
2/3/4 in future releases; the contract-stability promise (A5) names
this as a minor-version-additive change.

For the public-facing specification, see
`docs/integration-contract.md#exit-codes`.
"""

from __future__ import annotations

from collections.abc import Iterable

__all__ = [
    "EXIT_PASS",
    "EXIT_VALIDATION_FAILURE",
    "EXIT_CONFIG_ERROR",
    "EXIT_INVARIANT_VIOLATION",
    "EXIT_INFRASTRUCTURE",
    "highest_precedence",
]

# Code values are part of the public integration contract — do not change.
EXIT_PASS: int = 0
EXIT_VALIDATION_FAILURE: int = 1
EXIT_CONFIG_ERROR: int = 2
EXIT_INVARIANT_VIOLATION: int = 3
EXIT_INFRASTRUCTURE: int = 4

# Precedence ordering: highest first. Used by the orchestrator to
# select which code to emit when aggregating multiple validators.
# Pass last so it only "wins" if everything passed.
_PRECEDENCE: tuple[int, ...] = (
    EXIT_INFRASTRUCTURE,      # validator could not run — most severe
    EXIT_CONFIG_ERROR,        # validator cannot determine what to check
    EXIT_INVARIANT_VIOLATION, # cross-artefact rule break — coordination failure
    EXIT_VALIDATION_FAILURE,  # single-artefact field-level
    EXIT_PASS,                # all good
)

_VALID_CODES: frozenset[int] = frozenset(_PRECEDENCE)


def highest_precedence(codes: Iterable[int]) -> int:
    """Return the highest-precedence code from the given iterable.

    Empty input returns `EXIT_PASS`. Codes outside the taxonomy raise
    `ValueError` — the orchestrator should not silently accept unknown
    codes.

    Examples:
        >>> highest_precedence([EXIT_PASS, EXIT_VALIDATION_FAILURE])
        1
        >>> highest_precedence([EXIT_VALIDATION_FAILURE, EXIT_INFRASTRUCTURE])
        4
        >>> highest_precedence([])
        0
    """
    materialised = list(codes)
    for c in materialised:
        if c not in _VALID_CODES:
            raise ValueError(
                f"Unknown exit code {c!r}; expected one of {sorted(_VALID_CODES)}"
            )

    if not materialised:
        return EXIT_PASS

    seen = set(materialised)
    for code in _PRECEDENCE:
        if code in seen:
            return code

    # Unreachable: every code in materialised is in _VALID_CODES,
    # which equals the set of _PRECEDENCE.
    return EXIT_PASS
