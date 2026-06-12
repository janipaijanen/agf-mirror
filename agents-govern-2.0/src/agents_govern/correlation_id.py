# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""Universally Unique Identifier (UUID) v7 correlation identifiers.

Action-plan item B1 (issue #56). Strict-contract identifier used by
binding-layer artefacts (commit trailers per B2, sidecar metadata per
B3). Per the action plan:

    Generation: UUID v7 (time-ordered, sortable, no central authority).
    Concurrency: single identifier per task, parallel subtasks inherit,
    no Directed Acyclic Graph (DAG) semantics.
    Lifecycle: created at task assignment by Governor-Reviewer; closed
    at task completion (merge or escalation resolution).

The canonical Python `uuid.uuid7()` function only ships in CPython 3.13+;
the project's `requires-python = ">=3.12"` makes manual construction the
portable choice. Implementation follows Request For Comments (RFC) 9562
§5.7 verbatim:

    unix_ts_ms (48) || ver (4 = 0111) || rand_a (12) ||
    var (2 = 10)    || rand_b (62)

Random bytes from ``os.urandom`` (cryptographically strong); timestamp
from ``time.time_ns()`` truncated to milliseconds. Output is the
canonical lowercase hyphenated 36-character string per RFC 9562 §4.
"""

from __future__ import annotations

import os
import re
import time
import uuid

__all__ = [
    "generate",
    "is_valid",
    "extract_timestamp_ms",
]


_UUID_V7_RE = re.compile(
    r"^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$"
)


def generate() -> str:
    """Return a freshly-generated UUID v7 in canonical string form."""
    ts_ms = (time.time_ns() // 1_000_000) & ((1 << 48) - 1)

    rand_bytes = os.urandom(10)
    rand_a = int.from_bytes(rand_bytes[:2], "big") & 0x0FFF       # 12 bits
    rand_b = int.from_bytes(rand_bytes[2:], "big") & ((1 << 62) - 1)  # 62 bits

    value = (
        (ts_ms << 80)
        | (0x7 << 76)
        | (rand_a << 64)
        | (0b10 << 62)
        | rand_b
    )
    return str(uuid.UUID(int=value))


def is_valid(s: str) -> bool:
    """True iff ``s`` is the canonical lowercase string form of a UUID v7."""
    if not isinstance(s, str):
        return False
    return _UUID_V7_RE.match(s) is not None


def extract_timestamp_ms(s: str) -> int:
    """Return the embedded Unix-epoch millisecond timestamp.

    Raises ``ValueError`` if ``s`` is not a valid UUID v7 in canonical
    string form. Round-trippable with the system clock at the moment
    ``generate()`` was called (within one millisecond).
    """
    if not is_valid(s):
        raise ValueError(f"not a UUID v7: {s!r}")
    return uuid.UUID(s).int >> 80
