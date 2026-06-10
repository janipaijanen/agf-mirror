# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""Immutable data structures for validation results."""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path

__all__ = [
    "Finding",
    "ValidationResult",
    "AgentMeta",
    "ReadmeStat",
]

type FilePath = Path
type ExitCode = int
type YamlData = dict[str, object]


@dataclass(frozen=True, slots=True)
class Finding:
    """A single validation finding (immutable)."""

    file: str
    finding_id: str
    category: str
    message: str
    severity: str  # "block" | "warn" | "error"
    suggestion: str | None = None
    acked: bool = False


@dataclass(frozen=True, slots=True)
class ValidationResult:
    """Aggregated result from a validation run (immutable)."""

    findings: tuple[Finding, ...]
    files_checked: int

    @property
    def errors(self) -> int:
        return sum(1 for f in self.findings if f.severity == "error")

    @property
    def blocked(self) -> int:
        return sum(
            1
            for f in self.findings
            if f.severity == "block" and not f.acked
        )

    @property
    def warnings(self) -> int:
        return sum(1 for f in self.findings if f.severity == "warn")

    @property
    def acked(self) -> int:
        return sum(1 for f in self.findings if f.acked)

    @property
    def passed(self) -> bool:
        return self.blocked == 0 and self.errors == 0


@dataclass(frozen=True, slots=True)
class AgentMeta:
    """Extracted metadata from an agent YAML definition (immutable)."""

    name: str
    agent_id: str
    version: str
    framework_version: str
    tier: str
    phase: int | str
    has_purpose: bool
    has_role_mapping: bool
    has_boundaries: bool


@dataclass(frozen=True, slots=True)
class ReadmeStat:
    """A single stat claim from README vs actual count (immutable)."""

    label: str
    readme_value: int | None
    actual_value: int
    tolerance: int = 0

    @property
    def matches(self) -> bool:
        if self.readme_value is None:
            return False
        return abs(self.readme_value - self.actual_value) <= self.tolerance
